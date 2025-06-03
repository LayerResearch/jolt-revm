use revm::{
    precompile::{
        blake2::run,
        bn128::{mul::ISTANBUL_MUL_GAS_COST, run_add, run_mul},
        secp256k1::{ec_recover_run, ecrecover},
    },
    primitives::{hex, Bytes},
};


pub fn test() {
    let mut input = [1; 213];

    input[0] = 0;
    input[1] = 0;
    input[3] = 0;
    //round = 256

    for _ in 0..5000 {
        let res = run(&Bytes::from(input), 10000000000).unwrap();
        core::hint::black_box(res);
    }
}
