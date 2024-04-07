/*
 This code provides generation of gaussian distributions of discrete values. Discrete uniform generator
 relies on the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/

/*This is the header file for the Generic Sampler used for various Discrete
 * Gaussian Sampling applications. This class implements the generic sampler by
 * UCSD discussed in the https://eprint.iacr.org/2017/259.pdf and it is heavily
 * based on Michael Walter's original code. Along the sides of the
 * implementation there are also two different "base samplers", which are used
 * for the generic sampler or can be used on their own depending on the
 * requirements of needed application.
 *
 * The first base sampler uses Peikert's inversion method, discussed in
 * section 4.1 of https://eprint.iacr.org/2010/088.pdf and summarized in
 * section 3.2.2 of
 * https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf.
 * Peikert's method requires precomputation of CDF tables around a specific
 * center and the table must be kept during the sampling process. Hence,
 * Peikert's method works best if the DESIRED STANDARD DEVIATION IS SMALL and
 * THE MEAN OF THE DISTRIBUTION IS FIXED, as each new center will require a new
 * set of precomputations.
 *
 * Second base sampler is  the Knuth-Yao Sampler discussed in section 5 of
 * https://link.springer.com/content/pdf/10.1007%2Fs00200-014-0218-3.pdf .
 * Similar to Peikert's, Knuth-Yao precomputes the PDF's of the numbers based on
 * standard deviation and the center, which is used during the sampling process.
 * Therefore like Peikert's method,  Knuth-Yao works best method works best if
 * the DESIRED STANDARD DEVIATION IS SMALL and THE MEAN OF THE DISTRIBUTION IS
 * FIXED, as each new center will require a new set of precomputations, just
 * like Peikert's inversion method.
 *
 * The "generic sampler" on the other hand, works independent from standard
 * deviation of the distribution. It combines an array of previously discussed
 * base samplers centered around 0 to (2^b-1) / 2^b through convolution. The
 * tables of base samplers however, must be precomputed beforehand; but they do
 * not need to be recalculated at any time of the sampling process. It is USABLE
 * FOR ANY STANDARD DEVIATION AND MEAN, just like Karney's method defined in
 * discretegaussiangenerator.h, needs only one single precomputation and is not
 * prone to timing attacks unlike Karney. Karney's method, however, is faster
 * than the generic sampler.
 *
 * PARAMETER SELECTION FOR GENERIC SAMPLER
 *
 * The selection of parameters change the run time/memory usage/precision of the
 * generic sampler. The triple trade off between these parameters are defined in
 * the equation k = (PRECISION - FLIPS) / LOG_BASE. k denotes the level of
 * precision of the generic sampler. Higher the k is, higher the precision of
 * the generic sampler but higher the run time. PRECISION denotes the number of
 * decimal bits in the center of the distribution. Since we are using 'double'
 * for mean, it is fixed to 53 by definition. FLIPS denote the number of
 * Bernoulli flips used to approximate the bits used in combination of base
 * sampler. Higher the number of flips, larger the number of bits approximated
 * rather than calculated which means smaller run times. Generic sampler
 * requires a set of base samplers centered around 0/2^b to (2^b-1)/2^b;
 * LOG_BASE denotes b in this equation. Higher the LOG_BASE is, more base
 * samplers required which requires additional memory; but at the same time
 * smaller run times.
 *
 * The base samplers used in generic sampler requires varying centers between
 * 0/2^b and (2^b-1)/(2^b) with the same standard deviation. The standard
 * deviation required for base samplers must satisfy SIGMA>=4*SQRT(2)*N, where
 * sigma is the standard deviation of the base sampler and N is the smoothing
 * parameter
 *
* */

use rand::Rng;

const MAX_LEVELS: usize = 4;

enum BaseSamplerType {
    KNUTH_YAO = 0,
    PEIKERT = 1,
}

struct BitGenerator {
    sequence: u32,
    counter: u32,
}

impl BitGenerator {
    fn new() -> Self {
        BitGenerator {
            sequence: 0,
            counter: 0,
        }
    }

    fn generate(&mut self) -> i16 {
        if self.counter == 0 {
            self.sequence = rand::thread_rng().gen();
            self.counter = 32;
        }
        let bit = (self.sequence >> (self.counter - 1)) & 0x1;
        self.counter -= 1;
        bit as i16
    }
}

