/*
  This code provides generation of uniform distributions of discrete values. Discrete uniform generator
  relies on the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/
use rand::Rng;
use std::cmp::min;

const DUG_CHUNK_MIN: u32 = 0;
const DUG_CHUNK_WIDTH: u32 = std::u32::BITS;
const DUG_CHUNK_MAX: u32 = std::u32::MAX;

pub struct DiscreteUniformGeneratorImpl<VecType> {
    modulus: VecType::Integer,
    chunks_per_value: u32,
    shift_chunk: u32,
    bound: std::ops::Range<u32>,
}

impl<VecType> DiscreteUniformGeneratorImpl<VecType> {
    pub fn new() -> Self {
        Self {
            modulus: VecType::Integer::default(),
            chunks_per_value: 0,
            shift_chunk: 0,
            bound: DUG_CHUNK_MIN..DUG_CHUNK_MAX,
        }
    }

    pub fn set_modulus(&mut self, modulus: VecType::Integer) {
        self.modulus = modulus;
    }

    pub fn generate_integer(&self) -> VecType::Integer {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.bound.start, self.bound.end)
    }

    pub fn generate_vector(&self, size: u32) -> VecType {
        let mut vec = VecType::new();
        for _ in 0..size {
            vec.push(self.generate_integer());
        }
        vec
    }

    pub fn generate_vector_with_modulus(&self, size: u32, modulus: VecType::Integer) -> VecType {
        let mut vec = VecType::new();
        for _ in 0..size {
            vec.push(self.generate_integer() % modulus);
        }
        vec
    }
}
