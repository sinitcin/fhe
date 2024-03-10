use std::rc::Rc;
use std::vec::Vec;
use std::sync::Arc;
use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul};
use crate::errors::{ConfigError, NotImplementedError, NotAvailableError};
use crate::core::math::polynomial::DcrtpolyImpl;
use crate::core::math::integer::NativeInteger;
use crate::pke::scheme::gen_cryptocontext_parameters::Params;
use std::{f64, fmt};
use rayon::prelude::*;


struct DCRTPolyImpl<VecType> {
    params: Arc<Params>,
    format: Format,
    vectors: Vec<VecType>,
}

impl<VecType> fmt::Display for DCRTPolyImpl<VecType>
where
    VecType: fmt::Display, // Ensure VecType supports Display trait
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---START PRINT DOUBLE CRT-- WITH SIZE {}", self.m_vectors.len())?;
        for (i, v) in self.m_vectors.iter().enumerate() {
            writeln!(f, "VECTOR {}", i)?;
            writeln!(f, "{}", v)?;
        }
        writeln!(f, "---END PRINT DOUBLE CRT--")
    }
}


impl<VecType> DCRTPolyImpl<VecType>
where
    VecType: SwitchFormat + SwitchModulus + InverseExists + Sync + Send, // Traits to be defined based on VecType capabilities
 {
    fn new_from_poly_large_type(rhs: &PolyLargeType, params: Arc<Params>) -> Self {
        let mut instance = DCRTPolyImpl {
            params: params.clone(),
            format: rhs.get_format(),
            vectors: Vec::with_capacity(params.get_params().len()),
        };
        instance.params.set_original_modulus(rhs.get_modulus());
        let rdim = rhs.get_length();
        for p in params.get_params().iter() {
            let mut v = VecType::new(p.clone(), instance.format, true);
            let m = p.get_modulus();
            for j in 0..rdim {
                v[j] = rhs[j].modulus(m);
            }
            instance.vectors.push(v);
        }
        instance
    }

    pub fn drop_last_element(&mut self) {
        self.m_vectors.pop();
    }
    

    fn assign_from_poly_large_type(&mut self, rhs: &PolyLargeType) {
        self.params.set_original_modulus(rhs.get_modulus());
        self.vectors.clear();
        let rdim = rhs.get_length();
        for p in self.params.get_params().iter() {
            let mut v = VecType::new(p.clone(), self.format, true);
            let m = p.get_modulus();
            for e in 0..rdim {
                v[e] = rhs[e].modulus(m);
            }
            self.vectors.push(v);
        }
    }

    fn new_from_poly_type(rhs: &PolyType, params: Arc<Params>) -> Self {
        let mut instance = DCRTPolyImpl {
            params: params.clone(),
            format: rhs.get_format(),
            vectors: vec![VecType::new_from_poly(rhs); params.get_params().len()],
        };
        let p = params.get_params();
        for (i, v) in instance.vectors.iter_mut().enumerate().skip(1) {
            v.switch_modulus(&p[i].get_modulus(), &p[i].get_root_of_unity(), 0, 0);
        }
        instance
    }

    fn assign_from_poly_type(&mut self, rhs: &PolyType) {
        self.vectors.clear();
        let mut first = true;
        for p in self.params.get_params().iter() {
            let mut v = VecType::new_from_poly(rhs);
            if !first {
                v.switch_modulus(&p.get_modulus(), &p.get_root_of_unity(), 0, 0);
            }
            first = false;
            self.vectors.push(v);
        }
    }


    pub fn new(towers: Vec<PolyType>) -> Self {
        let m_params = towers[0].get_params();
        let m_format = towers[0].get_format();
        let mut parms = Vec::new();
        let cyclotomic_order = towers[0].get_cyclotomic_order();
        for v in &towers {
            if v.get_cyclotomic_order() != cyclotomic_order {
                panic!("Polys provided to constructor must have the same ring dimension");
            }
            parms.push(v.get_params());
        }
        let m_params = Rc::new(Params::new(cyclotomic_order, parms));
        DCRTPolyImpl {
            m_params: Some(m_params),
            m_format,
            m_vectors: towers,
        }
    }

    pub fn new_with_dgg(
        dgg: DggType,
        dcrt_params: Rc<Params>,
        format: Format,
    ) -> Self {
        let rdim = dcrt_params.get_ring_dimension();
        let dgg_values = dgg.generate_int_vector(rdim);
        let mut m_vectors = Vec::with_capacity(dcrt_params.get_params().len());
        for p in dcrt_params.get_params() {
            let modulus = p.get_modulus();
            let mut ildv = NativeVector::new(rdim, modulus);
            for j in 0..rdim {
                let k = dgg_values[j];
                let m = modulus.convert_to_int();
                let dcrt_qmodulus = m as NativeInteger::SignedNativeInt;
                let dgg_stddev = dgg.get_std();
                let k = if dgg_stddev > dcrt_qmodulus {
                    k % dcrt_qmodulus
                } else {
                    k
                };
                let ildv_j = if k < 0 {
                    k *= -1;
                    modulus.convert_to_int() - k as NativeInteger::Integer
                } else {
                    k as NativeInteger::Integer
                };
                ildv[j] = ildv_j;
            }
            let mut ilvector = PolyType::new(p);
            ilvector.set_values(ildv, Format::COEFFICIENT);
            ilvector.set_format(format);
            m_vectors.push(ilvector);
        }
        DCRTPolyImpl {
            m_params: Some(dcrt_params),
            m_format: format,
            m_vectors,
        }
    }

    pub fn new_with_dug(
        dug: DugType,
        dcrt_params: Rc<Params>,
        format: Format,
    ) -> Self {
        let mut m_vectors = Vec::with_capacity(dcrt_params.get_params().len());
        for p in dcrt_params.get_params() {
            let vals = dug.generate_vector(p.get_ring_dimension(), p.get_modulus());
            let mut ilvector = PolyType::new(p);
            ilvector.set_values(vals, format);
            m_vectors.push(ilvector);
        }
        DCRTPolyImpl {
            m_params: Some(dcrt_params),
            m_format: format,
            m_vectors,
        }
    }

    pub fn new_with_bug(
        bug: BugType,
        dcrt_params: Rc<Params>,
        format: Format,
    ) -> Self {
        let mut m_vectors = Vec::with_capacity(dcrt_params.get_params().len());
        let mut first = true;
        let mut ilvector = PolyType::new_with_bug(bug, dcrt_params.get_params()[0], Format::COEFFICIENT);
        for p in dcrt_params.get_params() {
            if !first {
                ilvector.switch_modulus(p.get_modulus(), p.get_root_of_unity(), 0, 0);
            }
            let new_vector = ilvector.clone();
            new_vector.set_format(format);
            m_vectors.push(new_vector);
            first = false;
        }
        DCRTPolyImpl {
            m_params: Some(dcrt_params),
            m_format: format,
            m_vectors,
        }
    }

    pub fn new_with_tug(
        tug: TugType,
        dcrt_params: Rc<Params>,
        format: Format,
        h: u32,
    ) -> Self {
        let rdim = dcrt_params.get_ring_dimension();
        let tug_values = tug.generate_int_vector(rdim, h);
        let mut m_vectors = Vec::with_capacity(dcrt_params.get_params().len());
        for p in dcrt_params.get_params() {
            let mut iltvs = NativeVector::new(rdim, p.get_modulus());
            for j in 0..rdim {
                let k = tug_values[j];
                let iltvs_j = if k < 0 {
                    k *= -1;
                    p.get_modulus().convert_to_int() - k as NativeInteger::Integer
                } else {
                    k as NativeInteger::Integer
                };
                iltvs[j] = iltvs_j;
            }
            let mut ilvector = PolyType::new(p);
            ilvector.set_values(iltvs, Format::COEFFICIENT);
            ilvector.set_format(format);
            m_vectors.push(ilvector);
        }
        DCRTPolyImpl {
            m_params: Some(dcrt_params),
            m_format: format,
            m_vectors,
        }
    }

    pub fn clone_with_noise(
        &self,
        dgg: DiscreteGaussianGeneratorImpl<VecType>,
        format: Format,
    ) -> Self {
        let res = DCRTPolyImpl {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors: Vec::new(),
        };
        let c = self.m_params.get_cyclotomic_order();
        let m = self.m_params.get_modulus();
        let parm = Rc::new(ILParamsImpl::new(c, m, 1));
        let mut element = PolyLargeType::new(parm);
        element.set_values(dgg.generate_vector(c / 2, m), m_format);
        res = element;
        res
    }

    pub fn clone_towers(&self, start_tower: u32, end_tower: u32) -> Self {
        let mut m = Vec::new();
        let mut r = Vec::new();
        for i in start_tower..=end_tower {
            m.push(self.m_params.get_params()[i].get_modulus());
            r.push(self.m_params.get_params()[i].get_root_of_unity());
        }
        let co = self.m_params.get_cyclotomic_order();
        let params = Rc::new(Params::new(co, m, r));
        let mut res = DCRTPolyImpl {
            m_params: Some(params),
            m_format: Format::EVALUATION,
            m_vectors: Vec::new(),
        };
        for i in start_tower..=end_tower {
            res.set_element_at_index(i - start_tower, self.get_element_at_index(i));
        }
        res
    }

    pub fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self> {
        let bdv = CRTInterpolate().base_decompose(base_bits, false);
        let mut result = Vec::with_capacity(bdv.len());
        for dv in bdv {
            let mut dv = DCRTPolyImpl {
                m_params: self.m_params.clone(),
                m_format: self.m_format,
                m_vectors: Vec::new(),
            };
            dv.set_params(self.m_params.clone());
            if eval_mode_answer {
                dv.switch_format();
            }
            result.push(dv);
        }
        result
    }

        pub fn crt_decompose(&self, base_bits: u32) -> Vec<DCRTPolyImpl<VecType>> {
        let mut cp = DCRTPolyImpl {
            m_format: self.m_format,
            m_params: self.m_params.clone(),
            m_vectors: self.m_vectors.clone(),
        };
        cp.switch_format();
        let coef = if self.m_format == Format::COEFFICIENT {
            self
        } else {
            &cp
        };
        let eval = if self.m_format == Format::COEFFICIENT {
            &cp
        } else {
            self
        };
        let size = self.m_vectors.len();
        if base_bits == 0 {
            let mut result = vec![eval.clone(); size];
            for i in 0..size {
                for k in 0..size {
                    if i != k {
                        let mut tmp = DCRTPolyImpl::PolyType::new(coef.m_vectors[i].clone());
                        tmp.switch_modulus(
                            coef.m_vectors[k].get_modulus(),
                            coef.m_vectors[k].get_root_of_unity(),
                            0,
                            0,
                        );
                        tmp.set_format(Format::EVALUATION);
                        result[i].m_vectors[k] = tmp;
                    }
                }
            }
            return result;
        }
        let mut n_windows = 0;
        let mut arr_windows = vec![0; size];
        for i in 0..size {
            let n_bits = self.m_vectors[i].get_modulus().get_length_for_base(2);
            let mut cur_windows = n_bits / base_bits;
            if n_bits % base_bits != 0 {
                cur_windows += 1;
            }
            arr_windows[i] = n_windows;
            n_windows += cur_windows;
        }
        let mut result = vec![DCRTPolyType::new(); n_windows];
        for i in 0..size {
            let decomposed = coef.m_vectors[i].base_decompose(base_bits, false);
            for j in 0..decomposed.len() {
                let mut current_dcrtpoly = DCRTPolyImpl {
                    m_params: coef.m_params.clone(),
                    m_format: coef.m_format,
                    m_vectors: coef.m_vectors.clone(),
                };
                for k in 0..size {
                    let mut tmp = decomposed[j].clone();
                    if i != k {
                        tmp.switch_modulus(
                            coef.m_vectors[k].get_modulus(),
                            coef.m_vectors[k].get_root_of_unity(),
                            0,
                            0,
                        );
                    }
                    current_dcrtpoly.m_vectors[k] = tmp;
                }
                current_dcrtpoly.switch_format();
                result[j + arr_windows[i]] = current_dcrtpoly;
            }
        }
        result
    }

    pub fn powers_of_base(&self, base_bits: u32) -> Vec<DCRTPolyImpl<VecType>> {
        let mut mods = Vec::with_capacity(self.m_params.get_params().len());
        for p in self.m_params.get_params() {
            mods.push(p.get_modulus());
        }
        let n_bits = self.m_params.get_modulus().get_length_for_base(2);
        let mut n_windows = n_bits / base_bits;
        if n_bits % base_bits != 0 {
            n_windows += 1;
        }
        let mut result = Vec::with_capacity(n_windows);
        let mut two_pow = Integer::from(1);
        let size = self.m_vectors.len();
        for _ in 0..n_windows {
            let mut x = DCRTPolyImpl {
                m_params: self.m_params.clone(),
                m_format: self.m_format,
                m_vectors: self.m_vectors.clone(),
            };
            two_pow.lshift_eq(base_bits);
            for t in 0..size {
                x.m_vectors[t] = self.m_vectors[t] * two_pow.mod_op(mods[t]).convert_to_int();
            }
            result.push(x);
        }
        result
    }

    pub fn automorphism_transform(&self, i: u32) -> DCRTPolyImpl<VecType> {
        let mut result = DCRTPolyImpl {
            m_format: self.m_format,
            m_params: self.m_params.clone(),
            m_vectors: Vec::with_capacity(self.m_vectors.len()),
        };
        for v in &self.m_vectors {
            result.m_vectors.push(v.automorphism_transform(i));
        }
        result
    }

    pub fn automorphism_transform_with_vec(
        &self,
        i: u32,
        vec: &Vec<u32>,
    ) -> DCRTPolyImpl<VecType> {
        let mut result = DCRTPolyImpl {
            m_format: self.m_format,
            m_params: self.m_params.clone(),
            m_vectors: Vec::with_capacity(self.m_vectors.len()),
        };
        for v in &self.m_vectors {
            result.m_vectors.push(v.automorphism_transform_with_vec(i, vec));
        }
        result
    }

    pub fn multiplicative_inverse(&self) -> DCRTPolyImpl<VecType> {
        let mut tmp = DCRTPolyImpl {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors: Vec::with_capacity(self.m_vectors.len()),
        };
        for v in &self.m_vectors {
            tmp.m_vectors.push(v.multiplicative_inverse());
        }
        tmp
    }

    pub fn negate(&self) -> DCRTPolyImpl<VecType> {
        let mut tmp = DCRTPolyImpl {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors: Vec::with_capacity(self.m_vectors.len()),
        };
        for v in &self.m_vectors {
            tmp.m_vectors.push(v.negate());
        }
        tmp
    }

    pub fn minus(&self, rhs: &DCRTPolyImpl<VecType>) -> DCRTPolyImpl<VecType> {
        if self.m_vectors.len() != rhs.m_vectors.len() {
            panic!("tower size mismatch; cannot subtract");
        }
        let mut tmp = DCRTPolyImpl {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors: Vec::with_capacity(self.m_vectors.len()),
        };
        for i in 0..self.m_vectors.len() {
            tmp.m_vectors.push(self.m_vectors[i].minus(&rhs.m_vectors[i]));
        }
        tmp
    }

    pub fn add_assign(&mut self, rhs: &DCRTPolyImpl<VecType>) -> &mut Self {
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] += &rhs.m_vectors[i];
        }
        self
    }

    pub fn add_assign_integer(&mut self, rhs: &Integer) -> &mut Self {
        let val = NativeInteger::from(rhs);
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] += &val;
        }
        self
    }

    pub fn add_assign_native_integer(&mut self, rhs: &NativeInteger) -> &mut Self {
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] += rhs;
        }
        self
    }

    pub fn sub_assign(&mut self, rhs: &DCRTPolyImpl<VecType>) -> &mut Self {
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] -= &rhs.m_vectors[i];
        }
        self
    }

    pub fn sub_assign_integer(&mut self, rhs: &Integer) -> &mut Self {
        let val = NativeInteger::from(rhs);
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] -= &val;
        }
        self
    }

    pub fn sub_assign_native_integer(&mut self, rhs: &NativeInteger) -> &mut Self {
        for i in 0..self.m_vectors.len() {
            self.m_vectors[i] -= rhs;
        }
        self
    }


    fn set_values_mod_switch(&mut self, element: &DcrtpolyImpl<VecType>, modulus: &NativeInteger) {
        if element.get_num_of_elements() != 1 {
            panic!("SetValuesModSwitch is implemented only for a DCRTPoly with one tower.");
        }
        let q = element.get_modulus();
        let qmod_double = modulus.convert_to_double() / q.convert_to_double();
        self.params.set_original_modulus(modulus.clone());
        let input = element.get_element_at_index(0);
        let input = input.set_format(Format::Coefficient);
        let size = self.vectors.len();
        let n_elem = element.params.get_ring_dimension();
        let n = self.get_ring_dimension();
        if n_elem > n {
            panic!("The ring dimension of the element to copy is larger than the ring dimension of the element to copy to.");
        }
        for i in 0..size {
            let mut tmp = NativeVector::new_with_modulus(n, modulus.clone());
            for j in 0..n_elem {
                tmp[j] = Integer::from((0.5 + input[j].convert_to_double() * qmod_double).floor() as u64).modulus(modulus);
            }
            self.vectors[i].set_values(tmp, Format::Coefficient);
        }
    }

    fn add_il_element_one(&mut self) {
        if self.format != Format::Evaluation {
            panic!("Cannot call AddILElementOne() on DCRTPoly in COEFFICIENT format.");
        }
        let size = self.vectors.len();
        self.vectors.par_iter_mut().for_each(|vector| {
            vector.add_il_element_one();
        });
    }

    fn is_empty(&self) -> bool {
        for v in &self.vectors {
            if !v.is_empty() {
                return false;
            }
        }
        true
    }

    fn drop_last_element(&mut self) {
        if self.vectors.is_empty() {
            panic!("Input has no elements to drop!");
        }
        if self.vectors.len() == 1 {
            panic!("Removing last element of DCRTPoly object renders it invalid!");
        }
        self.vectors.pop();
        let mut new_params = self.params.clone();
        new_params.pop_last_param();
        self.params = Arc::new(new_params);
    }

    fn drop_last_elements(&mut self, i: usize) {
        if self.vectors.len() <= i {
            panic!("There are not enough towers in the current ciphertext to perform the modulus reduction");
        }
        self.vectors.resize(self.vectors.len() - i, Default::default());
        let mut new_params = self.params.clone();
        for _ in 0..i {
            new_params.pop_last_param();
        }
        self.params = Arc::new(new_params);
    }

    fn drop_last_element_and_scale(&mut self, ql_ql_inv_modql_divql_modq: Vec<NativeInteger>, ql_ql_inv_modql_divql_modq_precon: Vec<NativeInteger>, ql_inv_modq: Vec<NativeInteger>, ql_inv_modq_precon: Vec<NativeInteger>) {
        let last_poly = self.vectors.pop().expect("No elements to pop");
        let last_poly = last_poly.set_format(Format::Coefficient);
        let size = self.vectors.len();
        self.vectors.par_iter_mut().enumerate().for_each(|(i, vector)| {
            let mut tmp = last_poly.clone();
            tmp.switch_modulus(&vector.get_modulus(), &vector.get_root_of_unity(), &0, &0);
            tmp *= &ql_ql_inv_modql_divql_modq[i];
            if self.format == Format::Evaluation {
                tmp.switch_format();
            }
            *vector *= &ql_inv_modq[i];
            *vector += &tmp;
            if self.format == Format::Coefficient {
                vector.switch_format();
            }
        });
    }

   pub fn mod_reduce(
        &mut self,
        t: &NativeInteger,
        t_modq_precon: &[NativeInteger],
        negt_inv_modq: &NativeInteger,
        negt_inv_modq_precon: &NativeInteger,
        ql_inv_modq: &[NativeInteger],
        ql_inv_modq_precon: &[NativeInteger],
    ) {
        let mut delta = self.m_vectors.last().unwrap().clone();
        delta.set_format(Format::COEFFICIENT);
        delta *= negt_inv_modq;
        self.m_vectors.pop();
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .with_num_threads(OpenFHEParallelControls.get_thread_limit(size))
            .for_each(|m_vector| {
                let mut tmp = delta.clone();
                tmp.switch_modulus(m_vector.get_modulus(), m_vector.get_root_of_unity(), 0, 0);
                if self.m_format == Format::EVALUATION {
                    tmp.switch_format();
                }
                *m_vector += (tmp *= t);
                *m_vector *= ql_inv_modq[i];
            });
    }

    pub fn at(&self, i: usize) -> &VecType::Integer {
        if self.m_vectors.is_empty() {
            panic!("No values in DCRTPolyImpl");
        }
        if i >= self.m_vectors.len() {
            panic!("out of range in DCRTPolyImpl.at()");
        }
        &self.crt_interpolate_index(i)[i]
    }

    pub fn at_mut(&mut self, i: usize) -> &mut VecType::Integer {
        if self.m_vectors.is_empty() {
            panic!("No values in DCRTPolyImpl");
        }
        if i >= self.m_vectors.len() {
            panic!("out of range in DCRTPolyImpl.at()");
        }
        &mut self.crt_interpolate_index(i)[i]
    }

    pub fn crt_interpolate(&self) -> PolyLargeType {
        let ring_dimension = self.m_params.get_ring_dimension();
        let n_towers = self.m_vectors.len();
        let big_modulus = self.m_params.get_modulus();

        let mut coefficients = VecType::new(ring_dimension, big_modulus);

        let mut multiplier = Vec::with_capacity(n_towers);
        for vi in 0..n_towers {
            let qj = self.m_vectors[vi].get_modulus().convert_to_int();
            let div_by = big_modulus / qj;
            let mod_inv = div_by.mod_inverse(qj).modulus(qj);
            multiplier.push(div_by * mod_inv);
        }

        let vecs = if self.m_format == Format::EVALUATION {
            let mut coeff_vecs = Vec::with_capacity(self.m_vectors.len());
            for i in 0..self.m_vectors.len() {
                let mut vec_copy = self.m_vectors[i].clone();
                vec_copy.set_format(Format::COEFFICIENT);
                coeff_vecs.push(vec_copy);
            }
            &coeff_vecs
        } else {
            &self.m_vectors
        };

        let mu = big_modulus.compute_mu();

        for ri in 0..ring_dimension {
            coefficients[ri] = 0;
            for vi in 0..n_towers {
                coefficients[ri] +=
                    Integer::from(vecs[vi].get_values()[ri].convert_to_int()) * multiplier[vi];
            }
            coefficients[ri].mod_eq(big_modulus, mu);
        }

        let polynomial_reconstructed = PolyLargeType::new(
            Arc::new(ILParamsImpl::new(
                self.m_params.get_cyclotomic_order(),
                big_modulus,
                1,
            )),
        );
        polynomial_reconstructed.set_values(coefficients, Format::COEFFICIENT);
        polynomial_reconstructed
    }

    pub fn crt_interpolate_index(&self, i: usize) -> PolyLargeType {
        let ring_dimension = self.m_params.get_ring_dimension();
        let n_towers = self.m_vectors.len();
        let big_modulus = self.m_params.get_modulus();
        let mut coefficients = VecType::new(ring_dimension, big_modulus);
        let mut multiplier = Vec::with_capacity(n_towers);
        for vi in 0..n_towers {
            let qj = self.m_vectors[vi].get_modulus().convert_to_int();
            let div_by = big_modulus / qj;
            let mod_inv = div_by.mod_inverse(qj).modulus(qj);
            multiplier.push(div_by * mod_inv);
        }

        let vecs = if self.m_format == Format::EVALUATION {
            let mut coeff_vecs = Vec::with_capacity(self.m_vectors.len());
            for ii in 0..self.m_vectors.len() {
                let mut vec_copy = self.m_vectors[ii].clone();
                vec_copy.set_format(Format::COEFFICIENT);
                coeff_vecs.push(vec_copy);
            }
            &coeff_vecs
        } else {
            &self.m_vectors
        };

        let mu = big_modulus.compute_mu();

        for ri in 0..ring_dimension {
            coefficients[ri] = 0;
            if ri == i {
                for vi in 0..n_towers {
                    coefficients[ri] +=
                        Integer::from(vecs[vi].get_values()[ri].convert_to_int()) * multiplier[vi];
                }
                coefficients[ri].mod_eq(big_modulus, mu);
            }
        }
        let polynomial_reconstructed = PolyLargeType::new(
            Arc::new(ILParamsImpl::new(
                self.m_params.get_cyclotomic_order(),
                big_modulus,
                1,
            )),
        );
        polynomial_reconstructed.set_values(coefficients, Format::COEFFICIENT);
        polynomial_reconstructed
    }

    pub fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> PolyType {
        self.crt_interpolate().decryption_crt_interpolate(ptm)
    }

    pub fn to_native_poly(&self) -> PolyType {
        self.crt_interpolate().to_native_poly()
    }

    pub fn get_working_modulus(&self) -> VecType::Integer {
        let mut modulus_q = 1;
        for p in self.m_params.get_params() {
            modulus_q.mul_eq(p.get_modulus());
        }
        modulus_q
    }

    pub fn get_extended_crt_basis(
        &self,
        params_p: Arc<Params>,
    ) -> Arc<Params> {
        let size_q = self.m_vectors.len();
        let size_p = params_p.get_params().len();
        let size_qp = size_q + size_p;
        let mut moduli_qp = Vec::with_capacity(size_qp);
        let mut roots_qp = Vec::with_capacity(size_qp);
        for i in 0..size_q {
            moduli_qp.push(self.m_params.get_params()[i].get_modulus());
            roots_qp.push(self.m_params.get_params()[i].get_root_of_unity());
        }
        for i in size_q..size_qp {
            moduli_qp.push(params_p.get_params()[i - size_q].get_modulus());
            roots_qp.push(params_p.get_params()[i - size_q].get_root_of_unity());
        }
        Arc::new(Params::new(2 * self.m_params.get_ring_dimension(), moduli_qp, roots_qp))
    }

    pub fn times_qovert(
        &mut self,
        params_q: Arc<Params>,
        t_inv_modq: &[NativeInteger],
        t: &NativeInteger,
        neg_q_modt: &NativeInteger,
        neg_q_modt_precon: &NativeInteger,
    ) {
        if t_inv_modq.len() < self.m_vectors.len() {
            panic!("Sizes of vectors do not match.");
        }
        let size = self.m_vectors.len();
        let ring_dim = self.m_params.get_ring_dimension();
        self.m_vectors
            .par_iter_mut()
            .with_num_threads(OpenFHEParallelControls.get_thread_limit(size))
            .for_each(|m_vector| {
                for ri in 0..ring_dim {
                    let xi = &mut m_vector[ri];
                    xi.mod_mul_fast_const_eq(neg_q_modt, t, neg_q_modt_precon);
                }
                *m_vector = m_vector.times(t_inv_modq[i]);
            });
    }

    pub fn approx_switch_crt_basis(
        &self,
        params_q: Arc<Params>,
        params_p: Arc<Params>,
        q_hat_inv_modq: &[NativeInteger],
        q_hat_inv_modq_precon: &[NativeInteger],
        q_hat_modp: &[Vec<NativeInteger>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) -> DCRTPolyImpl<VecType> {
        let mut ans = DCRTPolyImpl::new(params_p, self.m_format, true);
        let ring_dim = self.m_params.get_ring_dimension();
        let size_q = if self.m_vectors.len() > params_q.get_params().len() {
            params_q.get_params().len()
        } else {
            self.m_vectors.len()
        };
        let size_p = ans.m_vectors.len();
        for ri in 0..ring_dim {
            let mut sum = vec![0; size_p];
            for i in 0..size_q {
                let xi = &self.m_vectors[i][ri];
                let qi = self.m_vectors[i].get_modulus();
                let x_q_hat_inv_modqi = xi.mod_mul_fast_const(q_hat_inv_modq[i], qi, q_hat_inv_modq_precon[i]);
                for j in 0..size_p {
                    sum[j] += x_q_hat_inv_modqi.convert_to_int() * q_hat_modp[i][j].convert_to_int();
                }
            }
            for j in 0..size_p {
                let pj = ans.m_vectors[j].get_modulus();
                ans.m_vectors[j][ri] = barrett_uint128_mod_uint64(sum[j], pj.convert_to_int(), modp_barrett_mu[j]);
            }
        }
        ans
    }
}

