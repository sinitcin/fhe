struct NativeIntegerT<IntType> {}

type NativeInteger = NativeIntegerT<BasicInteger>;

struct NativeVectorT<IntType> {}

struct DataTypes<utype> {}

impl DataTypes<uint32_t> {
    type SignedType = int32_t;
    type DoubleType = uint64_t;
    type SignedDoubleType = int64_t;
}

impl DataTypes<uint64_t> {
    type SignedType = int64_t;
    type DoubleType = uint64_t;
    type SignedDoubleType = int64_t;
}

#[cfg(HAVE_INT128)]
impl DataTypes<uint64_t> {
    type DoubleType = uint128_t;
    type SignedDoubleType = int128_t;
}

#[cfg(HAVE_INT128)]
struct DataTypes<uint128_t> {
    type SignedType = int128_t;
    type DoubleType = uint128_t;
    type SignedDoubleType = int128_t;
}


use std::ops::{Add, Sub, Mul, Div};

pub struct NativeIntegerT<NativeInt> {
    m_value: NativeInt,
}

impl<NativeInt> NativeIntegerT<NativeInt> {
    const M_UINT_MAX: NativeInt = std::mem::size_of::<NativeInt>() as NativeInt;
    const M_UINT_BIT_LENGTH: usint = std::mem::size_of::<NativeInt>() * 8;

    pub fn new() -> Self {
        Self { m_value: 0 }
    }

    pub fn from_value(val: NativeInt) -> Self {
        Self { m_value: val }
    }

    pub fn from_string(val: &str) -> Self {
        let mut acc: NativeInt = 0;
        let mut tst: NativeInt = 0;
        for c in val.chars() {
            if !c.is_digit(10) {
                panic!("String contains a non-digit");
            }
            let digit = c.to_digit(10).unwrap() as NativeInt;
            if acc.checked_mul(10).is_none() || acc.checked_add(digit).is_none() {
                panic!("{} is too large to fit in this native integer object", val);
            }
            acc = acc * 10 + digit;
            tst = acc;
        }
        Self { m_value: acc }
    }

    pub fn set_value(&mut self, val: NativeIntegerT<NativeInt>) {
        self.m_value = val.m_value;
    }

    pub fn set_identity(&mut self) {
        self.m_value = 1;
    }
}

impl<NativeInt> From<NativeIntegerT<NativeInt>> for NativeInt {
    fn from(val: NativeIntegerT<NativeInt>) -> Self {
        val.m_value
    }
}

impl<NativeInt> From<NativeIntegerT<NativeInt>> for bool {
    fn from(val: NativeIntegerT<NativeInt>) -> Self {
        val.m_value != 0
    }
}

impl<NativeInt> Add for NativeIntegerT<NativeInt>
where
    NativeInt: Add<NativeInt, Output = NativeInt>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_value(self.m_value + other.m_value)
    }
}

impl<NativeInt> Sub for NativeIntegerT<NativeInt>
where
    NativeInt: Sub<NativeInt, Output = NativeInt>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from_value(self.m_value - other.m_value)
    }
}

impl<NativeInt> Mul for NativeIntegerT<NativeInt>
where
    NativeInt: Mul<NativeInt, Output = NativeInt>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from_value(self.m_value * other.m_value)
    }
}

impl<NativeInt> Div for NativeIntegerT<NativeInt>
where
    NativeInt: Div<NativeInt, Output = NativeInt>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::from_value(self.m_value / other.m_value)
    }
}


use std::ops::{Add, AddAssign, Div, DivAssign, Rem, RemAssign, Mul, MulAssign, ShrAssign};
use std::convert::From;
use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
struct NativeIntegerT<T> {
    m_value: T,
}

impl<T> NativeIntegerT<T> {
    fn new(value: T) -> Self {
        NativeIntegerT { m_value: value }
    }
}

impl<T> From<T> for NativeIntegerT<T> {
    fn from(value: T) -> Self {
        NativeIntegerT { m_value: value }
    }
}

impl<T> PartialEq for NativeIntegerT<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.m_value == other.m_value
    }
}

impl<T> Add for NativeIntegerT<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        NativeIntegerT::new(self.m_value + other.m_value)
    }
}

impl<T> AddAssign for NativeIntegerT<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.m_value += other.m_value;
    }
}

impl<T> Mul for NativeIntegerT<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        NativeIntegerT::new(self.m_value * other.m_value)
    }
}

impl<T> MulAssign for NativeIntegerT<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        self.m_value *= other.m_value;
    }
}

impl<T> Div for NativeIntegerT<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        NativeIntegerT::new(self.m_value / other.m_value)
    }
}

