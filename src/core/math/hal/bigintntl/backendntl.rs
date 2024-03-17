
/*
  This file contains the definitions for MATHBACKEND 6 also known as the NTL multi-precision backend.
  This backend uses bigintntl:: definition as default.
  requires GMP 6.1.2 / NTL 10.3.0 backend
 */

 // Assuming the functionality provided by NTL is similar to the `num-bigint` crate in Rust.
// First, add `num-bigint` to your `Cargo.toml` dependencies.

use num_bigint::{BigInt, BigUint};

#[cfg(feature = "with_ntl")]
mod bigintntl {
    pub use num_bigint::{BigInt as M6Integer, BigUint as M6Vector};
    // Additional functionality can be implemented or imported here
    // based on the specific needs that match the NTL library's capabilities.
}

#[cfg(not(feature = "with_ntl"))]
mod bigintntl {
    pub type M6Integer = (); // Rust doesn't have a direct equivalent to 'void', so we use '()' for a similar effect.
}

