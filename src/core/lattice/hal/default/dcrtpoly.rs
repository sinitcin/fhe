use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;

    type Vector<T>                = Vec<T>;
    type Integer               = VecType::Integer;
    type Params                = ILDCRTParams<Integer>;
    type PolyType              = PolyImpl<NativeVector>;
    type PolyLargeType         = PolyImpl<VecType>;
    type DCRTPolyType          = DCRTPolyImpl<VecType>;
    type DCRTPolyInterfaceType = DCRTPolyInterface<DCRTPolyImpl<VecType>, VecType, NativeVector, PolyImpl>;
    type Precomputations       = DCRTPolyInterfaceType::CRTBasisExtensionPrecomputations;
    type DggType               = DCRTPolyInterfaceType::DggType;
    type DugType               = DCRTPolyInterfaceType::DugType;
    type TugType               = DCRTPolyInterfaceType::TugType;
    type BugType               = DCRTPolyInterfaceType::BugType;

pub trait DCRTPolyInterface<T>: Clone + Debug + PartialEq
where
    T: Clone + Debug + PartialEq,
{
    type Integer: Clone + Debug + PartialEq;
    type Params: Clone + Debug + PartialEq;
    type PolyType: Clone + Debug + PartialEq;
    type PolyLargeType: Clone + Debug + PartialEq;
    type DCRTPolyType: Clone + Debug + PartialEq;
    type DCRTPolyInterfaceType: Clone + Debug + PartialEq;
    type Precomputations: Clone + Debug + PartialEq;
    type DggType: Clone + Debug + PartialEq;
    type DugType: Clone + Debug + PartialEq;
    type TugType: Clone + Debug + PartialEq;
    type BugType: Clone + Debug + PartialEq;

    fn clone_with_noise(&self, dgg: &Self::DggType, format: Format) -> Self::DCRTPolyType;
    fn clone_towers(&self, start_tower: u32, end_tower: u32) -> Self::DCRTPolyType;
    fn at(&self, i: usize) -> Self::Integer;
    fn at_mut(&mut self, i: usize) -> &mut Self::Integer;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn set_values_to_zero(&mut self);
    fn add_il_element_one(&mut self);
    fn drop_last_element(&mut self);
    fn drop_last_elements(&mut self, i: usize);
    fn drop_last_element_and_scale(
        &mut self,
        ql_ql_inv_modql_divql_modq: &[Self::Integer],
        ql_ql_inv_modql_divql_modq_precon: &[Self::Integer],
        ql_inv_modq: &[Self::Integer],
        ql_inv_modq_precon: &[Self::Integer],
    );
    fn mod_reduce(
        &mut self,
        t: &Self::Integer,
        t_modq_precon: &[Self::Integer],
        negt_inv_modq: &Self::Integer,
        negt_inv_modq_precon: &Self::Integer,
        ql_inv_modq: &[Self::Integer],
        ql_inv_modq_precon: &[Self::Integer],
    );
    fn crt_interpolate(&self) -> Self::PolyLargeType;
    fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> Self::PolyType;
    fn to_native_poly(&self) -> Self::PolyType;
    fn crt_interpolate_index(&self, i: usize) -> Self::PolyType;
    fn get_working_modulus(&self) -> Self::Integer;
    fn set_values_mod_switch(&mut self, element: &Self::DCRTPolyType, modulus: &Self::Integer);
    fn get_extended_crt_basis(&self, params_p: &Self::Params) -> Rc<Self::Params>;
    fn times_qovert(
        &mut self,
        params_q: &Rc<Self::Params>,
        t_inv_modq: &[Self::Integer],
        t: &Self::Integer,
        neg_q_modt: &Self::Integer,
        neg_q_modt_precon: &Self::Integer,
    );
    fn approx_switch_crt_basis(
        &self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) -> Self::DCRTPolyType;
    fn approx_mod_up(
        &mut self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        params_qp: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
    );
    fn approx_mod_down(
        &self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        p_inv_modq: &[Self::Integer],
        p_inv_modq_precon: &[Self::Integer],
        p_hat_inv_modp: &[Self::Integer],
        p_hat_inv_modp_precon: &[Self::Integer],
        p_hat_modq: &[Vec<Self::Integer>],
        modq_barrett_mu: &[DoubleNativeInt],
        t_inv_modp: &[Self::Integer],
        t_inv_modp_precon: &[Self::Integer],
        t: &Self::Integer,
        t_modq_precon: &[Self::Integer],
    ) -> Self::DCRTPolyType;
    fn switch_crt_basis(
        &self,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
    ) -> Self::DCRTPolyType;
    fn expand_crt_basis(
        &mut self,
        params_qp: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    );
    fn expand_crt_basis_reverse_order(
        &mut self,
        params_qp: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    );
    fn fast_expand_crt_basis_plover_q(&mut self, precomputed: &Self::Precomputations);
    fn expand_crt_basis_ql_hat(
        &mut self,
        params_q: &Rc<Self::Params>,
        ql_hat_modq: &[Self::Integer],
        ql_hat_modq_precon: &[Self::Integer],
        size_q: usize,
    );
    fn scale_and_round(
        &self,
        t: &Self::Integer,
        t_q_hat_inv_modq_divq_modt: &[Self::Integer],
        t_q_hat_inv_modq_divq_modt_precon: &[Self::Integer],
        t_q_hat_inv_modq_b_divq_modt: &[Self::Integer],
        t_q_hat_inv_modq_b_divq_modt_precon: &[Self::Integer],
        t_q_hat_inv_modq_divq_frac: &[f64],
        t_q_hat_inv_modq_b_divq_frac: &[f64],
    ) -> Self::DCRTPolyType;
    fn approx_scale_and_round(
        &self,
        params_p: &Rc<Self::Params>,
        t_p_s_hat_inv_mods_divs_modp: &[Vec<Self::Integer>],
        modp_barret_mu: &[DoubleNativeInt],
    ) -> Self::DCRTPolyType;
    fn scale_and_round_p_over_q(
        &mut self,
        params_q: &Rc<Self::Params>,
        p_inv_modq: &[Self::Integer],
    );
    fn fast_base_convq_to_bsk_montgomery(
        &mut self,
        params_q_bsk: &Rc<Self::Params>,
        moduli_q: &[Self::Integer],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        mtilde_q_hat_inv_modq: &[Self::Integer],
        mtilde_q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modbsk: &[Vec<Self::Integer>],
        q_hat_modmtilde: &[u64],
        q_modbsk: &[Self::Integer],
        q_modbsk_precon: &[Self::Integer],
        neg_q_inv_modmtilde: u64,
        mtilde_inv_modbsk: &[Self::Integer],
        mtilde_inv_modbsk_precon: &[Self::Integer],
    );
    fn fast_rns_floorq(
        &mut self,
        t: &Self::Integer,
        moduli_q: &[Self::Integer],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        t_q_hat_inv_modq: &[Self::Integer],
        t_q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modbsk: &[Vec<Self::Integer>],
        q_inv_modbsk: &[Vec<Self::Integer>],
        t_q_inv_modbsk: &[Self::Integer],
        t_q_inv_modbsk_precon: &[Self::Integer],
    );
    fn fast_base_conv_sk(
        &mut self,
        params_q: &Rc<Self::Params>,
        modq_barrett_mu: &[DoubleNativeInt],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        b_hat_inv_modb: &[Self::Integer],
        b_hat_inv_modb_precon: &[Self::Integer],
        b_hat_modmsk: &[Self::Integer],
        b_inv_modmsk: &Self::Integer,
        b_inv_modmsk_precon: &Self::Integer,
        b_hat_modq: &[Vec<Self::Integer>],
        b_modq: &[Self::Integer],
        b_modq_precon: &[Self::Integer],
    );
    fn switch_format(&mut self);
    fn switch_modulus_at_index(
        &mut self,
        index: usize,
        modulus: &Self::Integer,
        root_of_unity: &Self::Integer,
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct DCRTPolyImpl<T> {
    m_params: Rc<Params>,
    m_format: Format,
    m_vectors: Vec<PolyType>,
}

impl<T> DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn new() -> Self {
        Self {
            m_params: Rc::new(Vec::new()),
            m_format: Format::Evaluation,
            m_vectors: Vec::new(),
        }
    }

    pub fn from_poly_large_type(e: &PolyLargeType, params: &Rc<Params>) -> Self {
        let mut m_vectors = Vec::new();
        for p in params.get_params() {
            m_vectors.push(PolyType::new(p, e.get_format(), false));
        }
        Self {
            m_params: params.clone(),
            m_format: e.get_format(),
            m_vectors,
        }
    }

    pub fn from_poly_type(e: &PolyType, params: &Rc<Params>) -> Self {
        let mut m_vectors = Vec::new();
        for p in params.get_params() {
            m_vectors.push(PolyType::new(p, e.get_format(), false));
        }
        Self {
            m_params: params.clone(),
            m_format: e.get_format(),
            m_vectors,
        }
    }

    pub fn from_dgg_type(dgg: &DggType, p: &Rc<Params>, f: Format) -> Self {
        let mut m_vectors = Vec::new();
        for p in p.get_params() {
            m_vectors.push(PolyType::new(p, f, false));
        }
        Self {
            m_params: p.clone(),
            m_format: f,
            m_vectors,
        }
    }

    pub fn from_bug_type(bug: &BugType, p: &Rc<Params>, f: Format) -> Self {
        let mut m_vectors = Vec::new();
        for p in p.get_params() {
            m_vectors.push(PolyType::new(p, f, false));
        }
        Self {
            m_params: p.clone(),
            m_format: f,
            m_vectors,
        }
    }

    pub fn from_tug_type(tug: &TugType, p: &Rc<Params>, f: Format, h: u32) -> Self {
        let mut m_vectors = Vec::new();
        for p in p.get_params() {
            m_vectors.push(PolyType::new(p, f, false));
        }
        Self {
            m_params: p.clone(),
            m_format: f,
            m_vectors,
        }
    }

    pub fn from_dug_type(dug: &DugType, p: &Rc<Params>, f: Format) -> Self {
        let mut m_vectors = Vec::new();
        for p in p.get_params() {
            m_vectors.push(PolyType::new(p, f, false));
        }
        Self {
            m_params: p.clone(),
            m_format: f,
            m_vectors,
        }
    }

    pub fn clone_with_noise(&self, dgg: &DggType, format: Format) -> Self {
        let mut m_vectors = Vec::new();
        for p in self.m_params.get_params() {
            m_vectors.push(PolyType::new(p, format, false));
        }
        Self {
            m_params: self.m_params.clone(),
            m_format: format,
            m_vectors,
        }
    }

    pub fn clone_towers(&self, start_tower: u32, end_tower: u32) -> Self {
        let mut m_vectors = Vec::new();
        for p in self.m_params.get_params() {
            m_vectors.push(PolyType::new(p, self.m_format, false));
        }
        Self {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors,
        }
    }

    pub fn at(&self, i: usize) -> &T {
        &self.m_vectors[i]
    }

    pub fn at_mut(&mut self, i: usize) -> &mut T {
        &mut self.m_vectors[i]
    }

    pub fn len(&self) -> usize {
        self.m_vectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m_vectors.is_empty()
    }

    pub fn set_values_to_zero(&mut self) {
        for v in &mut self.m_vectors {
            v.set_values_to_zero();
        }
    }

    pub fn add_il_element_one(&mut self) {
        for v in &mut self.m_vectors {
            v.add_il_element_one();
        }
    }

    pub fn drop_last_element(&mut self) {
        for v in &mut self.m_vectors {
            v.drop_last_element();
        }
    }

    pub fn drop_last_elements(&mut self, i: usize) {
        for v in &mut self.m_vectors {
            v.drop_last_elements(i);
        }
    }

    pub fn drop_last_element_and_scale(
        &mut self,
        ql_ql_inv_modql_divql_modq: &[T],
        ql_ql_inv_modql_divql_modq_precon: &[T],
        ql_inv_modq: &[T],
        ql_inv_modq_precon: &[T],
    ) {
        for v in &mut self.m_vectors {
            v.drop_last_element_and_scale(
                ql_ql_inv_modql_divql_modq,
                ql_ql_inv_modql_divql_modq_precon,
                ql_inv_modq,
                ql_inv_modq_precon,
            );
        }
    }

    pub fn mod_reduce(
        &mut self,
        t: &T,
        t_modq_precon: &[T],
        negt_inv_modq: &T,
        negt_inv_modq_precon: &T,
        ql_inv_modq: &[T],
        ql_inv_modq_precon: &[T],
    ) {
        for v in &mut self.m_vectors {
            v.mod_reduce(
                t,
                t_modq_precon,
                negt_inv_modq,
                negt_inv_modq_precon,
                ql_inv_modq,
                ql_inv_modq_precon,
            );
        }
    }

    pub fn crt_interpolate(&self) -> PolyLargeType {
        let mut result = PolyLargeType::new();
        for v in &self.m_vectors {
            result.push(v.crt_interpolate());
        }
        result
    }

    pub fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> PolyType {
        let mut result = PolyType::new(self.m_params.clone(), self.m_format, false);
        for v in &self.m_vectors {
            result.push(v.decryption_crt_interpolate(ptm));
        }
        result
    }

    pub fn to_native_poly(&self) -> PolyType {
        let mut result = PolyType::new(self.m_params.clone(), self.m_format, false);
        for v in &self.m_vectors {
            result.push(v.to_native_poly());
        }
        result
    }

    pub fn crt_interpolate_index(&self, i: usize) -> PolyType {
        let mut result = PolyType::new(self.m_params.clone(), self.m_format, false);
        for v in &self.m_vectors {
            result.push(v.crt_interpolate_index(i));
        }
        result
    }

    pub fn get_working_modulus(&self) -> T {
        self.m_vectors[0].get_modulus()
    }

    pub fn set_values_mod_switch(&mut self, element: &Self, modulus: &T) {
        for (v, e) in self.m_vectors.iter_mut().zip(element.m_vectors.iter()) {
            v.set_values_mod_switch(e, modulus);
        }
    }

    pub fn get_extended_crt_basis(&self, params_p: &Rc<Params>) -> Rc<Params> {
        self.m_params.get_extended_crt_basis(params_p)
    }

    pub fn times_qovert(
        &mut self,
        params_q: &Rc<Params>,
        t_inv_modq: &[T],
        t: &T,
        neg_q_modt: &T,
        neg_q_modt_precon: &T,
    ) {
        for v in &mut self.m_vectors {
            v.times_qovert(params_q, t_inv_modq, t, neg_q_modt, neg_q_modt_precon);
        }
    }

    pub fn approx_switch_crt_basis(
        &self,
        params_q: &Rc<Params>,
        params_p: &Rc<Params>,
        q_hat_inv_modq: &[T],
        q_hat_inv_modq_precon: &[T],
        q_hat_modp: &[Vec<T>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) -> Self {
        let mut result = Self::new();
        for (v, q_hat_modp) in self.m_vectors.iter().zip(q_hat_modp.iter()) {
            result.m_vectors.push(v.approx_switch_crt_basis(
                params_q,
                params_p,
                q_hat_inv_modq,
                q_hat_inv_modq_precon,
                q_hat_modp,
                modp_barrett_mu,
            ));
        }
        result
    }

    pub fn approx_mod_up(
        &mut self,
        params_q: &Rc<Params>,
        params_p: &Rc<Params>,
        params_qp: &Rc<Params>,
        q_hat_inv_modq: &[T],
        q_hat_inv_modq_precon: &[T],
        q_hat_modp: &[Vec<T>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) {
        for v in &mut self.m_vectors {
            v.approx_mod_up(
                params_q,
                params_p,
                params_qp,
                q_hat_inv_modq,
                q_hat_inv_modq_precon,
                q_hat_modp,
                modp_barrett_mu,
            );
        }
    }

    pub fn approx_mod_down(
        &self,
        params_q: &Rc<Params>,
        params_p: &Rc<Params>,
        p_inv_modq: &[T],
        p_inv_modq_precon: &[T],
        p_hat_inv_modp: &[T],
        p_hat_inv_modp_precon: &[T],
        p_hat_modq: &[Vec<T>],
        modq_barrett_mu: &[DoubleNativeInt],
        t_inv_modp: &[T],
        t_inv_modp_precon: &[T],
        t: &T,
        t_modq_precon: &[T],
    ) -> Self {
        let mut result = Self::new();
        for (v, p_hat_modq) in self.m_vectors.iter().zip(p_hat_modq.iter()) {
            result.m_vectors.push(v.approx_mod_down(
                params_q,
                params_p,
                p_inv_modq,
                p_inv_modq_precon,
                p_hat_inv_modp,
                p_hat_inv_modp_precon,
                p_hat_modq,
                modq_barrett_mu,
                t_inv_modp,
                t_inv_modp_precon,
                t,
                t_modq_precon,
            ));
        }
        result
    }

    pub fn switch_crt_basis(
        &self,
        params_p: &Rc<Params>,
        q_hat_inv_modq: &[T],
        q_hat_inv_modq_precon: &[T],
        q_hat_modp: &[Vec<T>],
        alpha_q_modp: &[Vec<T>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
    ) -> Self {
        let mut result = Self::new();
        for (v, q_hat_modp, alpha_q_modp) in self
            .m_vectors
            .iter()
            .zip(q_hat_modp.iter())
            .zip(alpha_q_modp.iter())
        {
            result.m_vectors.push(v.switch_crt_basis(
                params_p,
                q_hat_inv_modq,
                q_hat_inv_modq_precon,
                q_hat_modp,
                alpha_q_modp,
                modp_barrett_mu,
                q_inv,
            ));
        }
        result
    }

    pub fn expand_crt_basis(
        &mut self,
        params_qp: &Rc<Params>,
        params_p: &Rc<Params>,
        q_hat_inv_modq: &[T],
        q_hat_inv_modq_precon: &[T],
        q_hat_modp: &[Vec<T>],
        alpha_q_modp: &[Vec<T>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    ) {
        for v in &mut self.m_vectors {
            v.expand_crt_basis(
                params_qp,
                params_p,
                q_hat_inv_modq,
                q_hat_inv_modq_precon,
                q_hat_modp,
                alpha_q_modp,
                modp_barrett_mu,
                q_inv,
                result_format,
            );
        }
    }

    pub fn expand_crt_basis_reverse_order(
        &mut self,
        params_qp: &Rc<Params>,
        params_p: &Rc<Params>,
        q_hat_inv_modq: &[T],
        q_hat_inv_modq_precon: &[T],
        q_hat_modp: &[Vec<T>],
        alpha_q_modp: &[Vec<T>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    ) {
        for v in &mut self.m_vectors {
            v.expand_crt_basis_reverse_order(
                params_qp,
                params_p,
                q_hat_inv_modq,
                q_hat_inv_modq_precon,
                q_hat_modp,
                alpha_q_modp,
                modp_barrett_mu,
                q_inv,
                result_format,
            );
        }
    }

    pub fn fast_expand_crt_basis_plover_q(&mut self, precomputed: &Precomputations) {
        for v in &mut self.m_vectors {
            v.fast_expand_crt_basis_plover_q(precomputed);
        }
    }

    pub fn expand_crt_basis_ql_hat(
        &mut self,
        params_q: &Rc<Params>,
        ql_hat_modq: &[T],
        ql_hat_modq_precon: &[T],
        size_q: usize,
    ) {
        for v in &mut self.m_vectors {
            v.expand_crt_basis_ql_hat(params_q, ql_hat_modq, ql_hat_modq_precon, size_q);
        }
    }

    pub fn scale_and_round(
        &self,
        t: &T,
        t_q_hat_inv_modq_divq_modt: &[T],
        t_q_hat_inv_modq_divq_modt_precon: &[T],
        t_q_hat_inv_modq_b_divq_modt: &[T],
        t_q_hat_inv_modq_b_divq_modt_precon: &[T],
        t_q_hat_inv_modq_divq_frac: &[f64],
        t_q_hat_inv_modq_b_divq_frac: &[f64],
    ) -> Self {
        let mut result = Self::new();
        for (
            v,
            t_q_hat_inv_modq_divq_modt,
            t_q_hat_inv_modq_divq_modt_precon,
            t_q_hat_inv_modq_b_divq_modt,
            t_q_hat_inv_modq_b_divq_modt_precon,
            t_q_hat_inv_modq_divq_frac,
            t_q_hat_inv_modq_b_divq_frac,
        ) in self
            .m_vectors
            .iter()
            .zip(t_q_hat_inv_modq_divq_modt.iter())
            .zip(t_q_hat_inv_modq_divq_modt_precon.iter())
            .zip(t_q_hat_inv_modq_b_divq_modt.iter())
            .zip(t_q_hat_inv_modq_b_divq_modt_precon.iter())
            .zip(t_q_hat_inv_modq_divq_frac.iter())
            .zip(t_q_hat_inv_modq_b_divq_frac.iter())
        {
            result.m_vectors.push(v.scale_and_round(
                t,
                t_q_hat_inv_modq_divq_modt,
                t_q_hat_inv_modq_divq_modt_precon,
                t_q_hat_inv_modq_b_divq_modt,
                t_q_hat_inv_modq_b_divq_modt_precon,
                t_q_hat_inv_modq_divq_frac,
                t_q_hat_inv_modq_b_divq_frac,
            ));
        }
        result
    }

    pub fn approx_scale_and_round(
        &self,
        params_p: &Rc<Params>,
        t_p_s_hat_inv_mods_divs_modp: &[Vec<T>],
        modp_barret_mu: &[DoubleNativeInt],
    ) -> Self {
        let mut result = Self::new();
        for (v, t_p_s_hat_inv_mods_divs_modp) in self
            .m_vectors
            .iter()
            .zip(t_p_s_hat_inv_mods_divs_modp.iter())
        {
            result.m_vectors.push(v.approx_scale_and_round(
                params_p,
                t_p_s_hat_inv_mods_divs_modp,
                modp_barret_mu,
            ));
        }
        result
    }

    pub fn scale_and_round_p_over_q(&mut self, params_q: &Rc<Params>, p_inv_modq: &[T]) {
        for v in &mut self.m_vectors {
            v.scale_and_round_p_over_q(params_q, p_inv_modq);
        }
    }

    pub fn fast_base_convq_to_bsk_montgomery(
        &mut self,
        params_q_bsk: &Rc<Params>,
        moduli_q: &[T],
        moduli_bsk: &[T],
        modbsk_barrett_mu: &[DoubleNativeInt],
        mtilde_q_hat_inv_modq: &[T],
        mtilde_q_hat_inv_modq_precon: &[T],
        q_hat_modbsk: &[Vec<T>],
        q_hat_modmtilde: &[u64],
        q_modbsk: &[T],
        q_modbsk_precon: &[T],
        neg_q_inv_modmtilde: u64,
        mtilde_inv_modbsk: &[T],
        mtilde_inv_modbsk_precon: &[T],
    ) {
        for v in &mut self.m_vectors {
            v.fast_base_convq_to_bsk_montgomery(
                params_q_bsk,
                moduli_q,
                moduli_bsk,
                modbsk_barrett_mu,
                mtilde_q_hat_inv_modq,
                mtilde_q_hat_inv_modq_precon,
                q_hat_modbsk,
                q_hat_modmtilde,
                q_modbsk,
                q_modbsk_precon,
                neg_q_inv_modmtilde,
                mtilde_inv_modbsk,
                mtilde_inv_modbsk_precon,
            );
        }
    }

    pub fn fast_rns_floorq(
        &mut self,
        t: &T,
        moduli_q: &[T],
        moduli_bsk: &[T],
        modbsk_barrett_mu: &[DoubleNativeInt],
        t_q_hat_inv_modq: &[T],
        t_q_hat_inv_modq_precon: &[T],
        q_hat_modbsk: &[Vec<T>],
        q_inv_modbsk: &[Vec<T>],
        t_q_inv_modbsk: &[T],
        t_q_inv_modbsk_precon: &[T],
    ) {
        for v in &mut self.m_vectors {
            v.fast_rns_floorq(
                t,
                moduli_q,
                moduli_bsk,
                modbsk_barrett_mu,
                t_q_hat_inv_modq,
                t_q_hat_inv_modq_precon,
                q_hat_modbsk,
                q_inv_modbsk,
                t_q_inv_modbsk,
                t_q_inv_modbsk_precon,
            );
        }
    }

    pub fn fast_base_conv_sk(
        &mut self,
        params_q: &Rc<Params>,
        modq_barrett_mu: &[DoubleNativeInt],
        moduli_bsk: &[T],
        modbsk_barrett_mu: &[DoubleNativeInt],
        b_hat_inv_modb: &[T],
        b_hat_inv_modb_precon: &[T],
        b_hat_modmsk: &[T],
        b_inv_modmsk: &T,
        b_inv_modmsk_precon: &T,
        b_hat_modq: &[Vec<T>],
        b_modq: &[T],
        b_modq_precon: &[T],
    ) {
        for v in &mut self.m_vectors {
            v.fast_base_conv_sk(
                params_q,
                modq_barrett_mu,
                moduli_bsk,
                modbsk_barrett_mu,
                b_hat_inv_modb,
                b_hat_inv_modb_precon,
                b_hat_modmsk,
                b_inv_modmsk,
                b_inv_modmsk_precon,
                b_hat_modq,
                b_modq,
                b_modq_precon,
            );
        }
    }

    pub fn switch_format(&mut self) {
        for v in &mut self.m_vectors {
            v.switch_format();
        }
    }

    pub fn switch_modulus_at_index(&mut self, index: usize, modulus: &T, root_of_unity: &T) {
        for v in &mut self.m_vectors {
            v.switch_modulus_at_index(index, modulus, root_of_unity);
        }
    }
}

impl<T> Clone for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    fn clone(&self) -> Self {
        Self {
            m_params: self.m_params.clone(),
            m_format: self.m_format,
            m_vectors: self.m_vectors.clone(),
        }
    }
}

impl<T> PartialEq for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.m_params == other.m_params
            && self.m_format == other.m_format
            && self.m_vectors == other.m_vectors
    }
}

impl<T> Index<usize> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.m_vectors[i]
    }
}

impl<T> IndexMut<usize> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.m_vectors[i]
    }
}

