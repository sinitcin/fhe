/*
  Defines an interface that any DCRT Polynomial implementation must implement in order to work in OpenFHE.
 */

use std::{cmp::PartialEq, marker::PhantomData, ops::{AddAssign, Index, IndexMut, Neg, SubAssign}};

trait ILElement<DerivedType, BigVecType> {
    // Define methods expected for ILElement here
}

struct ILDCRTParams<BigIntType> {
    // Define the structure here
}

struct DiscreteGaussianGeneratorImpl<LilVecType> {
    // Define the structure here
}

struct DiscreteUniformGeneratorImpl<LilVecType> {
    // Define the structure here
}

struct TernaryUniformGeneratorImpl<LilVecType> {
    // Define the structure here
}

struct BinaryUniformGeneratorImpl<LilVecType> {
    // Define the structure here
}

trait DCRTPolyInterface<DerivedType, BigVecType, LilVecType, RNSContainerType>
where
    RNSContainerType: Fn(LilVecType) -> Self,
{
    type BigIntType;
    type Params;
    type LilIntType;
    type TowerType;
    type PolyLargeType;
    type DggType;
    type DugType;
    type TugType;
    type BugType;

    fn get_derived(&self) -> &DerivedType;
    fn get_derived_mut(&mut self) -> &mut DerivedType;
}

