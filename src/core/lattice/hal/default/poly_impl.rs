use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Integer {
    value: i64,
}

impl Integer {
    fn new(value: i64) -> Self {
        Integer { value }
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::new(value)
    }
}

impl From<Integer> for i64 {
    fn from(integer: Integer) -> Self {
        integer.value
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Integer {
    type Output = Integer;

    fn add(self, other: Integer) -> Integer {
        Integer::new(self.value + other.value)
    }
}

impl AddAssign for Integer {
    fn add_assign(&mut self, other: Integer) {
        self.value += other.value;
    }
}

impl Sub for Integer {
    type Output = Integer;

    fn sub(self, other: Integer) -> Integer {
        Integer::new(self.value - other.value)
    }
}

impl SubAssign for Integer {
    fn sub_assign(&mut self, other: Integer) {
        self.value -= other.value;
    }
}

impl Mul for Integer {
    type Output = Integer;

    fn mul(self, other: Integer) -> Integer {
        Integer::new(self.value * other.value)
    }
}

impl MulAssign for Integer {
    fn mul_assign(&mut self, other: Integer) {
        self.value *= other.value;
    }
}

impl Div for Integer {
    type Output = Integer;

    fn div(self, other: Integer) -> Integer {
        Integer::new(self.value / other.value)
    }
}

impl DivAssign for Integer {
    fn div_assign(&mut self, other: Integer) {
        self.value /= other.value;
    }
}

impl Rem for Integer {
    type Output = Integer;

    fn rem(self, other: Integer) -> Integer {
        Integer::new(self.value % other.value)
    }
}

impl RemAssign for Integer {
    fn rem_assign(&mut self, other: Integer) {
        self.value %= other.value;
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        Integer::new(-self.value)
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl FromStr for Integer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(value) => Ok(Integer::new(value)),
            Err(_) => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct VecType {
    values: Vec<Integer>,
}

impl VecType {
    fn new(values: Vec<Integer>) -> Self {
        VecType { values }
    }
}

impl From<Vec<Integer>> for VecType {
    fn from(values: Vec<Integer>) -> Self {
        VecType::new(values)
    }
}

impl From<VecType> for Vec<Integer> {
    fn from(vec_type: VecType) -> Self {
        vec_type.values
    }
}

impl fmt::Display for VecType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self.values.iter().map(|x| x.to_string()).collect();
        write!(f, "{:?}", values)
    }
}

impl Add for VecType {
    type Output = VecType;

    fn add(self, other: VecType) -> VecType {
        let values: Vec<Integer> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() + y.clone())
            .collect();
        VecType::new(values)
    }
}

impl AddAssign for VecType {
    fn add_assign(&mut self, other: VecType) {
        self.values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() + y.clone())
            .collect();
    }
}

impl Sub for VecType {
    type Output = VecType;

    fn sub(self, other: VecType) -> VecType {
        let values: Vec<Integer> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() - y.clone())
            .collect();
        VecType::new(values)
    }
}

impl SubAssign for VecType {
    fn sub_assign(&mut self, other: VecType) {
        self.values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() - y.clone())
            .collect();
    }
}

impl Mul for VecType {
    type Output = VecType;

    fn mul(self, other: VecType) -> VecType {
        let values: Vec<Integer> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() * y.clone())
            .collect();
        VecType::new(values)
    }
}

impl MulAssign for VecType {
    fn mul_assign(&mut self, other: VecType) {
        self.values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() * y.clone())
            .collect();
    }
}

impl Div for VecType {
    type Output = VecType;

    fn div(self, other: VecType) -> VecType {
        let values: Vec<Integer> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() / y.clone())
            .collect();
        VecType::new(values)
    }
}

impl DivAssign for VecType {
    fn div_assign(&mut self, other: VecType) {
        self.values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() / y.clone())
            .collect();
    }
}

impl Rem for VecType {
    type Output = VecType;

    fn rem(self, other: VecType) -> VecType {
        let values: Vec<Integer> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() % y.clone())
            .collect();
        VecType::new(values)
    }
}

impl RemAssign for VecType {
    fn rem_assign(&mut self, other: VecType) {
        self.values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x.clone() % y.clone())
            .collect();
    }
}

impl Neg for VecType {
    type Output = VecType;

    fn neg(self) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| -x.clone()).collect();
        VecType::new(values)
    }
}

impl PartialOrd for VecType {
    fn partial_cmp(&self, other: &VecType) -> Option<Ordering> {
        Some(self.values.cmp(&other.values))
    }
}

impl VecType {
    fn mod_add_at_index(&self, index: usize, element: Integer) -> VecType {
        let mut values = self.values.clone();
        values[index] += element;
        VecType::new(values)
    }

