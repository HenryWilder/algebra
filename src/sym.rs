//! Definitions of algebraic types.

pub mod atom;
pub mod expr;
pub mod ops;

use atom::Atom;
use expr::Expr;

/// "Symbol" - Algebraic Notation.
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
#[derive(Debug, PartialEq, Clone)]
pub enum Sym {
    /// The smallest unit, a single value.
    ///
    /// See [`Atom`].
    Atom(Atom),

    /// A combination of atomics, able to be simplified.
    ///
    /// See [`Expr`]
    Expr(Expr),
}

impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Sym::*;
        match self {
            Atom(atom) => atom.fmt(f),
            Expr(expr) => expr.fmt(f),
        }
    }
}

impl Sym {
    /// If the notation represents an [`Atom`][Atom], returns that atom. Otherwise returns [`None`].
    pub fn atom(self) -> Option<Atom> {
        match self {
            Sym::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Atom`][Atom], false otherwise.
    pub fn is_atom(&self) -> bool {
        match self {
            Sym::Atom(_) => true,
            _ => false,
        }
    }

    /// If the notation represents an [`Expr`][Expr], returns that expr. Otherwise returns [`None`].
    pub fn expr(self) -> Option<Expr> {
        match self {
            Sym::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Expr`][Expr], false otherwise.
    pub fn is_expr(&self) -> bool {
        match self {
            Sym::Expr(_) => true,
            _ => false,
        }
    }
}

// # Equality

// ## Atoms

impl std::cmp::PartialEq<Atom> for Sym {
    fn eq(&self, other: &Atom) -> bool {
        match self {
            Self::Atom(atom) => atom == other,
            _ => false,
        }
    }
}

// ### Number

impl std::cmp::PartialEq<i32> for Sym {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Atom(atom) => atom == other,
            _ => false,
        }
    }
}

// ## Expressions

impl std::cmp::PartialEq<Expr> for Sym {
    fn eq(&self, other: &Expr) -> bool {
        match self {
            Self::Expr(expr) => expr == other,
            _ => false,
        }
    }
}

// Tests

#[cfg(test)]
mod tests {}