impl<DerivedType, BigVecType, LilVecType, RNSContainerType> DCRTPolyInterface<DerivedType, BigVecType, LilVecType, RNSContainerType> for DerivedType
where
    DerivedType: ILElement<DerivedType, BigVecType>,
    RNSContainerType: Fn(LilVecType) -> DerivedType,
{
    type BigIntType = <BigVecType as SomeTrait>::Integer; // Assuming SomeTrait is defined elsewhere
    type Params = ILDCRTParams<Self::BigIntType>;
    type LilIntType = <LilVecType as SomeTrait>::Integer; // Assuming SomeTrait is defined elsewhere
    type TowerType = RNSContainerType;
    type PolyLargeType = RNSContainerType;
    type DggType = DiscreteGaussianGeneratorImpl<LilVecType>;
    type DugType = DiscreteUniformGeneratorImpl<LilVecType>;
    type TugType = TernaryUniformGeneratorImpl<LilVecType>;
    type BugType = BinaryUniformGeneratorImpl<LilVecType>;

  /**
   * @brief Get the Derived object, this is apart of the CRTP software design pattern
   * it allows the base class (this one) to implement methods that call the derived
   * objects implementation.
   *
   * @ref Chapter 21.2 "C++ Templates The Complete Guide" by David Vandevoorde and Nicolai M. Josuttis
   * http://www.informit.com/articles/article.asp?p=31473
   *
   * @return DerivedType&
   */
    fn get_derived(&self) -> &DerivedType {
        self
    }

    fn get_derived_mut(&mut self) -> &mut DerivedType {
        self
    }



  /**
   * @brief Create lambda that allocates a zeroed element for the case when it
   * is called from a templated class
   *
   * @param params the params to use.
   * @param format - EVALUATION or COEFFICIENT
   */

    fn allocator(params: Arc<Params>, format: Format) -> Box<dyn Fn() -> DerivedType> {
        Box::new(move || {
            DerivedType::new(params.clone(), format, true)
        })
    }

    /**
     * @brief Allocator for discrete uniform distribution.
     *
     * @param params Params instance that is is passed.
     * @param resultFormat resultFormat for the polynomials generated.
     * @param stddev standard deviation for the discrete gaussian generator.
     * @return the resulting vector.
     */
    fn make_discrete_gaussian_coefficient_allocator(params: Arc<Params>, result_format: Format, stddev: f64) -> Box<dyn Fn() -> DerivedType> {
        Box::new(move || {
            let dgg = DggType::new(stddev);
            DerivedType::new_with_dgg(dgg, params.clone(), result_format)
        })
    }


    /**
     * @brief Allocator for discrete uniform distribution.
     *
     * @param params Params instance that is is passed.
     * @param format format for the polynomials generated.
     * @return the resulting vector.
     */
    fn make_discrete_uniform_allocator(params: Arc<Params>, format: Format) -> Box<dyn Fn() -> DerivedType> {
        Box::new(move || {
            let dug = DugType::new();
            DerivedType::new_with_dug(dug, params.clone(), format)
        })
    }
  
  /**
   * @brief Makes a copy of the DCRTPoly, but it includes only a sequential
   * subset of the towers that the original holds.
   *
   * @param startTower The index number of the first tower to clone
   * @param endTower The index number of the last tower to clone
   * @return new Element
   */
    fn clone_towers(&self, start_tower: u32, end_tower: u32) -> DerivedType {
        self.get_derived().clone_towers(start_tower, end_tower)
    }

    fn clone(&self) -> DerivedType {
        DerivedType::new_from_derived(self.get_derived())
    }

    fn clone_empty(&self) -> DerivedType {
        DerivedType::new_empty()
    }

    fn clone_parameters_only(&self) -> DerivedType {
        DerivedType::new_with_params_and_format(self.get_derived().get_params(), self.get_derived().get_format())
    }

   /**
   * @brief returns the parameters of the element.
   * @return the element parameter set.
   */
    fn get_params(&self) -> Arc<Params> {
        self.get_derived().get_params()
    }

        /**
   * @brief returns the element's cyclotomic order
   * @return returns the cyclotomic order of the element.
   */
    fn get_cyclotomic_order(&self) -> u32 {
        self.get_derived().get_params().get_cyclotomic_order()
    }

        /**
   * @brief returns the element's ring dimension
   * @return returns the ring dimension of the element.
   */
    fn get_ring_dimension(&self) -> u32 {
        self.get_derived().get_params().get_ring_dimension()
    }

        /**
   * @brief returns the element's modulus
   * @return returns the modulus of the element.
   */
    fn get_modulus(&self) -> BigIntType {
        self.get_derived().get_params().get_modulus()
    }


    /**
   * @brief returns the element's original modulus, derived from Poly
   * @return returns the modulus of the element.
   */
    fn get_original_modulus(&self) -> BigIntType {
        self.get_derived().get_params().get_original_modulus()
    }

    /**
   * @brief returns the element's root of unity.
   * @return the element's root of unity.
   */
    // TODO: this doesn't look right
    fn get_root_of_unity(&self) -> BigIntType {
        BigIntType::new(0)
    }



    /**
   * @brief Get method for length of each component element.
   * NOTE assumes all components are the same size. (Ring Dimension)
   *
   * @return length of the component element
   */
    fn get_length(&self) -> usize {
        self.get_derived().get_params().get_ring_dimension()
    }

    /**
   * @brief Get interpolated value of elements at all tower index i.
   * Note this operation is computationally intense. Does bound checking
   * @return interpolated value at index i.
   */
    fn at(&mut self, i: usize) -> &mut BigIntType;
    fn at(&self, i: usize) -> &BigIntType;


    /**
   * @brief Get interpolated value of element at index i.
   * Note this operation is computationally intense. No bound checking
   * @return interpolated value at index i.
   */

    impl Index<usize> for BigIntType {
        type Output = BigIntType;

        fn index(&self, i: usize) -> &Self::Output {
            // Implementation goes here
            unimplemented!()
        }
    }

    impl IndexMut<usize> for BigIntType {
        fn index_mut(&mut self, i: usize) -> &mut Self::Output {
            // Implementation goes here
            unimplemented!()
        }
    }

    /**
   * @brief Get method that returns a vector of all component elements.
   *
   * @returns a vector of the component elements.
   */
    
    fn get_all_elements(&self) -> Vec<TowerType> {
        &mut self.GetDerived().GetAllElements()
    }

    /**
   * @brief Get method of the number of component elements, also known as the
   * number of towers.
   *
   * @return the number of component elements.
   */
    fn get_num_of_elements(&self) -> usize {
        self.get_all_elements().len()
    }

    /**
   * @brief Get method of individual tower of elements.
   * Note this behavior is different than poly
   * @param i index of tower to be returned.
   * @returns a reference to the returned tower
   */
    fn get_element_at_index(&self, i: usize) -> &Self {
        &self.get_derived().get_all_elements()[i]
    }

    /**
   * @brief Get value of element at index i.
   *
   * @return value at index i.
   *
   * @warning Should be removed to disable access to the towers, all modifications
   * in the lattice layer should be done in the lattice layer. This means new functions
   * will be need in the lattice layer.
   */
    fn element_at_index(&self, i: usize) -> &T {
        &self.get_derived().get_all_elements()[i]
    }

    /**
   * @brief Sets element at index
   *
   * @param index where the element should be set
   * @param element The element to store
   */
    fn set_element_at_index(&mut self, index: usize, element: T) {
        self.get_derived().set_element_at_index(index, element);
    }

    /**
   * @brief Sets element at index
   *
   * @param index where the element should be set
   * @param element The element to store
   */
    fn set_element_at_index(&mut self, index: usize, element: TowerType) {
        self.get_derived().set_element_at_index(index, element);
    }
    /**
   * @brief Write the element as \f$ \sum\limits{i=0}^{\lfloor {\log q/base}
   * \rfloor} {(base^i u_i)} \f$ and return the vector of \f$ \left\{u_0,
   * u_1,...,u_{\lfloor {\log q/base} \rfloor} \right\} \in R_{{base}^{\lceil
   * {\log q/base} \rceil}} \f$; This is used as a subroutine in the
   * relinearization procedure.
   *
   * @param baseBits is the number of bits in the base, i.e., \f$ base =
   * 2^{baseBits} \f$.
   * @return is the pointer where the base decomposition vector is stored
   *
   * @warning not efficient and  not fast, uses multiprecision arithmetic and
   *          will be removed in future. Use @see DCRTPolyInterface::CRTDecompose instead.
   */
    // std::vector<DerivedType> BaseDecompose(usint baseBits, bool evalModeAnswer) const override = 0;

    /**
   * @brief Generate a vector of PolyImpl's as \f$ \left\{x, {base}*x,
   * {base}^2*x, ..., {base}^{\lfloor {\log q/{base}} \rfloor} \right\}*x \f$,
   * where \f$ x \f$ is the current PolyImpl object;
   * used as a subroutine in the relinearization procedure to get powers of a
   * certain "base" for the secret key element.
   *
   * @param baseBits is the number of bits in the base, i.e., \f$ base =
   * 2^{baseBits} \f$.
   * @return is the pointer where the base decomposition vector is stored
   *
   * @warning not efficient and  not fast, uses multiprecision arithmetic and
   *          will be removed in future. Use @see DCRTPolyInterface::CRTDecompose instead.
   */
    // std::vector<DerivedType> PowersOfBase(usint baseBits) const override = 0;

    /**
   * CRT basis decomposition of c as [c qi/q]_qi
   *
   * @param &baseBits bits in the base for additional digit decomposition if
   * base > 0
   * @return is the pointer where the resulting vector is stored
   */
    fn crt_decompose(&self, base_bits: u32) -> Vec<DerivedType> {
        self.get_derived().crt_decompose(base_bits)
    }
}




