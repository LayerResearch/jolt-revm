use revm::{
    precompile::{
        bn128::{mul::ISTANBUL_MUL_GAS_COST, run_add, run_mul},
        modexp::berlin_run,
        secp256k1::{ec_recover_run, ecrecover},
    },
    primitives::{hex, Bytes},
};

pub fn test() {
    let input_str = "\
            0000000000000000000000000000000000000000000000000000000000000064\
            0000000000000000000000000000000000000000000000000000000000000064\
            0000000000000000000000000000000000000000000000000000000000000064\
            5442ddc2b70f66c1f6d2b296c0a875be7eddd0a80958cbc7425f1899ccf90511\
            a5c318226e48ee23f130b44dc17a691ce66be5da18b85ed7943535b205aa125e\
            9f59294a00f05155c23e97dac6b3a00b0c63c8411bf815fc183b420b4d9dc5f7\
            15040d5c60957f52d334b843197adec58c131c907cd96059fc5adce9dda351b5\
            df3d666fcf3eb63c46851c1816e323f2119ebdf5ef35";
    let input = hex::decode(input_str).unwrap().into();

    for _ in 0..2_000_000 {
        let res = berlin_run(&input, 100_000_000).unwrap();
        core::hint::black_box(res);
    }
}
