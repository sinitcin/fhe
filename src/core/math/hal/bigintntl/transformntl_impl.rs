
// ATTENTION: this file contains implementations of the functions
//            declared in math/bigintntl/transformntl.h and
//            MUST be included in the end of math/bigintntl/transformntl.h ONLY
//            and nowhere else

use std::collections::HashMap;

lazy_static! {
    static ref M_CYCLOORDERINVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOORDERINVERSEPRECON_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSEREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYPRECONREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSEPRECONREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYREVERSE_NTTMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYNTTMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYTABLEBYMODULUSROOT: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSETABLEBYMODULUSROOT: HashMap<usize, VecType> = HashMap::new();
    static ref M_POWERSTABLEBYMODULUSROOT: HashMap<usize, VecType> = HashMap::new();
    static ref M_RBTABLEBYMODULUSROOTPAIR: HashMap<usize, VecType> = HashMap::new();
    static ref M_DEFAULTNTTMODULUSROOT: HashMap<usize, ModulusRoot<usize>> = HashMap::new();
    static ref M_ROOTOFUNITYDIVISIONTABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYDIVISIONINVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_DIVISIONNTTMODULUS: HashMap<usize, usize> = HashMap::new();
    static ref M_DIVISIONNTTROOTOFUNITY: HashMap<usize, usize> = HashMap::new();
    static ref M_NTTDIVISIONDIM: HashMap<usize, usize> = HashMap::new();
}

fn forward_transform_iterative(element: &VecType, root_of_unity_table: &VecType, result: &mut VecType) {
    let n = element.len();
    if result.len() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    result.set_modulus(modulus);
    let msb = (n - 1).leading_zeros();
    for i in 0..n {
        result[i] = element[reverse_bits(i, msb)];
    }
    let mut omega;
    let mut omega_factor;
    let mut odd_val;
    let mut even_val;
    let logm;
    let mut i;
    let mut j;
    let mut index_even;
    let mut index_odd;
    let logn = (n - 1).leading_zeros();
    for logm in 1..=logn {
        let mut indexes = Vec::with_capacity(1 << (logm - 1));
        for i in 0..(1 << (logm - 1)) {
            indexes.push(i << (logn - logm));
        }
        for j in (0..n).step_by(1 << logm) {
            for i in 0..(1 << (logm - 1)) {
                omega = root_of_unity_table[indexes[i]];
                index_even = j + i;
                index_odd = index_even + (1 << (logm - 1));
                odd_val = result[index_odd];
                omega_factor = omega.mod_mul(odd_val, modulus, mu);
                even_val = result[index_even];
                odd_val = even_val;
                odd_val += omega_factor;
                if odd_val >= modulus {
                    odd_val -= modulus;
                }
                if even_val < omega_factor {
                    even_val += modulus;
                }
                even_val -= omega_factor;
                result[index_even] = odd_val;
                result[index_odd] = even_val;
            }
        }
    }
}

fn inverse_transform_iterative(element: &VecType, root_of_unity_inverse_table: &VecType, result: &mut VecType) {
    let n = element.len();
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    forward_transform_iterative(element, root_of_unity_inverse_table, result);
    let cyclo_order_inv = IntType::from(n).mod_inverse(modulus);
    for i in 0..n {
        result[i].mod_mul_eq(cyclo_order_inv, modulus, mu);
    }
}

fn forward_transform_to_bit_reverse_in_place(root_of_unity_table: &VecType, element: &mut VecType) {
    let n = element.len();
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    let mut i;
    let mut m;
    let mut j1;
    let mut j2;
    let mut index_omega;
    let mut index_lo;
    let mut index_hi;
    let mut omega;
    let mut omega_factor;
    let mut lo_val;
    let mut hi_val;
    let zero = IntType::from(0);
    let mut t = n >> 1;
    let mut logt1 = t.leading_zeros();
    for m in 1..n {
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            index_omega = m + i;
            omega = root_of_unity_table[index_omega];
            for index_lo in j1..j2 {
                index_hi = index_lo + t;
                lo_val = element[index_lo];
                omega_factor = element[index_hi];
                omega_factor.mod_mul_fast_eq(omega, modulus, mu);
                hi_val = lo_val + omega_factor;
                if hi_val >= modulus {
                    hi_val -= modulus;
                }
                if lo_val < omega_factor {
                    lo_val += modulus;
                }
                lo_val -= omega_factor;
                element[index_lo] = hi_val;
                element[index_hi] = lo_val;
            }
        }
        t >>= 1;
        logt1 -= 1;
    }
}


use num::bigint::BigUint;
use num::traits::Num;
use num::traits::One;
use num::traits::Zero;