trait DerivedType: Sized {
    fn assign_tower_type(&mut self, rhs: &TowerType) -> &mut Self;
    /**
     * @brief Assignment Operator.
     *
     * @param &rhs the copied element.
     * @return the resulting element.
     */
    fn assign_derived_type(&mut self, rhs: &Self) -> &mut Self;

    /**
     * @brief Move Assignment Operator.
     *
     * @param &rhs the copied element.
     * @return the resulting element.
     */
    fn assign_derived_type_move(&mut self, rhs: Self) -> &mut Self;
    
    /**
     * @brief Initializer list
     *
     * @param &rhs the list to initialized the element.
     * @return the resulting element.
     */
    fn assign_from_initializer_list_u64(&mut self, rhs: &[u64]) -> &mut Self;
    
    /**
     * @brief Assignment Operator. The usint val will be set at index zero and all
     * other indices will be set to zero.
     *
     * @param val is the usint to assign to index zero.
     * @return the resulting vector.
     */
    fn assign_u64(&mut self, val: u64) -> &mut Self;
    
    /**
     * @brief Creates a Poly from a vector of signed integers (used for trapdoor
     * sampling)
     *
     * @param &rhs the vector to set the PolyImpl to.
     * @return the resulting PolyImpl.
     */
    fn assign_from_vector_i64(&mut self, rhs: &Vec<i64>) -> &mut Self;
    
