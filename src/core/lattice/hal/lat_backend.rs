// Assuming the equivalent Rust modules and structures are defined in a similar path and manner
// and that the conditional compilation features are defined in Cargo.toml

// Importing necessary modules
// mod lattice {
//     pub mod hal {
//         pub mod default {
//             pub mod ilparams;
//             pub mod ildcrtparams;
//             pub mod poly;
//             pub mod dcrtpoly;
//         }
//     }
// }

use crate::core::lattice::hal::default::ilparams::ILParamsImpl;
use crate::core::lattice::hal::default::ildcrtparams::ILDCRTParams;
use crate::core::lattice::hal::default::poly::PolyImpl;
use crate::core::lattice::hal::default::dcrtpoly::DCRTPolyImpl;

    use super::*;

    pub type ILNativeParams = ILParamsImpl<NativeInteger>;
    pub type ILParams = ILParamsImpl<BigInteger>;
    pub type Poly = PolyImpl<BigVector>;
    pub type NativePoly = PolyImpl<NativeVector>;
    pub type DCRTPoly = DCRTPolyImpl<BigVector>;

    #[cfg(feature = "with_be2")]
    pub type M2Params = ILParamsImpl<M2Integer>;
    #[cfg(feature = "with_be2")]
    pub type M2DCRTParams = ILDCRTParams<M2Integer>;
    #[cfg(feature = "with_be2")]
    pub type M2Poly = PolyImpl<M2Vector>;
    #[cfg(feature = "with_be2")]
    pub type M2DCRTPoly = DCRTPolyImpl<M2Vector>;

    #[cfg(not(feature = "with_be2"))]
    pub type M2Params = ();
    #[cfg(not(feature = "with_be2"))]
    pub type M2DCRTParams = ();
    #[cfg(not(feature = "with_be2"))]
    pub type M2Poly = ();
    #[cfg(not(feature = "with_be2"))]
    pub type M2DCRTPoly = ();

    #[cfg(feature = "with_be4")]
    pub type M4Params = ILParamsImpl<M4Integer>;
    #[cfg(feature = "with_be4")]
    pub type M4DCRTParams = ILDCRTParams<M4Integer>;
    #[cfg(feature = "with_be4")]
    pub type M4Poly = PolyImpl<M4Vector>;
    #[cfg(feature = "with_be4")]
    pub type M4DCRTPoly = DCRTPolyImpl<M4Vector>;

    #[cfg(not(feature = "with_be4"))]
    pub type M4Params = ();
    #[cfg(not(feature = "with_be4"))]
    pub type M4DCRTParams = ();
    #[cfg(not(feature = "with_be4"))]
    pub type M4Poly = ();
    #[cfg(not(feature = "with_be4"))]
    pub type M4DCRTPoly = ();

    #[cfg(feature = "with_ntl")]
    pub type M6Params = ILParamsImpl<M6Integer>;
    #[cfg(feature = "with_ntl")]
    pub type M6DCRTParams = ILDCRTParams<M6Integer>;
    #[cfg(feature = "with_ntl")]
    pub type M6Poly = PolyImpl<M6Vector>;
    #[cfg(feature = "with_ntl")]
    pub type M6DCRTPoly = DCRTPolyImpl<M6Vector>;

    #[cfg(not(feature = "with_ntl"))]
    pub type M6Params = ();
    #[cfg(not(feature = "with_ntl"))]
    pub type M6DCRTParams = ();
    #[cfg(not(feature = "with_ntl"))]
    pub type M6Poly = ();
    #[cfg(not(feature = "with_ntl"))]
    pub type M6DCRTPoly = ();
