use crate::{fhe_schemes::FHEScheme, scale_technique::ScalingTechnique};

#[allow(dead_code)]
pub struct SchemeParameters {
    scheme: FHEScheme,
    
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