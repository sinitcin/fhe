/*
 This code provides generation of a uniform distribution of binary values (modulus 2). Discrete uniform generator
 relies on the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/


use std::rand::Rng;

pub struct BinaryUniformGeneratorImpl<VecType> {
    m_distribution: std::rand::distributions::Bernoulli,
}

impl<VecType> BinaryUniformGeneratorImpl<VecType> {
    pub fn new() -> Self {
        BinaryUniformGeneratorImpl {
            m_distribution: std::rand::distributions::Bernoulli::new(0.5),
        }
    }

    pub fn generate_integer(&self) -> VecType::Integer {
        // Generate a random boolean value and convert it to the appropriate integer type
        let value: bool = self.m_distribution.sample(&mut rand::thread_rng());
        VecType::Integer::from(value as u64)
    }

    pub fn generate_vector(&self, size: usize, modulus: &VecType::Integer) -> VecType {
        let mut rng = rand::thread_rng();
        let mut result = VecType::new(size);
        for i in 0..size {
            result[i] = self.generate_integer();
        }
        result
    }
}
