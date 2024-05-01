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
    🇷🇺 Этот код обеспечивает генерацию гауссовых распределений дискретных значений. Дискретный равномерный генератор
    опирается на генератор для 32-битных беззнаковых целых чисел, определенный в <random

    Это заголовочный файл для класса DiscreteGaussianGenerator, который содержит 3 различных метода выборки.

    Первый реализованный метод выборки - выборка с отклонением, определенная в разделе 4.1
    в https://eprint.iacr.org/2007/432.pdf. Он подходит для произвольных центров и стандартных отклонений,
    и не требует никаких предварительных вычислений. Однако он имеет высокий процент отбраковки и подвержен временным атакам.
    В настоящее время он не используется нигде в библиотеке и приведен здесь по историческим соображениям.

    Второй реализованный метод выборки - это метод Карни, определенный в алгоритме D из https://arxiv.org/pdf/1303.6257.pdf,
    который является улучшенным методом, основанным на на выборке с отклонением. Он также работает для произвольных
    центров и стандартных отклонений без каких-либо предварительных вычислений. Коэффициент отклонения меньше,
    чем в метод выборки с отклонением, но он все еще может быть уязвим для атак по времени атаки.

    Последним методом выборки, определенным в этом классе, является метод инверсии Пейкерта рассмотренный в разделе 4.1
    https://eprint.iacr.org/2010/088.pdf и обобщенный в разделе 3.2.2 на
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf. Он требует хранения таблиц CDF вероятностей,
    которые сосредоточенных вокруг одного центра которые предварительно вычисляются в конструкторе. Метод не подвержен
    временным атакам, но применим только для одного центра и одного отклонения.

    Следует также отметить, что потребность в памяти растет с увеличением стандартного отклонением, поэтому рекомендуется
    использовать его при меньших отклонениях.

    🇬🇧 This code provides generation of gaussian distributions of discrete values. Discrete uniform generator
    relies on generator for 32-bit unsigned integers defined in <random>

    This is the header file for DiscreteGaussianGenerator class, which contains 3
    different sampling methods.

    First sampling method implemented is the rejection sampling defined in
    section 4.1 of https://eprint.iacr.org/2007/432.pdf. It is usable for
    arbitrary centers and standard deviations, and does not require any form of
    precomputation. However, it has high rejection rates and is prone to timing
    attacks. It is not used anywhere in the library at the moment and is here for
    historical reasons.

    Second sampling method implemented is Karney's method defined in Algorithm D
    from https://arxiv.org/pdf/1303.6257.pdf, which is an improved method based
    on rejection sampling. It also works for arbitrary centers and standard
    deviations without any precomputation. Its rejection rate is smaller than in
    the rejection sampling method but it may still be vulnerable to timing
    attacks.

    Final sampling method defined in this class is the Peikert's inversion method
    discussed in section 4.1 of https://eprint.iacr.org/2010/088.pdf and
    summarized in section 3.2.2 of
    https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf. It
    requires CDF tables of probabilities centered around single center to be
    kept, which are precalculated in constructor. The method is not prone to
    timing attacks but it is usable for single center, single deviation only.
    It should be also noted that the memory requirement grows with the standard
    deviation, therefore it is advised to use it with smaller deviations.
*/

pub const KARNEY_THRESHOLD: f64 = 300.0;

/*
 * @brief The class for Discrete Gaussian Distribution generator.
 */

// TODO: Выделить в отдельный модуль
pub trait IntegerHelper {
    fn zero() -> Self;
}

// TODO: Выделить в отдельный модуль
pub trait VectorType {
    type Integer: IntegerHelper;

    fn zero(size: usize) -> Self;
}

use rand::prelude::*;
use std::{f64::consts::E, marker::PhantomData};

pub struct DiscreteGaussianGenerator<VecType: VectorType> {
    m_std: f64,
    _m_a: f64,
    m_vals: Vec<f64>,
    _peikert: bool,
    _marker: PhantomData<VecType>,
}

