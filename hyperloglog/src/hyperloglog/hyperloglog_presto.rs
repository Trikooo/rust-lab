use crate::hyperloglog::Hash;
use crate::hyperloglog::utils::{Vec4, calculate_hash};
use std::cmp::max;
use std::collections::HashMap;
pub struct HyperLogLogPresto {
    initial_bits: u32,
    m: u32,
    dense_buckets: Vec4,
    overflow_buckets: HashMap<usize, u8>,
    cardinality: f64,
}

impl HyperLogLogPresto {
    pub fn new(initial_bits: u32) -> Self {
        let m = 2u32.pow(initial_bits);
        let data_size = (m + 1) / 2;
        let dense_buckets = Vec4 {
            data: vec![0; data_size as usize],
        };

        Self {
            initial_bits,
            m,
            dense_buckets,
            overflow_buckets: HashMap::new(),
            cardinality: 0.0,
        }
    }
    pub fn add_elem(&mut self, value: &impl Hash) {
        let hash_value = calculate_hash(&value);
        let dense_bucket_index = hash_value >> (64 - self.initial_bits);
        let trailing_zeros: u8 = (hash_value.trailing_zeros() + 1) as u8;
        let prev = self.dense_buckets.get(dense_bucket_index as usize);
        self.dense_buckets
            .set(dense_bucket_index as usize, max(prev, trailing_zeros as u8));
        if trailing_zeros > 15 {
            let overflow_nibble: u8 = trailing_zeros >> 4;
            self.overflow_buckets
                .insert(dense_bucket_index as usize, overflow_nibble);
        }
    }
    pub fn get_cardinality(&self) -> f64 {
        self.cardinality
    }
    pub fn compute_cardinality(&mut self) -> f64 {
        let m = self.m as f64;
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
        let num_buckets = self.dense_buckets.len();
        for i in 0..num_buckets {
            let dense_value = self.dense_buckets.get(i);
            let overflow_value = self.overflow_buckets.get(&i).copied().unwrap_or(0) << 4;
            let bucket_value = overflow_value | dense_value;
            summation += 2.0f64.powf(-(bucket_value as f64));
        }
        let cardinality = alpha * m * m / summation;
        self.cardinality = cardinality;
        cardinality
    }
}
