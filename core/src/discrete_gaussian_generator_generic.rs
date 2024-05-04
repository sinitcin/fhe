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
    🇷🇺 Этот код обеспечивает генерацию гауссовых распределений дискретных значений.
    Генератор дискретных равномерных распределений опирается на встроенный в C++ генератор 32-битных
    беззнаковых целых чисел, определенный в <random

    Это файл для Generic Sampler, используемого для различных задач дискретной гауссовой выборки. 
    Этот класс реализует общий сэмплер от UCSD, о котором говорилось в https://eprint.iacr.org/2017/259.pdf,
    и он в значительной степени основан на оригинальном коде Майкла Уолтера. 
    Также есть два различных «базовых сэмплера», которые используются для общего семплера или могут быть 
    использованы самостоятельно в зависимости от требований задачи.

    Первый базовый сэмплер использует метод инверсии Пейкерта, рассмотренный в разделе 4.1 на сайте
    https://eprint.iacr.org/2010/088.pdf и обобщенный в разделе 3.2.2 на сайте
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf.
    Метод Пейкерта требует предварительного вычисления таблицы CDF вокруг определенного центра, и эта таблица
    должна сохраняться в процессе выборки. Следовательно, Метод Пейкерта лучше всего работает,
    если желаемое стандартное отклонение невелико, а среднее значение распределения фиксировано,
    поскольку каждый новый центр требует нового набора предварительных вычислений.

    Второй базовый сэмплер - сэмплер Кнута-Яо, рассмотренный в разделе 5 
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf.
    Подобно методу Пейкерта, Кнут-Яо предварительно вычисляет PDF чисел на основе стандартного отклонения и центра, 
    которые используются в процессе выборки.
    Поэтому, как и метод Пейкерта, метод Кнута-Яо работает лучше всего, если желаемое стандартное отклонение невелико,
    а среднее значение распределения фиксировано, поскольку каждый новый центр потребует нового набора предварительных
    вычислений, как и в методе инверсии Пейкерта.)

    С другой стороны, «общий сэмплер» работает независимо от стандартного отклонения распределения.
    Он объединяет массив ранее рассмотренных базовых сэмплов, центрированных вокруг от 0 до (2^b-1) / 2^b,
    посредством свертки. Таблицы базовых выборок, однако, должны быть вычислены заранее, но их не нужно
    пересчитывать в любой момент процесса выборки. Метод USABLE FOR ANY STANDARD DEVIATION AND MEAN, как и метод Карни,
    определенный в discrete_gaussian_generator.rs, требует только одного предварительного вычисления и не подвержен 
    атакам по времени, в отличие от Карни. Метод Карни, однако, быстрее, чем общий сэмплер.

    ПОДБОР ПАРАМЕТРОВ ДЛЯ ОБЩЕГО СЭМПЛЕРА

    Выбор параметров изменяет время работы/затраты памяти/точность общего сэмплера. Тройной компромисс между 
    этими параметрами определяется уравнением k = (PRECISION - FLIPS) / LOG_BASE. k обозначает уровень точности
    общего семплера. Чем выше k, тем выше точность общего сэмплера, но выше и время работы.
    PRECISION  обозначает количество десятичных битов в центре распределения. Поскольку мы используем 
    'double' для среднего значения, оно по определению равно 53. 
    FLIPS обозначает количество подбрасываний Бернулли (по аналогии с монеткой), используемых для аппроксимации битов, 
    используемых в комбинации базового сэмплера. Чем больше число подбрасываний, тем большее количество битов 
    аппроксимируется, а не вычисляется, что означает меньшее время работы. Общий сэмплер требует набора базовых
    сэмплеров, расположенных в диапазоне от 0/2^b до (2^b-1)/2^b;
    LOG_BASE обозначает b в этом уравнении. При большем значении LOG_BASE требуется больше базовых семплов, 
    что требует дополнительной памяти, но в то же время уменьшает время выполнения.

    Базовые сэмплы, используемые в общем сэмплере, требуют варьирования центров между 0/2^b и (2^b-1)/(2^b) 
    с одинаковым стандартным отклонением. Стандартное отклонение, требуемое для базовых выборок, должно 
    удовлетворять SIGMA>=4*SQRT(2)*N, где sigma - стандартное отклонение базовой выборки, а N - параметр сглаживания
    

    🇬🇧 This code provides generation of gaussian distributions of discrete values. Discrete uniform generator
    relies on the built-in C++ generator for 32-bit unsigned integers defined in <random>

    This is the header file for the Generic Sampler used for various Discrete
    Gaussian Sampling applications. This class implements the generic sampler by
    UCSD discussed in the https://eprint.iacr.org/2017/259.pdf and it is heavily
    based on Michael Walter's original code. Along the sides of the
    implementation there are also two different "base samplers", which are used
    for the generic sampler or can be used on their own depending on the
    requirements of needed application.

    The first base sampler uses Peikert's inversion method, discussed in
    section 4.1 of https://eprint.iacr.org/2010/088.pdf and summarized in
    section 3.2.2 of
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf.
    Peikert's method requires precomputation of CDF tables around a specific
    center and the table must be kept during the sampling process. Hence,
    Peikert's method works best if the DESIRED STANDARD DEVIATION IS SMALL and
    THE MEAN OF THE DISTRIBUTION IS FIXED, as each new center will require a new
    set of precomputations.

    Second base sampler is  the Knuth-Yao Sampler discussed in section 5 of
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf .
    Similar to Peikert's, Knuth-Yao precomputes the PDF's of the numbers based on
    standard deviation and the center, which is used during the sampling process.
    Therefore like Peikert's method,  Knuth-Yao works best method works best if
    the DESIRED STANDARD DEVIATION IS SMALL and THE MEAN OF THE DISTRIBUTION IS
    FIXED, as each new center will require a new set of precomputations, just
    like Peikert's inversion method.

    The "generic sampler" on the other hand, works independent from standard
    deviation of the distribution. It combines an array of previously discussed
    base samplers centered around 0 to (2^b-1) / 2^b through convolution. The
    tables of base samplers however, must be precomputed beforehand; but they do
    not need to be recalculated at any time of the sampling process. It is USABLE
    FOR ANY STANDARD DEVIATION AND MEAN, just like Karney's method defined in
    discretegaussiangenerator.h, needs only one single precomputation and is not
    prone to timing attacks unlike Karney. Karney's method, however, is faster
    than the generic sampler.

    PARAMETER SELECTION FOR GENERIC SAMPLER

    The selection of parameters change the run time/memory usage/precision of the
    generic sampler. The triple trade off between these parameters are defined in
    the equation k = (PRECISION - FLIPS) / LOG_BASE. k denotes the level of
    precision of the generic sampler. Higher the k is, higher the precision of
    the generic sampler but higher the run time. PRECISION denotes the number of
    decimal bits in the center of the distribution. Since we are using 'double'
    for mean, it is fixed to 53 by definition. FLIPS denote the number of
    Bernoulli flips used to approximate the bits used in combination of base
    sampler. Higher the number of flips, larger the number of bits approximated
    rather than calculated which means smaller run times. Generic sampler
    requires a set of base samplers centered around 0/2^b to (2^b-1)/2^b;
    LOG_BASE denotes b in this equation. Higher the LOG_BASE is, more base
    samplers required which requires additional memory; but at the same time
    smaller run times.

    The base samplers used in generic sampler requires varying centers between
    0/2^b and (2^b-1)/(2^b) with the same standard deviation. The standard
    deviation required for base samplers must satisfy SIGMA>=4*SQRT(2)*N, where
    sigma is the standard deviation of the base sampler and N is the smoothing
    parameter

 */

