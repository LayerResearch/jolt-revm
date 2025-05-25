
## Introduction
This is a demonstration project showcasing the integration of [revm](https://github.com/bluealloy/revm) (Rust Ethereum Virtual Machine) with [jolt](https://github.com/joltxyz/jolt), a zero-knowledge proof framework.

The project consists of two parts:
- A guest program that initializes revm with minimal features enabled (secp256k1, serde, and portable)
- A host program that compiles the guest code

## Prerequisites

## Test provable functions
To run tests for the provable functions in the guest program:
```bash
cargo test -p revm-guest
```

## Build Host
```bash
cargo build --release
```

## Build Guest
To build the guest program and generate/verify proofs:
```bash
RUST_BACKTRACE=full ./target/release/revm-host
```
or build only the guest program without generating proofs, need to specify JOLT_FUNC_NAME if there are multiple provable in the guest program:
```bash
CARGO_ENCODED_RUSTFLAGS=$'-Clink-arg=-T/tmp/jolt-guest-linkers/revm-guest.ld\x1f-Cpasses=lower-atomic\x1f-Cpanic=abort\x1f-Cstrip=symbols\x1f-Copt-level=z' \
JOLT_FUNC_NAME=exec \
cargo build --release --features guest -p revm-guest --target-dir /tmp/jolt-guest-targets/revm-guest/ --target riscv32im-unknown-none-elf
```
For simplicity, you can combine these commands in one line:
```bash
cargo clean && clear && cargo test -p revm-guest && cargo build --release && RUST_BACKTRACE=full ./target/release/revm-host
```

## Troubleshooting
```bash
> rustc --print cfg --target=riscv32im-unknown-none-elf
debug_assertions
panic="abort"
target_abi=""
target_arch="riscv32"
target_endian="little"
target_env=""
target_feature="m"
target_os="none"
target_pointer_width="32"
target_vendor="unknown"
```

List dependency graph and features, e.g.
```bash
cargo tree --edges normal,features --target riscv32im-unknown-none-elf -f '{p} {f}' -i getrandom@0.2.16
```

Dump the disassembly of the guest program to analyze the generated RISC-V code
```
llvm-objdump-14 -d /tmp/jolt-guest-targets/revm-guest/riscv32im-unknown-none-elf/release/revm-guest > revm-guest.disasm
```

UNIX-like reverse engineering framework and command-line toolset
```
git clone https://github.com/radareorg/radare2
radare2/sys/install.sh
```

## Resolved issues
### Patches
- https://github.com/serde-rs/serde/pull/2924
- https://github.com/DaniPopes/const-hex/pull/20
- https://github.com/paradigmxyz/revmc/pull/100
- https://github.com/alloy-rs/rlp/pull/37

### error[E0432]: unresolved imports `core::sync::atomic::AtomicI64` in radium-0.70
radium 0.7.1 is at [3fdd72f3286110b1958f020b984999326190f42f](https://github.com/ferrilab/ferrilab/blob/3fdd72f3286110b1958f020b984999326190f42f/radium/Cargo.toml). 
Using the patched version can fix issues in radium-0.7.0.
```toml
[patch.crates-io]
radium = { git = "https://github.com/ferrilab/ferrilab", package = "radium", rev = "3fdd72f3286110b1958f020b984999326190f42f" }
```

### error: failed to run custom build command for `secp256k1-sys v0.10.1`
```toml
[patch.crates-io]
secp256k1 = { git = "https://github.com/sp1-patches/rust-secp256k1", tag = "patch-0.30.0-sp1-4.2.0" }
```

### build RISCV GNU toolchain (Optional)
Run the following commands to install the required packages(required to build RISCV GNU toolchain):
```bash
apt-get update && apt-get install build-essential clang libclang-dev gawk texinfo bison flex libgmp-dev libmpfr-dev libmpc-dev
```

Build RISCV GNU toolchain(required to build secp256k1):
```bash
mkdir -p /opt/riscv
export RISCV_GNU_TOOLCHAIN=/opt/riscv

cd /tmp
git clone https://github.com/riscv-collab/riscv-gnu-toolchain && cd riscv-gnu-toolchain
./configure --prefix="$RISCV_GNU_TOOLCHAIN" --with-arch=rv32im --with-abi=ilp32
make -j$(nproc)
```

### link with OpenSSL (Optional)
Install OpenSSL development package and pkg-config (required to link OpenSSL)

```
apt-get update && apt-get install libssl-dev pkg-config
```