impl<T: PartialEq> PartialEq for DCRTPolyImpl<T> {
    fn eq(&self, rhs: &Self) -> bool {
        (self.m_format == rhs.m_format) && (self.m_params.GetCyclotomicOrder() == rhs.m_params.GetCyclotomicOrder()) &&
        (self.m_params.GetModulus() == rhs.m_params.GetModulus()) && (self.m_vectors.len() == rhs.m_vectors.len()) &&
        (self.m_vectors == rhs.m_vectors)
    }
}

impl<T> DCRTPolyImpl<T> {
    fn set_values_to_zero(&mut self) {
        for v in &mut self.m_vectors {
            v.set_values_to_zero();
        }
    }
}

impl<T> Add<Integer> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn add(self, rhs: Integer) -> Self::Output {
        let val = NativeInteger::from(rhs);
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.plus(&val));
        tmp
    }
}

impl<T> Add<Vec<Integer>> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn add(self, rhs: Vec<Integer>) -> Self::Output {
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.plus(&NativeInteger::from(rhs[i])));
        tmp
    }
}

impl<T> Sub<Integer> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn sub(self, rhs: Integer) -> Self::Output {
        let val = NativeInteger::from(rhs);
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.minus(&val));
        tmp
    }
}

impl<T> Sub<Vec<Integer>> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn sub(self, rhs: Vec<Integer>) -> Self::Output {
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.minus(&NativeInteger::from(rhs[i])));
        tmp
    }
}

