//! Definitions of algebraic types.

pub mod atom;
pub mod expr;

use atom::{number::Number, AlgAtom};
use expr::{fraction::Fraction, radical::Radical, AlgExpr};

/// Algebraic Notation.
///
/// Notation representing an algebraic element.
#[derive(Debug)]
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

impl std::cmp::PartialEq<i32> for AlgNotation {
    fn eq(&self, other: &i32) -> bool {
        match self {
            AlgNotation::Atom(atom) => atom == other,
            _ => false, // Probably. Might wanna test the simplified form.
        }
    }
}

impl std::cmp::PartialEq<AlgNotation> for AlgNotation {
    /// <div class="warning">
    ///
    /// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.
    /// **Does not test literal equality either.** [`Undefined`][AlgAtom::Undefined] is not equal to [`Undefined`][AlgAtom::Undefined].
    /// This operation is intended only to be used on notation that has already been simplified.
    ///
    /// </div>
    fn eq(&self, other: &Self) -> bool {
        use AlgNotation::*;
        match (&self, &other) {
            (&Atom(atom_a), &Atom(atom_b)) => atom_a == atom_b,
            (&Expr(expr_a), &Expr(expr_b)) => expr_a == expr_b,
            (&Atom(_), &Expr(_)) | (&Expr(_), &Atom(_)) => false,
        }
    }
}
