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
#[derive(Debug)]
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

impl std::cmp::PartialEq for AlgExpr {
    /// <div class="warning">
    ///
    /// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.
    /// This operation is intended only to be used on notation that has already been simplified.
    ///
    /// </div>
    fn eq(&self, other: &Self) -> bool {
        // I want to get errors when I add a new AlgExpr type without an equality test.
        #[allow(unreachable_patterns)]
        match (self, other) {
            (AlgExpr::Fraction(frac_a), AlgExpr::Fraction(frac_b)) => frac_a == frac_b,
            (AlgExpr::Radical(rad_a), AlgExpr::Radical(rad_b)) => rad_a == rad_b,

            // Because we are expecting a simplified value, we already know that a radical and non-radical aren't equal.
            (AlgExpr::Radical(_), _) | (_, AlgExpr::Radical(_)) => false,

            // Because we are expecting a simplified value, we already know that a fraction and non-fraction aren't equal.
            (AlgExpr::Fraction(_), _) | (_, AlgExpr::Fraction(_)) => false,
        }
    }
}