impl<T> DivAssign for NativeIntegerT<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        self.m_value /= other.m_value;
    }
}

impl<T> Rem for NativeIntegerT<T>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        NativeIntegerT::new(self.m_value % other.m_value)
    }
}

impl<T> RemAssign for NativeIntegerT<T>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, other: Self) {
        self.m_value %= other.m_value;
    }
}

impl<T> ShrAssign<u32> for NativeIntegerT<T>
where
    T: ShrAssign<u32>,
{
    fn shr_assign(&mut self, other: u32) {
        self.m_value >>= other;
    }
}

impl<T> NativeIntegerT<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + PartialEq + From<u32> + Debug,
{
    fn exp(&self, p: u32) -> Self {
        let mut r: T = T::from(1);
        let mut x = self.m_value;
        let mut p = p;
        while p > 0 {
            p >>= 1;
            x *= x;
            r *= if p & 0x1 != 0 { x } else { T::from(1) };
        }
        NativeIntegerT::new(r)
    }

    fn exp_eq(&mut self, p: u32) -> &mut Self {
        let x = self.m_value;
        self.m_value = T::from(1);
        let mut p = p;
        while p > 0 {
            p >>= 1;
            x *= x;
            self.m_value *= if p & 0x1 != 0 { x } else { T::from(1) };
        }
        self
    }

    fn multiply_and_round(&self, p: &Self, q: &Self) -> Self {
        if q.m_value == T::from(0) {
            panic!("NativeIntegerT MultiplyAndRound: Divide by zero");
        }
        NativeIntegerT::new(
            (p.m_value as f64 * (self.m_value as f64 / q.m_value as f64) + 0.5) as T,
        )
    }

    fn multiply_and_round_eq(&mut self, p: &Self, q: &Self) -> &mut Self {
        if q.m_value == T::from(0) {
            panic!("NativeIntegerT MultiplyAndRoundEq: Divide by zero");
        }
        self.m_value = (p.m_value as f64 * (self.m_value as f64 / q.m_value as f64) + 0.5) as T;
        self
    }

    fn divide_and_round(&self, q: &Self) -> Self {
        if q.m_value == T::from(0) {
            panic!("NativeIntegerT DivideAndRound: zero");
        }
        let ans = self.m_value / q.m_value;
        let rem = self.m_value % q.m_value;
        let half_q = q.m_value >> 1;
        if rem > half_q {
            NativeIntegerT::new(ans + T::from(1))
        } else {
            NativeIntegerT::new(ans)
        }
    }

    fn divide_and_round_eq(&mut self, q: &Self) -> &mut Self {
        if q.m_value == T::from(0) {
            panic!("NativeIntegerT DivideAndRoundEq: zero");
        }
        let ans = self.m_value / q.m_value;
        let rem = self.m_value % q.m_value;
        let half_q = q.m_value >> 1;
        if rem > half_q {
            self.m_value = ans + T::from(1);
        } else {
            self.m_value = ans;
        }
        self
    }

    fn modulo(&self, modulus: &Self) -> Self {
        NativeIntegerT::new(self.m_value % modulus.m_value)
    }

    fn modulo_eq(&mut self, modulus: &Self) -> &mut Self {
        self.m_value %= modulus.m_value;
        self
    }

    fn compute_mu(&self) -> Self {
        if self.m_value == T::from(0) {
            panic!("NativeIntegerT ComputeMu: Divide by zero");
        }
        let tmp = T::from(1) << (2 * self.m_value.leading_zeros() + 3);
        NativeIntegerT::new(tmp / self.m_value)
    }

    fn modulo_with_mu(&self, modulus: &Self, mu: &Self) -> Self {
        let mut tmp: T;
        let mut ans = self.clone();
        let msb = modulus.m_value.leading_zeros() - 2;
        mod_mu(&mut tmp, &mut ans, modulus.m_value, mu.m_value, msb);
        ans
    }

    fn modulo_with_mu_eq(&mut self, modulus: &Self, mu: &Self) -> &mut Self {
        let mut tmp: T;
        let msb = modulus.m_value.leading_zeros() - 2;
        mod_mu(&mut tmp, self, modulus.m_value, mu.m_value, msb);
        self
    }

    fn modulo_add(&self, b: &Self, modulus: &Self) -> Self {
        let av = self.m_value;
        let bv = b.m_value;
        let mv = modulus.m_value;
        let av = if av >= mv { av % mv } else { av };
        let bv = if bv >= mv { bv % mv } else { bv };
        let mut r = av + bv;
        if r >= mv {
            r -= mv;
        }
        NativeIntegerT::new(r)
    }

