//! Algebraic types which cannot be broken down or simplified.

pub mod number;

use number::Number;

/// Algebraic Atom.
///
/// The smallest unit of an algebraic expression.
#[derive(Clone, Debug)]
pub enum Atom {
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

impl Atom {
    /// If [`Number`][Atom::Number], returns its value. Otherwise returns [`None`].
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::{Atom, number::Number};
    /// let number = Atom::from(5);
    /// assert_eq!(number.number(), Some(Number::from(5)));
    /// let undefined = Atom::Undefined;
    /// assert_eq!(undefined.number(), None);
    /// ```
    pub fn number(self) -> Option<Number> {
        match self {
            Atom::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns true for [`Number`][Atom::Number], false otherwise.
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::Atom;
    /// let number = Atom::from(5);
    /// assert!(number.is_number());
    /// let undefined = Atom::Undefined;
    /// assert!(!undefined.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match self {
            Atom::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true for [`Complex`][Atom::Complex], false otherwise.
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::Atom;
    /// let comlpex = Atom::Complex;
    /// assert!(comlpex.is_complex());
    /// let undefined = Atom::Undefined;
    /// assert!(!undefined.is_complex());
    /// ```
    pub fn is_complex(&self) -> bool {
        match self {
            Atom::Complex => true,
            _ => false,
        }
    }

    /// Returns true for [`Undefined`][Atom::Undefined], false otherwise.
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::Atom;
    /// let undefined = Atom::Undefined;
    /// assert!(undefined.is_undefined());
    /// let number = Atom::from(5);
    /// assert!(!number.is_undefined());
    /// ```
    pub fn is_undefined(&self) -> bool {
        match self {
            Atom::Undefined => true,
            _ => false,
        }
    }

    /// Returns true for [`Huge`][Atom::Huge], false otherwise.
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::Atom;
    /// let huge = Atom::Huge;
    /// assert!(huge.is_huge());
    /// let undefined = Atom::Undefined;
    /// assert!(!undefined.is_huge());
    /// ```
    pub fn is_huge(&self) -> bool {
        match self {
            Atom::Huge => true,
            _ => false,
        }
    }

    /// Returns true for [`Epsilon`][Atom::Epsilon], false otherwise.
    ///
    /// Example
    /// ```
    /// # use crate::algebra::notation::atom::Atom;
    /// let epsilon = Atom::Epsilon;
    /// assert!(epsilon.is_epsilon());
    /// let undefined = Atom::Undefined;
    /// assert!(!undefined.is_epsilon());
    /// ```
    pub fn is_epsilon(&self) -> bool {
        match self {
            Atom::Epsilon => true,
            _ => false,
        }
    }
}

impl From<i32> for Atom {
    /// Construct an [`Algebraic Atomic Number`][Atom::Number] from an integer.
    fn from(value: i32) -> Self {
        Atom::Number(Number::from(value))
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        match self {
            Number(num) => num.fmt(f),
            Complex => "𝑖".fmt(f),
            Undefined => "∅".fmt(f),
            Huge => "𝓗".fmt(f),
            Epsilon => "ε".fmt(f),
        }
    }
}

impl std::cmp::PartialEq for Atom {
    /// In the current implementation, only [`Number`][Atom::Number]s can be meaningfully tested for equality.
    ///
    /// [`Complex`][Atom::Complex], [`Huge`][Atom::Huge], and [`Epsilon`][Atom::Epsilon]
    /// do not store distinguishing information, despite equality being mathematical defined.
    ///
    /// [`Undefined`][Atom::Undefined] equality however, is **not** mathematically defined.\
    /// Two instances of $\frac{1}{0}$ aren't meaningfully equal; similar to [`NAN`][std::f32::NAN].
    fn eq(&self, other: &Self) -> bool {
        use Atom::*;
        match (self, other) {
            (Number(num_a), Number(num_b)) => num_a == num_b,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<Number> for Atom {
    fn eq(&self, other: &Number) -> bool {
        use Atom::*;
        if let Number(num) = self {
            num == other
        } else {
            false
        }
    }
}

impl std::cmp::PartialEq<i32> for Atom {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Atom::Number(n) => n == other,
            _ => false,
        }
    }
}
