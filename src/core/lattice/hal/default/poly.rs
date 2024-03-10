use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait PolyInterface<T>: Clone + Debug + PartialEq + From<Vec<T>> {
    type DggType: Debug;
    type DugType: Debug;
    type TugType: Debug;
    type BugType: Debug;

    fn get_format(&self) -> Format;
    fn set_format(&mut self, format: Format);
    fn get_params(&self) -> &Params;
    fn get_values(&self) -> &Vec<T>;
    fn set_values(&mut self, values: Vec<T>, format: Format);
    fn set_values_to_zero(&mut self);
    fn set_values_to_max(&mut self);
    fn plus(&self, rhs: &Self) -> Self;
    fn plus_no_check(&self, rhs: &Self) -> Self;
    fn minus(&self, rhs: &Self) -> Self;
    fn times(&self, rhs: &Self) -> Self;
    fn times_no_check(&self, rhs: &Self) -> Self;
    fn multiply_and_round(&self, p: &T, q: &T) -> Self;
    fn divide_and_round(&self, q: &T) -> Self;
    fn negate(&self) -> Self;
    fn add_i_lelement_one(&mut self);
    fn automorphism_transform(&self, k: u32) -> Self;
    fn automorphism_transform_with_vec(&self, k: u32, vec: &Vec<u32>) -> Self;
    fn multiplicative_inverse(&self) -> Self;
    fn mod_by_two(&self) -> Self;
    fn Mod(&self, modulus: &T) -> Self;
    fn switch_modulus(&mut self, modulus: &T, root_of_unity: &T, modulus_arb: &T, root_of_unity_arb: &T);
    fn switch_format(&mut self);
    fn MakeSparse(&mut self, w_factor: u32);
    fn InverseExists(&self) -> bool;
    fn Norm(&self) -> f64;
    fn BaseDecompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self>;
    fn PowersOfBase(&self, base_bits: usize) -> Vec<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    EVALUATION,
    COEFFICIENT,
}

#[derive(Clone, Debug)]
pub struct Params {
    // TODO: Define the fields of Params
}

#[derive(Clone, Debug)]
pub struct PolyImpl<T> {
    format: Format,
    params: Params,
    values: Vec<T>,
}

impl<T> PolyImpl<T> {
    pub fn new(params: Params, format: Format, initialize_element_to_zero: bool) -> Self {
        let values = if initialize_element_to_zero {
            vec![T::zero(); params.ring_dimension]
        } else {
            vec![]
        };
        Self {
            format,
            params,
            values,
        }
    }

    pub fn new_with_values(params: Params, format: Format, values: Vec<T>) -> Self {
        Self {
            format,
            params,
            values,
        }
    }

    pub fn set_values(&mut self, values: Vec<T>, format: Format) {
        self.values = values;
        self.format = format;
    }

    pub fn set_values_to_zero(&mut self) {
        self.values = vec![T::zero(); self.params.ring_dimension];
    }

    pub fn set_values_to_max(&mut self) {
        let max = self.params.modulus - T::one();
        self.values = vec![max; self.params.ring_dimension];
    }

