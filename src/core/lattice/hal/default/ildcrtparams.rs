use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{self, Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, PartialEq)]
struct BasicInteger {
    value: i64,
}

impl BasicInteger {
    fn new(value: i64) -> Self {
        BasicInteger { value }
    }
}

impl From<i64> for BasicInteger {
    fn from(value: i64) -> Self {
        BasicInteger { value }
    }
}

impl FromStr for BasicInteger {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(value) => Ok(BasicInteger { value }),
            Err(_) => Err(()),
        }
    }
}

impl Display for BasicInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for BasicInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for BasicInteger {
    type Output = BasicInteger;

    fn add(self, other: BasicInteger) -> BasicInteger {
        BasicInteger::new(self.value + other.value)
    }
}

impl Sub for BasicInteger {
    type Output = BasicInteger;

    fn sub(self, other: BasicInteger) -> BasicInteger {
        BasicInteger::new(self.value - other.value)
    }
}

impl Mul for BasicInteger {
    type Output = BasicInteger;

    fn mul(self, other: BasicInteger) -> BasicInteger {
        BasicInteger::new(self.value * other.value)
    }
}

impl Div for BasicInteger {
    type Output = BasicInteger;

    fn div(self, other: BasicInteger) -> BasicInteger {
        BasicInteger::new(self.value / other.value)
    }
}

#[derive(Clone, PartialEq)]
struct NativeInteger {
    value: i64,
}

impl NativeInteger {
    fn new(value: i64) -> Self {
        NativeInteger { value }
    }
}

impl From<i64> for NativeInteger {
    fn from(value: i64) -> Self {
        NativeInteger { value }
    }
}

impl FromStr for NativeInteger {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(value) => Ok(NativeInteger { value }),
            Err(_) => Err(()),
        }
    }
}

impl Display for NativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for NativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for NativeInteger {
    type Output = NativeInteger;

    fn add(self, other: NativeInteger) -> NativeInteger {
        NativeInteger::new(self.value + other.value)
    }
}

impl Sub for NativeInteger {
    type Output = NativeInteger;

    fn sub(self, other: NativeInteger) -> NativeInteger {
        NativeInteger::new(self.value - other.value)
    }
}

impl Mul for NativeInteger {
    type Output = NativeInteger;

    fn mul(self, other: NativeInteger) -> NativeInteger {
        NativeInteger::new(self.value * other.value)
    }
}

impl Div for NativeInteger {
    type Output = NativeInteger;

    fn div(self, other: NativeInteger) -> NativeInteger {
        NativeInteger::new(self.value / other.value)
    }
}

#[derive(Clone, PartialEq)]
struct ILParamsImpl<T> {
    corder: u32,
    modulus: T,
    root_of_unity: T,
}

impl<T> ILParamsImpl<T> {
    fn new(corder: u32, modulus: T, root_of_unity: T) -> Self {
        ILParamsImpl {
            corder,
            modulus,
            root_of_unity,
        }
    }
}

impl<T: Display> Display for ILParamsImpl<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {}, modulus: {}, root_of_unity: {}",
            self.corder, self.modulus, self.root_of_unity
        )
    }
}

impl<T: Debug> Debug for ILParamsImpl<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {:?}, modulus: {:?}, root_of_unity: {:?}",
            self.corder, self.modulus, self.root_of_unity
        )
    }
}

#[derive(Clone, PartialEq)]
struct ElemParams<T> {
    corder: u32,
    modulus: T,
    ciphertext_modulus: T,
}

impl<T> ElemParams<T> {
    fn new(corder: u32, modulus: T) -> Self {
        ElemParams {
            corder,
            modulus,
            ciphertext_modulus: modulus.clone(),
        }
    }
}

impl<T: Display> Display for ElemParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {}, modulus: {}, ciphertext_modulus: {}",
            self.corder, self.modulus, self.ciphertext_modulus
        )
    }
}

impl<T: Debug> Debug for ElemParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {:?}, modulus: {:?}, ciphertext_modulus: {:?}",
            self.corder, self.modulus, self.ciphertext_modulus
        )
    }
}

#[derive(Clone, PartialEq)]
struct ILDCRTParams<T> {
    corder: u32,
    original_modulus: T,
    params: Vec<ILParamsImpl<T>>,
}

