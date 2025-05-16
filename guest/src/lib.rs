#![cfg_attr(feature = "guest", no_std)]
use jolt_sdk as jolt;

use revm::{
    context::{BlockEnv, TxEnv},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

#[jolt::provable]
fn fib(n: u32) -> u128 {
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

#[jolt::provable]
fn exec(_n: u32) -> u128 {
    use revm::{
        database::{CacheDB, EmptyDB},
        primitives::{address, keccak256, Bytes, TxKind, U256},
        state::AccountInfo,
    };

    let mut cache = CacheDB::new(EmptyDB::default());
    let one_ether = U256::from(10_000_000_000_000_000_000u128);
    let account = AccountInfo {
        nonce: 0_u64,
        balance: one_ether,
        code_hash: keccak256(Bytes::new()),
        code: None,
    };
    let sender = address!("0000000000000000000000000000000000000080");
    let receiver = address!("0000000000000000000000000000000000000081");
    cache.insert_account_info(sender, account);

    let mut tx = TxEnv::default();
    tx.caller = sender;
    tx.kind = TxKind::Call(receiver);
    tx.value = U256::from(0_500_000_000u128);
    tx.gas_limit = 100_000u64;
    tx.gas_price = 1_000_000_000u128;
    tx.nonce = 0_u64;

    let mut revm = Context::mainnet()
        .with_db(cache)
        .with_block(BlockEnv::default())
        .with_tx(&tx)
        .build_mainnet();

    let result = revm.replay();
    result.is_ok().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        assert!(exec(1) == 1);
    }
}
