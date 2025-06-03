#![cfg_attr(any(feature = "guest", feature = "no-jolt"), no_std)]

use jolt_sdk as jolt;

use core::option::Option::None;
use revm::{
    context::{BlockEnv, TxEnv},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

#[cfg_attr(not(feature = "no-jolt"), jolt::provable)]
pub fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}

mod utils;
#[cfg_attr(not(feature = "no-jolt"), jolt::provable)]
pub fn exec(_n: u32) -> u128 {
    use utils::test;
    test();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        assert!(exec(1) == 1);
    }
}
