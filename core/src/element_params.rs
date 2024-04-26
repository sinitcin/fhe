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
  base class for parameters for a lattice element
 */

use std::fmt;
use serde::{Serialize, Deserialize};
use std::default::Default;
use std::clone::Clone;

/**
 * @class ElemParams
 * @file elemparams.h
 * @brief Wrapper class to hold the parameters for Element types and their
 * inheritors.
 */
#[derive(Serialize, Deserialize)]
pub struct ElemParams<IntegerType: Default + Clone> {
    m_ringDimension: u32,
    m_cyclotomicOrder: u32,
    m_ciphertextModulus: IntegerType,
    m_rootOfUnity: IntegerType,
    m_bigCiphertextModulus: IntegerType,
    m_bigRootOfUnity: IntegerType,
}

impl<IntegerType> ElemParams<IntegerType> {
    pub fn new(order: u32, ctModulus: IntegerType) -> Self {
        let ringDimension = Self::get_totient(order);
        ElemParams {
            m_ringDimension: ringDimension,
            m_cyclotomicOrder: order,
            m_ciphertextModulus: ctModulus,
            m_rootOfUnity: IntegerType::default(),
            m_bigCiphertextModulus: IntegerType::default(),
            m_bigRootOfUnity: IntegerType::default(),
        }
    }

    pub fn new_with_runity(order: u32, ctModulus: IntegerType, rUnity: IntegerType) -> Self {
        let ringDimension = Self::get_totient(order);
        ElemParams {
            m_ringDimension: ringDimension,
            m_cyclotomicOrder: order,
            m_ciphertextModulus: ctModulus,
            m_rootOfUnity: rUnity,
            m_bigCiphertextModulus: IntegerType::default(),
            m_bigRootOfUnity: IntegerType::default(),
        }
    }

    pub fn new_with_big_modulus(order: u32, ctModulus: IntegerType, rUnity: IntegerType, bigCtModulus: IntegerType, bigRUnity: IntegerType) -> Self {
        let ringDimension = Self::get_totient(order);
        ElemParams {
            m_ringDimension: ringDimension,
            m_cyclotomicOrder: order,
            m_ciphertextModulus: ctModulus,
            m_rootOfUnity: rUnity,
            m_bigCiphertextModulus: bigCtModulus,
            m_bigRootOfUnity: bigRUnity,
        }
    }

    pub fn get_cyclotomic_order(&self) -> u32 {
        self.m_cyclotomicOrder
    }

    pub fn get_ring_dimension(&self) -> u32 {
        self.m_ringDimension
    }

    pub fn get_modulus(&self) -> &IntegerType {
        &self.m_ciphertextModulus
    }

    pub fn get_big_modulus(&self) -> &IntegerType {
        &self.m_bigCiphertextModulus
    }

    pub fn get_root_of_unity(&self) -> &IntegerType {
        &self.m_rootOfUnity
    }

    pub fn get_big_root_of_unity(&self) -> &IntegerType {
        &self.m_bigRootOfUnity
    }

    fn get_totient(order: u32) -> u32 {
        // TODO: Implement GetTotient function
        unimplemented!()
    }
}

impl<IntegerType: fmt::Debug> fmt::Debug for ElemParams<IntegerType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[m={} n={} q={} ru={} bigq={} bigru={}]", self.m_cyclotomicOrder, self.m_ringDimension, self.m_ciphertextModulus, self.m_rootOfUnity, self.m_bigCiphertextModulus, self.m_bigRootOfUnity)
    }
}

impl<IntegerType> PartialEq for ElemParams<IntegerType> {
    fn eq(&self, other: &Self) -> bool {
        self.m_ringDimension == other.m_ringDimension &&
        self.m_cyclotomicOrder == other.m_cyclotomicOrder &&
        self.m_ciphertextModulus == other.m_ciphertextModulus &&
        self.m_rootOfUnity == other.m_rootOfUnity &&
        self.m_bigCiphertextModulus == other.m_bigCiphertextModulus &&
        self.m_bigRootOfUnity == other.m_bigRootOfUnity
    }
}

impl<IntegerType> Eq for ElemParams<IntegerType> {}

impl<IntegerType> Clone for ElemParams<IntegerType> {
    fn clone(&self) -> Self {
        ElemParams {
            m_ringDimension: self.m_ringDimension,
            m_cyclotomicOrder: self.m_cyclotomicOrder,
            m_ciphertextModulus: self.m_ciphertextModulus.clone(),
            m_rootOfUnity: self.m_rootOfUnity.clone(),
            m_bigCiphertextModulus: self.m_bigCiphertextModulus.clone(),
            m_bigRootOfUnity: self.m_bigRootOfUnity.clone(),
        }
    }
}

impl<IntegerType: Clone> Default for ElemParams<IntegerType> {
    fn default() -> Self {
        ElemParams {
            m_ringDimension: 0,
            m_cyclotomicOrder: 0,
            m_ciphertextModulus: IntegerType::default(),
            m_rootOfUnity: IntegerType::default(),
            m_bigCiphertextModulus: IntegerType::default(),
            m_bigRootOfUnity: IntegerType::default(),
        }
    }
}


