//==================================================================================
// BSD 2-Clause License
//
// Copyright (c) 2014-2023, NJIT, Duality Technologies Inc. and other contributors
//            translated from C++ and upgraded by Anton Sinitsyn
//
// All rights reserved.
//
// Author TPOC: contact@openfhe.org
// Anton Sinitsyn: antonsinitsyn@outlook.de
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//==================================================================================

/*
    🇷🇺 Определяет интерфейс, который должны реализовать все реализации DCRT-полинома, чтобы работать в OpenFHE.
    🇬🇧 Defines an interface that any DCRT Polynomial implementation must implement in order to work in OpenFHE.
*/

use crate::types::Format;

pub struct PolyInterface<DerivedType, VecType, ContainerType> {
    _vector: VecType,
    // params: Arc<Params>,
    _format: Format,
    _marker: std::marker::PhantomData<DerivedType>,
    _marker2: std::marker::PhantomData<ContainerType>,
}

// impl<DerivedType, VecType, ContainerType> ILElement for PolyInterface<DerivedType, VecType, ContainerType> {
//     type DerivedType = DerivedType;
//     type VecType = VecType;
//     type Integer = VecType::Integer;
//     type Params = ILParamsImpl<Integer>;
//     type PolyNative = ContainerType<NativeVector>;
//     type DggType = DiscreteGaussianGeneratorImpl<VecType>;
//     type DugType = DiscreteUniformGeneratorImpl<VecType>;
//     type TugType = TernaryUniformGeneratorImpl<VecType>;
//     type BugType = BinaryUniformGeneratorImpl<VecType>;

//     fn get_derived(&self) -> &Self::DerivedType {
//         self
//     }

//     fn get_derived_mut(&mut self) -> &mut Self::DerivedType {
//         self
//     }

//     fn allocator(params: &std::shared::shared_ptr<Params>, format: Format) -> std::function<DerivedType()> {
//         move || DerivedType::new(params, format, true)
//     }

//     fn make_discrete_gaussian_coefficient_allocator(params: &std::shared::shared_ptr<Params>, result_format: Format, stddev: f64) -> std::function<DerivedType()> {
//         move || {
//             let dgg = DggType::new(stddev);
//             DerivedType::new(dgg, params, result_format)
//         }
//     }

//     fn make_discrete_uniform_allocator(params: &std::shared::shared_ptr<Params>, format: Format) -> std::function<DerivedType()> {
//         move || {
//             let dug = DugType::new();
//             DerivedType::new(dug, params, format)
//         }
//     }

//     fn get_format(&self) -> Format {
//         self.format
//     }

//     fn get_params(&self) -> &std::shared::shared_ptr<Params> {
//         &self.params
//     }

//     fn get_ring_dimension(&self) -> usint {
//         self.params.get_ring_dimension()
//     }

//     fn get_root_of_unity(&self) -> &Integer {
//         self.params.get_root_of_unity()
//     }

//     fn get_modulus(&self) -> &Integer {
//         self.params.get_modulus()
//     }

//     fn get_cyclotomic_order(&self) -> usint {
//         self.params.get_cyclotomic_order()
//     }

//     fn get_length(&self) -> usint {
//         self.vector.get_length()
//     }

//     fn get_values(&self) -> &VecType {
//         &self.vector
//     }

//     fn get_values_mut(&mut self) -> &mut VecType {
//         &mut self.vector
//     }

//     fn at(&self, i: usint) -> &Integer {
//         self.vector.at(i)
//     }

//     fn at_mut(&mut self, i: usint) -> &mut Integer {
//         self.vector.at_mut(i)
//     }

//     fn operator_index(&self, i: usint) -> &Integer {
//         self.vector.operator_index(i)
//     }

//     fn operator_index_mut(&mut self, i: usint) -> &mut Integer {
//         self.vector.operator_index_mut(i)
//     }

//     fn plus(&self, rhs: &Self::DerivedType) -> Self::DerivedType {
//         self.get_derived().plus(rhs)
//     }

//     fn minus(&self, element: &Self::DerivedType) -> Self::DerivedType {
//         self.get_derived().minus(element)
//     }

//     fn times(&self, element: &Self::DerivedType) -> Self::DerivedType {
//         self.get_derived().times(element)
//     }

//     fn times_no_check(&self, rhs: &Self::DerivedType) -> Self::DerivedType {
//         self.get_derived().times_no_check(rhs)
//     }

//     fn plus_integer(&self, element: &Integer) -> Self::DerivedType {
//         self.get_derived().plus_integer(element)
//     }

//     fn minus_integer(&self, element: &Integer) -> Self::DerivedType {
//         self.get_derived().minus_integer(element)
//     }

//     fn times_integer(&self, element: &Integer) -> Self::DerivedType {
//         self.get_derived().times_integer(element)
//     }

//     fn times_signed_integer(&self, element: NativeInteger::SignedNativeInt) -> Self::DerivedType {
//         self.get_derived().times_signed_integer(element)
//     }

//     fn times_int64(&self, rhs: i64) -> Self::DerivedType {
//         self.get_derived().times_int64(rhs)
//     }

