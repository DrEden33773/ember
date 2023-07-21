//! # bit_ops
//!
//! ## Introduction
//!
//! A `bitwise operation` library

#![allow(dead_code)]

pub trait BitwiseOperative: Sized {
    fn set_zero_on(&mut self, positions: &[usize]);
    fn set_one_on(&mut self, positions: &[usize]);
    fn count_one(&self) -> usize;
    fn count_zero(&self) -> usize;
}
