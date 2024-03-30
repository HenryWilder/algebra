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
///
/// <div class="warning">
///
/// # Equality operation is intended only to be used on notation that has already been simplified.
///
/// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.\
///
/// </div>
///
/// ```
/// # use algebra::notation::{AlgNotation, expr::{fraction::Fraction, radical::Radical, AlgExpr, Simplify}};
/// let a = AlgExpr::from(Fraction::from_ints(1, 5));
/// let b = AlgExpr::from(Fraction::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = AlgExpr::from(Radical::from_ints(1, 5));
/// let b = AlgExpr::from(Radical::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = AlgExpr::from(Radical::from_ints(1, 1));
/// let b = AlgExpr::from(Fraction::from_ints(1, 1));
/// assert_ne!(a, b); // Even though both are equal to 1
///
/// let a = AlgExpr::from(Radical::from_ints(1, 8));
/// let b = AlgExpr::from(Radical::from_ints(2, 2));
/// assert_ne!(a, b); // Even though they are equivalent mathematically
/// assert_eq!(a.simplified(), AlgNotation::Expr(b)); // They need to be simplified first
/// ```
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

impl Simplify for AlgExpr {
    fn simplified(&self) -> AlgNotation {
        use AlgExpr::*;
        match self {
            Fraction(f) => f.simplified(),
            Radical(r) => r.simplified(),
        }
    }
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
    fn eq(&self, other: &Self) -> bool {
        use AlgExpr::*;
        // I want to get errors when I add a new AlgExpr type without an equality test.
        #[allow(unreachable_patterns)]
        match (self, other) {
            (Fraction(frac_a), Fraction(frac_b)) => frac_a == frac_b,
            (Radical(rad_a), Radical(rad_b)) => rad_a == rad_b,

            // Because we are expecting a simplified value, we already know that a radical and non-radical aren't equal.
            (Radical(_), _) | (_, Radical(_)) => false,

            // Because we are expecting a simplified value, we already know that a fraction and non-fraction aren't equal.
            (Fraction(_), _) | (_, Fraction(_)) => false,
        }
    }
}

impl std::cmp::PartialEq<Fraction> for AlgExpr {
    fn eq(&self, other: &Fraction) -> bool {
        use AlgExpr::*;
        if let Fraction(frac) = self {
            frac == other
        } else {
            false
        }
    }
}

impl std::cmp::PartialEq<Radical> for AlgExpr {
    fn eq(&self, other: &Radical) -> bool {
        use AlgExpr::*;
        if let Radical(rad) = self {
            rad == other
        } else {
            false
        }
    }
}
