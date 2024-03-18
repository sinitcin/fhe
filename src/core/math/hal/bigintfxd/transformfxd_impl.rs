use std::collections::HashMap;

lazy_static! {
    static ref M_CYCLOORDERINVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOORDERINVERSEPRECON_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYREVERSETABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSEREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYPRECONREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSEPRECONREVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYREVERSENTTMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_CYCLOTOMICPOLYNTTMAP: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYTABLEBYMODULUSROOT: HashMap<ModulusRoot<usize>, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYINVERSETABLEBYMODULUSROOT: HashMap<ModulusRoot<usize>, VecType> = HashMap::new();
    static ref M_POWERSTABLEBYMODULUSROOT: HashMap<ModulusRoot<usize>, VecType> = HashMap::new();
    static ref M_RBTABLEBYMODULUSROOTPAIR: HashMap<ModulusRootPair<usize>, VecType> = HashMap::new();
    static ref M_DEFAULTNTTMODULUSROOT: HashMap<usize, ModulusRoot<usize>> = HashMap::new();
    static ref M_ROOTOFUNITYDIVISIONTABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_ROOTOFUNITYDIVISIONINVERSE_TABLEBYMODULUS: HashMap<usize, VecType> = HashMap::new();
    static ref M_DIVISIONNTTMODULUS: HashMap<usize, usize> = HashMap::new();
    static ref M_DIVISIONNTTROOTOFUNITY: HashMap<usize, usize> = HashMap::new();
    static ref M_NTTDIVISIONDIM: HashMap<usize, usize> = HashMap::new();
}

