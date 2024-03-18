/*
  This file contains the vector manipulation functionality
 */

 use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BigVectorFixedT<IntegerType> {
    data: Vec<IntegerType>,
    modulus: IntegerType,
}

impl<IntegerType> BigVectorFixedT<IntegerType>
where
    IntegerType: Clone + From<u64> + for<'a> Add<&'a IntegerType, Output = IntegerType> + for<'a> Sub<&'a IntegerType, Output = IntegerType> + for<'a> Mul<&'a IntegerType, Output = IntegerType> + PartialEq + Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(length: usize, modulus: IntegerType) -> Self {
        BigVectorFixedT {
            data: vec![IntegerType::from(0); length],
            modulus,
        }
    }

    pub fn single(val: IntegerType, modulus: IntegerType) -> Self {
        let mut vec = Self::new(1, modulus);
        vec.data[0] = val;
        vec
    }

    pub fn with_value(length: usize, modulus: IntegerType, value: IntegerType) -> Self {
        BigVectorFixedT {
            data: vec![value; length],
            modulus,
        }
    }

    pub fn set_modulus(&mut self, value: IntegerType) {
        self.modulus = value;
    }

    pub fn get_modulus(&self) -> &IntegerType {
        &self.modulus
    }

    pub fn get_length(&self) -> usize {
        self.data.len()
    }

    pub fn mod_eq(&mut self, modulus: IntegerType) {
        self.modulus = modulus;
        for item in &mut self.data {
            *item = item.clone() % self.modulus.clone();
        }
    }

    pub fn mod_add(&self, other: &IntegerType) -> Self {
        let mut result = self.clone();
        for item in &mut result.data {
            *item = item.clone() + other.clone();
        }
        result
    }

    pub fn mod_add_eq(&mut self, other: &IntegerType) {
        for item in &mut self.data {
            *item = item.clone() + other.clone();
        }
    }

    pub fn mod_sub(&self, other: &IntegerType) -> Self {
        let mut result = self.clone();
        for item in &mut result.data {
            *item = item.clone() - other.clone();
        }
        result
    }

    pub fn mod_sub_eq(&mut self, other: &IntegerType) {
        for item in &mut self.data {
            *item = item.clone() - other.clone();
        }
    }

    pub fn mod_mul(&self, other: &IntegerType) -> Self {
        let mut result = self.clone();
        for item in &mut result.data {
            *item = item.clone() * other.clone();
        }
        result
    }

    pub fn mod_mul_eq(&mut self, other: &IntegerType) {
        for item in &mut self.data {
            *item = item.clone() * other.clone();
        }
    }
}

use serde::{Serialize, Deserialize};
use serde::ser::SerializeStruct;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
struct BigVectorFixedT<IntegerType> {
    #[serde(skip_serializing, skip_deserializing)]
    m_data: Vec<IntegerType>,
    m_length: usize,
    m_modulus: IntegerType,
}

impl<IntegerType> BigVectorFixedT<IntegerType>
where
    IntegerType: Serialize + Deserialize<'static> + Clone + Default,
{
    fn mod_mul(&self, b: &Self) -> Self {
        unimplemented!()
    }

    fn mod_mul_eq(&mut self, b: &Self) -> &mut Self {
        unimplemented!()
    }

    fn mod_mul_no_check_eq(&mut self, b: &Self) -> &mut Self {
        unimplemented!()
    }

    fn mod_exp(&self, b: &IntegerType) -> Self {
        unimplemented!()
    }

    fn mod_exp_eq(&mut self, b: &IntegerType) -> &mut Self {
        unimplemented!()
    }

    fn mod_inverse(&self) -> Self {
        unimplemented!()
    }

    fn mod_inverse_eq(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn mod_by_two(&self) -> Self {
        unimplemented!()
    }

    fn mod_by_two_eq(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn mult_with_out_mod(&self, b: &Self) -> Self {
        unimplemented!()
    }

    fn mult_with_out_mod_eq(&mut self, b: &Self) -> &mut Self {
        unimplemented!()
    }

    fn multiply_and_round(&self, p: &IntegerType, q: &IntegerType) -> Self {
        unimplemented!()
    }

    fn multiply_and_round_eq(&mut self, p: &IntegerType, q: &IntegerType) -> &mut Self {
        unimplemented!()
    }

    fn divide_and_round(&self, q: &IntegerType) -> Self {
        unimplemented!()
    }

    fn divide_and_round_eq(&mut self, q: &IntegerType) -> &mut Self {
        unimplemented!()
    }

    fn get_digit_at_index_for_base(&self, index: usize, base: usize) -> Self {
        unimplemented!()
    }
}

impl<IntegerType> fmt::Display for BigVectorFixedT<IntegerType>
where
    IntegerType: fmt::Display + Serialize + Deserialize<'static> + Clone + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, val) in self.m_data.iter().enumerate() {
            if i > 0 { write!(f, " ")?; }
            write!(f, "{}", val)?;
        }
        write!(f, "] modulus: {}", self.m_modulus)
    }
}

impl<IntegerType> BigVectorFixedT<IntegerType>
where
    IntegerType: Serialize + Deserialize<'static> + Clone + Default,
{
    fn serialized_object_name() -> &'static str {
        "FXDInteger"
    }

    fn serialized_version() -> u32 {
        1
    }
}



