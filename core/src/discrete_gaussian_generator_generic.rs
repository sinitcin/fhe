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


   discretegaussiangeneratorgeneric.h
   discretegaussiangeneratorgeneric.cpp
*/

use std::{collections::VecDeque, fmt::Debug};

const PRECISION: u32 = 53;
const BERNOULLI_FLIPS: u32 = 23;
const MAX_TREE_DEPTH: i32 = 64;
const MAX_LEVELS: i32 = 4;

// const double DG_ERROR = 8.27181e-25;
// const int32_t N_MAX = 16384;
// const double SIGMA = std::sqrt(std::log(2 * N_MAX / DG_ERROR) / M_PI);
// const int32_t PRECISION = 128;
// const double TAIL_CUT = std::sqrt(log(2)*2*(double)(PRECISION));
// const int32_t DDG_DEPTH = 13;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BaseSamplerType {
    KnuthYao,
    PeikertInversion,
}

/// 🇷🇺 Реализация класса для генерации случайных битов. Создан для централизации пулов случайных битов в сэмплерах
/// 🇬🇧 Class implementation to generate random bit. This is created for centralizing the random bit pools by the samplers.
#[derive(Debug, Clone)]
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

    /// 🇷🇺 Метод генерации случайного бита.
    /// 🇬🇧 Method for generating a random bit. Return A random bit
    pub fn generate(&mut self) -> i16 {
        if self.counter == 0 {
            self.sequence = rand::random::<u32>();
            self.counter = 32;
        }
        let bit = (self.sequence >> (self.counter - 1)) & 0x1;
        self.counter -= 1;
        bit as i16
    }

    pub fn generate_bit(&mut self) -> bool {
        self.generate() != 0
    }
}

/// 🇷🇺 Определение класса для базовых сэмплеров с предварительными вычислениями, которое используется для общего сэмплера UCSD
/// 🇬🇧 Class definition for base samplers with precomputation that is used for UCSD generic sampler
#[derive(Debug, Clone)]
pub struct BaseSamplerObject {
    /// 🇷🇺 Средняя величина используемого распределения
    /// 🇬🇧 Mean of the distribution used
    b_mean: f64,

    /// 🇷🇺 Стандартное отклонение распределения.
    /// 🇬🇧 The standard deviation of the distribution.
    b_std: f64,

    /// 🇷🇺 Генератор, используемый для создания случайных битов путем выборки
    /// 🇬🇧 Generator used for creating random bits through sampling
    bg: BitGenerator, // bg: Box<dyn BitGenerator>,

    /// 🇷🇺 Тип базового сэмплера (Knuth Yao или Peikert's Inversion)
    /// 🇬🇧 Type of the base sampler (Knuth Yao or Peikert's Inversion)
    b_type: BaseSamplerType,

    b_a: f64,
    ddg_tree: Vec<VecDeque<i32>>,

    /// 🇷🇺 Массив, хранящий веса Хэмминга матрицы вероятностей, используемой в выборке Кнута-Яо
    /// 🇬🇧 Array that stores the Hamming Weights of the probability matrix used in Knuth-Yao sampling
    hamming_weights: Vec<u32>,

    /// 🇷🇺 Размер матрицы вероятностей, используемой в Knuth-Yao
    /// 🇬🇧 Size of probability matrix used in Knuth-Yao
    b_matrix_size: i32,

    /// 🇷🇺 Индекс первого бита с ненулевым весом Хэмминга в таблице вероятностей
    /// 🇬🇧 Index of first bit with non zero Hamming weight in the probability table
    first_non_zero: i32,

    end_index: i32,

    fin: i32,
    m_vals: Vec<f64>,
}

/// 🇷🇺 Трейт для базовых сэмплеров
/// 🇬🇧 Trait for base samplers
trait BaseSampler {
    fn generate_integer(&mut self) -> i64;
}

