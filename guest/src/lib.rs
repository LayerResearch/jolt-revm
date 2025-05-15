#![cfg_attr(feature = "guest", no_std)]
use jolt_sdk as jolt;

use revm::{Context, MainBuilder, MainContext};

#[jolt::provable]
fn exec(n: u32) {
    let revm = Context::mainnet().build_mainnet();
}
