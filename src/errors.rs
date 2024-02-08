#[derive(Debug, thiserror::Error)]
pub enum FHEError {
    #[error("Invalid scheme: {0}")]
    InvalidScheme(String),
    #[error("Invalid ring dimension: {0}. Ring dimension must be a power of 2")]
    InvalidRingDimension(u32),
    #[error("The provided multiplicative depth: {0}. Multiplicative depth is not computationally feasible")]
    InvalidMultiplicativeDepth(u32),
}
