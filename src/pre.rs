use std::collections::{HashMap, HashSet};

pub struct Pre {
    pub normalize: HashMap<char, char>,
    pub emit: HashSet<char>,
}

impl Pre {
    pub fn new() -> Self {
        Self {
            normalize: Default::default(),
            emit: Default::default(),
        }
    }

    pub fn from_emitter_and_normalizer(emit: HashSet<char>, normalize: HashMap<char, char>) -> Self {
        Self {
            normalize,
            emit
        }
    }

    pub fn process(&self, input: &str) -> String {
        let mut output = String::new();
        for c in input.chars().map(|c| *self.normalize.get(&c).unwrap_or(&c)) {
            if self.emit.get(&c).is_some() {
                output.push(c)
            }
        }
        output
    }
}

pub const LATIN_MAJOR: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const LATIN_MINOR: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn latin(pre: &mut Pre) {
    for i in 0..LATIN_MAJOR.len() {
        pre.normalize.insert(LATIN_MINOR[i], LATIN_MAJOR[i]);
        pre.emit.insert(LATIN_MAJOR[i]);
    }
}
