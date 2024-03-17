


use std::collections::HashMap;

struct ChineseRemainderTransformFTTDyn<T> {
    m_cycloOrderInverseTableByModulus: HashMap<T::Integer, T>,
    m_cycloOrderInversePreconTableByModulus: HashMap<T::Integer, T>,
    m_rootOfUnityReverseTableByModulus: HashMap<T::Integer, T>,
    m_rootOfUnityInverseReverseTableByModulus: HashMap<T::Integer, T>,
    m_rootOfUnityPreconReverseTableByModulus: HashMap<T::Integer, T>,
    m_rootOfUnityInversePreconReverseTableByModulus: HashMap<T::Integer, T>,
}

struct ChineseRemainderTransformArbDyn<T> {
    m_cyclotomicPolyMap: HashMap<T::Integer, T>,
    m_cyclotomicPolyReverseNTTMap: HashMap<T::Integer, T>,
    m_cyclotomicPolyNTTMap: HashMap<T::Integer, T>,
    m_rootOfUnityDivisionTableByModulus: HashMap<T::Integer, T>,
    m_rootOfUnityDivisionInverseTableByModulus: HashMap<T::Integer, T>,
    m_DivisionNTTModulus: HashMap<T::Integer, T::Integer>,
    m_DivisionNTTRootOfUnity: HashMap<T::Integer, T::Integer>,
}

struct ModulusRoot<T> {
    // Assuming ModulusRoot is a struct or similar in the original context
}

struct ModulusRootPair<T> {
    // Assuming ModulusRootPair is a struct or similar in the original context
}

struct BluesteinFFTDyn<T> {
    m_rootOfUnityTableByModulusRoot: HashMap<ModulusRoot<T::Integer>, T>,
    m_rootOfUnityInverseTableByModulusRoot: HashMap<ModulusRoot<T::Integer>, T>,
    m_powersTableByModulusRoot: HashMap<ModulusRoot<T::Integer>, T>,
    m_RBTableByModulusRootPair: HashMap<ModulusRootPair<T::Integer>, T>,
    m_defaultNTTModulusRoot: HashMap<T::Integer, ModulusRoot<T::Integer>>,
}

struct ChineseRemainderTransformArbDyn<T> {
    m_nttDivisionDim: HashMap<u32, u32>, // Assuming usint is equivalent to u32 in Rust
}

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::vec::Vec;

fn forward_transform_iterative<VecType>(
    element: &VecType,
    root_of_unity_table: &VecType,
    result: &mut VecType,
) where
    VecType: AsRef<[BigUint]> + AsMut<[BigUint]> + Clone,
{
    let n = element.as_ref().len();
    if result.as_ref().len() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.as_ref()[0].clone(); // Assuming GetModulus is similar to accessing the first element
    let mu = compute_mu(&modulus); // Assuming ComputeMu is a function that computes mu based on modulus
    // Assuming SetModulus sets the modulus for all elements, which might not be needed in Rust
    let msb = get_msb64(n as u64 - 1);
    for i in 0..n {
        result.as_mut()[i] = element.as_ref()[reverse_bits(i as u64, msb) as usize].clone();
    }
    let mut omega;
    let mut omega_factor;
    let mut odd_val;
    let mut even_val;
    let mut logm;
    let mut i;
    let mut j;
    let mut index_even;
    let mut index_odd;
    let logn = get_msb64(n as u64 - 1);
    for logm in 1..=logn {
        let indexes: Vec<usize> = (0..1 << (logm - 1)).map(|i| i << (logn - logm)).collect();
        for j in (0..n).step_by(1 << logm) {
            for i in 0..1 << (logm - 1) {
                omega = root_of_unity_table.as_ref()[indexes[i]].clone();
                index_even = j + i;
                index_odd = index_even + (1 << (logm - 1));
                odd_val = result.as_ref()[index_odd].clone();
                omega_factor = mod_mul(&omega, &odd_val, &modulus, &mu);
                even_val = result.as_ref()[index_even].clone();
                odd_val = even_val.clone();
                odd_val += omega_factor.clone();
                if odd_val >= modulus {
                    odd_val -= &modulus;
                }
                if even_val < omega_factor {
                    even_val += &modulus;
                }
                even_val -= omega_factor;
                result.as_mut()[index_even] = odd_val;
                result.as_mut()[index_odd] = even_val;
            }
        }
    }
}

