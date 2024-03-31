/*
  This file contains mubintvecdyn, a <vector> of buintdyn, with associated modulus and modulo math operators
*/

 // Assuming the functionality of the included headers needs to be replicated in Rust,
// the following Rust code includes necessary crates and modules.
// Note: Rust does not have a direct equivalent of C++'s header guards or conditional compilation based on macros in the same way.
// Conditional compilation in Rust is handled through features in Cargo.toml and cfg attributes.

// In Rust, external crates are added in Cargo.toml, not in the source files.
// Assuming the functionality provided by the C++ headers, you might need crates like `num-bigint`, `serde` for serialization, and others.
// Here is an example of how you might start translating the includes and guards to Rust, focusing on functionality.

// Cargo.toml dependencies (example):
// [dependencies]
// num-bigint = "0.4"
// serde = { version = "1.0", features = ["derive"] }
// thiserror = "1.0"

// Rust does not use include guards or conditional compilation in the same way as C++.
// Features in Rust are enabled in Cargo.toml and checked with cfg attribute.

// Example Rust code starting point:

// main.rs or lib.rs
extern crate num_bigint;
extern crate serde;
extern crate thiserror;

use num_bigint::BigUint;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::collections::VecDeque;
use std::fmt;
use std::io;



extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Zero;
use std::vec::Vec;

struct BigVector {
    data: Vec<BigUint>,
}

impl BigVector {
    fn new() -> BigVector {
        BigVector { data: Vec::new() }
    }

    // Additional methods to manipulate BigVector can be added here
}


use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
enum State {
    GARBAGE,
    INITIALIZED,
}

#[derive(Clone, Debug)]
struct mubintvec<ubint_el_t> {
    m_modulus: ubint_el_t,
    m_modulus_state: State,
    m_data: Vec<ubint_el_t>,
}

impl<ubint_el_t> mubintvec<ubint_el_t> {
    fn new() -> Self {
        Self {
            m_modulus: Default::default(),
            m_modulus_state: State::GARBAGE,
            m_data: Vec::new(),
        }
    }

    fn single(val: ubint_el_t, modulus: ubint_el_t) -> Self {
        let mut vec = Self::new();
        vec.m_data.push(val);
        vec.m_modulus = modulus;
        vec
    }

    fn with_length(length: usize) -> Self {
        Self {
            m_modulus: Default::default(),
            m_modulus_state: State::GARBAGE,
            m_data: vec![Default::default(); length],
        }
    }

    fn with_length_and_modulus(length: usize, modulus: ubint_el_t) -> Self {
        Self {
            m_modulus: modulus,
            m_modulus_state: State::INITIALIZED,
            m_data: vec![Default::default(); length],
        }
    }

    fn with_length_modulus_and_value(length: usize, modulus: ubint_el_t, val: ubint_el_t) -> Self {
        Self {
            m_modulus: modulus,
            m_modulus_state: State::INITIALIZED,
            m_data: vec![val; length],
        }
    }

    fn with_length_modulus_and_string(length: usize, modulus: ubint_el_t, s: &str) -> Self {
        Self {
            m_modulus: ubint_el_t::from_str(s).unwrap(),
            m_modulus_state: State::INITIALIZED,
            m_data: vec![Default::default(); length],
        }
    }

    fn with_vector_string_and_modulus(s: &[String], modulus: ubint_el_t) -> Self {
        Self {
            m_modulus: modulus,
            m_modulus_state: State::INITIALIZED,
            m_data: s.iter().map(|x| ubint_el_t::from_str(x).unwrap()).collect(),
        }
    }

    fn with_vector_string_and_string(s: &[String], modulus: &str) -> Self {
        Self {
            m_modulus: ubint_el_t::from_str(modulus).unwrap(),
            m_modulus_state: State::INITIALIZED,
            m_data: s.iter().map(|x| ubint_el_t::from_str(x).unwrap()).collect(),
        }
    }

    fn set_modulus(&mut self, value: ubint_el_t) {
        self.m_modulus = value;
        self.m_modulus_state = State::INITIALIZED;
    }

