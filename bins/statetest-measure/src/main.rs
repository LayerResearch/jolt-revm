#![no_std]
#![no_main]
extern crate alloc;
extern crate core;
extern crate serde;

use alloc::format;


// // Create a custom prelude for no_std environment  
// mod prelude {
//     // Only include what we actually use
//     pub use core::result::Result;
//     pub use core::option::Option;
// }

// // Import the prelude
// use prelude::*;

mod riscv32im;
mod runner;

/// This is required to resolve `undefined reference to `_critical_section_1_0_acquire'`
#[cfg(target_arch = "riscv32")]
#[allow(unused_imports)]
use riscv as _;

use runner::execute_test_suite;
use revm_statetest_types::TestSuite;

// Embedded JSON test fixture
const TEST_JSON: &str = include_str!(
    "../../../execution-spec-tests/tests/state_tests/stShift/sar_2^255_255Filler.json"
);

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Call the statetest function which will be measured by spike
    let result = statetest();
    if result { 0 } else { 1 }
}

// Helper function to print error messages via HTIF
fn print_error_message(error: &serde_json::Error) {
    // Print prefix
    let prefix = "JSON Parse Error: ";
    for byte in prefix.bytes() {
        htif::htif_console_putchar(byte);
    }
    
    // Print error details
    let error_string = format!("{}", error); // This might need alloc
    for byte in error_string.bytes() {
        htif::htif_console_putchar(byte);
    }
    
    // Print newline
    htif::htif_console_putchar(b'\n');
}

#[no_mangle]
pub extern "C" fn statetest() -> bool {
    // Parse the embedded JSON test fixture
    let test_suite: TestSuite = match serde_json::from_str(TEST_JSON) {
        Ok(suite) => suite,
        Err(e) => {
            print_error_message(&e);
            return false; // Return false if parsing fails
        }
    };


    true
    // match execute_test_suite(&test_suite, false, false, "embedded_test") {
    //     Ok(_) => true,
    //     Err(_) => false,
    // }
}
