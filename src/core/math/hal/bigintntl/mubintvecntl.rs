/*
  This file contains mgmpintvec, a <vector> of gmpint, with associated math operators.
  NOTE: this has been refactored so that implied modulo (ring) aritmetic is in mbintvec
 */


use num_bigint::BigInt;
use num_traits::Zero;
use std::convert::TryInto;
use std::ops::{AddAssign, SubAssign, MulAssign};

pub struct myVecP<T> {
    vec: Vec<T>,
    modulus_state: ModulusState,
    modulus: T,
}

enum ModulusState {
    Garbage,
    Initialized,
}

impl<T> myVecP<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            modulus_state: ModulusState::Garbage,
            modulus: T::zero(),
        }
    }

    pub fn single(val: T, modulus: T) -> Self {
        let mut vec = Self::new();
        vec.set_modulus(modulus);
        vec.vec.push(val);
        vec
    }

    pub fn with_length(length: usize) -> Self {
        Self {
            vec: Vec::with_capacity(length),
            modulus_state: ModulusState::Garbage,
            modulus: T::zero(),
        }
    }

    pub fn with_length_type(length: usize) -> Self {
        Self {
            vec: Vec::with_capacity(length),
            modulus_state: ModulusState::Garbage,
            modulus: T::zero(),
        }
    }

    pub fn with_value(length: usize, modulus: T, value: T) -> Self {
        let mut vec = Self::with_length(length);
        vec.set_modulus(modulus);
        vec.vec.resize(length, value);
        vec
    }

    pub fn with_value_string(length: usize, modulus: T, rhs: Vec<String>) -> Self {
        let mut vec = Self::with_length(length);
        vec.set_modulus(modulus);
        for i in 0..length {
            vec.vec[i] = rhs[i].parse().unwrap();
        }
        vec
    }

    pub fn with_value_uint64(length: usize, modulus: T, rhs: Vec<u64>) -> Self {
        let mut vec = Self::with_length(length);
        vec.set_modulus(modulus);
        for i in 0..length {
            vec.vec[i] = rhs[i].try_into().unwrap();
        }
        vec
    }

    pub fn with_modulus(a: &myVecP<T>, q: T) -> Self {
        let mut vec = Self::new();
        vec.vec = a.vec.clone();
        vec.set_modulus(q);
        vec
    }

    pub fn with_modulus_string(a: &myVecP<T>, sq: &str) -> Self {
        let mut vec = Self::new();
        vec.vec = a.vec.clone();
        vec.set_modulus(sq.parse().unwrap());
        vec
    }

    pub fn with_modulus_uint64(a: &myVecP<T>, q: u64) -> Self {
        let mut vec = Self::new();
        vec.vec = a.vec.clone();
        vec.set_modulus(q.try_into().unwrap());
        vec
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn at(&self, i: usize) -> &T {
        &self.vec[i]
    }

    pub fn at_mut(&mut self, i: usize) -> &mut T {
        &mut self.vec[i]
    }

    pub fn push(&mut self, a: T) {
        self.vec.push(a);
    }

    pub fn switch_modulus(&mut self, new_modulus: T) {
        self.set_modulus(new_modulus);
    }

    pub fn is_modulus_set(&self) -> bool {
        self.modulus_state == ModulusState::Initialized
    }

    pub fn same_modulus(&self, a: &myVecP<T>) -> bool {
        self.modulus_state == a.modulus_state && self.modulus == a.modulus
    }

    pub fn set_modulus(&mut self, value: T) {
        if value.is_zero() {
            panic!("SetModulus cannot be zero");
        }
        self.modulus = value;
        self.modulus_state = ModulusState::Initialized;
    }

    pub fn get_modulus(&self) -> &T {
        if self.is_modulus_set() {
            &self.modulus
        } else {
            panic!("modulus not set");
        }
    }

    pub fn copy_modulus(&mut self, rhs: &myVecP<T>) -> i32 {
        self.modulus = rhs.modulus;
        self.modulus_state = rhs.modulus_state;
        if self.is_modulus_set() {
            0
        } else {
            self.modulus_state = ModulusState::Garbage;
            -1
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn resize(&mut self, n: usize) {
        self.vec.resize(n, T::zero());
    }

    pub fn mod_add(&self, b: &T) -> myVecP<T> {
        self.modulus_check("Warning: myVecP::ModAdd");
        let mut ans = self.clone();
        ans.mod_add_eq(b);
        ans
    }

    pub fn mod_add_eq(&mut self, b: &T) {
        self.modulus_check("Warning: myVecP::ModAdd");
        for i in 0..self.len() {
            self.vec[i] += b;
            self.vec[i] %= &self.modulus;
        }
    }

    pub fn mod_add_at_index(&self, i: usize, b: &T) -> myVecP<T> {
        let mut ans = self.clone();
        ans.mod_add_at_index_eq(i, b);
        ans
    }

    pub fn mod_add_at_index_eq(&mut self, i: usize, b: &T) {
        self.modulus_check("Warning: myVecP::ModAdd");
        self.vec[i] += b;
        self.vec[i] %= &self.modulus;
    }

    pub fn mod_add_vec(&self, b: &myVecP<T>) -> myVecP<T> {
        self.arg_check_vector("myVecP ModAdd()");
        let mut ans = self.clone();
        ans.mod_add_vec_eq(b);
        ans
    }

    pub fn mod_add_vec_eq(&mut self, b: &myVecP<T>) {
        self.arg_check_vector("myVecP ModAddEq()");
        for i in 0..self.len() {
            self.vec[i] += &b.vec[i];
            self.vec[i] %= &self.modulus;
        }
    }

    pub fn mod_add_no_check_vec_eq(&mut self, b: &myVecP<T>) {
        for i in 0..self.len() {
            self.vec[i] += &b.vec[i];
            self.vec[i] %= &self.modulus;
        }
    }

    pub fn mod_sub(&self, b: &T) -> myVecP<T> {
        self.modulus_check("Warning: myVecP::ModSub");
        let mut ans = self.clone();
        ans.mod_sub_eq(b);
        ans
    }

    pub fn mod_sub_eq(&mut self, b: &T) {
        self.modulus_check("Warning: myVecP::ModSubEq");
        for i in 0..self.len() {
            self.vec[i] -= b;
            self.vec[i] %= &self.modulus;
        }
    }

    pub fn mod_sub_vec(&self, b: &myVecP<T>) -> myVecP<T> {
        self.arg_check_vector("myVecP ModSub()");
        let mut ans = self.clone();
        ans.mod_sub_vec_eq(b);
        ans
    }

    pub fn mod_sub_vec_eq(&mut self, b: &myVecP<T>) {
        self.arg_check_vector("myVecP ModSubEq()");
        for i in 0..self.len() {
            self.vec[i] -= &b.vec[i];
            self.vec[i] %= &self.modulus;
        }
    }

    pub fn mod_mul(&self, b: &T) -> myVecP<T> {
        self.modulus_check("Warning: myVecP::ModMul");
        let mut ans = self.clone();
        ans.mod_mul_eq(b);
        ans
    }

    pub fn mod_mul_eq(&mut self, b: &T) {
        self.modulus_check("Warning: myVecP::ModMul");
        for i in 0..self.len() {
            self.vec[i] *= b;
            self.vec[i] %= &self.modulus;
        }
    }

    fn modulus_check(&self, warning: &str) {
        if !self.is_modulus_set() {
            panic!(warning);
        }
    }

    fn arg_check_vector(&self, warning: &str) {
        if self.len() != b.len() {
            panic!(warning);
        }
    }
}

impl<T: Clone> Clone for myVecP<T> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            modulus_state: self.modulus_state,
            modulus: self.modulus.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for myVecP<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
            && self.modulus_state == other.modulus_state
            && self.modulus == other.modulus
    }
}

