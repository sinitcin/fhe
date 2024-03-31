const BLOCK_VECTOR_ALLOCATION: u64 = 0;

pub struct NativeVector<T> {
    modulus: T,
    data: Vec<T>,
}

impl<T> NativeVector<T> {
    pub fn new(length: usize) -> Self {
        NativeVector {
            modulus: Default::default(),
            data: vec![Default::default(); length],
        }
    }

    pub fn single(val: T, modulus: T) -> Self {
        NativeVector {
            modulus,
            data: vec![val],
        }
    }

    pub fn with_modulus(length: usize, modulus: T) -> Self {
        NativeVector {
            modulus,
            data: vec![Default::default(); length],
        }
    }

    pub fn with_modulus_and_value(length: usize, modulus: T, val: T) -> Self {
        NativeVector {
            modulus,
            data: vec![val.modulus(modulus); length],
        }
    }

    pub fn set_modulus(&mut self, value: T) {
        if value.get_msb() > MAX_MODULUS_SIZE {
            let err_msg = format!(
                "Requested modulus' size {} is not supported. NativeVectorT supports only modulus size <= {}",
                value.get_msb(),
                MAX_MODULUS_SIZE
            );
            panic!(err_msg);
        }
        self.modulus = value;
    }
}

impl<T: Clone> Clone for NativeVector<T> {
    fn clone(&self) -> Self {
        NativeVector {
            modulus: self.modulus.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T> std::ops::Index<usize> for NativeVector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T> std::ops::IndexMut<usize> for NativeVector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

use std::ops::{Add, Sub, Mul};

fn switch_modulus<T: IntegerType>(value: &T) {}

impl<T: IntegerType> NativeVectorT<T> {
    fn get_modulus(&self) -> &T {
        &self.m_modulus
    }
    
    fn get_length(&self) -> usize {
        self.m_data.len()
    }
    
    fn mod_eq(&mut self, modulus: &T) -> &mut Self {
        self.m_modulus = modulus.clone();
        self
    }
    
    fn mod_add(&self, b: &T) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] += b;
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_add_eq(&mut self, b: &T) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] += b;
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_add_at_index(&self, i: usize, b: &T) -> Self {
        let mut result = self.clone();
        result.m_data[i] += b;
        result.m_data[i] %= self.m_modulus;
        result
    }
    
    fn mod_add_at_index_eq(&mut self, i: usize, b: &T) -> &mut Self {
        self.m_data[i] += b;
        self.m_data[i] %= self.m_modulus;
        self
    }
    
    fn mod_add_vec(&self, b: &Self) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] += b.m_data[i];
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_add_vec_eq(&mut self, b: &Self) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] += b.m_data[i];
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_add_no_check_eq(&mut self, b: &Self) -> &mut Self {
        let size = self.m_data.len();
        let mv = self.m_modulus.clone();
        for i in 0..size {
            self.m_data[i].mod_add_fast_eq(&b.m_data[i], &mv);
        }
        self
    }
    
    fn mod_sub(&self, b: &T) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] -= b;
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_sub_eq(&mut self, b: &T) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] -= b;
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_sub_vec(&self, b: &Self) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] -= b.m_data[i];
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_sub_vec_eq(&mut self, b: &Self) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] -= b.m_data[i];
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_mul(&self, b: &T) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] *= b;
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_mul_eq(&mut self, b: &T) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] *= b;
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_mul_vec(&self, b: &Self) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] *= b.m_data[i];
            result.m_data[i] %= self.m_modulus;
        }
        result
    }
    
    fn mod_mul_vec_eq(&mut self, b: &Self) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] *= b.m_data[i];
            self.m_data[i] %= self.m_modulus;
        }
        self
    }
    
    fn mod_mul_no_check_eq(&mut self, b: &Self) -> &mut Self {
        let size = self.m_data.len();
        let mv = self.m_modulus.clone();
        for i in 0..size {
            self.m_data[i].mod_mul_fast_eq(&b.m_data[i], &mv);
        }
        self
    }
    
    fn mult_without_mod(&self, b: &Self) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] *= b.m_data[i];
        }
        result
    }
    
    fn mod_exp(&self, b: &T) -> Self {
        let mut result = self.clone();
        for i in 0..self.m_data.len() {
            result.m_data[i] = result.m_data[i].mod_exp(b, &self.m_modulus);
        }
        result
    }
    
    fn mod_exp_eq(&mut self, b: &T) -> &mut Self {
        for i in 0..self.m_data.len() {
            self.m_data[i] = self.m_data[i].mod_exp(b, &self.m_modulus);
        }
        self
    }
    
    fn mod_inverse(&self) -> Self {
        let size = self.m_data.len();
        let mv = self.m_modulus.clone();
        let mut ans = Self::new(size, mv);
        for i in 0..size {
            ans.m_data[i] = self.m_data[i].mod_inverse(&mv);
        }
        ans
    }
}



