use std::time::Instant;
use revm_guest as guest;
pub fn main() {
    let target_dir = "/tmp/jolt-guest-targets";
    let program = guest::compile_exec(target_dir);

    println!("build done");
}
