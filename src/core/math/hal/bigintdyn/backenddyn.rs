/*
  This file contains the definitions for MATHBACKEND 4 also known as the dynamic multi-precision backend.
  This backend supports arbitrary bitwidths; no memory pool is used; can grow up to RAM limitation.
  Configurable type of underlying integer (either 32 or 64 bit)
 */

 #[cfg(feature = "with_be4")]
mod be4 {
    pub mod bigintdyn {
        pub struct BigInteger;
        pub struct BigVector;
    }

    pub use bigintdyn::BigInteger as M4Integer;
    pub use bigintdyn::BigVector as M4Vector;
}

#[cfg(not(feature = "with_be4"))]
mod no_be4 {
    pub type M4Integer = ();
}