    fn set_modulus_string(&mut self, value: &str) {
        self.m_modulus = ubint_el_t::from_str(value).unwrap();
        self.m_modulus_state = State::INITIALIZED;
    }

    fn set_modulus_mubintvec(&mut self, value: &mubintvec<ubint_el_t>) {
        self.m_modulus = value.get_modulus();
        self.m_modulus_state = State::INITIALIZED;
    }

    fn switch_modulus(&mut self, value: ubint_el_t) {
        // TODO: Implement switch_modulus
    }

    fn get_length(&self) -> usize {
        self.m_data.len()
    }

    fn at(&self, i: usize) -> &ubint_el_t {
        if !self.index_check(i) {
            panic!("index out of range");
        }
        &self.m_data[i]
    }

    fn at_mut(&mut self, i: usize) -> &mut ubint_el_t {
        if !self.index_check(i) {
            panic!("index out of range");
        }
        &mut self.m_data[i]
    }

    fn index_check(&self, length: usize) -> bool {
        length < self.m_data.len()
    }

    fn is_modulus_set(&self) -> bool {
        self.m_modulus_state == State::INITIALIZED
    }

    fn get_modulus(&self) -> &ubint_el_t {
        if self.m_modulus_state != State::INITIALIZED {
            panic!("GetModulus() on uninitialized mubintvec");
        }
        &self.m_modulus
    }

    fn mod_eq(&mut self, modulus: ubint_el_t) -> Self {
        // TODO: Implement mod_eq
        Self::new()
    }

    fn mod_add(&self, b: ubint_el_t) -> Self {
        // TODO: Implement mod_add
        Self::new()
    }

    fn mod_add_at_index(&self, i: usize, b: ubint_el_t) -> Self {
        // TODO: Implement mod_add_at_index
        Self::new()
    }

    fn mod_add_no_check_eq(&mut self, b: &mubintvec<ubint_el_t>) -> Self {
        // TODO: Implement mod_add_no_check_eq
        Self::new()
    }

    fn mod_sub(&self, b: ubint_el_t) -> Self {
        // TODO: Implement mod_sub
        Self::new()
    }

    fn mod_sub_eq(&mut self, b: &mubintvec<ubint_el_t>) -> Self {
        // TODO: Implement mod_sub_eq
        Self::new()
    }

    fn mod_mul(&self, b: ubint_el_t) -> Self {
        // TODO: Implement mod_mul
        Self::new()
    }

    fn mod_mul_eq(&mut self, b: &mubintvec<ubint_el_t>) -> Self {
        // TODO: Implement mod_mul_eq
        Self::new()
    }

    fn mod_mul_no_check_eq(&mut self, b: &mubintvec<ubint_el_t>) -> Self {
        // TODO: Implement mod_mul_no_check_eq
        Self::new()
    }

    fn mod_exp(&self, b: ubint_el_t) -> Self {
        // TODO: Implement mod_exp
        Self::new()
    }

    fn mod_exp_eq(&mut self, b: &mubintvec<ubint_el_t>) -> Self {
        // TODO: Implement mod_exp_eq
        Self::new()
    }

    fn mod_inverse(&self) -> Self {
        // TODO: Implement mod_inverse
        Self::new()
    }

    fn mod_inverse_eq(&mut self) -> Self {
        // TODO: Implement mod_inverse_eq
        Self::new()
    }

    fn mod_by_two(&self) -> Self {
        // TODO: Implement mod_by_two
        Self::new()
    }

    fn mod_by_two_eq(&mut self) -> Self {
        // TODO: Implement mod_by_two_eq
        Self::new()
    }

    fn multiply_and_round(&self, p: ubint_el_t, q: ubint_el_t) -> Self {
        // TODO: Implement multiply_and_round
        Self::new()
    }

    fn multiply_and_round_eq(&mut self, p: ubint_el_t, q: ubint_el_t) -> Self {
        // TODO: Implement multiply_and_round_eq
        Self::new()
    }

    fn divide_and_round(&self, q: ubint_el_t) -> Self {
        // TODO: Implement divide_and_round
        Self::new()
    }

    fn divide_and_round_eq(&mut self, q: ubint_el_t) -> Self {
        // TODO: Implement divide_and_round_eq
        Self::new()
    }

