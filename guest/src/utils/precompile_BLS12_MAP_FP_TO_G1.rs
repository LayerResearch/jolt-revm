use revm::{
    precompile::bls12_381::map_fp_to_g1::PRECOMPILE,
    primitives::{hex, Bytes},
};

pub fn test() {
    let input = hex::decode(
        "0x0000000000000000000000000000000016f6b59f8df4344269685680b9e2e3750321051ca0f8e064d480e2a57f07073e609993e1667326b477ddb78ac52b3e8a",
    )
    .unwrap();
    let input_bytes = Bytes::copy_from_slice(&input);

    for _ in 0..5000 {
        let precompile = *PRECOMPILE.precompile();
        let res = precompile(&input_bytes, u64::MAX).unwrap();
        core::hint::black_box(res);
    }
}