    /**
     * @brief Creates a Poly from a vector of signed integers (used for trapdoor
     * sampling)
     *
     * @param &rhs the vector to set the PolyImpl to.
     * @return the resulting PolyImpl.
     */
    
    fn assign_from_vector_i32(&mut self, rhs: &Vec<i32>) -> &mut Self;
    /**
     * @brief Initializer list
     *
     * @param &rhs the list to set the PolyImpl to.
     * @return the resulting PolyImpl.
     */
    fn assign_from_initializer_list_string(&mut self, rhs: &[String]) -> &mut Self;

    /**
     * @brief Unary minus on a element.
     * @return additive inverse of the an element.
     */
    fn neg(&self) -> Self;

    /**
     * @brief Equality operator.
     *
     * @param &rhs is the specified element to be compared with this element.
     * @return true if this element represents the same values as the specified
     * element, false otherwise.
     */
    fn equals(&self, rhs: &Self) -> bool;

    /**
     * @brief Performs an entry-wise addition over all elements of each tower with
     * the towers of the element on the right hand side.
     *
     * @param &rhs is the element to add with.
     * @return is the result of the addition.
     */
    fn add_assign(&mut self, rhs: &Self);

    /**
     * @brief Performs an entry-wise subtraction over all elements of each tower
     * with the towers of the element on the right hand side.
     *
     * @param &rhs is the element to subtract from.
     * @return is the result of the addition.
     */
    fn sub_assign(&mut self, rhs: &Self);

