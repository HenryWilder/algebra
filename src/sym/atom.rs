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
            Huge => NegHuge,
            NegHuge => Huge,
            Epsilon => NegEpsilon,
            NegEpsilon => Epsilon,

            signless => signless,
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
    /// If [`Atom::Num`], returns its value. Otherwise returns [`None`].
    pub fn number(self) -> Option<i32> {
        match self {
            Num(n) => Some(n),
            _ => None,
        }
    }

    /// Returns true for [`Atom::Num`], false otherwise.
    pub fn is_number(&self) -> bool {
        matches!(self, Num(_))
    }

    /// Returns true for
    /// - [`Atom::Num`] where >= 0
    /// - [`Huge`]
    /// - [`Epsilon`]
    /// and false otherwise.
    pub fn is_positive(&self) -> bool {
        matches!(self, Num(0..) | Huge | Epsilon)
    }

    /// Returns true for
    /// - [`Atom::Num`] where < 0
    /// - [`NegHuge`]
    /// - [`NegEpsilon`]
    /// and false otherwise.
    pub fn is_negative(&self) -> bool {
        matches!(self, Num(..=-1) | NegHuge | NegEpsilon)
    }

    /// Returns true for [`Complex`], false otherwise.
    pub fn is_complex(&self) -> bool {
        matches!(self, Complex)
    }

    /// Returns true for [`Undefined`], false otherwise.
    pub fn is_undefined(&self) -> bool {
        matches!(self, Undefined)
    }

    /// Returns true for [`Huge`] and [`NegHuge`], false otherwise.
    pub fn is_huge(&self) -> bool {
        matches!(self, Huge | NegHuge)
    }

    /// Returns true for [`Huge`], false otherwise.
    pub fn is_positive_huge(&self) -> bool {
        matches!(self, Huge)
    }

    /// Returns true for [`Huge`], false otherwise.
    pub fn is_negative_huge(&self) -> bool {
        matches!(self, NegHuge)
    }

    /// Returns true for [`Epsilon`] and [`NegEpsilon`], false otherwise.
    pub fn is_epsilon(&self) -> bool {
        matches!(self, Epsilon | NegEpsilon)
    }

    /// Returns true for [`Epsilon`], false otherwise.
    pub fn is_positive_epsilon(&self) -> bool {
        matches!(self, Epsilon)
    }

    /// Returns true for [`NegEpsilon`], false otherwise.
    pub fn is_negative_epsilon(&self) -> bool {
        matches!(self, NegEpsilon)
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
    /// In the current implementation, only [`Atom::Num`]s can be meaningfully tested for equality.
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
