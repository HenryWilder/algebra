//! Algebraic addition and subtraction

use crate::sym::{
    atom::Atom::{self, *},
    Sym,
};

/// If the result overflows, returns [`Huge`].\
/// If the result underflows, returns [`NegativeHuge`].\
/// Otherwise returns a [`Number`] with the value of the result.
fn algebraic_add(lhs: i32, rhs: i32) -> Sym {
    match lhs.checked_add(rhs) {
        // All is well
        Some(sum) => Sym::Atom(Num(sum)),

        // Over or under flow (need to figure out which)
        None => match lhs.saturating_add(rhs) {
            i32::MAX => Sym::Atom(Huge),
            i32::MIN => Sym::Atom(NegHuge),
            _ => unreachable!("Saturated over/underflow should be equal to max/min respectively."),
        },
    }
}

#[cfg(test)]
mod algebraic_add_tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        for a in -10..=10 {
            for b in -10..=10 {
                assert_eq!(algebraic_add(a, b), a + b);
            }
        }
    }

    #[test]
    fn test_positive_huge_zero_addition() {
        let sum = algebraic_add(0, i32::MAX).atom().unwrap();
        assert_eq!(sum, i32::MAX);
    }

    #[test]
    fn test_positive_huge_positive_addition() {
        for i in 1..=10 {
            let sum = algebraic_add(i, i32::MAX).atom().unwrap();
            assert!(sum.is_positive_huge());
        }
    }

    #[test]
    fn test_positive_huge_negative_addition() {
        for i in -1..=-10 {
            let sum = algebraic_add(i, i32::MAX).atom().unwrap();
            assert_eq!(sum, i32::MAX + i);
        }
    }

    #[test]
    fn test_positive_addition_becomes_huge() {
        const HUGE_PART: i32 = i32::MAX / 2 + 1;
        let sum = algebraic_add(HUGE_PART, HUGE_PART).atom().unwrap();
        assert!(sum.is_positive_huge());
    }

    #[test]
    fn test_negative_huge_addition() {
        let sum = algebraic_add(0, i32::MIN).atom().unwrap();
        assert_eq!(sum, i32::MIN);
    }

    #[test]
    fn test_negative_huge_positive_addition() {
        for i in 1..=10 {
            let sum = algebraic_add(i, i32::MIN).atom().unwrap();
            assert_eq!(sum, i32::MIN + i);
        }
    }

    #[test]
    fn test_negative_huge_negative_addition() {
        for i in -1..=-10 {
            let sum = algebraic_add(i, i32::MIN).atom().unwrap();
            assert!(sum.is_negative_huge());
        }
    }

    #[test]
    fn test_negative_addition_becomes_negative_huge() {
        const NHUGE_PART: i32 = i32::MIN / 2 - 1;
        let sum = algebraic_add(NHUGE_PART, NHUGE_PART).atom().unwrap();
        assert!(sum.is_negative_huge());
    }

    #[test]
    fn test_positive_and_negative_huge_addition_not_huge() {
        let sum_huges = algebraic_add(i32::MIN, i32::MAX).atom().unwrap();
        assert_eq!(sum_huges, -1);
    }
}

impl std::ops::Add for Sym {
    type Output = Self;

    /// Add two values.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sym::Atom(atom_a), Sym::Atom(atom_b)) => match (atom_a, atom_b) {
                (Num(num_a), Atom::Num(num_b)) => algebraic_add(num_a, num_b),

                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl std::ops::Sub for Sym {
    type Output = Self;

    /// Subtract two values.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sym::Atom(atom_a), Sym::Atom(atom_b)) => match (atom_a, atom_b) {
                (Atom::Num(num_a), Atom::Num(num_b)) => {
                    match num_b.checked_neg() {
                        Some(sub_b) => algebraic_add(num_a, sub_b),
                        // The edge cases where we can salvage lost information are too rare to worry about at the moment.
                        // The fact this case is reached already implies the user is working with numbers dangerously close to Huge anyway.
                        None => Sym::Atom(NegHuge),
                    }
                }

                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        for a in -10..=10 {
            for b in -10..=10 {
                assert_eq!(Sym::Atom(Num(a)) + Sym::Atom(Num(b)), a + b);
            }
        }
    }
}
