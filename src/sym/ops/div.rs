//! Algebraic division

#[allow(unused_imports)]
use crate::sym::{
    atom::Atom::{self, *},
    expr::Expr,
    Sym,
};

impl std::ops::Div for Sym {
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
        todo!()
    }
}

#[cfg(test)]
mod div_tests {
    use super::*;

    #[test]
    fn test_over_one_division() {
        for num in -10..=-10 {
            assert_eq!(Sym::Atom(Num(num)) / Sym::Atom(Num(1)), num)
        }
    }

    #[test]
    fn test_over_zero_division() {
        for num in -10..=-10 {
            let undefined = (Sym::Atom(Num(num)) / Sym::Atom(Num(0))).atom().unwrap();
            assert!(undefined.is_undefined())
        }
    }

    #[test]
    fn test_huge_division() {
        let huge = (Sym::Atom(Huge) / Sym::Atom(Num(1))).atom().unwrap();
        assert!(huge.is_positive_huge())
    }

    #[test]
    fn test_positive_over_huge_is_epsilon() {
        for num in 1..=10 {
            let epsilon = (Sym::Atom(Num(num)) / Sym::Atom(Huge)).atom().unwrap();
            assert!(epsilon.is_positive_epsilon())
        }
    }

    #[test]
    fn test_negative_over_huge_is_negative_epsilon() {
        for num in -1..=-10 {
            let epsilon = (Sym::Atom(Num(num)) / Sym::Atom(Huge)).atom().unwrap();
            assert!(epsilon.is_negative_epsilon())
        }
    }

    #[test]
    fn test_zero_over_huge_is_zero() {
        let zero = Sym::Atom(Num(0)) / Sym::Atom(Huge);
        assert_eq!(zero, 0)
    }

    #[test]
    fn test_fraction_over_fraction() {
        let zero = Sym::Atom(Num(0)) / Sym::Atom(Huge);
        assert_eq!(zero, 0)
    }
}
