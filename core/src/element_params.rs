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
    🇷🇺 Базовый объект для параметров элемента решетки
    🇬🇧 Base class for parameters for a lattice element
*/

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::default::Default;
use std::fmt::{Debug, Display, Formatter};

/// 🇷🇺 Объект-обертка для хранения параметров Element
/// 🇬🇧 Wrapper class to hold the parameters for Element types and their inheritors
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ElemParams<IntegerType>
where
    IntegerType: Default + Clone,
{
    m_ring_dimension: u32,
    m_cyclotomic_order: u32,
    m_ciphertext_modulus: IntegerType,
    m_root_of_unity: IntegerType,
    m_big_ciphertext_modulus: IntegerType,
    m_big_root_of_unity: IntegerType,
}

impl<IntegerType> ElemParams<IntegerType>
where
    IntegerType: Default + Clone,
{
    /// 🇷🇺 Простые методы-конструкторы, которые принимают на вход
    /// - корень из единственного числа
    /// - большой корень из единственного числа
    /// - циклотомический порядок
    /// - модуль шифртекста и модуль большого шифртекста
    /// Используется для операций побитовой упаковки.
    /// Параметры:
    /// * order - циклотомический порядок, обернутый набором параметров.
    /// * ctModulus - модуль шифртекста, обернутый набором параметров.
    /// * rUnity - к корень из единственного числа.
    /// * bigCtModulus - большой модуль шифртекста, используемый для упаковки битовых операций.
    /// * bigRUnity - большой корень из единства, используемый для операций упаковки битов.
    ///
    /// 🇬🇧 Simple constructor method that takes as input root of unity, big root of unity,
    /// cyclotomic order and the ciphertext modulus and big ciphertext Modulus.
    /// This is used for bit-packing operations.
    /// Parameters:
    /// * order the cyclotomic order wrapped by the parameter set.
    /// * ctModulus the ciphertext modulus wrapped by the parameter set.
    /// * rUnity the root of unity.
    /// * bigCtModulus the big ciphertext modulus used for bit packing operations.
    /// * bigRUnity the big root of unity used for bit packing operations.

    pub fn new(cyclotomic_order: u32, ciphertext_modulus: IntegerType) -> Self {
        let ring_dimension = Self::get_totient(cyclotomic_order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: cyclotomic_order,
            m_ciphertext_modulus: ciphertext_modulus,
            m_root_of_unity: IntegerType::default(),
            m_big_ciphertext_modulus: IntegerType::default(),
            m_big_root_of_unity: IntegerType::default(),
        }
    }

    /// 🇷🇺 Предыдущее название: `new_with_runity`
    /// 🇬🇧 The previous name is `new_with_runity`
    pub fn new_with_root_of_unity(
        cyclotomic_order: u32,
        ciphertext_modulus: IntegerType,
        root_of_unity: IntegerType,
    ) -> Self {
        let ring_dimension = Self::get_totient(cyclotomic_order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: cyclotomic_order,
            m_ciphertext_modulus: ciphertext_modulus,
            m_root_of_unity: root_of_unity,
            m_big_ciphertext_modulus: IntegerType::default(),
            m_big_root_of_unity: IntegerType::default(),
        }
    }

    pub fn new_with_big_modulus(
        cyclotomic_order: u32,
        ciphertext_modulus: IntegerType,
        root_of_unity: IntegerType,
        big_ciphertext_modulus: IntegerType,
        big_root_of_unity: IntegerType,
    ) -> Self {
        let ring_dimension = Self::get_totient(cyclotomic_order);
        ElemParams {
            m_ring_dimension: ring_dimension,
            m_cyclotomic_order: cyclotomic_order,
            m_ciphertext_modulus: ciphertext_modulus,
            m_root_of_unity: root_of_unity,
            m_big_ciphertext_modulus: big_ciphertext_modulus,
            m_big_root_of_unity: big_root_of_unity,
        }
    }

    /// 🇷🇺 Простой метод получения циклотомического порядка.
    /// 🇬🇧 Simple getter method for cyclotomic order.
    pub fn get_cyclotomic_order(&self) -> u32 {
        self.m_cyclotomic_order
    }

    /// 🇷🇺 Простой метод получения размерности кольца. Размерность кольца - это оценка функции Эйлера циклотомического порядка.
    /// 🇬🇧 Simple ring dimension getter method. The ring dimension is the evaluation of the totient function of the cyclotomic order.
    pub fn get_ring_dimension(&self) -> u32 {
        self.m_ring_dimension
    }

    /// 🇷🇺 Простой метод получения модуля шифртекста, а не большого модуля шифртекста.
    /// 🇬🇧 Simple getter method for the ciphertext modulus, not the big ciphertext modulus.
    pub fn get_modulus(&self) -> &IntegerType {
        &self.m_ciphertext_modulus
    }

    /// 🇷🇺 Простой метод получения большого модуля шифротекста. Это актуально не для всех приложений.
    /// 🇬🇧 Simpler getter method for the big ciphertext modulus. This is not relevant for all applications.
    pub fn get_big_modulus(&self) -> &IntegerType {
        &self.m_big_ciphertext_modulus
    }

    /// 🇷🇺 Простой метод получения корня из единицы (не путать с большим корнем из единицы).
    /// 🇬🇧 Simple getter method for the root of unity, not the big root of unity.
    pub fn get_root_of_unity(&self) -> &IntegerType {
        &self.m_root_of_unity
    }

    /// 🇷🇺 Получение большого корня из единицы
    /// 🇬🇧 Simple getter method for the big root of unity.
    pub fn get_big_root_of_unity(&self) -> &IntegerType {
        &self.m_big_root_of_unity
    }

    /// 🇷🇺 Функция Эйлера
    /// 🇬🇧 Totient function
    fn get_totient(_cyclotomic_order: u32) -> u32 {
        // TODO: Implement GetTotient function
        unimplemented!()
    }
}

impl<IntegerType> Debug for ElemParams<IntegerType>
where
    IntegerType: Default + Debug + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[m={} n={} q={} ru={} bigq={} bigru={}]",
            self.m_cyclotomic_order,
            self.m_ring_dimension,
            self.m_ciphertext_modulus,
            self.m_root_of_unity,
            self.m_big_ciphertext_modulus,
            self.m_big_root_of_unity
        )
    }
}

impl<IntegerType: Default + Clone> Default for ElemParams<IntegerType> {
    fn default() -> Self {
        ElemParams {
            m_ring_dimension: 0,
            m_cyclotomic_order: 0,
            m_ciphertext_modulus: IntegerType::default(),
            m_root_of_unity: IntegerType::default(),
            m_big_ciphertext_modulus: IntegerType::default(),
            m_big_root_of_unity: IntegerType::default(),
        }
    }
}
