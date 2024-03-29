#![warn(missing_docs)]

//! A library for handling algebra.

pub mod factor;
pub mod notation;

use std::cmp::Ordering;

use notation::AlgNotation;

/// Provides additional true/false information about numbers
pub trait NumericFlags {
    /// Returns true for odd numbers, false for even numbers.
    fn is_odd(&self) -> bool;

    /// Returns true for even numbers, false for odd numbers.
    fn is_even(&self) -> bool;

    /// Returns true for prime numbers, false for composites.
    fn is_prime(&self) -> bool;
}

impl NumericFlags for i32 {
    fn is_odd(&self) -> bool {
        (self & 1) != 0
    }

    fn is_even(&self) -> bool {
        (self & 1) == 0
    }

    fn is_prime(&self) -> bool {
        // First two checks are cheaper than calculating every factor for the number.
        (self.is_odd()) && (self % 5 != 0) && (factor::factors(*self).len() == 1)
    }
}

/// Returns the Least Common Multiple of the provided numbers.
pub fn lcm(ns: Vec<i32>) -> AlgNotation {
    todo!()
}

/// If the square root of n can be expressed as an integer, returns that integer. Otherwise returns [`None`].
pub fn sqrt_i(n: i32) -> Option<i32> {
    match n {
        ..=-1 => None,
        0..=1 => Some(n),
        2.. => {
            let mut root = 2;
            loop {
                match (root * root).cmp(&n) {
                    Ordering::Less => root += 1,
                    Ordering::Equal => break Some(root),
                    Ordering::Greater => break None,
                }
            }
        }
    }
}