impl<T> Mul<Integer> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn mul(self, rhs: Integer) -> Self::Output {
        let val = NativeInteger::from(rhs);
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.times(&val));
        tmp
    }
}

impl<T> Mul<NativeInteger::SignedNativeInt> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn mul(self, rhs: NativeInteger::SignedNativeInt) -> Self::Output {
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.times(rhs));
        tmp
    }
}

impl<T> Mul<Vec<Integer>> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn mul(self, rhs: Vec<Integer>) -> Self::Output {
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.times(&NativeInteger::from(rhs[i])));
        tmp
    }
}

impl<T> Mul<Vec<NativeInteger>> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn mul(self, rhs: Vec<NativeInteger>) -> Self::Output {
        if self.m_vectors.len() != rhs.len() {
            panic!("tower size mismatch; cannot multiply");
        }
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| tmp.m_vectors[i] = v.times(rhs[i]));
        tmp
    }
}

impl<T> Mul<Vec<NativeInteger>> for DCRTPolyImpl<T> {
    type Output = DCRTPolyImpl<T>;

    fn mul(self, rhs: Vec<NativeInteger>) -> Self::Output {
        let vec_size = self.m_vectors.len().min(rhs.len());
        let mut tmp = DCRTPolyImpl::new(self.m_params, self.m_format);
        self.m_vectors
            .par_iter()
            .enumerate()
            .take(vec_size)
            .for_each(|(i, v)| tmp.m_vectors[i] = v.times(rhs[i]));
        tmp
    }
}

impl<T> MulAssign<Integer> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: Integer) {
        let val = NativeInteger::from(rhs);
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= val);
    }
}

impl<T> MulAssign<NativeInteger> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: NativeInteger) {
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= rhs);
    }
}

impl<T> MulAssign<Vec<Integer>> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: Vec<Integer>) {
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= rhs);
        self.m_format = Format::COEFFICIENT;
    }
}

impl<T> MulAssign<Vec<Integer>> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: Vec<Integer>) {
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= rhs);
        self.m_format = Format::COEFFICIENT;
    }
}

impl<T> MulAssign<Vec<Integer>> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: Vec<Integer>) {
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= rhs);
        self.m_format = Format::COEFFICIENT;
    }
}

impl<T> MulAssign<Vec<Integer>> for DCRTPolyImpl<T> {
    fn mul_assign(&mut self, rhs: Vec<Integer>) {
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v *= rhs);
        self.m_format = Format::COEFFICIENT;
    }
}













use std::sync::Arc;

pub struct DCRTPolyImpl<VecType> {
    m_vectors: Vec<VecType>,
    m_params: Arc<Params>,
    m_format: Format,
}

impl<VecType> DCRTPolyImpl<VecType> {
    pub fn mod_reduce(
        &mut self,
        t: &NativeInteger,
        t_modq_precon: &[NativeInteger],
        negt_inv_modq: &NativeInteger,
        negt_inv_modq_precon: &NativeInteger,
        ql_inv_modq: &[NativeInteger],
        ql_inv_modq_precon: &[NativeInteger],
    ) {
        let mut delta = self.m_vectors.last().unwrap().clone();
        delta.set_format(Format::COEFFICIENT);
        delta *= negt_inv_modq;
        self.m_vectors.pop();
        let size = self.m_vectors.len();
        self.m_vectors
            .par_iter_mut()
            .with_num_threads(OpenFHEParallelControls.get_thread_limit(size))
            .for_each(|m_vector| {
                let mut tmp = delta.clone();
                tmp.switch_modulus(m_vector.get_modulus(), m_vector.get_root_of_unity(), 0, 0);
                if self.m_format == Format::EVALUATION {
                    tmp.switch_format();
                }
                *m_vector += (tmp *= t);
                *m_vector *= ql_inv_modq[i];
            });
    }

    pub fn at(&self, i: usize) -> &VecType::Integer {
        if self.m_vectors.is_empty() {
            panic!("No values in DCRTPolyImpl");
        }
        if i >= self.m_vectors.len() {
            panic!("out of range in DCRTPolyImpl.at()");
        }
        &self.crt_interpolate_index(i)[i]
    }

    pub fn at_mut(&mut self, i: usize) -> &mut VecType::Integer {
        if self.m_vectors.is_empty() {
            panic!("No values in DCRTPolyImpl");
        }
        if i >= self.m_vectors.len() {
            panic!("out of range in DCRTPolyImpl.at()");
        }
        &mut self.crt_interpolate_index(i)[i]
    }

    pub fn crt_interpolate(&self) -> PolyLargeType {
        let ring_dimension = self.m_params.get_ring_dimension();
        let n_towers = self.m_vectors.len();
        let big_modulus = self.m_params.get_modulus();

        let mut coefficients = VecType::new(ring_dimension, big_modulus);

        let mut multiplier = Vec::with_capacity(n_towers);
        for vi in 0..n_towers {
            let qj = self.m_vectors[vi].get_modulus().convert_to_int();
            let div_by = big_modulus / qj;
            let mod_inv = div_by.mod_inverse(qj).modulus(qj);
            multiplier.push(div_by * mod_inv);
        }

        let vecs = if self.m_format == Format::EVALUATION {
            let mut coeff_vecs = Vec::with_capacity(self.m_vectors.len());
            for i in 0..self.m_vectors.len() {
                let mut vec_copy = self.m_vectors[i].clone();
                vec_copy.set_format(Format::COEFFICIENT);
                coeff_vecs.push(vec_copy);
            }
            &coeff_vecs
        } else {
            &self.m_vectors
        };

        let mu = big_modulus.compute_mu();

        for ri in 0..ring_dimension {
            coefficients[ri] = 0;
            for vi in 0..n_towers {
                coefficients[ri] +=
                    Integer::from(vecs[vi].get_values()[ri].convert_to_int()) * multiplier[vi];
            }
            coefficients[ri].mod_eq(big_modulus, mu);
        }

        let polynomial_reconstructed = PolyLargeType::new(
            Arc::new(ILParamsImpl::new(
                self.m_params.get_cyclotomic_order(),
                big_modulus,
                1,
            )),
        );
        polynomial_reconstructed.set_values(coefficients, Format::COEFFICIENT);
        polynomial_reconstructed
    }

