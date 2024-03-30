//! Definitions of algebraic types.

pub mod atom;
pub mod expr;

use atom::{number::Number, Atom};
use expr::{fraction::Fraction, radical::Radical, Expr};

/// Algebraic Notation.
///
/// Notation representing an algebraic element.
///
/// <div class="warning">
///
/// # Equality operation is intended only to be used on notation that has already been simplified.
///
/// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.\
/// **Does not test literal equality either.** [`Undefined`][Atom::Undefined] is not equal to [`Undefined`][Atom::Undefined].
///
/// </div>
///
/// ```
/// # use algebra::notation::{Notation, expr::{fraction::Fraction, radical::Radical, Simplify}};
/// let a = Notation::from(Fraction::from_ints(1, 5));
/// let b = Notation::from(Fraction::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = Notation::from(Radical::from_ints(1, 5));
/// let b = Notation::from(Radical::from_ints(1, 5));
/// assert_eq!(a, b);
///
/// let a = Notation::from(Radical::from_ints(1, 1));
/// let b = Notation::from(Fraction::from_ints(1, 1));
/// assert_ne!(a, b); // Even though both are equal to 1
///
/// let a = Notation::from(Radical::from_ints(1, 8));
/// let b = Notation::from(Radical::from_ints(2, 2));
/// assert_ne!(a, b); // Even though they are equivalent mathematically
/// if let Notation::Expr(expr) = a {
///     assert_eq!(expr.simplified(), b); // They need to be simplified first
/// } else {
///     unreachable!();
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum Notation {
    /// The smallest unit, a single value.
    ///
    /// See [`Atom`].
    Atom(Atom),

    /// A combination of atomics, able to be simplified.
    ///
    /// See [`Expr`]
    Expr(Expr),
}

impl std::fmt::Display for Notation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Notation::*;
        match self {
            Atom(atom) => atom.fmt(f),
            Expr(expr) => expr.fmt(f),
        }
    }
}

impl Notation {
    /// If the notation represents an [`Atom`][Atom], returns that atom. Otherwise returns [`None`].
    pub fn atom(self) -> Option<Atom> {
        match self {
            Notation::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Atom`][Atom], false otherwise.
    pub fn is_atom(&self) -> bool {
        match self {
            Notation::Atom(_) => true,
            _ => false,
        }
    }

    /// If the notation represents an [`Expr`][Expr], returns that expr. Otherwise returns [`None`].
    pub fn expr(self) -> Option<Expr> {
        match self {
            Notation::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Expr`][Expr], false otherwise.
    pub fn is_expr(&self) -> bool {
        match self {
            Notation::Expr(_) => true,
            _ => false,
        }
    }
}

// # Conversion

// ## Atoms

impl From<Atom> for Notation {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

// ### Number

impl From<Number> for Notation {
    fn from(value: Number) -> Self {
        Self::from(Atom::Number(value))
    }
}

impl From<i32> for Notation {
    fn from(value: i32) -> Self {
        Self::from(Atom::from(value))
    }
}

// ## Expressions

impl From<Expr> for Notation {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

// ### Fraction

impl From<Fraction> for Notation {
    fn from(value: Fraction) -> Self {
        Self::from(Expr::from(value))
    }
}

// ### Radical

impl From<Radical> for Notation {
    fn from(value: Radical) -> Self {
        Self::from(Expr::from(value))
    }
}

// # Equality

// ## Atoms

impl std::cmp::PartialEq<Atom> for Notation {
    fn eq(&self, other: &Atom) -> bool {
        match self {
            Self::Atom(atom) => atom == other,
            _ => false,
        }
    }
}

// ### Number

impl std::cmp::PartialEq<Number> for Notation {
    fn eq(&self, other: &Number) -> bool {
        match self {
            Self::Atom(atom) => atom == other,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<i32> for Notation {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Atom(atom) => atom == other,
            _ => false,
        }
    }
}

// ## Expressions

impl std::cmp::PartialEq<Expr> for Notation {
    fn eq(&self, other: &Expr) -> bool {
        match self {
            Self::Expr(expr) => expr == other,
            _ => false,
        }
    }
}

// ### Fraction

impl std::cmp::PartialEq<Fraction> for Notation {
    fn eq(&self, other: &Fraction) -> bool {
        match self {
            Self::Expr(expr) => expr == other,
            _ => false,
        }
    }
}

// ### Radical

impl std::cmp::PartialEq<Radical> for Notation {
    fn eq(&self, other: &Radical) -> bool {
        match self {
            Self::Expr(expr) => expr == other,
            _ => false,
        }
    }
}

// Tests

#[cfg(test)]
mod tests {}
