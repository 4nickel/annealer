use std::collections::{HashMap, HashSet};

// Type alias for custom encoded characters.
// Can be set to u16 if needed.
pub type Char = u8;

#[derive(Clone, Debug)]
pub struct Alphabet {
    len: usize,
}

impl Alphabet {
    #[inline(always)]
    pub fn new(len: usize) -> Alphabet {
        Self { len }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn extend(&mut self, len: usize) {
        self.len += len;
    }

    #[inline(always)]
    pub fn char_vec(&self) -> Vec<Char> {
        (0..self.len).map(|i| i as Char).collect()
    }

    #[inline(always)]
    pub fn char_set(&self) -> HashSet<Char> {
        (0..self.len).map(|i| i as Char).collect()
    }

    #[inline(always)]
    pub fn random_char(&self) -> Char {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.len) as Char
    }
}

pub struct Encoding {
    encode_map: HashMap<char, Char>,
    decode_map: HashMap<Char, char>,
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
        assert!(encode_map.len() == decode_map.len(), "Length mismatch");
        Self {
            encode_map,
            decode_map,
        }
    }

    #[inline(always)]
    pub fn alphabet(&self) -> Alphabet {
        Alphabet::new(self.encode_map.len())
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