    pub fn plus(&self, rhs: &Self) -> Self {
        assert_eq!(self.params.ring_dimension, rhs.params.ring_dimension);
        assert_eq!(self.params.modulus, rhs.params.modulus);
        assert_eq!(self.format, rhs.format);
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn plus_no_check(&self, rhs: &Self) -> Self {
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn minus(&self, rhs: &Self) -> Self {
        assert_eq!(self.params.ring_dimension, rhs.params.ring_dimension);
        assert_eq!(self.params.modulus, rhs.params.modulus);
        assert_eq!(self.format, rhs.format);
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn times(&self, rhs: &Self) -> Self {
        assert_eq!(self.params.ring_dimension, rhs.params.ring_dimension);
        assert_eq!(self.params.modulus, rhs.params.modulus);
        assert_eq!(self.format, Format::EVALUATION);
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(a, b)| a * b)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn times_no_check(&self, rhs: &Self) -> Self {
        let values = self
            .values
            .iter()
            .zip(rhs.values.iter())
            .map(|(a, b)| a * b)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn multiply_and_round(&self, p: &T, q: &T) -> Self {
        let values = self
            .values
            .iter()
            .map(|a| a * p * q)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn divide_and_round(&self, q: &T) -> Self {
        let values = self
            .values
            .iter()
            .map(|a| a / q)
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn negate(&self) -> Self {
        let values = self.values.iter().map(|a| -a).collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn add_il_element_one(&mut self) {
        self.values[0] += T::one();
    }

    pub fn automorphism_transform(&self, k: u32) -> Self {
        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(i, a)| a * self.params.root_of_unity.pow(i as u32 * k))
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn automorphism_transform_with_vec(&self, k: u32, vec: &Vec<u32>) -> Self {
        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(i, a)| a * self.params.root_of_unity.pow(vec[i] as u32 * k))
            .collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn multiplicative_inverse(&self) -> Self {
        let values = self.values.iter().map(|a| a.multiplicative_inverse()).collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn mod_by_two(&self) -> Self {
        let values = self.values.iter().map(|a| a.mod_by_two()).collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn mod_(&self, modulus: &T) -> Self {
        let values = self.values.iter().map(|a| a.mod_(modulus)).collect();
        Self {
            format: self.format,
            params: self.params.clone(),
            values,
        }
    }

    pub fn switch_modulus(
        &mut self,
        modulus: &T,
        root_of_unity: &T,
        modulus_arb: &T,
        root_of_unity_arb: &T,
    ) {
        self.params.modulus = modulus.clone();
        self.params.root_of_unity = root_of_unity.clone();
        self.params.modulus_arb = modulus_arb.clone();
        self.params.root_of_unity_arb = root_of_unity_arb.clone();
    }

    pub fn switch_format(&mut self) {
        self.format = match self.format {
            Format::EVALUATION => Format::COEFFICIENT,
            Format::COEFFICIENT => Format::EVALUATION,
        };
    }

    pub fn make_sparse(&mut self, w_factor: u32) {
        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(i, a)| {
                if i % w_factor == 0 {
                    *a
                } else {
                    T::zero()
                }
            })
            .collect();
        self.values = values;
    }

    pub fn inverse_exists(&self) -> bool {
        self.values.iter().all(|a| a.inverse_exists())
    }

    pub fn norm(&self) -> f64 {
        self.values.iter().map(|a| a.norm()).sum()
    }

    pub fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self> {
        let mut result = vec![];
        let mut values = self.values.clone();
        let base = T::from(1 << base_bits);
        while !values.is_empty() {
            let mut value = T::zero();
            for _ in 0..base_bits {
                value *= base;
                value += values.pop().unwrap();
            }
            result.push(Self {
                format: self.format,
                params: self.params.clone(),
                values: vec![value],
            });
        }
        if eval_mode_answer {
            result.reverse();
        }
        result
    }

    pub fn powers_of_base(&self, base_bits: usize) -> Vec<Self> {
        let mut result = vec![];
        let base = T::from(1 << base_bits);
        let mut value = T::one();
        for _ in 0..self.params.ring_dimension {
            result.push(Self {
                format: self.format,
                params: self.params.clone(),
                values: vec![value],
            });
            value *= base;
        }
        result
    }
}

impl<T> Add for PolyImpl<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl<T> AddAssign for PolyImpl<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}

impl<T> Sub for PolyImpl<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl<T> SubAssign for PolyImpl<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(&rhs);
    }
}

impl<T> Mul for PolyImpl<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}

impl<T> MulAssign for PolyImpl<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.times(&rhs);
    }
}

impl<T> Div for PolyImpl<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide_and_round(&rhs)
    }
}

impl<T> DivAssign for PolyImpl<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = self.divide_and_round(&rhs);
    }
}

impl<T> Neg for PolyImpl<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl<T> PartialEq for PolyImpl<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.params.root_of_unity == other.params.root_of_unity
            && self.values == other.values
    }
}

impl<T> From<Vec<T>> for PolyImpl<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            format: Format::EVALUATION,
            params: Params::default(),
            values,
        }
    }
}