impl<T> AddAssign for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v += r.clone();
        }
    }
}

impl<T> AddAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v += r.clone();
        }
    }
}

impl<T> Add for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Add<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v += r.clone();
        }
    }
}

impl<T> AddAssign<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        for v in &mut self.m_vectors {
            *v += rhs.clone();
        }
    }
}

impl<T> Add<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    fn add_assign(&mut self, rhs: &T) {
        for v in &mut self.m_vectors {
            *v += rhs.clone();
        }
    }
}

impl<T> Add<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + AddAssign,
{
    type Output = Self;

    fn add(mut self, rhs: &T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> SubAssign for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v -= r.clone();
        }
    }
}

impl<T> SubAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    fn sub_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v -= r.clone();
        }
    }
}

impl<T> Sub for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Sub<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> SubAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    fn sub_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v -= r.clone();
        }
    }
}

impl<T> SubAssign<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        for v in &mut self.m_vectors {
            *v -= rhs.clone();
        }
    }
}

impl<T> Sub<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> SubAssign<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    fn sub_assign(&mut self, rhs: &T) {
        for v in &mut self.m_vectors {
            *v -= rhs.clone();
        }
    }
}

impl<T> Sub<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + SubAssign,
{
    type Output = Self;

    fn sub(mut self, rhs: &T) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> MulAssign for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v *= r.clone();
        }
    }
}

