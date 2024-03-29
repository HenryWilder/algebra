//! Algebraic types which cannot be broken down or simplified.

pub mod number;

use number::Number;

/// Algebraic Atom.
///
/// The smallest unit of an algebraic expression.
pub enum AlgAtom {
    /// An explicit integer value.
    Number(Number),

    /// The square root of a negative.
    Complex,

    /// Any number divided by zero.
    Undefined,

    /// A number which isn't infinite, but is too large to be operated on.
    Huge,

    /// A fraction which isn't zero, but is too small to be operated on.
    Epsilon,
}

impl From<i32> for AlgAtom {
    /// Construct an [`Algebraic Atomic Number`][AlgAtom::Number] from an integer.
    fn from(value: i32) -> Self {
        AlgAtom::Number(Number::from(value))
    }
}

impl ToString for AlgAtom {
    fn to_string(&self) -> String {
        use AlgAtom::*;
        match self {
            Number(num) => num.to_string(),
            Complex => "𝑖".to_string(),
            Undefined => "∅".to_string(),
            Huge => "𝓗".to_string(),
            Epsilon => "ε".to_string(),
        }
    }
}