impl<T: Eq> Eq for myVecP<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for myVecP<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("myVecP")
            .field("vec", &self.vec)
            .field("modulus_state", &self.modulus_state)
            .field("modulus", &self.modulus)
            .finish()
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> AddAssign<&myVecP<T>> for myVecP<T> {
    fn add_assign(&mut self, other: &myVecP<T>) {
        self.mod_add_vec_eq(other);
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> SubAssign<&myVecP<T>> for myVecP<T> {
    fn sub_assign(&mut self, other: &myVecP<T>) {
        self.mod_sub_vec_eq(other);
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> MulAssign<&myVecP<T>> for myVecP<T> {
    fn mul_assign(&mut self, other: &myVecP<T>) {
        self.mod_mul_vec_eq(other);
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> AddAssign<T> for myVecP<T> {
    fn add_assign(&mut self, other: T) {
        self.mod_add_eq(&other);
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> SubAssign<T> for myVecP<T> {
    fn sub_assign(&mut self, other: T) {
        self.mod_sub_eq(&other);
    }
}

impl<T: Zero + AddAssign + SubAssign + MulAssign + Clone> MulAssign<T> for myVecP<T> {
    fn mul_assign(&mut self, other: T) {
        self.mod_mul_eq(&other);
    }
}


use std::fmt;
use std::ops::{Index, IndexMut};

struct myVecP<T> {
    data: Vec<T>,
    modulus: T,
    modulus_state: ModulusState,
}

impl<T> myVecP<T> {
    fn new(data: Vec<T>, modulus: T) -> Self {
        Self {
            data,
            modulus,
            modulus_state: ModulusState::INITIALIZED,
        }
    }

    fn modulus_check(&self, msg: &str) {
        if self.modulus_state != ModulusState::INITIALIZED {
            panic!(msg.to_string() + " uninitialized this->modulus");
        }
    }

    fn arg_check_vector(&self, b: &myVecP<T>, fname: &str) {
        if self.modulus != b.modulus {
            panic!(fname.to_string() + " modulus vector modulus vector op of different moduli");
        } else if self.modulus_state != ModulusState::INITIALIZED {
            panic!(fname.to_string() + " modulus vector modulus vector op  GARBAGE  moduli");
        } else if self.data.len() != b.data.len() {
            panic!(fname.to_string() + " vectors of different lengths");
        }
    }

    fn renormalize(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] %= self.modulus;
        }
    }

    fn index_check(&self, index: usize) -> bool {
        index < self.data.len()
    }
}

impl<T: fmt::Display> fmt::Display for myVecP<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            write!(f, "{}", item)?;
            if i != self.data.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "] modulus: {}", self.modulus)
    }
}

impl<T> Index<usize> for myVecP<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for myVecP<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

enum ModulusState {
    GARBAGE,
    INITIALIZED,
}

fn mod_mul<T>(a: &myVecP<T>, b: &myVecP<T>) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    a.arg_check_vector(b, "myVecP Mul()");
    let mut ans = a.clone();
    ans.mod_mul_eq(b);
    ans
}

fn mod_mul_eq<T>(a: &mut myVecP<T>, b: &myVecP<T>)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    a.arg_check_vector(b, "myVecP Mul()");
    for i in 0..a.data.len() {
        a.data[i] *= b.data[i];
        a.data[i] %= a.modulus;
    }
}

fn mod_mul_no_check_eq<T>(a: &mut myVecP<T>, b: &myVecP<T>)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    for i in 0..a.data.len() {
        a.data[i] *= b.data[i];
        a.data[i] %= a.modulus;
    }
}

