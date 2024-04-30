/*
 This code provides generation of gaussian distributions of discrete values. Discrete uniform generator
 relies on the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/

/**
 * This is the header file for DiscreteGaussianGenerator class, which contains 3
 * different sampling methods.
 *
 * First sampling method implemented is the rejection sampling defined in
 * section 4.1 of https://eprint.iacr.org/2007/432.pdf. It is usable for
 * arbitrary centers and standard deviations, and does not require any form of
 * precomputation. However, it has high rejection rates and is prone to timing
 * attacks. It is not used anywhere in the library at the moment and is here for
 * historical reasons.
 *
 * Second sampling method implemented is Karney's method defined in Algorithm D
 * from https://arxiv.org/pdf/1303.6257.pdf, which is an improved method based
 * on rejection sampling. It also works for arbitrary centers and standard
 * deviations without any precomputation. Its rejection rate is smaller than in
 * the rejection sampling method but it may still be vulnerable to timing
 * attacks.
 *
 *
 * Final sampling method defined in this class is the Peikert's inversion method
 * discussed in section 4.1 of https://eprint.iacr.org/2010/088.pdf and
 * summarized in section 3.2.2 of
 * https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf. It
 * requires CDF tables of probabilities centered around single center to be
 * kept, which are precalculated in constructor. The method is not prone to
 * timing attacks but it is usable for single center, single deviation only.
 * It should be also noted that the memory requirement grows with the standard
 * deviation, therefore it is advised to use it with smaller deviations.   */

pub const KARNEY_THRESHOLD: f64 = 300.0;

/*
 * @brief The class for Discrete Gaussian Distribution generator.
 */

// TODO: Выделить в отдельный модуль
trait IntegerHelper {
    fn zero() -> Self;
}

// TODO: Выделить в отдельный модуль
trait VectorType {
    type Integer: IntegerHelper;

    fn zero(size: usize) -> Self;
}

use rand::prelude::*;
use std::{f64::consts::E, marker::PhantomData};

pub struct DiscreteGaussianGenerator<VecType: VectorType> {
    m_std: f64,
    m_a: f64,
    m_vals: Vec<f64>,
    peikert: bool,
    _marker: PhantomData<VecType>,
}

impl<VecType: VectorType> DiscreteGaussianGenerator<VecType> {
    /**
     * @brief         Basic constructor for specifying distribution parameter and
     * modulus.
     * @param modulus The modulus to use to generate discrete values.
     * @param std     The standard deviation for this Gaussian Distribution.
     */
    pub fn new(m_std: f64) -> Self {
        // Gyana to add precomputation methods and data members
        // all parameters are set as int because it is assumed that they are used for
        // generating "small" polynomials only
        DiscreteGaussianGenerator {
            m_std,
            m_a: 0.0,
            m_vals: Vec::new(),
            peikert: false,
            _marker: PhantomData,
        }
    }

    /**
     * @brief Check if the gaussian generator has been initialized with a standard deviation
     */
    pub fn is_initialized(&self) -> bool {
        !self.m_vals.is_empty()
    }

    /**
     * @brief Initializes the generator.
     */
    pub fn initialize(&mut self) {
        // Implement initialization logic here
    }

    /**
     * @brief  Returns the standard deviation of the generator.
     * @return The analytically obtained standard deviation of the generator.
     */
    pub fn get_std(&self) -> f64 {
        self.m_std
    }

    /**
     * @brief     Sets the standard deviation of the generator.
     * @param std The analytic standard deviation of the generator.
     */
    pub fn set_std(&mut self, std: f64) {
        self.m_std = std;
    }

    /**
     * @brief      Returns a generated signed integer. Uses Peikert's Inversion
     * Method
     * @return     a value generated with the distribution.
     */
    pub fn generate_int(&self) -> i32 {
        // Implement Peikert's Inversion Method here
        0
    }

    /**
     * @brief      Returns a generated integer vector. Uses Peikert's inversion
     * method.
     * @param size The number of values to return.
     * @return     A pointer to an array of integer values generated with the
     * distribution.
     */
    pub fn generate_int_vector(&self, size: u32) -> Vec<i64> {
        // Implement Peikert's Inversion Method here
        vec![0; size as usize]
    }

    /**
     * @brief  Returns a generated integer. Uses Peikert's inversion method.
     * @return A random value within this Discrete Gaussian Distribution.
     */
    pub fn generate_integer(&self, modulus: VecType::Integer) -> VecType::Integer {
        // Implement Peikert's Inversion Method here
        VecType::Integer::zero()
    }

    /**
     * @brief           Generates a vector of random values within this Discrete
     * Gaussian Distribution. Uses Peikert's inversion method.
     *
     * @param  size     The number of values to return.
     * @param  modulus  modulus of the polynomial ring.
     * @return          The vector of values within this Discrete Gaussian
     * Distribution.
     */
    pub fn generate_vector(&self, size: u32, modulus: VecType::Integer) -> VecType {
        // Implement Peikert's Inversion Method here
        VecType::zero(size as usize)
    }

