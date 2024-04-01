//! Functions related to factoring numbers.

use crate::sym::atom::Atom::{self, *};

/// Trait for types which can be factored.
pub trait Factoring: Sized {
    /// Test if `self` is a multiple of `other`.
    fn is_multiple_of(&self, other: Self) -> bool;

    /// Test if `other` is a multiple of `self`,
    /// making `self` a factor of `other`.
    fn is_factor_of(&self, other: Self) -> bool;

    /// Test if all others are evenly divisible by this number,
    /// making it a common factor among all of them.
    fn is_common_factor_of<const COUNT: usize>(&self, others: &[Self; COUNT]) -> bool;

    /// Test this number is evenly divisible by all others,
    /// making it a common multiple among all of them.
    fn is_common_multiple_of<const COUNT: usize>(&self, others: &[Self; COUNT]) -> bool;

    /// Returns all factors for the given number.
    fn factors(&self) -> Vec<(i32, i32)>;

    /// Returns the number of factors the given number has.
    ///
    /// **Note:** This is cheaper than constructing a list of all the factors, but **not free**.\
    /// If you need to use the factors anyway, find the [`len`][Vec::len] of [`factors`][Factoring::factors] instead.
    fn count_factors(&self) -> usize;

    /// Returns true if the number is composite, false if it is prime.
    ///
    /// Employs logical short-circuiting, stopping on the first factor that isn't 1.
    ///
    /// Used in [`is_prime`][crate::NumericFlags::is_prime()].
    fn has_multiple_factors(&self) -> bool;
}

impl Factoring for i32 {
    fn is_multiple_of(&self, other: Self) -> bool {
        self % other == 0
    }

    fn is_factor_of(&self, other: Self) -> bool {
        other.is_multiple_of(*self)
    }

    fn is_common_factor_of<const COUNT: usize>(&self, others: &[Self; COUNT]) -> bool {
        others.iter().all(|other| self.is_factor_of(*other))
    }

    fn is_common_multiple_of<const COUNT: usize>(&self, others: &[Self; COUNT]) -> bool {
        others.iter().all(|other| self.is_multiple_of(*other))
    }

    fn factors(&self) -> Vec<(i32, i32)> {
        let mut factors = Vec::from([(1, *self)]);

        let abs_n = self.abs();

        // Potential factor
        for pot_fac in 2..abs_n {
            if pot_fac.is_factor_of(abs_n) {
                let fac = pot_fac; // Confirmed
                factors.push((fac, self / fac));
            }
        }

        factors
    }

    fn count_factors(&self) -> usize {
        let abs_n = self.abs();

        let mut count = 1; // 1 is always a factor.

        // Potential factor
        for pot_fac in 2..abs_n {
            if pot_fac.is_factor_of(abs_n) {
                count += 1;
            }
        }

        count
    }

    fn has_multiple_factors(&self) -> bool {
        let abs_n = self.abs();

        for fac in 2..abs_n {
            if fac.is_factor_of(abs_n) {
                return true;
            }
        }

        false
    }
}

/// Given a set of numbers, returns the factors shared between them.
pub fn common_factors<const COUNT: usize>(ns: [i32; COUNT]) -> Vec<(i32, [i32; COUNT])> {
    assert!(COUNT > 0, "Empty set has no factors.");

    let mut factors = Vec::from([(1, ns)]);

    let abs_ns = ns.map(|n| n.abs());
    let abs_min = *abs_ns.iter().min().unwrap();

    for fac in 2..=abs_min {
        if fac.is_common_factor_of(&abs_ns) {
            factors.push((fac, ns.map(|x| x / fac)));
        }
    }

    factors
}

/// Returns the Greatest Common Factor of the provided numbers.
pub fn gcf<const COUNT: usize>(ns: [i32; COUNT]) -> i32 {
    assert!(COUNT > 0, "Empty set has no factors.");

    let abs_ns = ns.map(|x| x.abs());
    let n_min = *abs_ns.iter().min().unwrap();

    for gcf in (2..=n_min).rev() {
        if gcf.is_common_factor_of(&abs_ns) {
            return gcf;
        }
    }

    1 // 1 is a factor of every number, so we don't need to bother testing `is_factor_of` on it.
}

/// Returns the Least Common Multiple of the provided numbers.
///
/// ```
/// # use algebra::factor::lcm;
/// assert_eq!(lcm([ 4,  5]),  20);
/// assert_eq!(lcm([ 2, 12]),  12);
/// assert_eq!(lcm([ 8,  8]),   8);
/// assert_eq!(lcm([ 5, 21]), 105);
/// assert_eq!(lcm([16, 20]),  80);
/// ```
///
/// <div class="warning">
///
/// As it is currently implemented, this might mark some LCMs as huge when they aren't.
///
/// ### Consider the case of lcm(2^17, 2^17)
/// The LCM is 2^17, because they are the same, but the product is Huge.\
/// This function will return Huge for this pair; even though the LCM (2^17) isn't Huge.
///
/// </div>
///
/// ```should_panic
/// # use algebra::factor::lcm;
/// let not_huge = 2 << 17; // A big number; but its LCM isn't Huge.
/// assert_eq!(lcm([not_huge, not_huge]), not_huge);
/// ```
pub fn lcm<const COUNT: usize>(ns: [i32; COUNT]) -> Atom {
    assert!(COUNT > 0, "Empty set has no multiples.");

    let mut prod: i32 = 1;
    for n in &ns {
        match prod.checked_mul(*n) {
            Some(p) => prod = p,
            None => return Huge,
        }
    }
    let prod = prod;

    let abs_ns = ns.map(|x| x.abs());
    let abs_max = *abs_ns.iter().max().unwrap();

    for lcm in abs_max..prod {
        if lcm.is_common_multiple_of(&abs_ns) {
            return Num(lcm);
        }
    }

    Num(prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_factor_of() {
        for fac in 1..5 {
            for i in -20..20 {
                assert_eq!(fac.is_factor_of(i), i % fac == 0);
            }
        }
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm([2, 12]), 12);
    }
}
