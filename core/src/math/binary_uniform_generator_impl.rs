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
 This code provides generation of uniform distribution of binary values (modulus 2). Discrete uniform generator relies on
 the built-in Rust generator for 32-bit unsigned integers defined in rand crate
*/

use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;

pub struct BinaryUniformGenerator;

impl BinaryUniformGenerator {
    fn generate_integer() -> u32 {
        let mut rng = thread_rng();
        let distribution = Bernoulli::new(0.5).unwrap();
        if distribution.sample(&mut rng) {
            1
        } else {
            0
        }
    }

    pub fn generate_vector(size: usize) -> Vec<u32> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(Self::generate_integer());
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vector() {
        let size = 10;
        let vector = BinaryUniformGenerator::generate_vector(size);
        dbg!(&vector);
        assert_eq!(vector.len(), size);
    }
}
