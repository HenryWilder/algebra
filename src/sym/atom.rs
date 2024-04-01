//! Algebraic types which cannot be broken down or simplified.

/// Algebraic Atom.
///
/// The smallest unit of an algebraic expression.
#[derive(Clone, Debug)]
pub enum Atom {
    /// An explicit integer value.
    Num(i32),

    /// The square root of a negative.
    Complex,

    /// Any number divided by zero.
    Undefined,

    /// A number which isn't infinite, but whose magnitude is too large to be operated on.
    Huge,

    /// A negative number which isn't infinite, but whose magnitude is too large to be operated on.
    NegHuge,

    /// A fraction which isn't zero, but is too small to be operated on.
    Epsilon,

    /// A negative fraction which isn't zero, but is too small to be operated on.
    NegEpsilon,
}

impl std::ops::Neg for Atom {
    type Output = Atom;

    fn neg(self) -> Self::Output {
        match self {
            Num(n) => Num(-n),
            Complex => Complex,
            Undefined => Undefined,
            Huge => NegHuge,
            NegHuge => Huge,
            Epsilon => NegEpsilon,
            NegEpsilon => Epsilon,
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
    pub fn number(self) -> Option<i32> {
        match self {
            Num(n) => Some(n),
            _ => None,
        }
    }

    /// Returns true for [`Atom::Number`], false otherwise.
    pub fn is_number(&self) -> bool {
        match self {
            Num(_) => true,
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
            Num(0..) | Huge | Epsilon => true,
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
            Num(..=-1) | NegHuge | NegEpsilon => true,
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
            Huge | NegHuge => true,
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
            NegHuge => true,
            _ => false,
        }
    }

    /// Returns true for [`Epsilon`] and [`NegativeEpsilon`], false otherwise.
    pub fn is_epsilon(&self) -> bool {
        match self {
            Epsilon | NegEpsilon => true,
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
            NegEpsilon => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        match self {
            Num(n) => n.fmt(f),
            Complex => symbol![Imaginary].fmt(f),
            Undefined => symbol![EmptySet].fmt(f),
            Huge => symbol![Huge].fmt(f),
            NegHuge => concat!("-", symbol![Huge]).fmt(f),
            Epsilon => symbol![Epsilon].fmt(f),
            NegEpsilon => concat!("-", symbol![Epsilon]).fmt(f),
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
            (Num(a), Num(b)) => a == b,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<i32> for Atom {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Num(n) => n == other,
            _ => false,
        }
    }
}
