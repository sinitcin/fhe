/*
  This file contains the linear transform interface functionality for the fixed math backend
 */

 use std::collections::HashMap;

trait NumberTheoreticTransformFxd<VecType> {
    type IntType;

    fn forward_transform_iterative(&self, element: &VecType, root_of_unity_table: &VecType, result: &mut VecType);

    fn inverse_transform_iterative(&self, element: &VecType, root_of_unity_inverse_table: &VecType, result: &mut VecType);

    fn forward_transform_to_bit_reverse(&self, element: &VecType, root_of_unity_table: &VecType, result: &mut VecType);

    fn forward_transform_to_bit_reverse_in_place(&self, root_of_unity_table: &VecType, element: &mut VecType);

    fn forward_transform_to_bit_reverse_with_precon(&self, element: &VecType, root_of_unity_table: &VecType, precon_root_of_unity_table: &VecType, result: &mut VecType);

    fn forward_transform_to_bit_reverse_in_place_with_precon(&self, root_of_unity_table: &VecType, precon_root_of_unity_table: &VecType, element: &mut VecType);

    fn inverse_transform_from_bit_reverse(&self, element: &VecType, root_of_unity_inverse_table: &VecType, cyclo_order_inv: &Self::IntType, result: &mut VecType);

    fn inverse_transform_from_bit_reverse_in_place(&self, root_of_unity_inverse_table: &VecType, cyclo_order_inv: &Self::IntType, element: &mut VecType);

    fn inverse_transform_from_bit_reverse_with_precon(&self, element: &VecType, root_of_unity_inverse_table: &VecType, precon_root_of_unity_inverse_table: &VecType, cyclo_order_inv: &Self::IntType, precon_cyclo_order_inv: &Self::IntType, result: &mut VecType);

    fn inverse_transform_from_bit_reverse_in_place_with_precon(&self, root_of_unity_inverse_table: &VecType, precon_root_of_unity_inverse_table: &VecType, cyclo_order_inv: &Self::IntType, precon_cyclo_order_inv: &Self::IntType, element: &mut VecType);
}

trait ChineseRemainderTransformFTTFxd<VecType>: NumberTheoreticTransformFxd<VecType> {
    fn forward_transform_to_bit_reverse(&self, element: &VecType, root_of_unity: &Self::IntType, cyclo_order: usize, result: &mut VecType);

    fn forward_transform_to_bit_reverse_in_place(&self, root_of_unity: &Self::IntType, cyclo_order: usize, element: &mut VecType);

    fn inverse_transform_from_bit_reverse(&self, element: &VecType, root_of_unity: &Self::IntType, cyclo_order: usize, result: &mut VecType);

    fn inverse_transform_from_bit_reverse_in_place(&self, root_of_unity: &Self::IntType, cyclo_order: usize, element: &mut VecType);

    fn pre_compute(&self, root_of_unity: &Self::IntType, cyclo_order: usize, modulus: &Self::IntType);

    fn pre_compute_with_moduli_chain(&self, root_of_unity: &Vec<Self::IntType>, cyclo_order: usize, moduli_chain: &Vec<Self::IntType>);

    fn reset(&self);

    fn cyclo_order_inverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn cyclo_order_inverse_precon_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_reverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_inverse_reverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_precon_reverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_inverse_precon_reverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;
}


use std::collections::HashMap;

type ModulusRoot<IntType> = (IntType, IntType);
type ModulusRootPair<IntType> = (ModulusRoot<IntType>, ModulusRoot<IntType>);

trait BluesteinFFTFxd<VecType> {
    type IntType;

    fn forward_transform(&self, element: &VecType, root: Self::IntType, cyclo_order: usize) -> VecType;
    fn forward_transform_with_modulus_root(&self, element: &VecType, root: Self::IntType, cyclo_order: usize, ntt_modulus_root: ModulusRoot<Self::IntType>) -> VecType;

    fn pad_zeros(&self, a: &VecType, final_size: usize) -> VecType;

    fn resize(&self, a: &VecType, lo: usize, hi: usize) -> VecType;

    fn pre_compute_default_ntt_modulus_root(&mut self, cyclo_order: usize, modulus: Self::IntType);

    fn pre_compute_root_table_for_ntt(&mut self, cyclo_order: usize, ntt_modulus_root: ModulusRoot<Self::IntType>);

    fn pre_compute_powers(&mut self, cyclo_order: usize, modulus_root: ModulusRoot<Self::IntType>);

    fn pre_compute_rb_table(&mut self, cyclo_order: usize, modulus_root_pair: ModulusRootPair<Self::IntType>);

    fn reset(&mut self);

    fn root_of_unity_table_by_modulus_root() -> &'static mut HashMap<ModulusRoot<Self::IntType>, VecType>;

    fn root_of_unity_inverse_table_by_modulus_root() -> &'static mut HashMap<ModulusRoot<Self::IntType>, VecType>;

    fn powers_table_by_modulus_root() -> &'static mut HashMap<ModulusRoot<Self::IntType>, VecType>;

    fn rb_table_by_modulus_root_pair() -> &'static mut HashMap<ModulusRootPair<Self::IntType>, VecType>;

    fn default_ntt_modulus_root() -> &'static mut HashMap<Self::IntType, ModulusRoot<Self::IntType>>;
}

trait ChineseRemainderTransformArbFxd<VecType> {
    type IntType;

    fn set_cylotomic_polynomial(&mut self, poly: &VecType, mod_: Self::IntType);

    fn forward_transform(&self, element: &VecType, root: Self::IntType, big_mod: Self::IntType, big_root: Self::IntType, cyclo_order: usize) -> VecType;

    fn inverse_transform(&self, element: &VecType, root: Self::IntType, big_mod: Self::IntType, big_root: Self::IntType, cyclo_order: usize) -> VecType;

    fn reset(&mut self);

    fn pre_compute(&mut self, cycloto_order: usize, modulus: Self::IntType);

    fn set_pre_computed_ntt_modulus(&mut self, cycloto_order: usize, modulus: Self::IntType, ntt_mod: Self::IntType, ntt_root: Self::IntType);

    fn set_pre_computed_ntt_division_modulus(&mut self, cycloto_order: usize, modulus: Self::IntType, ntt_mod: Self::IntType, ntt_root: Self::IntType);

    fn inverse_poly_mod(&self, cyclo_poly: &VecType, modulus: Self::IntType, power: usize) -> VecType;

    fn pad(&self, element: &VecType, cyclo_order: usize, forward: bool) -> VecType;

    fn drop(&self, element: &VecType, cyclo_order: usize, forward: bool, big_mod: Self::IntType, big_root: Self::IntType) -> VecType;

    fn cyclotomic_poly_map() -> &'static mut HashMap<Self::IntType, VecType>;

    fn cyclotomic_poly_reverse_ntt_map() -> &'static mut HashMap<Self::IntType, VecType>;

    fn cyclotomic_poly_ntt_map() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_division_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn root_of_unity_division_inverse_table_by_modulus() -> &'static mut HashMap<Self::IntType, VecType>;

    fn division_ntt_modulus() -> &'static mut HashMap<Self::IntType, Self::IntType>;

    fn division_ntt_root_of_unity() -> &'static mut HashMap<Self::IntType, Self::IntType>;

    fn ntt_division_dim() -> &'static mut HashMap<usize, usize>;
}