impl<VecType: VectorType> DiscreteGaussianGenerator<VecType> {
    /// 🇷🇺 Базовый конструктор для задания параметра распределения и модуля.
    ///
    /// Параметры:
    /// modulus - Модуль, используемый для генерации дискретных значений.
    /// std     - Стандартное отклонение для этого распределения.
    ///
    /// 🇬🇧 Basic constructor for specifying distribution parameter and modulus.
    ///
    /// Parameters:
    /// modulus - The modulus to use to generate discrete values.
    /// std     - The standard deviation for this Gaussian Distribution.
    pub fn new(m_std: f64) -> Self {
        // Gyana to add precomputation methods and data members
        // all parameters are set as int because it is assumed that they are used for
        // generating "small" polynomials only
        DiscreteGaussianGenerator {
            m_std,
            _m_a: 0.0,
            m_vals: Vec::new(),
            _peikert: false,
            _marker: PhantomData,
        }
    }

    /// 🇷🇺 Проверка, инициализирован ли генератор гаусса со стандартным отклонением
    /// 🇬🇧 Check if the gaussian generator has been initialized with a standard deviation
    pub fn is_initialized(&self) -> bool {
        !self.m_vals.is_empty()
    }

    /// 🇷🇺 Инициализирует генератор.
    /// 🇬🇧 Initializes the generator.
    pub fn initialize(&mut self) {
        // Implement initialization logic here
    }

    /// 🇷🇺 Возвращает стандартное отклонение генератора.
    ///
    /// Результат: Аналитически полученное стандартное отклонение генератора.
    /// 🇬🇧 Returns the standard deviation of the generator.
    ///
    /// Result: The analytically obtained standard deviation of the generator.
    pub fn get_std(&self) -> f64 {
        self.m_std
    }

    /// 🇷🇺 Устанавливает стандартное отклонение генератора.
    ///
    /// Параметры:
    /// std - аналитическое стандартное отклонение генератора.
    ///
    /// 🇬🇧 Sets the standard deviation of the generator.
    ///
    /// Parameters:
    /// std The analytic standard deviation of the generator.
    pub fn set_std(&mut self, std: f64) {
        self.m_std = std;
    }

