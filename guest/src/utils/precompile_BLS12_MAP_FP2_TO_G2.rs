use revm::{
    precompile::bls12_381::map_fp2_to_g2::PRECOMPILE,
    primitives::{hex, Bytes},
};

pub fn test() {
    let input = hex::decode(
        "0000000000000000000000000000000016f6b59f8df4344269685680b9e2e3750321051ca0f8e064d480e2a57f07073e609993e1667326b477ddb78ac52b3e8a000000000000000000000000000000000fe06fc6b10b4bd2b7f217c5a107c0edaa2c6fc87ab160fd6574d4fa3903871dd8d9f31f0d595bfeefc9b29a2d8ca0c3",
    )
    .unwrap();
    let input_bytes = Bytes::copy_from_slice(&input);

    for _ in 0..5000 {
        let precompile = *PRECOMPILE.precompile();
        let res = precompile(&input_bytes, u64::MAX).unwrap();
        core::hint::black_box(res);
    }
}
