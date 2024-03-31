/*
  This file contains the main class for unsigned big integers: ubint. Big integers are
  represented as arrays of machine native unsigned integers. The native integer type is
  supplied as a template parameter. Currently implementation based on uint32_t and uint64_t is
  supported. a native double the base integer size is also needed.
*/

 use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use num_bigint::{BigInt, Sign};
use num_traits::{Zero, One};
use num_integer::Integer;

struct Ubint<LimbT> {
    msb: usize,
    value: Vec<LimbT>,
}

impl<LimbT> Ubint<LimbT> 
where
    LimbT: Integer + Clone + From<u8>,
{
    fn new() -> Self {
        Ubint {
            msb: 0,
            value: vec![LimbT::zero()],
        }
    }

    fn from_vec(v: Vec<LimbT>) -> Self {
        let mut ubint = Ubint {
            msb: 0, // This should be calculated based on the input vector `v`
            value: v,
        };
        ubint.normalize_limbs();
        ubint
    }

    fn normalize_limbs(&mut self) {
        // Implementation for normalizing limbs goes here
    }

    fn set_value(&mut self, strval: &str) {
        // Implementation for setting value from a string goes here
    }
}

impl<LimbT> From<&str> for Ubint<LimbT> 
where
    LimbT: Integer + Clone + From<u8>,
{
    fn from(strval: &str) -> Self {
        let mut ubint = Ubint::new();
        ubint.set_value(strval);
        ubint
    }
}

impl<LimbT> Clone for Ubint<LimbT> 
where
    LimbT: Clone,
{
    fn clone(&self) -> Self {
        Ubint {
            msb: self.msb,
            value: self.value.clone(),
        }
    }
}

impl<LimbT> From<u8> for Ubint<LimbT> 
where
    LimbT: From<u8>,
{
    fn from(val: u8) -> Self {
        Ubint {
            msb: 0, // This should be calculated based on `val`
            value: vec![LimbT::from(val)],
        }
    }
}

// Implementations for Add, Sub, Mul, Div traits could go here, depending on LimbT capabilities

// Log2 and DataTypes structures would be replaced by Rust's type system and traits


use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Neg};

impl Ubint {
    fn set_value(&mut self, val: &Ubint) {
        self.m_MSB = val.m_MSB;
        self.m_value = val.m_value.clone();
    }

    fn set_identity(&mut self) {
        self.m_MSB = 1;
        self.m_value = vec![1];
    }

    fn add(&self, b: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn add_eq(&mut self, b: &Ubint) {
        // Implementation omitted
    }

    fn sub(&self, b: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn sub_eq(&mut self, b: &Ubint) {
        // Implementation omitted
    }

    fn mul(&self, b: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn mul_eq(&mut self, b: &Ubint) {
        *self = self.mul(b);
    }

    fn divided_by(&self, b: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn divided_by_eq(&mut self, b: &Ubint) {
        // Implementation omitted
    }

    fn exp(&self, p: usize) -> Ubint {
        // Implementation omitted
    }

    fn exp_eq(&mut self, p: usize) {
        *self = self.exp(p);
    }

    fn multiply_and_round(&self, p: &Ubint, q: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn multiply_and_round_eq(&mut self, p: &Ubint, q: &Ubint) {
        *self = self.multiply_and_round(p, q);
    }

    fn divide_and_round(&self, q: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn divide_and_round_eq(&mut self, q: &Ubint) {
        *self = self.divide_and_round(q);
    }

    fn mod_(&self, modulus: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn mod_eq(&mut self, modulus: &Ubint) {
        // Implementation omitted
    }

    fn compute_mu(&self) -> Ubint {
        // Implementation omitted
    }

    fn mod_add(&self, b: &Ubint, modulus: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn mod_add_eq(&mut self, b: &Ubint, modulus: &Ubint) {
        // Implementation omitted
    }

    fn mod_add_fast(&self, b: &Ubint, modulus: &Ubint) -> Ubint {
        // Implementation omitted
    }

    fn mod_add_fast_eq(&mut self, b: &Ubint, modulus: &Ubint) {
        // Implementation omitted
    }
}


use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

fn mod_add<T: Into<BigUint>>(b: T, modulus: T, mu: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    let mu = mu.into();
    b.mod_add(&modulus)
}

fn mod_add_eq<T: Into<BigUint>>(b: T, modulus: T, mu: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    let mu = mu.into();
    this = b.mod_add(&modulus);
    this
}

fn mod_sub<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    self.mod_sub(&b, &modulus)
}

fn mod_sub_eq<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    this = this.mod_sub(&b, &modulus);
    this
}

fn mod_sub_fast<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    self.mod_sub_fast(&b, &modulus)
}

fn mod_sub_fast_eq<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    this = this.mod_sub_fast(&b, &modulus);
    this
}

fn mod_mul<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    self.mod_mul(&b, &modulus)
}

fn mod_mul_eq<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    this = this.mod_mul(&b, &modulus);
    this
}

fn mod_mul_fast<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    self.mod_mul_fast(&b, &modulus)
}

fn mod_mul_fast_eq<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    this = this.mod_mul_fast(&b, &modulus);
    this
}

fn mod_exp<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let b = b.into();
    let modulus = modulus.into();
    self.mod_exp(&b, &modulus)
}

fn mod_exp_eq<T: Into<BigUint>>(b: T, modulus: T) -> BigUint {
    let mut this = self.clone();
    let b = b.into();
    let modulus = modulus.into();
    this = this.mod_exp(&b, &modulus);
    this
}

fn mod_inverse<T: Into<BigUint>>(modulus: T) -> BigUint {
    let modulus = modulus.into();
    self.mod_inverse(&modulus)
}

fn mod_inverse_eq<T: Into<BigUint>>(modulus: T) -> BigUint {
    let mut this = self.clone();
    let modulus = modulus.into();
    this = this.mod_inverse(&modulus);
    this
}

fn l_shift(shift: u16) -> BigUint {
    self.lsh(shift)
}

fn l_shift_eq(shift: u16) -> BigUint {
    let mut this = self.clone();
    this = this.lsh(shift);
    this
}