    pub fn crt_interpolate_index(&self, i: usize) -> PolyLargeType {
        let ring_dimension = self.m_params.get_ring_dimension();
        let n_towers = self.m_vectors.len();
        let big_modulus = self.m_params.get_modulus();
        let mut coefficients = VecType::new(ring_dimension, big_modulus);
        let mut multiplier = Vec::with_capacity(n_towers);
        for vi in 0..n_towers {
            let qj = self.m_vectors[vi].get_modulus().convert_to_int();
            let div_by = big_modulus / qj;
            let mod_inv = div_by.mod_inverse(qj).modulus(qj);
            multiplier.push(div_by * mod_inv);
        }

        let vecs = if self.m_format == Format::EVALUATION {
            let mut coeff_vecs = Vec::with_capacity(self.m_vectors.len());
            for ii in 0..self.m_vectors.len() {
                let mut vec_copy = self.m_vectors[ii].clone();
                vec_copy.set_format(Format::COEFFICIENT);
                coeff_vecs.push(vec_copy);
            }
            &coeff_vecs
        } else {
            &self.m_vectors
        };

        let mu = big_modulus.compute_mu();

        for ri in 0..ring_dimension {
            coefficients[ri] = 0;
            if ri == i {
                for vi in 0..n_towers {
                    coefficients[ri] +=
                        Integer::from(vecs[vi].get_values()[ri].convert_to_int()) * multiplier[vi];
                }
                coefficients[ri].mod_eq(big_modulus, mu);
            }
        }
        let polynomial_reconstructed = PolyLargeType::new(
            Arc::new(ILParamsImpl::new(
                self.m_params.get_cyclotomic_order(),
                big_modulus,
                1,
            )),
        );
        polynomial_reconstructed.set_values(coefficients, Format::COEFFICIENT);
        polynomial_reconstructed
    }

    pub fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> PolyType {
        self.crt_interpolate().decryption_crt_interpolate(ptm)
    }

    pub fn to_native_poly(&self) -> PolyType {
        self.crt_interpolate().to_native_poly()
    }

    pub fn get_working_modulus(&self) -> VecType::Integer {
        let mut modulus_q = 1;
        for p in self.m_params.get_params() {
            modulus_q.mul_eq(p.get_modulus());
        }
        modulus_q
    }

    pub fn get_extended_crt_basis(
        &self,
        params_p: Arc<Params>,
    ) -> Arc<Params> {
        let size_q = self.m_vectors.len();
        let size_p = params_p.get_params().len();
        let size_qp = size_q + size_p;
        let mut moduli_qp = Vec::with_capacity(size_qp);
        let mut roots_qp = Vec::with_capacity(size_qp);
        for i in 0..size_q {
            moduli_qp.push(self.m_params.get_params()[i].get_modulus());
            roots_qp.push(self.m_params.get_params()[i].get_root_of_unity());
        }
        for i in size_q..size_qp {
            moduli_qp.push(params_p.get_params()[i - size_q].get_modulus());
            roots_qp.push(params_p.get_params()[i - size_q].get_root_of_unity());
        }
        Arc::new(Params::new(2 * self.m_params.get_ring_dimension(), moduli_qp, roots_qp))
    }

    pub fn times_qovert(
        &mut self,
        params_q: Arc<Params>,
        t_inv_modq: &[NativeInteger],
        t: &NativeInteger,
        neg_q_modt: &NativeInteger,
        neg_q_modt_precon: &NativeInteger,
    ) {
        if t_inv_modq.len() < self.m_vectors.len() {
            panic!("Sizes of vectors do not match.");
        }
        let size = self.m_vectors.len();
        let ring_dim = self.m_params.get_ring_dimension();
        self.m_vectors
            .par_iter_mut()
            .with_num_threads(OpenFHEParallelControls.get_thread_limit(size))
            .for_each(|m_vector| {
                for ri in 0..ring_dim {
                    let xi = &mut m_vector[ri];
                    xi.mod_mul_fast_const_eq(neg_q_modt, t, neg_q_modt_precon);
                }
                *m_vector = m_vector.times(t_inv_modq[i]);
            });
    }

    pub fn approx_switch_crt_basis(
        &self,
        params_q: Arc<Params>,
        params_p: Arc<Params>,
        q_hat_inv_modq: &[NativeInteger],
        q_hat_inv_modq_precon: &[NativeInteger],
        q_hat_modp: &[Vec<NativeInteger>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) -> DCRTPolyImpl<VecType> {
        let mut ans = DCRTPolyImpl::new(params_p, self.m_format, true);
        let ring_dim = self.m_params.get_ring_dimension();
        let size_q = if self.m_vectors.len() > params_q.get_params().len() {
            params_q.get_params().len()
        } else {
            self.m_vectors.len()
        };
        let size_p = ans.m_vectors.len();
        for ri in 0..ring_dim {
            let mut sum = vec![0; size_p];
            for i in 0..size_q {
                let xi = &self.m_vectors[i][ri];
                let qi = self.m_vectors[i].get_modulus();
                let x_q_hat_inv_modqi = xi.mod_mul_fast_const(q_hat_inv_modq[i], qi, q_hat_inv_modq_precon[i]);
                for j in 0..size_p {
                    sum[j] += x_q_hat_inv_modqi.convert_to_int() * q_hat_modp[i][j].convert_to_int();
                }
            }
            for j in 0..size_p {
                let pj = ans.m_vectors[j].get_modulus();
                ans.m_vectors[j][ri] = barrett_uint128_mod_uint64(sum[j], pj.convert_to_int(), modp_barrett_mu[j]);
            }
        }
        ans
    }
}

pub enum Format {
    COEFFICIENT,
    EVALUATION,
}

pub struct PolyLargeType {
    values: Vec<Integer>,
    format: Format,
}

impl PolyLargeType {
    pub fn new(params: Arc<ILParamsImpl<Integer>>) -> Self {
        Self {
            values: vec![],
            format: Format::COEFFICIENT,
        }
    }

    pub fn set_values(&mut self, values: Vec<Integer>, format: Format) {
        self.values = values;
        self.format = format;
    }
}

pub struct PolyType {
    values: Vec<Integer>,
    format: Format,
}

impl PolyType {
    pub fn new(params: Arc<ILParamsImpl<Integer>>) -> Self {
        Self {
            values: vec![],
            format: Format::COEFFICIENT,
        }
    }

    pub fn set_values(&mut self, values: Vec<Integer>, format: Format) {
        self.values = values;
        self.format = format;
    }

    pub fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> Self {
        unimplemented!()
    }

