#[cfg(feature = "precompile_blake2")]
pub mod precompile_blake2;
#[cfg(feature = "precompile_blake2")]
pub use precompile_blake2::test;

#[cfg(feature = "precompile_BLS12_G1ADD")]
pub mod precompile_BLS12_G1ADD;
#[cfg(feature = "precompile_BLS12_G1ADD")]
pub use precompile_BLS12_G1ADD::test;

#[cfg(feature = "precompile_BLS12_G1MSM")]
pub mod precompile_BLS12_G1MSM;
#[cfg(feature = "precompile_BLS12_G1MSM")]
pub use precompile_BLS12_G1MSM::test;

#[cfg(feature = "precompile_BLS12_G2ADD")]
pub mod precompile_BLS12_G2ADD;
#[cfg(feature = "precompile_BLS12_G2ADD")]
pub use precompile_BLS12_G2ADD::test;

#[cfg(feature = "precompile_BLS12_G2MSM")]
pub mod precompile_BLS12_G2MSM;
#[cfg(feature = "precompile_BLS12_G2MSM")]
pub use precompile_BLS12_G2MSM::test;

#[cfg(feature = "precompile_BLS12_MAP_FP_TO_G1")]
pub mod precompile_BLS12_MAP_FP_TO_G1;
#[cfg(feature = "precompile_BLS12_MAP_FP_TO_G1")]
pub use precompile_BLS12_MAP_FP_TO_G1::test;

#[cfg(feature = "precompile_BLS12_MAP_FP2_TO_G2")]
pub mod precompile_BLS12_MAP_FP2_TO_G2;
#[cfg(feature = "precompile_BLS12_MAP_FP2_TO_G2")]
pub use precompile_BLS12_MAP_FP2_TO_G2::test;

#[cfg(feature = "precompile_BLS12_PAIRING_CHECK")]
pub mod precompile_BLS12_PAIRING_CHECK;
#[cfg(feature = "precompile_BLS12_PAIRING_CHECK")]
pub use precompile_BLS12_PAIRING_CHECK::test;

#[cfg(feature = "precompile_ec_add")]
pub mod precompile_ec_add;
#[cfg(feature = "precompile_ec_add")]
pub use precompile_ec_add::test;

#[cfg(feature = "precompile_ec_mul")]
pub mod precompile_ec_mul;
#[cfg(feature = "precompile_ec_mul")]
pub use precompile_ec_mul::test;

#[cfg(feature = "precompile_ec_pair")]
pub mod precompile_ec_pair;
#[cfg(feature = "precompile_ec_pair")]
pub use precompile_ec_pair::test;

#[cfg(feature = "precompile_ecrecover")]
pub mod precompile_ecrecover;
#[cfg(feature = "precompile_ecrecover")]
pub use precompile_ecrecover::test;

#[cfg(feature = "precompile_kzg_point")]
pub mod precompile_kzg_point;
#[cfg(feature = "precompile_kzg_point")]
pub use precompile_kzg_point::test;

#[cfg(feature = "precompile_modexp")]
pub mod precompile_modexp;
#[cfg(feature = "precompile_modexp")]
pub use precompile_modexp::test;

#[cfg(feature = "precompile_ripemd160")]
pub mod precompile_ripemd160;
#[cfg(feature = "precompile_ripemd160")]
pub use precompile_ripemd160::test;

#[cfg(feature = "precompile_sha256")]
pub mod precompile_sha256;
#[cfg(feature = "precompile_sha256")]
pub use precompile_sha256::test;

#[cfg(feature = "solidity_fibonacci")]
pub mod solidity_fibonacci;
#[cfg(feature = "solidity_fibonacci")]
pub use solidity_fibonacci::test;

#[cfg(feature = "solidity_sqrt")]
pub mod solidity_sqrt;
#[cfg(feature = "solidity_sqrt")]
pub use solidity_sqrt::test;

#[cfg(feature = "noop")]
pub mod noop;
#[cfg(feature = "noop")]
pub use noop::test;