//     fn multiply_and_round(&self, p: &Integer, q: &Integer) -> Self::DerivedType {
//         self.get_derived().multiply_and_round(p, q)
//     }

//     fn divide_and_round(&self, q: &Integer) -> Self::DerivedType {
//         self.get_derived().divide_and_round(q)
//     }

//     fn negate(&self) -> Self::DerivedType {
//         self.get_derived().negate()
//     }

//     fn unary_minus(&self) -> Self::DerivedType {
//         self.get_derived().unary_minus()
//     }

//     fn operator_plus_assign_integer(&mut self, element: &Integer) {
//         self.get_derived_mut().operator_plus_assign_integer(element)
//     }

//     fn operator_minus_assign_integer(&mut self, element: &Integer) {
//         self.get_derived_mut().operator_minus_assign_integer(element)
//     }

//     fn operator_times_assign_integer(&mut self, element: &Integer) {
//         self.get_derived_mut().operator_times_assign_integer(element)
//     }

//     fn operator_plus_assign(&mut self, rhs: &Self::DerivedType) {
//         self.get_derived_mut().operator_plus_assign(rhs)
//     }

//     fn operator_minus_assign(&mut self, rhs: &Self::DerivedType) {
//         self.get_derived_mut().operator_minus_assign(rhs)
//     }

//     fn operator_times_assign(&mut self, element: &Self::DerivedType) {
//         self.get_derived_mut().operator_times_assign(element)
//     }

//     fn operator_equals(&self, rhs: &Self::DerivedType) -> bool {
//         self.get_derived().operator_equals(rhs)
//     }

//     fn add_ilelement_one(&mut self) {
//         self.get_derived_mut().add_ilelement_one()
//     }

//     fn automorphism_transform(&self, i: u32) -> Self::DerivedType {
//         self.get_derived().automorphism_transform(i)
//     }

//     fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self::DerivedType {
//         self.get_derived().automorphism_transform_with_vec(i, vec)
//     }

//     fn transpose(&self) -> Self::DerivedType {
//         self.get_derived().transpose()
//     }

//     fn multiplicative_inverse(&self) -> Self::DerivedType {
//         self.get_derived().multiplicative_inverse()
//     }

//     fn mod_by_two(&self) -> Self::DerivedType {
//         self.get_derived().mod_by_two()
//     }

//     fn modulus(&self, modulus: &Integer) -> Self::DerivedType {
//         self.get_derived().modulus(modulus)
//     }

//     fn switch_modulus(&mut self, modulus: &Integer, root_of_unity: &Integer, modulus_arb: &Integer, root_of_unity_arb: &Integer) {
//         self.get_derived_mut().switch_modulus(modulus, root_of_unity, modulus_arb, root_of_unity_arb)
//     }

//     fn switch_format(&mut self) {
//         self.get_derived_mut().switch_format()
//     }

//     fn override_format(&mut self, f: Format) {
//         self.get_derived_mut().override_format(f)
//     }

//     fn make_sparse(&mut self, w_factor: u32) {
//         self.get_derived_mut().make_sparse(w_factor)
//     }

//     fn is_empty(&self) -> bool {
//         self.get_derived().is_empty()
//     }

//     fn inverse_exists(&self) -> bool {
//         self.get_derived().inverse_exists()
//     }

//     fn norm(&self) -> f64 {
//         self.get_derived().norm()
//     }

//     fn base_decompose(&self, base_bits: usint, eval_mode_answer: bool) -> Vec<Self::DerivedType> {
//         self.get_derived().base_decompose(base_bits, eval_mode_answer)
//     }

//     fn powers_of_base(&self, base_bits: usint) -> Vec<Self::DerivedType> {
//         self.get_derived().powers_of_base(base_bits)
//     }

//     fn set_values(&mut self, values: VecType, format: Format) {
//         self.get_derived_mut().set_values(values, format)
//     }

//     fn set_values_to_zero(&mut self) {
//         self.get_derived_mut().set_values_to_zero()
//     }

//     fn set_values_to_max(&mut self) {
//         self.get_derived_mut().set_values_to_max()
//     }

//     fn crt_interpolate(&self) -> Self::DerivedType {
//         self.get_derived().crt_interpolate()
//     }

//     fn decryption_crt_interpolate(&self, ptm: PlaintextModulus) -> PolyNative {
//         self.get_derived().decryption_crt_interpolate(ptm)
//     }

//     fn to_native_poly(&self) -> PolyNative {
//         self.get_derived().to_native_poly()
//     }

//     fn clone(&self) -> Self::DerivedType {
//         self.get_derived().clone()
//     }

//     fn clone_empty(&self) -> Self::DerivedType {
//         self.get_derived().clone_empty()
//     }

//     fn clone_parameters_only(&self) -> Self::DerivedType {
//         self.get_derived().clone_parameters_only()
//     }

//     fn clone_with_noise(&self, dgg: DggType, format: Format) -> Self::DerivedType {
//         self.get_derived().clone_with_noise(dgg, format)
//     }

//     fn get_element_name(&self) -> String {
//         self.get_derived().get_element_name()
//     }
// }