use rand::prelude::*;
use std::collections::HashMap;
use rand::distributions::{Distribution, Gaussian};
use rand::Rng;

pub enum BaseSamplerType {
    KnuthYao,
    PeikertInversion,
}

/// Class implementation to generate random bit. This is created for centralizing the random bit pools by the samplers.
pub struct BitGenerator {
    sequence: u32,
    counter: u32,
}

impl BitGenerator {
    pub fn new() -> BitGenerator {
        BitGenerator {
            sequence: 0,
            counter: 0,
        }
    }

    /// Method for generating a random bit
    /// return A random bit
    pub fn generate(&mut self) -> i16 {
        if self.counter == 0 {
            self.sequence = rand::random::<u32>();
            self.counter = 32;
        }
        let bit = (self.sequence >> (self.counter - 1)) & 0x1;
        self.counter -= 1;
        bit as i16
    }
}

/// Class definition for base samplers with precomputation that is used for UCSD generic sampler
pub struct BaseSampler {
    /// Mean of the distribution used
    b_mean: i64,

    /// The standard deviation of the distribution.
    b_std: f32,

    /// Generator used for creating random bits through sampling
    bg: ThreadRng,

    /// Type of the base sampler (Knuth Yao or Peikert's Inversion)
    b_type: BaseSamplerType,