fn forward_transform_to_bit_reverse<T: Num + Clone + PartialOrd>(element: &Vec<T>,
                                                                 root_of_unity_table: &Vec<T>,
                                                                 result: &mut Vec<T>) {
    let n = element.len();
    if result.len() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element[0].clone();
    let mu = modulus.clone().compute_mu();
    result.iter_mut().zip(element.iter()).for_each(|(r, e)| *r = e.clone());
    let mut t = n >> 1;
    let mut logt1 = get_msb64(t);
    for m in 1..n {
        for i in 0..m {
            let j1 = i << logt1;
            let j2 = j1 + t;
            let index_omega = m + i;
            let omega = root_of_unity_table[index_omega].clone();
            for index_lo in j1..j2 {
                let index_hi = index_lo + t;
                let lo_val = result[index_lo].clone();
                let omega_factor = result[index_hi].clone();
                if omega_factor != T::zero() {
                    let omega_factor = omega_factor.mod_mul_fast(omega.clone(), modulus.clone(), mu.clone());
                    let hi_val = lo_val.clone() + omega_factor.clone();
                    let modulus_clone = modulus.clone();
                    if hi_val >= modulus_clone {
                        hi_val - modulus_clone;
                    }
                    let modulus_clone = modulus.clone();
                    let lo_val = lo_val.clone();
                    if lo_val < omega_factor {
                        lo_val + modulus_clone;
                    }
                    let lo_val = lo_val.clone() - omega_factor;
                    result[index_lo] = hi_val;
                    result[index_hi] = lo_val;
                } else {
                    result[index_hi] = lo_val;
                }
            }
        }
        t >>= 1;
        logt1 -= 1;
    }
}

fn forward_transform_to_bit_reverse_in_place<T: Num + Clone + PartialOrd>(root_of_unity_table: &Vec<T>,
                                                                          precon_root_of_unity_table: &Vec<T>,
                                                                          element: &mut Vec<T>) {
    let n = element.len();
    let modulus = element[0].clone();
    let mut t = n >> 1;
    let mut logt1 = get_msb64(t);
    for m in 1..n {
        let mut j1;
        let mut j2;
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            let index_omega = m + i;
            let omega = root_of_unity_table[index_omega].clone();
            let precon_omega = precon_root_of_unity_table[index_omega].clone();
            for index_lo in j1..j2 {
                let index_hi = index_lo + t;
                let lo_val = element[index_lo].clone();
                let omega_factor = element[index_hi].clone();
                let omega_factor = omega_factor.mod_mul_fast_const(omega.clone(), modulus.clone(), precon_omega.clone());
                let hi_val = lo_val.clone() + omega_factor.clone();
                let modulus_clone = modulus.clone();
                if hi_val >= modulus_clone {
                    hi_val - modulus_clone;
                }
                let modulus_clone = modulus.clone();
                let lo_val = lo_val.clone();
                if lo_val < omega_factor {
                    lo_val + modulus_clone;
                }
                let lo_val = lo_val.clone() - omega_factor;
                element[index_lo] = hi_val;
                element[index_hi] = lo_val;
            }
        }
        t >>= 1;
        logt1 -= 1;
    }
}

fn inverse_transform_from_bit_reverse_in_place<T: Num + Clone + PartialOrd>(root_of_unity_inverse_table: &Vec<T>,
                                                                            cyclo_order_inv: &T,
                                                                            element: &mut Vec<T>) {
    let n = element.len();
    let modulus = element[0].clone();
    let mu = modulus.clone().compute_mu();
    let mut lo_val;
    let mut hi_val;
    let mut omega;
    let mut omega_factor;
    let mut i;
    let mut m;
    let mut j1;
    let mut j2;
    let mut index_omega;
    let mut index_lo;
    let mut index_hi;
    let mut t = 1;
    let mut logt1 = 1;
    for m in (n >> 1)..=1 {
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            index_omega = m + i;
            omega = root_of_unity_inverse_table[index_omega].clone();
            for index_lo in j1..j2 {
                index_hi = index_lo + t;
                hi_val = element[index_hi].clone();
                lo_val = element[index_lo].clone();
                omega_factor = lo_val.clone();
                let modulus_clone = modulus.clone();
                if omega_factor < hi_val {
                    omega_factor + modulus_clone;
                }
                let modulus_clone = modulus.clone();
                let hi_val = hi_val.clone();
                omega_factor -= hi_val;
                lo_val += hi_val;
                if lo_val >= modulus_clone {
                    lo_val - modulus_clone;
                }
                omega_factor = omega_factor.mod_mul_fast(omega.clone(), modulus.clone(), mu.clone());
                element[index_lo] = lo_val;
                element[index_hi] = omega_factor;
            }
        }
        t <<= 1;
        logt1 += 1;
    }
    for i in 0..n {
        element[i] = element[i].mod_mul_fast(cyclo_order_inv.clone(), modulus.clone(), mu.clone());
    }
}

