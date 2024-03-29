//! Definitions of algebraic types.

pub mod atom;
pub mod expr;

use atom::{number::Number, AlgAtom};
use expr::{fraction::Fraction, radical::Radical, AlgExpr};

/// Algebraic Notation.
///
/// Notation representing an algebraic element.
pub enum AlgNotation {
    /// The smallest unit, a single value.
    ///
    /// See [`AlgAtom`].
    Atom(AlgAtom),

    /// A combination of atomics, able to be simplified.
    ///
    /// See [`AlgExpr`]
    Expr(AlgExpr),
}

impl ToString for AlgNotation {
    fn to_string(&self) -> String {
        use AlgNotation::*;
        match self {
            Atom(atom) => atom.to_string(),
            Expr(expr) => expr.to_string(),
        }
    }
}

impl From<AlgAtom> for AlgNotation {
    fn from(value: AlgAtom) -> Self {
        Self::Atom(value)
    }
}

impl From<Number> for AlgNotation {
    fn from(value: Number) -> Self {
        Self::Atom(AlgAtom::Number(value))
    }
}

impl From<i32> for AlgNotation {
    fn from(value: i32) -> Self {
        Self::from(AlgAtom::from(value))
    }
}

impl From<AlgExpr> for AlgNotation {
    fn from(value: AlgExpr) -> Self {
        Self::Expr(value)
    }
}

impl From<Fraction> for AlgNotation {
    fn from(value: Fraction) -> Self {
        Self::from(AlgExpr::Fraction(value))
    }
}

impl From<Radical> for AlgNotation {
    fn from(value: Radical) -> Self {
        Self::from(AlgExpr::Radical(value))
    }
}