fn r_shift(shift: u16) -> BigUint {
    self.rsh(shift)
}

fn r_shift_eq(shift: u16) -> BigUint {
    let mut this = self.clone();
    this = this.rsh(shift);
    this
}



use std::fmt;
use std::ops;

struct ubint {
    m_value: Vec<limb_t>,
    m_MSB: usint,
    m_limbBitLength: usint,
}

impl ubint {
    fn compare(&self, a: &ubint) -> i32 {
        if self.m_MSB < a.m_MSB {
            return -1;
        }
        if self.m_MSB > a.m_MSB {
            return 1;
        }
        for i in (0..self.m_value.len()).rev() {
            if self.m_value[i] < a.m_value[i] {
                return -1;
            }
            if self.m_value[i] > a.m_value[i] {
                return 1;
            }
        }
        return 0;
    }

    fn convert_to_int<T>(&self) -> T
    where
        T: From<limb_t> + ops::BitOrAssign + ops::ShlAssign,
    {
        let limblen: usint = std::mem::size_of::<T>() * 8;
        if self.m_limbBitLength >= limblen {
            return self.m_value[0].into();
        }
        if self.m_limbBitLength < limblen {
            let ceilInt = self.MSBToLimbs(limblen.max(self.m_MSB));
            let mut result: T = self.m_value[0].into();
            for i in 1..ceilInt {
                result |= (self.m_value[i].into() << (i * self.m_limbBitLength));
            }
            return result;
        }
    }

    fn convert_to_float(&self) -> f32 {
        // implementation goes here
    }

    fn convert_to_double(&self) -> f64 {
        // implementation goes here
    }

    fn convert_to_long_double(&self) -> f128 {
        // implementation goes here
    }

    fn from_binary_string(bitString: &str) -> ubint {
        // implementation goes here
    }

    fn get_MSB(&self) -> usint {
        self.m_MSB
    }

    fn get_number_of_limbs(&self) -> usize {
        self.m_value.len()
    }

    fn get_length_for_base(&self, base: usint) -> usint {
        self.get_MSB()
    }

    fn get_digit_at_index_for_base(&self, index: usint, base: usint) -> usint {
        // implementation goes here
    }

    fn get_bit_at_index(&self, index: usint) -> uschar {
        // implementation goes here
    }

    fn allocator() -> ubint {
        ubint {
            m_value: Vec::new(),
            m_MSB: 0,
            m_limbBitLength: 0,
        }
    }

    fn to_string(&self) -> String {
        // implementation goes here
    }

    fn integer_type_name() -> String {
        if std::mem::size_of::<limb_t>() == std::mem::size_of::<u32>() {
            return String::from("UBDYNINT_32");
        }
        if std::mem::size_of::<limb_t>() == std::mem::size_of::<u64>() {
            return String::from("UBDYNINT_64");
        }
        if std::mem::size_of::<limb_t>() == std::mem::size_of::<u128>() {
            return String::from("UBDYNINT_128");
        }
        panic!("Configuration Error: ubintdyn.h");
    }

    fn get_internal_representation(&self) -> String {
        let mut ret = String::new();
        for (i, value) in self.m_value.iter().enumerate() {
            ret += &value.to_string();
            if i < (self.m_value.len() - 1) {
                ret += " ";
            }
        }
        return ret;
    }

    fn print_integer_constants() {
        println!("sizeof UINT8_C  {}", std::mem::size_of::<u8>());
        println!("sizeof UINT16_C {}", std::mem::size_of::<u16>());
        println!("sizeof UINT32_C {}", std::mem::size_of::<u32>());
        println!("sizeof UINT64_C {}", std::mem::size_of::<u64>());
        println!("sizeof uint8_t  {}", std::mem::size_of::<u8>());
        println!("sizeof uint16_t {}", std::mem::size_of::<u16>());
        println!("sizeof uint32_t {}", std::mem::size_of::<u32>());
        println!("sizeof uint64_t {}", std::mem::size_of::<u64>());
        #[cfg(feature = "HAVE_INT128")]
        println!("sizeof uint128_t {}", std::mem::size_of::<u128>());
    }

    fn save(&self, ar: &mut dyn Archive, version: u32) {
        ar.serialize_field("v", &self.m_value);
        ar.serialize_field("m", &self.m_MSB);
    }

    fn load(&mut self, ar: &mut dyn Archive, version: u32) {
        if version > self.serialized_version() {
            panic!("serialized object version {} is from a later version of the library", version);
        }
        self.m_value = ar.deserialize_field("v").unwrap();
        self.m_MSB = ar.deserialize_field("m").unwrap();
    }

    fn serialized_object_name(&self) -> String {
        String::from("DYNInteger")
    }

    fn serialized_version() -> u32 {
        1
    }

    fn set_MSB(&mut self) {
        self.m_MSB = self.m_limbBitLength * (self.m_value.len() - 1) as usint;
        self.m_MSB += lbcrypto::GetMSB(self.m_value.last().unwrap());
    }

    fn normalize_limbs(&mut self) {
        let mut size = self.m_value.len() - 1;
        while size > 0 && self.m_value[size] == 0 {
            self.m_value.pop();
            size -= 1;
        }
        self.m_MSB = self.m_limbBitLength * (self.m_value.len() - 1) as usint;
        self.m_MSB += lbcrypto::GetMSB(self.m_value.last().unwrap());
    }

    fn divqr_vect(&self, q: &mut ubint, r: &mut ubint, u: &ubint, v: &ubint) {
        // implementation goes here
    }

    fn divq_vect(&self, q: &mut ubint, u: &ubint, v: &ubint) {
        // implementation goes here
    }

    fn divr_vect(&self, r: &mut ubint, u: &ubint, v: &ubint) {
        // implementation goes here
    }

    fn MSBToLimbs(msb: usint) -> usint {
        const mask: usint = self.m_limbBitLength - 1;
        if msb == 0 {
            return 1;
        }
        return (msb >> self.m_log2LimbBitLength) + ((msb & mask) != 0);
    }
}