    fn get_digit_at_index_for_base(&self, index: usize, base: usize) -> Self {
        // TODO: Implement get_digit_at_index_for_base
        Self::new()
    }
}

impl<ubint_el_t> Index<usize> for mubintvec<ubint_el_t> {
    type Output = ubint_el_t;

    fn index(&self, i: usize) -> &Self::Output {
        self.at(i)
    }
}

impl<ubint_el_t> IndexMut<usize> for mubintvec<ubint_el_t> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.at_mut(i)
    }
}

impl<ubint_el_t> std::fmt::Display for mubintvec<ubint_el_t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.m_data.len();
        write!(f, "[")?;
        for (i, val) in self.m_data.iter().enumerate() {
            write!(f, "{}", val)?;
            if i == len - 1 {
                write!(f, "]")?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, " modulus: {}", self.m_modulus)
    }
}

impl<ubint_el_t> lbcrypto::BigVectorInterface<mubintvec<ubint_el_t>, ubint_el_t> for mubintvec<ubint_el_t> {}

impl<ubint_el_t> lbcrypto::Serializable for mubintvec<ubint_el_t> {
    fn save(&self, ar: &mut dyn lbcrypto::CerealizeArchive) {
        ar.write_named_value("d", &self.m_data);
        ar.write_named_value("m", &self.m_modulus);
        ar.write_named_value("ms", &self.m_modulus_state);
    }

    fn load(&mut self, ar: &mut dyn lbcrypto::CerealizeArchive) {
        let version = ar.get_version();
        if version > Self::serialized_version() {
            panic!(
                "serialized object version {} is from a later version of the library",
                version
            );
        }
        self.m_data = ar.read_named_value("d").unwrap();
        self.m_modulus = ar.read_named_value("m").unwrap();
        self.m_modulus_state = ar.read_named_value("ms").unwrap();
    }

    fn serialized_object_name(&self) -> String {
        "ExpVector".to_string()
    }

    fn serialized_version() -> u32 {
        1
    }
}

use std::ops::{Index, IndexMut};
use std::string::String;
use std::vec::Vec;

pub struct BigInteger {
    // implementation details
}

pub struct mubintvec<T> {
    m_data: Vec<T>,
    m_modulus: T,
    m_modulus_state: State,
}

pub type xmubintvec = mubintvec<BigInteger>;
pub type BigVector = xmubintvec;

impl<T> mubintvec<T> {
    pub fn new() -> Self {
        mubintvec {
            m_data: Vec::new(),
            m_modulus: T::default(),
            m_modulus_state: State::UNINITIALIZED,
        }
    }

    pub fn single(val: T, modulus: T) -> Self {
        let mut vec = mubintvec::new();
        vec.m_data.push(val);
        vec.m_modulus = modulus;
        vec.m_modulus_state = State::INITIALIZED;
        vec
    }

    pub fn with_length(length: usize) -> Self {
        mubintvec {
            m_data: Vec::with_capacity(length),
            m_modulus: T::default(),
            m_modulus_state: State::UNINITIALIZED,
        }
    }

    pub fn with_length_and_modulus(length: usize, modulus: T) -> Self {
        mubintvec {
            m_data: Vec::with_capacity(length),
            m_modulus,
            m_modulus_state: State::INITIALIZED,
        }
    }

    pub fn with_length_modulus_and_value(length: usize, modulus: T, val: T) -> Self {
        mubintvec {
            m_data: vec![val; length],
            m_modulus,
            m_modulus_state: State::INITIALIZED,
        }
    }

    pub fn with_length_and_modulus_string(length: usize, modulus: String) -> Self {
        mubintvec {
            m_data: Vec::with_capacity(length),
            m_modulus: T::from(modulus),
            m_modulus_state: State::INITIALIZED,
        }
    }

    pub fn with_strings(s: Vec<String>, modulus: T) -> Self {
        let mut vec = mubintvec::with_length(s.len());
        for i in 0..s.len() {
            vec.m_data[i] = T::from(s[i].clone());
        }
        vec.m_modulus = modulus;
        vec.m_modulus_state = State::INITIALIZED;
        vec
    }