    fn modulo_add_eq(&mut self, b: &Self, modulus: &Self) -> &mut Self {
        let bv = b.m_value;
        let mv = modulus.m_value;
        if self.m_value >= mv {
            self.m_value %= mv;
        }
        let bv = if bv >= mv { bv % mv } else { bv };
        self.m_value += bv;
        if self.m_value >= mv {
            self.m_value -= mv;
        }
        self
    }

    fn modulo_add_fast(&self, b: &Self, modulus: &Self) -> Self {
        let r = self.m_value + b.m_value;
        let mv = modulus.m_value;
        if r >= mv {
            NativeIntegerT::new(r - mv)
        } else {
            NativeIntegerT::new(r)
        }
    }

    fn modulo_add_fast_eq(&mut self, b: &Self, modulus: &Self) -> &mut Self {
        let mv = modulus.m_value;
        self.m_value += b.m_value;
        if self.m_value >= mv {
            self.m_value -= mv;
        }
        self
    }

    fn modulo_add_with_mu(&self, b: &Self, modulus: &Self, mu: &Self) -> Self {
        let mv = modulus.m_value;
        let av = self.clone();
        let bv = b.clone();
        let av = if av.m_value >= mv {
            av.modulo_with_mu(modulus, mu)
        } else {
            av
        };
        let bv = if bv.m_value >= mv {
            bv.modulo_with_mu(modulus, mu)
        } else {
            bv
        };
        let mut r = av.m_value + bv.m_value;
        if r >= mv {
            r -= mv;
        }
        NativeIntegerT::new(r)
    }

    fn modulo_add_with_mu_eq(&mut self, b: &Self, modulus: &Self, mu: &Self) -> &mut Self {
        let mv = modulus.m_value;
        let bv = b.clone();
        if self.m_value >= mv {
            self.modulo_with_mu_eq(modulus, mu);
        }
        if bv.m_value >= mv {
            bv.modulo_with_mu_eq(modulus, mu);
        }
        self.m_value += bv.m_value;
        if self.m_value >= mv {
            self.m_value -= mv;
        }
        self
    }
}

fn mod_mu<T>(tmp: &mut T, ans: &mut NativeIntegerT<T>, modulus: T, mu: T, msb: u32)
where
    T: Copy + Mul<Output = T> + Add<Output = T> + PartialEq + From<u32>,
{
    *tmp = T::from(1) << msb;
    *ans = NativeIntegerT::new((*ans * mu) % modulus);
}

fn main() {
    let a: NativeIntegerT<u32> = NativeIntegerT::new(10);
    let b: NativeIntegerT<u32> = NativeIntegerT::new(5);
    let modulus: NativeIntegerT<u32> = NativeIntegerT::new(7);
    let mu: NativeIntegerT<u32> = NativeIntegerT::new(3);

    let exp_result = a.exp(3);
    println!("exp_result: {:?}", exp_result);

    let mut exp_eq_result = a.clone();
    exp_eq_result.exp_eq(3);
    println!("exp_eq_result: {:?}", exp_eq_result);

    let multiply_and_round_result = a.multiply_and_round(&b, &modulus);
    println!("multiply_and_round_result: {:?}", multiply_and_round_result);

    let mut multiply_and_round_eq_result = a.clone();
    multiply_and_round_eq_result.multiply_and_round_eq(&b, &modulus);
    println!("multiply_and_round_eq_result: {:?}", multiply_and_round_eq_result);

    let divide_and_round_result = a.divide_and_round(&b);
    println!("divide_and_round_result: {:?}", divide_and_round_result);

    let mut divide_and_round_eq_result = a.clone();
    divide_and_round_eq_result.divide_and_round_eq(&b);
    println!("divide_and_round_eq_result: {:?}", divide_and_round_eq_result);

    let modulo_result = a.modulo(&modulus);
    println!("modulo_result: {:?}", modulo_result);

    let mut modulo_eq_result = a.clone();
    modulo_eq_result.modulo_eq(&modulus);
    println!("modulo_eq_result: {:?}", modulo_eq_result);

    let compute_mu_result = a.compute_mu();
    println!("compute_mu_result: {:?}", compute_mu_result);

    let modulo_with_mu_result = a.modulo_with_mu(&modulus, &mu);
    println!("modulo_with_mu_result: {:?}", modulo_with_mu_result);

    let mut modulo_with_mu_eq_result = a.clone();
    modulo_with_mu_eq_result.modulo_with_mu_eq(&modulus, &mu);
    println!("modulo_with_mu_eq_result: {:?}", modulo_with_mu_eq_result);

    let modulo_add_result = a.modulo_add(&b, &modulus);
    println!("modulo_add_result: {:?}", modulo_add_result);

    let mut modulo_add_eq_result = a.clone();
    modulo_add_eq_result.modulo_add_eq(&b, &modulus);
    println!("modulo_add_eq_result: {:?}", modulo_add_eq_result);

    let modulo_add_fast_result = a.modulo_add_fast(&b, &modulus);
    println!("modulo_add_fast_result: {:?}", modulo_add_fast_result);

    let mut modulo_add_fast_eq_result = a.clone();
    modulo_add_fast_eq_result.modulo_add_fast_eq(&b, &modulus);
    println!("modulo_add_fast_eq_result: {:?}", modulo_add_fast_eq_result);

    let modulo_add_with_mu_result = a.modulo_add_with_mu(&b, &modulus, &mu);
    println!("modulo_add_with_mu_result: {:?}", modulo_add_with_mu_result);

    let mut modulo_add_with_mu_eq_result = a.clone();
    modulo_add_with_mu_eq_result.modulo_add_with_mu_eq(&b, &modulus, &mu);
    println!("modulo_add_with_mu_eq_result: {:?}", modulo_add_with_mu_eq_result);
}


