/*
  This code provides generation of uniform distribution of binary values (modulus 2). Discrete uniform generator relies on
  the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/
use rand::distributions::Bernoulli;
use rand::Rng;

pub struct BinaryUniformGeneratorImpl<VecType> {
    distribution: Bernoulli,
    phantom: std::marker::PhantomData<VecType>,
}

impl<VecType> BinaryUniformGeneratorImpl<VecType> {
    pub fn new() -> Self {
        BinaryUniformGeneratorImpl {
            distribution: Bernoulli::new(0.5).unwrap(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn generate_integer(&self) -> usize {
        let mut rng = rand::thread_rng();
        if rng.sample(&self.distribution) {
            1
        } else {
            0
        }
    }

    pub fn generate_vector(&self, size: usize, modulus: usize) -> Vec<usize> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(self.generate_integer());
        }
        v
    }
}