fn inverse_transform_iterative<VecType>(
    element: &VecType,
    root_of_unity_inverse_table: &VecType,
    result: &mut VecType,
) where
    VecType: AsRef<[BigUint]> + AsMut<[BigUint]> + Clone,
{
    let n = element.as_ref().len();
    let modulus = element.as_ref()[0].clone(); // Assuming GetModulus is similar to accessing the first element
    let mu = compute_mu(&modulus); // Assuming ComputeMu is a function that computes mu based on modulus
    forward_transform_iterative(element, root_of_unity_inverse_table, result);
    let cyclo_order_inv = mod_inverse(&BigUint::from(n as u64), &modulus).unwrap();
    for i in 0..n {
        result.as_mut()[i] = mod_mul_eq(&result.as_ref()[i], &cyclo_order_inv, &modulus, &mu);
    }
}

fn forward_transform_to_bit_reverse_in_place<VecType>(
    root_of_unity_table: &VecType,
    element: &mut VecType,
) where
    VecType: AsRef<[BigUint]> + AsMut<[BigUint]> + Clone,
{
    let n = element.as_ref().len();
    let modulus = element.as_ref()[0].clone(); // Assuming GetModulus is similar to accessing the first element
    let mu = compute_mu(&modulus); // Assuming ComputeMu is a function that computes mu based on modulus
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
    let zero = BigUint::zero();
    let mut t = n >> 1;
    let mut logt1 = get_msb64(t as u64);
    for m in (1..n).step_by(m * 2) {
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            index_omega = m + i;
            omega = root_of_unity_table.as_ref()[index_omega].clone();
            for index_lo in (j1..j2).step_by(1) {
                index_hi = index_lo + t;
                lo_val = element.as_ref()[index_lo].clone();
                omega_factor = element.as_ref()[index_hi].clone();
                omega_factor = mod_mul_fast_eq(&omega_factor, &omega, &modulus, &mu);
                hi_val = lo_val.clone() + omega_factor.clone();
                if hi_val >= modulus {
                    hi_val -= &modulus;
                }
                if lo_val < omega_factor {
                    lo_val += &modulus;
                }
                lo_val -= omega_factor;
                element.as_mut()[index_lo] = hi_val;
                element.as_mut()[index_hi] = lo_val;
            }
        }
        t >>= 1;
        logt1 -= 1;
    }
}

// Placeholder functions for `compute_mu`, `mod_mul`, `mod_mul_eq`, `mod_mul_fast_eq`, `mod_inverse`, `get_msb64`, and `reverse_bits`
// These functions would need to be implemented based on the mathematical operations they are supposed to perform.


use std::collections::HashMap;

pub struct NumberTheoreticTransformDyn<VecType> {
    // implementation details
}

impl<VecType> NumberTheoreticTransformDyn<VecType> {
    pub fn inverse_transform_from_bit_reverse_in_place(
        &self,
        root_of_unity_inverse_table: &VecType,
        cyclo_order_inv: &IntType,
        element: &mut VecType,
    ) {
        let n = element.get_length();
        let modulus = element.get_modulus();
        let mu = modulus.compute_mu();
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
                omega = root_of_unity_inverse_table[index_omega];
                for index_lo in j1..j2 {
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
                    omega_factor.mod_mul_fast_eq(omega, modulus, mu);
                    element[index_lo] = lo_val;
                    element[index_hi] = omega_factor;
                }
            }
            t <<= 1;
            logt1 += 1;
        }
        for i in 0..n {
            element[i].mod_mul_fast_eq(cyclo_order_inv, modulus, mu);
        }
    }

    pub fn inverse_transform_from_bit_reverse(
        &self,
        element: &VecType,
        root_of_unity_inverse_table: &VecType,
        cyclo_order_inv: &IntType,
        result: &mut VecType,
    ) {
        let n = element.get_length();
        if result.get_length() != n {
            panic!("size of input element and size of output element not of same size");
        }
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            result[i] = element[i];
        }
        self.inverse_transform_from_bit_reverse_in_place(
            root_of_unity_inverse_table,
            cyclo_order_inv,
            result,
        );
    }

    pub fn inverse_transform_from_bit_reverse_in_place_with_precon(
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
        for m in (n >> 1)..=1 {
            for i in 0..m {
                j1 = i << logt1;
                j2 = j1 + t;
                index_omega = m + i;
                omega = root_of_unity_inverse_table[index_omega];
                precon_omega = precon_root_of_unity_inverse_table[index_omega];
                for index_lo in j1..j2 {
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
                }
            }
            t <<= 1;
            logt1 += 1;
        }
        for i in 0..n {
            element[i].mod_mul_fast_const_eq(cyclo_order_inv, modulus, precon_cyclo_order_inv);
        }
    }

    pub fn inverse_transform_from_bit_reverse_with_precon(
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
        for i in 0..n {
            result[i] = element[i];
        }
        self.inverse_transform_from_bit_reverse_in_place_with_precon(
            root_of_unity_inverse_table,
            precon_root_of_unity_inverse_table,
            cyclo_order_inv,
            precon_cyclo_order_inv,
            result,
        );
    }
}

