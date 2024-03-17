use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Sub, Mul, Neg};
use std::fmt::Debug;

trait ILElement {
    fn plus(&self, rhs: &Self) -> Self;
    fn minus(&self, element: &Self) -> Self;
    fn times(&self, element: &Self) -> Self;
    fn times_no_check(&self, rhs: &Self) -> Self;
    fn plus_integer(&self, element: &Integer) -> Self;
    fn minus_integer(&self, element: &Integer) -> Self;
    fn times_integer(&self, element: &Integer) -> Self;
    fn times_signed_native_int(&self, element: NativeInteger::SignedNativeInt) -> Self;
    fn multiply_and_round(&self, p: &Integer, q: &Integer) -> Self;
    fn divide_and_round(&self, q: &Integer) -> Self;
    fn negate(&self) -> Self;
    fn automorphism_transform(&self, i: u32) -> Self;
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self;
    fn multiplicative_inverse(&self) -> Self;
    fn mod_by_two(&self) -> Self;
    fn modulo(&self, modulus: &Integer) -> Self;
    fn switch_modulus(&self, modulus: &Integer, root_of_unity: &Integer, modulus_arb: &Integer, root_of_unity_arb: &Integer);
    fn switch_format(&self);
    fn override_format(&self, f: Format);
    fn make_sparse(&self, w_factor: u32);
    fn is_empty(&self) -> bool;
    fn inverse_exists(&self) -> bool;
    fn norm(&self) -> f64;
    fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self>;
    fn powers_of_base(base_bits: usize) -> Self where Self: Sized;
    fn set_values(&mut self, values: Vec<Self>, format: Format);
    fn set_values_to_zero(&mut self);
    fn set_values_to_max(&mut self);
    fn crt_interpolate(&self) -> Self where Self: Sized;
    fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> PolyNative;
    fn to_native_poly(&self) -> PolyNative;
    fn clone(&self) -> Self where Self: Sized;
    fn clone_empty(&self) -> Self where Self: Sized;
    fn clone_parameters_only(&self) -> Self where Self: Sized;
    fn clone_with_noise(&self, dgg: DggType, format: Format) -> Self where Self: Sized;
    fn get_element_name(&self) -> String;
}