    fn automorphism_transform(&self, i: u32) -> Self where Self: Sized;
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self where Self: Sized;
    fn transpose(&self) -> Self where Self: Sized {
        if self.get_format() == Format::Coefficient {
            panic!("DCRTPolyInterface element transposition is currently implemented only in the Evaluation representation.");
        }
        self.automorphism_transform(self.get_cyclotomic_order() - 1)
    }
    fn plus(&self, rhs: &Self) -> Self where Self: Sized;
    fn times(&self, rhs: &Self) -> Self where Self: Sized;
    fn minus(&self, rhs: &Self) -> Self where Self: Sized;
    fn plus_bigint(&self, rhs: &BigIntType) -> Self where Self: Sized;
    fn plus_bigint_vec(&self, rhs: &Vec<BigIntType>) -> Self where Self: Sized {
        self.plus(rhs)
    }
    fn minus_bigint(&self, rhs: &BigIntType) -> Self where Self: Sized;
    fn minus_bigint_vec(&self, rhs: &Vec<BigIntType>) -> Self where Self: Sized {
        self.minus(rhs)
    }
    fn times_bigint(&self, rhs: &BigIntType) -> Self where Self: Sized;
    fn times_native_int(&self, rhs: i64) -> Self where Self: Sized; // Assuming SignedNativeInt is equivalent to i64 in Rust
    fn times(&self, rhs: i64) -> Self where Self: Sized;
    fn times_vec_native_integer(&self, rhs: &Vec<NativeInteger>) -> Self where Self: Sized;
    fn times_no_check(&self, rhs: &Vec<NativeInteger>) -> Self where Self: Sized;
    fn times_vec_big_int_type(&self, rhs: &Vec<BigIntType>) -> Self where Self: Sized;
    fn multiply_and_round(&self, p: &BigIntType, q: &BigIntType) -> Self where Self: Sized;
    fn divide_and_round(&self, q: &BigIntType) -> Self where Self: Sized;
    fn negate(&self) -> Self where Self: Sized;
    fn add_assign_big_int_type(&mut self, rhs: &BigIntType);
    fn add_assign_lil_int_type(&mut self, rhs: &LilIntType);
    fn sub_assign_big_int_type(&mut self, rhs: &BigIntType);
    fn sub_assign_lil_int_type(&mut self, rhs: &LilIntType);
    fn mul_assign_big_int_type(&mut self, rhs: &BigIntType);
    fn mul_assign_lil_int_type(&mut self, rhs: &LilIntType);
    fn mul_assign_derived_type(&mut self, rhs: &Self);
    fn multiplicative_inverse(&self) -> Self where Self: Sized;
    fn mod_by_two(&self) -> Self where Self: Sized;
    fn mod_op(&self, modulus: &BigIntType) -> Self where Self: Sized;
    fn get_values(&self) -> &BigVecType;
    fn set_values(&mut self, values: &BigVecType, format: Format);
    fn set_values_to_zero(&mut self);
    fn set_values_mod_switch(&mut self, element: &Self, modulus: &NativeInteger) where Self: Sized;
    fn add_i_l_element_one(&mut self);
    fn add_random_noise(&self, modulus: &BigIntType) -> Self where Self: Sized;
    fn make_sparse(&mut self, w_factor: u32);
    fn is_empty(&self) -> bool;
    fn drop_last_element(&mut self);
    fn drop_last_elements(&mut self, i: usize);
    fn drop_last_element_and_scale(&mut self, ql_ql_inv_modql_divql_modq: &Vec<NativeInteger>, ql_ql_inv_modql_divql_modq_precon: &Vec<NativeInteger>, ql_inv_modq: &Vec<NativeInteger>, ql_inv_modq_precon: &Vec<NativeInteger>);
    fn mod_reduce(&mut self, t: &NativeInteger, t_modq_precon: &Vec<NativeInteger>, negt_inv_modq: &NativeInteger, negt_inv_modq_precon: &NativeInteger, ql_inv_modq: &Vec<NativeInteger>, ql_inv_modq_precon: &Vec<NativeInteger>);
    fn crt_interpolate(&self) -> PolyLargeType;
    fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> TowerType;
    fn to_native_poly(&self) -> TowerType;
    fn crt_interpolate_index(&self, i: usize) -> PolyLargeType;
    fn get_working_modulus(&self) -> BigIntType;
    fn get_extended_crt_basis(&self, params_p: &Arc<Params>) -> Arc<Params>;
    fn times_qovert(&mut self, params_q: &Arc<Params>, t_inv_modq: &Vec<NativeInteger>, t: &NativeInteger, neg_q_modt: &NativeInteger, neg_q_modt_precon: &NativeInteger);



}

    /**
   * @brief Equality operator.
   *
   * @param &rhs is the specified element to be compared with this element.
   * @return true if this element represents the same values as the specified
   * element, false otherwise.
   */
impl PartialEq for dyn DerivedType {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Neg for dyn DerivedType {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}

    /**
   * @brief Performs an entry-wise addition over all elements of each tower with
   * the towers of the element on the right hand side.
   *
   * @param &rhs is the element to add with.
   * @return is the result of the addition.
   */
impl AddAssign for dyn DerivedType {
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other);
    }
}


    /**
   * @brief Performs an entry-wise subtraction over all elements of each tower
   * with the towers of the element on the right hand side.
   *
   * @param &rhs is the element to subtract from.
   * @return is the result of the addition.
   */
impl SubAssign for dyn DerivedType {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other);
    }
}


trait CRTBasis {
    type DerivedType;