pub struct ChineseRemainderTransformFTTDyn<VecType> {
    // implementation details
}

impl<VecType> ChineseRemainderTransformFTTDyn<VecType> {
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
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformDyn<VecType>().forward_transform_to_bit_reverse_in_place(
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
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformDyn<VecType>().forward_transform_to_bit_reverse(
            element,
            m_root_of_unity_reverse_table_by_modulus[modulus],
            result,
        );
    }
}

fn is_power_of_two(n: usint) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn pre_compute(root_of_unity: &IntType, cyclo_order: usint, modulus: &IntType) {
    // implementation details
}



use std::collections::HashMap;
use std::vec::Vec;
use num::traits::{One, Zero};
use num::Integer;
use num::bigint::BigUint;

struct ChineseRemainderTransformFTTDyn<VecType> {
    m_rootOfUnityReverseTableByModulus: HashMap<IntType, VecType>,
    m_rootOfUnityInverseReverseTableByModulus: HashMap<IntType, VecType>,
    m_cycloOrderInverseTableByModulus: HashMap<IntType, VecType>,
}

impl<VecType> ChineseRemainderTransformFTTDyn<VecType> {
    fn inverse_transform_from_bit_reverse_in_place(&self, root_of_unity: IntType, cyclo_order: usint, element: &mut VecType) {
        if root_of_unity == IntType::one() || root_of_unity == IntType::zero() {
            return;
        }
        if !is_power_of_two(cyclo_order) {
            OPENFHE_THROW(math_error, "CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if element.len() != cyclo_order_hf {
            OPENFHE_THROW(math_error, "element size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let msb = get_msb64(cyclo_order_hf - 1);
        NumberTheoreticTransformDyn<VecType>().inverse_transform_from_bit_reverse_in_place(
            self.m_rootOfUnityInverseReverseTableByModulus[&modulus],
            self.m_cycloOrderInverseTableByModulus[&modulus][msb],
            element,
        );
    }

    fn inverse_transform_from_bit_reverse(&self, element: &VecType, root_of_unity: IntType, cyclo_order: usint, result: &mut VecType) {
        if root_of_unity == IntType::one() || root_of_unity == IntType::zero() {
            *result = element.clone();
            return;
        }
        if !is_power_of_two(cyclo_order) {
            OPENFHE_THROW(math_error, "CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if result.len() != cyclo_order_hf {
            OPENFHE_THROW(math_error, "result size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let n = element.len();
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            result[i] = element[i];
        }
        let msb = get_msb64(cyclo_order_hf - 1);
        NumberTheoreticTransformDyn<VecType>().inverse_transform_from_bit_reverse_in_place(
            self.m_rootOfUnityInverseReverseTableByModulus[&modulus],
            self.m_cycloOrderInverseTableByModulus[&modulus][msb],
            result,
        );
    }

    fn pre_compute(&self, root_of_unity: IntType, cyclo_order: usint, modulus: IntType) {
        let cyclo_order_hf = cyclo_order >> 1;
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            #pragma omp critical
            {
                let x = IntType::one();
                let xinv = IntType::one();
                let msb = get_msb64(cyclo_order_hf - 1);
                let mu = modulus.compute_mu();
                let mut table = VecType::new(cyclo_order_hf, modulus);
                let mut table_i = VecType::new(cyclo_order_hf, modulus);
                let root_of_unity_inverse = root_of_unity.mod_inverse(modulus);
                let mut iinv;
                for i in 0..cyclo_order_hf {
                    iinv = reverse_bits(i, msb);
                    table[iinv] = x.clone();
                    table_i[iinv] = xinv.clone();
                    x.mod_mul_eq(&root_of_unity, &modulus, &mu);
                    xinv.mod_mul_eq(&root_of_unity_inverse, &modulus, &mu);
                }
                self.m_rootOfUnityReverseTableByModulus.insert(modulus.clone(), table);
                self.m_rootOfUnityInverseReverseTableByModulus.insert(modulus.clone(), table_i);
                let mut table_coi = VecType::new(msb + 1, modulus);
                for i in 0..msb + 1 {
                    let co_inv = IntType::from(1 << i).mod_inverse(&modulus);
                    table_coi[i] = co_inv;
                }
                self.m_cycloOrderInverseTableByModulus.insert(modulus.clone(), table_coi);
            }
        }
    }

    fn pre_compute_vec(&self, root_of_unity: Vec<IntType>, cyclo_order: usint, modulii_chain: Vec<IntType>) {
        let num_of_root_u = root_of_unity.len();
        let num_modulii = modulii_chain.len();
        if num_of_root_u != num_modulii {
            OPENFHE_THROW(math_error, "size of root of unity and size of moduli chain not of same size");
        }
        for i in 0..num_of_root_u {
            let current_root = root_of_unity[i].clone();
            let current_mod = modulii_chain[i].clone();
            self.pre_compute(current_root, cyclo_order, current_mod);
        }
    }

    fn reset(&self) {
        self.m_cycloOrderInverseTableByModulus.clear();
        self.m_cycloOrderInversePreconTableByModulus.clear();
        self.m_rootOfUnityReverseTableByModulus.clear();
        self.m_rootOfUnityInverseReverseTableByModulus.clear();
        self.m_rootOfUnityPreconReverseTableByModulus.clear();
        self.m_rootOfUnityInversePreconReverseTableByModulus.clear();
    }
}

struct BluesteinFFTDyn<VecType> {
    m_defaultNTTModulusRoot: HashMap<IntType, ModulusRoot<IntType>>,
    m_rootOfUnityTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_rootOfUnityInverseTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
}

impl<VecType> BluesteinFFTDyn<VecType> {
    fn pre_compute_default_ntt_modulus_root(&self, cyclo_order: usint, modulus: IntType) {
        let ntt_dim = pow(2, ceil(log2(2 * cyclo_order - 1)));
        let ntt_modulus = last_prime(log2(ntt_dim) + 2 * modulus.get_msb(), ntt_dim);
        let ntt_root = root_of_unity(ntt_dim, ntt_modulus);
        let ntt_modulus_root = ModulusRoot(ntt_modulus, ntt_root);
        self.m_defaultNTTModulusRoot.insert(modulus.clone(), ntt_modulus_root);
        self.pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root);
    }

    fn pre_compute_root_table_for_ntt(&self, cycloto_order: usint, ntt_modulus_root: ModulusRoot<IntType>) {
        let ntt_dim = pow(2, ceil(log2(2 * cycloto_order - 1)));
        let ntt_modulus = ntt_modulus_root.first;
        let ntt_root = ntt_modulus_root.second;
        let root = IntType::from(ntt_root);
        let root_inv = root.mod_inverse(ntt_modulus);
        let ntt_dim_hf = ntt_dim >> 1;
        let mut root_table = VecType::new(ntt_dim_hf, ntt_modulus);
        let mut root_table_inverse = VecType::new(ntt_dim_hf, ntt_modulus);
        let mut x = IntType::one();
        for i in 0..ntt_dim_hf {
            root_table[i] = x.clone();
            x = x.mod_mul(&root, &ntt_modulus);
        }
        x = IntType::one();
        for i in 0..ntt_dim_hf {
            root_table_inverse[i] = x.clone();
            x = x.mod_mul(&root_inv, &ntt_modulus);
        }
        self.m_rootOfUnityTableByModulusRoot.insert(ntt_modulus_root.clone(), root_table);
        self.m_rootOfUnityInverseTableByModulusRoot.insert(ntt_modulus_root.clone(), root_table_inverse);
    }
}

use std::collections::HashMap;
use num::traits::{Zero, One};
use num::pow;
use num::integer::Integer;

struct ModulusRoot<T> {
    modulus: T,
    root: T,
}

struct ModulusRootPair<T> {
    modulus_root: ModulusRoot<T>,
    ntt_modulus_root: ModulusRoot<T>,
}

struct BluesteinFFTDyn<T> {
    m_powers_table_by_modulus_root: HashMap<ModulusRoot<T>, Vec<T>>,
    m_rb_table_by_modulus_root_pair: HashMap<ModulusRootPair<T>, Vec<T>>,
    m_root_of_unity_table_by_modulus_root: HashMap<ModulusRoot<T>, Vec<T>>,
    m_root_of_unity_inverse_table_by_modulus_root: HashMap<ModulusRoot<T>, Vec<T>>,
    m_default_ntt_modulus_root: HashMap<T, ModulusRoot<T>>,
}

impl<T> BluesteinFFTDyn<T> {
    fn pre_compute_powers(&mut self, cyclo_order: usize, modulus_root: ModulusRoot<T>) {
        let modulus = modulus_root.modulus;
        let root = modulus_root.root;
        let mut powers = vec![T::zero(); cyclo_order];
        powers[0] = T::one();
        for i in 1..cyclo_order {
            let i_sqr = (i * i) % (T::from(2).unwrap() * cyclo_order);
            let val = root.mod_exp(&T::from(i_sqr).unwrap(), &modulus);
            powers[i] = val;
        }
        self.m_powers_table_by_modulus_root.insert(modulus_root, powers);
    }

    fn pre_compute_rb_table(&mut self, cyclo_order: usize, modulus_root_pair: ModulusRootPair<T>) {
        let modulus_root = modulus_root_pair.modulus_root;
        let modulus = modulus_root.modulus;
        let root = modulus_root.root;
        let root_inv = root.mod_inverse(&modulus);
        let ntt_modulus_root = modulus_root_pair.ntt_modulus_root;
        let ntt_modulus = ntt_modulus_root.modulus;

        let root_table = self.m_root_of_unity_table_by_modulus_root[&ntt_modulus_root].clone();
        let ntt_dim = pow(T::from(2).unwrap(), (2 * cyclo_order - 1).log2().ceil() as usize);
        let mut b = vec![T::zero(); 2 * cyclo_order - 1];
        b[cyclo_order - 1] = T::one();
        for i in 1..cyclo_order {
            let i_sqr = (i * i) % (T::from(2).unwrap() * cyclo_order);
            let val = root_inv.mod_exp(&T::from(i_sqr).unwrap(), &modulus);
            b[cyclo_order - 1 + i] = val;
            b[cyclo_order - 1 - i] = val;
        }
        let rb = pad_zeros(&b, ntt_dim);
        let mut RB = vec![T::zero(); ntt_dim];
        number_theoretic_transform_dyn::forward_transform_iterative(&rb, &root_table, &mut RB);
        self.m_rb_table_by_modulus_root_pair.insert(modulus_root_pair, RB);
    }

    fn forward_transform(&self, element: &Vec<T>, root: &T, cyclo_order: usize) -> Vec<T> {
        let modulus = element[0].clone();
        let ntt_modulus_root = self.m_default_ntt_modulus_root[&modulus].clone();
        self.forward_transform_with_ntt_modulus(element, root, cyclo_order, &ntt_modulus_root)
    }

    fn forward_transform_with_ntt_modulus(&self, element: &Vec<T>, root: &T, cyclo_order: usize, ntt_modulus_root: &ModulusRoot<T>) -> Vec<T> {
        if element.len() != cyclo_order {
            panic!("expected size of element vector should be equal to cyclotomic order");
        }
        let modulus = element[0].clone();
        let modulus_root = ModulusRoot { modulus: modulus.clone(), root: root.clone() };
        let powers = self.m_powers_table_by_modulus_root[&modulus_root].clone();
        let ntt_modulus = ntt_modulus_root.modulus.clone();

        let root_table = self.m_root_of_unity_table_by_modulus_root[ntt_modulus_root].clone();
        let root_table_inverse = self.m_root_of_unity_inverse_table_by_modulus_root[ntt_modulus_root].clone();
        let x = element.iter().zip(powers.iter()).map(|(a, b)| a * b).collect();
        let ntt_dim = pow(T::from(2).unwrap(), (2 * cyclo_order - 1).log2().ceil() as usize);
        let ra = pad_zeros(&x, ntt_dim);
        let mut RA = vec![T::zero(); ntt_dim];
        number_theoretic_transform_dyn::forward_transform_iterative(&ra, &root_table, &mut RA);
        let modulus_root_pair = ModulusRootPair { modulus_root: modulus_root.clone(), ntt_modulus_root: ntt_modulus_root.clone() };
        let RB = self.m_rb_table_by_modulus_root_pair[&modulus_root_pair].clone();
        let mut RC = vec![T::zero(); ntt_dim];
        number_theoretic_transform_dyn::inverse_transform_iterative(&RA.iter().zip(RB.iter()).map(|(a, b)| a * b).collect(), &root_table_inverse, &mut RC);
        let resize_rc = resize(&RC, cyclo_order - 1, 2 * (cyclo_order - 1));
        let mut result = resize_rc.iter().zip(powers.iter()).map(|(a, b)| a * b).collect();
        result
    }

    fn pad_zeros(a: &Vec<T>, final_size: usize) -> Vec<T> {
        let s = a.len();
        let mut result = vec![T::zero(); final_size];
        for i in 0..s {
            result[i] = a[i].clone();
        }
        result
    }

    fn resize(a: &Vec<T>, lo: usize, hi: usize) -> Vec<T> {
        let mut result = vec![T::zero(); hi - lo + 1];
        for i in lo..=hi {
            result[i - lo] = a[i].clone();
        }
        result
    }

    fn reset(&mut self) {
        self.m_root_of_unity_table_by_modulus_root.clear();
        self.m_root_of_unity_inverse_table_by_modulus_root.clear();
        self.m_powers_table_by_modulus_root.clear();
        self.m_rb_table_by_modulus_root_pair.clear();
        self.m_default_ntt_modulus_root.clear();
    }
}

struct ChineseRemainderTransformArbDyn<T> {
    m_cyclotomic_poly_map: HashMap<T, Vec<T>>,
    m_ntt_division_dim: HashMap<usize, usize>,
    m_division_ntt_modulus: HashMap<T, T>,
    m_division_ntt_root_of_unity: HashMap<T, T>,
    m_root_of_unity_division_table_by_modulus: HashMap<T, Vec<T>>,
    m_root_of_unity_division_inverse_table_by_modulus: HashMap<T, Vec<T>>,
    m_cyclotomic_poly_reverse_ntt_map: HashMap<T, Vec<T>>,
    m_cyclotomic_poly_ntt_map: HashMap<T, Vec<T>>,
}

impl<T> ChineseRemainderTransformArbDyn<T> {
    fn set_cyclotomic_polynomial(&mut self, poly: Vec<T>, modulus: T) {
        self.m_cyclotomic_poly_map.insert(modulus, poly);
    }

    fn pre_compute(&mut self, cycloto_order: usize, modulus: T) {
        BluesteinFFTDyn<T>().pre_compute_default_ntt_modulus_root(cycloto_order, modulus);
    }

    fn set_pre_computed_ntt_modulus(&mut self, cycloto_order: usize, modulus: T, ntt_modulus: T, ntt_root: T) {
        let ntt_modulus_root = ModulusRoot { modulus: ntt_modulus, root: ntt_root };
        BluesteinFFTDyn<T>().pre_compute_root_table_for_ntt(cycloto_order, ntt_modulus_root);
    }

    fn set_pre_computed_ntt_division_modulus(&mut self, cycloto_order: usize, modulus: T, ntt_mod: T, ntt_root_big: T) {
        let n = get_totient(cycloto_order);
        let power = cycloto_order - n;
        self.m_ntt_division_dim.insert(cycloto_order, 2 * pow(T::from(2).unwrap(), (power as f64).log2().ceil() as usize));
        let ntt_dim_big = pow(T::from(2).unwrap(), (2 * cycloto_order - 1).log2().ceil() as usize);

        let ntt_root = ntt_root_big.mod_exp(&T::from(ntt_dim_big / self.m_ntt_division_dim[&cycloto_order]).unwrap(), &ntt_mod);
        self.m_division_ntt_modulus.insert(modulus.clone(), ntt_mod);
        self.m_division_ntt_root_of_unity.insert(modulus.clone(), ntt_root);

        let ntt_dim = self.m_ntt_division_dim[&cycloto_order];
        let root = ntt_root.clone();
        let root_inv = root.mod_inverse(&ntt_mod);
        let ntt_dim_hf = ntt_dim >> 1;
        let mut root_table = vec![T::zero(); ntt_dim_hf];
        let mut root_table_inverse = vec![T::zero(); ntt_dim_hf];
        let mut x = T::one();
        for i in 0..ntt_dim_hf {
            root_table[i] = x.clone();
            x = x.mod_mul(&root, &ntt_mod);
        }
        x = T::one();
        for i in 0..ntt_dim_hf {
            root_table_inverse[i] = x.clone();
            x = x.mod_mul(&root_inv, &ntt_mod);
        }
        self.m_root_of_unity_division_table_by_modulus.insert(ntt_mod.clone(), root_table);
        self.m_root_of_unity_division_inverse_table_by_modulus.insert(ntt_mod.clone(), root_table_inverse);

        let rev_cpm = inverse_poly_mod(&self.m_cyclotomic_poly_map[&modulus], &modulus, power);
        let rev_cpm_padded = BluesteinFFTDyn<T>::pad_zeros(&rev_cpm, ntt_dim);
        rev_cpm_padded.set_modulus(ntt_mod);

        let mut RA = vec![T::zero(); ntt_dim];
        number_theoretic_transform_dyn::forward_transform_iterative(&rev_cpm_padded, &root_table, &mut RA);
        self.m_cyclotomic_poly_reverse_ntt_map.insert(modulus.clone(), RA);
        let cyclo_poly = &self.m_cyclotomic_poly_map[&modulus];
        let mut Q_forward_transform = vec![T::zero(); ntt_dim];
        for i in 0..cyclo_poly.len() {
            Q_forward_transform[i] = cyclo_poly[i].clone();
        }
        let mut Q_fwd_result = vec![T::zero(); ntt_dim];
        number_theoretic_transform_dyn::forward_transform_iterative(&Q_forward_transform, &root_table, &mut Q_fwd_result);
        self.m_cyclotomic_poly_ntt_map.insert(modulus.clone(), Q_fwd_result);
    }
}

fn pad_zeros<T: Clone + Zero>(a: &Vec<T>, final_size: usize) -> Vec<T> {
    let s = a.len();
    let mut result = vec![T::zero(); final_size];
    for i in 0..s {
        result[i] = a[i].clone();
    }
    for i in s..final_size {
        result[i] = T::zero();
    }
    result
}

fn resize<T: Clone>(a: &Vec<T>, lo: usize, hi: usize) -> Vec<T> {
    let mut result = vec![T::zero(); hi - lo + 1];
    for i in lo..=hi {
        result[i - lo] = a[i].clone();
    }
    result
}

fn inverse_poly_mod<T: Clone + Zero + One>(poly: &Vec<T>, modulus: &T, power: usize) -> Vec<T> {
    let mut result = vec![T::zero(); power];
    let mut inv_mod = modulus.clone().mod_inverse(&T::from(power).unwrap());
    for i in 0..power {
        result[i] = poly[i].clone().mod_mul(&inv_mod, modulus);
        inv_mod = inv_mod.mod_mul(&modulus, modulus);
    }
    result
}

fn get_totient(n: usize) -> usize {
    let mut result = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Rem};

fn ceil_log2(x: usize) -> usize {
    let mut result = 0;
    let mut n = x;
    while n > 1 {
        n >>= 1;
        result += 1;
    }
    result
}

fn mod_sub<T>(a: T, b: T, modulus: T, mu: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy,
{
    let diff = a - b;
    if diff >= T::zero() {
        diff
    } else {
        modulus + diff
    }
}

fn poly_mod<T>(a: &[T], b: &[T], modulus: T) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy,
{
    let mut result = vec![T::zero(); b.len()];
    for i in 0..b.len() {
        if i < a.len() {
            result[i] = mod_sub(a[i], b[i], modulus, modulus);
        } else {
            result[i] = mod_sub(T::zero(), b[i], modulus, modulus);
        }
    }
    result
}

fn polynomial_multiplication<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    let mut result = vec![T::zero(); a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i + j] = result[i + j] + a[i] * b[j];
        }
    }
    result
}

fn forward_transform<T>(element: &[T], root: T, ntt_modulus: T, ntt_root: T, cyclo_order: usize) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    let phim = get_totient(cyclo_order);
    if element.len() != phim {
        panic!("element size should be equal to phim");
    }
    let modulus = element[0];
    let modulus_root = (modulus, root);
    let ntt_modulus_root = (ntt_modulus, ntt_root);
    let modulus_root_pair = (modulus_root, ntt_modulus_root);

