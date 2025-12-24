pub mod utils;
pub mod hyperloglog_presto;
use std::{cmp::max, hash::Hash};
use utils::calculate_hash;

pub struct HyperLogLog {
    registers: Vec<u32>,
    initial_bits: u32,
    cardinality: f64,
}

impl HyperLogLog {
    pub fn new(initial_bits: u32) -> Self {
        let m = 2u32.pow(initial_bits);
        Self {
            registers: vec![0; m as usize],
            initial_bits,
            cardinality: 0.0,
        }
    }

    pub fn get_cardinality(&self) -> f64 {
        self.cardinality
    }

    pub fn add_elem(&mut self, value: &impl Hash) {
        let hash_value = calculate_hash(&value);
        let register_index = hash_value >> (64 - self.initial_bits);
        let shifted_hash = hash_value << self.initial_bits;
        let msb_position = shifted_hash.leading_zeros() + 1;
        let old_register_value = self.registers[register_index as usize];
        let new_register_value = max(old_register_value, msb_position as u32);

        self.registers[register_index as usize] = new_register_value;
    }

    pub fn compute_cardinality(&mut self) -> f64 {
        let m = self.registers.len() as f64;
        let alpha: f64 = if m >= 128.0 {
            0.7213 / (1.0 + 1.079 / m)
        } else if m >= 64.0 {
            0.709
        } else if m >= 32.0 {
            0.697
        } else {
            0.673
        };

        let mut summation = 0.0;
        for &register in &self.registers {
            summation += 2.0f64.powf(-(register as f64));
        }

        let cardinality = alpha * m * m / summation;
        self.cardinality = cardinality;

        cardinality
    }
}
