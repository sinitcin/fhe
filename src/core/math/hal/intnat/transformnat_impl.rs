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
    let msb = get_msb(n - 1);
    for i in 0..n {
        result[i] = element[reverse_bits(i, msb)];
    }
    let mut omega;
    let mut omega_factor;
    let mut odd_val;
    let mut even_val;
    let logn = get_msb(n - 1);
    for logm in 1..=logn {
        let mut indexes = Vec::with_capacity(1 << (logm - 1));
        for i in 0..(1 << (logm - 1)) {
            indexes.push(i << (logn - logm));
        }
        for j in (0..n).step_by(1 << logm) {
            for i in 0..(1 << (logm - 1)) {
                omega = root_of_unity_table[indexes[i]];
                let index_even = j + i;
                let index_odd = index_even + (1 << (logm - 1));
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
    let mut t = n >> 1;
    let mut logt1 = get_msb(t);
    for m in 1..n {
        for i in 0..m {
            let j1 = i << logt1;
            let j2 = j1 + t;
            let index_omega = m + i;
            let omega = root_of_unity_table[index_omega];
            for index_lo in j1..j2 {
                let index_hi = index_lo + t;
                let lo_val = element[index_lo];
                let omega_factor = element[index_hi];
                omega_factor.mod_mul_fast_eq(omega, modulus, mu);
                let hi_val = lo_val + omega_factor;
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

fn forward_transform_to_bit_reverse(element: &VecType, root_of_unity_table: &VecType, result: &mut VecType) {
    let n = element.len();
    if result.len() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    result.set_modulus(modulus);
    for i in 0..n {
        result[i] = element[i];
    }
    let mut t = n >> 1;
    let mut logt1 = get_msb(t);
    for m in 1..n {
        for i in 0..m {
            let j1 = i << logt1;
            let j2 = j1 + t;
            let index_omega = m + i;
            let omega = root_of_unity_table[index_omega];
            for index_lo in j1..j2 {
                let index_hi = index_lo + t;
                let lo_val = result[index_lo];
                let omega_factor = result[index_hi];
                if omega_factor != 0 {
                    omega_factor.mod_mul_fast_eq(omega, modulus, mu);
                    let hi_val = lo_val + omega_factor;
                    if hi_val >= modulus {
                        hi_val -= modulus;
                    }
                    if lo_val < omega_factor {
                        lo_val += modulus;
                    }
                    lo_val -= omega_factor;
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


use num_traits::identities::Zero;
use num_traits::One;
use num_traits::Pow;
use num_traits::ToPrimitive;
use num_traits::Zero;
use num_traits::One;
use num_traits::Pow;
use num_traits::ToPrimitive;

fn forward_transform_to_bit_reverse_in_place<T>(root_of_unity_table: &Vec<T>,
                                                precon_root_of_unity_table: &Vec<T>,
                                                element: &mut Vec<T>) {
    let modulus = element.get_modulus();
    let n = element.get_length() >> 1;
    let mut t = n;
    let mut logt = get_msb(t);
    let mut m = 1;
    while m < n {
        for i in 0..m {
            let omega = root_of_unity_table[i + m];
            let precon_omega = precon_root_of_unity_table[i + m];
            let j1 = i << logt;
            let j2 = j1 + t;
            for j1 in j1..j2 {
                let omega_factor = element[j1 + t];
                omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
                let lo_val = element[j1 + 0];
                let hi_val = lo_val + omega_factor;
                if hi_val >= modulus {
                    hi_val -= modulus;
                }
                if lo_val < omega_factor {
                    lo_val += modulus;
                }
                lo_val -= omega_factor;
                element[j1 + 0] = hi_val;
                element[j1 + t] = lo_val;
            }
        }
        m <<= 1;
        t >>= 1;
        logt -= 1;
    }
    for i in (0..(n << 1)).step_by(2) {
        let omega_factor = element[i + 1];
        let omega = root_of_unity_table[(i >> 1) + n];
        let precon_omega = precon_root_of_unity_table[(i >> 1) + n];
        omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
        let lo_val = element[i + 0];
        let hi_val = lo_val + omega_factor;
        if hi_val >= modulus {
            hi_val -= modulus;
        }
        if lo_val < omega_factor {
            lo_val += modulus;
        }
        lo_val -= omega_factor;
        element[i + 0] = hi_val;
        element[i + 1] = lo_val;
    }
}

fn forward_transform_to_bit_reverse<T>(element: &Vec<T>,
                                       root_of_unity_table: &Vec<T>,
                                       precon_root_of_unity_table: &Vec<T>,
                                       result: &mut Vec<T>) {
    let n = element.get_length();
    if result.get_length() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.get_modulus();
    result.set_modulus(modulus);
    for i in 0..n {
        result[i] = element[i];
    }
    let mut t = n >> 1;
    let mut logt1 = get_msb(t);
    let mut m = 1;
    while m < n {
        let mut j1;
        let mut j2;
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            let index_omega = m + i;
            let omega = root_of_unity_table[index_omega];
            let precon_omega = precon_root_of_unity_table[index_omega];
            for index_lo in j1..j2 {
                let index_hi = index_lo + t;
                let lo_val = result[index_lo];
                let omega_factor = result[index_hi];
                if omega_factor != Zero::zero() {
                    omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
                    let hi_val = lo_val + omega_factor;
                    if hi_val >= modulus {
                        hi_val -= modulus;
                    }
                    if lo_val < omega_factor {
                        lo_val += modulus;
                    }
                    lo_val -= omega_factor;
                    result[index_lo] = hi_val;
                    result[index_hi] = lo_val;
                } else {
                    result[index_hi] = lo_val;
                }
            }
        }
        m <<= 1;
        t >>= 1;
        logt1 -= 1;
    }
}

fn inverse_transform_from_bit_reverse_in_place<T>(root_of_unity_inverse_table: &Vec<T>,
                                                  cyclo_order_inv: &T,
                                                  element: &mut Vec<T>) {
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
    m = n >> 1;
    while m >= 1 {
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
        m >>= 1;
    }
    for i in 0..n {
        element[i].mod_mul_fast_eq(cyclo_order_inv, modulus, mu);
    }
}

fn inverse_transform_from_bit_reverse<T>(element: &Vec<T>,
                                         root_of_unity_inverse_table: &Vec<T>,
                                         cyclo_order_inv: &T,
                                         result: &mut Vec<T>) {
    let n = element.get_length();
    if result.get_length() != n {
        panic!("size of input element and size of output element not of same size");
    }
    result.set_modulus(element.get_modulus());
    for i in 0..n {
        result[i] = element[i];
    }
    inverse_transform_from_bit_reverse_in_place(root_of_unity_inverse_table, cyclo_order_inv, result);
}

fn inverse_transform_from_bit_reverse_in_place<T>(root_of_unity_inverse_table: &Vec<T>,
                                                  precon_root_of_unity_inverse_table: &Vec<T>,
                                                  cyclo_order_inv: &T,
                                                  precon_cyclo_order_inv: &T,
                                                  element: &mut Vec<T>) {
    let modulus = element.get_modulus();
    let n = element.get_length();
    for i in (0..n).step_by(2) {
        let omega = root_of_unity_inverse_table[(i + n) >> 1];
        let precon_omega = precon_root_of_unity_inverse_table[(i + n) >> 1];
        let hi_val = element[i + 1];
        let lo_val = element[i + 0];
        let omega_factor = lo_val;
        if omega_factor < hi_val {
            omega_factor += modulus;
        }
        omega_factor -= hi_val;
        lo_val += hi_val;
        if lo_val >= modulus {
            lo_val -= modulus;
        }
        lo_val.mod_mul_fast_const_eq(cyclo_order_inv, modulus, precon_cyclo_order_inv);
        omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
        omega_factor.mod_mul_fast_const_eq(cyclo_order_inv, modulus, precon_cyclo_order_inv);
        element[i + 0] = lo_val;
        element[i + 1] = omega_factor;
    }
    let mut m = n >> 2;
    let mut t = 2;
    let mut logt = 2;
    while m >= 1 {
        for i in 0..m {
            let omega = root_of_unity_inverse_table[i + m];
            let precon_omega = precon_root_of_unity_inverse_table[i + m];
            for j1 in (i << logt)..(j1 + t) {
                let hi_val = element[j1 + t];
                let lo_val = element[j1 + 0];
                let omega_factor = lo_val;
                if omega_factor < hi_val {
                    omega_factor += modulus;
                }
                omega_factor -= hi_val;
                lo_val += hi_val;
                if lo_val >= modulus {
                    lo_val -= modulus;
                }
                omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
                element[j1 + 0] = lo_val;
                element[j1 + t] = omega_factor;
            }
        }
        m >>= 1;
        t <<= 1;
        logt += 1;
    }
}


use std::collections::HashMap;

pub struct NumberTheoreticTransformNat<T> {
    // implementation details
}

impl<T> NumberTheoreticTransformNat<T> {
    pub fn inverse_transform_from_bit_reverse(
        &self,
        element: &Vec<T>,
        root_of_unity_inverse_table: &Vec<T>,
        precon_root_of_unity_inverse_table: &Vec<T>,
        cyclo_order_inv: &T,
        precon_cyclo_order_inv: &T,
        result: &mut Vec<T>,
    ) {
        let n = element.len();
        if result.len() != n {
            panic!("size of input element and size of output element not of same size");
        }
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            result[i] = element[i];
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

pub struct ChineseRemainderTransformFTTNat<T> {
    // implementation details
}

impl<T> ChineseRemainderTransformFTTNat<T> {
    pub fn forward_transform_to_bit_reverse_in_place(
        &self,
        root_of_unity: &T,
        cyclo_order: usize,
        element: &mut Vec<T>,
    ) {
        if *root_of_unity == T(1) || *root_of_unity == T(0) {
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if element.len() != cyclo_order_hf {
            panic!("element size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformNat<VecType>().forward_transform_to_bit_reverse_in_place(
            m_root_of_unity_reverse_table_by_modulus[modulus],
            m_root_of_unity_precon_reverse_table_by_modulus[modulus],
            element,
        );
    }

    pub fn forward_transform_to_bit_reverse(
        &self,
        element: &Vec<T>,
        root_of_unity: &T,
        cyclo_order: usize,
        result: &mut Vec<T>,
    ) {
        if *root_of_unity == T(1) || *root_of_unity == T(0) {
            *result = element.clone();
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if result.len() != cyclo_order_hf {
            panic!("result size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let n = element.len();
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            result[i] = element[i];
        }
        NumberTheoreticTransformNat<VecType>().forward_transform_to_bit_reverse(
            element,
            m_root_of_unity_reverse_table_by_modulus[modulus],
            m_root_of_unity_precon_reverse_table_by_modulus[modulus],
            result,
        );
    }

    pub fn inverse_transform_from_bit_reverse_in_place(
        &self,
        root_of_unity: &T,
        cyclo_order: usize,
        element: &mut Vec<T>,
    ) {
        if *root_of_unity == T(1) || *root_of_unity == T(0) {
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if element.len() != cyclo_order_hf {
            panic!("element size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let msb = get_msb(cyclo_order_hf - 1);
        NumberTheoreticTransformNat<VecType>().inverse_transform_from_bit_reverse_in_place(
            m_root_of_unity_inverse_reverse_table_by_modulus[modulus],
            m_root_of_unity_inverse_precon_reverse_table_by_modulus[modulus],
            m_cyclo_order_inverse_table_by_modulus[modulus][msb],
            m_cyclo_order_inverse_precon_table_by_modulus[modulus][msb],
            element,
        );
    }

    pub fn inverse_transform_from_bit_reverse(
        &self,
        element: &Vec<T>,
        root_of_unity: &T,
        cyclo_order: usize,
        result: &mut Vec<T>,
    ) {
        if *root_of_unity == T(1) || *root_of_unity == T(0) {
            *result = element.clone();
            return;
        }
        if !is_power_of_two(cyclo_order) {
            panic!("CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if result.len() != cyclo_order_hf {
            panic!("result size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let n = element.len();
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            result[i] = element[i];
        }
        let msb = get_msb(cyclo_order_hf - 1);
        NumberTheoreticTransformNat<VecType>().inverse_transform_from_bit_reverse_in_place(
            m_root_of_unity_inverse_reverse_table_by_modulus[modulus],
            m_root_of_unity_inverse_precon_reverse_table_by_modulus[modulus],
            m_cyclo_order_inverse_table_by_modulus[modulus][msb],
            m_cyclo_order_inverse_precon_table_by_modulus[modulus][msb],
            result,
        );
    }

    pub fn pre_compute(&self, root_of_unity: &T, cyclo_order: usize, modulus: &T) {
        let cyclo_order_hf = cyclo_order >> 1;
        let map_search = m_root_of_unity_reverse_table_by_modulus.get(modulus);
        if map_search.is_none() || map_search.unwrap().len() != cyclo_order_hf {
            let x = T(1);
            let xinv = T(1);
            let msb = get_msb(cyclo_order_hf - 1);
            let mu = modulus.compute_mu();
            let mut table = Vec::with_capacity(cyclo_order_hf);
            let mut table_i = Vec::with_capacity(cyclo_order_hf);
            let root_of_unity_inverse = root_of_unity.mod_inverse(modulus);
            for i in 0..cyclo_order_hf {
                let iinv = reverse_bits(i, msb);
                table[iinv] = x;
                table_i[iinv] = xinv;
                x.mod_mul_eq(root_of_unity, modulus, mu);
                xinv.mod_mul_eq(root_of_unity_inverse, modulus, mu);
            }
            m_root_of_unity_reverse_table_by_modulus[modulus] = table;
            m_root_of_unity_inverse_reverse_table_by_modulus[modulus] = table_i;
            let mut table_coi = Vec::with_capacity(msb + 1);
            for i in 0..msb + 1 {
                let co_inv = (T(1 << i)).mod_inverse(modulus);
                table_coi[i] = co_inv;
            }
            m_cyclo_order_inverse_table_by_modulus[modulus] = table_coi;
            let native_modulus = modulus.convert_to_int();
            let mut precon_table = Vec::with_capacity(cyclo_order_hf);
            let mut precon_table_i = Vec::with_capacity(cyclo_order_hf);
            for i in 0..cyclo_order_hf {
                precon_table[i] = NativeInteger(m_root_of_unity_reverse_table_by_modulus[modulus][i].convert_to_int())
                    .prep_mod_mul_const(native_modulus);
                precon_table_i[i] = NativeInteger(m_root_of_unity_inverse_reverse_table_by_modulus[modulus][i].convert_to_int())
                    .prep_mod_mul_const(native_modulus);
            }
            let mut precon_table_coi = Vec::with_capacity(msb + 1);
            for i in 0..msb + 1 {
                precon_table_coi[i] = NativeInteger(m_cyclo_order_inverse_table_by_modulus[modulus][i].convert_to_int())
                    .prep_mod_mul_const(native_modulus);
            }
            m_root_of_unity_precon_reverse_table_by_modulus[modulus] = precon_table;
            m_root_of_unity_inverse_precon_reverse_table_by_modulus[modulus] = precon_table_i;
            m_cyclo_order_inverse_precon_table_by_modulus[modulus] = precon_table_coi;
        }
    }

    pub fn pre_compute_with_vec(
        &self,
        root_of_unity: &Vec<T>,
        cyclo_order: usize,
        modulii_chain: &Vec<T>,
    ) {
        let num_of_root_u = root_of_unity.len();
        let num_modulii = modulii_chain.len();
        if num_of_root_u != num_modulii {
            panic!("size of root of unity and size of moduli chain not of same size");
        }
        for i in 0..num_of_root_u {
            let current_root = root_of_unity[i];
            let current_mod = modulii_chain[i];
            self.pre_compute(&current_root, cyclo_order, &current_mod);
        }
    }

    pub fn reset(&self) {
        m_cyclo_order_inverse_table_by_modulus.clear();
        m_cyclo_order_inverse_precon_table_by_modulus.clear();
        m_root_of_unity_reverse_table_by_modulus.clear();
        m_root_of_unity_inverse_reverse_table_by_modulus.clear();
        m_root_of_unity_precon_reverse_table_by_modulus.clear();
        m_root_of_unity_inverse_precon_reverse_table_by_modulus.clear();
    }
}

pub struct BluesteinFFTNat<T> {
    // implementation details
}

impl<T> BluesteinFFTNat<T> {
    pub fn pre_compute_default_ntt_modulus_root(&self, cyclo_order: usize, modulus: &T) {
        let ntt_dim = 2usize.pow((2 * cyclo_order - 1).log2().ceil() as u32);
        let ntt_modulus = last_prime((ntt_dim as f64).log2() + 2 * modulus.get_msb(), ntt_dim);
        let ntt_root = root_of_unity(ntt_dim, ntt_modulus);
        let ntt_modulus_root = ModulusRoot {
            modulus: ntt_modulus,
            root: ntt_root,
        };
        m_default_ntt_modulus_root[modulus] = ntt_modulus_root;
        pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root);
    }
}


use std::collections::HashMap;
use num::traits::{Zero, One};
use num::pow;
use num::integer::Integer;
use num::bigint::BigInt;

struct BluesteinFFTNat<VecType> {
    m_rootOfUnityTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_rootOfUnityInverseTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_powersTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_RBTableByModulusRootPair: HashMap<ModulusRootPair<IntType>, VecType>,
    m_defaultNTTModulusRoot: HashMap<IntType, ModulusRoot<IntType>>,
}

impl<VecType> BluesteinFFTNat<VecType> {
    fn PreComputeRootTableForNTT(&mut self, cyclotoOrder: usint, nttModulusRoot: ModulusRoot<IntType>) {
        let nttDim = pow(2, (2 * cyclotoOrder - 1).log2().ceil());
        let nttModulus = nttModulusRoot.0;
        let nttRoot = nttModulusRoot.1;
        let root = IntType::from(nttRoot);
        let rootInv = root.ModInverse(nttModulus);
        let nttDimHf = nttDim >> 1;
        let mut rootTable = VecType::new(nttDimHf, nttModulus);
        let mut rootTableInverse = VecType::new(nttDimHf, nttModulus);
        let mut x = IntType::one();
        for i in 0..nttDimHf {
            rootTable[i] = x;
            x = x.ModMul(&root, &nttModulus);
        }
        x = IntType::one();
        for i in 0..nttDimHf {
            rootTableInverse[i] = x;
            x = x.ModMul(&rootInv, &nttModulus);
        }
        self.m_rootOfUnityTableByModulusRoot.insert(nttModulusRoot.clone(), rootTable);
        self.m_rootOfUnityInverseTableByModulusRoot.insert(nttModulusRoot, rootTableInverse);
    }

    fn PreComputePowers(&mut self, cycloOrder: usint, modulusRoot: ModulusRoot<IntType>) {
        let modulus = modulusRoot.0;
        let root = modulusRoot.1;
        let mut powers = VecType::new(cycloOrder, modulus);
        powers[0] = IntType::one();
        for i in 1..cycloOrder {
            let iSqr = (i * i) % (2 * cycloOrder);
            let val = root.ModExp(&IntType::from(iSqr), &modulus);
            powers[i] = val;
        }
        self.m_powersTableByModulusRoot.insert(modulusRoot, powers);
    }

    fn PreComputeRBTable(&mut self, cycloOrder: usint, modulusRootPair: ModulusRootPair<IntType>) {
        let modulusRoot = modulusRootPair.0;
        let modulus = modulusRoot.0;
        let root = modulusRoot.1;
        let rootInv = root.ModInverse(&modulus);
        let nttModulusRoot = modulusRootPair.1;
        let nttModulus = nttModulusRoot.0;

        let rootTable = self.m_rootOfUnityTableByModulusRoot.get(&nttModulusRoot).unwrap();
        let nttDim = pow(2, (2 * cycloOrder - 1).log2().ceil());
        let mut b = VecType::new(2 * cycloOrder - 1, modulus);
        b[cycloOrder - 1] = IntType::one();
        for i in 1..cycloOrder {
            let iSqr = (i * i) % (2 * cycloOrder);
            let val = rootInv.ModExp(&IntType::from(iSqr), &modulus);
            b[cycloOrder - 1 + i] = val;
            b[cycloOrder - 1 - i] = val;
        }
        let Rb = PadZeros(&b, nttDim);
        Rb.SetModulus(&nttModulus);
        let mut RB = VecType::new(nttDim);
        NumberTheoreticTransformNat<VecType>().ForwardTransformIterative(&Rb, &rootTable, &mut RB);
        self.m_RBTableByModulusRootPair.insert(modulusRootPair, RB);
    }

    fn ForwardTransform(&self, element: &VecType, root: &IntType, cycloOrder: usint) -> VecType {
        let modulus = element.GetModulus();
        let nttModulusRoot = self.m_defaultNTTModulusRoot.get(&modulus).unwrap();
        self.ForwardTransform(element, root, cycloOrder, nttModulusRoot)
    }

    fn ForwardTransform(&self, element: &VecType, root: &IntType, cycloOrder: usint, nttModulusRoot: &ModulusRoot<IntType>) -> VecType {
        if element.GetLength() != cycloOrder {
            panic!("expected size of element vector should be equal to cyclotomic order");
        }
        let modulus = element.GetModulus();
        let modulusRoot = (modulus, root.clone());
        let powers = self.m_powersTableByModulusRoot.get(&modulusRoot).unwrap();
        let nttModulus = nttModulusRoot.0;

        let rootTable = self.m_rootOfUnityTableByModulusRoot.get(&nttModulusRoot).unwrap();
        let rootTableInverse = self.m_rootOfUnityInverseTableByModulusRoot.get(&nttModulusRoot).unwrap();
        let x = element.ModMul(&powers);
        let nttDim = pow(2, (2 * cycloOrder - 1).log2().ceil());
        let Ra = PadZeros(&x, nttDim);
        Ra.SetModulus(&nttModulus);
        let mut RA = VecType::new(nttDim);
        NumberTheoreticTransformNat<VecType>().ForwardTransformIterative(&Ra, &rootTable, &mut RA);
        let modulusRootPair = (modulusRoot, nttModulusRoot.clone());
        let RB = self.m_RBTableByModulusRootPair.get(&modulusRootPair).unwrap();
        let RC = RA.ModMul(&RB);
        let mut Rc = VecType::new(nttDim);
        NumberTheoreticTransformNat<VecType>().InverseTransformIterative(&RC, &rootTableInverse, &mut Rc);
        let resizeRc = Resize(&Rc, cycloOrder - 1, 2 * (cycloOrder - 1));
        resizeRc.SetModulus(&modulus);
        resizeRc.ModEq(&modulus);
        let result = resizeRc.ModMul(&powers);
        result
    }

    fn PadZeros(a: &VecType, finalSize: usint) -> VecType {
        let s = a.GetLength();
        let mut result = VecType::new(finalSize, a.GetModulus());
        for i in 0..s {
            result[i] = a[i];
        }
        for i in s..finalSize {
            result[i] = IntType::zero();
        }
        result
    }

    fn Resize(a: &VecType, lo: usint, hi: usint) -> VecType {
        let mut result = VecType::new(hi - lo + 1, a.GetModulus());
        for i in lo..=hi {
            result[i - lo] = a[i];
        }
        result
    }

    fn Reset(&mut self) {
        self.m_rootOfUnityTableByModulusRoot.clear();
        self.m_rootOfUnityInverseTableByModulusRoot.clear();
        self.m_powersTableByModulusRoot.clear();
        self.m_RBTableByModulusRootPair.clear();
        self.m_defaultNTTModulusRoot.clear();
    }
}

struct ChineseRemainderTransformArbNat<VecType> {
    m_cyclotomicPolyMap: HashMap<IntType, VecType>,
    m_nttDivisionDim: HashMap<usint, usint>,
    m_DivisionNTTModulus: HashMap<IntType, IntType>,
    m_DivisionNTTRootOfUnity: HashMap<IntType, IntType>,
    m_rootOfUnityDivisionTableByModulus: HashMap<IntType, VecType>,
    m_rootOfUnityDivisionInverseTableByModulus: HashMap<IntType, VecType>,
    m_cyclotomicPolyReverseNTTMap: HashMap<IntType, VecType>,
    m_cyclotomicPolyNTTMap: HashMap<IntType, VecType>,
}

impl<VecType> ChineseRemainderTransformArbNat<VecType> {
    fn SetCylotomicPolynomial(&mut self, poly: &VecType, mod: &IntType) {
        self.m_cyclotomicPolyMap.insert(mod.clone(), poly.clone());
    }

    fn PreCompute(&mut self, cyclotoOrder: usint, modulus: &IntType) {
        BluesteinFFTNat<VecType>().PreComputeDefaultNTTModulusRoot(cyclotoOrder, modulus);
    }

    fn SetPreComputedNTTModulus(&mut self, cyclotoOrder: usint, modulus: &IntType, nttModulus: &IntType, nttRoot: &IntType) {
        let nttModulusRoot = (nttModulus.clone(), nttRoot.clone());
        BluesteinFFTNat<VecType>().PreComputeRootTableForNTT(cyclotoOrder, nttModulusRoot);
    }

    fn SetPreComputedNTTDivisionModulus(&mut self, cyclotoOrder: usint, modulus: &IntType, nttMod: &IntType, nttRootBig: &IntType) {
        let n = GetTotient(cyclotoOrder);
        let power = cyclotoOrder - n;
        self.m_nttDivisionDim.insert(cyclotoOrder, 2 * pow(2, (power as f64).log2().ceil()) as usint);
        let nttDimBig = pow(2, (2 * cyclotoOrder - 1).log2().ceil());
        let nttRoot = nttRootBig.ModExp(&(nttDimBig / self.m_nttDivisionDim[&cyclotoOrder]), nttMod);
        self.m_DivisionNTTModulus.insert(modulus.clone(), nttMod.clone());
        self.m_DivisionNTTRootOfUnity.insert(modulus.clone(), nttRoot.clone());

        let nttDim = self.m_nttDivisionDim[&cyclotoOrder];
        let root = IntType::from(nttRoot);
        let rootInv = root.ModInverse(nttMod);
        let nttDimHf = nttDim >> 1;
        let mut rootTable = VecType::new(nttDimHf, nttMod);
        let mut rootTableInverse = VecType::new(nttDimHf, nttMod);
        let mut x = IntType::one();
        for i in 0..nttDimHf {
            rootTable[i] = x;
            x = x.ModMul(&root, &nttMod);
        }
        x = IntType::one();
        for i in 0..nttDimHf {
            rootTableInverse[i] = x;
            x = x.ModMul(&rootInv, &nttMod);
        }
        self.m_rootOfUnityDivisionTableByModulus.insert(nttMod.clone(), rootTable);
        self.m_rootOfUnityDivisionInverseTableByModulus.insert(nttMod.clone(), rootTableInverse);

        let RevCPM = InversePolyMod(&self.m_cyclotomicPolyMap[modulus], modulus, power);
        let RevCPMPadded = BluesteinFFTNat<VecType>().PadZeros(&RevCPM, nttDim);
        RevCPMPadded.SetModulus(&nttMod);

        let mut RA = VecType::new(nttDim);
        NumberTheoreticTransformNat<VecType>().ForwardTransformIterative(&RevCPMPadded, &rootTable, &mut RA);
        self.m_cyclotomicPolyReverseNTTMap.insert(modulus.clone(), RA);
        let cycloPoly = &self.m_cyclotomicPolyMap[modulus];
        let mut QForwardTransform = VecType::new(nttDim, nttMod);
        for i in 0..cycloPoly.GetLength() {
            QForwardTransform[i] = cycloPoly[i];
        }
        let mut QFwdResult = VecType::new(nttDim);
        NumberTheoreticTransformNat<VecType>().ForwardTransformIterative(&QForwardTransform, &rootTable, &mut QFwdResult);
        self.m_cyclotomicPolyNTTMap.insert(modulus.clone(), QFwdResult);
    }
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

fn pow<T: Mul<Output = T> + Copy>(x: T, y: usize) -> T {
    let mut result = x;
    for _ in 1..y {
        result = result * x;
    }
    result
}

fn mod_sub<T: Sub<Output = T> + Rem<Output = T> + Copy>(a: T, b: T, modulus: T, mu: T) -> T {
    let diff = a - b;
    if diff >= T::zero() {
        diff
    } else {
        diff + modulus
    } % modulus
}

fn mod_add<T: Add<Output = T> + Rem<Output = T> + Copy>(a: T, b: T, modulus: T, mu: T) -> T {
    (a + b) % modulus
}

fn mod_inverse<T: Copy + Rem<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialEq>(
    a: T,
    modulus: T,
) -> Option<T> {
    let mut t = T::zero();
    let mut new_t = T::one();
    let mut r = modulus;
    let mut new_r = a;

    while new_r != T::zero() {
        let quotient = r / new_r;
        let old_t = t;
        t = new_t;
        new_t = old_t - quotient * new_t;
        let old_r = r;
        r = new_r;
        new_r = old_r - quotient * new_r;
    }

    if r > T::one() {
        None
    } else {
        Some((t + modulus) % modulus)
    }
}

fn polynomial_multiplication<T: Add<Output = T> + Mul<Output = T> + Copy>(
    a: &[T],
    b: &[T],
    modulus: T,
) -> Vec<T> {
    let mut result = vec![T::zero(); a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i + j] = result[i + j] + a[i] * b[j];
        }
    }
    result.iter().map(|&x| x % modulus).collect()
}

fn poly_mod<T: Sub<Output = T> + Rem<Output = T> + Copy>(
    a: &[T],
    b: &[T],
    modulus: T,
) -> Vec<T> {
    let mut result = a.to_vec();
    for i in 0..a.len() {
        if i < b.len() {
            result[i] = mod_sub(result[i], b[i], modulus, modulus);
        } else {
            result[i] = mod_sub(result[i], T::zero(), modulus, modulus);
        }
    }
    result
}

fn forward_transform<T: Add<Output = T> + Mul<Output = T> + Copy>(
    element: &[T],
    root: T,
    ntt_modulus: T,
    ntt_root: T,
    cyclo_order: usize,
) -> Vec<T> {
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
    drop(output_bluestein, cyclo_order, true, ntt_modulus, ntt_root)
}

fn inverse_transform<T: Add<Output = T> + Mul<Output = T> + Copy>(
    element: &[T],
    root: T,
    ntt_modulus: T,
    ntt_root: T,
    cyclo_order: usize,
) -> Vec<T> {
    let phim = get_totient(cyclo_order);
    if element.len() != phim {
        panic!("element size should be equal to phim");
    }
    let modulus = element[0];
    let root_inverse = mod_inverse(root, modulus).unwrap();
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
    let cyclotomic_inverse = mod_inverse(T::from(cyclo_order).unwrap(), modulus).unwrap();
    let mut output = drop(output_bluestein, cyclo_order, false, ntt_modulus, ntt_root);
    for i in 0..output.len() {
        output[i] = output[i] * cyclotomic_inverse;
    }
    output
}

fn pad<T: Copy>(element: &[T], cyclo_order: usize, forward: bool) -> Vec<T> {
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

fn drop<T: Copy>(element: &[T], cyclo_order: usize, forward: bool, big_mod: T, big_root: T) -> Vec<T> {
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
                    output[i] = mod_add(output[i], coeff_n, modulus, mu);
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
            let ab = polynomial_multiplication(&a, &m_cyclotomic_poly_reverse_ntt_map[&modulus], modulus);
            let root_table_inverse = &m_root_of_unity_division_inverse_table_by_modulus[&ntt_mod];
            let mut quotient = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            for i in 0..power {
                quotient[i] = ab[i];
            }
            quotient = mod_eq(&quotient, modulus);
            quotient = set_modulus(&quotient, ntt_mod);
            let mut new_quotient = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            forward_transform_iterative(&quotient, root_table, &mut new_quotient);
            new_quotient = polynomial_multiplication(&new_quotient, &m_cyclotomic_poly_ntt_map[&modulus], modulus);
            let mut new_quotient2 = vec![T::zero(); m_ntt_division_dim[&cyclo_order]];
            inverse_transform_iterative(&new_quotient, root_table_inverse, &mut new_quotient2);
            new_quotient2 = set_modulus(&new_quotient2, modulus);
            new_quotient2 = mod_eq(&new_quotient2, modulus);
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