fn inverse_transform_from_bit_reverse<T: Num + Clone + PartialOrd>(element: &Vec<T>,
                                                                  root_of_unity_inverse_table: &Vec<T>,
                                                                  cyclo_order_inv: &T,
                                                                  result: &mut Vec<T>) {
    let n = element.len();
    if result.len() != n {
        panic!("size of input element and size of output element not of same size");
    }
    result.iter_mut().zip(element.iter()).for_each(|(r, e)| *r = e.clone());
    inverse_transform_from_bit_reverse_in_place(root_of_unity_inverse_table, cyclo_order_inv, result);
}

use std::collections::HashMap;

pub struct NumberTheoreticTransformNtl<VecType> {
    // implementation details
}

impl<VecType> NumberTheoreticTransformNtl<VecType> {
    pub fn inverse_transform_from_bit_reverse_in_place(
        &self,
        root_of_unity_inverse_table: &VecType,
        precon_root_of_unity_inverse_table: &VecType,
        cyclo_order_inv: &IntType,
        precon_cyclo_order_inv: &IntType,
        element: &mut VecType,
    ) {
        let n = element.get_length();
        let modulus = element.get_modulus();
        let mut lo_val;
        let mut hi_val;
        let mut omega;
        let mut omega_factor;
        let mut precon_omega;
        let mut i;
        let mut m;
        let mut j1;
        let mut j2;
        let mut index_omega;
        let mut index_lo;
        let mut index_hi;
        let mut t = 1;
        let mut logt1 = 1;
        m = n >> 1;
        while m >= 1 {
            i = 0;
            while i < m {
                j1 = i << logt1;
                j2 = j1 + t;
                index_omega = m + i;
                omega = root_of_unity_inverse_table[index_omega];
                precon_omega = precon_root_of_unity_inverse_table[index_omega];
                index_lo = j1;
                while index_lo < j2 {
                    index_hi = index_lo + t;
                    hi_val = element[index_hi];
                    lo_val = element[index_lo];
                    omega_factor = lo_val;
                    if omega_factor < hi_val {
                        omega_factor += modulus;
                    }
                    omega_factor -= hi_val;
                    lo_val += hi_val;
                    if lo_val >= modulus {
                        lo_val -= modulus;
                    }
                    omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
                    element[index_lo] = lo_val;
                    element[index_hi] = omega_factor;
                    index_lo += 1;
                }
                i += 1;
            }
            t <<= 1;
            logt1 += 1;
            m >>= 1;
        }
        i = 0;
        while i < n {
            element[i].mod_mul_fast_const_eq(cyclo_order_inv, modulus, precon_cyclo_order_inv);
            i += 1;
        }
    }

    pub fn inverse_transform_from_bit_reverse(
        &self,
        element: &VecType,
        root_of_unity_inverse_table: &VecType,
        precon_root_of_unity_inverse_table: &VecType,
        cyclo_order_inv: &IntType,
        precon_cyclo_order_inv: &IntType,
        result: &mut VecType,
    ) {
        let n = element.get_length();
        if result.get_length() != n {
            panic!("size of input element and size of output element not of same size");
        }
        result.set_modulus(element.get_modulus());
        let mut i = 0;
        while i < n {
            result[i] = element[i];
            i += 1;
        }
        self.inverse_transform_from_bit_reverse_in_place(
            root_of_unity_inverse_table,
            precon_root_of_unity_inverse_table,
            cyclo_order_inv,
            precon_cyclo_order_inv,
            result,
        );
    }
}

pub struct ChineseRemainderTransformFTTNtl<VecType> {
    // implementation details
}

impl<VecType> ChineseRemainderTransformFTTNtl<VecType> {
    pub fn forward_transform_to_bit_reverse_in_place(
        &self,
        root_of_unity: &IntType,
        cyclo_order: usint,
        element: &mut VecType,
    ) {
        if *root_of_unity == IntType(1) || *root_of_unity == IntType(0) {
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if element.get_length() != cyclo_order_hf {
            panic!("element size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformNtl<VecType>().forward_transform_to_bit_reverse_in_place(
            m_root_of_unity_reverse_table_by_modulus[modulus],
            element,
        );
    }

    pub fn forward_transform_to_bit_reverse(
        &self,
        element: &VecType,
        root_of_unity: &IntType,
        cyclo_order: usint,
        result: &mut VecType,
    ) {
        if *root_of_unity == IntType(1) || *root_of_unity == IntType(0) {
            *result = element;
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if result.get_length() != cyclo_order_hf {
            panic!("result size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformNtl<VecType>().forward_transform_to_bit_reverse(
            element,
            m_root_of_unity_reverse_table_by_modulus[modulus],
            result,
        );
    }
}

fn is_power_of_two(n: usint) -> bool {
    n != 0 && (n & (n - 1)) == 0
}




