use crypto_hashes::digest::Digest;
use revm::{precompile::hash::ripemd160_run, primitives::Bytes};

pub fn test() {
    const SIZE: usize = 10 * 48 * 1024;
    let input = [0; SIZE];
    for _ in 0..100 {
        let res = ripemd160_run(&Bytes::from(input), 100_000_000).unwrap();
        core::hint::black_box(res);
    }
}
