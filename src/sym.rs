//! Definitions of algebraic types.

pub mod atom;
pub mod expr;
pub mod form;
pub mod ops;

use atom::Atom;
use expr::Expr;
use form::Form;

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
    #[doc = include_str!("sym/atom.md")]
    Atom(Atom),

    #[doc = include_str!("sym/expr.md")]
    Expr(Expr),

    #[doc = include_str!("sym/form.md")]
    Form(Form),
}

impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Sym::*;
        match self {
            Atom(atom) => atom.fmt(f),
            Expr(expr) => expr.fmt(f),
            Form(form) => form.fmt(f),
        }
    }
}

impl Sym {
    /// If the notation represents an [`Atom`], returns that atom. Otherwise returns [`None`].
    pub fn atom(self) -> Option<Atom> {
        match self {
            Sym::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Atom`], false otherwise.
    pub fn is_atom(&self) -> bool {
        matches!(self, Sym::Atom(_))
    }

    /// If the notation represents an [`Expr`], returns that expr. Otherwise returns [`None`].
    pub fn expr(self) -> Option<Expr> {
        match self {
            Sym::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    /// Returns true if the notation represents an [`Expr`], false otherwise.
    pub fn is_expr(&self) -> bool {
        matches!(self, Sym::Expr(_))
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
