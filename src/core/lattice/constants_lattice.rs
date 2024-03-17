use std::fmt;

#[derive(Debug)]
pub enum SecretKeyDist {
    Gaussian,
    UniformTernary,
    SparseTernary,
}

impl SecretKeyDist {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "GAUSSIAN" => Some(SecretKeyDist::Gaussian),
            "UNIFORM_TERNARY" => Some(SecretKeyDist::UniformTernary),
            "SPARSE_TERNARY" => Some(SecretKeyDist::SparseTernary),
            _ => None,
        }
    }

    pub fn from_u32(num: u32) -> Option<Self> {
        match num {
            0 => Some(SecretKeyDist::Gaussian),
            1 => Some(SecretKeyDist::UniformTernary),
            2 => Some(SecretKeyDist::SparseTernary),
            _ => None,
        }
    }
}

impl fmt::Display for SecretKeyDist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SecretKeyDist::Gaussian => "GAUSSIAN",
                SecretKeyDist::UniformTernary => "UNIFORM_TERNARY",
                SecretKeyDist::SparseTernary => "SPARSE_TERNARY",
            }
        )
    }
}


