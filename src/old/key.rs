use crate::glyph::{Alphabet, Char};
use generic_array::{ArrayLength, GenericArray};
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone, Debug)]
pub struct Key<N: ArrayLength<Char>>(GenericArray<Char, N>);

impl<N: ArrayLength<Char>> Key<N> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        N::to_usize()
    }

    #[inline(always)]
    pub fn decode(&self, encoded: &[Char], decoded: &mut [Char]) {
        for (index, &c) in encoded.iter().enumerate() {
            decoded[index] = self[c as usize];
        }
    }

    #[inline(always)]
    pub fn copy(&mut self, key: &Key<N>) {
        self.0.copy_from_slice(key.0.as_slice());
    }

    #[inline(always)]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    pub fn random_putc(&mut self, n: usize, alphabet: &Alphabet) {
        let len = self.len();
        assert!(n < len, "Too large");
        for i in 0..n {
            self[len - i] = alphabet.random_char();
            self[len - i] = alphabet.random_char();
            self[len - i] = alphabet.random_char();
        }
    }

    pub fn random_swap(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..self.len());
        let b = rng.gen_range(0..self.len());
        self.0.swap(a, b);
    }

    pub fn random(cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) -> Self {
        let mut key = Self::default();
        key.randomize(cipher_alphabet, output_alphabet);
        key
    }

    pub fn randomize(&mut self, cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) {
        let mut encoded = cipher_alphabet.char_set();
        let mut decoded = output_alphabet.char_set();

        while encoded.len() > 0 && decoded.len() > 0 {
            use crate::util;
            let encoded_char = util::take_random_element_from_set(&mut encoded);
            let decoded_char = util::take_random_element_from_set(&mut decoded);
            self[encoded_char as usize] = decoded_char;
        }

        for &encoded_char in encoded.iter() {
            let decoded_char = output_alphabet.random_char();
            self[encoded_char as usize] = decoded_char;
        }
    }
}

impl<N: ArrayLength<Char>> Default for Key<N> {
    fn default() -> Self {
        // TODO: use alloca?
        let vec = vec![0 as Char; N::to_usize()];
        Self(GenericArray::clone_from_slice(&vec))
    }
}

impl<N: ArrayLength<Char>> Index<usize> for Key<N> {
    type Output = Char;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<N: ArrayLength<Char>> IndexMut<usize> for Key<N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Clone, Debug)]
pub struct Keys<N: ArrayLength<Char>, K: ArrayLength<Key<N>>>(GenericArray<Key<N>, K>);

impl<N: ArrayLength<Char>, K: ArrayLength<Key<N>>> Keys<N, K> {

    #[inline(always)]
    pub fn period(&self) -> usize {
        K::to_usize()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        N::to_usize()
    }

    #[inline(always)]
    pub fn decode(&self, encoded: &[Char], decoded: &mut [Char]) {
        for (index, &c) in encoded.iter().enumerate() {
            let period = index % self.period();
            decoded[index] = self[period][c as usize];
        }
    }

    #[inline(always)]
    pub fn copy(&mut self, keys: &Keys<N, K>) {
        self.0.clone_from_slice(keys.0.as_slice())
    }

    pub fn random(
        cipher_alphabet: &Alphabet,
        output_alphabet: &Alphabet,
    ) -> Self {
        let mut keys = Self::default();
        keys.randomize(cipher_alphabet, output_alphabet);
        keys
    }

    pub fn randomize(&mut self, cipher_alphabet: &Alphabet, output_alphabet: &Alphabet) {
        for period in 0..self.period() {
            self[period].randomize(cipher_alphabet, output_alphabet);
        }
    }

    #[inline(always)]
    pub fn random_putc(&mut self, period: usize, n: usize, alphabet: &Alphabet) {
        self[period].random_putc(n, alphabet);
    }

    #[inline(always)]
    pub fn random_swap(&mut self, period: usize) {
        self[period].random_swap();
    }
}

impl<N: ArrayLength<Char>, K: ArrayLength<Key<N>>> Default for Keys<N, K> {
    fn default() -> Self {
        // TODO: use alloca?
        let vec = vec![Default::default(); N::to_usize()];
        Self(GenericArray::clone_from_slice(&vec))
    }
}

impl<N: ArrayLength<Char>, K: ArrayLength<Key<N>>> Index<usize> for Keys<N, K> {
    type Output = Key<N>;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<N: ArrayLength<Char>, K: ArrayLength<Key<N>>> IndexMut<usize> for Keys<N, K> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
