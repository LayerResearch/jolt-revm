#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

pub mod compat;
pub mod prelude;

mod merkle_trie;
mod runner;

pub use runner::execute_test_suite;
