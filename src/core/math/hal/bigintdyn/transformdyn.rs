
/*
  This file contains the linear transform interface functionality for the dynamic math backend
 */

 use std::collections::HashMap;

trait NumberTheoreticTransformDyn<VecType> {
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

struct ChineseRemainderTransformFTTDyn<VecType> {
    _marker: std::marker::PhantomData<VecType>,
}

impl<VecType> ChineseRemainderTransformFTTDyn<VecType> {
    fn forward_transform_to_bit_reverse(&self, element: &VecType, root_of_unity: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, cyclo_order: usize, result: &mut VecType);

    fn forward_transform_to_bit_reverse_in_place(&self, root_of_unity: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, cyclo_order: usize, element: &mut VecType);

    fn inverse_transform_from_bit_reverse(&self, element: &VecType, root_of_unity: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, cyclo_order: usize, result: &mut VecType);

    fn inverse_transform_from_bit_reverse_in_place(&self, root_of_unity: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, cyclo_order: usize, element: &mut VecType);

    fn pre_compute(&self, root_of_unity: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, cyclo_order: usize, modulus: &<VecType as NumberTheoreticTransformDyn<VecType>>::IntType);

    fn pre_compute_with_vectors(&self, root_of_unity: &Vec<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType>, cyclo_order: usize, moduli_chain: &Vec<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType>);

    fn reset(&self);
}

impl<VecType> ChineseRemainderTransformFTTDyn<VecType> {
    static mut CYCLO_ORDER_INVERSE_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
    static mut CYCLO_ORDER_INVERSE_PRECON_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
    static mut ROOT_OF_UNITY_REVERSE_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
    static mut ROOT_OF_UNITY_INVERSE_REVERSE_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
    static mut ROOT_OF_UNITY_PRECON_REVERSE_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
    static mut ROOT_OF_UNITY_INVERSE_PRECON_REVERSE_TABLE_BY_MODULUS: HashMap<<VecType as NumberTheoreticTransformDyn<VecType>>::IntType, VecType> = HashMap::new();
}

use std::collections::HashMap;

struct ModulusRoot<T> {
    first: T,
    second: T,
}

struct ModulusRootPair<T> {
    first: ModulusRoot<T>,
    second: ModulusRoot<T>,
}

struct BluesteinFFTDyn<VecType> 
where
    VecType: IntegerType, // Assuming VecType has an associated Integer type
{
    // Static fields
    root_of_unity_table_by_modulus_root: HashMap<ModulusRoot<VecType::Integer>, VecType>,
    root_of_unity_inverse_table_by_modulus_root: HashMap<ModulusRoot<VecType::Integer>, VecType>,
    powers_table_by_modulus_root: HashMap<ModulusRoot<VecType::Integer>, VecType>,
    rb_table_by_modulus_root_pair: HashMap<ModulusRootPair<VecType::Integer>, VecType>,
    default_ntt_modulus_root: HashMap<VecType::Integer, ModulusRoot<VecType::Integer>>,
}

impl<VecType> BluesteinFFTDyn<VecType> 
where
    VecType: IntegerType, // Placeholder trait to represent the operations
{
    // Methods would be implemented here
}

struct ChineseRemainderTransformArbDyn<VecType> 
where
    VecType: IntegerType, // Assuming VecType has an associated Integer type
{
    // Static fields
    cyclotomic_poly_map: HashMap<VecType::Integer, VecType>,
    cyclotomic_poly_reverse_ntt_map: HashMap<VecType::Integer, VecType>,
    cyclotomic_poly_ntt_map: HashMap<VecType::Integer, VecType>,
    root_of_unity_division_table_by_modulus: HashMap<VecType::Integer, VecType>,
    root_of_unity_division_inverse_table_by_modulus: HashMap<VecType::Integer, VecType>,
    division_ntt_modulus: HashMap<VecType::Integer, VecType::Integer>,
    division_ntt_root_of_unity: HashMap<VecType::Integer, VecType::Integer>,
    ntt_division_dim: HashMap<usize, usize>,
}

impl<VecType> ChineseRemainderTransformArbDyn<VecType> 
where
    VecType: IntegerType, // Placeholder trait to represent the operations
{
    // Methods would be implemented here
}




