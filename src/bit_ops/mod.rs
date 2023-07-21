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

impl BitwiseOperative for usize {
    fn set_zero_on(&mut self, positions: &[usize]) {
        for position in positions {
            *self &= !(1 << position)
        }
    }
    fn set_one_on(&mut self, positions: &[usize]) {
        for position in positions {
            *self |= 1 << position
        }
    }
    fn count_one(&self) -> usize {
        let how_many_bits = std::mem::size_of::<usize>() * 8;
        let mut count = 0;
        for i in 0..how_many_bits {
            if self & (1 << i) != 0 {
                count += 1;
            }
        }
        count
    }
    fn count_zero(&self) -> usize {
        let how_many_bits = std::mem::size_of::<usize>() * 8;
        let mut count = 0;
        for i in 0..how_many_bits {
            if self & (1 << i) == 0 {
                count += 1;
            }
        }
        count
    }
}
