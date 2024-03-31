//! Algebraic division

#[allow(unused_imports)]
use crate::notation::{
    atom::{
        number::Number,
        Atom::{self, *},
    },
    expr::{fraction::Fraction, simplify::Simplify, Expr},
    Notation,
};

impl std::ops::Div for Notation {
    type Output = Self;

    /// Divide two values.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// If the result underflows, returns [`NegativeHuge`].\
    /// If the result has a [`Huge`] or [`NegativeHuge`] denominator, returns [`Epsilon`] if positive overall and [`NegativeEpsilon`] if overall negative.\
    /// If the result has a denominator of 0, or contains [`Undefined`], returns [`Undefined`].\
    /// If the result an integer, returns a [`Number`] with the value of the result.\
    /// Otherwise returns a [`Fraction`].
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Notation::Atom(num), Notation::Atom(den)) => Fraction { num, den }.simplify(),

            (Notation::Expr(Expr::Fraction(frac_num)), Notation::Atom(den)) => {
                if let Notation::Atom(num) = frac_num.simplify() {
                    Fraction { num, den }.simplify()
                } else {
                    todo!()
                }
            }
            (Notation::Atom(num), Notation::Expr(Expr::Fraction(frac_den))) => {
                if let Notation::Atom(den) = frac_den.simplify() {
                    Fraction { num, den }.simplify()
                } else {
                    todo!()
                }
            }

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod div_tests {
    use super::*;

    #[test]
    fn test_over_one_division() {
        for num in -10..=-10 {
            assert_eq!(Notation::from(num) / Notation::from(1), num)
        }
    }

    #[test]
    fn test_over_zero_division() {
        for num in -10..=-10 {
            let undefined = (Notation::from(num) / Notation::from(0)).atom().unwrap();
            assert!(undefined.is_undefined())
        }
    }

    #[test]
    fn test_huge_division() {
        let huge = (Notation::from(Huge) / Notation::from(1)).atom().unwrap();
        assert!(huge.is_positive_huge())
    }

    #[test]
    fn test_positive_over_huge_is_epsilon() {
        for num in 1..=10 {
            let epsilon = (Notation::from(num) / Notation::from(Huge)).atom().unwrap();
            assert!(epsilon.is_positive_epsilon())
        }
    }

    #[test]
    fn test_negative_over_huge_is_negative_epsilon() {
        for num in -1..=-10 {
            let epsilon = (Notation::from(num) / Notation::from(Huge)).atom().unwrap();
            assert!(epsilon.is_negative_epsilon())
        }
    }

    #[test]
    fn test_zero_over_huge_is_zero() {
        let zero = Notation::from(0) / Notation::from(Huge);
        assert_eq!(zero, 0)
    }
}