struct PolyInterface<DerivedType, VecType, ContainerType> {
    vector: VecType,
    params: Rc<Params>,
    format: Format,
    _marker: std::marker::PhantomData<DerivedType>,
    _marker2: std::marker::PhantomData<ContainerType>,
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType> {
    fn new(vector: VecType, params: Rc<Params>, format: Format) -> Self {
        Self {
            vector,
            params,
            format,
            _marker: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
        }
    }
    
    fn get_vector(&self) -> &VecType {
        &self.vector
    }
    
    fn get_vector_mut(&mut self) -> &mut VecType {
        &mut self.vector
    }
    
    fn get_params(&self) -> &Rc<Params> {
        &self.params
    }
    
    fn get_format(&self) -> Format {
        self.format
    }
    
    fn get_ring_dimension(&self) -> usize {
        self.params.get_ring_dimension()
    }
    
    fn get_root_of_unity(&self) -> &Integer {
        &self.params.get_root_of_unity()
    }
    
    fn get_modulus(&self) -> &Integer {
        &self.params.get_modulus()
    }
    
    fn get_cyclotomic_order(&self) -> usize {
        self.params.get_cyclotomic_order()
    }
    
    fn get_length(&self) -> usize {
        self.vector.get_length()
    }
    
    fn at(&self, i: usize) -> &Integer {
        self.vector.at(i)
    }
    
    fn at_mut(&mut self, i: usize) -> &mut Integer {
        self.vector.at_mut(i)
    }
    
    fn operator_index(&self, i: usize) -> &Integer {
        self.vector.operator_index(i)
    }
    
    fn operator_index_mut(&mut self, i: usize) -> &mut Integer {
        self.vector.operator_index_mut(i)
    }
    
    fn plus(&self, rhs: &Self) -> Self {
        Self::new(self.vector.plus(&rhs.vector), self.params.clone(), self.format)
    }
    
    fn minus(&self, element: &Self) -> Self {
        self.get_derived().minus(element)
    }
    
    fn times(&self, element: &Self) -> Self {
        self.get_derived().times(element)
    }
    
    fn times_no_check(&self, rhs: &Self) -> Self {
        Self::new(self.vector.times_no_check(&rhs.vector), self.params.clone(), self.format)
    }
    
    fn plus_integer(&self, element: &Integer) -> Self {
        self.get_derived().plus_integer(element)
    }
    
    fn minus_integer(&self, element: &Integer) -> Self {
        self.get_derived().minus_integer(element)
    }
    
    fn times_integer(&self, element: &Integer) -> Self {
        self.get_derived().times_integer(element)
    }
    
    fn times_signed_native_int(&self, element: NativeInteger::SignedNativeInt) -> Self {
        self.get_derived().times_signed_native_int(element)
    }
    
    fn multiply_and_round(&self, p: &Integer, q: &Integer) -> Self {
        self.get_derived().multiply_and_round(p, q)
    }
    
    fn divide_and_round(&self, q: &Integer) -> Self {
        self.get_derived().divide_and_round(q)
    }
    
    fn negate(&self) -> Self {
        self.get_derived().negate()
    }
    
    fn operator_minus(&self) -> Self {
        self.get_derived().operator_minus()
    }
    
    fn operator_plus_assign(&mut self, element: &Integer) {
        self.get_derived_mut().operator_plus_assign(element)
    }
    
    fn operator_minus_assign(&mut self, element: &Integer) {
        self.get_derived_mut().operator_minus_assign(element)
    }
    
    fn operator_times_assign(&mut self, element: &Integer) {
        self.get_derived_mut().operator_times_assign(element)
    }
    
    fn operator_plus_assign_poly(&mut self, rhs: &Self) {
        self.get_derived_mut().operator_plus_assign_poly(rhs)
    }
    
    fn operator_minus_assign_poly(&mut self, rhs: &Self) {
        self.get_derived_mut().operator_minus_assign_poly(rhs)
    }
    
    fn operator_times_assign_poly(&mut self, element: &Self) {
        self.get_derived_mut().operator_times_assign_poly(element)
    }
    
    fn operator_eq(&self, rhs: &Self) -> bool {
        self.get_derived().operator_eq(rhs)
    }
    
    fn add_ilelement_one(&mut self) {
        self.get_derived_mut().add_ilelement_one()
    }
    
    fn automorphism_transform(&self, i: u32) -> Self {
        self.get_derived().automorphism_transform(i)
    }
    
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self {
        self.get_derived().automorphism_transform_with_vec(i, vec)
    }
    
    fn transpose(&self) -> Self {
        if self.get_format() == Format::COEFFICIENT {
            panic!("PolyInterface element transposition is currently implemented only in the Evaluation representation.");
        }
        self.get_derived().automorphism_transform(self.get_cyclotomic_order() - 1)
    }
    
    fn multiplicative_inverse(&self) -> Self {
        self.get_derived().multiplicative_inverse()
    }
    
    fn mod_by_two(&self) -> Self {
        self.get_derived().mod_by_two()
    }
    
    fn modulo(&self, modulus: &Integer) -> Self {
        self.get_derived().modulo(modulus)
    }
    
    fn switch_modulus(&mut self, modulus: &Integer, root_of_unity: &Integer, modulus_arb: &Integer, root_of_unity_arb: &Integer) {
        self.get_derived_mut().switch_modulus(modulus, root_of_unity, modulus_arb, root_of_unity_arb)
    }
    
    fn switch_format(&mut self) {
        self.get_derived_mut().switch_format()
    }
    
    fn override_format(&mut self, f: Format) {
        self.get_derived_mut().override_format(f)
    }
    
    fn make_sparse(&mut self, w_factor: u32) {
        self.get_derived_mut().make_sparse(w_factor)
    }
    
    fn is_empty(&self) -> bool {
        self.get_derived().is_empty()
    }
    
    fn inverse_exists(&self) -> bool {
        self.get_derived().inverse_exists()
    }
    
    fn norm(&self) -> f64 {
        self.get_derived().norm()
    }
    
    fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self> {
        self.get_derived().base_decompose(base_bits, eval_mode_answer)
    }
}

impl<DerivedType, VecType, ContainerType> ILElement for PolyInterface<DerivedType, VecType, ContainerType> {
    fn plus(&self, rhs: &Self) -> Self {
        self.plus(rhs)
    }
    
    fn minus(&self, element: &Self) -> Self {
        self.minus(element)
    }
    
    fn times(&self, element: &Self) -> Self {
        self.times(element)
    }
    
    fn times_no_check(&self, rhs: &Self) -> Self {
        self.times_no_check(rhs)
    }
    
    fn plus_integer(&self, element: &Integer) -> Self {
        self.plus_integer(element)
    }
    
    fn minus_integer(&self, element: &Integer) -> Self {
        self.minus_integer(element)
    }
    
    fn times_integer(&self, element: &Integer) -> Self {
        self.times_integer(element)
    }
    
    fn times_signed_native_int(&self, element: NativeInteger::SignedNativeInt) -> Self {
        self.times_signed_native_int(element)
    }
    
    fn multiply_and_round(&self, p: &Integer, q: &Integer) -> Self {
        self.multiply_and_round(p, q)
    }
    
    fn divide_and_round(&self, q: &Integer) -> Self {
        self.divide_and_round(q)
    }
    
    fn negate(&self) -> Self {
        self.negate()
    }
    
    fn automorphism_transform(&self, i: u32) -> Self {
        self.automorphism_transform(i)
    }
    
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self {
        self.automorphism_transform_with_vec(i, vec)
    }
    
    fn multiplicative_inverse(&self) -> Self {
        self.multiplicative_inverse()
    }
    
    fn mod_by_two(&self) -> Self {
        self.mod_by_two()
    }
    
    fn modulo(&self, modulus: &Integer) -> Self {
        self.modulo(modulus)
    }
    