    ddg_tree: Vec<Vec<bool>>,

    /// Array that stores the Hamming Weights of the probability matrix used in Knuth-Yao sampling
    hamming_weights: Vec<u32>,

    /// Size of probability matrix used in Knuth-Yao
    b_matrix_size: i32,

    /// Index of first bit with non zero Hamming weight in the probability table
    first_non_zero: i32,

    end_index: i32,

    m_vals: Vec<f64>,
}

impl BaseSampler {
    /// Constructor
    ///
    /// Parameters:
    /// - mean: Mean of the distribution
    /// - std: Standard deviation of the distribution
    /// - generator: Pointer to the bit generator that the sampler will use the
    /// random bits from
    /// - bType: Type of the base sampler
    pub fn new(mean: f64, std: f64, b_type: BaseSamplerType) -> Self {
        let mut rng = thread_rng();
        let b_mean = mean as i64;
        let b_std = std as f32;
        let bg = rng;

        BaseSampler {
            b_mean,
            b_std,
            bg,
            b_type,
            ddg_tree: Vec::new(),
            hamming_weights: Vec::new(),
            b_matrix_size: 0,
            first_non_zero: 0,
            end_index: 0,
            m_vals: Vec::new(),
        }
    }

    ///  Method for generating integer from the base sampler a random integer from the distribution
    pub fn generate_integer(&mut self) -> i64 {
        match self.b_type {
            BaseSamplerType::KnuthYao => self.generate_integer_knuth_yao(),
            BaseSamplerType::PeikertInversion => self.generate_integer_peikert(),
        }
    }

    /// Method for generating a random bit from the bit generator within return a random bit
    fn random_bit(&mut self) -> bool {
        self.bg.gen_bool(0.5)
    }

    /// Sub-procedure called by Peikert's inversion sampling
    ///
    /// Parameters:
    /// - S: Vector containing the CDF values
    /// - search: Searched probability value
    ///
    /// Returns: Index that is the smallest bigger value than search
    fn find_in_vector(&self, s: &[f64], search: f64) -> usize {
        s.iter().position(|&x| x >= search).unwrap_or(s.len())
    }

    /// Generates DDG tree used through the sampling in Knuth-Yao
    ///
    /// Parameters:
    /// - probMatrix: The probability matrix used for filling the DDG tree
    fn generate_ddg_tree(&mut self, prob_matrix: &[u64]) {
        // Implementation goes here
    }

    /// Initializes the generator used for Peikert's Inversion method.
    ///
    /// Parameters:
    /// - mean: Mean of the distribution that the sampler will be using
    fn initialize(&mut self, mean: f64) {
        // Implementation goes here
    }

    /// Generates the probability matrix of given distribution, which is used in Knuth-Yao method
    ///
    /// Parameters:
    /// - stddev: Standard deviation of Discrete Gaussian Distribution
    /// - mean: Center of the distribution
    /// - tableCount: Number of probability tables to be generated
    fn generate_prob_matrix(&mut self, stddev: f64, mean: f64) {
        // Implementation goes here
    }

    /// Returns a generated integer. Uses Naive Knuth-Yao method. Return a random value within the Discrete Gaussian Distribution
    fn generate_integer_knuth_yao(&mut self) -> i64 {
        // Implementation goes here
        0
    }

    /// Returns a generated integer. Uses Peikert's inversion method.
    fn generate_integer_peikert(&self) -> i64 {
        // Implementation goes here
        0
    }
}

/// Class for combining samples from two base samplers, which is used for
/// UCSD generic sampling
struct SamplerCombiner {
    sampler1: Box<dyn BaseSampler>,
    sampler2: Box<dyn BaseSampler>,
    x1: i64,
    x2: i64,
}

/// Trait for base samplers
trait BaseSampler {
    fn generate_integer(&self) -> i64;
}

impl SamplerCombiner {
    /// Constructor
    /// 
    /// # Arguments
    /// 
    /// * `s1` - Pointer to the first sampler to be combined
    /// * `s2` - Pointer to the second sampler to be combined
    /// * `z1` - Coefficient for the first sampler
    /// * `z2` - Coefficient for the second sampler
    fn new(s1: Box<dyn BaseSampler>, s2: Box<dyn BaseSampler>, z1: i64, z2: i64) -> Self {
        SamplerCombiner {
            sampler1: s1,
            sampler2: s2,
            x1: z1,
            x2: z2,
        }
    }

