// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ops::{Rem, Add};

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    match dividend > divisor {
        true => (dividend/divisor, dividend % divisor),
        false => (dividend/divisor, dividend % divisor),
        _ => panic!("error")
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
    .filter(|(x, _)| x % 2 == 0)
    .map(|(_, y)| y)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        ((self.0).abs_diff(0) + (self.1).abs_diff(0)).try_into().unwrap()
    }
}