    fn approx_switch_crt_basis(
        &self,
        params_q: Rc<Params>,
        params_p: Rc<Params>,
        q_hat_inv_modq: &Vec<NativeInteger>,
        q_hat_inv_modq_precon: &Vec<NativeInteger>,
        q_hat_modp: &Vec<Vec<NativeInteger>>,
        modp_barrett_mu: &Vec<DoubleNativeInt>,
    ) -> Self::DerivedType;

    fn approx_mod_up(
        &self,
        params_q: Rc<Params>,
        params_p: Rc<Params>,
        params_qp: Rc<Params>,
        q_hat_inv_modq: &Vec<NativeInteger>,
        q_hat_inv_modq_precon: &Vec<NativeInteger>,
        q_hat_modp: &Vec<Vec<NativeInteger>>,
        modp_barrett_mu: &Vec<DoubleNativeInt>,
    );

    fn approx_mod_down(
        &self,
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
    ) -> Self::DerivedType;

    fn switch_crt_basis(
        &self,
        params_p: Rc<Params>,
        q_hat_inv_modq: &Vec<NativeInteger>,
        q_hat_inv_modq_precon: &Vec<NativeInteger>,
        q_hat_modp: &Vec<Vec<NativeInteger>>,
        alpha_q_modp: &Vec<Vec<NativeInteger>>,
        modp_barrett_mu: &Vec<DoubleNativeInt>,
        q_inv: &Vec<f64>,
    ) -> Self::DerivedType;

    fn expand_crt_basis(
        &self,
        params_qp: Rc<Params>,
        params_p: Rc<Params>,
        q_hat_inv_modq: &Vec<NativeInteger>,
        q_hat_inv_modq_precon: &Vec<NativeInteger>,
        q_hat_modp: &Vec<Vec<NativeInteger>>,
        alpha_q_modp: &Vec<Vec<NativeInteger>>,
        modp_barrett_mu: &Vec<DoubleNativeInt>,
        q_inv: &Vec<f64>,
        result_format: Format,
    );

    fn expand_crt_basis_reverse_order(
        &self,
        params_qp: Rc<Params>,
        params_p: Rc<Params>,
        q_hat_inv_modq: &Vec<NativeInteger>,
        q_hat_inv_modq_precon: &Vec<NativeInteger>,
        q_hat_modp: &Vec<Vec<NativeInteger>>,
        alpha_q_modp: &Vec<Vec<NativeInteger>>,
        modp_barrett_mu: &Vec<DoubleNativeInt>,
        q_inv: &Vec<f64>,
        result_format: Format,
    );
}

struct CRTBasisExtensionPrecomputations {
    params_ql_pl: std::shared_ptr<Params>,
    params_pl: std::shared_ptr<Params>,
    params_ql: std::shared_ptr<Params>,
    m_pl_q_hat_inv_modq: Vec<NativeInteger>,
    m_pl_q_hat_inv_modq_precon: Vec<NativeInteger>,
    q_inv_modp: Vec<Vec<NativeInteger>>,
    modp_barrett_mu: Vec<DoubleNativeInt>,
    pl_hat_inv_modp: Vec<NativeInteger>,
    pl_hat_inv_modp_precon: Vec<NativeInteger>,
    pl_hat_modq: Vec<Vec<NativeInteger>>,
    alpha_pl_modq: Vec<Vec<NativeInteger>>,
    modq_barrett_mu: Vec<DoubleNativeInt>,
    p_inv: Vec<f64>,
}

