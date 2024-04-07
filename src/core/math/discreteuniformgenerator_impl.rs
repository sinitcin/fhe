/*
  This code provides generation of uniform distributions of discrete values. Discrete uniform generator relies on
  the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/
use rand::Rng;

pub struct DiscreteUniformGeneratorImpl<VecType> {
    modulus: VecType::Integer,
    chunks_per_value: usize,
    shift_chunk: usize,
    bound: (u32, u32),
}

impl<VecType> DiscreteUniformGeneratorImpl<VecType> {
    pub fn new(modulus: VecType::Integer) -> Self {
        Self {
            modulus,
            chunks_per_value: 0,
            shift_chunk: 0,
            bound: (0, 0),
        }
    }

    pub fn set_modulus(&mut self, modulus: VecType::Integer) {
        self.modulus = modulus;
        self.chunks_per_value = (self.modulus.get_msb() - 1) / DUG_CHUNK_WIDTH;
        self.shift_chunk = self.chunks_per_value * DUG_CHUNK_WIDTH;
        self.bound = (
            DUG_CHUNK_MIN,
            (self.modulus >> self.shift_chunk).convert_to_int(),
        );
    }

    pub fn generate_integer(&self) -> VecType::Integer {
        if self.modulus == VecType::Integer::from(0) {
            panic!("0 modulus?");
        }
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(DUG_CHUNK_MIN, DUG_CHUNK_MAX);
        loop {
            let mut result = VecType::Integer::from(0);
            let mut shift = 0;
            for _ in 0..self.chunks_per_value {
                result += VecType::Integer::from(rng.sample(dist)) << shift;
                shift += DUG_CHUNK_WIDTH;
            }
            result += VecType::Integer::from(rng.sample(dist)) << self.shift_chunk;
            if result < self.modulus {
                return result;
            }
        }
    }

    pub fn generate_vector(&self, size: u32) -> VecType {
        let mut v = VecType::new(size, self.modulus);
        for i in 0..size {
            v[i as usize] = self.generate_integer();
        }
        v
    }

    pub fn generate_vector_with_modulus(
        &mut self,
        size: u32,
        modulus: VecType::Integer,
    ) -> VecType {
        self.set_modulus(modulus);
        let mut v = VecType::new(size, self.modulus);
        for i in 0..size {
            v[i as usize] = self.generate_integer();
        }
        v
    }
}