    pub fn to_native_poly(&self) -> Self {
        unimplemented!()
    }
}

pub struct ILParamsImpl<Integer> {
    cyclotomic_order: usize,
    modulus: Integer,
    _phantom: std::marker::PhantomData<Integer>,
}

impl<Integer> ILParamsImpl<Integer> {
    pub fn new(cyclotomic_order: usize, modulus: Integer, _: usize) -> Self {
        Self {
            cyclotomic_order,
            modulus,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_cyclotomic_order(&self) -> usize {
        self.cyclotomic_order
    }

    pub fn get_modulus(&self) -> &Integer {
        &self.modulus
    }
}

pub struct NativeInteger;

impl NativeInteger {
    pub fn convert_to_int(&self) -> i64 {
        unimplemented!()
    }

    pub fn mod_inverse(&self, _: NativeInteger) -> NativeInteger {
        unimplemented!()
    }

    pub fn modulus(&self, _: NativeInteger) -> NativeInteger {
        unimplemented!()
    }

    pub fn mod_eq(&mut self, _: NativeInteger, _: NativeInteger) {
        unimplemented!()
    }

    pub fn mod_mul_fast_const_eq(&mut self, _: NativeInteger, _: NativeInteger, _: NativeInteger) {
        unimplemented!()
    }
}

pub struct DoubleNativeInt;

fn barrett_uint128_mod_uint64(_: i64, _: i64, _: DoubleNativeInt) -> i64 {
    unimplemented!()
}

pub struct Integer;

impl Integer {
    pub fn from(_: i64) -> Self {
        unimplemented!()
    }

    pub fn mod_eq(&mut self, _: NativeInteger, _: NativeInteger) {
        unimplemented!()
    }

    pub fn modulus(&self, _: NativeInteger) -> NativeInteger {
        unimplemented!()
    }

    pub fn mul_eq(&mut self, _: NativeInteger) {
        unimplemented!()
    }
}

pub struct PlaintextModulus;

impl DCRTPolyImpl<VecType> {
    pub fn new(params: Arc<Params>, format: Format, _: bool) -> Self {
        Self {
            m_vectors: vec![],
            m_params: params,
            m_format: format,
        }
    }
}

pub struct VecType {
    modulus: NativeInteger,
    root_of_unity: NativeInteger,
}

impl VecType {
    pub fn new(_: usize, modulus: NativeInteger) -> Self {
        Self {
            modulus,
            root_of_unity: NativeInteger,
        }
    }

    pub fn get_modulus(&self) -> &NativeInteger {
        &self.modulus
    }

    pub fn get_root_of_unity(&self) -> &NativeInteger {
        &self.root_of_unity
    }

    pub fn set_format(&mut self, _: Format) {
        unimplemented!()
    }

    pub fn times(&self, _: NativeInteger) -> Self {
        unimplemented!()
    }
}

pub fn mul128(_: i64, _: i64) -> i64 {
    unimplemented!()
}

impl PolyType {
    pub fn switch_modulus(&mut self, _: NativeInteger, _: NativeInteger, _: usize, _: usize) {
        unimplemented!()
    }

    pub fn switch_format(&mut self) {
        unimplemented!()
    }
}

impl PolyLargeType {
    pub fn switch_modulus(&mut self, _: NativeInteger, _: NativeInteger, _: usize, _: usize) {
        unimplemented!()
    }

    pub fn switch_format(&mut self) {
        unimplemented!()
    }
}

pub struct OpenFHEParallelControls;

impl OpenFHEParallelControls {
    pub fn get_thread_limit(&self, _: usize) -> usize {
        unimplemented!()
    }
}

fn approx_mod_up(
    params_q: Rc<Params>,
    params_p: Rc<Params>,
    params_qp: Rc<Params>,
    q_hat_inv_modq: &Vec<NativeInteger>,
    q_hat_inv_modq_precon: &Vec<NativeInteger>,
    q_hat_modp: &Vec<Vec<NativeInteger>>,
    modp_barrett_mu: &Vec<DoubleNativeInt>,
) {
    let mut poly_in_ntt: Vec<DCRTPolyImpl::PolyType> = Vec::new();

    if m_format == Format::EVALUATION {
        poly_in_ntt = m_vectors;
        self.set_format(Format::COEFFICIENT);
    }

    let size_q = m_vectors.len();
    let size_p = params_p.get_params().len();
    let size_qp = params_qp.get_params().len();
    let part_p = approx_switch_crt_basis(
        params_q,
        params_p,
        q_hat_inv_modq,
        q_hat_inv_modq_precon,
        q_hat_modp,
        modp_barrett_mu,
    );
    m_vectors.resize(size_qp);

    for j in 0..size_p {
        m_vectors[size_q + j] = part_p.m_vectors[j];
        m_vectors[size_q + j].set_format(Format::EVALUATION);
    }

    if poly_in_ntt.len() > 0 {
        for i in 0..size_q {
            m_vectors[i] = poly_in_ntt[i];
        }
    } else {
        for i in 0..size_q {
            m_vectors[i].switch_format();
        }
    }

    m_format = Format::EVALUATION;
    m_params = params_qp;
}

fn approx_mod_down(
    params_q: Rc<Params>,
    params_p: Rc<Params>,
    p_inv_modq: &Vec<NativeInteger>,
    p_inv_modq_precon: &Vec<NativeInteger>,
    p_hat_inv_modp: &Vec<NativeInteger>,
    p_hat_inv_modp_precon: &Vec<NativeInteger>,
    p_hat_modq: &Vec<Vec<NativeInteger>>,
    modq_barrett_mu: &Vec<DoubleNativeInt>,
    t_inv_modp: &Vec<NativeInteger>,
    t_inv_modp_precon: &Vec<NativeInteger>,
    t: &NativeInteger,
    t_modq_precon: &Vec<NativeInteger>,
) -> DCRTPolyImpl<VecType> {
    let size_qp = m_vectors.len();
    let size_p = params_p.get_params().len();
    let size_q = size_qp - size_p;
    let mut part_p = DCRTPolyImpl::new(params_p, m_format, true);

    for j in 0..size_p {
        part_p.m_vectors[j] = m_vectors[size_q + j];
        part_p.m_vectors[j].set_format(Format::COEFFICIENT);

        if t > 0 {
            part_p.m_vectors[j] *= t_inv_modp[j];
        }
    }

    part_p.override_format(Format::COEFFICIENT);
    let part_p_switched_to_q = part_p.approx_switch_crt_basis(
        params_p,
        params_q,
        p_hat_inv_modp,
        p_hat_inv_modp_precon,
        p_hat_modq,
        modq_barrett_mu,
    );

    let mut ans = DCRTPolyImpl::new(params_q, Format::EVALUATION, true);
    let diff_q = params_q.get_params().len() - size_q;

    if diff_q > 0 {
        ans.drop_last_elements(diff_q);
    }

    for i in 0..size_q {
        if t > 0 {
            part_p_switched_to_q.m_vectors[i] *= t;
        }

        part_p_switched_to_q.m_vectors[i].set_format(Format::EVALUATION);
        ans.m_vectors[i] = (m_vectors[i] - part_p_switched_to_q.m_vectors[i]) * p_inv_modq[i];
    }

    ans
}

fn switch_crt_basis(
    params_p: Rc<Params>,
    q_hat_inv_modq: &Vec<NativeInteger>,
    q_hat_inv_modq_precon: &Vec<NativeInteger>,
    q_hat_modp: &Vec<Vec<NativeInteger>>,
    alpha_q_modp: &Vec<Vec<NativeInteger>>,
    modp_barrett_mu: &Vec<DoubleNativeInt>,
    q_inv: &Vec<f64>,
) -> DCRTPolyImpl<VecType> {
    let mut ans = DCRTPolyImpl::new(params_p, m_format, true);
    let ring_dim = m_params.get_ring_dimension();
    let size_q = m_vectors.len();
    let size_p = ans.m_vectors.len();

    for ri in 0..ring_dim {
        let mut x_q_hat_inv_modq: Vec<NativeInteger> = Vec::new();
        let mut nu = 0.5;

        for i in 0..size_q {
            let qi = m_vectors[i].get_modulus();
            x_q_hat_inv_modq[i] = m_vectors[i][ri].mod_mul_fast_const(q_hat_inv_modq[i], qi, q_hat_inv_modq_precon[i]);
            nu += x_q_hat_inv_modq[i].convert_to_double() * q_inv[i];
        }

        let alpha = nu as usize;
        let alpha_q_modpri = alpha_q_modp[alpha];

        for j in 0..size_p {
            let mut cur_value = 0;
            let pj = ans.m_vectors[j].get_modulus();
            let q_hat_modpj = q_hat_modp[j];

            for i in 0..size_q {
                cur_value += mul128(x_q_hat_inv_modq[i].convert_to_int(), q_hat_modpj[i].convert_to_int());
            }

            let cur_native_value = NativeInteger(barrett_uint128_mod_uint64(
                cur_value,
                pj.convert_to_int(),
                modp_barrett_mu[j],
            ));

            ans.m_vectors[j][ri] = cur_native_value.mod_sub_fast(alpha_q_modpri[j], pj);
        }
    }

    ans
}


impl<VecType: 'static> DCRTPolyImpl<VecType> {
    fn expand_crt_basis(
        &mut self,
        params_qp: Arc<Params>,
        params_p: Arc<Params>,
        q_hat_inv_modq: Vec<NativeInteger>,
        q_hat_inv_modq_precon: Vec<NativeInteger>,
        q_hat_modp: Vec<Vec<NativeInteger>>,
        alpha_q_modp: Vec<Vec<NativeInteger>>,
        modp_barrett_mu: Vec<DoubleNativeInt>,
        q_inv: Vec<f64>,
        result_format: Format,
    ) {
        let mut poly_in_ntt: Vec<PolyType> = Vec::new();

        if self.format == Format::Evaluation {
            poly_in_ntt = self.vectors.clone();
            self.set_format(Format::Coefficient);
        }
        let part_p = self.switch_crt_basis(
            params_p,
            q_hat_inv_modq.clone(),
            q_hat_inv_modq_precon.clone(),
            q_hat_modp.clone(),
            alpha_q_modp.clone(),
            modp_barrett_mu.clone(),
            q_inv.clone(),
        );
        let size_q = self.vectors.len();
        let size_p = part_p.vectors.len();
        let size_qp = size_p + size_q;
        self.vectors.resize(size_qp, Default::default());

        self.vectors[size_q..]
            .clone_from_slice(&part_p.vectors[..size_p]);

        for j in size_q..size_qp {
            self.vectors[j].set_format(result_format);
        }

        if result_format == Format::Evaluation {
            if !poly_in_ntt.is_empty() {
                for i in 0..size_q {
                    self.vectors[i] = poly_in_ntt[i].clone();
                }
            } else {
                for i in 0..size_q {
                    self.vectors[i].set_format(Format::Evaluation);
                }
            }
        }

        self.format = result_format;
        self.params = params_qp;
    }

    fn expand_crt_basis_reverse_order(
        &mut self,
        params_qp: Arc<Params>,
        params_p: Arc<Params>,
        q_hat_inv_modq: Vec<NativeInteger>,
        q_hat_inv_modq_precon: Vec<NativeInteger>,
        q_hat_modp: Vec<Vec<NativeInteger>>,
        alpha_q_modp: Vec<Vec<NativeInteger>>,
        modp_barrett_mu: Vec<DoubleNativeInt>,
        q_inv: Vec<f64>,
        result_format: Format,
    ) {
        let mut poly_in_ntt: Vec<PolyType> = Vec::new();

        if self.format == Format::Evaluation {
            poly_in_ntt = self.vectors.clone();
            self.set_format(Format::Coefficient);
        }
        let part_p = self.switch_crt_basis(
            params_p,
            q_hat_inv_modq.clone(),
            q_hat_inv_modq_precon.clone(),
            q_hat_modp.clone(),
            alpha_q_modp.clone(),
            modp_barrett_mu.clone(),
            q_inv.clone(),
        );
        let size_q = self.vectors.len();
        let size_p = part_p.vectors.len();
        let size_qp = size_p + size_q;
        let mut temp: Vec<PolyType> = Vec::with_capacity(size_qp);
        temp.extend(part_p.vectors.into_iter());
        temp.extend(self.vectors.into_iter());

        for i in 0..size_qp {
            temp[i].set_format(result_format);
        }

        if result_format == Format::Evaluation {
            if !poly_in_ntt.is_empty() {
                for i in 0..size_q {
                    temp[size_p + i] = poly_in_ntt[i].clone();
                }
            } else {
                for i in 0..size_q {
                    temp[size_p + i].set_format(Format::Evaluation);
                }
            }
        }

        self.format = result_format;
        self.params = params_qp;
        self.vectors = temp;
    }

    fn fast_expand_crt_basis_plover_q(&mut self, precomputed: &Precomputations) {
        let ring_dim = self.params.get_ring_dimension();
        let size_q = self.vectors.len();
        let mut part_pl = DCRTPolyImpl::new(precomputed.params_pl.clone(), self.format, true);
        let size_pl = part_pl.vectors.len();

        #[cfg(all(feature = "HAVE_INT128", feature = "NATIVEINT_64"))]
        {
            #[cfg(feature = "OPENMP")]
            let thread_limit = OpenFHEParallelControls::get_thread_limit(size_pl);

            #[cfg(feature = "OPENMP")]
            rayon::scope(|s| {
                s.spawn(|_| {
                    for ri in 0..ring_dim {
                        let mut sum = vec![0; size_pl];
                        for i in 0..size_q {
                            let xi = self.vectors[i][ri].clone();
                            let qi = self.vectors[i].get_modulus();
                            let q_inv_modpi = precomputed.q_inv_modp[i].clone();
                            let x_q_hat_inv_modqi = xi.mod_mul_fast_const(
                                precomputed.m_pl_q_hat_inv_modq[i].clone(),
                                qi.clone(),
                                precomputed.m_pl_q_hat_inv_modq_precon[i].clone(),
                            );
                            for j in 0..size_pl {
                                let a = x_q_hat_inv_modqi.convert_to_int();
                                let b = q_inv_modpi[j].convert_to_int();
                                sum[j] += mul128(a, b);
                            }
                        }
                        for j in 0..size_pl {
                            let pj = part_pl.vectors[j].get_modulus();
                            part_pl.vectors[j][ri] =
                                barrett_uint128_mod_uint64(sum[j], pj.convert_to_int(), precomputed.modp_barrett_mu[j]);
                        }
                    }
                });
            });

            let part_ql = part_pl.switch_crt_basis(
                precomputed.params_ql.clone(),
                precomputed.pl_hat_inv_modp.clone(),
                precomputed.pl_hat_inv_modp_precon.clone(),
                precomputed.pl_hat_modq.clone(),
                precomputed.alpha_pl_modq.clone(),
                precomputed.modq_barrett_mu.clone(),
                precomputed.p_inv.clone(),
            );
            let size_ql = size_pl;
            let size_ql_pl = size_pl + size_ql;

            self.vectors.resize(size_ql_pl, Default::default());

            self.vectors[..size_ql].clone_from_slice(&part_ql.vectors[..size_ql]);

            self.vectors[size_ql..].clone_from_slice(&part_pl.vectors[..size_pl]);

            self.params = precomputed.params_ql_pl.clone();
        }

        #[cfg(not(all(feature = "HAVE_INT128", feature = "NATIVEINT_64")))]
        {
            #[cfg(feature = "OPENMP")]
            let thread_limit = OpenFHEParallelControls::get_thread_limit(size_pl);

            #[cfg(feature = "OPENMP")]
            rayon::scope(|s| {
                s.spawn(|_| {
                    for ri in 0..ring_dim {
                        for i in 0..size_q {
                            let xi = self.vectors[i][ri].clone();
                            let qi = self.vectors[i].get_modulus();
                            let q_inv_modpi = precomputed.q_inv_modp[i].clone();
                            let x_q_hat_inv_modqi = xi.mod_mul_fast_const(
                                precomputed.m_pl_q_hat_inv_modq[i].clone(),
                                qi.clone(),
                                precomputed.m_pl_q_hat_inv_modq_precon[i].clone(),
                            );
                            for j in 0..size_pl {
                                let pj = part_pl.vectors[j].get_modulus();
                                let mu_j = pj.compute_mu();
                                part_pl.vectors[j][ri].mod_add_fast_eq(
                                    x_q_hat_inv_modqi.mod_mul_fast(q_inv_modpi[j].clone(), pj.clone(), mu_j),
                                    pj,
                                );
                            }
                        }
                    }
                });
            });

            let part_ql = part_pl.switch_crt_basis(
                precomputed.params_ql.clone(),
                precomputed.pl_hat_inv_modp.clone(),
                precomputed.pl_hat_inv_modp_precon.clone(),
                precomputed.pl_hat_modq.clone(),
                precomputed.alpha_pl_modq.clone(),
                precomputed.modq_barrett_mu.clone(),
                precomputed.p_inv.clone(),
            );
            let size_ql = size_pl;
            let size_ql_pl = size_pl + size_ql;

            self.vectors.resize(size_ql_pl, Default::default());

            self.vectors[..size_ql].clone_from_slice(&part_ql.vectors[..size_ql]);

            self.vectors[size_ql..].clone_from_slice(&part_pl.vectors[..size_pl]);

            self.params = precomputed.params_ql_pl.clone();
        }
    }

    fn expand_crt_basis_ql_hat(
        &mut self,
        params_q: Arc<Params>,
        ql_hat_modq: Vec<NativeInteger>,
        ql_hat_modq_precon: Vec<NativeInteger>,
        size_q: usize,
    ) {
        let size_ql = self.vectors.len();
        let ring_dim = self.params.get_ring_dimension();

        #[cfg(feature = "OPENMP")]
        let thread_limit = OpenFHEParallelControls::get_thread_limit(size_ql);

        #[cfg(feature = "OPENMP")]
        rayon::scope(|s| {
            s.spawn(|_| {
                for i in 0..size_ql {
                    let qi = self.vectors[i].get_modulus();
                    let ql_hat_modqi = ql_hat_modq[i].clone();
                    let ql_hat_modqi_precon = ql_hat_modq_precon[i].clone();
                    for ri in 0..ring_dim {
                        self.vectors[i][ri].mod_mul_fast_const_eq(ql_hat_modqi.clone(), qi.clone(), ql_hat_modqi_precon.clone());
                    }
                }
            });
        });

        self.vectors.resize(size_q, Default::default());

        for i in size_ql..size_q {
            let new_vec = PolyType::new(params_q.get_params()[i].clone(), self.format, true);
            self.vectors[i] = new_vec;
        }

        self.params = params_q;
    }
}


fn scale_and_round(
    &self,
    t: &NativeInteger,
    t_q_hat_inv_mod_q_div_q_mod_t: &Vec<NativeInteger>,
    t_q_hat_inv_mod_q_div_q_mod_t_precon: &Vec<NativeInteger>,
    t_q_hat_inv_mod_q_b_div_q_mod_t: &Vec<NativeInteger>,
    t_q_hat_inv_mod_q_b_div_q_mod_t_precon: &Vec<NativeInteger>,
    t_q_hat_inv_mod_q_div_q_frac: &Vec<f64>,
    t_q_hat_inv_mod_q_div_q_b_frac: &Vec<f64>,
) -> PolyType {
    let ring_dim = self.params.get_ring_dimension();
    let size_q = self.vectors.len();

    let q_msb = self.vectors[0].get_modulus().get_msb();

    let t_msb = t.get_msb();

    let size_q_msb = get_msb64(size_q);
    let mut coefficients = PolyType::new(ring_dim, t.convert_to_int());

    if is_power_of_two(t.convert_to_int()) {
        let t_minus_1 = t.convert_to_int() - 1;

        if q_msb + size_q_msb < 52 {
            if q_msb + t_msb + size_q_msb < 63 {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.5;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let tmp = self.vectors[i][ri];
                        float_sum += tmp.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];

                        let mut tmp = tmp;
                        tmp.mul_eq_fast(t_q_hat_inv_mod_q_div_q_mod_t[i]);
                        int_sum.add_eq_fast(tmp);
                    }
                    int_sum += float_sum as u64;

                    coefficients[ri] = int_sum.convert_to_int() & t_minus_1;
                }
            } else {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.5;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let tmp = self.vectors[i][ri];
                        float_sum += tmp.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        let mut tmp = tmp;
                        tmp.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_div_q_mod_t_precon[i],
                        );
                        int_sum.add_eq_fast(tmp);
                    }
                    int_sum += float_sum as u64;

