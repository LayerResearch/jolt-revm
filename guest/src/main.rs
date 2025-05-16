#![cfg_attr(feature = "guest", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

use revm_guest as guest;

/// This is required to resolve `undefined reference to `_critical_section_1_0_acquire'`
#[cfg(target_arch = "riscv32")]
#[allow(unused_imports)]
use riscv as _;

#[allow(unused_imports)]
use guest::*;
