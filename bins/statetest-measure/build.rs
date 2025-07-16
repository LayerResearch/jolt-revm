fn main() {
    println!("cargo:rerun-if-changed=riscv-baremetal.ld");
    println!("cargo:rerun-if-changed=build.rs");
} 