    /// 🇷🇺 Возвращает сгенерированное знаковое целое число. Использует метод инверсии Пейкерта
    /// Возвращает значение, полученное с помощью распределения.
    ///
    /// 🇬🇧 Returns a generated signed integer. Uses Peikert's Inversion Method
    /// Return a value generated with the distribution.
    pub fn generate_int(&self) -> i32 {
        // Implement Peikert's Inversion Method here
        0
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод инверсии Пейкерта.
    ///
    /// Результат: Вектор целочисленных значений, сгенерированных с помощью распределения.
    ///
    /// 🇬🇧 Returns a generated integer. Uses Peikert's inversion method.
    ///
    /// Result: A Vector of integer values generated with the distribution.
    pub fn generate_int_vector(&self, size: u32) -> Vec<i64> {
        // Implement Peikert's Inversion Method here
        vec![0; size as usize]
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод инверсии Пейкерта.
    ///
    /// Результат: Случайная величина в рамках данного дискретного гауссова распределения.
    ///
    /// 🇬🇧 Returns a generated integer. Uses Peikert's inversion method.
    ///
    /// Result: A random value within this Discrete Gaussian Distribution.
    pub fn generate_integer(&self, _modulus: VecType::Integer) -> VecType::Integer {
        // Implement Peikert's Inversion Method here
        VecType::Integer::zero()
    }

    /// 🇷🇺 Генерирует вектор случайных значений в рамках данного дискретного гауссова распределения.
    /// Использует метод инверсии Пейкерта.
    ///
    /// Параметры:
    /// size - Количество возвращаемых значений.
    /// modulus - модуль кольца полиномов.
    ///
    /// Результат: Вектор случайных значений в рамках данного дискретного гауссова распределения.
    ///
    /// 🇬🇧 Generates a vector of random values within this Discrete Gaussian Distribution. Uses Peikert's inversion method.
    ///
    /// Parameters:
    /// size - The number of values to return.
    /// modulus - modulus of the polynomial ring.
    ///
    /// Result: The vector of values within this Discrete Gaussian Distribution.
    pub fn generate_vector(&self, size: u32, _modulus: VecType::Integer) -> VecType {
        // Implement Peikert's Inversion Method here
        VecType::zero(size as usize)
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод [выборки с отклонением](https://en.wikipedia.org/wiki/Rejection_sampling)
    ///
    /// Параметры:
    /// mean - центр дискретного гауссовского распределения.
    /// std_dev - стандартное отклонение дискретного гауссовского распределения.
    /// n - размер кольца
    ///
    /// Результат: случайное целое число, принадлежащее этому дискретному гауссовскому распределению.
    ///
    /// 🇬🇧 Returns a generated integer. Uses rejection method.
    ///
    /// Parameters:
    /// mean - center of discrete Gaussian distribution.
    /// std_dev - standard deviation of discrete Gaussian distribution.
    /// n - ring dimension
    ///
    /// Result: a random value within this Discrete Gaussian Distribution.
    pub fn generate_integer_rejection(
        &self,
        _mean: f64,
        _std_dev: f64,
        _n: usize,
        _modulus: VecType::Integer,
    ) -> VecType::Integer {
        // Implement rejection method here
        VecType::Integer::zero()
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод [выборки с отклонением](https://en.wikipedia.org/wiki/Rejection_sampling)
    ///
    /// Параметры:
    /// mean - центр дискретного гауссовского распределения.
    /// std_dev - стандартное отклонение дискретного гауссовского распределения.
    /// n - размер кольца
    ///
    /// Результат: случайное целое число, принадлежащее этому дискретному гауссовскому распределению.
    ///
    /// 🇬🇧 Returns a generated integer. Uses rejection method.
    ///
    /// Parameters:
    /// mean - center of discrete Gaussian distribution.
    /// std_dev - standard deviation of discrete Gaussian distribution.
    /// n - ring dimension
    ///
    /// Result: a random value within this Discrete Gaussian Distribution.
    pub fn generate_integer_rejection2(&self, _mean: f64, _std_dev: f64, _n: usize) -> i32 {
        // Implement rejection method here
        0
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод Карни,
    /// определенный как алгоритм D в https://arxiv.org/pdf/1303.6257.pdf.
    /// Параметры:
    /// mean - центр дискретного гауссовского распределения.
    /// std_dev - стандартное отклонение дискретного гауссовского распределения.
    ///
    /// Возвращает: случайное целое число, принадлежащее этому дискретному гауссовскому распределению.
    ///
    /// 🇬🇧 Returns a generated integer. Uses Karney's method defined as Algorithm D in https://arxiv.org/pdf/1303.6257.pdf
    /// Parameters:
    /// mean - center of discrete Gaussian distribution.
    /// std_dev - standard deviation of discrete Gaussian distribution.
    ///
    /// Returns: A random value within this Discrete Gaussian Distribution.
    pub fn generate_integer_karney(_mean: f64, _std_dev: f64) -> i64 {
        // Implement Karney's method here
        0
    }

    pub fn find_in_vector(&self, _s: &[f64], _search: f64) -> u32 {
        // Implement find_in_vector method here
        0
    }

    /// 🇷🇺 Вычисляет функцию плотности вероятности (PDF - probability density function) ненормированного гауссова распределения.
    /// Функция принимает три параметра: среднее, сигма и x.
    /// Для вычисления PDF используется формула exp(-(x - среднее)^2 / (2 * сигма^2)).
    /// Результат возвращается в виде двойного значения.
    ///
    /// 🇬🇧 Calculates the probability density function (PDF) of an unnormalized Gaussian distribution.
    /// It takes three parameters: mean, sigma, and x.
    /// The function uses the formula exp(-(x - mean)^2 / (2 * sigma^2)) to calculate the PDF.
    /// The result is returned as a double value.
    pub fn unnormalized_gaussian_pdf(mean: f64, sigma: f64, x: i32) -> f64 {
        E.powf(-((x as f64 - mean).powi(2) / (2.0 * sigma * sigma)))
    }

    /// 🇷🇺 Вычисляет функцию плотности вероятности (PDF - probability density function) ненормированного гауссова распределения.
    /// Тоже, что и предыдущая функция, но с использованием оптимизации.
    ///
    /// 🇬🇧 Calculates the probability density function (PDF) of an unnormalized Gaussian distribution.
    /// The same as the previous function, but with optimization.
    pub fn unnormalized_gaussian_pdf_optimized(mean: f64, sigma_factor: f64, x: i32) -> f64 {
        E.powf(sigma_factor * (x as f64 - mean).powi(2))
    }

    /// 🇷🇺 Метод Карни использует подпрограмму для генерации целого числа с вероятностью exp(-n/2).
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для отклонений
    /// n Число для проверки с вероятностью exp(-n/2)
    ///
    /// Результат: Принято или нет
    ///
    /// 🇬🇧 Subroutine used by Karney's Method to generate an integer with probability exp(-n/2).
    ///
    /// Parameters:
    ///
    /// g Mersenne Twister Engine used for deviates
    /// n Number to test with exp(-n/2) probability
    ///
    /// Result: Accept/Reject result
    pub fn algorithm_p(_g: &mut ThreadRng, _n: i32) -> bool {
        // Implement AlgorithmP here
        false
    }

    /// 🇷🇺 Метод Карни использует подпрограмму для генерации целого числа с вероятностью exp(-k/2)(1 - exp(-1/2)).
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для отклонений
    ///
    /// Результат: Случайная число k
    ///
    /// 🇬🇧 Subroutine used by Karney's Method to generate an integer with probability exp(-k/2)(1 - exp(-1/2)).
    ///
    /// Parameters:
    ///
    /// g Mersenne Twister Engine used for deviates
    ///
    /// Result: Random number k
    pub fn algorithm_g(_g: &mut ThreadRng) -> i32 {
        // Implement AlgorithmG here
        0
    }

    /// 🇷🇺 Генерирует случайную величину Бернулли H, которая истинна с вероятностью exp(-1/2).
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для равномерных отклонений
    ///
    /// Результат: Случайная величина Бернулли H
    ///
    /// 🇬🇧 Generates a Bernoulli random value H which is true with probability exp(-1/2).
    ///
    /// Parameters:
    ///
    /// g Mersenne Twister Engine used for uniform deviates
    ///
    /// Result: Bernoulli random value H
    pub fn algorithm_h(_g: &mut ThreadRng) -> bool {
        // Implement AlgorithmH here
        false
    }

    /// 🇷🇺 Генерирует случайную величину Бернулли H, которая истинна с вероятностью exp(-1/2). Используется двойная точность.
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для равномерных отклонений
    ///
    /// Результат: Случайная величина Бернулли H
    ///
    /// 🇬🇧 Generates a Bernoulli random value H which is true with probability exp(-1/2). Uses double precision.
    ///
    /// Parameters: g Mersenne Twister Engine used for uniform deviates
    ///
    /// Result: Bernoulli random value H
    pub fn algorithm_h_double(_g: &mut ThreadRng) -> bool {
        // Implement AlgorithmHDouble here
        false
    }

    /// 🇷🇺 Метод Бернулли с вероятностью exp(-x(2k + x)/(2k + 2)). Используется двойная точность.
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для равномерных отклонений
    /// Отклонение k, используемое для расчетов
    /// Отклонение x, используемое для расчетов
    ///
    /// Результат: Четное или нечетное количество прогонов
    ///
    /// 🇬🇧 Bernoulli trial with probability exp(-x(2k + x)/(2k + 2)). Uses double precision.
    ///
    /// Parameters:
    /// g Mersenne Twister Engine used for uniform deviates
    /// k Deviate k used for calculations
    /// x Deviate x used for calculations
    ///
    /// Return Whether the number of runs are even or not
    pub fn algorithm_b(_g: &mut ThreadRng, _k: i32, _x: f64) -> bool {
        // Implement AlgorithmB here
        false
    }

    /// 🇷🇺 Метод Бернулли с вероятностью exp(-x(2k + x)/(2k + 2)). Используется двойная точность.
    ///
    /// Параметры:
    /// g (Вихрь Мерсенна)[https://ru.wikipedia.org/wiki/Вихрь_Мерсенна] используется для равномерных отклонений
    /// Отклонение k, используемое для расчетов
    /// Отклонение x, используемое для расчетов
    ///
    /// Результат: Четное или нечетное количество прогонов
    ///
    /// 🇬🇧 Bernoulli trial with probability exp(-x(2k + x)/(2k + 2)). Uses double precision.
    ///
    /// Parameters:
    /// g Mersenne Twister Engine used for uniform deviates
    /// k Deviate k used for calculations
    /// x Deviate x used for calculations
    ///
    /// Return Whether the number of runs are even or not
    pub fn algorithm_b_double(_g: &mut ThreadRng, _k: i32, _x: f64) -> bool {
        // Implement AlgorithmBDouble here
        false
    }
}