                    coefficients[ri] = int_sum.convert_to_int() & t_minus_1;
                }
            }
        } else {
            let q_msb_hf = q_msb >> 1;
            if q_msb_hf + t_msb + size_q_msb < 62 {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.5;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let mut tmp_lo = self.vectors[i][ri];
                        let mut tmp_hi = tmp_lo.r_shift(q_msb_hf);
                        tmp_lo.sub_eq_fast(tmp_hi.l_shift(q_msb_hf));
                        float_sum += tmp_lo.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        float_sum += tmp_hi.convert_to_double() * t_q_hat_inv_mod_q_div_q_b_frac[i];

                        tmp_lo.mul_eq_fast(t_q_hat_inv_mod_q_div_q_mod_t[i]);
                        tmp_hi.mul_eq_fast(t_q_hat_inv_mod_q_b_div_q_mod_t[i]);
                        int_sum.add_eq_fast(tmp_lo);
                        int_sum.add_eq_fast(tmp_hi);
                    }
                    int_sum += float_sum as u64;

                    coefficients[ri] = int_sum.convert_to_int() & t_minus_1;
                }
            } else {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.5;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let mut tmp_lo = self.vectors[i][ri];
                        let mut tmp_hi = tmp_lo.r_shift(q_msb_hf);
                        tmp_lo.sub_eq_fast(tmp_hi.l_shift(q_msb_hf));
                        float_sum += tmp_lo.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        float_sum += tmp_hi.convert_to_double() * t_q_hat_inv_mod_q_div_q_b_frac[i];
                        tmp_lo.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_div_q_mod_t_precon[i],
                        );
                        tmp_hi.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_b_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_b_div_q_mod_t_precon[i],
                        );
                        int_sum.add_eq_fast(tmp_lo);
                        int_sum.add_eq_fast(tmp_hi);
                    }
                    int_sum += float_sum as u64;

                    coefficients[ri] = int_sum.convert_to_int() & t_minus_1;
                }
            }
        }
    } else {
        let td = t.convert_to_int() as f64;
        let t_inv = 1.0 / td;

        if q_msb + size_q_msb < 52 {
            if q_msb + t_msb + size_q_msb < 52 {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.0;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let tmp = self.vectors[i][ri];
                        float_sum += tmp.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];

                        let mut tmp = tmp;
                        tmp.mul_eq_fast(t_q_hat_inv_mod_q_div_q_mod_t[i]);
                        int_sum.add_eq_fast(tmp);
                    }

                    float_sum += int_sum.convert_to_int() as f64;
                    let quot = (float_sum * t_inv) as u64;
                    float_sum -= td * quot as f64;

                    coefficients[ri] = (float_sum + 0.5) as u64;
                }
            } else {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.0;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let tmp = self.vectors[i][ri];
                        float_sum += tmp.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        let mut tmp = tmp;
                        tmp.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_div_q_mod_t_precon[i],
                        );
                        int_sum.add_eq_fast(tmp);
                    }

                    float_sum += int_sum.convert_to_double();
                    let quot = (float_sum * t_inv) as u64;
                    float_sum -= td * quot as f64;

                    coefficients[ri] = (float_sum + 0.5) as u64;
                }
            }
        } else {
            let q_msb_hf = q_msb >> 1;
            if q_msb_hf + t_msb + size_q_msb < 52 {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.0;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let mut tmp_lo = self.vectors[i][ri];
                        let mut tmp_hi = tmp_lo.r_shift(q_msb_hf);
                        tmp_lo.sub_eq_fast(tmp_hi.l_shift(q_msb_hf));
                        float_sum += tmp_lo.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        float_sum += tmp_hi.convert_to_double() * t_q_hat_inv_mod_q_div_q_b_frac[i];

                        let mut tmp_lo = tmp_lo;
                        let mut tmp_hi = tmp_hi;
                        tmp_lo.mul_eq_fast(t_q_hat_inv_mod_q_div_q_mod_t[i]);
                        tmp_hi.mul_eq_fast(t_q_hat_inv_mod_q_b_div_q_mod_t[i]);
                        int_sum.add_eq_fast(tmp_lo);
                        int_sum.add_eq_fast(tmp_hi);
                    }

                    float_sum += int_sum.convert_to_int() as f64;
                    let quot = (float_sum * t_inv) as u64;
                    float_sum -= td * quot as f64;

                    coefficients[ri] = (float_sum + 0.5) as u64;
                }
            } else {
                for ri in 0..ring_dim {
                    let mut float_sum = 0.0;
                    let mut int_sum = 0;
                    for i in 0..size_q {
                        let mut tmp_lo = self.vectors[i][ri];
                        let mut tmp_hi = tmp_lo.r_shift(q_msb_hf);
                        tmp_lo.sub_eq_fast(tmp_hi.l_shift(q_msb_hf));
                        float_sum += tmp_lo.convert_to_double() * t_q_hat_inv_mod_q_div_q_frac[i];
                        float_sum += tmp_hi.convert_to_double() * t_q_hat_inv_mod_q_div_q_b_frac[i];
                        tmp_lo.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_div_q_mod_t_precon[i],
                        );
                        tmp_hi.mod_mul_fast_const_eq(
                            t_q_hat_inv_mod_q_b_div_q_mod_t[i],
                            t,
                            t_q_hat_inv_mod_q_b_div_q_mod_t_precon[i],
                        );
                        int_sum.add_eq_fast(tmp_lo);
                        int_sum.add_eq_fast(tmp_hi);
                    }

                    float_sum += int_sum.convert_to_int() as f64;
                    let quot = (float_sum * t_inv) as u64;
                    float_sum -= td * quot as f64;

                    coefficients[ri] = (float_sum + 0.5) as u64;
                }
            }
        }
    }

    let result = PolyType::new(
        Rc::new(PolyType::Params::new(self.params.get_cyclotomic_order(), t.convert_to_int(), 1)),
    );
    result.set_values(coefficients, Format::Coefficient);
    result
}

fn approx_scale_and_round(
    &self,
    params_p: &Rc<Params>,
    t_p_s_hat_inv_mods_divs_mod_p: &Vec<Vec<NativeInteger>>,
    mod_p_barret_mu: &Vec<DoubleNativeInt>,
) -> DCRTPolyImpl<VecType> {
    let mut ans = DCRTPolyImpl::new(params_p, self.format, true);
    let ring_dim = self.params.get_ring_dimension();
    let size_qp = self.vectors.len();
    let size_p = ans.m_vectors.len();
    let size_q = size_qp - size_p;

    let mut coefficients = Vec::with_capacity(ring_dim);
    for ri in 0..ring_dim {
        let mut ri_coefficients = Vec::with_capacity(size_p);
        for j in 0..size_p {
            let mut cur_value = 0;
            let pj = params_p.get_params()[j].get_modulus();
            let t_p_s_hat_inv_mods_divs_mod_pj = &t_p_s_hat_inv_mods_divs_mod_p[j];
            for i in 0..size_q {
                let xi = self.vectors[i][ri];
                cur_value += mul_128(xi.convert_to_int(), t_p_s_hat_inv_mods_divs_mod_pj[i].convert_to_int());
            }
            let xi = self.vectors[size_q + j][ri];
            cur_value += mul_128(xi.convert_to_int(), t_p_s_hat_inv_mods_divs_mod_pj[size_q].convert_to_int());
            ri_coefficients.push(barrett_uint128_mod_uint64(cur_value, pj.convert_to_int(), mod_p_barret_mu[j]));
        }
        coefficients.push(ri_coefficients);
    }
    ans.set_values(coefficients, Format::Coefficient);
    ans
}