    pub fn with_strings_and_modulus_string(s: Vec<String>, modulus: String) -> Self {
        let mut vec = mubintvec::with_length(s.len());
        for i in 0..s.len() {
            vec.m_data[i] = T::from(s[i].clone());
        }
        vec.m_modulus = T::from(modulus);
        vec.m_modulus_state = State::INITIALIZED;
        vec
    }

    pub fn set_modulus(&mut self, value: T) {
        self.m_modulus = value;
        self.m_modulus_state = State::INITIALIZED;
    }

    pub fn set_modulus_string(&mut self, value: String) {
        self.m_modulus = T::from(value);
        self.m_modulus_state = State::INITIALIZED;
    }

    pub fn set_modulus_from_vec(&mut self, value: &mubintvec<T>) {
        self.m_modulus = value.m_modulus.clone();
        self.m_modulus_state = State::INITIALIZED;
    }

    pub fn length(&self) -> usize {
        self.m_data.len()
    }

    pub fn at(&self, i: usize) -> &T {
        if !self.index_check(i) {
            panic!("index out of range");
        }
        &self.m_data[i]
    }

    pub fn at_mut(&mut self, i: usize) -> &mut T {
        if !self.index_check(i) {
            panic!("index out of range");
        }
        &mut self.m_data[i]
    }
}

impl<T> Index<usize> for mubintvec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.m_data[i]
    }
}

impl<T> IndexMut<usize> for mubintvec<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.m_data[i]
    }
}

enum State {
    UNINITIALIZED,
    INITIALIZED,
}

use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
struct ubint_el_t {
    // Define the structure of ubint_el_t here
}

#[derive(Clone, Debug)]
enum State {
    GARBAGE,
    INITIALIZED,
}

#[derive(Clone, Debug)]
struct mubintvec {
    m_modulus: ubint_el_t,
    m_modulus_state: State,
    m_data: Vec<ubint_el_t>,
}

impl mubintvec {
    fn new() -> Self {
        mubintvec {
            m_modulus: ubint_el_t::default(),
            m_modulus_state: State::GARBAGE,
            m_data: Vec::new(),
        }
    }

    fn switch_modulus(&self, value: &ubint_el_t) {
        // Implement SwitchModulus function here
    }

    fn get_modulus(&self) -> Result<&ubint_el_t, String> {
        if self.m_modulus_state != State::INITIALIZED {
            Err("GetModulus() on uninitialized mubintvec".to_string())
        } else {
            Ok(&self.m_modulus)
        }
    }

    fn modulus(&self, modulus: &ubint_el_t) -> mubintvec {
        // Implement Mod function here
        mubintvec::new()
    }

    fn modulus_eq(&mut self, modulus: &ubint_el_t) -> &mut mubintvec {
        // Implement ModEq function here
        self
    }

    fn modulus_add(&self, b: &ubint_el_t) -> mubintvec {
        // Implement ModAdd function here
        mubintvec::new()
    }

    fn modulus_add_eq(&mut self, b: &ubint_el_t) -> &mut mubintvec {
        // Implement ModAddEq function here
        self
    }

    fn modulus_add_at_index(&self, i: usize, b: &ubint_el_t) -> mubintvec {
        // Implement ModAddAtIndex function here
        mubintvec::new()
    }

    fn modulus_add_at_index_eq(&mut self, i: usize, b: &ubint_el_t) -> &mut mubintvec {
        // Implement ModAddAtIndexEq function here
        self
    }

    fn modulus_add_vec(&self, b: &mubintvec) -> mubintvec {
        // Implement ModAdd function here
        mubintvec::new()
    }

    fn modulus_add_vec_eq(&mut self, b: &mubintvec) -> &mut mubintvec {
        // Implement ModAddEq function here
        self
    }

    fn modulus_add_no_check_eq(&mut self, b: &mubintvec) -> &mut mubintvec {
        // Implement ModAddNoCheckEq function here
        self
    }

    fn modulus_sub(&self, b: &ubint_el_t) -> mubintvec {
        // Implement ModSub function here
        mubintvec::new()
    }

