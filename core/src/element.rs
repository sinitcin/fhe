//==================================================================================
// BSD 2-Clause License
//
// Copyright (c) 2014-2023, NJIT, Duality Technologies Inc. and other contributors
//
// All rights reserved.
//
// Author TPOC: contact@openfhe.org
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
    🇷🇺 Представляет и задает объекты элементов целочисленных решеток в OpenFHE
    🇬🇧 Represents and defines integer lattice element objects in OpenFHE
*/

pub struct DiscreteGaussianGeneratorImpl<X> {}

pub struct Format;

/// 🇷🇺 Интерфейс для идеальных решеток
///
/// Каждая решетка должна реализовать эти чистые виртуальные функции, чтобы
/// правильно взаимодействовать с OpenFHE PKE. Element - это возвращаемый
/// тип для всех этих виртуальных функций. В базовом классе здесь нет конструктора;
/// он не содержит данных для построения
///
/// 🇬🇧 Interface for ideal lattices
///
/// Every lattice must implement these pure virtual functions in order to properly
/// interoperate with OpenFHE PKE. Element is the return type for all of these
/// virtual functions. There is no constructor here in the base class; it
/// contains no data to construct.
pub trait ILElement:
    // Standard clone
    Clone +
    // Standard destruction
    Drop {
    type Element;
    type VecType;
    type IntType;


    // TODO: Заменить на стандартный трейт Clone

    /// 🇷🇺 Клонирование объекта, но чтобы он ничего не содержал
    /// 🇬🇧 Clone the object, but have it contain nothing
    fn clone_empty(&self) -> Self::Element;

    /// 🇷🇺 Клонирует параметры элемента, оставляет вектор инициализированным в 0
    /// 🇬🇧 Clones the element's parameters, leaves vector initialized to 0. 
    fn clone_parameters_only(&self) -> Self::Element;

    /// 🇷🇺 Клонирует элемент с параметрами и шумом для вектора
    /// 🇬🇧 Clones the element with parameters and with noise for the vector
    fn clone_with_noise(
        &self,
        dgg: &DiscreteGaussianGeneratorImpl<Self::VecType>,
        format: Format,
    ) -> Self::Element;

    // 🇷🇺 Операторы присваивания
    // 🇬🇧 Assignment operators

    /// 🇷🇺 Оператор присваивания, копирующий элементы.
    /// 🇬🇧 Assignment operator that copies elements.
    fn assign(&self, rhs: &Self::Element) -> &Self::Element;

    /// 🇷🇺 Оператор присваивания, копирующий элементы.
    /// 🇬🇧 Assignment operator that copies elements.
    fn assign_move(&self, rhs: &Self::Element) -> &Self::Element;

    /// 🇷🇺 Оператор присваивания, копирующий элементы.
    /// 🇬🇧 Assignment operator that copies elements.
    fn assign_initializer_list(&self, rhs: Vec<u64>) -> &Self::Element;

    // 🇷🇺 Функции получения данных
    // 🇬🇧 GETTERS

    /// 🇷🇺 Получение формата элемента. Формат - COEFFICIENT или EVALUATION
    /// 🇬🇧 Get format of the element. Format is either COEFFICIENT or EVALUATION
    fn get_format(&self) -> Format;

    /// 🇷🇺 Получение длины элемента.
    /// 🇬🇧 Get the length of the element.
    fn get_length(&self) -> usize;

    /// 🇷🇺 Получить модуль элемента
    /// 🇬🇧 Get modulus of the element
    fn get_modulus(&self) -> &Self::IntType;

    /// 🇷🇺 Получение значений для элемента
    /// 🇬🇧 Get the values for the element
    fn get_values(&self) -> &Self::VecType;

    /// 🇷🇺 Получение циклотомического порядка
    /// 🇬🇧 Get the cyclotomic order
    fn get_cyclotomic_order(&self) -> usize;

    /// 🇷🇺 Получение значения в элементе по индексу. Это реализовано только для некоторых 
    /// производных объектов, поэтому по умолчанию  выбрасывается исключение.
    /// 🇬🇧  Gets the Value in the Element that is At Index and returns it.
    /// This is only implemented for some derived classes, so the default implementation 
    /// throws an exception
    fn at(&self, i: usize) -> &Self::IntType;
    fn at_mut(&mut self, i: usize) -> &mut Self::IntType;
    fn index(&self, i: usize) -> &Self::IntType;
    fn index_mut(&mut self, i: usize) -> &mut Self::IntType;

    //  virtual NativePoly DecryptionCRTInterpolate(PlaintextModulus ptm) const
    //= 0;

    // OPERATORS
    // Neg + Add + Sub + AddAssign + SubAssign + Eq + Ne
    /**
     * @brief Unary negation on a lattice
     * @return -lattice
    */
    fn neg(&self) -> Self::Element;

    /**
     * @brief Scalar addition - add an element to the first index only.
     * This operation is only allowed in COEFFICIENT format.
     *
     * @param &element is the element to add entry-wise.
     * @return is the return of the addition operation.
    */
    fn plus(&self, element: &Self::IntType) -> Self::Element;

    /**
     * @brief Scalar subtraction - subtract an element frp, all entries.
     *
     * @param &element is the element to subtract entry-wise.
     * @return is the return value of the minus operation.
    */
    fn minus(&self, element: &Self::IntType) -> Self::Element;

    /**
     * @brief Scalar multiplication - multiply all entries.
     *
     * @param &element is the element to multiply entry-wise.
     * @return is the return value of the times operation.
    */
    fn times(&self, element: &Self::IntType) -> Self::Element;

    /**
     * @brief Scalar multiplication - mulltiply by a signed integer
     *
     * @param &element is the element to multiply entry-wise.
     * @return is the return value of the times operation.
    */
    fn times_signed(&self, element: i64) -> Self::Element;

    /**
     * @brief Performs an addition operation and returns the result.
     *
     * @param &element is the element to add with.
     * @return is the result of the addition.
    */
    fn plus_element(&self, element: &Self::Element) -> Self::Element;

    /**
     * @brief Performs a subtraction operation and returns the result.
     *
     * @param &element is the element to subtract with.
     * @return is the result of the subtraction.
    */
    fn minus_element(&self, element: &Self::Element) -> Self::Element;

    /**
     * @brief Performs a multiplication operation and returns the result.
     *
     * @param &element is the element to multiply with.
     * @return is the result of the multiplication.
    */
    fn times_element(&self, element: &Self::Element) -> Self::Element;

    // overloaded op= operators
    /**
     * @brief Performs += operation with a BigInteger and returns the result.
     *
     * @param &element is the element to add
     * @return is the result of the addition.
    */
    fn plus_assign(&mut self, element: &Self::IntType) -> &Self::Element;

    /**
     * @brief Performs -= operation with a BigInteger and returns the result.
     *
     * @param &element is the element to subtract
     * @return is the result of the addition.
    */
    fn minus_assign(&mut self, element: &Self::IntType) -> &Self::Element;

    /**
     * @brief Performs *= operation with a BigInteger and returns the result.
     *
     * @param &element is the element to multiply by
     * @return is the result of the multiplication.
    */
    fn times_assign(&mut self, element: &Self::IntType) -> &Self::Element;

    /**
     * @brief Performs an addition operation and returns the result.
     *
     * @param &element is the element to add
     * @return is the result of the addition.
    */
    fn plus_assign_element(&mut self, element: &Self::Element) -> &Self::Element;

    /**
     * @brief Performs an subtraction operation and returns the result.
     *
     * @param &element is the element to subtract
     * @return is the result of the addition.
    */
    fn minus_assign_element(&mut self, element: &Self::Element) -> &Self::Element;

    /**
     * @brief Performs an multiplication operation and returns the result.
     *
     * @param &element is the element to multiply by
     * @return is the result of the multiplication.
    */
    fn times_assign_element(&mut self, element: &Self::Element) -> &Self::Element;

    /**
     * @brief Equality operator.  Compares values of element to be compared to.
     * @param element the element to compare to.
    */
    fn eq(&self, element: &Self::Element) -> bool;

    /**
     * @brief Inequality operator.  Compares values of element to be compared to.
     * @param element the element to compare to.
    */
    fn ne(&self, element: &Self::Element) -> bool;

    /**
     * @brief Adds one to every entry of the Element, in place
    */
    fn add_element_one(&mut self);

    /**
     * @brief Performs an automorphism transform operation and returns the result.
     *
     * @param &i is the element to perform the automorphism transform with.
     * @return is the result of the automorphism transform.
    */
    fn automorphism_transform(&self, i: u32) -> Self::Element;

    /**
     * @brief Performs an automorphism transform operation using precomputed bit
     * reversal indices.
     *
     * @param &i is the element to perform the automorphism transform with.
     * @param &vec a vector with precomputed indices
     * @return is the result of the automorphism transform.
    */
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self::Element;

    /**
     * @brief Transpose the ring element using the automorphism operation
     *
     * @return is the result of the transposition.
    */
    fn transpose(&self) -> Self::Element;

    /**
     * @brief Write the element as \f$ \sum\limits{i=0}^{\lfloor {\log q/base}
     * \rfloor} {(base^i u_i)} \f$ and return the vector of \f$ \left\{u_0,
     * u_1,...,u_{\lfloor {\log q/base} \rfloor} \right\} \in R_{{base}^{\lceil
     * {\log q/base} \rceil}} \f$; This is used as a subroutine in the
     * relinearization procedure.
     *
     * @param baseBits is the number of bits in the base, i.e., base = 2^baseBits
     * @param evalModeAnswer - if true, convert the resultant polynomials to
     * evaluation mode
     * @result is the pointer where the base decomposition vector is stored
    */
    fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self::Element>;

    /**
     * @brief Scalar division followed by rounding operation - operation on all
     * entries.
     *
     * @param &q is the element to divide entry-wise.
     * @return is the return value of the divide, followed by rounding operation.
    */
    fn divide_and_round(&self, q: &Self::IntType) -> Self::Element;

    /**
     * @brief Determines if inverse exists
     *
     * @return true if there exists a multiplicative inverse.
    */
    fn inverse_exists(&self) -> bool;

    /**
     * @brief Returns the infinity norm, basically the largest value in the ring
     * element.
     *
     * @return the largest value in the ring element.
    */
    fn norm(&self) -> f64;

    /**
     * @brief Returns true if the vector is empty/ m_values==nullptr
     *
     * @return true if the vector is empty and all values nullptr.  false
     * otherwise.
    */
    fn is_empty(&self) -> bool;

    /**
     * @brief Make the element Sparse for SHE KeyGen operations.
     * Sets every index not equal to zero mod the wFactor to zero.
     *
     * @param &wFactor ratio between the original element's ring dimension and the
     * new ring dimension.
    */
    fn make_sparse(&mut self, w_factor: u32);

    /**
     * @brief Calculate Element mod 2
     *
     * @return result of performing a mod-2 operation on the element.
    */
    fn mod_by_two(&self) -> Self::Element;

    /**
     * @brief Calculate and return the Multiplicative Inverse of the element
     * @return the multiplicative inverse of the element, if it exists.
    */
    fn multiplicative_inverse(&self) -> Self::Element;

    /**
     * @brief Scalar multiplication followed by division and rounding operation -
     * operation on all entries.
     *
     * @param &p is the integer muliplicand.
     * @param &q is the integer divisor.
     * @return is the return value of the multiply, divide and followed by
     * rounding operation.
    */
    fn multiply_and_round(&self, p: &Self::IntType, q: &Self::IntType) -> Self::Element;

    /**
     * @brief Calculate a vector of elements by raising the base element to
     * successive powers
     *
     * @param baseBits
     * @return
    */
    fn powers_of_base(&self, base_bits: usize) -> Vec<Self::Element>;

    /**
     * @brief Mod - perform a modulus operation.
     * Does proper mapping of [-modulus/2, modulus/2) to [0, modulus).
     *
     * @param modulus is the modulus to use.
     * @return is the return value of the modulus.
    */
    fn modulo(&self, modulus: &Self::IntType) -> Self::Element;

    /**
     * @brief Switch modulus and adjust the values
     *
     * @param &modulus is the modulus to be set.
     * @param &rootOfUnity is the corresponding root of unity for the modulus
     * @param &modulusArb is the modulus used for arbitrary cyclotomics CRT
     * @param &rootOfUnityArb is the corresponding root of unity for the modulus
     * ASSUMPTION: This method assumes that the caller provides the correct
     * rootOfUnity for the modulus.
    */
    fn switch_modulus(
        &mut self,
        modulus: &Self::IntType,
        root_of_unity: &Self::IntType,
        modulus_arb: &Self::IntType,
        root_of_unity_arb: &Self::IntType,
    );

    /**
     * @brief onvert from Coefficient to CRT or vice versa; calls FFT and inverse FFT.
    */
    fn switch_format(&mut self);

    /**
     * @brief Sets the format/representation of the element.
     * @param format the format/representation to set.
    */
    fn set_format(&mut self, format: Format);
}