impl BaseSamplerObject {
    /// 🇷🇺 Конструктор
    ///
    /// Параметры:
    /// - mean: Среднее значение распределения
    /// - std: Стандартное отклонение распределения
    /// - generator: Указатель на генератор битов, из которого сэмплер будет брать случайные биты
    /// - bType: Тип базового сэмплера
    ///
    ///
    /// 🇬🇧 Constructor
    ///
    /// Parameters:
    /// - mean: Mean of the distribution
    /// - std: Standard deviation of the distribution
    /// - generator: Pointer to the bit generator that the sampler will use the random bits from
    /// - bType: Type of the base sampler
    fn new(mean: f64, std: f64, generator: BitGenerator, b_type: BaseSamplerType) -> Self {
        let acc: f64 = 1e-17;
        let fin = (std * (-2.0 * acc.ln()).sqrt()).ceil() as i32;
        let b_mean = if mean >= 0.0 {
            mean.floor()
        } else {
            mean.ceil()
        };

        let mean = mean - b_mean;
        let mut sampler = BaseSamplerObject {
            b_mean,
            b_std: std,
            bg: generator,
            b_type,
            fin,
            m_vals: Vec::new(),
            b_a: 0.0,
            b_matrix_size: 0,
            hamming_weights: vec![0; 64],
            ddg_tree: Vec::new(),
            first_non_zero: -1,
            end_index: 0,
        };

        if b_type == BaseSamplerType::PeikertInversion {
            sampler.initialize(mean);
        } else {
            sampler.generate_prob_matrix(std, mean);
        }

        sampler
    }

    /// 🇷🇺 Метод генерации случайного бита с помощью генератора битов в пределах возврата случайного бита
    /// 🇬🇧 Method for generating a random bit from the bit generator within return a random bit
    fn random_bit(&mut self) -> bool {
        self.bg.generate() != 0
    }

    /// 🇷🇺 Подпроцедура, вызываемая инверсионной выборкой Пейкерта
    ///
    /// Параметры:
    /// - S: Вектор, содержащий значения CDF
    /// - search: Значение искомой вероятности
    ///
    /// Возвращает: Индекс, который является наименьшим большим значением, чем искомое
    ///
    /// 🇬🇧 Sub-procedure called by Peikert's inversion sampling
    ///
    /// Parameters:
    /// - S: Vector containing the CDF values
    /// - search: Searched probability value
    ///
    /// Returns: Index that is the smallest bigger value than search

    // fn find_in_vector(&self, s: &[f64], search: f64) -> i32 {
    //     match s.binary_search_by(|x| x.partial_cmp(&search).unwrap()) {
    //         Ok(index) => index as i32,
    //         Err(_) => panic!("DGG Inversion Sampling. FindInVector value not found: {}", search),
    //     }
    // }
    fn find_in_vector(&self, s: &[f64], search: f64) -> usize {
        s.iter().position(|&x| x >= search).unwrap_or(s.len())
    }

    /// 🇷🇺 Генерирует дерево DDG, используемое для выборки в Knuth-Yao
    ///
    /// Параметры:
    /// - probMatrix: Матрица вероятностей, используемая для заполнения DDG-дерева
    ///
    /// 🇬🇧 Generates DDG tree used through the sampling in Knuth-Yao
    ///
    /// Parameters:
    /// - probMatrix: The probability matrix used for filling the DDG tree
    fn generate_ddg_tree(&mut self, prob_matrix: &[u64]) {
        self.first_non_zero = -1;
        for i in 0..64 {
            if self.hamming_weights[i] != 0 {
                self.first_non_zero = i as i32;
                break;
            }
        }

        self.end_index = self.first_non_zero;
        let mut inode_count = 1;
        for _ in 0..self.first_non_zero {
            inode_count *= 2;
        }

        // let mut end = false;
        let mut max_node_count = inode_count;
        for i in self.first_non_zero..MAX_TREE_DEPTH {
            inode_count *= 2;
            self.end_index += 1;
            if inode_count as u32 >= max_node_count {
                max_node_count = inode_count;
            }
            inode_count -= self.hamming_weights[i as usize];
            if inode_count <= 0 {
                // end = true;
                if inode_count < 0 {
                    self.end_index -= 1;
                }
                break;
            }
        }

        let size = max_node_count as usize;
        self.ddg_tree =
            vec![VecDeque::with_capacity((self.end_index - self.first_non_zero) as usize); size];

        for i in 0..size {
            self.ddg_tree[i] =
                VecDeque::from(vec![-2; (self.end_index - self.first_non_zero) as usize]);
        }

        inode_count = 1;
        for _ in 0..self.first_non_zero {
            inode_count *= 2;
        }

        for i in self.first_non_zero..self.end_index {
            inode_count *= 2;
            inode_count -= self.hamming_weights[i as usize];
            for j in 0..(inode_count as u32) {
                self.ddg_tree[j as usize][(i - self.first_non_zero) as usize] = -1;
            }
            let mut enode_count = 0;
            for j in 0..prob_matrix.len() {
                if (prob_matrix[j] >> (63 - i as u32)) & 1 != 0 {
                    self.ddg_tree[inode_count as usize + enode_count]
                        [(i - self.first_non_zero) as usize] = j as i32;
                    enode_count += 1;
                }
            }
        }
    }

