# `compiler-builtins-tester`

> Testing the `compiler-builtins` crate on `std` and `no_std` targets.

## Usage

- Define a test case template for the intrinsic.

In `src/main.rs`, create a new struct representing the intrinsic to test and
implement the `TestCase` trait for it.

- Generate a test file for the intrinsic.

In `src/main.rs`, add `file("new-intrinsics.rs", mk_tests::<NewIntrinsic,
_>(number_of_tests, rng))` to the `main` function. Then invoke `cargo run`.

- Run the test suite

Go into the `tester` directory and invoke

`cargo test` to run the test suite on the host,

`cross test --target arm-unknown-linux-gnueabi` to run the test suite on an
emulated `std` target, or

`xargo test --target thumbv7m-linux-eabi; qemu-arm
target/thumbv7m-none-eabi/debug/muldi3-deadbeef`, to run the test suite on an
emulated `no_std` target.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