use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Shr;
use std::ops::Sub;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::ops::Add;
use std::ops::BitAndAssign;
use std::ops::BitOrAssign;
use std::ops::BitXorAssign;
use std::ops::ShrAssign;
use std::ops::SubAssign;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::RemAssign;
use std::ops::NotAssign;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::transmute;
use std::num::Wrapping;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::copy_nonoverlapping;
use std::ptr::NonNull;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::slice::Iter;
use std::slice::IterMut;
use std::slice::Split;
use std::slice::SplitMut;
use std::slice::SplitN;
use std::slice::SplitNMut;
use std::slice::Windows;
use std::vec::IntoIter;
use std::vec::Vec;

pub struct NativeVectorT<T> {
    m_data: Vec<T>,
    m_modulus: T,
}

impl<T> NativeVectorT<T> {
    pub fn new() -> Self {
        NativeVectorT {
            m_data: Vec::new(),
            m_modulus: T::default(),
        }
    }

    pub fn ModInverse(&self, mv: T) -> T {
        unimplemented!()
    }

    pub fn ModInverseEq(&mut self) -> &mut Self {
        let size = self.m_data.len();
        let mv = self.m_modulus;
        for i in 0..size {
            self.m_data[i] = self.m_data[i].ModInverse(mv);
        }
        self
    }

    pub fn ModByTwo(&self) -> NativeVectorT<T> {
        unimplemented!()
    }

    pub fn ModByTwoEq(&mut self) -> &mut Self {
        unimplemented!()
    }

    pub fn MultiplyAndRound(&self, p: IntegerType, q: IntegerType) -> NativeVectorT<T> {
        unimplemented!()
    }

    pub fn MultiplyAndRoundEq(&mut self, p: IntegerType, q: IntegerType) -> &mut Self {
        unimplemented!()
    }

    pub fn DivideAndRound(&self, q: IntegerType) -> NativeVectorT<T> {
        unimplemented!()
    }

    pub fn DivideAndRoundEq(&mut self, q: IntegerType) -> &mut Self {
        unimplemented!()
    }

    pub fn GetDigitAtIndexForBase(&self, index: usint, base: usint) -> NativeVectorT<T> {
        unimplemented!()
    }
}

impl<T> Debug for NativeVectorT<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let len = self.m_data.len();
        write!(f, "[")?;
        for i in 0..len {
            write!(f, "{}", self.m_data[i])?;
            if i == len - 1 {
                write!(f, "]")?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, " modulus: {}", self.m_modulus)
    }
}

impl<T> PartialEq for NativeVectorT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.m_data == other.m_data && self.m_modulus == other.m_modulus
    }
}

impl<T> Eq for NativeVectorT<T> {}

impl<T> PartialOrd for NativeVectorT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for NativeVectorT<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.m_data.cmp(&other.m_data).then(self.m_modulus.cmp(&other.m_modulus))
    }
}

impl<T> Hash for NativeVectorT<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.m_data.hash(state);
        self.m_modulus.hash(state);
    }
}

impl<T> Display for NativeVectorT<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let len = self.m_data.len();
        write!(f, "[")?;
        for i in 0..len {
            write!(f, "{}", self.m_data[i])?;
            if i == len - 1 {
                write!(f, "]")?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, " modulus: {}", self.m_modulus)
    }
}

impl<T> Deref for NativeVectorT<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.m_data
    }
}

