/*
 This code provides generation of uniform distributions of discrete values. Discrete uniform generator relies on
 the built-in Rust generator for 32-bit unsigned integers defined in rand crate
*/

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp::Ordering;

pub struct DiscreteUniformGeneratorImpl<VecType> {
    m_modulus: VecType::Integer,
    m_chunksPerValue: u32,
    m_shiftChunk: u32,
    m_bound: Uniform<u32>,
}

impl<VecType> DiscreteUniformGeneratorImpl<VecType>
where
    VecType: VecTypeTrait,
{
    pub fn new(modulus: VecType::Integer) -> Self {
        let mut generator = Self {
            m_modulus: modulus.clone(),
            m_chunksPerValue: 0,
            m_shiftChunk: 0,
            m_bound: Uniform::new(0, 1), // Placeholder, will be set in SetModulus
        };
        generator.set_modulus(modulus);
        generator
    }

    pub fn set_modulus(&mut self, modulus: VecType::Integer) {
        self.m_modulus = modulus;

        // Get the number of chunks in the modulus
        // 1 is subtracted to make sure the last chunk is fully used by the modulus
        self.m_chunksPerValue = (self.m_modulus.get_msb() - 1) / DUG_CHUNK_WIDTH;

        self.m_shiftChunk = self.m_chunksPerValue * DUG_CHUNK_WIDTH;

        self.m_bound = Uniform::new(
            DUG_CHUNK_MIN,
            (self.m_modulus.clone() >> self.m_shiftChunk).convert_to_int(),
        );
    }

    pub fn generate_integer(&self) -> VecType::Integer {
        if self.m_modulus == VecType::Integer::zero() {
            panic!("0 modulus?");
        }

        let dist = Uniform::new(DUG_CHUNK_MIN, DUG_CHUNK_MAX);
        let mut rng = rand::thread_rng();

        loop {
            let mut result = VecType::Integer::zero();
            for i in 0..self.m_chunksPerValue {
                let shift = i * DUG_CHUNK_WIDTH;
                result = result + (VecType::Integer::from(dist.sample(&mut rng)) << shift);
            }
            result = result
                + (VecType::Integer::from(self.m_bound.sample(&mut rng)) << self.m_shiftChunk);

            if result < self.m_modulus {
                return result;
            }
        }
    }

    pub fn generate_vector(&self, size: u32) -> VecType {
        let mut v = VecType::new(size, self.m_modulus.clone());
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
        let mut v = VecType::new(size, self.m_modulus.clone());
        for i in 0..size {
            v[i as usize] = self.generate_integer();
        }
        v
    }
}

pub trait VecTypeTrait {
    type Integer: Clone
        + PartialOrd
        + From<u32>
        + std::ops::Shl<u32, Output = Self::Integer>
        + std::ops::Add<Output = Self::Integer>
        + std::ops::AddAssign
        + std::ops::Sub<Output = Self::Integer>
        + std::ops::SubAssign
        + std::ops::Div<Output = Self::Integer>
        + std::ops::DivAssign
        + std::ops::Rem<Output = Self::Integer>
        + std::ops::RemAssign
        + std::ops::BitAnd<Output = Self::Integer>
        + std::ops::BitAndAssign
        + std::ops::BitOr<Output = Self::Integer>
        + std::ops::BitOrAssign
        + std::ops::BitXor<Output = Self::Integer>
        + std::ops::BitXorAssign
        + std::ops::Not<Output = Self::Integer>
        + std::fmt::Debug;

    fn new(size: u32, modulus: Self::Integer) -> Self;
    fn get_msb(&self) -> u32;
    fn convert_to_int(&self) -> u32;
}

const DUG_CHUNK_WIDTH: u32 = 32;
const DUG_CHUNK_MIN: u32 = 0;
const DUG_CHUNK_MAX: u32 = u32::MAX;
