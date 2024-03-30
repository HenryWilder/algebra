//! Algebraic types which cannot be broken down or simplified.

pub mod number;

use number::Number;

/// Algebraic Atom.
///
/// The smallest unit of an algebraic expression.
#[derive(Clone, Debug)]
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

impl std::cmp::PartialEq<i32> for AlgAtom {
    fn eq(&self, other: &i32) -> bool {
        match self {
            AlgAtom::Number(n) => n == other,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq for AlgAtom {
    fn eq(&self, other: &Self) -> bool {
        use AlgAtom::*;
        // I want to get errors when I add a new AlgAtom type without an equality test.
        #[allow(unreachable_patterns)]
        match (self, other) {
            (Number(num_a), Number(num_b)) => num_a == num_b,

            // Number cannot be compared with non-numbers.
            (Number(_), _) | (_, Number(_)) => false,

            // As of the current implementation, two complex numbers do not have enough information to be compared.
            (Complex, _) | (_, Complex) => false,

            // As of the current implementation, two "Epsilon" numbers do not have enough information to be compared.
            (Epsilon, _) | (_, Epsilon) => false,

            // As of the current implementation, two "Huge" numbers do not have enough information to be compared.
            (Huge, _) | (_, Huge) => false,

            // In no implementation can undefined numbers be meaningfully equal to anything.
            (Undefined, _) | (_, Undefined) => false,
        }
    }
}

impl std::cmp::PartialEq<Number> for AlgAtom {
    fn eq(&self, other: &Number) -> bool {
        use AlgAtom::*;
        if let Number(num) = self {
            num == other
        } else {
            false
        }
    }
}
