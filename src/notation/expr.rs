//! Algebraic expressions comprised of multiple parts, which can be simplified.

pub mod fraction;
pub mod radical;
pub mod simplify;

use crate::Notation;
use fraction::Fraction;
use radical::Radical;
use simplify::Simplify;

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
/// # use algebra::notation::{Notation, expr::{fraction::Fraction, radical::Radical, Expr, Simplify}};
/// let a = Expr::from(Fraction::from_ints(1, 5));
/// let b = Expr::from(Fraction::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = Expr::from(Radical::new(5));
/// let b = Expr::from(Radical::new(5));
/// assert_eq!(a, b);
///
/// let a = Expr::from(Radical::from(1));
/// let b = Expr::from(Fraction::from(1));
/// assert_ne!(a, b); // Even though both are equal to 1
///
/// let a = Expr::from(Radical::new(8));
/// let b = Expr::from(Radical{ coef: 2, rad: 2 });
/// assert_ne!(a, b); // Even though they are equivalent mathematically
/// assert_eq!(a.simplified(), Notation::Expr(b)); // They need to be simplified first
/// ```
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A fraction.
    ///
    /// See [`Fraction`]
    Fraction(Fraction),

    /// A radical.
    ///
    /// See [`Radical`]
    Radical(Radical),
}

impl Expr {
    /// If the expression represents a [`Fraction`], returns that fraction. Otherwise returns [`None`].
    pub fn fraction(self) -> Option<Fraction> {
        match self {
            Expr::Fraction(frac) => Some(frac),
            _ => None,
        }
    }

    /// Returns true if the expression represents a [`Fraction`], false otherwise.
    pub fn is_fraction(&self) -> bool {
        match self {
            Expr::Fraction(_) => true,
            _ => false,
        }
    }

    /// If the expression represents a [`Radical`], returns that radical. Otherwise returns [`None`].
    pub fn radical(self) -> Option<Radical> {
        match self {
            Expr::Radical(rad) => Some(rad),
            _ => None,
        }
    }

    /// Returns true if the expression represents a [`Radical`], false otherwise.
    pub fn is_radical(&self) -> bool {
        match self {
            Expr::Radical(_) => true,
            _ => false,
        }
    }
}

impl Simplify for Expr {
    fn simplify(self) -> Notation {
        use Expr::*;
        match self {
            Fraction(f) => f.simplify(),
            Radical(r) => r.simplify(),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        todo!()
    }
}

// # Conversion

// ## Fraction

impl From<Fraction> for Expr {
    fn from(value: Fraction) -> Self {
        Expr::Fraction(value)
    }
}

// ## Radical

impl From<Radical> for Expr {
    fn from(value: Radical) -> Self {
        Expr::Radical(value)
    }
}

// # Equality

// ## Fraction

impl std::cmp::PartialEq<Fraction> for Expr {
    fn eq(&self, other: &Fraction) -> bool {
        use Expr::*;
        if let Fraction(frac) = self {
            frac == other
        } else {
            false
        }
    }
}

// ## Radical

impl std::cmp::PartialEq<Radical> for Expr {
    fn eq(&self, other: &Radical) -> bool {
        use Expr::*;
        if let Radical(rad) = self {
            rad == other
        } else {
            false
        }
    }
}