impl fmt::Display for ubint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T: Archive> Archive for ubint {
    fn save(&self, ar: &mut T, version: u32) {
        self.save(ar, version);
    }

    fn load(&mut self, ar: &mut T, version: u32) {
        self.load(ar, version);
    }
}

impl<T: Archive> Serialize for ubint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ar = CerealArchive::new(serializer);
        self.save(&mut ar, self.serialized_version());
        ar.into_inner()
    }
}

impl<'de, T: Archive> Deserialize<'de> for ubint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ar = CerealArchive::new(deserializer);
        let mut obj = Self::allocator();
        obj.load(&mut ar, obj.serialized_version());
        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let a = ubint {
            m_value: vec![1, 2, 3],
            m_MSB: 10,
            m_limbBitLength: 32,
        };
        let b = ubint {
            m_value: vec![1, 2, 4],
            m_MSB: 10,
            m_limbBitLength: 32,
        };
        assert_eq!(a.compare(&b), -1);
    }
}



/*
  This file contains the main class for unsigned big integers: ubint. Big integers are represented as arrays of
  machine native unsigned integers. The native integer type is supplied as a template parameter.
  Currently implementation based on uint32_t and uint64_t is supported. a native double the base integer size is also needed.
*/
use std::vec::Vec;

pub struct ubint<limb_t> {
    m_value: Vec<limb_t>,
    m_MSB: u32,
    m_limbBitLength: u32,
    m_MaxLimb: u32,
}

impl<limb_t> ubint<limb_t> {
    pub fn Add(&self, b: &ubint<limb_t>) -> ubint<limb_t> {
        let A = self;
        let sizeA = self.m_value.len();
        let B = b;
        let sizeB = b.m_value.len();
        if sizeA < sizeB {
            let temp = A;
            A = B;
            B = temp;
            let tempSize = sizeA;
            sizeA = sizeB;
            sizeB = tempSize;
        }
        if B.m_MSB == 0 {
            return A.clone();
        }
        let mut r = Vec::with_capacity(sizeA + 1);
        let mut c: Dlimb_t = 0;
        for i in 0..sizeA {
            let av = A.m_value[i] as Dlimb_t;
            let bv = if i < sizeB { B.m_value[i] as Dlimb_t } else { 0 };
            r[i] = (c += av + bv) as limb_t;
            c >>= m_limbBitLength;
        }
        r[sizeA] = c as limb_t;
        return ubint { m_value: r };
    }

    pub fn AddEq(&mut self, b: &ubint<limb_t>) -> &ubint<limb_t> {
        let A = self;
        let sizeA = self.m_value.len();
        let B = b;
        let sizeB = B.m_value.len();
        if sizeA < sizeB {
            let temp = A;
            A = B;
            B = temp;
            let tempSize = sizeA;
            sizeA = sizeB;
            sizeB = tempSize;
        }
        if B.m_MSB == 0 {
            return self = A.clone();
        }
        let mut r = Vec::with_capacity(sizeA + 1);
        let mut c: Dlimb_t = 0;
        for i in 0..sizeA {
            let av = A.m_value[i] as Dlimb_t;
            let bv = if i < sizeB { B.m_value[i] as Dlimb_t } else { 0 };
            r[i] = (c += av + bv) as limb_t;
            c >>= m_limbBitLength;
        }
        r[sizeA] = c as limb_t;
        self.m_value = r;
        ubint<limb_t>::NormalizeLimbs();
        return self;
    }

    pub fn Sub(&self, b: &ubint<limb_t>) -> ubint<limb_t> {
        if *self <= *b {
            return ubint();
        }
        let mut result = self.clone();
        for i in 0..b.m_value.len() {
            if result.m_value[i] < b.m_value[i] {
                let mut cntr = i;
                result.m_value[cntr] += (m_MaxLimb - b.m_value[cntr]) + 1;
                while 0 == result.m_value[++cntr] {
                    result.m_value[cntr] = m_MaxLimb;
                }
                result.m_value[cntr]--;
            } else {
                result.m_value[i] -= b.m_value[i];
            }
        }
        result.NormalizeLimbs();
        return result;
    }

    pub fn SubEq(&mut self, b: &ubint<limb_t>) -> &ubint<limb_t> {
        if *self <= *b {
            self.m_MSB = 0;
            self.m_value[0] = 0;
            self.m_value.resize(1);
            return self;
        }
        for i in 0..b.m_value.len() {
            if self.m_value[i] < b.m_value[i] {
                let mut cntr = i;
                self.m_value[cntr] += (m_MaxLimb - b.m_value[cntr]) + 1;
                while 0 == self.m_value[++cntr] {
                    self.m_value[cntr] = m_MaxLimb;
                }
                self.m_value[cntr]--;
            } else {
                self.m_value[i] -= b.m_value[i];
            }
        }
        ubint<limb_t>::NormalizeLimbs();
        return self;
    }

    pub fn Mul(&self, b: &ubint<limb_t>) -> ubint<limb_t> {
        if self.m_MSB == 0 || b.m_MSB == 0 {
            return ubint();
        }
        if b.m_MSB == 1 {
            return self.clone();
        }
        if self.m_MSB == 1 {
            return b.clone();
        }
        let A = self;
        let aSize = self.m_value.len();
        let B = b;
        let bSize = b.m_value.len();
        if aSize < bSize {
            let temp = A;
            A = B;
            B = temp;
            let tempSize = aSize;
            aSize = bSize;
            bSize = tempSize;
        }
        let mut ans = ubint();
        for i in 0..bSize {
            let mut c = Vec::with_capacity(i + aSize + 1);
            let limbb = B.m_value[i] as Dlimb_t;
            let mut ofl: Dlimb_t = 0;
            for j in 0..aSize {
                c[i + j] = (ofl += limbb * A.m_value[j] as Dlimb_t) as limb_t;
                ofl >>= m_limbBitLength;
            }
            c[i + aSize] = ofl as limb_t;
            ans = ans.Add(ubint { m_value: c });
        }
        return ans;
    }

