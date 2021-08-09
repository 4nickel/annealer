use crate::encoding::{Encoding, Char, Alphabet};
use crate::key::{Crib, Key};

const MIN_ENERGY: f64 = -99e99;

pub struct Config<REPORT, ENERGY, ACCEPT, RANDOMKEY, DERIVEKEY, MUTATEKEY, CRIB>
where
    REPORT: Fn(&Climber, usize, usize, usize),
    ENERGY: Fn(&[Char]) -> f64,
    ACCEPT: Fn(f64, f64) -> bool,
    RANDOMKEY: Fn(&mut Climber),
    DERIVEKEY: Fn(&mut Climber),
    MUTATEKEY: Fn(&mut Climber),
    CRIB: Fn(&mut Climber),
{
    pub cycle: usize,
    pub derive_cycle: usize,
    pub mutate_cycle: usize,
    pub energy: ENERGY,
    pub accept: ACCEPT,
    pub random_key: RANDOMKEY,
    pub derive_key: DERIVEKEY,
    pub mutate_key: MUTATEKEY,
    pub report: REPORT,
    pub crib: CRIB,
}

#[derive(Debug, Clone)]
pub struct Climber {
    pub cipher_encoding: Encoding,
    pub cipher_alphabet: Alphabet,
    pub output_encoding: Encoding,
    pub output_alphabet: Alphabet,
    pub cipher_buf: Vec<Char>,
    pub output_buf: Vec<Char>,
    pub top_key: Key,
    pub run_key: Key,
    pub fix_key: Key,
    pub run_energy: f64,
    pub top_energy: f64,
    pub crib: Crib,
}

impl Climber {
    pub fn new(
        cipher_buf: Vec<Char>,
        cipher_encoding: Encoding,
        output_encoding: Encoding,
        homophone_ratio: f64,
    ) -> Self {
        let output_alphabet = output_encoding.alphabet(0.0);
        let cipher_alphabet = cipher_encoding.alphabet(homophone_ratio);
        let fix_key = Key::new(cipher_alphabet.len());
        let top_key = Key::new(cipher_alphabet.len());
        let run_key = Key::new(cipher_alphabet.len());
        let output_buf = cipher_buf.clone();
        println!("Key length: {}", run_key.len());
        let crib = Crib::new(run_key.len());
        Self {
            cipher_encoding,
            output_encoding,
            cipher_alphabet,
            output_alphabet,
            cipher_buf,
            output_buf,
            fix_key,
            run_key,
            top_key,
            run_energy: MIN_ENERGY,
            top_energy: MIN_ENERGY,
            crib,
        }
    }

    pub fn crib_char(&mut self, encoded_char: Char, decoded_char: Char) {
        self.fix_key[encoded_char as usize] = decoded_char;
        self.crib.fix(encoded_char as usize);
    }

    pub fn crib_char_at(&mut self, index: usize, decoded_char: Char) {
        let encoded_char = self.cipher_buf[index];
        self.crib_char(encoded_char, decoded_char);
    }

    pub fn crib_str(&mut self, offset: usize, decoded: &str) {
        let crib = self.output_encoding.encode_str(decoded);
        for (i, &c) in crib.iter().enumerate() {
            self.crib_char_at(offset + i, c);
        }
    }

    pub fn climb<REPORT, ENERGY, ACCEPT, RANDOMKEY, DERIVEKEY, MUTATEKEY, CRIB>(
        &mut self,
        config: &Config<REPORT, ENERGY, ACCEPT, RANDOMKEY, DERIVEKEY, MUTATEKEY, CRIB>,
    ) where
        REPORT: Fn(&Climber, usize, usize, usize),
        ENERGY: Fn(&[Char]) -> f64,
        ACCEPT: Fn(f64, f64) -> bool,
        RANDOMKEY: Fn(&mut Climber),
        DERIVEKEY: Fn(&mut Climber),
        MUTATEKEY: Fn(&mut Climber),
        CRIB: Fn(&mut Climber),
    {
        (config.random_key)(self);

        for &index in self.crib.fixed.iter() {
            self.run_key[index] = self.fix_key[index];
        }
        self.run_key.decode(&self.cipher_buf, &mut self.output_buf);

        let mut cycle = 0;
        let mut mutate_cycle = 0;
        let mut derive_cycle = 0;
        let mut accepted = 1;
        let mut rejected = 1;
        while cycle < config.cycle {
            (config.report)(&self, cycle, accepted, rejected);
            for ii in 0..self.crib.loose.len() {
                for jj in ii+1..self.crib.loose.len() {

                    if ii >= self.crib.loose.len() || jj >= self.crib.loose.len() {
                        break;
                    }

                    let (i, j) = (self.crib.loose[ii], self.crib.loose[jj]);

                    if derive_cycle == config.derive_cycle {
                        (config.derive_key)(self);
                        derive_cycle = 0;
                    }
                    if mutate_cycle == config.mutate_cycle {
                        (config.mutate_key)(self);
                        mutate_cycle = 0;
                    }

                    self.run_key.swap(i, j);
                    self.run_key.decode(&self.cipher_buf, &mut self.output_buf);

                    let energy = (config.energy)(&self.output_buf);
                    if !(config.accept)(self.run_energy, energy) {
                        self.run_key.swap(i, j);
                        mutate_cycle += 1;
                        derive_cycle += 1;
                        rejected += 1;
                        continue
                    }
                    accepted += 1;

                    self.run_energy = energy;
                    if self.run_energy > self.top_energy {
                        self.top_key.copy(&self.run_key);
                        self.top_energy = self.run_energy;
                        (config.crib)(self);
                        mutate_cycle = 0;
                        derive_cycle = 0;
                    }
                }
            }
            cycle += 1;
        }
    }
}
