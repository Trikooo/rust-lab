use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub fn calculate_hash(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub struct Vec4 {
    pub data: Vec<u8>,
}

impl Vec4 {
    pub fn get(&self, i: usize) -> u8 {
        let byte = self.data[i / 2];
        if i % 2 == 0 {
            byte >> 4
        } else {
            byte & 0b00001111
        }
    }

    pub fn set(&mut self, i: usize, value: u8) {
        let v = value & 0b00001111;
        let byte = &mut self.data[i / 2];
        if i % 2 == 0 {
            *byte = (v << 4) | (*byte & 0b00001111);
        } else {
            *byte = (*byte & 0b11110000) | v;
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() * 2
    }
}