    if m_root_of_unity_table_by_modulus_root.get(&ntt_modulus_root).map_or(true, |v| v.is_empty()) {
        pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root);
    }
    if m_powers_table_by_modulus_root.get(&modulus_root).map_or(true, |v| v.is_empty()) {
        pre_compute_powers(cyclo_order, modulus_root);
    }
    if m_rb_table_by_modulus_root_pair.get(&modulus_root_pair).map_or(true, |v| v.is_empty()) {
        pre_compute_rb_table(cyclo_order, modulus_root_pair);
    }

    let input_to_bluestein = pad(element, cyclo_order, true);
    let output_bluestein = forward_transform(input_to_bluestein, root, cyclo_order, ntt_modulus_root);
    let output = drop(output_bluestein, cyclo_order, true, ntt_modulus, ntt_root);
    output
}

fn inverse_transform<T>(element: &[T], root: T, ntt_modulus: T, ntt_root: T, cyclo_order: usize) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    let phim = get_totient(cyclo_order);
    if element.len() != phim {
        panic!("element size should be equal to phim");
    }
    let modulus = element[0];
    let root_inverse = root.mod_inverse(modulus);
    let modulus_root_inverse = (modulus, root_inverse);
    let ntt_modulus_root = (ntt_modulus, ntt_root);
    let modulus_root_pair = (modulus_root_inverse, ntt_modulus_root);

    if m_root_of_unity_table_by_modulus_root.get(&ntt_modulus_root).map_or(true, |v| v.is_empty()) {
        pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root);
    }
    if m_powers_table_by_modulus_root.get(&modulus_root_inverse).map_or(true, |v| v.is_empty()) {
        pre_compute_powers(cyclo_order, modulus_root_inverse);
    }
    if m_rb_table_by_modulus_root_pair.get(&modulus_root_pair).map_or(true, |v| v.is_empty()) {
        pre_compute_rb_table(cyclo_order, modulus_root_pair);
    }

    let input_to_bluestein = pad(element, cyclo_order, false);
    let output_bluestein = forward_transform(input_to_bluestein, root_inverse, cyclo_order, ntt_modulus_root);
    let cyclotomic_inverse = cyclo_order.mod_inverse(modulus);
    let mut output = drop(output_bluestein, cyclo_order, false, ntt_modulus, ntt_root);
    for i in 0..output.len() {
        output[i] = output[i] * cyclotomic_inverse;
    }
    output
}

