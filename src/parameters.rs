use std::mem;

use crate::{fhe_schemes::FHEScheme, scale_technique::ScalingTechnique};

#[allow(dead_code)]
pub struct SchemeParameters {
    scheme: FHEScheme,

    // PlaintextModulus ptModulus is used in BGV/BFV type schemes and impacts noise growth
    pt_modulus: u64,

    // digitSize is used in BV Key Switching only (KeySwitchTechnique = BV) and impacts noise growth
    digit_size: u32,

    /// standardDeviation is used for Gaussian error generation
    standard_deviation: f64,

    /// Secret key distribution: GAUSSIAN, UNIFORM_TERNARY, etc.
    secret_key_distribution: SecretKeyDistribution,

    // Max relinearization degree of secret key polynomial (used for lazy relinearization)
    max_relinearization_secret_key_degree: u32,

    // key switching technique: BV or HYBRID currently
    // For BV we do not have extra modulus, so the security depends on ciphertext modulus Q.
    // For HYBRID we do have extra modulus P, so the security depends on modulus P*Q
    // For BV we need digitSize - digit size in digit decomposition
    // For HYBRID we need numLargeDigits - number of digits in digit decomposition
    // it is good to have alternative to numLargeDigits (possibly numPrimesInDigit?)
    key_switch_technique: KeySwitchTechnique,

    // rescaling/modulus switching technique used in CKKS/BGV: FLEXIBLEAUTOEXT, FIXEDMANUL, FLEXIBLEAUTO, etc.
    // see https://eprint.iacr.org/2022/915 for details
    scale_technique: ScalingTechnique,

    // max batch size of messages to be packed in encoding (number of slots)
    batch_size: u32,

    // PRE security mode
    proxy_reencryption_mode: Option<ProxyReEncryptionMode>,

    // Multiparty security mode in BFV/BGV
    // NOISE_FLOODING_MULTIPARTY is more secure than FIXED_NOISE_MULTIPARTY.
    multiparty_mode: MultipartyMode,

    // Execution mode in CKKS
    // In EXEC_NOISE_ESTIMATION mode, we estimate the noise we need to add to the actual computation to guarantee good security.
    // In EXEC_EVALUATION mode, we input our noise estimate and perform the desired secure encrypted computation.
    execution_mode: ExecutionMode,

    // Decryption noise mode in CKKS
    // NOISE_FLOODING_DECRYPT is more secure than FIXED_NOISE_DECRYPT, but it requires executing all computations twice.
    decryption_noise_mode: DecryptionNoiseMode,

    // Noise estimate in CKKS for NOISE_FLOODING_DECRYPT mode.
    // This estimate is obtained from running the computation in EXEC_NOISE_ESTIMATION mode.
    noise_estimate: f64,

    // Desired precision for 128-bit CKKS. We use this value in NOISE_FLOODING_DECRYPT mode to determine the scaling factor.
    desired_precision: f64,

    // Statistical security of CKKS in NOISE_FLOODING_DECRYPT mode. This is the bound on the probability of success
    // that any adversary can have. Specifically, they a probability of success of at most 2^(-statisticalSecurity).
    statistical_security: u32,

    // This is the number of adversarial queries a user is expecting for their application, which we use to ensure
    // security of CKKS in NOISE_FLOODING_DECRYPT mode.
    num_adversarial_queries: u32,

    // This is the number of parties in a threshold application, which is used for bound on the joint secret key
    threshold_num_of_parties: u32,

    // firstModSize and scalingModSize are used to calculate ciphertext modulus. The ciphertext modulus should be seen as:
    // Q = q_0 * q_1 * ... * q_n * q'
    // where q_0 is first prime, and it's number of bits is firstModSize
    // other q_i have same number of bits and is equal to scalingModSize
    // the prime q' is not explicitly given,
    // but it is used internally in CKKS and BGV schemes (in *EXT scaling methods)
    first_mod_size: u32,
    scaling_mod_size: u32,

    // see KeySwitchTechnique - number of digits in HYBRID key switching
    num_large_digits: u32,

    // multiplicative depth
    multiplicative_depth: u32,

    // security level:
    // We use the values from the security standard  at
    // http://homomorphicencryption.org/wp-content/uploads/2018/11/HomomorphicEncryptionStandardv1.1.pdf
    // For given ring dimension and security level we have
    // upper bound of possible highest modulus (Q for BV or P*Q for HYBRID)
    security_level: SecurityLevel,

