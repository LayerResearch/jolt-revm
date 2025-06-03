use revm::{
    precompile::bls12_381::g1_add::PRECOMPILE,
    primitives::{hex, Bytes},
};

pub fn test() {
    let input = hex::decode(
        "000000000000000000000000000000000e8665724fa74bb34215b1d774a93d6958fbcee1bf34fdd20b2f71cf312bcd087c3880af08593ac95b4e85624d72794f000000000000000000000000000000000b550f3c34a9753c632cf641c25ff4d5cf4fafe576e6ed0c2ac3c984df4139d75e474f493578ebeeb4497c4e40b0fa9f0000000000000000000000000000000004f387c4a4ea043717a073ece1fdad218dc4c619799d76cfe3910c9d64fb76e87105c6ad822872c5a23a446ee333705c00000000000000000000000000000000076d882de6cde580e6f4146b6a7dcf1030b5d523a5909ed80dc4ba79818faf5b60dd0ba010ed68a9b2ba2037f350b70a",
    )
    .unwrap();
    let input_bytes = Bytes::copy_from_slice(&input);

    for _ in 0..5000 {
        let precompile = *PRECOMPILE.precompile();
        let res = precompile(&input_bytes, u64::MAX).unwrap();
        core::hint::black_box(res);
    }
}
