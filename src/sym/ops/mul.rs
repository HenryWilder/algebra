//! Algebraic multiplication

use crate::sym::{
    atom::Atom::{self, *},
    Sym,
};

/// If the result overflows, returns [`Huge`].\
/// If the result underflows, returns [`NegHuge`].\
/// Otherwise returns a [`Num`] with the value of the result.
fn algebraic_mul(lhs: i32, rhs: i32) -> Sym {
    match lhs.checked_mul(rhs) {
        // All is well
        Some(prod) => Sym::Atom(Num(prod)),

        // Over or under flow (need to figure out which)
        None => match lhs.saturating_mul(rhs) {
            i32::MAX => Sym::Atom(Huge),
            i32::MIN => Sym::Atom(NegHuge),
            _ => unreachable!("Saturated over/underflow should be equal to max/min respectively."),
        },
    }
}

#[cfg(test)]
mod algebraic_mul_tests {
    use super::*;

    #[test]
    fn test_basic_multiplication() {
        for a in -10..=10 {
            for b in -10..=10 {
                assert_eq!(algebraic_mul(a, b), a * b);
                assert_eq!(algebraic_mul(b, a), b * a);
            }
        }
    }

    #[test]
    fn test_overflowing_multiplication() {
        let prod = algebraic_mul(i32::MAX, 2).atom().unwrap();
        assert!(prod.is_positive_huge());

        let prod = algebraic_mul(2, i32::MAX).atom().unwrap();
        assert!(prod.is_positive_huge());
    }

    #[test]
    fn test_underflowing_multiplication() {
        let prod = algebraic_mul(i32::MAX, -2).atom().unwrap();
        assert!(prod.is_negative_huge());

        let prod = algebraic_mul(-2, i32::MAX).atom().unwrap();
        assert!(prod.is_negative_huge());
    }
}

impl std::ops::Mul for Sym {
    type Output = Self;

    /// Multiply two values.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// Otherwise returns a [`Num`] with the value of the result.
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sym::Atom(atom_a), Sym::Atom(atom_b)) => match (atom_a, atom_b) {
                (Atom::Num(num_a), Atom::Num(num_b)) => algebraic_mul(num_a, num_b),

                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod mul_tests {
    use super::*;

    #[test]
    fn test_basic_multiplication() {
        for a in -10..=10 {
            for b in -10..=10 {
                assert_eq!(Sym::Atom(Num(a)) * Sym::Atom(Num(b)), a * b);
            }
        }
    }
}
