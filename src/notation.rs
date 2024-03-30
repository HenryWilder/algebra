//! Definitions of algebraic types.

pub mod atom;
pub mod expr;

use atom::{number::Number, AlgAtom};
use expr::{fraction::Fraction, radical::Radical, AlgExpr};

/// Algebraic Notation.
///
/// Notation representing an algebraic element.
///
/// <div class="warning">
///
/// # Equality operation is intended only to be used on notation that has already been simplified.
///
/// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.\
/// **Does not test literal equality either.** [`Undefined`][AlgAtom::Undefined] is not equal to [`Undefined`][AlgAtom::Undefined].
///
/// </div>
///
/// ```
/// # use algebra::notation::{AlgNotation, expr::{fraction::Fraction, radical::Radical, Simplify}};
/// let a = AlgNotation::from(Fraction::from_ints(1, 5));
/// let b = AlgNotation::from(Fraction::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = AlgNotation::from(Radical::from_ints(1, 5));
/// let b = AlgNotation::from(Radical::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = AlgNotation::from(Radical::from_ints(1, 1));
/// let b = AlgNotation::from(Fraction::from_ints(1, 1));
/// assert_ne!(a, b); // Even though both are equal to 1
///
/// let a = AlgNotation::from(Radical::from_ints(1, 8));
/// let b = AlgNotation::from(Radical::from_ints(2, 2));
/// assert_ne!(a, b); // Even though they are equivalent mathematically
/// if let AlgNotation::Expr(expr) = a {
///     assert_eq!(expr.simplified(), b); // They need to be simplified first
/// } else {
///     unreachable!();
/// }
/// ```
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

impl<T: Into<AlgAtom>> From<T> for AlgNotation {
    fn from(value: T) -> Self {
        Self::Atom(value.into())
    }
}

impl<T: Into<AlgExpr>> From<T> for AlgNotation {
    fn from(value: T) -> Self {
        Self::Expr(value.into())
    }
}

impl std::cmp::PartialEq for AlgNotation {
    fn eq(&self, other: &Self) -> bool {
        use AlgNotation::*;
        match (self, other) {
            (Atom(atom_a), Atom(atom_b)) => atom_a == atom_b,
            (Expr(expr_a), Expr(expr_b)) => expr_a == expr_b,
            (Atom(_), Expr(_)) | (Expr(_), Atom(_)) => false,
        }
    }
}

impl<T> std::cmp::PartialEq<T> for AlgNotation
where
    T: PartialEq<AlgAtom>
        + PartialEq<Number>
        + PartialEq<AlgExpr>
        + PartialEq<Fraction>
        + PartialEq<Radical>
        + PartialEq<i32>,
{
    fn eq(&self, other: &T) -> bool {
        use AlgNotation::*;
        match self {
            Atom(atom) => other == atom,
            Expr(expr) => other == expr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_expr() {
        let full = AlgNotation::Expr(AlgExpr::Fraction(Fraction::from_ints(1, 1)));
        let short = AlgNotation::from(Fraction::from_ints(1, 1));
        assert_eq!(full, short);
    }
}
