use crate::encoding::{Alphabet, Char};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Crib {
    pub fixed: Vec<usize>,
    pub loose: Vec<usize>
}

impl Crib {
    pub fn new(len: usize) -> Self {
        let mut loose = vec![0; len];
        let fixed = vec![];
        for index in 0..len {
            loose[index] = index;
        }
        Self { loose, fixed }
    }

    pub fn sample(&self) -> usize {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.loose[rng.gen_range(0..self.loose.len())]
    }

    pub fn find_loose_index(&self, item: usize) -> Option<usize> {
        for index in 0..self.loose.len() {
            if self.loose[index] == item {
                return Some(index)
            }
        }
        None
    }

    pub fn fix(&mut self, item: usize) {
        if let Some(index) = self.find_loose_index(item) {
            self.loose.remove(index);
            self.fixed.push(item);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Key(Vec<Char>);

impl Key {
    #[inline(always)]
    pub fn new(len: usize) -> Self {
        Self(vec![0 as Char; len])
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    #[inline(always)]
    pub fn decode(&self, cipher: &[Char], output: &mut [Char]) {
        for (index, &c) in cipher.iter().enumerate() {
            output[index] = self[c as usize];
        }
    }

    #[inline(always)]
    pub fn copy(&mut self, key: &Key) {
        self.0.copy_from_slice(key.0.as_slice());
    }

    pub fn splice(&mut self, key: &Key) {
        for index in 0..key.len() {
            if crate::util::probability(0.5) {
                self[index] = key[index];
            }
        }
    }

    pub fn random_distribution(len: usize, frq: &[f64]) -> Self {
        let mut tmp = Self::new(len);
        for index in 0..tmp.len() {
            tmp[index] = 0;
        }
        let mut key = Self::new(len);

        let mut key_index = 0;
        let mut frq_index = 0;
        loop {
            key_index %= key.len();
            frq_index %= frq.len();

            // Check if done
            let mut done = true;
            for index in 0..key.len() {
                if tmp[index] == 0 {
                    done = false;
                }
            }
            if done {
                break
            }

            // Skip done letters
            if tmp[key_index] != 0 {
                key_index += 1;
                continue
            }

            // Probabilistically set character
            if crate::util::probability(frq[frq_index]) {
                key[key_index] = frq_index as Char;
                tmp[key_index] = 1;
            }
            frq_index += 1;
            key_index += 1;
        }

        key
    }

    pub fn random(cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) -> Self {
        let mut init = Self::new(cipher_alphabet.len());
        init.randomize(cipher_alphabet, output_alphabet);
        init
    }

    pub fn randomize(&mut self, cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) {
        let mut cipher = cipher_alphabet.char_set();
        let mut output = output_alphabet.char_set();

        while cipher.len() > 0 && output.len() > 0 {
            use crate::util;
            let cipher_char = util::take_random_element_from_set(&mut cipher);
            let output_char = util::take_random_element_from_set(&mut output);
            self[cipher_char as usize] = output_char;
        }

        for &cipher_char in cipher.iter() {
            let output_char = output_alphabet.random_char();
            self[cipher_char as usize] = output_char;
        }
    }

    pub fn random_putc(&mut self, alphabet: &Alphabet) {
        let len = self.0.len();
        self.0[len-3] = alphabet.random_char();
        self.0[len-2] = alphabet.random_char();
        self.0[len-1] = alphabet.random_char();
    }

    pub fn random_swap(&mut self, crib: &Crib) {
        let (a, b) = (crib.sample(), crib.sample());
        self.0.swap(a, b);
    }
}

impl Index<usize> for Key {
    type Output = Char;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Key {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Clone, Debug)]
pub struct Keys(Vec<Key>);

impl Keys {
    #[inline(always)]
    pub fn new(count: usize, len: usize) -> Self {
        Self(vec![Key::new(len); count])
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self[0].len()
    }

    #[inline(always)]
    pub fn decode(&self, cipher: &[Char], output: &mut [Char]) {
        for (i, &c) in cipher.iter().enumerate() {
            let k = i % self.count();
            output[i] = self[k][c as usize];
        }
    }

    #[inline(always)]
    pub fn copy(&mut self, keys: &Keys) {
        for key in 0..self.count() {
            self[key].copy(&keys[key]);
        }
    }

    pub fn random(
        count: usize,
        cipher_alphabet: &Alphabet,
        output_alphabet: &Alphabet,
    ) -> Self {
        let mut init = Self::new(count, cipher_alphabet.len());
        init.randomize(cipher_alphabet, output_alphabet);
        init
    }

    pub fn randomize(&mut self, cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) {
        for key in 0..self.count() {
            self[key].randomize(cipher_alphabet, output_alphabet);
        }
    }

    #[inline(always)]
    pub fn random_putc(&mut self, key: usize, alphabet: &Alphabet) {
        self[key].random_putc(alphabet);
    }

    #[inline(always)]
    pub fn random_swap(&mut self, key: usize, crib: &Crib) {
        self[key].random_swap(crib);
    }
}

impl Index<usize> for Keys {
    type Output = Key;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Keys {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
