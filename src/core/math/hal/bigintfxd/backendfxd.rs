
/*
  This file contains the definitions for MATHBACKEND 2 also known as the fixed multi-precision backend.
  This backend supports fixed bitwidths; Uses bigintfxd:: definition as default;
  Implemented as a vector of integers.
  Configurable maximum bit length and type of underlying integer
 */


 // Assuming the functionality of the C++ code is to conditionally include and use certain types based on compilation flags,
// and considering Rust doesn't have a direct preprocessor feature like C++, we'll use feature flags in Cargo.toml and conditional compilation in Rust.

// In Cargo.toml, you would define features like this:
// [features]
// default = []
// with_be2 = []

// Then, in your Rust code:

#[cfg(feature = "with_be2")]
mod bigintfxd {
    // Assuming the bigintfxd module and its types are defined elsewhere in your Rust project or in a dependency.
    pub use crate::math::hal::bigintfxd::ubintfxd::BigInteger;
    pub use crate::math::hal::bigintfxd::mubintvecfxd::BigVector;
    // Other imports or definitions related to bigintfxd...
}

#[cfg(feature = "with_be2")]
type M2Integer = bigintfxd::BigInteger;

#[cfg(feature = "with_be2")]
type M2Vector = bigintfxd::BigVector;

#[cfg(not(feature = "with_be2"))]
type M2Integer = (); // Rust doesn't have a direct equivalent to C++'s `void` for types, using `()` as a placeholder.