fn mod_add_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    let mut av = self.clone();
    let mut bv = b.clone();
    let mv = modulus.m_value;
    if av.m_value >= mv {
        av.mod_eq(modulus, mu);
    }
    if bv.m_value >= mv {
        bv.mod_eq(modulus, mu);
    }
    self.m_value = av.m_value + bv.m_value;
    if self.m_value >= mv {
        self.m_value -= mv;
    }
    self
}

fn mod_sub(&self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> NativeIntegerT {
    let av = self.m_value;
    let bv = b.m_value;
    let mv = modulus.m_value;
    if av >= mv {
        av %= mv;
    }
    if bv >= mv {
        bv %= mv;
    }
    if av < bv {
        return NativeIntegerT::new(av + mv - bv);
    }
    NativeIntegerT::new(av - bv)
}

fn mod_sub_eq(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> &mut NativeIntegerT {
    let av = self.m_value;
    let bv = b.m_value;
    let mv = modulus.m_value;
    if av >= mv {
        av %= mv;
    }
    if bv >= mv {
        bv %= mv;
    }
    if av < bv {
        *self = NativeIntegerT::new(av + mv - bv);
    } else {
        *self = NativeIntegerT::new(av - bv);
    }
    self
}

fn mod_sub_fast(&self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> NativeIntegerT {
    if self.m_value < b.m_value {
        return NativeIntegerT::new(self.m_value + modulus.m_value - b.m_value);
    }
    NativeIntegerT::new(self.m_value - b.m_value)
}

fn mod_sub_fast_eq(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> &mut NativeIntegerT {
    if self.m_value < b.m_value {
        *self = NativeIntegerT::new(self.m_value + modulus.m_value - b.m_value);
    } else {
        *self = NativeIntegerT::new(self.m_value - b.m_value);
    }
    self
}

fn mod_sub<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    let mv = modulus.m_value;
    #[cfg(NATIVEINT_BARRET_MOD)]
    {
        let mut av = self.clone();
        let mut bv = b.clone();
        if av.m_value >= mv {
            av.mod_eq(modulus, mu);
        }
        if bv.m_value >= mv {
            bv.mod_eq(modulus, mu);
        }
        if av.m_value < bv.m_value {
            return NativeIntegerT::new(av.m_value + mv - bv.m_value);
        }
        NativeIntegerT::new(av.m_value - bv.m_value)
    }
    #[cfg(not(NATIVEINT_BARRET_MOD))]
    {
        let av = self.m_value;
        let bv = b.m_value;
        if av >= mv {
            av %= mv;
        }
        if bv >= mv {
            bv %= mv;
        }
        if av < bv {
            return NativeIntegerT::new(av + mv - bv);
        }
        NativeIntegerT::new(av - bv)
    }
}

fn mod_sub<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    let mut av = self.clone();
    let mut bv = b.clone();
    let mv = modulus.m_value;
    if av.m_value >= mv {
        av.mod_eq(modulus, mu);
    }
    if bv.m_value >= mv {
        bv.mod_eq(modulus, mu);
    }
    if av.m_value < bv.m_value {
        return NativeIntegerT::new(av.m_value + mv - bv.m_value);
    }
    NativeIntegerT::new(av.m_value - bv.m_value)
}

fn mod_sub_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    let mv = modulus.m_value;
    #[cfg(NATIVEINT_BARRET_MOD)]
    {
        let mut av = self.clone();
        let mut bv = b.clone();
        if av.m_value >= mv {
            av.mod_eq(modulus, mu);
        }
        if bv.m_value >= mv {
            bv.mod_eq(modulus, mu);
        }
        if av.m_value < bv.m_value {
            *self = NativeIntegerT::new(av.m_value + mv - bv.m_value);
        } else {
            *self = NativeIntegerT::new(av.m_value - bv.m_value);
        }
    }
    #[cfg(not(NATIVEINT_BARRET_MOD))]
    {
        let bv = b.m_value;
        let av = self.m_value;
        if bv >= mv {
            bv %= mv;
        }
        if av >= mv {
            av %= mv;
        }
        if av < bv {
            *self = NativeIntegerT::new(av + mv - bv);
        } else {
            *self = NativeIntegerT::new(av - bv);
        }
    }
    self
}

fn mod_sub_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    let mut av = self.clone();
    let mut bv = b.clone();
    let mv = modulus.m_value;
    if av.m_value >= mv {
        av.mod_eq(modulus, mu);
    }
    if bv.m_value >= mv {
        bv.mod_eq(modulus, mu);
    }
    if av.m_value < bv.m_value {
        *self = NativeIntegerT::new(av.m_value + mv - bv.m_value);
    } else {
        *self = NativeIntegerT::new(av.m_value - bv.m_value);
    }
    self
}

