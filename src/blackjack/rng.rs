use rand::prelude::*;
use rand::distributions::Uniform;
use std::vec::Vec;

pub struct RandomNumberGenerator {
    rd: StdRng,
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: StdRng::from_entropy(),
        }
    }

    pub fn fetch_uniform(&mut self, from: i32, to: i32, num: usize) -> Vec<i32> {
        let mut uniform_numbers = Vec::new();
        let dist = Uniform::new(from, to);
        for _ in 0..num {
            uniform_numbers.push(dist.sample(&mut self.rd));
        }
        uniform_numbers
    }
}