    /// 🇷🇺 Инициализирует генератор, используемый для метода инверсии Пейкерта.
    ///
    /// Параметры:
    /// - mean: Среднее значение распределения, которое будет использовать сэмплер.
    ///
    /// 🇬🇧 Initializes the generator used for Peikert's Inversion method.
    ///
    /// Parameters:
    /// - mean: Mean of the distribution that the sampler will be using

    fn initialize(&mut self, mean: f64) {
        self.m_vals.clear();
        let variance = self.b_std.powi(2);
        let mut cusum = 0.0;

        for x in (-self.fin)..=self.fin {
            cusum += (-(x as f64 - mean).powi(2) / (variance * 2.0)).exp();
        }

        self.b_a = 1.0 / cusum;

        for i in (-self.fin)..=self.fin {
            let temp = self.b_a * (-(f64::from(i as f64 - mean).powi(2) / (2.0 * variance))).exp();
            self.m_vals.push(temp);
        }

        for i in 1..self.m_vals.len() {
            self.m_vals[i] += self.m_vals[i - 1];
        }
    }

    /// 🇷🇺 Генерирует матрицу вероятностей заданного распределения, которая используется в методе Кнута-Яо
    ///
    /// Параметры:
    /// - stddev: стандартное отклонение дискретного гауссовского распределения
    /// - mean: Центр распределения
    /// - tableCount: Количество таблиц вероятностей, которые будут сгенерированы
    ///
    /// 🇬🇧 Generates the probability matrix of given distribution, which is used in Knuth-Yao method
    ///
    /// Parameters:
    /// - stddev: Standard deviation of Discrete Gaussian Distribution
    /// - mean: Center of the distribution
    /// - tableCount: Number of probability tables to be generated
    fn generate_prob_matrix(&mut self, stddev: f64, mean: f64) {
        let mut prob_matrix = vec![0u64; 2 * self.fin as usize + 1];
        let mut probs = vec![0.0; 2 * self.fin as usize + 1];
        let mut s = 0.0;
        self.b_std = stddev;
        let mut error = 1.0;

        for i in -self.fin..=self.fin {
            let prob = (-((i as f64 - mean).powi(2) / (2.0 * stddev.powi(2)))).exp();
            s += prob;
            probs[(i + self.fin) as usize] = prob;
        }

        match prob_matrix.last_mut() {
            Some(last) => *last = (error * 2.0f64.powi(64)) as u64,
            None => panic!("Error in generate_prob_matrix"),
        }

        for i in 0..prob_matrix.len() {
            error -= probs[i] / s;
            prob_matrix[i] = (probs[i] / s * 2.0f64.powi(64)) as u64;
            for j in 0..64 {
                self.hamming_weights[j] += ((prob_matrix[i] >> (63 - j)) & 1) as u32;
            }
        }

        self.generate_ddg_tree(&prob_matrix);
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует наивный метод Кнута-Яо.
    /// Алгоритм Кнута-Яо использует дискретное вероятностное распределение, представленное в виде DDG дерева,
    /// для генерации случайных чисел. Этот алгоритм эффективен для создания переменных со сложными распределениями
    /// благодаря своей способности точно моделировать дискретные вероятности через бинарное дерево представление.
    /// Важным аспектом является его итеративный подход к определению значения переменной, последовательно рассматривая
    /// каждый бит случайного двоичного числа до достижения условия успеха или ошибки.
    /// Возвращает случайную величину в рамках дискретного гауссова распределения
    /// 🇬🇧 Returns a generated integer. Uses Naive Knuth-Yao method. Return a random value within the Discrete Gaussian Distribution

    fn generate_integer_knuth_yao(&mut self) -> i64 {
        // Начальное недопустимое значение для результата
        let mut ans = -1;
        // Флаг, указывающий на удачную генерацию числа
        let mut hit = false;

        while !hit {
            // Индекс узла в DDG дереве
            let mut node_index = 0;
            // Флаг для обнаружения ошибок в процессе генерации
            let mut error = false;

            // Цикл по глубине дерева
            for i in 0..MAX_TREE_DEPTH {
                // Генерация случайного бита
                let bit = self.bg.generate_bit();
                // Построение индекса пути по дереву: если bit == true, выбирается правый потомок, иначе - левый
                node_index *= 2;
                if bit {
                    node_index += 1;
                }

                // Проверка, что текущая глубина достигла значимой части дерева
                if self.first_non_zero <= i as i32 {
                    if i as i32 <= self.end_index {
                        // Извлечение значения из DDG дерева. Это значение соответствует сумме вероятностей
                        // для данного бинарного пути и глубины
                        ans = self.ddg_tree[node_index as usize]
                            [(i - self.first_non_zero as i32) as usize];
                    }
                    if ans >= 0 {
                        // Если ans не является специальным значением границы (self.b_matrix_size - 1),
                        // то найдено допустимое случайное значение
                        if ans != (self.b_matrix_size - 1) as i32 {
                            // Успех, нужное значение сгенерировано
                            hit = true;
                        } else {
                            // Специальное значение указывает на необходимость перегенерации
                            error = true;
                        }
                    } else {
                        // Особый случай ошибки
                        if ans == -2 {
                            error = true;
                        }
                    }
                }
                if hit || error {
                    // Прерывание цикла при достижении успеха или ошибки
                    break;
                }
            }
        }

        // Корректировка сгенерированного значения с учетом параметров распределения
        (ans - self.fin + self.b_mean as i32) as i64
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует метод инверсии Пейкерта.
    /// 🇬🇧 Returns a generated integer. Uses Peikert's inversion method.
    fn generate_integer_peikert(&self) -> i64 {
        use rand::prelude::*;

        let mut rng = thread_rng();
        let seed: f64 = rng.gen();
        let val = self.find_in_vector(&self.m_vals, seed);
        (val as i32 - self.fin + self.b_mean as i32) as i64
    }
}

impl BaseSampler for BaseSamplerObject {
    /// 🇷🇺 Метод генерации целого числа из базового сэмпла случайного целого числа из распределения
    /// 🇬🇧 Method for generating integer from the base sampler a random integer from the distribution
    fn generate_integer(&mut self) -> i64 {
        match self.b_type {
            BaseSamplerType::KnuthYao => self.generate_integer_knuth_yao(),
            BaseSamplerType::PeikertInversion => self.generate_integer_peikert(),
        }
    }
}

/// 🇷🇺 Класс для объединения образцов из двух базовых пробоотборников, который используется для общего отбора образцов UCSD
/// 🇬🇧 Class for combining samples from two base samplers, which is used for UCSD generic sampling
#[derive(Clone)]
struct SamplerCombiner {
    sampler1: Box<dyn BaseSampler>,
    sampler2: Box<dyn BaseSampler>,
    x1: i64,
    x2: i64,
}

impl SamplerCombiner {
    /// 🇷🇺 Конструктор
    ///
    /// # Аргументы
    ///
    /// * `s1` - Указатель на первый объединяемый семплер
    /// * `s2` - Указатель на второй объединяемый семплер
    /// * `z1` - Коэффициент для первого сэмплера
    /// * `z2` - Коэффициент для второго пробоотборника
    ///
    /// 🇬🇧 Constructor
    ///
    /// # Arguments
    ///
    /// * `s1` - Pointer to the first sampler to be combined
    /// * `s2` - Pointer to the second sampler to be combined
    /// * `z1` - Coefficient for the first sampler
    /// * `z2` - Coefficient for the second sampler
    fn new<T>(s1: Box<T>, s2: Box<T>, z1: i64, z2: i64) -> Self
    where
        T: BaseSampler + Clone + 'static,
    {
        SamplerCombiner {
            sampler1: s1,
            sampler2: s2,
            x1: z1,
            x2: z2,
        }
    }
}

impl BaseSampler for SamplerCombiner {
    /// 🇷🇺 Возвращает комбинированное значение для двух сэмплеров с заданными коэффициентами
    /// 🇬🇧 Return the combined value for two samplers with given coefficients
    fn generate_integer(&mut self) -> i64 {
        self.x1 * self.sampler1.generate_integer() + self.x2 * self.sampler2.generate_integer()
    }
}

/// 🇷🇺 Структура для генератора дискретного гауссовского распределения Generic.
/// 🇬🇧 The struct for Generic Discrete Gaussian Distribution generator.
struct DiscreteGaussianGeneratorGeneric {
    base_samplers: Vec<Box<dyn BaseSampler>>,
    wide_sampler: Box<dyn BaseSampler>,
    combiners: Vec<Box<dyn BaseSampler>>,
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
    /// 🇷🇺 Основной конструктор, который выполняет предварительные вычисления.
    ///
    /// # Аргументы
    ///
    /// * `samplers` - Массив, содержащий базовые выборки
    /// * `std` - Стандартное отклонение базовых выборок
    /// * `b` - Лог числа центров, используемых для вычисления базовых выборок (напомним, что базовые выборки центрированы от 0 до (2^b-1)/2^b)
    /// * `N` - Параметр сглаживания
    ///
    /// 🇬🇧 Basic constructor which does the precomputations.
    ///
    /// # Arguments
    ///
    /// * `samplers` - Array containing the base samplers
    /// * `std` - Standard deviation of the base samplers
    /// * `b` - Log of number of centers that are used for calculating base samplers (Recall that base samplers are centered from 0 to (2^b-1)/2^b)
    /// * `N` - Smoothing parameter
    // fn new(samplers: Vec<Box<dyn BaseSampler>>, std: f64, b: i32, N: f64) -> Self {
    //     // Precomputation logic here
    //     DiscreteGaussianGeneratorGeneric {
    //         base_samplers: samplers,
    //         wide_sampler: BaseSampler::new(),
    //         combiners: vec![],
    //         wide_variance: 0.0,
    //         sampler_variance: 0.0,
    //         x: 0.0,
    //         c: 0.0,
    //         ci: 0.0,
    //         k: 0,
    //         log_base: b,
    //         mask: 0,
    //     }
    // }

    fn new(samplers: Vec<Box<dyn BaseSampler>>, std: f64, b: i32, n: f64) -> Self {
        let mut wide_sampler: Box<dyn BaseSampler> = Box::new(samplers[0].clone());
        let mut wide_variance = std.powi(2);
        let mut combiners: Vec<Box<dyn BaseSampler>> = Vec::new();

        for _ in 1..MAX_LEVELS {
            let x1 = (wide_variance / (2.0 * n.powi(2))).sqrt().floor() as u32;
            let x2 = std::cmp::max(x1 - 1, 1);
            wide_sampler = Box::new(SamplerCombiner::new(
                wide_sampler.clone(),
                wide_sampler,
                x1 as i64,
                x2 as i64,
            ));
            combiners.push(wide_sampler.clone());
            wide_variance = (x1.pow(2) + x2.pow(2)) as f64 * wide_variance;
        }

        let k = ((PRECISION - BERNOULLI_FLIPS) as f64 / b as f64).ceil() as u32;
        let mask = (1u64 << b) - 1;

        let mut sampler_variance = 1.0;
        let t = 1 / (1u64 << (2 * b));
        let mut s = 1.0;
        for _ in 1..k {
            s *= t as f64;
            sampler_variance += s;
        }
        sampler_variance *= std.powi(2);

        Self {
            base_samplers: samplers,
            log_base: b,
            wide_sampler,
            wide_variance,
            combiners,
            k: k as i32,
            mask,
            sampler_variance,
            x: 0.0,
            c: 0.0,
            ci: 0.0,
        }
    }

    /// 🇷🇺 Возвращает сгенерированное целое число. Использует общий алгоритм из статьи UCSD, основанный на Sample Z
    ///
    /// # Аргументы
    ///
    /// * `mean` - Среднее значение распределения
    /// * `variance` - Вариация желаемого распределения
    ///
    /// 🇬🇧 Returns a generated integer. Uses generic algorithm in UCSD paper, based on Sample Z
    ///
    /// # Arguments
    ///
    /// * `mean` - Mean of the distribution
    /// * `variance` - Variance of the desired distribution
    // fn generate_integer(&self, mean: f64, variance: f64) -> i64 {
    //     let mut rng = rand::thread_rng();
    //     let gaussian = Gaussian::new(mean, variance.sqrt());
    //     let sample = gaussian.sample(&mut rng);
    //     sample.round() as i64
    // }
    fn generate_integer(&mut self, center: f64, std: f64) -> i64 {
        let variance = std.powi(2);
        let x = self.wide_sampler.generate_integer();

        let c =
            center + (x as f64) * ((variance - self.sampler_variance) / self.wide_variance).sqrt();

        let ci = c.floor() as i64;
        let c = c - ci as f64;

        ci + self.flip_and_round(c)
    }

    /// 🇷🇺 Возвращает сгенерированное целое число с использованием базового сэмплера.
    /// 🇬🇧 Returns a generated integer using the base sampler.
    // fn generate_integer(&self) -> i64 {
    //     self.base_samplers[0].generate_integer()
    // }

    /// 🇷🇺 Подпрограмма, используемая в Выборке C
    ///
    /// # Аргументы
    ///
    /// * `center` - Центр распределения
    ///
    /// 🇬🇧 Subroutine used by Sample C
    ///
    /// # Arguments
    ///
    /// * `center` - Center of the distribution
    ///
    fn flip_and_round(&mut self, center: f64) -> i64 {
        let c = (center as u64 * (1u64 << PRECISION)) as i64;
        let base_c = c >> BERNOULLI_FLIPS;
        let mut random_bit;

        for i in (0..BERNOULLI_FLIPS).rev() {
            random_bit = self.base_samplers[0].generate_integer();
            if random_bit > self.extract_bit(c, i as i32).into() {
                return self.sample_c(base_c);
            }
            if random_bit < self.extract_bit(c, i as i32).into() {
                return self.sample_c(base_c + 1);
            }
        }
        self.sample_c(base_c + 1)
    }

    // fn flip_and_round(&self, center: f64) -> i64 {
    //     let mut rng = rand::thread_rng();
    //     let gaussian = Gaussian::new(center, self.wide_variance.sqrt());
    //     let sample = gaussian.sample(&mut rng);
    //     sample.round() as i64
    // }

    /// 🇷🇺 Выборка C, определенная в статье
    ///
    /// # Аргументы
    ///
    /// * `center` - Центр распределения
    ///
    /// 🇬🇧 Sample C defined in the paper
    ///
    /// # Arguments
    ///
    /// * `center` - Center of the distribution
    fn sample_c(&mut self, center: i64) -> i64 {
        let mut c = center;
        let mut sample;
        for _ in 0..self.k {
            sample = self.base_samplers[(self.mask & c as u64) as usize].generate_integer();
            if (self.mask & c as u64) > 0 && c < 0 {
                sample -= 1;
            }
            c /= 2i64.pow(self.log_base as u32);
            c += sample;
        }
        c
    }

    // fn sample_c(&self, center: i64) -> i64 {
    //     let mut result = 0;
    //     for i in 0..self.k {
    //         let bit = self.extract_bit(center, i as i32);
    //         if bit == 1 {
    //             result += self.combiners[i as usize].generate_integer();
    //         }
    //     }
    //     result
    // }

    /// 🇷🇺 Метод, возвращающий n-й бит числа
    ///
    /// # Аргументы
    ///
    /// * `number` - Число, в котором находится искомый бит.
    /// * `n` - Номер желаемого бита
    ///
    /// 🇬🇧 Method to return the nth bit of a number
    ///
    /// # Arguments
    ///
    /// * `number` - The number that the bit of desired
    /// * `n` - Desired bit number
    fn extract_bit(&self, number: i64, n: i32) -> i32 {
        ((number >> n) & 0x1) as i32
    }
}
