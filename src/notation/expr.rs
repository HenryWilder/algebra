//! Algebraic expressions comprised of multiple parts, which can be simplified.

pub mod fraction;
pub mod radical;

use crate::AlgNotation;
use fraction::Fraction;
use radical::Radical;

/// An expression capable of being simplified.
pub trait Simplify {
    /// Returns the simplest form of the expression.
    fn simplified(&self) -> AlgNotation;
}

/// Algebraic Expression.
///
/// Notation representing an algebraic expression.
/// Expressions can be simplified.
pub enum AlgExpr {
    /// A fraction.
    ///
    /// See [`Fraction`]
    Fraction(Fraction),

    /// A radical.
    ///
    /// See [`Radical`]
    Radical(Radical),
}

impl ToString for AlgExpr {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl From<Fraction> for AlgExpr {
    fn from(value: Fraction) -> Self {
        AlgExpr::Fraction(value)
    }
}

impl From<Radical> for AlgExpr {
    fn from(value: Radical) -> Self {
        AlgExpr::Radical(value)
    }
}
