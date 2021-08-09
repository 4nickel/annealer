use std::collections::HashMap;
use crate::pre::Pre;

pub struct NGrams<'a, T> {
    size: usize,
    index: usize,
    slice: &'a [T],
}

impl<'a, T> NGrams<'a, T> {
    pub fn new(slice: &'a [T], size: usize) -> Self {
        assert!(size > 0, "NGrams: size may not be 0");
        Self {
            size,
            slice,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for NGrams<'a, T> {
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

pub struct Count {
    pub map: HashMap<String, u64>,
    pub n: usize,
}

impl Count {
    pub fn new(n: usize) -> Self {
        assert!(n != 0, "Invalid argument: n may not be zero");
        Self {
            map: Default::default(),
            n,
        }
    }

    pub fn total(&self) -> u64 {
        self.map.values().sum()
    }

    pub fn add(&mut self, input: &str) {
        if input.len() >= self.n {
            for i in 0..=(input.len() - self.n) {
                let gram = &input[i..(i + self.n)];
                match self.map.get_mut(gram) {
                    Some(count) => {
                        *count += 1;
                    }
                    None => {
                        self.map.insert(gram.into(), 1);
                    }
                }
            }
        }
    }

    pub fn from_file(path: &str, n: usize) -> Self {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let mut count = Self::new(n);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let words = line.split_whitespace().collect::<Vec<_>>();
            assert!(words[0].len() == n);
            count.map.insert(words[0].into(), words[1].parse::<u64>().unwrap());
        }
        count
    }
}

pub struct Frequency {
    pub map: HashMap<String, f64>,
    pub n: usize,
    pub floor: f64,
}

impl Frequency {
    pub fn new(n: usize, floor: f64) -> Self {
        Self {
            map: Default::default(),
            n,
            floor,
        }
    }

    pub fn from_count(count: &Count) -> Self {
        let total = count.total() as f64;
        let floor = 1.0 / -(0.1f64 / total).log10();
        let mut frequency = Self::new(count.n, floor);
        for (key, &val) in count.map.iter() {
            let score = 1.0 / -(val as f64 / total).log10();
            frequency.map.insert(key.clone(), score);
        }
        frequency
    }

    pub fn ordered(&self) -> Vec<(String, f64)> {
        let mut output = Vec::new();
        for (key, &val) in self.map.iter() {
            output.push((key.into(), val));
        }
        output.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        output
    }
}

#[derive(Debug, Clone)]
pub struct Dictionary(pub HashSet<String>);

impl Dictionary {
    pub fn from_file(path: &str, pre: &Pre) -> Self {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let mut dict = HashSet::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let word = pre.process(&line);
            dict.insert(word.into());
        }
        Self(dict)
    }
}

use std::collections::HashSet;
pub fn find_words<'a>(text: &'a str, dict: &HashSet<String>) -> Vec<(usize, usize)> {
    let max_len = 10;
    let mut words = Vec::new();
    for i in 0..text.len() {
        let limit = (i + max_len + 1).min(text.len());
        let chunk = &text[i..limit];
        for j in 1..chunk.len()+1 {
            let word = &chunk[..j];
            if dict.get(word).is_some() {
                words.push((i, i+j));
            }
        }
    }
    words
}