fn mod_mul<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    let av = self.m_value;
    let bv = b.m_value;
    let mv = modulus.m_value;
    if av >= mv {
        av %= mv;
    }
    if bv >= mv {
        bv %= mv;
    }
    let rv = av as DNativeInt * bv as DNativeInt;
    let dmv = mv as DNativeInt;
    if rv >= dmv {
        return NativeIntegerT::new(rv % dmv);
    }
    NativeIntegerT::new(rv)
}

fn mod_mul<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    let mut tmp = typeD::default();
    let av = self.clone();
    let mv = modulus.m_value;
    let mu = modulus.compute_mu().m_value;
    let n = modulus.get_msb() - 2;
    if av.m_value >= mv {
        mod_mu(&mut tmp, &av, mv, mu, n);
    }
    let bv = b.clone();
    if bv.m_value >= mv {
        mod_mu(&mut tmp, &bv, mv, mu, n);
    }
    mult_d(av.m_value, bv.m_value, &mut tmp);
    let r = tmp;
    mult_d(r_shift_d(&mut tmp, n), mu, &mut tmp);
    mult_d(r_shift_d(&mut tmp, n + 7), mv, &mut tmp);
    subtract_d(&mut r, &tmp);
    if r.lo >= mv {
        r.lo -= mv;
    }
    NativeIntegerT::new(r.lo)
}

fn mod_mul_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    let av = self.m_value;
    let bv = b.m_value;
    let mv = modulus.m_value;
    if av >= mv {
        av %= mv;
    }
    if bv >= mv {
        bv %= mv;
    }
    let rv = av as DNativeInt * bv as DNativeInt;
    let dmv = mv as DNativeInt;
    if rv >= dmv {
        *self = NativeIntegerT::new(rv % dmv);
    } else {
        *self = NativeIntegerT::new(rv);
    }
    self
}

fn mod_mul_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    let av = self.clone();
    let mv = modulus.m_value;
    let mut tmp = typeD::default();
    let mu = modulus.compute_mu().m_value;
    let n = modulus.get_msb() - 2;
    if av.m_value >= mv {
        mod_mu(&mut tmp, &av, mv, mu, n);
    }
    let bv = b.clone();
    if bv.m_value >= mv {
        mod_mu(&mut tmp, &bv, mv, mu, n);
    }
    mult_d(av.m_value, bv.m_value, &mut tmp);
    let r = tmp;
    mult_d(r_shift_d(&mut tmp, n), mu, &mut tmp);
    mult_d(r_shift_d(&mut tmp, n + 7), mv, &mut tmp);
    subtract_d(&mut r, &tmp);
    self.m_value = r.lo;
    if self.m_value >= mv {
        self.m_value -= mv;
    }
    self
}

