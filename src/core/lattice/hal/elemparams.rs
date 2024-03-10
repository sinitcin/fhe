use std::fmt;
use serde::{Serialize, Deserialize};

pub trait Serializable {
    fn save(&self, ar: &mut dyn Archive, version: u32);
    fn load(&mut self, ar: &mut dyn Archive, version: u32);
    fn serialized_object_name(&self) -> String;
    fn serialized_version() -> u32;
}

pub struct ElemParams<IntegerType> {
    m_ring_dimension: u32,
    m_cyclotomic_order: u32,
    m_ciphertext_modulus: IntegerType,
    m_root_of_unity: IntegerType,
    m_big_ciphertext_modulus: IntegerType,
    m_big_root_of_unity: IntegerType,
}

impl<IntegerType> ElemParams<IntegerType> {
    pub fn new(order: u32, ct_modulus: IntegerType) -> Self {
        let ring_dimension = get_totient(order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: order,
            m_ciphertext_modulus: ct_modulus,
            m_root_of_unity: IntegerType::default(),
            m_big_ciphertext_modulus: IntegerType::default(),
            m_big_root_of_unity: IntegerType::default(),
        }
    }

    pub fn new_with_runity(order: u32, ct_modulus: IntegerType, r_unity: IntegerType) -> Self {
        let ring_dimension = get_totient(order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: order,
            m_ciphertext_modulus: ct_modulus,
            m_root_of_unity: r_unity,
            m_big_ciphertext_modulus: IntegerType::default(),
            m_big_root_of_unity: IntegerType::default(),
        }
    }

    pub fn new_with_big_modulus(
        order: u32,
        ct_modulus: IntegerType,
        r_unity: IntegerType,
        big_ct_modulus: IntegerType,
        big_r_unity: IntegerType,
    ) -> Self {
        let ring_dimension = get_totient(order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: order,
            m_ciphertext_modulus: ct_modulus,
            m_root_of_unity: r_unity,
            m_big_ciphertext_modulus: big_ct_modulus,
            m_big_root_of_unity: big_r_unity,
        }
    }

    pub fn get_cyclotomic_order(&self) -> u32 {
        self.m_cyclotomic_order
    }

    pub fn get_ring_dimension(&self) -> u32 {
        self.m_ring_dimension
    }

    pub fn get_modulus(&self) -> &IntegerType {
        &self.m_ciphertext_modulus
    }

    pub fn get_big_modulus(&self) -> &IntegerType {
        &self.m_big_ciphertext_modulus
    }

    pub fn get_root_of_unity(&self) -> &IntegerType {
        &self.m_root_of_unity
    }

    pub fn get_big_root_of_unity(&self) -> &IntegerType {
        &self.m_big_root_of_unity
    }
}

impl<IntegerType: fmt::Debug> fmt::Debug for ElemParams<IntegerType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElemParams")
            .field("m_ring_dimension", &self.m_ring_dimension)
            .field("m_cyclotomic_order", &self.m_cyclotomic_order)
            .field("m_ciphertext_modulus", &self.m_ciphertext_modulus)
            .field("m_root_of_unity", &self.m_root_of_unity)
            .field("m_big_ciphertext_modulus", &self.m_big_ciphertext_modulus)
            .field("m_big_root_of_unity", &self.m_big_root_of_unity)
            .finish()
    }
}

impl<IntegerType: PartialEq> PartialEq for ElemParams<IntegerType> {
    fn eq(&self, other: &Self) -> bool {
        self.m_ring_dimension == other.m_ring_dimension
            && self.m_cyclotomic_order == other.m_cyclotomic_order
            && self.m_ciphertext_modulus == other.m_ciphertext_modulus
            && self.m_root_of_unity == other.m_root_of_unity
            && self.m_big_ciphertext_modulus == other.m_big_ciphertext_modulus
            && self.m_big_root_of_unity == other.m_big_root_of_unity
    }
}

impl<IntegerType: Serialize> Serializable for ElemParams<IntegerType> {
    fn save(&self, ar: &mut dyn Archive, version: u32) {
        ar.save("co", &self.m_cyclotomic_order);
        ar.save("rd", &self.m_ring_dimension);
        ar.save("cm", &self.m_ciphertext_modulus);
        ar.save("ru", &self.m_root_of_unity);
        ar.save("bm", &self.m_big_ciphertext_modulus);
        ar.save("br", &self.m_big_root_of_unity);
    }

    fn load(&mut self, ar: &mut dyn Archive, version: u32) {
        if version > Self::serialized_version() {
            panic!(
                "serialized object version {} is from a later version of the library",
                version
            );
        }
        ar.load("co", &mut self.m_cyclotomic_order);
        ar.load("rd", &mut self.m_ring_dimension);
        ar.load("cm", &mut self.m_ciphertext_modulus);
        ar.load("ru", &mut self.m_root_of_unity);
        ar.load("bm", &mut self.m_big_ciphertext_modulus);
        ar.load("br", &mut self.m_big_root_of_unity);
    }

    fn serialized_object_name(&self) -> String {
        String::from("ElemParams")
    }

    fn serialized_version() -> u32 {
        1
    }
}

fn get_totient(order: u32) -> u32 {
    // implementation of GetTotient function
    // ...
    0
}


