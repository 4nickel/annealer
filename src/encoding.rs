use std::collections::{HashMap, HashSet};
use crate::stats::NGrams;
use crate::stats::Frequency as UnicodeFrequency;

// Type alias for custom encoded characters.
// Can be set to u16 if needed.
pub type Char = u8;

#[derive(Clone, Debug)]
pub struct Alphabet {
    len: usize,
}

impl Alphabet {
    pub fn new(len: usize) -> Alphabet {
        Self { len }
    }

    pub fn extend(&mut self, n: usize) {
        self.len += n;
    }

    pub fn char_set(&self) -> HashSet<Char> {
        (0 as Char..self.len as Char).collect()
    }

    pub fn char_vec(&self) -> Vec<Char> {
        (0 as Char..self.len as Char).collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn random_char(&self) -> Char {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0 as Char..self.len as Char)
    }
}

#[derive(Clone, Debug)]
pub struct Encoding {
    pub encode_map: HashMap<char, Char>,
    pub decode_map: HashMap<Char, char>,
}

impl Encoding {
    pub fn from_alphabet_string(alphabet: &str) -> Self {
        let encode_map: HashMap<char, Char> = alphabet
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as Char))
            .collect();
        let decode_map: HashMap<Char, char> = alphabet
            .chars()
            .enumerate()
            .map(|(i, c)| (i as Char, c))
            .collect();
        Self {
            encode_map,
            decode_map,
        }
    }

    #[inline(always)]
    pub fn alphabet(&self, homophone_ratio: f64) -> Alphabet {
        if homophone_ratio < 0.0 {
            panic!();
        }
        let extend_by = (self.encode_map.len() as f64 * homophone_ratio).floor();
        Alphabet::new(self.encode_map.len() + extend_by as usize)
    }

    #[inline(always)]
    pub fn char_set(&self) -> HashSet<char> {
        self.encode_map.keys().map(|c| *c).collect()
    }

    #[inline(always)]
    pub fn encode(&self, decoded: &[char], encoded: &mut [Char]) {
        for (index, value) in decoded.iter().enumerate() {
            encoded[index] = *self.encode_map.get(value).expect("Encode error");
        }
    }

    #[inline(always)]
    pub fn decode(&self, encoded: &[Char], decoded: &mut [char]) {
        for (index, value) in encoded.iter().enumerate() {
            decoded[index] = *self.decode_map.get(value).expect("Decode error");
        }
    }

    pub fn encode_str(&self, decoded: &str) -> Vec<Char> {
        let chars: Vec<char> = decoded.chars().collect();
        let mut encoded = vec![0; decoded.len()];
        self.encode(chars.as_slice(), encoded.as_mut_slice());
        encoded
    }

    pub fn decode_str(&self, encoded: &[Char]) -> String {
        let mut chars = vec![0 as char; encoded.len()];
        self.decode(&encoded, chars.as_mut_slice());
        chars.iter().collect()
    }
}

#[derive(Clone, Debug)]
pub struct Frequency {
    pub map: HashMap<Vec<Char>, f64>,
    pub n: usize,
    pub floor: f64,
}

impl Frequency {
    pub fn new(frequency: &UnicodeFrequency, encoding: &Encoding) -> Self {
        let mut map: HashMap<Vec<Char>, f64> = HashMap::with_capacity(frequency.map.len());
        for (key, &val) in frequency.map.iter() {
            let decoded = key.chars().collect::<Vec<char>>();
            let mut encoded = vec![0; key.len()];
            encoding.encode(&decoded, &mut encoded);
            map.insert(encoded, val);
        }
        Self {
            map,
            n: frequency.n,
            floor: frequency.floor,
        }
    }

    pub fn score(&self, glyphs: &[Char]) -> f64 {
        let mut score = 0.0;
        for gram in NGrams::new(glyphs, self.n) {
            score += self.map.get(gram).unwrap_or(&self.floor);
        }
        score
    }
}