struct BaseSampler {
    b_a: f64,
    b_mean: i64,
    b_std: f32,
    bg: BitGenerator,
    b_type: BaseSamplerType,
    fin: i32,
    ddg_tree: Vec<Vec<i16>>,
    hamming_weights: Vec<u32>,
    b_matrix_size: i32,
    first_non_zero: i32,
    end_index: i32,
    m_vals: Vec<f64>,
}

impl BaseSampler {
    fn new(mean: f64, std: f64, generator: &mut BitGenerator, b_type: BaseSamplerType) -> Self {
        BaseSampler {
            b_a: 0.0,
            b_mean: 0,
            b_std: 0.0,
            bg: generator.clone(),
            b_type,
            fin: 0,
            ddg_tree: Vec::new(),
            hamming_weights: Vec::new(),
            b_matrix_size: 0,
            first_non_zero: 0,
            end_index: 0,
            m_vals: Vec::new(),
        }
    }

    fn generate_integer(&mut self) -> i64 {
        // TODO: Implement GenerateInteger function
        0
    }

    fn random_bit(&mut self) -> i16 {
        self.bg.generate()
    }

    fn find_in_vector(&self, s: &[f64], search: f64) -> u32 {
        // TODO: Implement FindInVector function
        0
    }

    fn generate_ddg_tree(&mut self, prob_matrix: &[u64]) {
        // TODO: Implement GenerateDDGTree function
    }

    fn initialize(&mut self, mean: f64) {
        // TODO: Implement Initialize function
    }

    fn generate_prob_matrix(&mut self, stddev: f64, mean: f64) {
        // TODO: Implement GenerateProbMatrix function
    }

    fn generate_integer_knuth_yao(&mut self) -> i64 {
        // TODO: Implement GenerateIntegerKnuthYao function
        0
    }

    fn generate_integer_peikert(&self) -> i64 {
        // TODO: Implement GenerateIntegerPeikert function
        0
    }
}

struct SamplerCombiner<'a> {
    sampler1: &'a mut BaseSampler,
    sampler2: &'a mut BaseSampler,
    x1: i64,
    x2: i64,
}

impl<'a> SamplerCombiner<'a> {
    fn new(s1: &'a mut BaseSampler, s2: &'a mut BaseSampler, z1: i64, z2: i64) -> Self {
        SamplerCombiner {
            sampler1: s1,
            sampler2: s2,
            x1: z1,
            x2: z2,
        }
    }

    fn generate_integer(&mut self) -> i64 {
        self.x1 * self.sampler1.generate_integer() + self.x2 * self.sampler2.generate_integer()
    }
}

struct DiscreteGaussianGeneratorGeneric<'a> {
    base_samplers: &'a mut [BaseSampler],
    wide_sampler: &'a mut BaseSampler,
    combiners: [&'a mut BaseSampler; MAX_LEVELS],
    wide_variance: f64,
    sampler_variance: f64,
    x: f64,
    c: f64,
    ci: f64,
    k: i32,
    log_base: i32,
    mask: u64,
}

impl<'a> DiscreteGaussianGeneratorGeneric<'a> {
    fn new(samplers: &'a mut [BaseSampler], std: f64, b: i32, n: f64) -> Self {
        DiscreteGaussianGeneratorGeneric {
            base_samplers: samplers,
            wide_sampler: &mut samplers[0],
            combiners: [&mut samplers[0]; MAX_LEVELS],
            wide_variance: 0.0,
            sampler_variance: 0.0,
            x: 0.0,
            c: 0.0,
            ci: 0.0,
            k: 0,
            log_base: 0,
            mask: 0,
        }
    }

    fn generate_integer(&mut self, mean: f64, std: f64) -> i64 {
        // TODO: Implement GenerateInteger function
        0
    }

    fn flip_and_round(&self, center: f64) -> i64 {
        // TODO: Implement flipAndRound function
        0
    }

    fn sample_c(&self, center: i64) -> i64 {
        // TODO: Implement SampleC function
        0
    }

    fn extract_bit(&self, number: i64, n: i32) -> i16 {
        ((number >> n) & 0x1) as i16
    }
}