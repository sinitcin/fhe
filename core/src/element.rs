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
    🇷🇺 Представляет и задает объекты элементов целочисленных решеток в OpenFHE
    🇬🇧 Represents and defines integer lattice element objects in OpenFHE
*/

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct DiscreteGaussianGeneratorImpl<X> {
    _phantom: std::marker::PhantomData<X>,
}

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
pub trait ILElement<Idx>:
    // 🇷🇺 Должен быть реализован стандартный типаж для клонирования
    // 🇬🇧 Standard clone
    Clone +

    // 🇷🇺 Оператор неравенства.
    // Параметр: element элемент, с которым нужно сравнить.
    // 🇬🇧 Inequality operator.  Compares values of element to be compared to.
    // Param element the element to compare to.

    // 🇷🇺 Оператор равенства.  Сравнивает значения элемента, с которым нужно сравнить.
    // Параметр: element элемент, с которым нужно сравнить.
    // 🇬🇧 Equality operator.  Compares values of element to be compared to.
    // Param element the element to compare to.
    Eq +

    // 🇷🇺 Унарное отрицание на решетке, возвращаемое значение: -решетка 
    // 🇬🇧 Unary negation on a lattice. Return -lattice
    Neg +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    //
    // 1) Скалярное сложение - добавление элемента только к первому индексу.
    // Эта операция допустима только в формате COEFFICIENT.
    // Параметр: element - элемент для добавления по индексу.
    // Результат: возврат операции сложения.
    // 
    // 2) Выполняет операцию сложения и возвращает результат.
    // Параметр: element - элемент для сложения.
    // Результат: значение сложения.
    // 
    // 🇬🇧 Must be implemented a couple of types of additions:
    // 
    // 1) Scalar addition - add an element to the first index only.
    // This operation is only allowed in COEFFICIENT format.
    // Parameter: element is the element to add entry-wise.
    // Return is the value of the addition operation.
    //
    // 2) Performs an addition operation and returns the result.
    // Parameter: element is the element to add with.
    // Return is the value of the addition.
    Add +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    // 1) Скалярное вычитание - вычитание элемента frp, все записи.
    // Parameter: element - элемент, из которого нужно вычесть все записи.
    // @return - возвращаемое значение операции вычитания.
    //
    // 2) Выполняет операцию вычитания и возвращает результат.
    // Параметр: element - элемент для вычитания.
    // Результат: значение вычитания.
    //
    // 🇬🇧 Must be implemented a couple of types of subtractions:
    // 1) Scalar subtraction - subtract an element frp, all entries.
    // Parameter: element is the element to subtract entry-wise.
    // Return is the value of the minus operation.
    //
    // 2) Performs a subtraction operation and returns the result.
    // Parameter: element is the element to subtract with.
    // Return is the value of the subtraction.
    Sub +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    //
    // 1) Скалярное умножение - перемножение всех записей.
    // Параметр: element - элемент, который нужно умножить на знаковое целое.
    // Результат: возвращаемое значение операции умножения.
    //
    // 2) Выполняет операцию умножения и возвращает результат.
    // Параметр: element - элемент для умножения.
    // Результат: значение умножения.
    //
    // 🇬🇧 Must be a couple of implementations:
    //
    // 1) Scalar multiplication - multiply all entries.
    // Parameter: element is the element to multiply entry-wise.
    // Return is the value of the times operation.
    //
    // 2) Performs a multiplication operation and returns the result.
    // Parameter: element is the element to multiply with.
    // Return is the value of the multiplication.
    Mul +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    //
    // 1) Выполняет операцию += с BigInteger и возвращает результат.
    // Параметр: element - элемент для добавления.
    // Результат: результат сложения.
    //
    // 2) Выполняет операцию сложения и возвращает результат.
    // Параметр: element - элемент для добавления.
    // Результат: значение сложения.
    //
    // 🇬🇧 Must be a couple of implementations:
    //
    // 1) Performs += operation with a BigInteger and returns the result.
    // Parameter: element is the element to add
    // Return is the value of the addition.
    //
    // 2) Performs an addition operation and returns the result.
    // Parameter: element is the element to add
    // Return is the value of the addition.
    AddAssign +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    //
    // 1) Выполняет операцию -= с BigInteger и возвращает результат.
    // Параметр: element - элемент для вычитания.
    // Результат: значение сложения.
    //
    // 2) Выполняет операцию вычитания и возвращает результат.
    // Параметр: element - элемент для вычитания.
    // Результат: значение сложения.
    //
    // 🇬🇧 Must be a couple of implementations:
    //
    // 1) Performs -= operation with a BigInteger and returns the result.
    // Parameter: element is the element to subtract
    // Return is the value of the addition.
    //
    // 2) Performs an subtraction operation and returns the result.
    // Parameter: element is the element to subtract
    // Return is the value of the addition.
    SubAssign +

    // 🇷🇺 Должно быть реализовано несколько вариантов:
    //
    // 1) Выполняет операцию *= с BigInteger и возвращает результат.
    // Параметр: element - элемент, на который нужно умножить.
    // Результат: значение умножения.
    //
    // 2) Выполняет операцию умножения и возвращает результат.
    //
    // Параметр: element - элемент, на который нужно умножить.
    // Результат: значение умножения.
    //
    // 🇬🇧 Must be a couple of implementations:
    //
    // 1) Performs *= operation with a BigInteger and returns the result.
    // Parameter: element is the element to multiply by
    // Return is the value of the multiplication.
    //
    // 2) Performs an multiplication operation and returns the result.
    // Parameter: element is the element to multiply by
    // Return is the value of the multiplication.
    MulAssign +

    // 🇷🇺 Получение значения в элементе по индексу. Это реализовано только для некоторых 
    // производных объектов, поэтому по умолчанию  выбрасывается исключение.
    // 🇬🇧 Gets the Value in the Element that is At Index and returns it.
    // This is only implemented for some derived classes, so the default implementation 
    // throws an exception
    Index<Idx> + IndexMut<Idx>
    where Idx: ?Sized,
    {
    type Element;
    type VecType;
    type IntType;

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
    //  virtual NativePoly DecryptionCRTInterpolate(PlaintextModulus ptm) const
    //= 0;

    /// 🇷🇺 Добавляет единицу к каждой записи элемента, вместо
    /// 🇬🇧 Adds one to every entry of the Element, in place
    fn add_element_one(&mut self);

    /// 🇷🇺 Выполняет операцию преобразования автоморфизма и возвращает результат.
    /// Параметр:       &i - элемент, с которым выполняется автоморфное преобразование.
    /// Результат:      результат автоморфного преобразования.
    /// 
    /// 🇬🇧 Performs an automorphism transform operation and returns the result.
    /// Param &i is the element to perform the automorphism transform with.
    /// Return is the result of the automorphism transform.
    fn automorphism_transform(&self, i: u32) -> Self::Element;

    /// 🇷🇺 Выполняет операцию преобразования автоморфизма с использованием предварительно вычисленных индексов разворота битов.
    /// Параметры:
    ///     &i - элемент, с которым выполняется автоморфное преобразование.
    ///     &vec вектор с предварительно вычисленными индексами
    /// Результат:    результат автоморфного преобразования.
    /// 
    /// 🇬🇧 Performs an automorphism transform operation using precomputed bit reversal indices.
    ///
    /// Param &i is the element to perform the automorphism transform with.
    /// Param &vec a vector with precomputed indices
    /// Return is the result of the automorphism transform.
    fn automorphism_transform_with_vec(&self, i: u32, vec: &Vec<u32>) -> Self::Element;

    /// 🇷🇺 Транспонирование элемента кольца с помощью операции автоморфизма
    /// Результат:    результат транспонирования.
    /// 
    /// 🇬🇧 Transpose the ring element using the automorphism operation
    /// Return is the result of the transposition.
    fn transpose(&self) -> Self::Element;

    /// 🇷🇺 Запишем элемент как \f$ \sum\limits{i=0}^{\lfloor {\log q/base}
    /// \rfloor} {(base^i u_i)} \f$ и вернуть вектор \f$ \left\{u_0,
    /// u_1,...,u_{\lfloor {\log q/base} \rfloor} \right\} \in R_{{base}^{\lceil
    /// {\log q/base} \rceil}} \f$; Это используется как подпрограмма в процедуре
    /// процедуре реалинеаризации.
    /// 
    /// Параметры:
    ///     baseBits - количество битов в базе, т.е. base = 2^baseBits
    ///     evalModeAnswer - если true, преобразовать результирующие полиномы в
    ///     режим оценки
    /// Результат:    указатель, в котором хранится вектор разложения базы
    /// 
    /// 🇬🇧  Write the element as \f$ \sum\limits{i=0}^{\lfloor {\log q/base}
    /// \rfloor} {(base^i u_i)} \f$ and return the vector of \f$ \left\{u_0,
    /// u_1,...,u_{\lfloor {\log q/base} \rfloor} \right\} \in R_{{base}^{\lceil
    /// {\log q/base} \rceil}} \f$; This is used as a subroutine in the
    /// relinearization procedure.
    ///
    /// @param baseBits is the number of bits in the base, i.e., base = 2^baseBits
    /// @param evalModeAnswer - if true, convert the resultant polynomials to
    /// evaluation mode
    /// @result is the pointer where the base decomposition vector is stored
    fn base_decompose(&self, base_bits: usize, eval_mode_answer: bool) -> Vec<Self::Element>;

    /// 🇷🇺 Скалярное деление с последующей операцией округления - операция над всеми записями. 
    /// Параметр &q - элемент, на который нужно разделить запись.
    /// Результат: возвращаемое значение операции деления с последующим округлением.
    ///
    /// 🇬🇧  Scalar division followed by rounding operation - operation on all entries.
    /// @param &q is the element to divide entry-wise.
    /// @return is the return value of the divide, followed by rounding operation.
    fn divide_and_round(&self, q: &Self::IntType) -> Self::Element;

    /// 🇷🇺 Определяет, существует ли обратная величина.
    /// Результат: true, если существует мультипликативная обратная величина.
    /// 
    /// 🇬🇧 Determines if inverse exists
    /// @return true if there exists a multiplicative inverse.
    fn inverse_exists(&self) -> bool;

    /// 🇷🇺 Возвращает норму бесконечности, в основном наибольшее значение в элементе кольца
    /// Результат: наибольшее значение в элементе кольца.
    /// 
    /// 🇬🇧 Returns the infinity norm, basically the largest value in the ring element.
    /// @return the largest value in the ring element.
    fn norm(&self) -> f64;

    /// 🇷🇺 Возвращает true, если вектор пуст/ m_values==nullptr
    /// Результат: true, если вектор пуст и все значения nullptr, иначе false.
    /// 
    /// 🇬🇧 Returns true if the vector is empty/ m_values==nullptr
    /// @return true if the vector is empty and all values nullptr. false otherwise.
    fn is_empty(&self) -> bool;

    /// 🇷🇺 Сделайте элемент разреженным для операций SHE KeyGen.
    /// Устанавливает каждый индекс, не равный нулю, по модулю wFactor в ноль.
    /// Параметр &wFactor отношение между размерами кольца исходного элемента и новым размером кольца.
    /// 
    /// 🇬🇧 Make the element Sparse for SHE KeyGen operations.
    /// Sets every index not equal to zero mod the wFactor to zero.
    /// @param &wFactor ratio between the original element's ring dimension and the new ring dimension.
    fn make_sparse(&mut self, w_factor: u32);

    /// 🇷🇺 Вычислить элемент по модулю 2
    /// Результат: Результат выполнения операции mod-2 над элементом.
    /// 
    /// 🇬🇧 Calculate Element mod 2
    /// @return result of performing a mod-2 operation on the element.
    fn mod_by_two(&self) -> Self::Element;

    /// 🇷🇺 Вычисление и возврат мультипликативной обратной величины элемента
    /// Результат: мультипликативная обратная величина элемента, если она существует.
    /// 
    /// 🇬🇧 Calculate and return the Multiplicative Inverse of the element
    /// @return the multiplicative inverse of the element, if it exists.
    fn multiplicative_inverse(&self) -> Self::Element;

    /// 🇷🇺 Скалярное умножение с последующим делением и операцией округления - операция над всеми записями.
    /// * Параметры:
    ///     &p - целое число мультипликатора.
    ///     &q - целочисленный делитель.
    /// Результат: возвращаемое значение операции умножения, деления и последующего округления.
    /// 
    /// 🇬🇧 Scalar multiplication followed by division and rounding operation - operation on all entries.
    /// @param &p is the integer multiplicand.
    /// @param &q is the integer divisor.
    /// @return is the return value of the multiply, divide and followed by
    /// rounding operation.
    fn multiply_and_round(&self, p: &Self::IntType, q: &Self::IntType) -> Self::Element;

    /// 🇷🇺 Вычислить вектор элементов путем возведения базового элемента в последовательные степени
    /// Параметр baseBits
    /// Результат: вектор элементов
    /// 
    /// 🇬🇧 Calculate a vector of elements by raising the base element to successive powers
    /// @param baseBits
    /// @return
    fn powers_of_base(&self, base_bits: usize) -> Vec<Self::Element>;

    /// 🇷🇺 Mod - выполняет операцию модуляции. Выполняет корректное отображение [-modulus/2, modulus/2) на [0, modulus)]
    /// Параметр modulus - используемый модуль.
    /// Результат - возвращаемое значение модуля.
    /// 
    /// 🇬🇧 Mod - perform a modulus operation.
    /// Does proper mapping of [-modulus/2, modulus/2) to [0, modulus).
    /// @param modulus is the modulus to use.
    /// @return is the return value of the modulus.
    fn modulo(&self, modulus: &Self::IntType) -> Self::Element;

     /// 🇷🇺 Переключение модуля и настройка значений
     /// Параметры: 
     ///    &modulus - модуль, который нужно установить.
     ///    &rootOfUnity - соответствующий корень единственности для модуля.
     ///    &modulusArb - модуль, используемый для произвольного циклотомического CRT
     ///    &rootOfUnityArb - соответствующий корень из единства для модуля
     /// ПРЕДПОЛОЖЕНИЕ: Этот метод предполагает, что вызывающая сторона предоставила правильный rootOfUnity для модуля.
     /// 
     /// 🇬🇧 Switch modulus and adjust the values 
     /// @param &modulus is the modulus to be set.
     /// @param &rootOfUnity is the corresponding root of unity for the modulus
     /// @param &modulusArb is the modulus used for arbitrary cyclotomic CRT
     /// @param &rootOfUnityArb is the corresponding root of unity for the modulus
     /// ASSUMPTION: This method assumes that the caller provides the correct
     /// rootOfUnity for the modulus.
    fn switch_modulus(
        &mut self,
        modulus: &Self::IntType,
        root_of_unity: &Self::IntType,
        modulus_arb: &Self::IntType,
        root_of_unity_arb: &Self::IntType,
    );

    /// 🇷🇺 Преобразование из Coefficient в CRT или наоборот; вызов FFT и обратного FFT.
    /// 🇬🇧 Convert from Coefficient to CRT or vice versa; calls FFT and inverse FFT.
    fn switch_format(&mut self);

    /// 🇷🇺 Устанавливает формат/представление элемента.
    /// Параметр format формат/представление для установки.
    /// 🇬🇧 Sets the format/representation of the element.
    /// @param format the format/representation to set.
    fn set_format(&mut self, format: Format);
}
