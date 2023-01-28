use rand::prelude::*;
use rand::distributions::Uniform;
use std::vec::Vec;

pub struct RandomNumberGenerator {
    rd: ThreadRng,
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: rand::thread_rng(),
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