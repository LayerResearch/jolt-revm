#![no_std]
#![no_main]
extern crate alloc;
extern crate core;
extern crate serde;
extern crate spin;

mod riscv32im;

/// This is required to resolve `undefined reference to `_critical_section_1_0_acquire'`
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[allow(unused_imports)]
use riscv;

use alloc::sync::Arc;
use core::time::Duration;
use default_env::default_env;
use htif::println;
use revm_statetest_types::TestSuite;
use statetest_measure::compat::Mutex;
use statetest_measure::execute_test_suite;
use statetest_measure::prelude::*;

// Embedded JSON test fixture
const TEST_JSON: &str = include_str!(default_env!("TEST_JSON_PATH",
    "../../../execution-spec-tests/stable/state_tests/frontier/opcodes/all_opcodes/all_opcodes.json"
    ));

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Call the statetest function which will be measured by spike
    let result = statetest();
    if result {
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn statetest() -> bool {
    // Parse the embedded JSON test fixture
    let test_suite: TestSuite = match serde_json::from_str(TEST_JSON) {
        Ok(suite) => suite,
        Err(e) => {
            println!("JSON Parse Error: {}", e);
            return false; // Return false if parsing fails
        }
    };

    let elapsed = Arc::new(Mutex::new(Duration::ZERO));

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    let start_instret = riscv::register::minstret::read();

    let result = execute_test_suite("", &test_suite, &elapsed, false, false).is_ok();

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    {
        let end_instret = riscv::register::minstret::read();
        let instret = end_instret - start_instret;
        println!("instret: {}", instret);
    }

    result
}