impl<T> DerefMut for NativeVectorT<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m_data
    }
}

impl<T> NativeVectorT<T> {
    pub fn SerializedObjectName(&self) -> String {
        "NativeVectorT".to_string()
    }

    pub fn SerializedVersion() -> u32 {
        1
    }
}

impl<T> Serialize for NativeVectorT<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.m_data.len();
        let mut seq = serializer.serialize_seq(Some(len + 1))?;
        for i in 0..len {
            seq.serialize_element(&self.m_data[i])?;
        }
        seq.serialize_element(&self.m_modulus)?;
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for NativeVectorT<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NativeVectorTVisitor<T> {
            marker: PhantomData<fn() -> NativeVectorT<T>>,
        }

        impl<'de, T> Visitor<'de> for NativeVectorTVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = NativeVectorT<T>;

            fn expecting(&self, formatter: &mut Formatter) -> Result {
                formatter.write_str("struct NativeVectorT")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<NativeVectorT<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut m_data = Vec::new();
                while let Some(value) = seq.next_element()? {
                    m_data.push(value);
                }
                let m_modulus = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(NativeVectorT { m_data, m_modulus })
            }
        }

        deserializer.deserialize_seq(NativeVectorTVisitor {
            marker: PhantomData,
        })
    }
}

impl<T> NativeVectorT<T> {
    pub fn save<S>(&self, serializer: S, version: u32) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.m_data.len();
        let mut seq = serializer.serialize_seq(Some(len + 1))?;
        for i in 0..len {
            seq.serialize_element(&self.m_data[i])?;
        }
        seq.serialize_element(&self.m_modulus)?;
        seq.end()
    }

    pub fn load<'de, D>(&mut self, deserializer: D, version: u32) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NativeVectorTVisitor<T> {
            marker: PhantomData<fn() -> NativeVectorT<T>>,
        }

        impl<'de, T> Visitor<'de> for NativeVectorTVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = NativeVectorT<T>;

            fn expecting(&self, formatter: &mut Formatter) -> Result {
                formatter.write_str("struct NativeVectorT")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<NativeVectorT<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut m_data = Vec::new();
                while let Some(value) = seq.next_element()? {
                    m_data.push(value);
                }
                let m_modulus = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(NativeVectorT { m_data, m_modulus })
            }
        }

        let visitor = NativeVectorTVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

impl<T> NativeVectorT<T> {
    pub fn save<S>(&self, serializer: S, version: u32) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.m_data.len();
        let mut seq = serializer.serialize_seq(Some(len + 1))?;
        for i in 0..len {
            seq.serialize_element(&self.m_data[i])?;
        }
        seq.serialize_element(&self.m_modulus)?;
        seq.end()
    }

    pub fn load<'de, D>(&mut self, deserializer: D, version: u32) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NativeVectorTVisitor<T> {
            marker: PhantomData<fn() -> NativeVectorT<T>>,
        }

        impl<'de, T> Visitor<'de> for NativeVectorTVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = NativeVectorT<T>;

            fn expecting(&self, formatter: &mut Formatter) -> Result {
                formatter.write_str("struct NativeVectorT")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<NativeVectorT<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut m_data = Vec::new();
                while let Some(value) = seq.next_element()? {
                    m_data.push(value);
                }
                let m_modulus = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(NativeVectorT { m_data, m_modulus })
            }
        }

        let visitor = NativeVectorTVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

impl<T> NativeVectorT<T> {
    pub fn save<S>(&self, serializer: S, version: u32) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.m_data.len();
        let mut seq = serializer.serialize_seq(Some(len + 1))?;
        for i in 0..len {
            seq.serialize_element(&self.m_data[i])?;
        }
        seq.serialize_element(&self.m_modulus)?;
        seq.end()
    }

    pub fn load<'de, D>(&mut self, deserializer: D, version: u32) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NativeVectorTVisitor<T> {
            marker: PhantomData<fn() -> NativeVectorT<T>>,
        }

        impl<'de, T> Visitor<'de> for NativeVectorTVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = NativeVectorT<T>;

            fn expecting(&self, formatter: &mut