    pub fn DividedBy(&self, b: &ubint<limb_t>) -> ubint<limb_t> {
        if b.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "Divisor is zero");
        }
        if b.m_MSB > self.m_MSB {
            return ubint();
        }
        if b.m_MSB == self.m_MSB && b.m_value.back() == self.m_value.back() {
            return ubint(1);
        }
        let mut ans = ubint();
        divq_vect(ans, self, b);
        return ans;
    }

    pub fn DividedByEq(&mut self, b: &ubint<limb_t>) -> &ubint<limb_t> {
        if b.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "Divisor is zero");
        }
        if b.m_MSB > self.m_MSB {
            self.m_MSB = 0;
            self.m_value.resize(1);
            self.m_value[0] = 0;
            return self;
        }
        if b.m_MSB == self.m_MSB && b.m_value.back() == self.m_value.back() {
            self.m_MSB = 1;
            self.m_value.resize(1);
            self.m_value[0] = 1;
            return self;
        }
        let mut ans = ubint();
        divq_vect(ans, self, b);
        return self = ans;
    }

    pub fn Exp(&self, p: usint) -> ubint<limb_t> {
        if p == 0 {
            return ubint(1);
        }
        if p == 1 {
            return self.clone();
        }
        let mut tmp = ubint<limb_t>::Exp(p >> 1);
        tmp = tmp.Mul(tmp);
        if p & 0x1 {
            return tmp.Mul(self);
        }
        return tmp;
    }

    pub fn MultiplyAndRound(&self, p: &ubint<limb_t>, q: &ubint<limb_t>) -> ubint<limb_t> {
        if q.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "MultiplyAndRound() Divisor is zero");
        }
        let t = self.Mul(p);
        let halfQ = q >> 1;
        if t <= halfQ {
            return ubint();
        }
        if (t.m_MSB == halfQ.m_MSB) || ((t.m_MSB == q.m_MSB) && (t.m_value.back() < q.m_value.back())) {
            return ubint(1);
        }
        let mut ans = ubint();
        let mut rv = ubint();
        divqr_vect(ans, rv, t, q);
        if rv > halfQ {
            return ans.Add(ubint(1));
        }
        return ans;
    }

    pub fn DivideAndRound(&self, q: &ubint<limb_t>) -> ubint<limb_t> {
        if q.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "DivideAndRound() Divisor is zero");
        }
        let halfQ = q >> 1;
        if *self <= halfQ {
            return ubint();
        }
        if (self.m_MSB == halfQ.m_MSB) || ((self.m_MSB == q.m_MSB) && (self.m_value.back() < q.m_value.back())) {
            return ubint(1);
        }
        let mut ans = ubint();
        let mut rv = ubint();
        divqr_vect(ans, rv, self, q);
        if rv > halfQ {
            return ans.Add(ubint(1));
        }
        return ans;
    }

    pub fn Mod(&self, modulus: &ubint<limb_t>) -> ubint<limb_t> {
        if modulus.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "Mod() using zero modulus");
        }
        if *self < *modulus {
            return self.clone();
        }
        if modulus.m_MSB == 2 && modulus.m_value[0] == 2 {
            return ubint(self.m_value[0] & 0x1);
        }
        let mut ans = ubint();
        divr_vect(ans, self, modulus);
        return ans;
    }

    pub fn ModEq(&mut self, modulus: &ubint<limb_t>) -> &ubint<limb_t> {
        if modulus.m_MSB == 0 {
            OPENFHE_THROW(lbcrypto::math_error, "Mod() using zero modulus");
        }
        if *self < *modulus {
            self.m_value.resize(1);
            self.m_value[0] &= 0x1;
            self.m_MSB = self.m_value[0];
            return self;
        }
        let mut ans = ubint();
        divr_vect(ans, self, modulus);
        return self = ans;
    }

    pub fn ModAdd(&self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> ubint<limb_t> {
        let mut bv = b.clone();
        if bv >= *modulus {
            bv.ModEq(modulus);
        }
        let mut av = self.clone();
        if av >= *modulus {
            av.ModEq(modulus);
        }
        av = av.Add(bv);
        if av >= *modulus {
            return av.Sub(modulus);
        }
        return av;
    }

    pub fn ModAddEq(&mut self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> &ubint<limb_t> {
        let mut bv = b.clone();
        if bv >= *modulus {
            bv.ModEq(modulus);
        }
        if *self >= *modulus {
            ubint<limb_t>::ModEq(modulus);
        }
        *self = bv.Add(*self);
        if *self >= *modulus {
            return ubint<limb_t>::SubEq(modulus);
        }
        return self;
    }

    pub fn ModAddFast(&self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> ubint<limb_t> {
        let ans = b.Add(*self);
        if ans >= *modulus {
            return ans.Sub(modulus);
        }
        return ans;
    }

    pub fn ModAddFastEq(&mut self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> &ubint<limb_t> {
        *self = b.Add(*self);
        if *self >= *modulus {
            return ubint<limb_t>::SubEq(modulus);
        }
        return self;
    }

    pub fn ModSub(&self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> ubint<limb_t> {
        let mut av = self.clone();
        let mut bv = b.clone();
        if bv >= *modulus {
            bv.ModEq(modulus);
        }
        if av >= *modulus {
            av.ModEq(modulus);
        }
        if av < bv {
            av = modulus.Add(av);
        }
        return av.Sub(bv);
    }

    pub fn ModSubEq(&mut self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> &ubint<limb_t> {
        let mut bv = b.clone();
        if bv >= *modulus {
            bv.ModEq(modulus);
        }
        if *self >= *modulus {
            ubint<limb_t>::ModEq(modulus);
        }
        if *self < bv {
            *self = modulus.Add(*self);
        }
        return ubint<limb_t>::SubEq(bv);
    }

    pub fn ModSubFast(&self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> ubint<limb_t> {
        if *self < *b {
            return modulus.Add(*self).Sub(b);
        }
        return ubint<limb_t>::Sub(b);
    }

    pub fn ModSubFastEq(&mut self, b: &ubint<limb_t>, modulus: &ubint<limb_t>) -> &ubint<limb_t> {
        if *self < *b {
            return *self = std::move(modulus.Add(*self).Sub(b));
        }
        return ubint<limb_t>::SubEq(b);
    }
}


use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr};

#[derive(Clone, Debug, PartialEq, Eq)]
struct ubint<T> {
    m_value: Vec<T>,
    m_MSB: usize,
    m_limbBitLength: usize,
    m_log2LimbBitLength: usize,
}

impl<T> ubint<T> {
    fn new() -> Self {
        ubint {
            m_value: Vec::new(),
            m_MSB: 0,
            m_limbBitLength: 0,
            m_log2LimbBitLength: 0,
        }
    }

    fn set_MSB(&mut self) {
        self.m_MSB = self.m_value.len() * self.m_limbBitLength;
        if let Some(&last) = self.m_value.last() {
            let mut mask = 1 << (self.m_limbBitLength - 1);
            while mask > 0 && last & mask == 0 {
                self.m_MSB -= 1;
                mask >>= 1;
            }
        }
    }

    fn add(&self, other: &ubint<T>) -> ubint<T> {
        let mut ans = ubint::new();
        let mut carry = 0;
        let mut i = 0;
        while i < self.m_value.len() || i < other.m_value.len() || carry > 0 {
            let mut sum = carry;
            if i < self.m_value.len() {
                sum += self.m_value[i];
            }
            if i < other.m_value.len() {
                sum += other.m_value[i];
            }
            ans.m_value.push(sum % (1 << self.m_limbBitLength));
            carry = sum / (1 << self.m_limbBitLength);
            i += 1;
        }
        ans.set_MSB();
        ans
    }

    fn mod_add(&self, other: &ubint<T>, modulus: &ubint<T>) -> ubint<T> {
        let ans = self.add(other);
        if ans >= *modulus {
            ans.modulus(modulus)
        } else {
            ans
        }
    }

    fn modulus(&self, modulus: &ubint<T>) -> ubint<T> {
        let mut ans = self.clone();
        while ans >= *modulus {
            ans = ans.sub(modulus);
        }
        ans
    }

    fn sub(&self, other: &ubint<T>) -> ubint<T> {
        let mut ans = ubint::new();
        let mut borrow = 0;
        let mut i = 0;
        while i < self.m_value.len() || i < other.m_value.len() || borrow > 0 {
            let mut diff = borrow;
            if i < self.m_value.len() {
                diff += self.m_value[i];
            }
            if i < other.m_value.len() {
                diff -= other.m_value[i];
            }
            if diff < 0 {
                diff += 1 << self.m_limbBitLength;
                borrow = -1;
            } else {
                borrow = 0;
            }
            ans.m_value.push(diff);
            i += 1;
        }
        ans.set_MSB();
        ans
    }

    fn mod_sub(&self, other: &ubint<T>, modulus: &ubint<T>) -> ubint<T> {
        let ans = self.sub(other);
        if ans < ubint::new() {
            ans.add(modulus)
        } else {
            ans
        }
    }

    fn mul(&self, other: &ubint<T>) -> ubint<T> {
        let mut ans = ubint::new();
        ans.m_value.resize(self.m_value.len() + other.m_value.len(), 0);
        for i in 0..self.m_value.len() {
            let mut carry = 0;
            for j in 0..other.m_value.len() {
                let product = self.m_value[i] * other.m_value[j] + ans.m_value[i + j] + carry;
                ans.m_value[i + j] = product % (1 << self.m_limbBitLength);
                carry = product / (1 << self.m_limbBitLength);
            }
            ans.m_value[i + other.m_value.len()] = carry;
        }
        ans.set_MSB();
        ans
    }

    fn mod_mul(&self, other: &ubint<T>, modulus: &ubint<T>) -> ubint<T> {
        let ans = self.mul(other);
        ans.modulus(modulus)
    }

    fn div(&self, other: &ubint<T>) -> ubint<T> {
        let mut ans = ubint::new();
        let mut remainder = self.clone();
        let mut quotient = ubint::new();
        while remainder >= *other {
            let mut shift = remainder.m_MSB - other.m_MSB;
            let mut divisor = other.clone();
            divisor <<= shift;
            if remainder < divisor {
                shift -= 1;
                divisor >>= 1;
            }
            remainder = remainder.sub(&divisor);
            quotient.m_value.push(1 << shift);
        }
        quotient.set_MSB();
        quotient
    }

    fn mod_div(&self, other: &ubint<T>, modulus: &ubint<T>) -> ubint<T> {
        let ans = self.div(other);
        ans.modulus(modulus)
    }

    fn shl(&self, shift: usize) -> ubint<T> {
        let mut ans = self.clone();
        ans.m_MSB += shift;
        let shift_by_limb = shift >> self.m_log2LimbBitLength;
        let shift = shift % self.m_limbBitLength;
        if shift > 0 {
            let mut carry = 0;
            for v in ans.m_value.iter_mut() {
                let new_carry = *v >> (self.m_limbBitLength - shift);
                *v = (*v << shift) + carry;
                carry = new_carry;
            }
            if carry > 0 {
                ans.m_value.push(carry);
            }
        }
        if shift_by_limb > 0 {
            let j = ans.m_value.len();
            let i = j + shift_by_limb;
            ans.m_value.resize(i, 0);
            while i > 0 {
                ans.m_value[i - 1] = if j > 0 { ans.m_value[j - 1] } else { 0 };
                i -= 1;
                j -= 1;
            }
        }
        ans
    }

    fn shr(&self, shift: usize) -> ubint<T> {
        if self.m_MSB <= shift {
            return ubint::new();
        }
        let mut ans = self.clone();
        ans.m_MSB -= shift;
        let shift_by_limb = shift >> self.m_log2LimbBitLength;
        let shift = shift % self.m_limbBitLength;
        let mut tmp = ans.m_value[shift_by_limb] >> shift;
        let lshift = self.m_limbBitLength - shift;
        let size = ans.m_value.len() - shift_by_limb;
        for i in 0..size {
            tmp |= (ans.m_value[i + shift_by_limb] as T) << lshift;
            ans.m_value[i] = tmp % (1 << self.m_limbBitLength);
            tmp /= 1 << self.m_limbBitLength;
        }
        ans.m_value.resize(size, 0);
        if tmp > 0 {
            ans.m_value.push(tmp);
        }
        ans
    }

    fn mod_exp(&self, b: &ubint<T>, modulus: &ubint<T>) -> ubint<T> {
        let mut t = self.modulus(modulus);
        let mut p = b.clone();
        let mut r = ubint::new();
        if p.m_value[0] & 0x1 > 0 {
            r = r.mod_mul(&t, modulus);
        }
        while p >>= 1 > 0 {
            t = t.mod_mul(&t, modulus);
            if p.m_value[0] & 0x1 > 0 {
                r = r.mod_mul(&t, modulus);
            }
        }
        r
    }

    fn from_binary_string(vin: &str) -> ubint<T> {
        let mut v = vin.to_string();
        v = v.trim_start_matches('0').to_string();
        if v.is_empty() {
            return ubint::new();
        }
        let mut value = ubint::new();
        value.m_value.clear();
        let len = v.len();
        let cntr = (len as f64 / value.m_limbBitLength as f64).ceil() as usize;
        let mut partial_value = 0;
        for i in 0..cntr {
            let start = len - (i + 1) * value.m_limbBitLength;
            let end = if len > (i + 1) * value.m_limbBitLength {
                len - (i + 1) * value.m_limbBitLength
            } else {
                0
            };
            let val = &v[end..start];
            for j in 0..val.len() {
                partial_value += val[j..j + 1].parse::<T>().unwrap();
                partial_value <<= 1;
            }
            partial_value >>= 1;
            value.m_value.push(partial_value);
            partial_value = 0;
        }
        value.set_MSB();
        value
    }

    fn to_string(&self) -> String {
        let mut val = Vec::new();
        val.reserve(self.m_MSB >> 1);
        for i in (1..=self.m_MSB).rev() {
            let mut ofl = self.get_bit_at_index(i);
            for a in &mut val {
                *a = (*a << 1) + ofl;
                if *a > 9 {
                    *a -= 10;
                    ofl = 1;
                } else {
                    ofl = 0;
                }
            }
            if ofl > 0 {
                val.push(1);
            }
        }
        val.iter()
            .rev()
            .map(|a| (a + '0' as T) as char)
            .collect::<String>()
    }

    fn get_bit_at_index(&self, index: usize) -> T {
        let digit_len = (self.m_limbBitLength as f64).log2().ceil() as usize;
        let mut digit = 0;
        let new_index = 1 + (index - 1) * digit_len;
        for i in 1..self.m_limbBitLength {
            digit += self.get_bit_at_index(new_index + i) * (1 << i);
        }
        digit
    }
}

impl<T: Add<Output = T> + Copy> Add for ubint<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(&other)
    }
}

impl<T: Sub<Output = T> + Copy> Sub for ubint<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub(&other)
    }
}

impl<T: Mul<Output = T> + Copy> Mul for ubint<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.mul(&other)
    }
}