    // ring dimension N of the scheme : the ring is Z_Q[x] / (X^N+1)
    ring_dim: u32,

    // number of additions (used for setting noise in BGV and BFV)
    eval_add_count: u32,

    // number of key switching operations (used for setting noise in BGV and BFV)
    key_switch_count: u32,

    // size of moduli used for PRE in the provable HRA setting
    multi_hop_mod_size: u32,

    // STANDARD or EXTENDED mode for BFV encryption
    // EXTENDED slightly reduces the size of Q (by few bits) but makes encryption somewhat slower
    // see https://eprint.iacr.org/2022/915 for details
    encryption_technique: EncryptionTechnique,

    // multiplication method in BFV: BEHZ, HPS, etc.
    // see https://eprint.iacr.org/2022/915 for details
    multiplication_technique: MultiplicationTechnique,

    // Interactive multi-party bootstrapping parameter
    // Set the compression level in ciphertext (SLACK or COMPACT)
    // SLACK has weaker security assumption, thus less efficient
    // COMPACT has stronger security assumption, thus more efficient
    interactive_boot_compression_level: CompressionLevel,
}

impl SchemeParameters {
    fn set_to_defaults(&mut self, scheme: FHEScheme) {
        match scheme {
            FHEScheme::CKKSRNS => {
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        pt_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::HYBRID,
                        scale_technique: ScalingTechnique::FlexibleAutoExt,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 25.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 60,
                        scaling_mod_size: 59,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dim: 0,
                        eval_add_count: 0,
                        key_switch_count: 0,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPS,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
            FHEScheme::BFVRNS => {
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        pt_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::BV,
                        scale_technique: ScalingTechnique::NoRescale,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 0.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 60,
                        scaling_mod_size: 60,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dim: 0,
                        eval_add_count: 0,
                        key_switch_count: 0,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPSPOVERQLEVELED,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
            FHEScheme::BGVRNS => {
                // TODO: Сверить с src/pke/include/scheme/gen-cryptocontext-params-defaults.h
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        pt_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::HYBRID,
                        scale_technique: ScalingTechnique::FlexibleAutoExt,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 25.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 60,
                        scaling_mod_size: 59,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dim: 0,
                        eval_add_count: 0,
                        key_switch_count: 0,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPS,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
        }
        todo!()
    }

    fn validate_ring_dim(ring_dim: u32) {
        todo!()
    }

    fn validate_multiplicative_depth(multiplicative_depth: u32) {
        todo!()
    }
}

pub type Params = SchemeParameters;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretKeyDistribution {
    Gaussian = 0,
    // UNIFORM_BINARY,
    #[default]
    UniformTernary = 1,
    SparseTernary = 2,
    // UNIFORM_QUATERNARY,
    // UNIFORM_QUINARY,
}

pub enum KeySwitchTechnique {
    Invalid = 0,
    BV,
    HYBRID,
}

pub enum ProxyReEncryptionMode {
    InDCPA,
    FixedNoiseHRA,
    NoiseFloodingHRA,
    DivideAndRoundHRA,
}

pub enum MultipartyMode {
    InvalidMultipartyMode = 0,
    FixedNoiseMultiparty,
    NoiseFloodingMultiparty,
}

pub enum ExecutionMode {
    Evaluation = 0,
    NoiseEstimation,
}

pub enum DecryptionNoiseMode {
    FixedNoiseDecrypt = 0,
    NoiseFloodingDecrypt,
}

pub enum SecurityLevel {
    HEstd128Classic,
    HEstd192Classic,
    HEstd256Classic,
    HEstd128Quantum,
    HEstd192Quantum,
    HEstd256Quantum,
    HEstdNotSet,
}

pub enum EncryptionTechnique {
    STANDARD = 0,
    EXTENDED,
}

pub enum MultiplicationTechnique {
    BEHZ = 0,
    HPS,
    HPSPOVERQ,
    HPSPOVERQLEVELED,
}

// Defining the level to which the input ciphertext is brought to before
// interactive multi-party bootstrapping
enum CompressionLevel {
    // we don't support 0 or 1 compression levels
    // do not change values here
    Compact = 2, // more efficient with stronger security assumption
    Slack = 3,   // less efficient with weaker security assumption
}