    fn switch_modulus(&self, modulus: &Integer, root_of_unity: &Integer, modulus_arb: &Integer, root_of_unity_arb: &Integer) {
        self.switch_modulus(modulus, root_of_unity, modulus_arb, root_of_unity_arb)
    }
    
    fn switch_format(&self) {
        self.switch_format()
    }
    
    fn override_format(&self, f: Format) {
        self.override_format(f)
    }
    
    fn make_sparse(&self, w_factor: u32) {
        self.make_sparse(w_factor)
    }
    
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    
    fn inverse_exists(&self) -> bool {
        self.inverse_exists()
    }
    
    fn norm(&self) -> f64 {
        self.norm()
    }
    
    fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self> {
        self.base_decompose(base_bits, eval_mode_answer)
    }
}

impl<DerivedType, VecType, ContainerType> Add for PolyInterface<DerivedType, VecType, ContainerType> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl<DerivedType, VecType, ContainerType> Sub for PolyInterface<DerivedType, VecType, ContainerType> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl<DerivedType, VecType, ContainerType> Mul for PolyInterface<DerivedType, VecType, ContainerType> {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}

impl<DerivedType, VecType, ContainerType> Neg for PolyInterface<DerivedType, VecType, ContainerType> {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl<DerivedType, VecType, ContainerType> PartialEq for PolyInterface<DerivedType, VecType, ContainerType> {
    fn eq(&self, rhs: &Self) -> bool {
        self.operator_eq(rhs)
    }
}

impl<DerivedType, VecType, ContainerType> Debug for PolyInterface<DerivedType, VecType, ContainerType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

fn allocator<DerivedType>(params: Rc<Params>, format: Format) -> Box<dyn Fn() -> DerivedType> {
    Box::new(move || DerivedType::new(params.clone(), format, true))
}

fn make_discrete_gaussian_coefficient_allocator<DerivedType>(params: Rc<Params>, result_format: Format, stddev: f64) -> Box<dyn Fn() -> DerivedType> {
    Box::new(move || {
        let dgg = DggType::new(stddev);
        DerivedType::new(dgg, params.clone(), result_format)
    })
}

fn make_discrete_uniform_allocator<DerivedType>(params: Rc<Params>, format: Format) -> Box<dyn Fn() -> DerivedType> {
    Box::new(move || {
        let dug = DugType::new();
        DerivedType::new(dug, params.clone(), format)
    })
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType> {
    fn get_derived(&self) -> &DerivedType {
        unsafe {
            let derived_type = self as *const Self as *const DerivedType;
            &*derived_type
        }
    }
    
    fn get_derived_mut(&mut self) -> &mut DerivedType {
        unsafe {
            let derived_type = self as *mut Self as *mut DerivedType;
            &mut *derived_type
        }
    }
}

impl<DerivedType, VecType, ContainerType> Clone for PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: Clone,
    VecType: Clone,
{
    fn clone(&self) -> Self {
        Self {
            vector: self.vector.clone(),
            params: self.params.clone(),
            format: self.format,
            _marker: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
        }
    }
}

impl<DerivedType, VecType, ContainerType> Copy for PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: Copy,
    VecType: Copy,
{
}

impl<DerivedType, VecType, ContainerType> Default for PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: Default,
    VecType: Default,
{
    fn default() -> Self {
        Self {
            vector: VecType::default(),
            params: Rc::new(Params::default()),
            format: Format::COEFFICIENT,
            _marker: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
        }
    }
}

impl<DerivedType, VecType, ContainerType> PartialEq for PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.vector == rhs.vector
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

impl<DerivedType, VecType, ContainerType> PolyInterface<DerivedType, VecType, ContainerType>
where
    DerivedType: PartialEq,
    VecType: PartialEq,
{
    fn operator_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}



impl Display for PolyInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.get_format() == Format::Evaluation ? "EVAL" : "COEF", self.get_values())
    }
}

impl Add for PolyInterface {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.plus(other)
    }
}

impl Add<Integer> for PolyInterface {
    type Output = Self;

    fn add(self, other: Integer) -> Self::Output {
        self.plus(other)
    }
}

impl Add<PolyInterface> for Integer {
    type Output = PolyInterface;

    fn add(self, other: PolyInterface) -> Self::Output {
        other.plus(self)
    }
}

impl Sub for PolyInterface {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.minus(other)
    }
}

impl Sub<Integer> for PolyInterface {
    type Output = Self;

    fn sub(self, other: Integer) -> Self::Output {
        self.minus(other)
    }
}

impl Mul for PolyInterface {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.times(other)
    }
}

impl Mul<Integer> for PolyInterface {
    type Output = Self;

    fn mul(self, other: Integer) -> Self::Output {
        self.times(other)
    }
}

impl Mul<PolyInterface> for Integer {
    type Output = PolyInterface;

    fn mul(self, other: PolyInterface) -> Self::Output {
        other.times(self)
    }
}

impl Mul<i64> for PolyInterface {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        self.times(NativeInteger::SignedNativeInt(other))
    }
}

impl Mul<PolyInterface> for i64 {
    type Output = PolyInterface;

    fn mul(self, other: PolyInterface) -> Self::Output {
        other.times(NativeInteger::SignedNativeInt(self))
    }
}