impl<T> MulAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    fn mul_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v *= r.clone();
        }
    }
}

impl<T> Mul for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Mul<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    type Output = Self;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> MulAssign<&Self> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    fn mul_assign(&mut self, rhs: &Self) {
        for (v, r) in self.m_vectors.iter_mut().zip(rhs.m_vectors.iter()) {
            *v *= r.clone();
        }
    }
}

impl<T> MulAssign<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        for v in &mut self.m_vectors {
            *v *= rhs.clone();
        }
    }
}

impl<T> Mul<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> MulAssign<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    fn mul_assign(&mut self, rhs: &T) {
        for v in &mut self.m_vectors {
            *v *= rhs.clone();
        }
    }
}

impl<T> Mul<&T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + MulAssign,
{
    type Output = Self;

    fn mul(mut self, rhs: &T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Neg for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq + Neg,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for v in &mut self.m_vectors {
            *v = -v.clone();
        }
        self
    }
}

impl<T> DCRTPolyInterface<T> for DCRTPolyImpl<T>
where
    T: Clone + Debug + PartialEq,
{
    type Integer = T;
    type Params = Params;
    type PolyType = PolyType;
    type PolyLargeType = PolyLargeType;
    type DCRTPolyType = DCRTPolyImpl<T>;
    type DCRTPolyInterfaceType = DCRTPolyInterface<DCRTPolyImpl<T>>;
    type Precomputations = CRTBasisExtensionPrecomputations;
    type DggType = DggType;
    type DugType = DugType;
    type TugType = TugType;
    type BugType = BugType;

    fn clone_with_noise(&self, dgg: &DggType, format: Format) -> Self::DCRTPolyType {
        self.clone_with_noise(dgg, format)
    }

    fn clone_towers(&self, start_tower: u32, end_tower: u32) -> Self::DCRTPolyType {
        self.clone_towers(start_tower, end_tower)
    }

    fn at(&self, i: usize) -> Self::Integer {
        self.at(i)
    }

    fn at_mut(&mut self, i: usize) -> &mut Self::Integer {
        self.at_mut(i)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn set_values_to_zero(&mut self) {
        self.set_values_to_zero();
    }

    fn add_il_element_one(&mut self) {
        self.add_il_element_one();
    }

    fn drop_last_element(&mut self) {
        self.drop_last_element();
    }

    fn drop_last_elements(&mut self, i: usize) {
        self.drop_last_elements(i);
    }

    fn drop_last_element_and_scale(
        &mut self,
        ql_ql_inv_modql_divql_modq: &[Self::Integer],
        ql_ql_inv_modql_divql_modq_precon: &[Self::Integer],
        ql_inv_modq: &[Self::Integer],
        ql_inv_modq_precon: &[Self::Integer],
    ) {
        self.drop_last_element_and_scale(
            ql_ql_inv_modql_divql_modq,
            ql_ql_inv_modql_divql_modq_precon,
            ql_inv_modq,
            ql_inv_modq_precon,
        );
    }

    fn mod_reduce(
        &mut self,
        t: &Self::Integer,
        t_modq_precon: &[Self::Integer],
        negt_inv_modq: &Self::Integer,
        negt_inv_modq_precon: &Self::Integer,
        ql_inv_modq: &[Self::Integer],
        ql_inv_modq_precon: &[Self::Integer],
    ) {
        self.mod_reduce(
            t,
            t_modq_precon,
            negt_inv_modq,
            negt_inv_modq_precon,
            ql_inv_modq,
            ql_inv_modq_precon,
        );
    }

    fn crt_interpolate(&self) -> Self::PolyLargeType {
        self.crt_interpolate()
    }

    fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> Self::PolyType {
        self.decryption_crt_interpolate(ptm)
    }

    fn to_native_poly(&self) -> Self::PolyType {
        self.to_native_poly()
    }

    fn crt_interpolate_index(&self, i: usize) -> Self::PolyType {
        self.crt_interpolate_index(i)
    }

    fn get_working_modulus(&self) -> Self::Integer {
        self.get_working_modulus()
    }

    fn set_values_mod_switch(&mut self, element: &Self::DCRTPolyType, modulus: &Self::Integer) {
        self.set_values_mod_switch(element, modulus);
    }

    fn get_extended_crt_basis(&self, params_p: &Rc<Self::Params>) -> Rc<Self::Params> {
        self.get_extended_crt_basis(params_p)
    }

    fn times_qovert(
        &mut self,
        params_q: &Rc<Self::Params>,
        t_inv_modq: &[Self::Integer],
        t: &Self::Integer,
        neg_q_modt: &Self::Integer,
        neg_q_modt_precon: &Self::Integer,
    ) {
        self.times_qovert(params_q, t_inv_modq, t, neg_q_modt, neg_q_modt_precon);
    }

    fn approx_switch_crt_basis(
        &self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) -> Self::DCRTPolyType {
        self.approx_switch_crt_basis(
            params_q,
            params_p,
            q_hat_inv_modq,
            q_hat_inv_modq_precon,
            q_hat_modp,
            modp_barrett_mu,
        )
    }

    fn approx_mod_up(
        &mut self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        params_qp: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
    ) {
        self.approx_mod_up(
            params_q,
            params_p,
            params_qp,
            q_hat_inv_modq,
            q_hat_inv_modq_precon,
            q_hat_modp,
            modp_barrett_mu,
        );
    }

    fn approx_mod_down(
        &self,
        params_q: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        p_inv_modq: &[Self::Integer],
        p_inv_modq_precon: &[Self::Integer],
        p_hat_inv_modp: &[Self::Integer],
        p_hat_inv_modp_precon: &[Self::Integer],
        p_hat_modq: &[Vec<Self::Integer>],
        modq_barrett_mu: &[DoubleNativeInt],
        t_inv_modp: &[Self::Integer],
        t_inv_modp_precon: &[Self::Integer],
        t: &Self::Integer,
        t_modq_precon: &[Self::Integer],
    ) -> Self::DCRTPolyType {
        self.approx_mod_down(
            params_q,
            params_p,
            p_inv_modq,
            p_inv_modq_precon,
            p_hat_inv_modp,
            p_hat_inv_modp_precon,
            p_hat_modq,
            modq_barrett_mu,
            t_inv_modp,
            t_inv_modp_precon,
            t,
            t_modq_precon,
        )
    }

    fn switch_crt_basis(
        &self,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
    ) -> Self::DCRTPolyType {
        self.switch_crt_basis(
            params_p,
            q_hat_inv_modq,
            q_hat_inv_modq_precon,
            q_hat_modp,
            alpha_q_modp,
            modp_barrett_mu,
            q_inv,
        )
    }

    fn expand_crt_basis(
        &mut self,
        params_qp: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    ) {
        self.expand_crt_basis(
            params_qp,
            params_p,
            q_hat_inv_modq,
            q_hat_inv_modq_precon,
            q_hat_modp,
            alpha_q_modp,
            modp_barrett_mu,
            q_inv,
            result_format,
        );
    }

    fn expand_crt_basis_reverse_order(
        &mut self,
        params_qp: &Rc<Self::Params>,
        params_p: &Rc<Self::Params>,
        q_hat_inv_modq: &[Self::Integer],
        q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modp: &[Vec<Self::Integer>],
        alpha_q_modp: &[Vec<Self::Integer>],
        modp_barrett_mu: &[DoubleNativeInt],
        q_inv: &[f64],
        result_format: Format,
    ) {
        self.expand_crt_basis_reverse_order(
            params_qp,
            params_p,
            q_hat_inv_modq,
            q_hat_inv_modq_precon,
            q_hat_modp,
            alpha_q_modp,
            modp_barrett_mu,
            q_inv,
            result_format,
        );
    }

    fn fast_expand_crt_basis_plover_q(&mut self, precomputed: &Self::Precomputations) {
        self.fast_expand_crt_basis_plover_q(precomputed);
    }

    fn expand_crt_basis_ql_hat(
        &mut self,
        params_q: &Rc<Self::Params>,
        ql_hat_modq: &[Self::Integer],
        ql_hat_modq_precon: &[Self::Integer],
        size_q: usize,
    ) {
        self.expand_crt_basis_ql_hat(params_q, ql_hat_modq, ql_hat_modq_precon, size_q);
    }

    fn scale_and_round(
        &self,
        moduliq: &[Self::Integer],
        t: &Self::Integer,
        tgamma: &Self::Integer,
        t_q_hat_inv_modq_divq_modt: &[Self::Integer],
        t_q_hat_inv_modq_divq_modt_precon: &[Self::Integer],
        neg_inv_q_mod_t_gamma: &[Self::Integer],
        neg_inv_q_mod_t_gamma_precon: &[Self::Integer],
    ) {
        self.scale_and_round(
            t,
            t_q_hat_inv_modq_divq_modt,
            t_q_hat_inv_modq_divq_modt_precon,
            t_q_hat_inv_modq_b_divq_modt,
            t_q_hat_inv_modq_b_divq_modt_precon,
            t_q_hat_inv_modq_divq_frac,
            t_q_hat_inv_modq_b_divq_frac,
        )
    }

    //     template <class Archive>
    // void save(Archive& ar, std::uint32_t const version) const {
    //     ar(::cereal::make_nvp("v", m_vectors));
    //     ar(::cereal::make_nvp("f", m_format));
    //     ar(::cereal::make_nvp("p", m_params));
    // }

    // template <class Archive>
    // void load(Archive& ar, std::uint32_t const version) {
    //     if (version > SerializedVersion()) {
    //         OPENFHE_THROW(deserialize_error, "serialized object version " + std::to_string(version) +
    //                                              " is from a later version of the library");
    //     }
    //     ar(::cereal::make_nvp("v", m_vectors));
    //     ar(::cereal::make_nvp("f", m_format));
    //     ar(::cereal::make_nvp("p", m_params));
    // }

    fn approx_scale_and_round(
        &self,
        params_p: &Rc<Self::Params>,
        t_p_s_hat_inv_mods_divs_modp: &[Vec<Self::Integer>],
        modp_barret_mu: &[DoubleNativeInt],
    ) -> Self::DCRTPolyType {
        self.approx_scale_and_round(params_p, t_p_s_hat_inv_mods_divs_modp, modp_barret_mu)
    }

    fn scale_and_round_p_over_q(
        &mut self,
        params_q: &Rc<Self::Params>,
        p_inv_modq: &[Self::Integer],
    ) {
        self.scale_and_round_p_over_q(params_q, p_inv_modq)
    }

    fn fast_base_convq_to_bsk_montgomery(
        &mut self,
        params_q_bsk: &Rc<Self::Params>,
        moduli_q: &[Self::Integer],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        mtilde_q_hat_inv_modq: &[Self::Integer],
        mtilde_q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modbsk: &[Vec<Self::Integer>],
        q_hat_modmtilde: &[u64],
        q_modbsk: &[Self::Integer],
        q_modbsk_precon: &[Self::Integer],
        neg_q_inv_modmtilde: u64,
        mtilde_inv_modbsk: &[Self::Integer],
        mtilde_inv_modbsk_precon: &[Self::Integer],
    ) {
        self.fast_base_convq_to_bsk_montgomery(
            params_q_bsk,
            moduli_q,
            moduli_bsk,
            modbsk_barrett_mu,
            mtilde_q_hat_inv_modq,
            mtilde_q_hat_inv_modq_precon,
            q_hat_modbsk,
            q_hat_modmtilde,
            q_modbsk,
            q_modbsk_precon,
            neg_q_inv_modmtilde,
            mtilde_inv_modbsk,
            mtilde_inv_modbsk_precon,
        )
    }

    fn fast_rns_floorq(
        &mut self,
        t: &Self::Integer,
        moduli_q: &[Self::Integer],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        t_q_hat_inv_modq: &[Self::Integer],
        t_q_hat_inv_modq_precon: &[Self::Integer],
        q_hat_modbsk: &[Vec<Self::Integer>],
        q_inv_modbsk: &[Vec<Self::Integer>],
        t_q_inv_modbsk: &[Self::Integer],
        t_q_inv_modbsk_precon: &[Self::Integer],
    ) {
        self.fast_rns_floorq(
            t,
            moduli_q,
            moduli_bsk,
            modbsk_barrett_mu,
            t_q_hat_inv_modq,
            t_q_hat_inv_modq_precon,
            q_hat_modbsk,
            q_inv_modbsk,
            t_q_inv_modbsk,
            t_q_inv_modbsk_precon,
        )
    }

    fn fast_base_conv_sk(
        &mut self,
        params_q: &Rc<Self::Params>,
        modq_barrett_mu: &[DoubleNativeInt],
        moduli_bsk: &[Self::Integer],
        modbsk_barrett_mu: &[DoubleNativeInt],
        b_hat_inv_modb: &[Self::Integer],
        b_hat_inv_modb_precon: &[Self::Integer],
        b_hat_modmsk: &[Self::Integer],
        b_inv_modmsk: &Self::Integer,
        b_inv_modmsk_precon: &Self::Integer,
        b_hat_modq: &[Vec<Self::Integer>],
        b_modq: &[Self::Integer],
        b_modq_precon: &[Self::Integer],
    ) {
        self.fast_base_conv_sk(
            params_q,
            modq_barrett_mu,
            moduli_bsk,
            modbsk_barrett_mu,
            b_hat_inv_modb,
            b_hat_inv_modb_precon,
            b_hat_modmsk,
            b_inv_modmsk,
            b_inv_modmsk_precon,
            b_hat_modq,
            b_modq,
            b_modq_precon,
        )
    }

    fn switch_format(&mut self) {
        self.switch_format()
    }

    fn switch_modulus_at_index(
        &mut self,
        index: usize,
        modulus: &Self::Integer,
        root_of_unity: &Self::Integer,
    ) {
        self.switch_modulus_at_index(index, modulus, root_of_unity)
    }
}

impl<T> DCRTPolyImpl<T> {
    fn set_element_at_index(&mut self, index: usint, element: &PolyType) {
        self.m_vectors[index] = element;
    }

    fn get_all_elements(&self) -> &Vec<PolyType> {
        &self.m_vectors
    }

    fn get_element_name() -> &'static str {
        "DCRTPolyImpl"
    }

    fn serialized_object_name() -> &str {
        "DCRTPoly"
    }

    fn serialized_version() -> uint32_t {
        return 1;
    }

    #[inline]
    fn get_format(&self) -> &Format {
        &self.m_format
    }

    fn override_format(&mut self, f: Format) {
        self.m_format = f;
    }

    #[inline]
    fn get_params(&self) -> Rc<Params> {
        self.m_params.clone()
    }

    fn get_all_elements(&self) -> &Vec<PolyType> {
        &self.m_vectors
    }
}