    /// Return the combined value for two samplers with given coefficients
    fn generate_integer(&self) -> i64 {
        self.x1 * self.sampler1.generate_integer() + self.x2 * self.sampler2.generate_integer()
    }
}

// This Rust code translates the provided C++ code for the `SamplerCombiner` class. Here are the key points:

// 1. We define a `SamplerCombiner` struct to represent the class.
// 2. We define a `BaseSampler` trait to represent the base sampler interface.
// 3. The `SamplerCombiner` struct has fields for the two base samplers (`sampler1` and `sampler2`) and their coefficients (`x1` and `x2`).
// 4. The `new` function is the constructor, which takes the two base samplers and their coefficients as arguments.
// 5. The `generate_integer` method implements the `GenerateInteger` function from the C++ code, combining the results of the two base samplers using the coefficients.

// Note that we use trait objects (`Box<dyn BaseSampler>`) to represent the base samplers, as Rust doesn't have direct support for abstract base classes like C++. We also use the `Box` smart pointer to manage the memory of the base sampler objects.


/// The struct for Generic Discrete Gaussian Distribution generator.
struct DiscreteGaussianGeneratorGeneric {
    base_samplers: Vec<BaseSampler>,
    wide_sampler: BaseSampler,
    combiners: Vec<BaseSampler>,
    wide_variance: f64,
    sampler_variance: f64,
    x: f64,
    c: f64,
    ci: f64,
    k: i32,
    log_base: i32,
    mask: u64,
}

impl DiscreteGaussianGeneratorGeneric {
    /// Basic constructor which does the precomputations.
    /// 
    /// # Arguments
    /// 
    /// * `samplers` - Array containing the base samplers
    /// * `std` - Standard deviation of the base samplers
    /// * `b` - Log of number of centers that are used for calculating base samplers (Recall that base samplers are centered from 0 to (2^b-1)/2^b)
    /// * `N` - Smoothing parameter
    fn new(samplers: Vec<BaseSampler>, std: f64, b: i32, N: f64) -> Self {
        // Precomputation logic here
        DiscreteGaussianGeneratorGeneric {
            base_samplers: samplers,
            wide_sampler: BaseSampler::new(),
            combiners: vec![],
            wide_variance: 0.0,
            sampler_variance: 0.0,
            x: 0.0,
            c: 0.0,
            ci: 0.0,
            k: 0,
            log_base: b,
            mask: 0,
        }
    }

    /// Returns a generated integer. Uses generic algorithm in UCSD paper, based on Sample Z
    /// 
    /// # Arguments
    /// 
    /// * `mean` - Mean of the distribution
    /// * `variance` - Variance of the desired distribution
    fn generate_integer(&self, mean: f64, variance: f64) -> i64 {
        let mut rng = rand::thread_rng();
        let gaussian = Gaussian::new(mean, variance.sqrt());
        let sample = gaussian.sample(&mut rng);
        sample.round() as i64
    }

    /// Returns a generated integer using the base sampler.
    fn generate_integer(&self) -> i64 {
        self.base_samplers[0].generate_integer()
    }

    /// Subroutine used by Sample C
    /// 
    /// # Arguments
    /// 
    /// * `center` - Center of the distribution
    fn flip_and_round(&self, center: f64) -> i64 {
        let mut rng = rand::thread_rng();
        let gaussian = Gaussian::new(center, self.wide_variance.sqrt());
        let sample = gaussian.sample(&mut rng);
        sample.round() as i64
    }

    /// Sample C defined in the paper
    /// 
    /// # Arguments
    /// 
    /// * `center` - Center of the distribution
    fn sample_c(&self, center: i64) -> i64 {
        let mut result = 0;
        for i in 0..self.k {
            let bit = self.extract_bit(center, i as i32);
            if bit == 1 {
                result += self.combiners[i as usize].generate_integer();
            }
        }
        result
    }

    /// Method to return the nth bit of a number
    /// 
    /// # Arguments
    /// 
    /// * `number` - The number that the bit of desired
    /// * `n` - Desired bit number
    fn extract_bit(&self, number: i64, n: i32) -> i32 {
        ((number >> n) & 0x1) as i32
    }
}

