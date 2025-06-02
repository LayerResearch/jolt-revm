#![cfg_attr(any(feature = "guest", feature = "no-jolt"), no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

mod riscv32im;

use revm_guest as guest;

/// This is required to resolve `undefined reference to `_critical_section_1_0_acquire'`
#[cfg(target_arch = "riscv32")]
#[allow(unused_imports)]
use riscv as _;

#[allow(unused_imports)]
use guest::*;

#[cfg(feature = "no-jolt")]
#[no_mangle]
pub extern "C" fn main() -> i32 {
    let result = exec(0);
    0
}
