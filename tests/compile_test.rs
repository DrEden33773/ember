//! # Compile Test
//!
//! ## Introduction
//!
//! A test to ensure that the library compiles without any error
//!
//! ## Expect
//!
//! 1. Successfully compiled
//! 2. `Hello, `Ember`!` will be printed to the `standard error output(console)`

#![allow(unused_imports)]
#[test]
fn compile_test() {
    use ember::*;
    eprintln!("Hello, `Ember`!");
}