fn scale_and_round(
    params_output: &std::rc::Rc<Params>,
    t_o_shat_inv_mods_divs_modo: &Vec<Vec<NativeInteger>>,
    t_o_shat_inv_mods_divs_frac: &Vec<f64>,
    modo_barret_mu: &Vec<DoubleNativeInt>,
) -> DCRTPolyImpl<VecType> {
    if NATIVEINT == 32 {
        panic!("Use of ScaleAndRound with NATIVEINT == 32 may lead to overflow");
    }
    let ans = DCRTPolyImpl::new(params_output.clone(), m_format, true);
    let ring_dim = m_params.get_ring_dimension();
    let size_qp = m_vectors.len();
    let size_o = ans.m_vectors.len();
    let size_i = size_qp - size_o;
    let mut input_index = 0;
    let mut output_index = 0;
    if params_output.get_params()[0].get_modulus() == m_params.get_params()[0].get_modulus() {
        input_index = size_o;
    } else {
        output_index = size_i;
    }
    let mut mu = vec![NativeInteger::default(); size_o];
    for j in 0..size_o {
        mu[j] = params_output.get_params()[j].get_modulus().compute_mu();
    }
    #[cfg(feature = "HAVE_INT128")]
    #[cfg(NATIVEINT == 64)]
    {
        #pragma omp parallel for
        for ri in 0..ring_dim {
            let mut nu = 0.5;
            for i in 0..size_i {
                let xi = m_vectors[i + input_index][ri];
                nu += t_o_shat_inv_mods_divs_frac[i] * xi.convert_to_double();
            }
            if is_convertable_to_native_int(nu) {
                let alpha = nu as BasicInteger;
                for j in 0..size_o {
                    let t_o_shat_inv_mods_divs_modoj = &t_o_shat_inv_mods_divs_modo[j];
                    let mut cur_value = DoubleNativeInt::default();
                    for i in 0..size_i {
                        let xi = m_vectors[i + input_index][ri];
                        cur_value += xi.convert_to_int() * t_o_shat_inv_mods_divs_modoj[i].convert_to_int();
                    }
                    let xi = m_vectors[output_index + j][ri];
                    cur_value += xi.convert_to_int() * t_o_shat_inv_mods_divs_modoj[size_i].convert_to_int();
                    let oj = params_output.get_params()[j].get_modulus();
                    let cur_native_value = barrett_uint128_mod_uint64(cur_value, oj.convert_to_int(), modo_barret_mu[j]);
                    let mut cur_alpha = alpha;
                    if alpha >= oj {
                        cur_alpha = alpha.modulus(oj, mu[j]);
                    }
                    ans.m_vectors[j][ri] = NativeInteger::default().mod_add_fast(cur_native_value, cur_alpha, oj);
                }
            } else {
                let alpha = nu as DoubleNativeInt;
                for j in 0..size_o {
                    let t_o_shat_inv_mods_divs_modoj = &t_o_shat_inv_mods_divs_modo[j];
                    let mut cur_value = DoubleNativeInt::default();
                    for i in 0..size_i {
                        let xi = m_vectors[i + input_index][ri];
                        cur_value += xi.convert_to_int() * t_o_shat_inv_mods_divs_modoj[i].convert_to_int();
                    }
                    let xi = m_vectors[output_index + j][ri];
                    cur_value += xi.convert_to_int() * t_o_shat_inv_mods_divs_modoj[size_i].convert_to_int();
                    let oj = params_output.get_params()[j].get_modulus();
                    let cur_native_value = barrett_uint128_mod_uint64(cur_value, oj.convert_to_int(), modo_barret_mu[j]);
                    ans.m_vectors[j][ri] = NativeInteger::default().mod_add_fast(cur_native_value, alpha, oj);
                }
            }
        }
    }
    #[cfg(not(feature = "HAVE_INT128"))]
    {
        #pragma omp parallel for
        for ri in 0..ring_dim {
            let mut nu = 0.5;
            for i in 0..size_i {
                let xi = m_vectors[i + input_index][ri];
                nu += t_o_shat_inv_mods_divs_frac[i] * xi.convert_to_double();
            }
            if is_convertable_to_native_int(nu) {
                let alpha = nu as BasicInteger;
                for j in 0..size_o {
                    let t_o_shat_inv_mods_divs_modoj = &t_o_shat_inv_mods_divs_modo[j];
                    let oj = ans.m_vectors[j].get_modulus();
                    let cur_value = &mut ans.m_vectors[j][ri];
                    for i in 0..size_i {
                        let xi = m_vectors[i + input_index][ri];
                        cur_value.mod_add_fast_eq(xi.mod_mul_fast(t_o_shat_inv_mods_divs_modoj[i], oj, mu[j]), oj);
                    }
                    let xi = m_vectors[output_index + j][ri];
                    cur_value.mod_add_fast_eq(xi.mod_mul_fast(t_o_shat_inv_mods_divs_modoj[size_i], oj, mu[j]), oj);
                    cur_value.mod_add_fast_eq(alpha >= oj ? alpha.modulus(oj, mu[j]) : alpha, oj);
                }
            } else {
                let (exp, mant) = std::frexp(nu);
                let mantissa = (mant * (1ULL << 53)) as BasicInteger;
                let exponent = (1ULL << (exp - 53)) as BasicInteger;
                for j in 0..size_o {
                    let t_o_shat_inv_mods_divs_modoj = &t_o_shat_inv_mods_divs_modo[j];
                    let oj = ans.m_vectors[j].get_modulus();
                    let cur_value = &mut ans.m_vectors[j][ri];
                    for i in 0..size_i {
                        let xi = m_vectors[i + input_index][ri];
                        cur_value.mod_add_fast_eq(xi.mod_mul_fast(t_o_shat_inv_mods_divs_modoj[i], oj, mu[j]), oj);
                    }
                    let xi = m_vectors[output_index + j][ri];
                    cur_value.mod_add_fast_eq(xi.mod_mul_fast(t_o_shat_inv_mods_divs_modoj[size_i], oj, mu[j]), oj);
                    cur_value.mod_add_fast_eq(exponent.mod_mul(mantissa, oj, mu[j]), oj);
                }
            }
        }
    }
    ans
}

fn scale_and_round(
    moduli_q: &Vec<NativeInteger>,
    t: &NativeInteger,
    tgamma: &NativeInteger,
    tgamma_q_hat_mod_q: &Vec<NativeInteger>,
    tgamma_q_hat_mod_q_precon: &Vec<NativeInteger>,
    neg_inv_q_mod_tgamma: &Vec<NativeInteger>,
    neg_inv_q_mod_tgamma_precon: &Vec<NativeInteger>,
) -> PolyType {
    let n = m_params.get_ring_dimension();
    let size_q = m_vectors.len();
    let gamma_minus_1 = (1 << 26) - 1;
    let mut coefficients = vec![t.convert_to_int(); n];
    #pragma omp parallel for
    for k in 0..n {
        let mut s = NativeInteger::default();
        let mut tmp = NativeInteger::default();
        for i in 0..size_q {
            let qi = moduli_q[i];
            tmp = m_vectors[i][k];
            tmp.mod_mul_fast_const_eq(tgamma_q_hat_mod_q[i], qi, tgamma_q_hat_mod_q_precon[i]);
            tmp = tmp.mod_mul_fast_const(neg_inv_q_mod_tgamma[i], tgamma, neg_inv_q_mod_tgamma_precon[i]);
            s.mod_add_fast_eq(tmp, tgamma);
        }
        s += NativeInteger(s.convert_to_int() & gamma_minus_1);
        coefficients[k] = s >> 26;
    }
    let result = PolyType::new(
        std::rc::Rc::new(PolyType::Params::new(m_params.get_cyclotomic_order(), t.convert_to_int(), 1)),
    );
    result.set_values(coefficients, Format::COEFFICIENT);
    result
}

fn scale_and_round_p_over_q(params_q: &std::rc::Rc<Params>, p_inv_mod_q: &Vec<NativeInteger>) {
    let size_q = m_vectors.len() - 1;
    let ring_dim = m_params.get_ring_dimension();
    for i in 0..size_q {
        let qi = params_q.get_params()[i].get_modulus();
        for ri in 0..ring_dim {
            m_vectors[i][ri].mod_sub_eq(m_vectors[size_q][ri], qi);
        }
    }
    m_vectors.resize(size_q);
    m_params = params_q;
    *this = this.times(p_inv_mod_q);
}

fn fast_base_conv_q_to_bsk_montgomery(
    params_q_bsk: &std::rc::Rc<Params>,
    moduli_q: &Vec<NativeInteger>,
    moduli_bsk: &Vec<NativeInteger>,
    modbsk_barret_mu: &Vec<DoubleNativeInt>,
    mtilde_q_hat_inv_mod_q: &Vec<NativeInteger>,
    mtilde_q_hat_inv_mod_q_precon: &Vec<NativeInteger>,
    q_hat_mod_bsk: &Vec<Vec<NativeInteger>>,
    q_hat_mod_mtilde: &Vec<u64>,
    q_mod_bsk: &Vec<NativeInteger>,
    q_mod_bsk_precon: &Vec<NativeInteger>,
    neg_q_inv_mod_mtilde: u64,
    mtilde_inv_mod_bsk: &Vec<NativeInteger>,
    mtilde_inv_mod_bsk_precon: &Vec<NativeInteger>,
) {
    let poly_in_ntt = if m_format == Format::EVALUATION {
        let poly_in_ntt = m_vectors.clone();
        this.set_format(Format::COEFFICIENT);
        poly_in_ntt
    } else {
        vec![]
    };
    m_params = params_q_bsk;
    m_vectors.resize(m_params.get_params().len());
    let num_q = moduli_q.len();
    let num_bsk = moduli_bsk.len();
    let n = m_params.get_ring_dimension();
    let mut ximtilde_q_hat_mod_qi = vec![NativeInteger::default(); n * num_q];
    for i in 0..num_q {
        let current_mtilde_q_hat_inv_mod_q = mtilde_q_hat_inv_mod_q[i];
        let current_mtilde_q_hat_inv_mod_q_precon = mtilde_q_hat_inv_mod_q_precon[i];
        for k in 0..n {
            ximtilde_q_hat_mod_qi[i * n + k] =
                m_vectors[i][k].mod_mul_fast_const(current_mtilde_q_hat_inv_mod_q, moduli_q[i], current_mtilde_q_hat_inv_mod_q_precon);
        }
    }
    #[cfg(feature = "HAVE_INT128")]
    #[cfg(NATIVEINT == 64)]
    {
        for j in 0..num_bsk {
            let newvec = DCRTPolyImpl::PolyType::new(m_params.get_params()[num_q + j], m_format, true);
            m_vectors[num_q + j] = newvec;
            for k in 0..n {
                let mut result = DoubleNativeInt::default();
                for i in 0..num_q {
                    let q_hat_mod_bskij = q_hat_mod_bsk[i][j];
                    result += ximtilde_q_hat_mod_qi[i * n + k].convert_to_int() * q_hat_mod_bskij.convert_to_int();
                }
                m_vectors[num_q + j][k] = barrett_uint128_mod_uint64(result, moduli_bsk[j].convert_to_int(), modbsk_barret_mu[j]);
            }
        }
    }
    #[cfg(not(feature = "HAVE_INT128"))]
    {
        let mut mu = vec![NativeInteger::default(); num_bsk];
        for j in 0..num_bsk {
            mu[j] = moduli_bsk[j].compute_mu();
        }
        for j in 0..num_bsk {
            let newvec = DCRTPolyImpl::PolyType::new(m_params.get_params()[num_q + j], m_format, true);
            m_vectors[num_q + j] = newvec;
            for k in 0..n {
                for i in 0..num_q {
                    let q_hat_mod_bskij = q_hat_mod_bsk[i][j];
                    m_vectors[num_q + j][k].mod_add_fast_eq(
                        ximtilde_q_hat_mod_qi[i * n + k].mod_mul_fast(q_hat_mod_bskij, moduli_bsk[j], mu[j]),
                        moduli_bsk[j],
                    );
                }
            }
        }
    }
    let mtilde = 1 << 16;
    let mtilde_half = mtilde >> 1;
    let mtilde_minus_1 = mtilde - 1;
    let mut result_mtilde = vec![0u64; n];
    #pragma omp parallel for
    for k in 0..n {
        for i in 0..num_q {
            result_mtilde[k] += ximtilde_q_hat_mod_qi[i * n + k].convert_to_int() * q_hat_mod_mtilde[i];
        }
        result_mtilde[k] &= mtilde_minus_1;
    }
    #pragma omp parallel for
    for k in 0..n {
        result_mtilde[k] *= neg_q_inv_mod_mtilde;
        result_mtilde[k] &= mtilde_minus_1;
    }
    for i in 0..num_bsk {
        let current_q_mod_bski = q_mod_bsk[i];
        let current_q_mod_bski_precon = q_mod_bsk_precon[i];
        #pragma omp parallel for
        for k in 0..n {
            let mut r_m_tilde = NativeInteger::default();
            if result_mtilde[k] >= mtilde_half {
                r_m_tilde += moduli_bsk[i] - mtilde;
            }
            r_m_tilde.mod_mul_fast_const_eq(current_q_mod_bski, moduli_bsk[i], current_q_mod_bski_precon);
            r_m_tilde.mod_add_fast_eq(m_vectors[num_q + i][k], moduli_bsk[i]);
            m_vectors[num_q + i][k] = r_m_tilde.mod_mul_fast_const(mtilde_inv_mod_bsk[i], moduli_bsk[i], mtilde_inv_mod_bsk_precon[i]);
        }
    }
    if poly_in_ntt.len() > 0 {
        for i in 0..num_q {
            m_vectors[i] = poly_in_ntt[i];
        }
    } else {
        #pragma omp parallel for num_threads(OpenFHEParallelControls.GetThreadLimit(num_q))
        for i in 0..num_q {
            m_vectors[i].switch_format();
        }
    }
    #pragma omp parallel for num_threads(OpenFHEParallelControls.GetThreadLimit(num_bsk))
    for i in 0..num_bsk {
        m_vectors[num_q + i].switch_format();
    }
    m_format = EVALUATION;
}


use std::sync::Arc;
use std::vec::Vec;