impl CRTBasisExtensionPrecomputations {
    fn new(
        params_ql_pl: std::shared_ptr<Params>,
        params_pl: std::shared_ptr<Params>,
        params_ql: std::shared_ptr<Params>,
        m_pl_q_hat_inv_modq: Vec<NativeInteger>,
        m_pl_q_hat_inv_modq_precon: Vec<NativeInteger>,
        q_inv_modp: Vec<Vec<NativeInteger>>,
        modp_barrett_mu: Vec<DoubleNativeInt>,
        pl_hat_inv_modp: Vec<NativeInteger>,
        pl_hat_inv_modp_precon: Vec<NativeInteger>,
        pl_hat_modq: Vec<Vec<NativeInteger>>,
        alpha_pl_modq: Vec<Vec<NativeInteger>>,
        modq_barrett_mu: Vec<DoubleNativeInt>,
        p_inv: Vec<f64>,
    ) -> Self {
        Self {
            params_ql_pl,
            params_pl,
            params_ql,
            m_pl_q_hat_inv_modq,
            m_pl_q_hat_inv_modq_precon,
            q_inv_modp,
            modp_barrett_mu,
            pl_hat_inv_modp,
            pl_hat_inv_modp_precon,
            pl_hat_modq,
            alpha_pl_modq,
            modq_barrett_mu,
            p_inv,
        }
    }
}


use std::shared::Shared;
use std::vec::Vec;
use std::string::String;

struct CRTBasisExtensionPrecomputations;

trait CRTBasisExtension {
    fn fast_expand_crt_basis_plover_q(&self, precomputed: &CRTBasisExtensionPrecomputations);
    fn expand_crt_basis_ql_hat(&self, params_q: Shared<Params>, ql_hat_modq: Vec<NativeInteger>, ql_hat_modq_precon: Vec<NativeInteger>, size_q: usint);
    fn scale_and_round(&self, t: NativeInteger, t_qhat_inv_modq_divq_modt: Vec<NativeInteger>, t_qhat_inv_modq_divq_modt_precon: Vec<NativeInteger>, t_qhat_inv_modq_b_divq_modt: Vec<NativeInteger>, t_qhat_inv_modq_b_divq_modt_precon: Vec<NativeInteger>, t_qhat_inv_modq_divq_frac: Vec<f64>, t_qhat_inv_modq_b_divq_frac: Vec<f64>) -> TowerType;
    fn approx_scale_and_round(&self, params_p: Shared<Params>, t_pshat_inv_mods_divs_modp: Vec<Vec<NativeInteger>>, modp_barret_mu: Vec<DoubleNativeInt>) -> DerivedType;
    fn scale_and_round(&self, params_output: Shared<Params>, t_oshat_inv_mods_divs_modo: Vec<Vec<NativeInteger>>, t_oshat_inv_mods_divs_frac: Vec<f64>, modo_barret_mu: Vec<DoubleNativeInt>) -> DerivedType;
    fn scale_and_round(&self, moduli_q: Vec<NativeInteger>, t: NativeInteger, tgamma: NativeInteger, tgamma_qhat_modq: Vec<NativeInteger>, tgamma_qhat_modq_precon: Vec<NativeInteger>, neg_invq_modtgamma: Vec<NativeInteger>, neg_invq_modtgamma_precon: Vec<NativeInteger>) -> TowerType;
    fn scale_and_round_p_over_q(&self, params_q: Shared<Params>, p_inv_modq: Vec<NativeInteger>);
    fn fast_base_convq_to_bsk_montgomery(&self, params_q_bsk: Shared<Params>, moduli_q: Vec<NativeInteger>, moduli_bsk: Vec<NativeInteger>, modbsk_barrett_mu: Vec<DoubleNativeInt>, mtilde_qhat_inv_modq: Vec<NativeInteger>, mtilde_qhat_inv_modq_precon: Vec<NativeInteger>, qhat_modbsk: Vec<Vec<NativeInteger>>, qhat_modmtilde: Vec<u64>, q_modbsk: Vec<NativeInteger>, q_modbsk_precon: Vec<NativeInteger>, neg_q_inv_modmtilde: u64, mtilde_inv_modbsk: Vec<NativeInteger>, mtilde_inv_modbsk_precon: Vec<NativeInteger>);
    fn fast_rns_floorq(&self, t: NativeInteger, moduli_q: Vec<NativeInteger>, moduli_bsk: Vec<NativeInteger>, modbsk_barrett_mu: Vec<DoubleNativeInt>, t_qhat_inv_modq: Vec<NativeInteger>, t_qhat_inv_modq_precon: Vec<NativeInteger>, qhat_modbsk: Vec<Vec<NativeInteger>>, q_inv_modbsk: Vec<Vec<NativeInteger>>, t_q_inv_modbsk: Vec<NativeInteger>, t_q_inv_modbsk_precon: Vec<NativeInteger>);
    fn fast_base_conv_sk(&self, params_q: Shared<Params>, modq_barrett_mu: Vec<DoubleNativeInt>, moduli_bsk: Vec<NativeInteger>, modbsk_barrett_mu: Vec<DoubleNativeInt>, b_hat_inv_modb: Vec<NativeInteger>, b_hat_inv_modb_precon: Vec<NativeInteger>, b_hat_modmsk: Vec<NativeInteger>, b_inv_modmsk: NativeInteger, b_inv_modmsk_precon: NativeInteger, b_hat_modq: Vec<Vec<NativeInteger>>, b_modq: Vec<NativeInteger>, b_modq_precon: Vec<NativeInteger>);
    fn switch_format(&self);
    fn override_format(&self, f: Format);
    fn switch_modulus(&self, modulus: BigIntType, root_of_unity: BigIntType, modulus_arb: BigIntType, root_of_unity_arb: BigIntType);
    fn switch_modulus_at_index(&self, index: usize, modulus: BigIntType, root_of_unity: BigIntType);
    fn inverse_exists(&self) -> bool;
    fn norm(&self) -> f64 {
        PolyLargeType(self.get_derived().crt_interpolate()).norm()
    }
    fn get_element_name(&self) -> String {
        self.get_derived().get_element_name()
    }
}