    fn modulus_sub_eq(&mut self, b: &ubint_el_t) -> &mut mubintvec {
        // Implement ModSubEq function here
        self
    }

    fn modulus_sub_vec(&self, b: &mubintvec) -> mubintvec {
        // Implement ModSub function here
        mubintvec::new()
    }

    fn modulus_sub_vec_eq(&mut self, b: &mubintvec) -> &mut mubintvec {
        // Implement ModSubEq function here
        self
    }

    fn modulus_mul(&self, b: &ubint_el_t) -> mubintvec {
        // Implement ModMul function here
        mubintvec::new()
    }

    fn modulus_mul_eq(&mut self, b: &ubint_el_t) -> &mut mubintvec {
        // Implement ModMulEq function here
        self
    }

    fn modulus_mul_vec(&self, b: &mubintvec) -> mubintvec {
        // Implement ModMul function here
        mubintvec::new()
    }

    fn modulus_mul_vec_eq(&mut self, b: &mubintvec) -> &mut mubintvec {
        // Implement ModMulEq function here
        self
    }

    fn modulus_mul_no_check_eq(&mut self, b: &mubintvec) -> &mut mubintvec {
        // Implement ModMulNoCheckEq function here
        self
    }

    fn modulus_exp(&self, b: &ubint_el_t) -> mubintvec {
        // Implement ModExp function here
        mubintvec::new()
    }

    fn modulus_exp_eq(&mut self, b: &ubint_el_t) -> &mut mubintvec {
        // Implement ModExpEq function here
        self
    }

    fn modulus_inverse(&self) -> mubintvec {
        // Implement ModInverse function here
        mubintvec::new()
    }

    fn modulus_inverse_eq(&mut self) -> &mut mubintvec {
        // Implement ModInverseEq function here
        self
    }

    fn modulus_by_two(&self) -> mubintvec {
        // Implement ModByTwo function here
        mubintvec::new()
    }

    fn modulus_by_two_eq(&mut self) -> &mut mubintvec {
        // Implement ModByTwoEq function here
        self
    }

    fn multiply_and_round(&self, p: &ubint_el_t, q: &ubint_el_t) -> mubintvec {
        // Implement MultiplyAndRound function here
        mubintvec::new()
    }

    fn multiply_and_round_eq(&mut self, p: &ubint_el_t, q: &ubint_el_t) -> &mut mubintvec {
        // Implement MultiplyAndRoundEq function here
        self
    }

    fn divide_and_round(&self, q: &ubint_el_t) -> mubintvec {
        // Implement DivideAndRound function here
        mubintvec::new()
    }

    fn divide_and_round_eq(&mut self, q: &ubint_el_t) -> &mut mubintvec {
        // Implement DivideAndRoundEq function here
        self
    }

    fn get_digit_at_index_for_base(&self, index: usize, base: usize) -> mubintvec {
        // Implement GetDigitAtIndexForBase function here
        mubintvec::new()
    }
}

impl fmt::Display for mubintvec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.m_data.len();
        write!(f, "[")?;
        for (i, val) in self.m_data.iter().enumerate() {
            write!(f, "{}", val)?;
            if i == len - 1 {
                write!(f, "]")?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, " modulus: {}", self.m_modulus)
    }
}

impl Add for mubintvec {
    type Output = mubintvec;

    fn add(self, other: mubintvec) -> mubintvec {
        // Implement Add trait for mubintvec here
        mubintvec::new()
    }
}

impl Sub for mubintvec {
    type Output = mubintvec;

    fn sub(self, other: mubintvec) -> mubintvec {
        // Implement Sub trait for mubintvec here
        mubintvec::new()
    }
}

impl Mul for mubintvec {
    type Output = mubintvec;

    fn mul(self, other: mubintvec) -> mubintvec {
        // Implement Mul trait for mubintvec here
        mubintvec::new()
    }
}

impl Default for ubint_el_t {
    fn default() -> Self {
        // Implement default value for ubint_el_t here
        ubint_el_t {}
    }
}

impl Default for mubintvec {
    fn default() -> Self {
        // Implement default value for mubintvec here
        mubintvec::new()
    }
}