fn mod_mul<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    #[cfg(NATIVEINT_BARRET_MOD)]
    {
        let av = self.clone();
        let mv = modulus.m_value;
        let mut tmp = typeD::default();
        let n = modulus.get_msb() - 2;
        if av.m_value >= mv {
            mod_mu(&mut tmp, &av, mv, mu.m_value, n);
        }
        let bv = b.clone();
        if bv.m_value >= mv {
            mod_mu(&mut tmp, &bv, mv, mu.m_value, n);
        }
        mult_d(av.m_value, bv.m_value, &mut tmp);
        let rv = get_d(&tmp);
        mult_d(r_shift_d(&mut tmp, n), mu.m_value, &mut tmp);
        rv -= DNativeInt(mv) * (get_d(&tmp) >> n + 7);
        let mut r = typeD::default();
        r.lo = rv;
        if r.lo >= mv {
            r.lo -= mv;
        }
        NativeIntegerT::new(r.lo)
    }
    #[cfg(not(NATIVEINT_BARRET_MOD))]
    {
        let mv = modulus.m_value;
        let bv = b.m_value;
        let av = self.m_value;
        if bv >= mv {
            bv %= mv;
        }
        if av >= mv {
            av %= mv;
        }
        let rv = av as DNativeInt * bv as DNativeInt;
        let dmv = mv as DNativeInt;
        if rv >= dmv {
            return NativeIntegerT::new(rv % dmv);
        }
        NativeIntegerT::new(rv)
    }
}