fn forward_transform_iterative(element: &VecType, root_of_unity_table: &VecType, result: &mut VecType) {
    let n = element.get_length();
    if result.get_length() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    result.set_modulus(modulus);
    let msb = get_msb64(n - 1);
    for i in 0..n {
        result[i] = element[reverse_bits(i, msb)];
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
    let logn = get_msb64(n - 1);
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
    let n = element.get_length();
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    forward_transform_iterative(element, root_of_unity_inverse_table, result);
    let cyclo_order_inv = IntType::from(n).mod_inverse(modulus);
    for i in 0..n {
        result[i].mod_mul_eq(cyclo_order_inv, modulus, mu);
    }
}

fn forward_transform_to_bit_reverse_in_place(root_of_unity_table: &VecType, element: &mut VecType) {
    let n = element.get_length();
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
    let mut logt1 = get_msb64(t);
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

fn forward_transform_to_bit_reverse(element: &VecType, root_of_unity_table: &VecType, result: &mut VecType) {
    let n = element.get_length();
    if result.get_length() != n {
        panic!("size of input element and size of output element not of same size");
    }
    let modulus = element.get_modulus();
    let mu = modulus.compute_mu();
    result.set_modulus(modulus);
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
    for i in 0..n {
        result[i] = element[i];
    }
    let mut t = n >> 1;
    let mut logt1 = get_msb64(t);
    for m in 1..n {
        for i in 0..m {
            j1 = i << logt1;
            j2 = j1 + t;
            index_omega = m + i;
            omega = root_of_unity_table[index_omega];
            for index_lo in j1..j2 {
                index_hi = index_lo + t;
                lo_val = result[index_lo];
                omega_factor = result[index_hi];
                if omega_factor != zero {
                    omega_factor.mod_mul_fast_eq(omega, modulus, mu);
                    hi_val = lo_val + omega_factor;
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


use std::collections::HashMap;

struct NumberTheoreticTransformFxd<VecType> {
    // implementation details
}

impl<VecType> NumberTheoreticTransformFxd<VecType> {
    fn forward_transform_to_bit_reverse_in_place(&self, root_of_unity_table: &VecType, precon_root_of_unity_table: &VecType, element: &mut VecType) {
        let n = element.get_length();
        let modulus = element.get_modulus();
        let mut index_omega: u32;
        let mut index_hi: u32;
        let mut precon_omega: IntType;
        let mut omega: IntType;
        let mut omega_factor: IntType;
        let mut lo_val: IntType;
        let mut hi_val: IntType;
        let zero = IntType(0);
        let mut t = n >> 1;
        let mut logt1 = get_msb64(t);
        for m in 1..n {
            m <<= 1;
            t >>= 1;
            logt1 -= 1;
            for i in 0..m {
                let j1 = i << logt1;
                let j2 = j1 + t;
                index_omega = m + i;
                omega = root_of_unity_table[index_omega];
                precon_omega = precon_root_of_unity_table[index_omega];
                for index_lo in j1..j2 {
                    index_hi = index_lo + t;
                    lo_val = element[index_lo];
                    omega_factor = element[index_hi];
                    omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
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
        }
    }

    fn forward_transform_to_bit_reverse(&self, element: &VecType, root_of_unity_table: &VecType, precon_root_of_unity_table: &VecType, result: &mut VecType) {
        let n = element.get_length();
        if result.get_length() != n {
            panic!("size of input element and size of output element not of same size");
        }
        let modulus = element.get_modulus();
        result.set_modulus(modulus);
        for i in 0..n {
            result[i] = element[i];
        }
        let mut index_omega: u32;
        let mut index_hi: u32;
        let mut precon_omega: IntType;
        let mut omega: IntType;
        let mut omega_factor: IntType;
        let mut lo_val: IntType;
        let mut hi_val: IntType;
        let zero = IntType(0);
        let mut t = n >> 1;
        let mut logt1 = get_msb64(t);
        for m in 1..n {
            m <<= 1;
            t >>= 1;
            logt1 -= 1;
            for i in 0..m {
                let j1 = i << logt1;
                let j2 = j1 + t;
                index_omega = m + i;
                omega = root_of_unity_table[index_omega];
                precon_omega = precon_root_of_unity_table[index_omega];
                for index_lo in j1..j2 {
                    index_hi = index_lo + t;
                    lo_val = result[index_lo];
                    omega_factor = result[index_hi];
                    if omega_factor != zero {
                        omega_factor.mod_mul_fast_const_eq(omega, modulus, precon_omega);
                        hi_val = lo_val + omega_factor;
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
        }
    }

    // other methods
}

struct ChineseRemainderTransformFTTFxd<VecType> {
    // implementation details
}

impl<VecType> ChineseRemainderTransformFTTFxd<VecType> {
    fn forward_transform_to_bit_reverse_in_place(&self, root_of_unity: &IntType, cyclo_order: usint, element: &mut VecType) {
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
            pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformFxd<VecType>().forward_transform_to_bit_reverse_in_place(
            m_root_of_unity_reverse_table_by_modulus[modulus], element);
    }

    fn forward_transform_to_bit_reverse(&self, element: &VecType, root_of_unity: &IntType, cyclo_order: usint, result: &mut VecType) {
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
            pre_compute(root_of_unity, cyclo_order, modulus);
        }
        NumberTheoreticTransformFxd<VecType>().forward_transform_to_bit_reverse(
            element, m_root_of_unity_reverse_table_by_modulus[modulus], result);
    }

    // other methods
}

// helper functions


use std::collections::HashMap;

struct ChineseRemainderTransformFTTFxd<VecType> {
    m_rootOfUnityReverseTableByModulus: HashMap<IntType, VecType>,
    m_rootOfUnityInverseReverseTableByModulus: HashMap<IntType, VecType>,
    m_cycloOrderInverseTableByModulus: HashMap<IntType, VecType>,
}

impl<VecType> ChineseRemainderTransformFTTFxd<VecType> {
    fn inverse_transform_from_bit_reverse_in_place(&self, root_of_unity: IntType, cyclo_order: usint, element: &mut VecType) {
        if root_of_unity == IntType(1) || root_of_unity == IntType(0) {
            return;
        }
        if !is_power_of_two(cyclo_order) {
            OPENFHE_THROW(math_error, "CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if element.get_length() != cyclo_order_hf {
            OPENFHE_THROW(math_error, "element size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let msb = get_msb64(cyclo_order_hf - 1);
        NumberTheoreticTransformFxd<VecType>().inverse_transform_from_bit_reverse_in_place(
            self.m_rootOfUnityInverseReverseTableByModulus[&modulus],
            self.m_cycloOrderInverseTableByModulus[&modulus][msb],
            element,
        );
    }

    fn inverse_transform_from_bit_reverse(&self, element: &VecType, root_of_unity: IntType, cyclo_order: usint, result: &mut VecType) {
        if root_of_unity == IntType(1) || root_of_unity == IntType(0) {
            *result = element.clone();
            return;
        }
        if !is_power_of_two(cyclo_order) {
            OPENFHE_THROW(math_error, "CyclotomicOrder is not a power of two");
        }
        let cyclo_order_hf = cyclo_order >> 1;
        if result.get_length() != cyclo_order_hf {
            OPENFHE_THROW(math_error, "result size must be equal to CyclotomicOrder / 2");
        }
        let modulus = element.get_modulus();
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            self.pre_compute(root_of_unity, cyclo_order, modulus);
        }
        let n = element.get_length();
        result.set_modulus(element.get_modulus());
        for i in 0..n {
            (*result)[i] = element[i];
        }
        let msb = get_msb64(cyclo_order_hf - 1);
        NumberTheoreticTransformFxd<VecType>().inverse_transform_from_bit_reverse_in_place(
            self.m_rootOfUnityInverseReverseTableByModulus[&modulus],
            self.m_cycloOrderInverseTableByModulus[&modulus][msb],
            result,
        );
    }

    fn pre_compute(&self, root_of_unity: IntType, cyclo_order: usint, modulus: IntType) {
        let cyclo_order_hf = cyclo_order >> 1;
        let map_search = self.m_rootOfUnityReverseTableByModulus.get(&modulus);
        if map_search.is_none() || map_search.unwrap().get_length() != cyclo_order_hf {
            #pragma omp critical
            {
                let x = IntType(1);
                let xinv = IntType(1);
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
                    x.mod_mul_eq(root_of_unity.clone(), modulus.clone(), mu.clone());
                    xinv.mod_mul_eq(root_of_unity_inverse.clone(), modulus.clone(), mu.clone());
                }
                self.m_rootOfUnityReverseTableByModulus.insert(modulus.clone(), table);
                self.m_rootOfUnityInverseReverseTableByModulus.insert(modulus.clone(), table_i);
                let mut table_coi = VecType::new(msb + 1, modulus);
                for i in 0..msb + 1 {
                    let co_inv = IntType((1 << i).mod_inverse(modulus.clone()));
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
            OPENFHE_THROW(lbcrypto::math_error, "size of root of unity and size of moduli chain not of same size");
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

struct BluesteinFFTFxd<VecType> {
    m_rootOfUnityTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_rootOfUnityInverseTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_powersTableByModulusRoot: HashMap<ModulusRoot<IntType>, VecType>,
    m_RBTableByModulusRootPair: HashMap<ModulusRootPair<IntType>, VecType>,
    m_defaultNTTModulusRoot: HashMap<IntType, ModulusRoot<IntType>>,
}

impl<VecType> BluesteinFFTFxd<VecType> {
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
        let root = ntt_root.clone();
        let root_inv = root.mod_inverse(ntt_modulus.clone());
        let ntt_dim_hf = ntt_dim >> 1;
        let mut root_table = VecType::new(ntt_dim_hf, ntt_modulus.clone());
        let mut root_table_inverse = VecType::new(ntt_dim_hf, ntt_modulus.clone());
        let mut x = IntType(1);
        for i in 0..ntt_dim_hf {
            root_table[i] = x.clone();
            x = x.mod_mul(root.clone(), ntt_modulus.clone());
        }
        x = IntType(1);
        for i in 0..ntt_dim_hf {
            root_table_inverse[i] = x.clone();
            x = x.mod_mul(root_inv.clone(), ntt_modulus.clone());
        }
        self.m_rootOfUnityTableByModulusRoot.insert(ntt_modulus_root.clone(), root_table);
        self.m_rootOfUnityInverseTableByModulusRoot.insert(ntt_modulus_root.clone(), root_table_inverse);
    }

    fn pre_compute_powers(&self, cyclo_order: usint, modulus_root: ModulusRoot<IntType>) {
        let modulus = modulus_root.first;
        let root = modulus_root.second;
        let mut powers = VecType::new(cyclo_order, modulus.clone());
        powers[0] = 1;
        for i in 1..cyclo_order {
            let i_sqr = (i * i) % (2 * cyclo_order);
            let val = root.mod_exp(IntType(i_sqr), modulus.clone());
            powers[i] = val;
        }
        self.m_powersTableByModulusRoot.insert(modulus_root.clone(), powers);
    }

    fn pre_compute_rb_table(&self, cyclo_order: usint, modulus_root_pair: ModulusRootPair<IntType>) {
        let modulus_root = modulus_root_pair.first;
        let modulus = modulus_root.first;
        let root = modulus_root.second;
        let root_inv = root.mod_inverse(modulus.clone());
        let ntt_modulus_root = modulus_root_pair.second;
        let ntt_modulus = ntt_modulus_root.first;
        let root_table = self.m_rootOfUnityTableByModulusRoot[&ntt_modulus_root];
        let ntt_dim = pow(2, ceil(log2(2 * cyclo_order - 1)));
        let mut b = VecType::new(2 * cyclo_order - 1, modulus.clone());
        b[cyclo_order - 1] = 1;
        for i in 1..cyclo_order {
            let i_sqr = (i * i) % (2 * cyclo_order);
            let val = root_inv.mod_exp(IntType(i_sqr), modulus.clone());
            b[cyclo_order - 1 + i] = val;
            b[cyclo_order - 1 - i] = val;
        }
        let rb = pad_zeros(b, ntt_dim);
        rb.set_modulus(ntt_modulus.clone());
        let mut RB = VecType::new(ntt_dim);
        NumberTheoreticTransformFxd<VecType>().forward_transform_iterative(rb, root_table, &mut RB);
        self.m_RBTableByModulusRootPair.insert(modulus_root_pair.clone(), RB);
    }

    fn forward_transform(&self, element: &VecType, root: IntType, cyclo_order: usint) -> VecType {
        let modulus = element.get_modulus();
        let ntt_modulus_root = self.m_defaultNTTModulusRoot[&modulus];
        self.forward_transform(element, root, cyclo_order, ntt_modulus_root)
    }

    fn forward_transform(&self, element: &VecType, root: IntType, cyclo_order: usint, ntt_modulus_root: ModulusRoot<IntType>) -> VecType {
        if element.get_length() != cyclo_order {
            OPENFHE_THROW(math_error, "expected size of element vector should be equal to cyclotomic order");
        }
        let modulus = element.get_modulus();
        let modulus_root = ModulusRoot(modulus, root);
        let powers = self.m_powersTableByModulusRoot[&modulus_root];
        let ntt_modulus = ntt_modulus_root.first;
        let root_table = self.m_rootOfUnityTableByModulusRoot[&ntt_modulus_root];
        let root_table_inverse = self.m_rootOfUnityInverseTableByModulusRoot[&ntt_modulus_root];
        let mut x = element.mod_mul(&powers);
        let ntt_dim = pow(2, ceil(log2(2 * cyclo_order - 1)));
        let mut Ra = pad_zeros(x, ntt_dim);
        Ra.set_modulus(ntt_modulus.clone());
        let mut RA = VecType::new(ntt_dim);
        NumberTheoreticTransformFxd<VecType>().forward_transform_iterative(Ra, root_table, &mut RA);
        let modulus_root_pair = ModulusRootPair(modulus_root, ntt_modulus_root);
        let RB = self.m_RBTableByModulusRootPair[&modulus_root_pair];
        let mut RC = RA.mod_mul(&RB);
        let mut Rc = VecType::new(ntt_dim);
        NumberTheoreticTransformFxd<VecType>().inverse_transform_iterative(RC, root_table_inverse, &mut Rc);
        let resize_rc = resize(Rc, cyclo_order - 1, 2 * (cyclo_order - 1));
        resize_rc.set_modulus(modulus.clone());
        resize_rc.mod_eq(modulus.clone());
        let result = resize_rc.mod_mul(&powers);
        result
    }

    fn pad_zeros(&self, a: &VecType, final_size: usint) -> VecType {
        let s = a.get_length();
        let mut result = VecType::new(final_size, a.get_modulus());
        for i in 0..s {
            result[i] = a[i].clone();
        }
        for i in s..final_size {
            result[i] = IntType(0);
        }
        result
    }

    fn resize(&self, a: &VecType, lo: usint, hi: usint) -> VecType {
        let mut result = VecType::new(hi - lo + 1, a.get_modulus());
        for i in lo..=hi {
            result[i - lo] = a[i].clone();
        }
        result
    }

    fn reset(&self) {
        self.m_rootOfUnityTableByModulusRoot.clear();
        self.m_rootOfUnityInverseTableByModulusRoot.clear();
        self.m_powersTableByModulusRoot.clear();
        self.m_RBTableByModulusRootPair.clear();
        self.m_defaultNTTModulusRoot.clear();
    }
}

struct ChineseRemainderTransformArbFxd<VecType> {
    m_cyclotomicPolyMap: HashMap<IntType, VecType>,
}

impl<VecType> ChineseRemainderTransformArbFxd<VecType> {
    fn set_cylotomic_polynomial(&self, poly: VecType, mod: IntType) {
        self.m_cyclotomicPolyMap.insert(mod, poly);
    }

    fn pre_compute(&self, cycloto_order: usint, modulus: IntType) {
        BluesteinFFTFxd<VecType>().pre_compute_default_ntt_modulus_root(cycloto_order, modulus);
    }

    fn set_pre_computed_ntt_modulus(&self, cycloto_order: usint, modulus: IntType, ntt_modulus: IntType, ntt_root: IntType) {
        let ntt_modulus_root = ModulusRoot(ntt_modulus, ntt_root);
        BluesteinFFTFxd<VecType>().pre_compute_root_table_for_ntt(cycloto_order, ntt_modulus_root);
    }
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Debug)]
struct IntType {
    value: i64,
}

impl IntType {
    fn new(value: i64) -> Self {
        IntType { value }
    }

    fn mod_exp(&self, exponent: IntType, modulus: IntType) -> IntType {
        let mut result = IntType::new(1);
        let mut base = self.value.rem_euclid(modulus.value);
        let mut exp = exponent.value;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&IntType::new(base)).rem(&modulus);
            }
            base = base.mul(&IntType::new(base)).rem(&modulus);
            exp /= 2;
        }
        result
    }

    fn mod_inverse(&self, modulus: IntType) -> IntType {
        let mut a = self.value;
        let mut b = modulus.value;
        let mut x = 0;
        let mut y = 1;
        while a != 0 {
            let q = b / a;
            let temp = b;
            b = a;
            a = temp - q * a;
            let temp = x;
            x = y;
            y = temp - q * y;
        }
        if b == 1 {
            IntType::new((x + modulus.value) % modulus.value)
        } else {
            panic!("Modular inverse does not exist");
        }
    }

    fn mod_add(&self, other: IntType, modulus: IntType) -> IntType {
        IntType::new((self.value + other.value).rem_euclid(modulus.value))
    }

    fn mod_sub(&self, other: IntType, modulus: IntType) -> IntType {
        IntType::new((self.value - other.value).rem_euclid(modulus.value))
    }

    fn mod_mul(&self, other: IntType, modulus: IntType) -> IntType {
        IntType::new((self.value * other.value).rem_euclid(modulus.value))
    }

    fn mod_sub_eq(&mut self, other: IntType, modulus: IntType) {
        self.value = (self.value - other.value).rem_euclid(modulus.value);
    }

    fn mod_add_eq(&mut self, other: IntType, modulus: IntType) {
        self.value = (self.value + other.value).rem_euclid(modulus.value);
    }

    fn mod_mul_eq(&mut self, other: IntType, modulus: IntType) {
        self.value = (self.value * other.value).rem_euclid(modulus.value);
    }

    fn mod_eq(&mut self, modulus: IntType) {
        self.value = self.value.rem_euclid(modulus.value);
    }
}

impl Add for IntType {
    type Output = IntType;

    fn add(self, other: IntType) -> IntType {
        IntType::new(self.value + other.value)
    }
}

impl Sub for IntType {
    type Output = IntType;

    fn sub(self, other: IntType) -> IntType {
        IntType::new(self.value - other.value)
    }
}

impl Mul for IntType {
    type Output = IntType;

    fn mul(self, other: IntType) -> IntType {
        IntType::new(self.value * other.value)
    }
}

impl Rem for IntType {
    type Output = IntType;

    fn rem(self, modulus: IntType) -> IntType {
        IntType::new(self.value.rem_euclid(modulus.value))
    }
}

impl Div for IntType {
    type Output = IntType;

    fn div(self, other: IntType) -> IntType {
        IntType::new(self.value / other.value)
    }
}

#[derive(Clone, Debug)]
struct VecType {
    values: Vec<IntType>,
    modulus: IntType,
}

impl VecType {
    fn new(values: Vec<IntType>, modulus: IntType) -> Self {
        VecType { values, modulus }
    }

    fn get_length(&self) -> usize {
        self.values.len()
    }

    fn get_modulus(&self) -> IntType {
        self.modulus.clone()
    }

    fn get(&self, index: usize) -> IntType {
        self.values[index].clone()
    }

    fn set(&mut self, index: usize, value: IntType) {
        self.values[index] = value;
    }

    fn pad_zeros(&self, size: usize) -> Self {
        let mut values = self.values.clone();
        values.resize(size, IntType::new(0));
        VecType::new(values, self.modulus.clone())
    }

    fn forward_transform_iterative(&self, root_table: &VecType, result: &mut VecType) {
        let n = self.get_length();
        let mut j = 0;
        for i in 0..n - 1 {
            if i < j {
                self.swap(i, j);
            }
            let mut k = n / 2;
            while k <= j {
                j -= k;
                k /= 2;
            }
            j += k;
        }
        let mut m = 1;
        while m < n {
            let omega_m = root_table.get(m);
            let mut i = 0;
            while i < n {
                let mut omega = IntType::new(1);
                for j in 0..m {
                    let mut t = self.get(i + j + m).clone();
                    t = t.mul(&omega);
                    let mut u = self.get(i + j).clone();
                    u = u.add(t).rem(&self.modulus);
                    let mut v = self.get(i + j + m).clone();
                    v = u.sub(t).rem(&self.modulus);
                    self.set(i + j, u);
                    self.set(i + j + m, v);
                    omega = omega.mul(&omega_m);
                }
                i += 2 * m;
            }
            m *= 2;
        }
        *result = self.clone();
    }

    fn inverse_transform_iterative(&self, root_table_inverse: &VecType, result: &mut VecType) {
        let n = self.get_length();
        let mut j = 0;
        for i in 0..n - 1 {
            if i < j {
                self.swap(i, j);
            }
            let mut k = n / 2;
            while k <= j {
                j -= k;
                k /= 2;
            }
            j += k;
        }
        let mut m = 1;
        while m < n {
            let omega_m = root_table_inverse.get(m);
            let mut i = 0;
            while i < n {
                let mut omega = IntType::new(1);
                for j in 0..m {
                    let mut t = self.get(i + j + m).clone();
                    t = t.mul(&omega);
                    let mut u = self.get(i + j).clone();
                    u = u.add(t).rem(&self.modulus);
                    let mut v = self.get(i + j + m).clone();
                    v = u.sub(t).rem(&self.modulus);
                    self.set(i + j, u);
                    self.set(i + j + m, v);
                    omega = omega.mul(&omega_m);
                }
                i += 2 * m;
            }
            m *= 2;
        }
        let n_inv = IntType::new(n as i64).mod_inverse(self.modulus.clone());
        for i in 0..n {
            let mut t = self.get(i).clone();
            t = t.mul(&n_inv);
            self.set(i, t);
        }
        *result = self.clone();
    }

    fn swap(&mut self, i: usize, j: usize) {
        let temp = self.get(i);
        self.set(i, self.get(j));
        self.set(j, temp);
    }

    fn multiply(&self, other: &VecType) -> VecType {
        let n = self.get_length();
        let mut result = VecType::new(vec![IntType::new(0); n], self.modulus.clone());
        for i in 0..n {
            let mut t = self.get(i).clone();
            t = t.mul(&other.get(i));
            result.set(i, t);
        }
        result
    }

    fn mod_eq(&mut self, modulus: IntType) {
        self.modulus = modulus;
        for i in 0..self.get_length() {
            self.set(i, self.get(i).rem(&modulus));
        }
    }
}

#[derive(Clone, Debug)]
struct ModulusRoot<T> {
    modulus: T,
    root: T,
}

#[derive(Clone, Debug)]
struct ModulusRootPair<T> {
    modulus_root1: ModulusRoot<T>,
    modulus_root2: ModulusRoot<T>,
}

#[derive(Clone, Debug)]
struct BluesteinFFTFxd<T> {
    root_of_unity_table_by_modulus_root: HashMap<ModulusRoot<T>, VecType>,
    powers_table_by_modulus_root: HashMap<ModulusRoot<T>, VecType>,
    rb_table_by_modulus_root_pair: HashMap<ModulusRootPair<T>, VecType>,
}

impl<T> BluesteinFFTFxd<T>
where
    T: Clone + PartialEq,
{
    fn new() -> Self {
        BluesteinFFTFxd {
            root_of_unity_table_by_modulus_root: HashMap::new(),
            powers_table_by_modulus_root: HashMap::new(),
            rb_table_by_modulus_root_pair: HashMap::new(),
        }
    }

    fn pre_compute_root_table_for_ntt(&mut self, cyclo_order: usize, ntt_modulus_root: ModulusRoot<T>) {
        let ntt_modulus = ntt_modulus_root.modulus.clone();
        let ntt_root = ntt_modulus_root.root.clone();
        let mut root_table = VecType::new(Vec::new(), ntt_modulus.clone());
        let phim = get_totient(cyclo_order);
        let omega = ntt_root.mod_exp(
            IntType::new(phim as i64 / cyclo_order as i64),
            ntt_modulus.clone(),
        );
        let omega_inv = omega.mod_inverse(ntt_modulus.clone());
        let mut omega_m = IntType::new(1);
        for _ in 0..cyclo_order {
            root_table.values.push(omega_m.clone());
            omega_m = omega_m.mul(&omega);
        }
        self.root_of_unity_table_by_modulus_root
            .insert(ntt_modulus_root, root_table);
    }

    fn pre_compute_powers(&mut self, cyclo_order: usize, modulus_root: ModulusRoot<T>) {
        let modulus = modulus_root.modulus.clone();
        let root = modulus_root.root.clone();
        let mut powers_table = VecType::new(Vec::new(), modulus.clone());
        let phim = get_totient(cyclo_order);
        let mut omega_m = IntType::new(1);
        for _ in 0..cyclo_order {
            powers_table.values.push(omega_m.clone());
            omega_m = omega_m.mul(&root);
        }
        self.powers_table_by_modulus_root
            .insert(modulus_root, powers_table);
    }

    fn pre_compute_rb_table(&mut self, cyclo_order: usize, modulus_root_pair: ModulusRootPair<T>) {
        let modulus_root1 = modulus_root_pair.modulus_root1.clone();
        let modulus_root2 = modulus_root_pair.modulus_root2.clone();
        let modulus1 = modulus_root1.modulus.clone();
        let modulus2 = modulus_root2.modulus.clone();
        let root1 = modulus_root1.root.clone();
        let root2 = modulus_root2.root.clone();
        let mut rb_table = VecType::new(Vec::new(), modulus1.clone());
        let phim = get_totient(cyclo_order);
        let mut omega_m = IntType::new(1);
        for _ in 0..cyclo_order {
            rb_table.values.push(omega_m.clone());
            omega_m = omega_m.mul(&root1).mul(&root2);
        }
        self.rb_table_by_modulus_root_pair
            .insert(modulus_root_pair, rb_table);
    }

    fn forward_transform(
        &self,
        input: VecType,
        root: IntType,
        cyclo_order: usize,
        ntt_modulus_root: ModulusRoot<T>,
    ) -> VecType {
        let ntt_modulus = ntt_modulus_root.modulus.clone();
        let ntt_root = ntt_modulus_root.root.clone();
        let mut result = VecType::new(Vec::new(), ntt_modulus.clone());
        let phim = get_totient(cyclo_order);
        if input.get_length() != phim {
            panic!("element size should be equal to phim");
        }
        let modulus = input.get_modulus();
        let modulus_root = ModulusRoot {
            modulus: modulus.clone(),
            root: root.clone(),
        };
        let modulus_root_inverse = ModulusRoot {
            modulus: modulus.clone(),
            root: root.mod_inverse(modulus.clone()),
        };
        let modulus_root_pair = ModulusRootPair {
            modulus_root1: modulus_root_inverse.clone(),
            modulus_root2: ntt_modulus_root.clone(),
        };
        if self
            .root_of_unity_table_by_modulus_root
            .get(&ntt_modulus_root)
            .unwrap()
            .get_length()
            == 0
        {
            self.pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root.clone());
        }
        if self
            .powers_table_by_modulus_root
            .get(&modulus_root)
            .unwrap()
            .get_length()
            == 0
        {
            self.pre_compute_powers(cyclo_order, modulus_root.clone());
        }
        if self
            .rb_table_by_modulus_root_pair
            .get(&modulus_root_pair)
            .unwrap()
            .get_length()
            == 0
        {
            self.pre_compute_rb_table(cyclo_order, modulus_root_pair.clone());
        }
        let input_to_bluestein = input.pad_zeros(cyclo_order);
        let mut output_bluestein = input_to_bluestein.clone();
        output_bluestein.forward_transform_iterative(
            self.root_of_unity_table_by_modulus_root
                .get(&ntt_modulus_root)
                .unwrap(),
            &mut result,
        );
        let output = output_bluestein.drop(
            cyclo_order,
            true,
            ntt_modulus.clone(),
            ntt_root.clone(),
        );
        output
    }

    fn reset(&mut self) {
        self.root_of_unity_table_by_modulus_root.clear();
        self.powers_table_by_modulus_root.clear();
        self.rb_table_by_modulus_root_pair.clear();
    }
}

#[derive(Clone, Debug)]
struct ChineseRemainderTransformArbFxd<T> {
    cyclotomic_poly_map: HashMap<IntType, VecType>,
    cyclotomic_poly_reverse_ntt_map: HashMap<IntType, VecType>,
    cyclotomic_poly_ntt_map: HashMap<IntType, VecType>,
    root_of_unity_division_table_by_modulus: HashMap<IntType, VecType>,
    root_of_unity_division_inverse_table_by_modulus: HashMap<IntType, VecType>,
    division_ntt_modulus: HashMap<IntType, IntType>,
    division_ntt_root_of_unity: HashMap<IntType, IntType>,
    ntt_division_dim: HashMap<usiz, usize>,
    bluestein_fft_fxd: BluesteinFFTFxd<T>,
}

impl<T> ChineseRemainderTransformArbFxd<T>
where
    T: Clone + PartialEq,
{
    fn new() -> Self {
        ChineseRemainderTransformArbFxd {
            cyclotomic_poly_map: HashMap::new(),
            cyclotomic_poly_reverse_ntt_map: HashMap::new(),
            cyclotomic_poly_ntt_map: HashMap::new(),
            root_of_unity_division_table_by_modulus: HashMap::new(),
            root_of_unity_division_inverse_table_by_modulus: HashMap::new(),
            division_ntt_modulus: HashMap::new(),
            division_ntt_root_of_unity: HashMap::new(),
            ntt_division_dim: HashMap::new(),
            bluestein_fft_fxd: BluesteinFFTFxd::new(),
        }
    }

    fn set_pre_computed_ntt_division_modulus(
        &mut self,
        cycloto_order: usize,
        modulus: IntType,
        ntt_mod: IntType,
        ntt_root_big: IntType,
    ) {
        let n = get_totient(cycloto_order);
        let power = cycloto_order - n;
        self.ntt_division_dim
            .insert(cycloto_order, 2 * (2 as usize).pow((power as f64).log2().ceil() as u32));
        let ntt_dim_big = (2 * cycloto_order - 1).pow(2.0f64.log2().ceil() as u32);
        let ntt_root = ntt_root_big.mod_exp(
            IntType::new(ntt_dim_big / self.ntt_division_dim[&cycloto_order]),
            ntt_mod.clone(),
        );
        self.division_ntt_modulus.insert(modulus.clone(), ntt_mod.clone());
        self.division_ntt_root_of_unity
            .insert(modulus.clone(), ntt_root);
        let ntt_dim = self.ntt_division_dim[&cycloto_order];
        let root = ntt_root.clone();
        let root_inv = root.mod_inverse(ntt_mod.clone());
        let ntt_dim_hf = ntt_dim >> 1;
        let mut root_table = VecType::new(Vec::new(), ntt_mod.clone());
        let mut root_table_inverse = VecType::new(Vec::new(), ntt_mod.clone());
        let mut x = IntType::new(1);
        for _ in 0..ntt_dim_hf {
            root_table.values.push(x.clone());
            x = x.mul(&root);
        }
        x = IntType::new(1);
        for _ in 0..ntt_dim_hf {
            root_table_inverse.values.push(x.clone());
            x = x.mul(&root_inv);
        }
        self.root_of_unity_division_table_by_modulus
            .insert(ntt_mod.clone(), root_table);
        self.root_of_unity_division_inverse_table_by_modulus
            .insert(ntt_mod.clone(), root_table_inverse);
        let rev_cpm = self.inverse_poly_mod(
            self.cyclotomic_poly_map[&modulus].clone(),
            modulus.clone(),
            power,
        );
        let rev_cpm_padded = self.bluestein_fft_fxd.pad_zeros(rev_cpm, ntt_dim);
        rev_cpm_padded.mod_eq(ntt_mod.clone());
        let mut ra = VecType::new(vec![IntType::new(0); ntt_dim], modulus.clone());
        rev_cpm_padded.forward_transform_iterative(
            &root_table,
            &mut ra,
        );
        self.cyclotomic_poly_reverse_ntt_map.insert(modulus.clone(), ra);
        let cyclo_poly = self.cyclotomic_poly_map[&modulus].clone();
        let mut q_forward_transform = VecType::new(Vec::new(), ntt_mod.clone());
        for i in 0..cyclo_poly.get_length() {
            q_forward_transform.values.push(cyclo_poly.get(i));
        }
        let mut q_fwd_result = VecType::new(vec![IntType::new(0); ntt_dim], modulus.clone());
        q_forward_transform.forward_transform_iterative(
            &root_table,
            &mut q_fwd_result,
        );
        self.cyclotomic_poly_ntt_map.insert(modulus.clone(), q_fwd_result);
    }

    fn inverse_poly_mod(&self, cyclo_poly: VecType, modulus: IntType, power: usize) -> VecType {
        let mut result = VecType::new(vec![IntType::new(0); power], modulus.clone());
        let r = (power as f64).log2().ceil() as usize;
        let mut h = VecType::new(vec![IntType::new(0); 1], modulus.clone());
        h.set(0, IntType::new(1));
        let mu = modulus.compute_mu();
        for i in 0..r {
            let q_degree = (2 as usize).pow(i as u32 + 1);
            let mut q = VecType::new(vec![IntType::new(0); q_degree + 1], modulus.clone());
            q.set(q_degree, IntType::new(1));
            let h_square = h.multiply(&h);
            let mut a = h.clone();
            a.mod_mul_eq(IntType::new(2), modulus.clone());
            let mut b = h_square.multiply(&cyclo_poly);
            for j in 0..b.get_length() {
                if j < a.get_length() {
                    b.set(j, a.get(j).mod_sub(b.get(j), modulus.clone(), mu.clone()));
                } else {
                    b.set(j, modulus.mod_sub(b.get(j), modulus.clone(), mu.clone()));
                }
            }
            h = self.poly_mod(b, q, modulus.clone());
        }
        for i in 0..power {
            result.set(i, h.get(i));
        }
        result
    }

    fn forward_transform(
        &self,
        element: VecType,
        root: IntType,
        ntt_modulus: IntType,
        ntt_root: IntType,
        cyclo_order: usize,
    ) -> VecType {
        let phim = get_totient(cyclo_order);
        if element.get_length() != phim {
            panic!("element size should be equal to phim");
        }
        let modulus = element.get_modulus();
        let modulus_root = ModulusRoot {
            modulus: modulus.clone(),
            root: root.clone(),
        };
        let ntt_modulus_root = ModulusRoot {
            modulus: ntt_modulus.clone(),
            root: ntt_root.clone(),
        };
        let modulus_root_pair = ModulusRootPair {
            modulus_root1: modulus_root.clone(),
            modulus_root2: ntt_modulus_root.clone(),
        };
        if self
            .bluestein_fft_fxd
            .root_of_unity_table_by_modulus_root
            .get(&ntt_modulus_root)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root.clone());
        }
        if self
            .bluestein_fft_fxd
            .powers_table_by_modulus_root
            .get(&modulus_root)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_powers(cyclo_order, modulus_root.clone());
        }
        if self
            .bluestein_fft_fxd
            .rb_table_by_modulus_root_pair
            .get(&modulus_root_pair)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_rb_table(cyclo_order, modulus_root_pair.clone());
        }
        let input_to_bluestein = element.pad_zeros(cyclo_order, true);
        let mut output_bluestein = input_to_bluestein.clone();
        output_bluestein.forward_transform_iterative(
            self.bluestein_fft_fxd
                .root_of_unity_table_by_modulus_root
                .get(&ntt_modulus_root)
                .unwrap(),
            &mut output_bluestein,
        );
        let output = output_bluestein.drop(
            cyclo_order,
            true,
            ntt_modulus.clone(),
            ntt_root.clone(),
        );
        output
    }

    fn inverse_transform(
        &self,
        element: VecType,
        root: IntType,
        ntt_modulus: IntType,
        ntt_root: IntType,
        cyclo_order: usize,
    ) -> VecType {
        let phim = get_totient(cyclo_order);
        if element.get_length() != phim {
            panic!("element size should be equal to phim");
        }
        let modulus = element.get_modulus();
        let root_inverse = root.mod_inverse(modulus.clone());
        let modulus_root_inverse = ModulusRoot {
            modulus: modulus.clone(),
            root: root_inverse.clone(),
        };
        let ntt_modulus_root = ModulusRoot {
            modulus: ntt_modulus.clone(),
            root: ntt_root.clone(),
        };
        let modulus_root_pair = ModulusRootPair {
            modulus_root1: modulus_root_inverse.clone(),
            modulus_root2: ntt_modulus_root.clone(),
        };
        if self
            .bluestein_fft_fxd
            .root_of_unity_table_by_modulus_root
            .get(&ntt_modulus_root)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_root_table_for_ntt(cyclo_order, ntt_modulus_root.clone());
        }
        if self
            .bluestein_fft_fxd
            .powers_table_by_modulus_root
            .get(&modulus_root_inverse)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_powers(cyclo_order, modulus_root_inverse.clone());
        }
        if self
            .bluestein_fft_fxd
            .rb_table_by_modulus_root_pair
            .get(&modulus_root_pair)
            .unwrap()
            .get_length()
            == 0
        {
            self.bluestein_fft_fxd
                .pre_compute_rb_table(cyclo_order, modulus_root_pair.clone());
        }
        let input_to_bluestein = element.pad_zeros(cyclo_order, false);
        let mut output_bluestein = input_to_bluestein.clone();
        output_bluestein.forward_transform_iterative(
            self.bluestein_fft_fxd
                .root_of_unity_table_by_modulus_root
                .get(&ntt_modulus_root)
                .unwrap(),
            &mut output_bluestein,
        );
        let cyclotomic_inverse = IntType::new(cyclo_order as i64).mod_inverse(modulus.clone());
        output_bluestein.mod_mul_eq(cyclotomic_inverse, modulus.clone());
        let output = output_bluestein.drop(
            cyclo_order,
            false,
            ntt_modulus.clone(),
            ntt_root.clone(),
        );
        output
    }

    fn pad(&self, element: VecType, cyclo_order: usize, forward: bool) -> VecType {
        let n = get_totient(cyclo_order);
        let modulus = element.get_modulus();
        let mut input_to_bluestein = VecType::new(Vec::new(), modulus.clone());
        if forward {
            for i in 0..n {
                input_to_bluestein.values.push(element.get(i));
            }
        } else {
            let t_list = get_totient_list(cyclo_order);
            let mut i = 0;
            for coprime in t_list {
                input_to_bluestein.values.push(element.get(coprime));
                i += 1;
            }
        }
        input_to_bluestein
    }

    fn drop(
        &self,
        element: VecType,
        cyclo_order: usize,
        forward: bool,
        big_mod: IntType,
        big_root: IntType,
    ) -> VecType {
        let n = get_totient(cyclo_order);
        let modulus = element.get_modulus();
        let mut output = VecType::new(Vec::new(), modulus.clone());
        if forward {
            let t_list = get_totient_list(cyclo_order);
            for i in 0..n {
                output.values.push(element.get(t_list[i]));
            }
        } else {
            if n + 1 == cyclo_order {
                let mu = modulus.compute_mu();
                let coeff_n = element.get(n);
                for i in 0..n {
                    let mut t = element.get(i);
                    t = t.mod_sub(coeff_n.clone(), modulus.clone(), mu.clone());
                    output.values.push(t);
                }
            } else if (n + 1) * 2 == cyclo_order {
                let mu = modulus.compute_mu();
                for i in 0..n {
                    let coeff_i = element.get(i);
                    let coeff_ip = element.get(i + n + 1);
                    let mut t = coeff_i.mod_sub(coeff_ip.clone(), modulus.clone(), mu.clone());
                    output.values.push(t);
                }
                let coeff_n = element.get(n).mod_sub(element.get(2 * n + 1), modulus.clone(), mu.clone());
                for i in 0..n {
                    if i % 2 == 0 {
                        output.values[i].mod_sub_eq(coeff_n.clone(), modulus.clone(), mu.clone());
                    } else {
                        output.values[i].mod_add_eq(coeff_n.clone(), modulus.clone(), mu.clone());
                    }
                }
            } else {
                if self.root_of_unity_division_table_by_modulus.get(&big_mod).unwrap().get_length() == 0
                    || self.division_ntt_modulus[&modulus] != big_mod
                {
                    self.set_pre_computed_ntt_division_modulus(
                        cyclo_order,
                        modulus.clone(),
                        big_mod.clone(),
                        big_root.clone(),
                    );
                }
                let ntt_mod = self.division_ntt_modulus[&modulus];
                let root_table = self.root_of_unity_division_table_by_modulus.get(&ntt_mod).unwrap();
                let mut a_padded2 = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], ntt_mod.clone());
                let power = cyclo_order - n;
                for i in n..element.get_length() {
                    a_padded2.values[power - (i - n) - 1] = element.get(i);
                }
                let mut a = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], ntt_mod.clone());
                a_padded2.forward_transform_iterative(root_table, &mut a);
                let ab = a.multiply(&self.cyclotomic_poly_reverse_ntt_map[&modulus]);
                let root_table_inverse = self.root_of_unity_division_inverse_table_by_modulus.get(&ntt_mod).unwrap();
                let mut a = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], ntt_mod.clone());
                ab.inverse_transform_iterative(root_table_inverse, &mut a);
                let mut quotient = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], modulus.clone());
                for i in 0..power {
                    quotient.values[i] = a.values[i].clone();
                }
                quotient.mod_eq(ntt_mod.clone());
                let mut new_quotient = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], ntt_mod.clone());
                quotient.forward_transform_iterative(root_table, &mut new_quotient);
                new_quotient = new_quotient.multiply(&self.cyclotomic_poly_ntt_map[&modulus]);
                let mut new_quotient2 = VecType::new(vec![IntType::new(0); self.ntt_division_dim[&cyclo_order]], modulus.clone());
                new_quotient.inverse_transform_iterative(root_table_inverse, &mut new_quotient2);
                new_quotient2.mod_eq(modulus.clone());
                let mu = modulus.compute_mu();
                for i in 0..n {
                    let mut t = element.get(i);
                    t = t.mod_sub(new_quotient2.get(cyclo_order - 1 - i), modulus.clone(), mu.clone());
                    output.values.push(t);
                }
            }
        }
        output
    }

    fn reset(&mut self) {
        self.cyclotomic_poly_map.clear();
        self.cyclotomic_poly_reverse_ntt_map.clear();
        self.cyclotomic_poly_ntt_map.clear();
        self.root_of_unity_division_table_by_modulus.clear();
        self.root_of_unity_division_inverse_table_by_modulus.clear();
        self.division_ntt_modulus.clear();
        self.division_ntt_root_of_unity.clear();
        self.ntt_division_dim.clear();
        self.bluestein_fft_fxd.reset();
    }
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

fn get_totient_list(n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 1..n {
        if n % i != 0 {
            result.push(i);
        }
    }
    result
}