    fn mod_sub(&self, element: Integer) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| x.clone() - element.clone()).collect();
        VecType::new(values)
    }

    fn mod_mul(&self, element: Integer) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| x.clone() * element.clone()).collect();
        VecType::new(values)
    }

    fn mod_sub_eq(&mut self, element: Integer) {
        self.values = self.values.iter().map(|x| x.clone() - element.clone()).collect();
    }

    fn mod_add_eq(&mut self, element: Integer) {
        self.values = self.values.iter().map(|x| x.clone() + element.clone()).collect();
    }

    fn mod_mul_eq(&mut self, element: Integer) {
        self.values = self.values.iter().map(|x| x.clone() * element.clone()).collect();
    }

    fn mod_inverse(&self) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| Integer::new(1) / x.clone()).collect();
        VecType::new(values)
    }

    fn mod_by_two(&self) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| x.clone() % Integer::new(2)).collect();
        VecType::new(values)
    }

    fn mod(&self, modulus: Integer) -> VecType {
        let values: Vec<Integer> = self.values.iter().map(|x| x.clone() % modulus.clone()).collect();
        VecType::new(values)
    }

    fn switch_modulus(&mut self, modulus: Integer) {
        self.values = self.values.iter().map(|x| x.clone() % modulus.clone()).collect();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Format {
    Coefficient,
    Evaluation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Params {
    ring_dimension: usize,
    modulus: Integer,
    root_of_unity: Integer,
}

impl Params {
    fn new(ring_dimension: usize, modulus: Integer, root_of_unity: Integer) -> Self {
        Params {
            ring_dimension,
            modulus,
            root_of_unity,
        }
    }

    fn get_ring_dimension(&self) -> usize {
        self.ring_dimension
    }

    fn get_modulus(&self) -> Integer {
        self.modulus.clone()
    }

    fn get_root_of_unity(&self) -> Integer {
        self.root_of_unity.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PolyImpl {
    format: Format,
    params: Params,
    values: VecType,
}

impl PolyImpl {
    fn new(format: Format, params: Params, values: VecType) -> Self {
        PolyImpl {
            format,
            params,
            values,
        }
    }

    fn set_format(&mut self, format: Format) {
        self.format = format;
    }

    fn set_values(&mut self, values: VecType, format: Format) {
        self.values = values;
        self.format = format;
    }

    fn get_values(&self) -> VecType {
        self.values.clone()
    }

    fn get_root_of_unity(&self) -> Integer {
        self.params.get_root_of_unity()
    }

    fn plus(&self, element: Integer) -> PolyImpl {
        let values = self.values.mod_add_at_index(0, element);
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn minus(&self, element: Integer) -> PolyImpl {
        let values = self.values.mod_sub(element);
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn times(&self, element: Integer) -> PolyImpl {
        let values = self.values.mod_mul(element);
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn minus_poly(&self, rhs: PolyImpl) -> PolyImpl {
        let values = self.values - rhs.values;
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn multiply_and_round(&self, p: Integer, q: Integer) -> PolyImpl {
        let values = self.values * p * q;
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn divide_and_round(&self, q: Integer) -> PolyImpl {
        let values = self.values / q;
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn negate(&self) -> PolyImpl {
        let values = -self.values.clone();
        PolyImpl::new(self.format.clone(), self.params.clone(), values)
    }

    fn automorphism_transform(&self, k: u32) -> PolyImpl {
        let n = self.params.get_ring_dimension();
        let m = self.params.get_root_of_unity();
        let bp = n == m / 2;
        let bf = self.format == Format::Evaluation;

        if !bp {
            panic!("Automorphism Poly Format not EVALUATION or not power-of-two");
        }

        if k % 2 == 0 {
            panic!("Automorphism index not odd");
        }

        let mut result = PolyImpl::new(self.format.clone(), self.params.clone(), self.values.clone());

        let logm = (m as f64).log2() as u32 - 1;
        let logn = logm - 1;
        let mask = (1 << logn) - 1;

        if bf {
            for j in 0..n {
                let jrev = j.reverse_bits() >> (32 - logn);
                let idxrev = ((k >> 1) & mask).reverse_bits() >> (32 - logn);
                result.values.values[jrev as usize] = self.values.values[idxrev as usize].clone();
            }
            return result;
        }

        let q = self.params.get_modulus();
        for j in 0..n {
            let jk = j * k;
            result.values.values[(jk & mask) as usize] = if ((jk >> logn) & 0x1) != 0 {
                q - self.values.values[j as usize].clone()
            } else {
                self.values.values[j as usize].clone()
            };
        }
        result
    }

    fn arbitrary_switch_format(&mut self) {
        let lr = self.params.get_root_of_unity();
        let bm = self.params.get_modulus();
        let br = self.params.get_root_of_unity();
        let co = self.params.get_ring_dimension();

        if self.format == Format::Coefficient {
            self.format = Format::Evaluation;
            let values = self
                .values
                .values
                .iter()
                .map(|x| Integer::from_str(&x.to_string()).unwrap())
                .collect();
            self.values = VecType::new(values);
        } else {
            self.format = Format::Coefficient;
            let values = self
                .values
                .values
                .iter()
                .map(|x| Integer::from_str(&x.to_string()).unwrap())
                .collect();
            self.values = VecType::new(values);
        }
    }
}

impl fmt::Display for PolyImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.values)
    }
}

// fn main() {
//     let values = vec![Integer::new(1), Integer::new(2), Integer::new(3)];
//     let vec_type = VecType::new(values);
//     let params = Params::new(3, Integer::new(10), Integer::new(5));
//     let poly_impl = PolyImpl::new(Format::Coefficient, params, vec_type);
//     println!("{}", poly_impl);
// }


