//! A fraction made from a combination of algebraic atomics.

use crate::{
    factor::{gcf, Factoring},
    notation::{
        atom::{number::Number, Atom},
        expr::Simplify,
        Notation,
    },
};

/// A fraction made from a combination of algebraic atomics.
#[derive(Clone, Debug, PartialEq)]
pub struct Fraction {
    /// The numerator.
    ///
    /// Upper side of the fraction; the part being divided.
    pub num: Atom,

    /// The denominator.
    ///
    /// Lower side of the fraction; the part dividing the numerator.
    pub den: Atom,
}

impl From<i32> for Fraction {
    /// Convert an integer value into a fraction with denominator of 1.
    ///
    /// Use [`new`][Fraction::new()] to create a fraction with both numerator and denominator.\
    /// Use `Fraction { num, den }` if your fraction needs to be created from [`Atom`]s.
    fn from(num: i32) -> Self {
        Self {
            num: num.into(),
            den: 1.into(),
        }
    }
}

impl Fraction {
    /// Constructs a fraction from integer numerator and denominator.
    ///
    /// Use [`from`][Fraction::from()] to create a fraction equivalent to a whole integer.\
    /// Use `Fraction { num, den }` if your fraction needs to be created from [`Atom`]s.
    pub fn new(num: i32, den: i32) -> Self {
        Self {
            num: num.into(),
            den: den.into(),
        }
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { num, den } = self;
        format!("{num}/{den}").fmt(f)
    }
}

impl Simplify for Fraction {
    fn simplify(self) -> Notation {
        match self {
            Fraction {
                num: Atom::Number(Number { value: num }),
                den: Atom::Number(Number { value: den }),
            } => {
                if den == 0 {
                    // Division by zero
                    Notation::from(Atom::Undefined)
                } else if den.is_factor_of(num) {
                    // Division leaves no remainder
                    Notation::from(num / den)
                } else {
                    // Transfer sign to the top
                    let sign = if (num < 0) != (den < 0) { -1 } else { 1 };
                    let (num_abs, den_abs) = (num.abs(), den.abs());
                    let gcf = gcf([num_abs, den_abs]);
                    Notation::from(Fraction::new(sign * num_abs / gcf, den_abs / gcf))
                }
            }

            Fraction {
                num: Atom::Undefined,
                den: _,
            }
            | Fraction {
                num: _,
                den: Atom::Undefined,
            } => {
                // Propigate undefined
                Notation::from(Atom::Undefined)
            }

            // All other fractions cannot be simplified but do not propigate
            _ => Notation::from(self),
        }
    }
}

#[cfg(test)]
mod simplify_fraction_tests {
    use super::*;

    #[test]
    fn test_denominator_of_1() {
        for num in 0..10 {
            let frac = Fraction::from(num);
            assert_eq!(frac.simplified(), num);
        }
    }

    #[test]
    fn test_simplifies_to_integer() {
        for den in 1..10 {
            for n in 0..10 {
                let frac = Fraction::new(den * n, den);
                assert_eq!(frac.simplified(), n);
            }
        }
    }

    #[test]
    fn test_already_simplest() {
        for den in 2..10 {
            let frac = Fraction::new(1, den);
            assert_eq!(frac.simplified(), frac);
        }
    }

    #[test]
    fn test_simplifies_to_half() {
        for num in 1..10 {
            let frac = Fraction::new(num, num * 2);
            let simplest = Fraction::new(1, 2);
            assert_eq!(frac.simplified(), simplest);
        }
    }

    #[test]
    fn test_division_by_zero() {
        for num in 1..10 {
            let frac = Fraction::new(num, 0);
            assert!(frac.simplified().atom().is_some_and(|x| x.is_undefined()));
        }
    }
}
