use std::collections::{HashMap, HashSet};

pub struct Transposition(Vec<usize>);

impl Transposition {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn from_mapper<F>(len: usize, map: F) -> Result<Self, &'static str>
    where
        F: Fn(usize) -> usize,
    {
        let mut indices = vec![0; len];
        for index in 0..len {
            indices[index] = map(index);
        }

        Self(indices).validate()
    }

    pub fn apply_to<T>(&self, input: &[T], output: &mut [T]) {
        for index in 0..input.len() {
            output[index] = input[self.0[index]];
        }
    }

    pub fn validate(self) -> Result<Self, &'static str> {
        let len = self.0.len();
        let lhs = self.0.iter().enumerate().map(|(i, _)| i).collect::<HashSet<usize>>();
        let rhs = self.0.iter().enumerate().map(|(_, u)| *u).collect::<HashSet<usize>>();
        if rhs.len() != len {
            return Err("Duplicate transposition indices");
        }
        if lhs.difference(&rhs).collect::<HashSet<_>>().len() != 0 {
            return Err("Mismatched transposition indices");
        }
        if rhs.difference(&lhs).collect::<HashSet<_>>().len() != 0 {
            return Err("Mismatched transposition indices");
        }
        for index in 0..len {
            if lhs.get(&index).is_none() {
                return Err("Missing transposition indices");
            }
            if rhs.get(&index).is_none() {
                return Err("Missing transposition indices");
            }
        }
        Ok(self)
    }
}
