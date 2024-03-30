#![warn(missing_docs)]

//! A library for handling algebra.

pub mod factor;
pub mod notation;

use std::cmp::Ordering;

use factor::Factoring;
use notation::AlgNotation;

/// Provides additional true/false information about numbers
pub trait NumericFlags {
    /// Returns true for odd numbers, false for even numbers.
    fn is_odd(&self) -> bool;

    /// Returns true for even numbers, false for odd numbers.
    fn is_even(&self) -> bool;

    /// Returns true for prime numbers, false for composites.
    fn is_prime(&self) -> bool;

    /// Returns true for composite numbers, false for primes.
    fn is_composite(&self) -> bool;
}

impl NumericFlags for i32 {
    fn is_odd(&self) -> bool {
        (self & 1) != 0
    }

    fn is_even(&self) -> bool {
        (self & 1) == 0
    }

    fn is_prime(&self) -> bool {
        *self != 0 && !self.has_multiple_factors()
    }

    fn is_composite(&self) -> bool {
        self.has_multiple_factors()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd() {
        for odd in [1, 3, 5, 7, 9, 11, 13, 15, 17, 19] {
            assert!(odd.is_odd());
            assert!((-odd).is_odd());
        }
        for even in [2, 4, 6, 8, 10, 12, 14, 16, 18, 20] {
            assert!(!even.is_odd());
            assert!(!(-even).is_odd());
        }
        assert!(!0.is_odd());
    }

    #[test]
    fn test_is_even() {
        for odd in [1, 3, 5, 7, 9, 11, 13, 15, 17, 19] {
            assert!(!odd.is_even());
            assert!(!(-odd).is_even());
        }
        for even in [2, 4, 6, 8, 10, 12, 14, 16, 18, 20] {
            assert!(even.is_even());
            assert!((-even).is_even());
        }
        assert!(0.is_even());
    }

    #[test]
    fn test_is_prime() {
        for prime in [1, 2, 3, 5, 7, 11, 13, 17, 19] {
            assert!(prime.is_prime());
            assert!((-prime).is_prime());
        }
        for composite in [4, 6, 8, 9, 10, 12, 14, 15, 16, 18] {
            assert!(!composite.is_prime());
            assert!(!(-composite).is_prime());
        }
        assert!(!0.is_prime());
    }

    #[test]
    fn test_is_composite() {
        for prime in [1, 2, 3, 5, 7, 11, 13, 17, 19] {
            assert!(!prime.is_composite());
            assert!(!(-prime).is_composite());
        }
        for composite in [4, 6, 8, 9, 10, 12, 14, 15, 16, 18] {
            assert!(composite.is_composite());
            assert!((-composite).is_composite());
        }
        assert!(!0.is_composite());
    }
}