impl<T: Div<Output = T> + Copy> Div for ubint<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.div(&other)
    }
}

impl<T: Rem<Output = T> + Copy> Rem for ubint<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        self.mod_div(&other, &ubint::new())
    }
}

impl<T: Shl<usize, Output = T> + Copy> Shl<usize> for ubint<T> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self {
        self.shl(shift)
    }
}

impl<T: Shr<usize, Output = T> + Copy> Shr<usize> for ubint<T> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self {
        self.shr(shift)
    }
}

impl<T: PartialOrd> PartialOrd for ubint<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for ubint<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.m_MSB > other.m_MSB {
            Ordering::Greater
        } else if self.m_MSB < other.m_MSB {
            Ordering::Less
        } else {
            for i in (0..self.m_value.len()).rev() {
                if self.m_value[i] > other.m_value[i] {
                    return Ordering::Greater;
                } else if self.m_value[i] < other.m_value[i] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

/* q[0], r[0], u[0], and v[0] contain the LEAST significant words.
 (The sequence is in little-endian order).

 This is a fairly precise implementation of Knuth's Algorithm D, for a
 binary computer with base b = 2**(32|64). The caller supplies:
 1. Space q for the quotient, m - n + 1 words (at least one).
 2. Space r for the remainder (optional), n words.
 3. The dividend u, m words, m >= 1.
 4. The divisor v, n words, n >= 2.
 The most significant digit of the divisor, v[n-1], must be nonzero.  The
 dividend u may have leading zeros; this just makes the algorithm take
 longer and makes the quotient contain more leading zeros.  A value of
 nullptr may be given for the address of the remainder to signify that the
 caller does not want the remainder.
 The program does not alter the input parameters u and v.
 The quotient and remainder returned may have leading zeros.  The
 function itself returns a value of 0 for success and 1 for invalid
 parameters (e.g., division by 0).
 For now, we must have m >= n.  Knuth's Algorithm D also requires
 that the dividend be at least as long as the divisor.  (In his terms,
 m >= 0 (unstated).  Therefore m+n >= n.) */

 fn divqr_vect<T: Copy + Default + PartialOrd + Div<Output = T> + Rem<Output = T>>(
    qin: &mut ubint<T>,
    rin: &mut ubint<T>,
    uin: &ubint<T>,
    vin: &ubint<T>,
) {
    let u = &uin.m_value;
    let m = u.len();
    let v = &vin.m_value;
    let n = v.len();
    let q = &mut qin.m_value;
    q.resize(m - n + 1, Default::default());
    let r = &mut rin.m_value;
    let mut ofl: T = Default::default();
    if n == 1 {
        for i in (0..m).rev() {
            ofl = (ofl << m_limbBitLength) | u[i];
            q[i] = ofl / v[0];
            ofl %= v[0];
        }
        qin.NormalizeLimbs();
        r.resize(1, Default::default());
        r[0] = ofl;
        rin.m_MSB = lbcrypto::GetMSB(r[0]);
        return;
    }
    let sl = m_limbBitLength - lbcrypto::GetMSB(v[v.len() - 1]);
    let mut vn = Vec::with_capacity(n);
    ofl = Default::default();
    for i in 0..n {
        ofl |= (v[i] as Dlimb_t) << sl;
        vn.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    let mut un = Vec::with_capacity(m + 1);
    ofl = Default::default();
    for i in 0..m {
        ofl |= (u[i] as Dlimb_t) << sl;
        un.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    un.push(ofl as limb_t);
    let mut qhat: Dlimb_t;
    let mut rhat: Dlimb_t;
    let mut p: Dlimb_t;
    for j in (0..=m - n).rev() {
        ofl = (un[j + n] as Dlimb_t) << m_limbBitLength | un[j + n - 1] as Dlimb_t;
        qhat = ofl / vn[n - 1] as Dlimb_t;
        rhat = ofl % vn[n - 1] as Dlimb_t;
        while qhat >> m_limbBitLength != 0
            || qhat * vn[n - 2] as Dlimb_t > (rhat << m_limbBitLength) | un[j + n - 2] as Dlimb_t
        {
            qhat -= 1;
            rhat += vn[n - 1] as Dlimb_t;
            if rhat >> m_limbBitLength != 0 {
                break;
            }
        }
        let mut k: SDlimb_t = 0;
        let mut t: SDlimb_t;
        for i in 0..n {
            p = qhat * vn[i] as Dlimb_t;
            t = un[i + j] as SDlimb_t - k - (p & m_MaxLimb as Dlimb_t) as SDlimb_t;
            un[i + j] = t as limb_t;
            k = (p >> m_limbBitLength) as SDlimb_t - (t >> m_limbBitLength) as SDlimb_t;
        }
        t = un[j + n] as SDlimb_t - k;
        un[j + n] = t as limb_t;
        q[j] = qhat as limb_t;
        if t < 0 {
            q[j] -= 1;
            k = 0;
            for i in 0..n {
                t = un[i + j] as Dlimb_t + vn[i] as Dlimb_t + k as Dlimb_t;
                un[i + j] = t as limb_t;
                k = (t >> m_limbBitLength) as SDlimb_t;
            }
            un[j + n] = (un[j + n] as Dlimb_t + k as Dlimb_t) as limb_t;
        }
    }
    qin.NormalizeLimbs();
    ofl = (un[0] as Dlimb_t) >> sl;
    let sr = m_limbBitLength - sl;
    r.resize(n, Default::default());
    for i in 0..n {
        ofl |= (un[i + 1] as Dlimb_t) << sr;
        r[i] = ofl as limb_t;
        ofl >>= m_limbBitLength;
    }
    r[n] = (un[n] as Dlimb_t) >> sl;
    rin.NormalizeLimbs();
}

fn divq_vect<T: Copy + Default + PartialOrd + Div<Output = T> + Rem<Output = T>>(
    qin: &mut ubint<T>,
    uin: &ubint<T>,
    vin: &ubint<T>,
) {
    let u = &uin.m_value;
    let m = u.len();
    let v = &vin.m_value;
    let n = v.len();
    let q = &mut qin.m_value;
    q.resize(m - n + 1, Default::default());
    let mut ofl: T = Default::default();
    if n == 1 {
        for i in (0..m).rev() {
            ofl = (ofl << m_limbBitLength) | u[i];
            q[i] = ofl / v[0];
            ofl %= v[0];
        }
        qin.NormalizeLimbs();
        return;
    }
    let sl = m_limbBitLength - lbcrypto::GetMSB(v[v.len() - 1]);
    let mut vn = Vec::with_capacity(n);
    ofl = Default::default();
    for i in 0..n {
        ofl |= (v[i] as Dlimb_t) << sl;
        vn.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    let mut un = Vec::with_capacity(m + 1);
    ofl = Default::default();
    for i in 0..m {
        ofl |= (u[i] as Dlimb_t) << sl;
        un.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    un.push(ofl as limb_t);
    let mut qhat: Dlimb_t;
    let mut rhat: Dlimb_t;
    let mut p: Dlimb_t;
    for j in (0..=m - n).rev() {
        ofl = (un[j + n] as Dlimb_t) << m_limbBitLength | un[j + n - 1] as Dlimb_t;
        qhat = ofl / vn[n - 1] as Dlimb_t;
        rhat = ofl % vn[n - 1] as Dlimb_t;
        while qhat >> m_limbBitLength != 0
            || qhat * vn[n - 2] as Dlimb_t > (rhat << m_limbBitLength) | un[j + n - 2] as Dlimb_t
        {
            qhat -= 1;
            rhat += vn[n - 1] as Dlimb_t;
            if rhat >> m_limbBitLength != 0 {
                break;
            }
        }
        let mut k: SDlimb_t = 0;
        let mut t: SDlimb_t;
        for i in 0..n {
            p = qhat * vn[i] as Dlimb_t;
            t = un[i + j] as SDlimb_t - k - (p & m_MaxLimb as Dlimb_t) as SDlimb_t;
            un[i + j] = t as limb_t;
            k = (p >> m_limbBitLength) as SDlimb_t - (t >> m_limbBitLength) as SDlimb_t;
        }
        t = un[j + n] as SDlimb_t - k;
        un[j + n] = t as limb_t;
        q[j] = qhat as limb_t;
        if t < 0 {
            q[j] -= 1;
            k = 0;
            for i in 0..n {
                t = un[i + j] as Dlimb_t + vn[i] as Dlimb_t + k as Dlimb_t;
                un[i + j] = t as limb_t;
                k = (t >> m_limbBitLength) as SDlimb_t;
            }
            un[j + n] = (un[j + n] as Dlimb_t + k as Dlimb_t) as limb_t;
        }
    }
    qin.NormalizeLimbs();
}

fn divr_vect<T: Copy + Default + PartialOrd + Div<Output = T> + Rem<Output = T>>(
    rin: &mut ubint<T>,
    uin: &ubint<T>,
    vin: &ubint<T>,
) {
    let u = &uin.m_value;
    let m = u.len();
    let v = &vin.m_value;
    let n = v.len();
    let r = &mut rin.m_value;
    let mut ofl: T = Default::default();
    if n == 1 {
        let mut q = Vec::with_capacity(m - n + 1);
        for i in (0..m).rev() {
            ofl = (ofl << m_limbBitLength) | u[i];
            q.push(ofl / v[0]);
            ofl %= v[0];
        }
        r[0] = ofl;
        rin.m_MSB = lbcrypto::GetMSB(r[0]);
        return;
    }
    let sl = m_limbBitLength - lbcrypto::GetMSB(v[v.len() - 1]);
    let mut vn = Vec::with_capacity(n);
    ofl = Default::default();
    for i in 0..n {
        ofl |= (v[i] as Dlimb_t) << sl;
        vn.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    let mut un = Vec::with_capacity(m + 1);
    ofl = Default::default();
    for i in 0..m {
        ofl |= (u[i] as Dlimb_t) << sl;
        un.push(ofl as limb_t);
        ofl >>= m_limbBitLength;
    }
    un.push(ofl as limb_t);
    let mut qhat: Dlimb_t;
    let mut rhat: Dlimb_t;
    let mut p: Dlimb_t;
    for j in (0..=m - n).rev() {
        ofl = (un[j + n] as Dlimb_t) << m_limbBitLength | un[j + n - 1] as Dlimb_t;
        qhat = ofl / vn[n - 1] as Dlimb_t;
        rhat = ofl % vn[n - 1] as Dlimb_t;
        while qhat >> m_limbBitLength != 0
            || qhat * vn[n - 2] as Dlimb_t > (rhat << m_limbBitLength) | un[j + n - 2] as Dlimb_t
        {
            qhat -= 1;
            rhat += vn[n - 1] as Dlimb_t;
            if rhat >> m_limbBitLength != 0 {
                break;
            }
        }
        let mut k: SDlimb_t = 0;
        let mut t: SDlimb_t;
        for i in 0..n {
            p = qhat * vn[i] as Dlimb_t;
            t = un[i + j] as SDlimb_t - k - (p & m_MaxLimb as Dlimb_t) as SDlimb_t;
            un[i + j] = t as limb_t;
            k = (p >> m_limbBitLength) as SDlimb_t - (t >> m_limbBitLength) as SDlimb_t;
        }
        t = un[j + n] as SDlimb_t - k;
        un[j + n] = t as limb_t;
        if t < 0 {
            k = 0;
            for i in 0..n {
                t = un[i + j] as Dlimb_t + vn[i] as Dlimb_t + k as Dlimb_t;
                un[i + j] = t as limb_t;
                k = (t >> m_limbBitLength) as SDlimb_t;
            }
            un[j + n] = (un[j + n] as Dlimb_t + k as Dlimb_t) as limb_t;
        }
    }
    r.resize(n, Default::default());
    ofl = (un[0] as Dlimb_t) >> sl;
    let sr = m_limbBitLength - sl;
    for i in 0..n {
        ofl |= (un[i + 1] as Dlimb_t) << sr;
        r[i] = ofl as limb_t;
        ofl >>= m_limbBitLength;
    }
    r[n] = (un[n] as Dlimb_t) >> sl;
    rin.NormalizeLimbs();
}



// Initializes the vector of limbs from the string equivalent of ubint
// Algorithm used is repeated division by 2
// Reference:http://pctechtips.org/convert-from-decimal-to-binary-with-recursion-in-java/

use std::convert::TryInto;

pub struct Ubint<T> {
    m_value: Vec<T>,
    m_limb_bit_length: usize,
    m_MSB: usize,
}

impl<T> Ubint<T> {
    pub fn set_value(&mut self, vin: &str) {
        let mut v = vin.to_string();
        
        v = v.trim_start_matches('0').to_string();
        if v.is_empty() {
            v = "0".to_string();
        }
        let arr_size = v.len() - 1;
        let v: Vec<u8> = v.chars().map(|c| c.to_digit(10).unwrap().try_into().unwrap()).collect();
        self.m_value.clear();
        
        let mut cnt = 0;
        let mut val: T = 0.try_into().unwrap();
        let mut zptr = 0;
        while zptr <= arr_size {
            val |= (v[arr_size] & 0x1).try_into().unwrap() << cnt;
            for i in zptr..arr_size {
                v[i + 1] += (v[i] & 0x1) * 10;
                v[i] >>= 1;
            }
            v[arr_size] >>= 1;
            if v[zptr] == 0 {
                zptr += 1;
            }
            if cnt == self.m_limb_bit_length || zptr > arr_size {
                self.m_value.push(val);
                cnt = 0;
                val = 0.try_into().unwrap();
            }
        }
        self.normalize_limbs();
    }

    pub fn get_bit_at_index(&self, index: usize) -> u8 {
        let mask = self.m_limb_bit_length - 1;
        if index > self.m_MSB {
            return 0;
        }
        let idx = self.MSB_to_limbs(index) - 1;
        let index = index & mask;
        return ((self.m_value[idx] >> (if index != 0 { index - 1 } else { mask })) & 0x1).try_into().unwrap();
    }

    fn normalize_limbs(&mut self) {
        // implementation of NormalizeLimbs function
    }

    fn MSB_to_limbs(&self, index: usize) -> usize {
        // implementation of MSBToLimbs function
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_value() {
        let mut ubint: Ubint<u32> = Ubint {
            m_value: vec![],
            m_limb_bit_length: 32,
            m_MSB: 0,
        };
        ubint.set_value("12345");
        assert_eq!(ubint.m_value, vec![12345]);
    }

    #[test]
    fn test_get_bit_at_index() {
        let ubint: Ubint<u32> = Ubint {
            m_value: vec![12345],
            m_limb_bit_length: 32,
            m_MSB: 0,
        };
        assert_eq!(ubint.get_bit_at_index(0), 1);
        assert_eq!(ubint.get_bit_at_index(1), 0);
        assert_eq!(ubint.get_bit_at_index(2), 1);
        assert_eq!(ubint.get_bit_at_index(3), 0);
        assert_eq!(ubint.get_bit_at_index(4), 1);
    }
}