fn mod_mul<T: NativeInt>(&self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> NativeIntegerT
where
    T: DNativeInt,
{
    let av = self.clone();
    let mv = modulus.m_value;
    let mut tmp = typeD::default();
    let n = modulus.get_msb() - 2;
    if av.m_value >= mv {
        mod_mu(&mut tmp, &av, mv, mu.m_value, n);
    }
    let bv = b.clone();
    if bv.m_value >= mv {
        mod_mu(&mut tmp, &bv, mv, mu.m_value, n);
    }
    mult_d(av.m_value, bv.m_value, &mut tmp);
    let r = tmp;
    mult_d(r_shift_d(&mut tmp, n), mu.m_value, &mut tmp);
    mult_d(r_shift_d(&mut tmp, n + 7), mv, &mut tmp);
    subtract_d(&mut r, &tmp);
    if r.lo >= mv {
        r.lo -= mv;
    }
    NativeIntegerT::new(r.lo)
}

fn mod_mul_eq<T: NativeInt>(&mut self, b: &NativeIntegerT, modulus: &NativeIntegerT, mu: &NativeIntegerT) -> &mut NativeIntegerT
where
    T: DNativeInt,
{
    #[cfg(NATIVEINT_BARRET_MOD)]
    {
        let av = self.clone();
        let bv = b.clone();
        let mv = modulus.m_value;
        let mut tmp = typeD::default();
        let muv = mu.m_value;
        let n = modulus.get_msb() - 2;
        if av.m_value >= mv {
            mod_mu(&mut tmp, &av, mv, muv, n);
        }
        if bv.m_value >= mv {
            mod_mu(&mut tmp, &bv, mv, muv, n);
        }
        mult_d(av.m_value, bv.m_value, &mut tmp);
        let rv = get_d(&tmp);
        mult_d(r_shift_d(&mut tmp, n), muv, &mut tmp);
        rv -= DNativeInt(mv) * (get_d(&tmp) >> n + 7);
        self.m_value = rv as NativeInt;
        if self.m_value >= mv {
            self.m_value -= mv;
        }
    }
    #[cfg(not(NATIVEINT_BARRET_MOD))]
    {
        let bv = b.m_value;
        let av = self.m_value;
        let mv = modulus.m_value;
        if bv >= mv {
            bv %= mv;
        }
        if av >= mv {
            av %= mv;
        }
        let rv = av as DNativeInt * bv as DNativeInt;
        let dmv = mv as DNativeInt;
        if rv >= dmv {
            self.m_value = (rv % dmv) as NativeInt;
        } else {
            self.m_value = rv as NativeInt;
        }
        if self.m_value >= mv {
            self.m_value -= mv;
        }
    }
    self
}

use std::ops::{Mul, Rem, Shr};

impl<T: Into<i64> + From<i64> + Copy> NativeIntegerT<T> {
    fn mod_mul_eq(&mut self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>, mu: &NativeIntegerT<T>) -> &mut Self {
        let n = modulus.get_msb() - 2;
        let av = *self;
        let bv = *b;
        let mv = modulus.m_value;
        let mut tmp: i64;
        if av.m_value >= mv {
            mod_mu(&mut tmp, av, mv, mu.m_value, n);
        }
        if bv.m_value >= mv {
            mod_mu(&mut tmp, bv, mv, mu.m_value, n);
        }
        mult_d(av.m_value, bv.m_value, tmp);
        let r = tmp;
        mult_d(r_shift_d(tmp, n), mu.m_value, tmp);
        mult_d(r_shift_d(tmp, n + 7), mv, tmp);
        subtract_d(r, tmp);
        self.m_value = r.lo;
        if r.lo >= mv {
            self.m_value -= mv;
        }
        self
    }

    fn mod_mul_fast(&self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>) -> NativeIntegerT<T> {
        let rv = DNativeInt::from(self.m_value) * DNativeInt::from(b.m_value);
        let dmv = DNativeInt::from(modulus.m_value);
        if rv >= dmv {
            rv % dmv
        } else {
            rv.into()
        }
    }

    fn mod_mul_fast_eq(&mut self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>) -> &mut Self {
        let rv = DNativeInt::from(self.m_value) * DNativeInt::from(b.m_value);
        let dmv = DNativeInt::from(modulus.m_value);
        if rv >= dmv {
            rv % dmv
        } else {
            rv.into()
        }
        self
    }

    fn mod_mul_fast_const(&self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>, b_inv: &NativeIntegerT<T>) -> NativeIntegerT<T> {
        let q = mult_d_hi(self.m_value, b_inv.m_value) + 1;
        let yprime = (self.m_value * b.m_value - q * modulus.m_value) as SignedNativeInt;
        if yprime >= 0 {
            yprime.into()
        } else {
            (yprime + modulus.m_value).into()
        }
    }

    fn mod_mul_fast_const_eq(&mut self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>, b_inv: &NativeIntegerT<T>) -> &mut Self {
        let q = mult_d_hi(self.m_value, b_inv.m_value) + 1;
        let yprime = (self.m_value * b.m_value - q * modulus.m_value) as SignedNativeInt;
        self.m_value = if yprime >= 0 {
            yprime
        } else {
            yprime + modulus.m_value
        };
        self
    }

    fn mod_exp(&self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>) -> NativeIntegerT<T> {
        let t = self.m_value;
        let p = b.m_value;
        let m = modulus.m_value;
        let mut r = 1;
        if p & 0x1 != 0 {
            r = r * t;
            if r >= m {
                r = r % m;
            }
        }
        let mut p = p;
        while p >>= 1 {
            t = t * t;
            if t >= m {
                t = t % m;
            }
            if p & 0x1 != 0 {
                r = r * t;
                if r >= m {
                    r = r % m;
                }
            }
        }
        r.into()
    }

    fn mod_exp_eq(&mut self, b: &NativeIntegerT<T>, modulus: &NativeIntegerT<T>) -> &mut Self {
        *self = self.mod_exp(b, modulus);
        self
    }

    fn mod_inverse(&self, modulus: &NativeIntegerT<T>) -> NativeIntegerT<T> {
        let modulus = modulus.m_value as SignedNativeInt;
        let a = self.m_value % modulus;
        if a == 0 {
            let msg = format!("{} does not have a ModInverse using {}", self.m_value, modulus);
            panic!(msg);
        }
        if modulus == 1 {
            NativeIntegerT::default()
        } else {
            let mut y = 0;
            let mut x = 1;
            let mut a = a;
            while a > 1 {
                let t = modulus;
                let q = a / t;
                modulus = a % t;
                a = t;
                let t = y;
                y = x - q * y;
                x = t;
            }
            if x < 0 {
                x += modulus;
            }
            x.into()
        }
    }

    fn mod_inverse_eq(&mut self, modulus: &NativeIntegerT<T>) -> &mut Self {
        *self = self.mod_inverse(modulus);
        self
    }

    fn l_shift(&self, shift: u16) -> NativeIntegerT<T> {
        (self.m_value << shift).into()
    }

    fn l_shift_eq(&mut self, shift: u16) -> &mut Self {
        self.m_value = self.m_value << shift;
        self
    }

    fn r_shift(&self, shift: u16) -> NativeIntegerT<T> {
        (self.m_value >> shift).into()
    }

    fn r_shift_eq(&mut self, shift: u16) -> &mut Self {
        self.m_value = self.m_value >> shift;
        self
    }
}

fn mod_mu<T: Into<i64> + From<i64> + Copy>(tmp: &mut i64, av: NativeIntegerT<T>, mv: T, mu: T, n: i64) {
    // implementation of ModMu function
}

fn mult_d<T: Into<i64> + From<i64> + Copy>(av: T, bv: T, tmp: &mut i64) {
    // implementation of MultD function
}

fn subtract_d<T: Into<i64> + From<i64> + Copy>(r: &mut i64, tmp: i64) {
    // implementation of SubtractD function
}

fn r_shift_d<T: Into<i64> + From<i64> + Copy>(tmp: i64, n: i64) -> i64 {
    // implementation of RShiftD function
}

fn mult_d_hi<T: Into<i64> + From<i64> + Copy>(av: T, bv: T) -> i64 {
    // implementation of MultDHi function
}

struct NativeIntegerT<T> {
    m_value: T,
}

impl<T> NativeIntegerT<T> {
    fn get_msb(&self) -> i64 {
        // implementation of GetMSB function
    }
}

impl<T: Into<i64> + From<i64> + Copy> From<i64> for NativeIntegerT<T> {
    fn from(value: i64) -> Self {
        NativeIntegerT { m_value: value.into() }
    }
}

impl<T: Into<i64> + From<i64> + Copy> Mul for NativeIntegerT<T> {
    type Output = NativeIntegerT<T>;

    fn mul(self, rhs: NativeIntegerT<T>) -> Self::Output {
        self.mod_mul_fast(&rhs, &NativeIntegerT::default())
    }
}

impl<T: Into<i64> + From<i64> + Copy> Rem for NativeIntegerT<T> {
    type Output = NativeIntegerT<T>;

    fn rem(self, rhs: NativeIntegerT<T>) -> Self::Output {
        self.mod_mul_fast(&rhs, &NativeIntegerT::default())
    }
}

impl<T: Into<i64> + From<i64> + Copy> Shr<u16> for NativeIntegerT<T> {
    type Output = NativeIntegerT<T>;

    fn shr(self, rhs: u16) -> Self::Output {
        self.r_shift(rhs)
    }
}

use std::convert::TryInto;
use std::ops::{BitAnd, BitOr, Shl, Shr, Sub};
use std::cmp::PartialOrd;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct NativeIntegerT {
    m_value: u64,
}

impl NativeIntegerT {
    fn new(value: u64) -> Self {
        NativeIntegerT { m_value: value }
    }

    fn compare(&self, a: &NativeIntegerT) -> i32 {
        if self.m_value < a.m_value {
            -1
        } else if self.m_value > a.m_value {
            1
        } else {
            0
        }
    }

    fn convert_to_int<T>(&self) -> T
    where
        T: From<u64>,
    {
        self.m_value.try_into().unwrap()
    }

    fn convert_to_double(&self) -> f64 {
        self.m_value as f64
    }

    fn from_binary_string(bit_string: &str) -> Self {
        if bit_string.len() > NativeIntegerT::max_bits() {
            panic!("Bit string is too long to fit in an intnat");
        }
        let mut v: u64 = 0;
        for c in bit_string.chars() {
            let n = c.to_digit(10).unwrap();
            if n < 0 || n > 1 {
                panic!("Bit string must contain only 0 or 1");
            }
            v = (v << 1) | n as u64;
        }
        NativeIntegerT::new(v)
    }

    fn get_msb(&self) -> usize {
        self.m_value.leading_zeros() as usize
    }

    fn get_length_for_base(&self, base: usize) -> usize {
        NativeIntegerT::get_msb()
    }

    fn get_digit_at_index_for_base(&self, index: usize, base: usize) -> usize {
        let digit_len = f64::ceil(f64::log2(base as f64)) as usize;
        let mut digit = 0;
        let mut new_index = 1 + (index - 1) * digit_len;
        for i in 1..base {
            digit += self.get_bit_at_index(new_index) * i;
            new_index += 1;
        }
        digit
    }

    fn get_bit_at_index(&self, index: usize) -> u8 {
        if index == 0 {
            panic!("Zero index in GetBitAtIndex");
        }
        ((self.m_value >> (index - 1)) & 0x1) as u8
    }

    fn allocator() -> Self {
        NativeIntegerT::new(0)
    }

    fn to_string(&self) -> String {
        self.m_value.to_string()
    }

    fn integer_type_name() -> &'static str {
        "UBNATINT"
    }

    fn max_bits() -> usize {
        64
    }
}

impl Display for NativeIntegerT {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for NativeIntegerT {
    fn eq(&self, other: &Self) -> bool {
        self.m_value == other.m_value
    }
}

impl PartialOrd for NativeIntegerT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.m_value.partial_cmp(&other.m_value)
    }
}

impl BitAnd for NativeIntegerT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        NativeIntegerT::new(self.m_value & rhs.m_value)
    }
}

impl BitOr for NativeIntegerT {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        NativeIntegerT::new(self.m_value | rhs.m_value)
    }
}

impl Shl<usize> for NativeIntegerT {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        NativeIntegerT::new(self.m_value << rhs)
    }
}

impl Shr<usize> for NativeIntegerT {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        NativeIntegerT::new(self.m_value >> rhs)
    }
}

impl Sub for NativeIntegerT {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NativeIntegerT::new(self.m_value - rhs.m_value)
    }
}

impl FromStr for NativeIntegerT {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u64::from_str(s)?;
        Ok(NativeIntegerT::new(value))
    }
}

fn main() {
    let a = NativeIntegerT::new(10);
    let b = NativeIntegerT::new(5);
    let c = a + b;
    println!("{}", c);
}




