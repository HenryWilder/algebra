//! A fraction made from a combination of algebraic atomics.

use crate::notation::{atom::AlgAtom, expr::Simplify, AlgNotation};

/// A fraction made from a combination of algebraic atomics.
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

impl Fraction {
    /// Constructs a fraction with the whole number value of 1.
    pub fn new() -> Self {
        Self {
            num: AlgAtom::from(1),
            den: AlgAtom::from(1),
        }
    }

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
        todo!()
    }
}