fn pad<T>(element: &[T], cyclo_order: usize, forward: bool) -> Vec<T>
where
    T: Copy,
{
    let n = get_totient(cyclo_order);
    let modulus = element[0];
    let mut input_to_bluestein = vec![T::zero(); cyclo_order];
    if forward {
        for i in 0..n {
            input_to_bluestein[i] = element[i];
        }
    } else {
        let t_list = get_totient_list(cyclo_order);
        let mut i = 0;
        for &coprime in &t_list {
            input_to_bluestein[coprime] = element[i];
            i += 1;
        }
    }
    input_to_bluestein
}

fn drop<T>(element: &[T], cyclo_order: usize, forward: bool, big_mod: T, big_root: T) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    let n = get_totient(cyclo_order);
    let modulus = element[0];
    let mut output = vec![T::zero(); n];
    if forward {
        let t_list = get_totient_list(cyclo_order);
        for i in 0..n {
            output[i] = element[t_list[i]];
        }
    } else {
        if (n + 1) == cyclo_order {
            let mu = modulus.compute_mu();
            let coeff_n = element[n];
            for i in 0..n {
                output[i] = mod_sub(element[i], coeff_n, modulus, mu);
            }
        } else if (n + 1) * 2 == cyclo_order {
            let mu = modulus.compute_mu();
            for i in 0..n {
                let coeff_i = element[i];
                let coeff_ip = element[i + n + 1];
                output[i] = mod_sub(coeff_i, coeff_ip, modulus, mu);
            }
            let coeff_n = mod_sub(element[n], element[2 * n + 1], modulus, mu);
            for i in 0..n {
                if i % 2 == 0 {
                    output[i] = mod_sub(output[i], coeff_n, modulus, mu);
                } else {
                    output[i] = output[i] + coeff_n;
                }
            }
        } else {
            if m_root_of_unity_division_table_by_modulus.get(&big_mod).map_or(true, |v| v.is_empty())
                || m_division_ntt_modulus.get(&modulus) != Some(big_mod)
            {
                set_pre_computed_ntt_division_modulus(cyclo_order, modulus, big_mod, big_root);
            }
            let ntt_mod = m_division_ntt_modulus[&modulus];
            let root_table = &m_root_of_unity_division_table_by_modulus[&ntt_mod];
            let mut a_padded2 = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            let power = cyclo_order - n;
            for i in n..element.len() {
                a_padded2[power - (i - n) - 1] = element[i];
            }
            let mut a = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            forward_transform_iterative(&a_padded2, root_table, &mut a);
            let ab = polynomial_multiplication(&a, &m_cyclotomic_poly_reverse_ntt_map[&modulus]);
            let root_table_inverse = &m_root_of_unity_division_inverse_table_by_modulus[&ntt_mod];
            let mut quotient = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            for i in 0..power {
                quotient[i] = ab[i];
            }
            quotient.mod_eq(&modulus);
            quotient.set_modulus(ntt_mod);
            let mut new_quotient = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            forward_transform_iterative(&quotient, root_table, &mut new_quotient);
            new_quotient = new_quotient * m_cyclotomic_poly_ntt_map[&modulus];
            let mut new_quotient2 = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            inverse_transform_iterative(&new_quotient, root_table_inverse, &mut new_quotient2);
            new_quotient2.set_modulus(modulus);
            new_quotient2.mod_eq(&modulus);
            let mu = modulus.compute_mu();
            for i in 0..n {
                output[i] = mod_sub(element[i], new_quotient2[cyclo_order - 1 - i], modulus, mu);
            }
        }
    }
    output
}

fn reset() {
    m_cyclotomic_poly_map.clear();
    m_cyclotomic_poly_reverse_ntt_map.clear();
    m_cyclotomic_poly_ntt_map.clear();
    m_root_of_unity_division_table_by_modulus.clear();
    m_root_of_unity_division_inverse_table_by_modulus.clear();
    m_division_ntt_modulus.clear();
    m_division_ntt_root_of_unity.clear();
    m_ntt_division_dim.clear();
    reset_bluestein();
}




