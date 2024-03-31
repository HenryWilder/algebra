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

    /// A number which isn't infinite, but whose magnitude is too large to be operated on.
    Huge,

    /// A negative number which isn't infinite, but whose magnitude is too large to be operated on.
    NegativeHuge,

    /// A fraction which isn't zero, but is too small to be operated on.
    Epsilon,

    /// A negative fraction which isn't zero, but is too small to be operated on.
    NegativeEpsilon,
}

impl std::ops::Neg for Atom {
    type Output = Atom;

    fn neg(self) -> Self::Output {
        match self {
            Atom::Number(n) => Atom::Number(-n),
            Complex => Complex,
            Undefined => Undefined,
            Huge => NegativeHuge,
            NegativeHuge => Huge,
            Epsilon => NegativeEpsilon,
            NegativeEpsilon => Epsilon,
        }
    }
}

macro_rules! symbol {
    [Imaginary] => ("𝑖");
    [EmptySet] => ("∅");
    [Huge] => ("𝓗");
    [Epsilon] => ("ε");
}

use Atom::*;

impl Atom {
    /// If [`Atom::Number`], returns its [`Number`]. Otherwise returns [`None`].
    pub fn number(self) -> Option<Number> {
        match self {
            Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns true for [`Atom::Number`], false otherwise.
    pub fn is_number(&self) -> bool {
        match self {
            Number(_) => true,
            _ => false,
        }
    }

    /// Returns true for
    /// - [`Atom::Number`] where >= 0
    /// - [`Huge`]
    /// - [`Epsilon`]
    /// and false otherwise.
    pub fn is_positive(&self) -> bool {
        match self {
            Number(Number { value: 0.. }) | Huge | Epsilon => true,
            _ => false,
        }
    }

    /// Returns true for
    /// - [`Atom::Number`] where < 0
    /// - [`NegativeHuge`]
    /// - [`NegativeEpsilon`]
    /// and false otherwise.
    pub fn is_negative(&self) -> bool {
        match self {
            Number(Number { value: ..=-1 }) | NegativeHuge | NegativeEpsilon => true,
            _ => false,
        }
    }

    /// Returns true for [`Complex`], false otherwise.
    pub fn is_complex(&self) -> bool {
        match self {
            Complex => true,
            _ => false,
        }
    }

    /// Returns true for [`Undefined`], false otherwise.
    pub fn is_undefined(&self) -> bool {
        match self {
            Undefined => true,
            _ => false,
        }
    }

    /// Returns true for [`Huge`] and [`NegativeHuge`], false otherwise.
    pub fn is_huge(&self) -> bool {
        match self {
            Huge | NegativeHuge => true,
            _ => false,
        }
    }

    /// Returns true for [`Huge`], false otherwise.
    pub fn is_positive_huge(&self) -> bool {
        match self {
            Huge => true,
            _ => false,
        }
    }

    /// Returns true for [`Huge`], false otherwise.
    pub fn is_negative_huge(&self) -> bool {
        match self {
            NegativeHuge => true,
            _ => false,
        }
    }

    /// Returns true for [`Epsilon`] and [`NegativeEpsilon`], false otherwise.
    pub fn is_epsilon(&self) -> bool {
        match self {
            Epsilon | NegativeEpsilon => true,
            _ => false,
        }
    }

    /// Returns true for [`Epsilon`], false otherwise.
    pub fn is_positive_epsilon(&self) -> bool {
        match self {
            Epsilon => true,
            _ => false,
        }
    }

    /// Returns true for [`NegativeEpsilon`], false otherwise.
    pub fn is_negative_epsilon(&self) -> bool {
        match self {
            NegativeEpsilon => true,
            _ => false,
        }
    }
}

impl From<i32> for Atom {
    /// Construct an [`Atom::Number`] from an integer.
    fn from(value: i32) -> Self {
        Number(Number::from(value))
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        match self {
            Number(n) => n.fmt(f),
            Complex => symbol![Imaginary].fmt(f),
            Undefined => symbol![EmptySet].fmt(f),
            Huge => symbol![Huge].fmt(f),
            NegativeHuge => concat!("-", symbol![Huge]).fmt(f),
            Epsilon => symbol![Epsilon].fmt(f),
            NegativeEpsilon => concat!("-", symbol![Epsilon]).fmt(f),
        }
    }
}

impl std::cmp::PartialEq for Atom {
    /// In the current implementation, only [`Atom::Number`]s can be meaningfully tested for equality.
    ///
    /// [`Complex`], [`Huge`], and [`Epsilon`]
    /// do not store distinguishing information, despite equality being mathematical defined.
    ///
    /// [`Undefined`] equality however, is **not** mathematically defined.\
    /// Two instances of 1/0 aren't meaningfully equal; similar to [`NAN`][std::f32::NAN].
    fn eq(&self, other: &Self) -> bool {
        use Atom::*;
        match (self, other) {
            (Number(a), Number(b)) => a == b,
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
            Number(n) => n == other,
            _ => false,
        }
    }
}
