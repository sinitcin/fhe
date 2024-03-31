#[cfg(target_pointer_width = "128")]
const MAX_MODULUS_SIZE: u32 = 121;
type BasicInteger = u128;
type DoubleNativeInt = u128;
type uint128_t = u128;
type int128_t = i128;

#[cfg(target_pointer_width = "64")]
#[cfg(feature = "int128")]
const MAX_MODULUS_SIZE: u32 = 60;
#[cfg(target_pointer_width = "64")]
#[cfg(feature = "int128")]
type BasicInteger = u64;
#[cfg(target_pointer_width = "64")]
#[cfg(feature = "int128")]
type DoubleNativeInt = u128;
#[cfg(target_pointer_width = "64")]
#[cfg(feature = "int128")]
type uint128_t = u128;
#[cfg(target_pointer_width = "64")]
#[cfg(feature = "int128")]
type int128_t = i128;

#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "int128"))]
const MAX_MODULUS_SIZE: u32 = 57;
#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "int128"))]
type BasicInteger = u64;
#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "int128"))]
type DoubleNativeInt = u64;
#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "int128"))]
type uint128_t = u64;
#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "int128"))]
type int128_t = i64;

#[cfg(target_pointer_width = "32")]
const MAX_MODULUS_SIZE: u32 = 28;
#[cfg(target_pointer_width = "32")]
type BasicInteger = u32;
#[cfg(target_pointer_width = "32")]
type DoubleNativeInt = u64;
#[cfg(target_pointer_width = "32")]
type uint128_t = u64;
#[cfg(target_pointer_width = "32")]
type int128_t = i64;


