//! A fraction made from a combination of algebraic atomics.

use crate::{
    factor::{gcf, Factoring},
    notation::{
        atom::{number::Number, AlgAtom},
        expr::Simplify,
        AlgNotation,
    },
};

/// A fraction made from a combination of algebraic atomics.
#[derive(Clone, Debug, PartialEq)]
pub struct Fraction {
    /// The numerator.
    ///
    /// Upper side of the fraction; the part being divided.
    pub num: AlgAtom,

    /// The denominator.
    ///
    /// Lower side of the fraction; the part dividing the numerator.
    pub den: AlgAtom,
}

impl Default for Fraction {
    /// Constructs a fraction with the whole number value of 1.
    fn default() -> Self {
        Self {
            num: AlgAtom::from(1),
            den: AlgAtom::from(1),
        }
    }
}

impl Fraction {
    /// Constructs a fraction from a pair of numbers.
    pub fn from_ints(num: i32, den: i32) -> Self {
        Self {
            num: AlgAtom::from(num),
            den: AlgAtom::from(den),
        }
    }

    /// Constructs a fraction from a pair of atoms.
    pub fn from_atoms(num: AlgAtom, den: AlgAtom) -> Self {
        Self { num, den }
    }
}

impl ToString for Fraction {
    fn to_string(&self) -> String {
        format!("{}/{}", self.num.to_string(), self.den.to_string())
    }
}

impl Simplify for Fraction {
    fn simplified(&self) -> AlgNotation {
        match (&self.num, &self.den) {
            (&AlgAtom::Number(Number { value: num }), &AlgAtom::Number(Number { value: den })) => {
                if den == 0 {
                    // Division by zero
                    AlgNotation::from(AlgAtom::Undefined)
                } else if den.is_factor_of(num) {
                    // Division leaves no remainder
                    AlgNotation::from(num / den)
                } else {
                    // Transfer sign to the top
                    let sign = if (num < 0) != (den < 0) { -1 } else { 1 };
                    let (num_abs, den_abs) = (num.abs(), den.abs());
                    let gcf = gcf([num_abs, den_abs]);
                    AlgNotation::from(Fraction::from_ints(sign * num_abs / gcf, den_abs / gcf))
                }
            }

            (&AlgAtom::Undefined, _) | (_, &AlgAtom::Undefined) => {
                // Propigate undefined
                AlgNotation::from(AlgAtom::Undefined)
            }

            // All other fractions cannot be simplified but do not propigate
            _ => AlgNotation::from(self.clone()),
        }
    }
}
