use rand::distributions::Uniform;
use rand::prelude::*;
use std::vec::Vec;

pub struct RandomNumberGenerator {
    rd: StdRng,
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: StdRng::from_entropy(),
        }
    }

    pub fn fetch_uniform(&mut self, from: i32, to: i32, num: usize) -> Vec<i32> {
        let mut uniform_numbers = Vec::new();
        let dist = Uniform::from(from..to);
        for _ in 0..num {
            uniform_numbers.push(dist.sample(&mut self.rd));
        }
        uniform_numbers
    }
}