    /**
     * @brief  Returns a generated integer. Uses rejection method.
     * @param mean center of discrete Gaussian distribution.
     * @param stddev standard deviatin of discrete Gaussian distribution.
     * @param n is ring dimension
     * param modulus modulus
     * @return A random value within this Discrete Gaussian Distribution.
     */
    pub fn generate_integer_rejection(
        &self,
        mean: f64,
        stddev: f64,
        n: usize,
        modulus: VecType::Integer,
    ) -> VecType::Integer {
        // Implement rejection method here
        VecType::Integer::zero()
    }

    /**
     * @brief  Returns a generated integer. Uses rejection method.
     * @param mean center of discrete Gaussian distribution.
     * @param stddev standard deviatin of discrete Gaussian distribution.
     * @param n is ring dimension
     * @return A random value within this Discrete Gaussian Distribution.
     */
    pub fn generate_integer_rejection2(&self, mean: f64, stddev: f64, n: usize) -> i32 {
        // Implement rejection method here
        0
    }

    /**
     * @brief  Returns a generated integer (int32_t). Uses rejection method.
     * @param mean center of discrete Gaussian distribution.
     * @param stddev standard deviation of discrete Gaussian distribution.
     * @return A random value within this Discrete Gaussian Distribution.
     */
    // int32_t GenerateInt32 (double mean, double stddev);
    // will be defined later

    /**
     * @brief Returns a generated integer. Uses Karney's method defined as
     * Algorithm D in https://arxiv.org/pdf/1303.6257.pdf
     * @param mean center of discrete Gaussian distribution.
     * @param stddev standard deviation of discrete Gaussian distribution.
     * @return A random value within this Discrete Gaussian Distribution.
     */
    pub fn generate_integer_karney(mean: f64, stddev: f64) -> i64 {
        // Implement Karney's method here
        0
    }

    pub fn find_in_vector(&self, s: &[f64], search: f64) -> u32 {
        // Implement find_in_vector method here
        0
    }

    pub fn unnormalized_gaussian_pdf(mean: f64, sigma: f64, x: i32) -> f64 {
        E.powf(-((x as f64 - mean).powi(2) / (2.0 * sigma * sigma)))
    }

    pub fn unnormalized_gaussian_pdf_optimized(mean: f64, sigma_factor: f64, x: i32) -> f64 {
        E.powf(sigma_factor * (x as f64 - mean).powi(2))
    }

    /**
     * @brief Subroutine used by Karney's Method to accept an integer with
     * probability exp(-n/2).
     * @param g Mersenne Twister Engine used for deviates
     * @param n Number to test with exp(-n/2) probability
     * @return Accept/Reject result
     */
    pub fn algorithm_p(g: &mut ThreadRng, n: i32) -> bool {
        // Implement AlgorithmP here
        false
    }

    /**
     * @brief Subroutine used by Karney's Method to generate an integer with
     * probability exp(-k/2)(1 - exp(-1/2)).
     * @param g Mersenne Twister Engine used for deviates
     * @return Random number k
     */
    pub fn algorithm_g(g: &mut ThreadRng) -> i32 {
        // Implement AlgorithmG here
        0
    }

    /**
     * @brief Generates a Bernoulli random value H which is true with probability
     * exp(-1/2).
     * @param g Mersenne Twister Engine used for uniform deviates
     * @return Bernoulli random value H
     */
    pub fn algorithm_h(g: &mut ThreadRng) -> bool {
        // Implement AlgorithmH here
        false
    }

    /**
     * @brief Generates a Bernoulli random value H which is true with probability
     * exp(-1/2). Uses double precision.
     * @param g Mersenne Twister Engine used for uniform deviates
     * @return Bernoulli random value H
     */
    pub fn algorithm_h_double(g: &mut ThreadRng) -> bool {
        // Implement AlgorithmHDouble here
        false
    }

    /**
     * @brief Bernoulli trial with probability exp(-x(2k + x)/(2k + 2)).
     * @param g Mersenne Twister Engine used for uniform deviates
     * @param k Deviate k used for calculations
     * @param x Deviate x used for calculations
     * @return Whether the number of runs are even or not
     */
    pub fn algorithm_b(g: &mut ThreadRng, k: i32, x: f64) -> bool {
        // Implement AlgorithmB here
        false
    }

    /**
     * @brief Bernoulli trial with probability exp(-x(2k + x)/(2k + 2)). Uses
     * double precision.
     * @param g Mersenne Twister Engine used for uniform deviates
     * @param k Deviate k used for calculations
     * @param x Deviate x used for calculations
     * @return Whether the number of runs are even or not
     */
    pub fn algorithm_b_double(g: &mut ThreadRng, k: i32, x: f64) -> bool {
        // Implement AlgorithmBDouble here
        false
    }
}
