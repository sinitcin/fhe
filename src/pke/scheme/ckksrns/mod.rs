use std::{marker::PhantomData, ops::Deref};

use crate::{fhe_schemes::FHEScheme, pke::scheme::gen_cryptocontext_parameters::SchemeParameters};

pub struct CryptoContextCKKSRNS;

// every CCParams class should include the following forward declaration as there is
// no general CCParams class template. This way we may create scheme specific classes
// derived from Params or have them completely independent.
pub struct CryptoContextParams<T> {
    inner: SchemeParameters,
    phantom: PhantomData<T>,
}
pub type CCParams<T> = CryptoContextParams<T>;

impl CryptoContextParams<CryptoContextCKKSRNS> {
    pub fn new() -> CryptoContextParams<CryptoContextCKKSRNS> {
        Self {
            inner: SchemeParameters::new(FHEScheme::CKKSRNS),
            phantom: PhantomData::<CryptoContextCKKSRNS>,
        }
    }
}

impl<T> Deref for CryptoContextParams<T> {
    type Target = SchemeParameters;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
