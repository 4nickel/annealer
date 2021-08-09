use std::collections::HashMap;
use crate::glyph::{Encoding, Char};
use crate::gram::Frequency as UtfFrequency;

pub struct Frequency {
    pub map: HashMap<Vec<Char>, f64>,
    pub n: usize,
    pub floor: f64,
}

pub struct GramIter<'a, T> {
    size: usize,
    index: usize,
    slice: &'a [T],
}

impl<'a, T> GramIter<'a, T> {
    pub fn new(slice: &'a [T], size: usize) -> Self {
        assert!(size > 0, "GramIter: size may not be 0");
        Self {
            size,
            slice,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for GramIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.size > self.slice.len() {
            return None;
        }
        let slice = &self.slice[self.index..self.index + self.size];
        self.index += 1;
        Some(slice)
    }
}


impl Frequency {
    pub fn new(frequency: &UtfFrequency, encoding: &Encoding) -> Self {
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

    pub fn fitness(&self, chars: &[Char]) -> f64 {
        let mut score = 0.0;
        for gram in GramIter::new(chars, self.n) {
            score += self.map.get(gram).unwrap_or(&self.floor);
        }
        score
    }
}