impl<T> ILDCRTParams<T> {
    fn new(corder: u32, modulus: T, root_of_unity: T) -> Self {
        let params = vec![ILParamsImpl::new(corder, modulus, root_of_unity)];
        let composite_modulus = modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: modulus,
            params,
        }
    }

    fn new_with_depth(&self, corder: u32, depth: u32, bits: u32) -> Self {
        let params = vec![ILParamsImpl::new(corder, self.modulus, self.root_of_unity)];
        let composite_modulus = self.modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            self.modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: self.modulus,
            params,
        }
    }

    fn new_with_moduli(
        &self,
        corder: u32,
        moduli: Vec<T>,
        roots_of_unity: Vec<T>,
    ) -> Self {
        let limbs = moduli.len();
        let params = Vec::with_capacity(limbs);
        let composite_modulus = self.modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            self.modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: self.modulus,
            params,
        }
    }

    fn new_with_moduli_big(
        &self,
        corder: u32,
        moduli: Vec<T>,
        roots_of_unity: Vec<T>,
        moduli_big: Vec<T>,
        roots_of_unity_big: Vec<T>,
        input_original_modulus: T,
    ) -> Self {
        let limbs = moduli.len();
        let params = Vec::with_capacity(limbs);
        let composite_modulus = self.modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            self.modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: self.modulus,
            params,
        }
    }

    fn new_with_moduli_input_original_modulus(
        &self,
        corder: u32,
        moduli: Vec<T>,
        input_original_modulus: T,
    ) -> Self {
        let limbs = moduli.len();
        let params = Vec::with_capacity(limbs);
        let composite_modulus = self.modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            self.modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: self.modulus,
            params,
        }
    }

    fn new_with_params_input_original_modulus(
        &self,
        corder: u32,
        params: Vec<ILParamsImpl<T>>,
        input_original_modulus: T,
    ) -> Self {
        let limbs = params.len();
        let composite_modulus = self.modulus.clone();
        let ciphertext_modulus = composite_modulus.clone();
        ElemParams {
            corder,
            self.modulus,
            ciphertext_modulus,
        };
        ILDCRTParams {
            corder,
            original_modulus: self.modulus,
            params,
        }
    }

    fn get_params(&self) -> &Vec<ILParamsImpl<T>> {
        &self.params
    }

    fn get_param_partition(&self, start: u32, end: u32) -> Vec<ILParamsImpl<T>> {
        self.params[start as usize..=end as usize].to_vec()
    }

    fn get_original_modulus(&self) -> &T {
        &self.original_modulus
    }

    fn set_original_modulus(&mut self, input_original_modulus: T) {
        self.original_modulus = input_original_modulus;
    }

    fn pop_last_param(&mut self) {
        self.ciphertext_modulus /= self.params.last().unwrap().modulus.clone();
        self.params.pop();
    }

    fn pop_first_param(&mut self) {
        self.ciphertext_modulus /= self.params[0].modulus.clone();
        self.params.remove(0);
    }
}

impl<T: Display> Display for ILDCRTParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {}, original_modulus: {}, params: {:?}",
            self.corder, self.original_modulus, self.params
        )
    }
}

impl<T: Debug> Debug for ILDCRTParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {:?}, original_modulus: {:?}, params: {:?}",
            self.corder, self.original_modulus, self.params
        )
    }
}

impl<T: PartialEq> PartialEq<ElemParams<T>> for ILDCRTParams<T> {
    fn eq(&self, other: &ElemParams<T>) -> bool {
        if let Some(dcrt_params) = other.downcast_ref::<ILDCRTParams<T>>() {
            if self.corder != dcrt_params.corder {
                return false;
            }
            if self.params.len() != dcrt_params.params.len() {
                return false;
            }
            for i in 0..self.params.len() {
                if self.params[i] != dcrt_params.params[i] {
                    return false;
                }
            }
            return self.original_modulus == dcrt_params.original_modulus;
        }
        false
    }
}

impl<T> ILDCRTParams<T> {
    fn recalculate_modulus(&mut self) {
        self.ciphertext_modulus = T::new(1);
        for i in 0..self.params.len() {
            self.ciphertext_modulus *= self.params[i].modulus.clone();
        }
    }

    fn recalculate_big_modulus(&mut self) {
        self.big_ciphertext_modulus = T::new(1);
        for i in 0..self.params.len() {
            self.big_ciphertext_modulus *= self.params[i].big_modulus.clone();
        }
    }
}

impl<T: Display> Display for ILDCRTParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {}, original_modulus: {}, params: {:?}",
            self.corder, self.original_modulus, self.params
        )
    }
}

impl<T: Debug> Debug for ILDCRTParams<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "corder: {:?}, original_modulus: {:?}, params: {:?}",
            self.corder, self.original_modulus, self.params
        )
    }
}

// fn main() {
//     let il_params = ILParamsImpl::new(0, BasicInteger::new(0), BasicInteger::new(0));
//     let elem_params = ElemParams::new(0, BasicInteger::new(0));
//     let ildcrt_params = ILDCRTParams::new(0, BasicInteger::new(0), BasicInteger::new(0));
//     println!("ILParamsImpl: {}", il_params);
//     println!("ElemParams: {}", elem_params);
//     println!("ILDCRTParams: {}", ildcrt_params);
// }


