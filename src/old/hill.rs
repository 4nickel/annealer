use crate::glyph::{Alphabet, Char};
use crate::key::{Key, Keys};
use generic_array::ArrayLength;

pub const MIN_ENERGY: f64 = -99e99;

pub struct Combinator {
    n: usize,
    m: usize,
    i: usize,
    j: usize,
}

impl Combinator {
    pub fn new(n: usize) -> Self {
        let m = n >> 2;
        Self { n, m, i: 0, j: m, }
    }
}

impl Iterator for Combinator {
    type Item=(usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.j < self.n {
            let j = self.j;
            self.j += 1;
            return Some((self.i, j))
        }
        if self.i < self.m {
            let i = self.i;
            self.i += 1;
            self.j = self.m;
            return Some((i, self.j))
        }
        None
    }
}

pub struct ClimberConfig<N, K, E, A, R, M>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
    E: Fn(&[Char]) -> f64,
    A: Fn(f64, f64) -> bool,
    R: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet),
    M: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet, usize),
{
    pub energy: E,
    pub accept: A,
    pub random_key: R,
    pub mutate_key: M,
    _phantom_n: std::marker::PhantomData<N>,
    _phantom_k: std::marker::PhantomData<K>,
}

impl<N, K, E, A, R, M> ClimberConfig<N, K, E, A, R, M>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
    E: Fn(&[Char]) -> f64,
    A: Fn(f64, f64) -> bool,
    R: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet),
    M: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet, usize),
{
    pub fn new(energy: E, accept: A, random_key: R, mutate_key: M) -> Self {
        Self {
            energy,
            accept,
            random_key,
            mutate_key,
            _phantom_n: std::marker::PhantomData,
            _phantom_k: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClimberRun<N, K>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
{
    pub key: Keys<N, K>,
    pub energy: f64,
}

impl<N, K> ClimberRun<N, K>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
{
    pub fn copy(&mut self, run: &ClimberRun<N, K>) {
        self.energy = run.energy;
        self.key.copy(&run.key);
    }
}

#[derive(Debug)]
pub struct Climber<'a, N, K>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
{
    pub cipher_alphabet: Alphabet,
    pub output_alphabet: Alphabet,
    pub cipher_buf: &'a [Char],
    pub output_buf: &'a mut [Char],
    pub run: ClimberRun<N, K>,
    pub tmp: ClimberRun<N, K>,
    pub top: ClimberRun<N, K>,
}

impl<'a, N, K> Climber<'a, N, K>
where
    N: ArrayLength<Char>,
    K: ArrayLength<Key<N>>,
{
    pub fn new(
        cipher_buf: &'a [Char],
        output_buf: &'a mut [Char],
        cipher_alphabet: Alphabet,
        output_alphabet: Alphabet,
    ) -> Self {
        assert!(cipher_buf.len() == output_buf.len(), "Length mismatch");
        let top = ClimberRun {
            key: Keys::default(),
            energy: MIN_ENERGY,
        };
        let run = top.clone();
        let tmp = top.clone();
        Self {
            cipher_alphabet,
            output_alphabet,
            cipher_buf,
            output_buf,
            run,
            tmp,
            top,
        }
    }

    pub fn climb<E, A, R, M>(
        &mut self,
        config: &ClimberConfig<N, K, E, A, R, M>,
    ) where
        E: Fn(&[Char]) -> f64,
        A: Fn(f64, f64) -> bool,
        R: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet),
        M: Fn(&mut Keys<N, K>, &Alphabet, &Alphabet, usize),
    {
        (config.random_key)(
            &mut self.run.key,
            &self.cipher_alphabet,
            &self.output_alphabet,
        );
        self.run.key.decode(&self.cipher_buf, self.output_buf);
        self.run.energy = (config.energy)(self.output_buf);

        let mut counter = 0;
        let mut stale = 0;
        while counter < 1000 {
            for (i, j) in Combinator::new(self.run.key.len()) {
                if stale == 100 {
                    (config.mutate_key)(
                        &mut self.run.key,
                        &self.cipher_alphabet,
                        &self.output_alphabet,
                        0,
                    );
                    stale = 0;
                }
                self.tmp.key.copy(&self.run.key);
                self.tmp.key[0].swap(i, j);
                self.tmp.key.decode(&self.cipher_buf, self.output_buf);
                self.tmp.energy = (config.energy)(self.output_buf);
                if (config.accept)(self.tmp.energy, self.run.energy) {
                    self.run.copy(&self.tmp);
                    if self.tmp.energy > self.top.energy {
                        self.top.copy(&self.tmp);
                        stale = 0;
                        continue;
                    }
                }
                stale += 1;
            }
            counter += 1;
        }
    }
}