fn mod_exp<T>(a: &myVecP<T>, b: &T) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModExp function
}

fn mod_exp_eq<T>(a: &mut myVecP<T>, b: &T)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModExpEq function
}

fn mod_inverse<T>(a: &myVecP<T>) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModInverse function
}

fn mod_inverse_eq<T>(a: &mut myVecP<T>)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModInverseEq function
}

fn mod_by_two<T>(a: &myVecP<T>) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModByTwo function
}

fn mod_by_two_eq<T>(a: &mut myVecP<T>)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for ModByTwoEq function
}

fn multiply_and_round<T>(a: &myVecP<T>, p: &T, q: &T) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for MultiplyAndRound function
}

fn multiply_and_round_eq<T>(a: &mut myVecP<T>, p: &T, q: &T)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for MultiplyAndRoundEq function
}

fn divide_and_round<T>(a: &myVecP<T>, q: &T) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for DivideAndRound function
}

fn divide_and_round_eq<T>(a: &mut myVecP<T>, q: &T)
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for DivideAndRoundEq function
}

fn get_digit_at_index_for_base<T>(a: &myVecP<T>, index: usize, base: usize) -> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    // implementation for GetDigitAtIndexForBase function
}

impl<T: fmt::Display> fmt::Display for myVecP<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            write!(f, "{}", item)?;
            if i != self.data.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "] modulus: {}", self.modulus)
    }
}

impl<T: Clone> Clone for myVecP<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            modulus: self.modulus,
            modulus_state: self.modulus_state,
        }
    }
}

impl<T> myVecP<T>
where
    T: std::ops::MulAssign + std::ops::RemAssign + Copy,
{
    fn mod_mul_eq(&mut self, b: &myVecP<T>) {
        self.arg_check_vector(b, "myVecP Mul()");
        for i in 0..self.data.len() {
            self.data[i] *= b.data[i];
            self.data[i] %= self.modulus;
        }
    }
}

impl<T> Index<usize> for myVecP<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for myVecP<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: fmt::Display> fmt::Display for myVecP<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            write!(f, "{}", item)?;
            if i != self.data.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "] modulus: {}", self.modulus)
    }
}

fn main() {
    // example usage of the myVecP struct and functions
    let a = myVecP::new(vec![1, 2, 3], 5);
    let b = myVecP::new(vec![4, 5, 6], 5);
    let c = mod_mul(&a, &b);
    println!("{}", c);
}