impl<VecType> DCRTPolyImpl<VecType> {
    fn fast_rns_floorq(
        &mut self,
        t: &NativeInteger,
        moduli_q: &Vec<NativeInteger>,
        moduli_bsk: &Vec<NativeInteger>,
        modbsk_barrett_mu: &Vec<DoubleNativeInt>,
        t_qhat_inv_modq: &Vec<NativeInteger>,
        t_qhat_inv_modq_precon: &Vec<NativeInteger>,
        qhat_modbsk: &Vec<Vec<NativeInteger>>,
        q_inv_modbsk: &Vec<Vec<NativeInteger>>,
        t_q_inv_modbsk: &Vec<NativeInteger>,
        t_q_inv_modbsk_precon: &Vec<NativeInteger>,
    ) {
        let num_q = moduli_q.len();
        let num_bsk = moduli_bsk.len();
        let n = self.get_length();

        let mut txiqi_divq_modqi = vec![NativeInteger::default(); n * num_bsk];
        for i in 0..num_q {
            let current_tq_divqi_modqi = &t_qhat_inv_modq[i];
            let current_tq_divqi_modqi_precon = &t_qhat_inv_modq_precon[i];
            for k in 0..n {
                self.m_vectors[i][k].mod_mul_fast_const_eq(
                    current_tq_divqi_modqi,
                    &moduli_q[i],
                    current_tq_divqi_modqi_precon,
                );
            }
        }

        #[cfg(all(feature = "HAVE_INT128", NATIVEINT = "64"))]
        {
            for j in 0..num_bsk {
                for k in 0..n {
                    let mut aq = 0;
                    for i in 0..num_q {
                        let inv_qi_mod_bj_value = &q_inv_modbsk[i][j];
                        let xi = &self.m_vectors[i][k];
                        aq += xi.convert_to_int() * inv_qi_mod_bj_value.convert_to_int();
                    }
                    txiqi_divq_modqi[j * n + k] = barrett_uint128_mod_uint64(
                        aq,
                        moduli_bsk[j].convert_to_int(),
                        modbsk_barrett_mu[j],
                    );
                }
            }

            for i in 0..num_bsk {
                let current_t_divq_mod_bski = &t_q_inv_modbsk[i];
                let current_t_divq_mod_bski_precon = &t_q_inv_modbsk_precon[i];
                for k in 0..n {
                    self.m_vectors[i + num_q][k].mod_mul_fast_const_eq(
                        current_t_divq_mod_bski,
                        &moduli_bsk[i],
                        current_t_divq_mod_bski_precon,
                    );
                    self.m_vectors[i + num_q][k].mod_sub_fast_eq(
                        &txiqi_divq_modqi[i * n + k],
                        &moduli_bsk[i],
                    );
                }
            }
        }
        #[cfg(not(all(feature = "HAVE_INT128", NATIVEINT = "64")))]
        {
            let mut mu = vec![NativeInteger::default(); num_bsk];
            for j in 0..num_bsk {
                mu[j] = moduli_bsk[j].compute_mu();
            }
            for j in 0..num_bsk {
                for k in 0..n {
                    for i in 0..num_q {
                        let inv_qi_mod_bj_value = &q_inv_modbsk[i][j];
                        let xi = &self.m_vectors[i][k];
                        txiqi_divq_modqi[j * n + k].mod_add_fast_eq(
                            &xi.mod_mul_fast(inv_qi_mod_bj_value, &moduli_bsk[j], &mu[j]),
                            &moduli_bsk[j],
                        );
                    }
                }
            }

            for i in 0..num_bsk {
                let current_t_divq_mod_bski = &t_q_inv_modbsk[i];
                let current_t_divq_mod_bski_precon = &t_q_inv_modbsk_precon[i];
                for k in 0..n {
                    self.m_vectors[i + num_q][k].mod_mul_fast_const_eq(
                        current_t_divq_mod_bski,
                        &moduli_bsk[i],
                        current_t_divq_mod_bski_precon,
                    );
                    self.m_vectors[i + num_q][k].mod_sub_fast_eq(
                        &txiqi_divq_modqi[i * n + k],
                        &moduli_bsk[i],
                    );
                }
            }
        }

        txiqi_divq_modqi.clear();
    }

    fn fast_base_conv_sk(
        &mut self,
        params_q: &Arc<Params>,
        modq_barrett_mu: &Vec<DoubleNativeInt>,
        moduli_bsk: &Vec<NativeInteger>,
        modbsk_barrett_mu: &Vec<DoubleNativeInt>,
        b_hat_inv_modb: &Vec<NativeInteger>,
        b_hat_inv_modb_precon: &Vec<NativeInteger>,
        b_hat_modmsk: &Vec<NativeInteger>,
        b_inv_modmsk: &NativeInteger,
        b_inv_modmsk_precon: &NativeInteger,
        b_hat_modq: &Vec<Vec<NativeInteger>>,
        b_modq: &Vec<NativeInteger>,
        b_modq_precon: &Vec<NativeInteger>,
    ) {
        #[cfg(all(feature = "HAVE_INT128", NATIVEINT = "64"))]
        {
            self.m_params = params_q.clone();
            let size_q = params_q.get_params().len();
            let mut moduli_q = vec![NativeInteger::default(); size_q];
            for i in 0..size_q {
                moduli_q[i] = params_q.get_params()[i].get_modulus();
            }

            let size_bsk = moduli_bsk.len();
            let n = self.get_length();
            for i in 0..size_bsk - 1 {
                let current_b_div_bi_mod_bi = &b_hat_inv_modb[i];
                let current_b_div_bi_mod_bi_precon = &b_hat_inv_modb_precon[i];
                for k in 0..n {
                    self.m_vectors[size_q + i][k].mod_mul_fast_const_eq(
                        current_b_div_bi_mod_bi,
                        &moduli_bsk[i],
                        current_b_div_bi_mod_bi_precon,
                    );
                }
            }
            for j in 0..size_q {
                for k in 0..n {
                    let mut result = 0;
                    for i in 0..size_bsk - 1 {
                        let current_b_div_bi_mod_qj = &b_hat_modq[i][j];
                        let xi = &self.m_vectors[size_q + i][k];
                        result += xi.convert_to_int() * current_b_div_bi_mod_qj.convert_to_int();
                    }
                    self.m_vectors[j][k] = barrett_uint128_mod_uint64(
                        result,
                        moduli_q[j].convert_to_int(),
                        modq_barrett_mu[j],
                    );
                }
            }

            let mut alphaskx_vector = vec![NativeInteger::default(); n];
            for k in 0..n {
                let mut result = 0;
                for i in 0..size_bsk - 1 {
                    let current_b_div_bi_mod_msk = &b_hat_modmsk[i];
                    result += self.m_vectors[size_q + i][k].convert_to_int()
                        * current_b_div_bi_mod_msk.convert_to_int();
                }
                alphaskx_vector[k] = barrett_uint128_mod_uint64(
                    result,
                    moduli_bsk[size_bsk - 1].convert_to_int(),
                    modbsk_barrett_mu[size_bsk - 1],
                );
            }

            for k in 0..n {
                alphaskx_vector[k] = alphaskx_vector[k].mod_sub_fast(
                    &self.m_vectors[size_q + size_bsk - 1][k],
                    &moduli_bsk[size_bsk - 1],
                );
                alphaskx_vector[k].mod_mul_fast_const_eq(
                    b_inv_modmsk,
                    &moduli_bsk[size_bsk - 1],
                    b_inv_modmsk_precon,
                );
            }

            let msk_div_two = moduli_bsk[size_bsk - 1] / 2;
            for i in 0..size_q {
                let current_b_mod_qi = &b_modq[i];
                let current_b_mod_qi_precon = &b_modq_precon[i];
                for k in 0..n {
                    let mut alphask_b_mod_qi = alphaskx_vector[k];
                    if alphask_b_mod_qi > msk_div_two {
                        alphask_b_mod_qi = alphask_b_mod_qi.mod_sub_fast(
                            &moduli_bsk[size_bsk - 1],
                            &moduli_q[i],
                        );
                    }
                    alphask_b_mod_qi.mod_mul_fast_const_eq(
                        current_b_mod_qi,
                        &moduli_q[i],
                        current_b_mod_qi_precon,
                    );
                    self.m_vectors[i][k].mod_sub_fast_eq(&alphask_b_mod_qi, &moduli_q[i]);
                }
            }

            if size_q < self.m_vectors.len() {
                let start_i = size_q;
                if start_i + size_bsk >= self.m_vectors.len() {
                    self.m_vectors.drain(start_i..);
                } else {
                    self.m_vectors.drain(start_i..start_i + size_bsk);
                }
            }
            alphaskx_vector.clear();
        }
        #[cfg(not(all(feature = "HAVE_INT128", NATIVEINT = "64")))]
        {
            self.m_params = params_q.clone();
            let size_q = params_q.get_params().len();
            let mut moduli_q = vec![NativeInteger::default(); size_q];
            for i in 0..size_q {
                moduli_q[i] = params_q.get_params()[i].get_modulus();
            }

            let size_bsk = moduli_bsk.len();
            let n = self.get_length();
            for i in 0..size_bsk - 1 {
                let current_b_div_bi_mod_bi = &b_hat_inv_modb[i];
                let current_b_div_bi_mod_bi_precon = &b_hat_inv_modb_precon[i];
                for k in 0..n {
                    self.m_vectors[size_q + i][k].mod_mul_fast_const_eq(
                        current_b_div_bi_mod_bi,
                        &moduli_bsk[i],
                        current_b_div_bi_mod_bi_precon,
                    );
                }
            }
            let mut mu = vec![NativeInteger::default(); size_q];
            for j in 0..size_q {
                mu[j] = moduli_q[j].compute_mu();
            }
            for j in 0..size_q {
                for k in 0..n {
                    self.m_vectors[j][k] = NativeInteger::default();
                    for i in 0..size_bsk - 1 {
                        let current_b_div_bi_mod_qj = &b_hat_modq[i][j];
                        let xi = &self.m_vectors[size_q + i][k];
                        self.m_vectors[j][k].mod_add_fast_eq(
                            &xi.mod_mul_fast(current_b_div_bi_mod_qj, &moduli_q[j], &mu[j]),
                            &moduli_q[j],
                        );
                    }
                }
            }
            let mu_bsk = moduli_bsk[size_bsk - 1].compute_mu();

            let mut alphaskx_vector = vec![NativeInteger::default(); n];
            for k in 0..n {
                for i in 0..size_bsk - 1 {
                    let current_b_div_bi_mod_msk = &b_hat_modmsk[i];
                    alphaskx_vector[k].mod_add_eq(
                        &self.m_vectors[size_q + i][k].mod_mul(
                            current_b_div_bi_mod_msk,
                            &moduli_bsk[size_bsk - 1],
                            &mu_bsk,
                        ),
                        &moduli_bsk[size_bsk - 1],
                    );
                }
            }

            for k in 0..n {
                alphaskx_vector[k] = alphaskx_vector[k].mod_sub_fast(
                    &self.m_vectors[size_q + size_bsk - 1][k],
                    &moduli_bsk[size_bsk - 1],
                );
                alphaskx_vector[k].mod_mul_fast_const_eq(
                    b_inv_modmsk,
                    &moduli_bsk[size_bsk - 1],
                    b_inv_modmsk_precon,
                );
            }

            let msk_div_two = moduli_bsk[size_bsk - 1] / 2;
            for i in 0..size_q {
                let current_b_mod_qi = &b_modq[i];
                let current_b_mod_qi_precon = &b_modq_precon[i];
                for k in 0..n {
                    let mut alphask_b_mod_qi = alphaskx_vector[k];
                    if alphask_b_mod_qi > msk_div_two {
                        alphask_b_mod_qi = alphask_b_mod_qi.mod_sub_fast(
                            &moduli_bsk[size_bsk - 1],
                            &moduli_q[i],
                        );
                    }
                    alphask_b_mod_qi.mod_mul_fast_const_eq(
                        current_b_mod_qi,
                        &moduli_q[i],
                        current_b_mod_qi_precon,
                    );
                    self.m_vectors[i][k].mod_sub_fast_eq(&alphask_b_mod_qi, &moduli_q[i]);
                }
            }

            if size_q < self.m_vectors.len() {
                let start_i = size_q;
                if start_i + size_bsk >= self.m_vectors.len() {
                    self.m_vectors.drain(start_i..);
                } else {
                    self.m_vectors.drain(start_i..start_i + size_bsk);
                }
            }
            alphaskx_vector.clear();
        }
    }
}