impl DerivedType {
    fn operator<<(&self, os: &std::io::Write) {
        for i in 0..self.get_all_elements().len() {
            if i != 0 {
                os.write("\n");
            }
            os.write(i.to_string());
            os.write(": ");
            os.write(self.get_all_elements()[i].to_string());
        }
    }
}

impl DerivedType {
    fn operator+(&self, a: &DerivedType, b: &DerivedType) -> DerivedType {
        a.plus(b)
    }
    
    fn operator+(&self, a: &DerivedType, b: &BigIntType) -> DerivedType {
        a.plus(b)
    }
    
    fn operator+(&self, a: &BigIntType, b: &DerivedType) -> DerivedType {
        b.plus(a)
    }
    
    fn operator+(&self, a: &DerivedType, b: &Vec<BigIntType>) -> DerivedType {
        a.plus(b)
    }
    
    fn operator+(&self, a: &Vec<BigIntType>, b: &DerivedType) -> DerivedType {
        b.plus(a)
    }
    
    fn operator-(&self, a: &DerivedType, b: &DerivedType) -> DerivedType {
        a.minus(b)
    }
    
    fn operator-(&self, a: &DerivedType, b: &Vec<BigIntType>) -> DerivedType {
        a.minus(b)
    }
    
    fn operator-(&self, a: &Vec<BigIntType>, b: &DerivedType) -> DerivedType {
        b.minus(a)
    }
    
    fn operator-(&self, a: &DerivedType, b: &BigIntType) -> DerivedType {
        a.minus(b)
    }
    
    fn operator*(&self, a: &DerivedType, b: &DerivedType) -> DerivedType {
        a.times(b)
    }
    
    fn operator*(&self, a: &DerivedType, b: &BigIntType) -> DerivedType {
        a.times(b)
    }
    
    fn operator*(&self, a: &DerivedType, b: &Vec<BigIntType>) -> DerivedType {
        a.times(b)
    }
    
    fn operator*(&self, a: &BigIntType, b: &DerivedType) -> DerivedType {
        b.times(a)
    }
    
    fn operator*(&self, a: &DerivedType, b: i64) -> DerivedType {
        a.times(b as NativeInteger::SignedNativeInt)
    }
    
    fn operator*(&self, a: i64, b: &DerivedType) -> DerivedType {
        b.times(a as NativeInteger::SignedNativeInt)
    }
}


