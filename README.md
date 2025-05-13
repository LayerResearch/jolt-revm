
## Introduction
This is a demonstration project showcasing the integration of [revm](https://github.com/bluealloy/revm) (Rust Ethereum Virtual Machine) with [jolt](https://github.com/joltxyz/jolt), a zero-knowledge proof framework.

The project consists of two parts:
- A guest program that initializes revm with minimal features enabled (secp256k1, serde, and portable)
- A host program that compiles the guest code

## Build Host

```bash
cargo build --release
```

## Build Guest
```bash
./target/release/revm-host 
```
or
```bash
CARGO_ENCODED_RUSTFLAGS=$'-Clink-arg=-T/tmp/jolt-guest-linkers/revm-guest.ld\x1f-Cpasses=lower-atomic\x1f-Cpanic=abort\x1f-Cstrip=symbols\x1f-Copt-level=z' \
cargo build --release --features guest -p revm-guest --target-dir /tmp/jolt-guest-targets/revm-guest/ --target riscv32im-unknown-none-elf
```

The build fails with the following errors:
```
   Compiling getrandom v0.2.16
   Compiling radium v0.7.0
   Compiling serde v1.0.219
   Compiling secp256k1-sys v0.10.1
   Compiling spin v0.9.8
   Compiling once_cell v1.21.3
   Compiling jolt-sdk-macros v0.1.0 (https://github.com/a16z/jolt#22306a11)
error[E0463]: can't find crate for `std`
 --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.16/src/error_impls.rs:1:1
  |
1 | extern crate std;
  | ^^^^^^^^^^^^^^^^^ can't find crate
  |
  = note: the `riscv32im-unknown-none-elf` target may not support the standard library

error: target is not supported, for more information see: https://docs.rs/getrandom/#unsupported-targets
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.16/src/lib.rs:351:9
    |
351 | /         compile_error!("target is not supported, for more information see: \
352 | |                         https://docs.rs/getrandom/#unsupported-targets");
    | |________________________________________________________________________^

error[E0432]: unresolved imports `core::sync::atomic::AtomicI64`, `core::sync::atomic::AtomicU64`
  --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/radium-0.7.0/src/lib.rs:53:34
   |
53 |         use core::sync::atomic::{AtomicI64, AtomicU64};
   |                                  ^^^^^^^^^  ^^^^^^^^^ no `AtomicU64` in `sync::atomic`
   |                                  |
   |                                  no `AtomicI64` in `sync::atomic`
   |
help: a similar name exists in the module
   |
53 -         use core::sync::atomic::{AtomicI64, AtomicU64};
53 +         use core::sync::atomic::{AtomicI8, AtomicU64};
   |
help: a similar name exists in the module
   |
53 -         use core::sync::atomic::{AtomicI64, AtomicU64};
53 +         use core::sync::atomic::{AtomicI64, AtomicU8};
   |

error[E0412]: cannot find type `AtomicI64` in module `core::sync::atomic`
    --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/radium-0.7.0/src/types.rs:52:41
     |
52   |       if atomic(64) { core::sync::atomic::AtomicI64 }
     |                                           ^^^^^^^^^ help: a struct with a similar name exists: `AtomicI16`
     |
    ::: /usr/local/rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3475:1
     |
3475 | / atomic_int! {
3476 | |     cfg(target_has_atomic = "16"),
3477 | |     cfg(target_has_atomic_equal_alignment = "16"),
3478 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
3491 | |     i16 AtomicI16
3492 | | }
     | |_- similarly named struct `AtomicI16` defined here

error[E0412]: cannot find type `AtomicU64` in module `core::sync::atomic`
    --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/radium-0.7.0/src/types.rs:58:41
     |
58   |       if atomic(64) { core::sync::atomic::AtomicU64 }
     |                                           ^^^^^^^^^ help: a struct with a similar name exists: `AtomicU16`
     |
    ::: /usr/local/rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:3494:1
     |
3494 | / atomic_int! {
3495 | |     cfg(target_has_atomic = "16"),
3496 | |     cfg(target_has_atomic_equal_alignment = "16"),
3497 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
3510 | |     u16 AtomicU16
3511 | | }
     | |_- similarly named struct `AtomicU16` defined here

error[E0433]: failed to resolve: use of unresolved module or unlinked crate `imp`
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.16/src/lib.rs:402:9
    |
402 |         imp::getrandom_inner(dest)?;
    |         ^^^ use of unresolved module or unlinked crate `imp`
    |
    = help: if you wanted to use a crate named `imp`, use `cargo add imp` to add it to your `Cargo.toml`

Some errors have detailed explanations: E0433, E0463.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `getrandom` (lib) due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0599]: no method named `compare_exchange` found for struct `core::sync::atomic::AtomicU8` in the current scope
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/spin-0.9.8/src/once.rs:114:18
    |
112 |               match self
    |  ___________________-
113 | |                 .0
114 | |                 .compare_exchange(old as u8, new as u8, success, failure)
    | |                 -^^^^^^^^^^^^^^^^ method not found in `AtomicU8`
    | |_________________|
    |

error[E0599]: no method named `compare_exchange` found for struct `AtomicUsize` in the current scope
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/race.rs:161:20
    |
161 |         self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
    |                    ^^^^^^^^^^^^^^^^ method not found in `AtomicUsize`

For more information about this error, try `rustc --explain E0599`.
error[E0599]: no method named `compare_exchange` found for struct `AtomicPtr` in the current scope
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/race.rs:322:14
    |
321 | /         self.inner
322 | |             .compare_exchange(
    | |             -^^^^^^^^^^^^^^^^ method not found in `AtomicPtr<T>`
    | |_____________|
    |

error[E0599]: no method named `compare_exchange` found for struct `AtomicPtr` in the current scope
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/race.rs:413:39
    |
413 |             let exchange = self.inner.compare_exchange(
    |                            -----------^^^^^^^^^^^^^^^^ method not found in `AtomicPtr<T>`

error: could not compile `spin` (lib) due to 1 previous error
error[E0599]: no method named `compare_exchange` found for struct `AtomicPtr` in the current scope
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/race.rs:465:39
    |
465 |             let exchange = self.inner.compare_exchange(
    |                            -----------^^^^^^^^^^^^^^^^ method not found in `AtomicPtr<T>`

Some errors have detailed explanations: E0412, E0432.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `radium` (lib) due to 3 previous errors
error: could not compile `once_cell` (lib) due to 4 previous errors
error[E0432]: unresolved import `alloc::sync`
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.219/src/lib.rs:224:20
    |
224 |     pub use alloc::sync::{Arc, Weak as ArcWeak};
    |                    ^^^^ could not find `sync` in `alloc`
    |
note: found an item that was configured out
   --> /usr/local/rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/lib.rs:245:9
    |
245 | pub mod sync;
    |         ^^^^
note: the item is gated here
   --> /usr/local/rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/lib.rs:244:1
    |
244 | #[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0432`.
error: could not compile `serde` (lib) due to 1 previous error
```