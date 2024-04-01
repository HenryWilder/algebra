#![doc = include_str!("atom.md")]

#[doc = include_str!("atom.md")]
#[derive(Clone, Debug)]
pub enum Atom {
    /// An explicit integer value.
    Num(i32),

    /// A variable who value is unknown or being calculated.
    ///
    /// The [`String`] value is the variable's ID, like "x" or "r_distance"
    Var(String),

    /// The square root of a negative.
    ///
    /// The [`i32`] value is the negative number the variable is a square root of.
    Imaginary(i32),

    /// Any number divided by zero.
    Undefined,

    /// A number which isn't infinite, but whose magnitude is too large to be operated on.
    ///
    /// More specifically, Huge represents any number which causes integer overflow.
    Huge,

    /// A negative number which isn't infinite, but whose magnitude is too large to be operated on.
    ///
    /// More specifically, NegHuge represents any number which causes integer underflow.
    NegHuge,

    /// A fraction which isn't zero, but is too small to be operated on.
    ///
    /// More specifically, Epsilon represents a positive fraction whose denominator causes integer overflow.
    Epsilon,

    /// A negative fraction which isn't zero, but is too small to be operated on.
    ///
    /// More specifically, Epsilon represents a negative fraction whose denominator causes integer overflow.
    NegEpsilon,

    /// Edge case where there isn't enough information to reasonably confirm a value.
    ///
    /// Consider the case of [`Epsilon`] divided by [`Epsilon`]:\
    /// If both are the SAME fraction, the result would be exactly 1.
    /// However, [`Epsilon`] doesn't store enough information to distinguish from one another.
    ///
    /// If the numerator [`Epsilon`] is twice the value of the denominator [`Epsilon`], the result would be 2.\
    /// If the denominator [`Epsilon`] is twice the value of the numerator [`Epsilon`], the result would be 1/2.\
    /// But [`Epsilon`]s can't be compared. They are by definition unrepresentable within our limited digits.
    ///
    /// We know it is definitely a finite number.
    /// It is neither [`Complex`] nor [`Undefined`], but we don't know what it _is_.
    Unknown,
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

use Atom::*;

impl Atom {
    /// If [`Atom::Num`], returns its value. Otherwise returns [`None`].
    pub fn number(self) -> Option<i32> {
        match self {
            Num(n) => Some(n),
            _ => None,
        }
    }

    /// Returns true if `self` is of the provided variant, otherwise false.
    pub fn is_variant(&self, variant: Atom) -> bool {
        match variant {
            Num(_) => matches!(self, Num(_)),
            Var(_) => matches!(self, Var(_)),
            Complex => matches!(self, Complex),
            Undefined => matches!(self, Undefined),
            Huge => matches!(self, Huge),
            NegHuge => matches!(self, NegHuge),
            Epsilon => matches!(self, Epsilon),
            NegEpsilon => matches!(self, NegEpsilon),
            Unknown => matches!(self, Unknown),
        }
    }

    /// Returns true for [`Atom::Num`], false otherwise.
    pub fn is_number(&self) -> bool {
        matches!(self, Num(_))
    }

    /// Returns true for [`Atom::Var`], false otherwise.
    pub fn is_variable(&self) -> bool {
        matches!(self, Var(_))
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

    /// Returns true for [`Unknown`], false otherwise.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Unknown)
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        match self {
            Num(n) => n.fmt(f),
            Var(v) => v.fmt(f),
            Complex => "𝑖".fmt(f),
            Undefined => "∅".fmt(f),
            Huge => "𝓗".fmt(f),
            NegHuge => "-𝓗".fmt(f),
            Epsilon => "ε".fmt(f),
            NegEpsilon => "-ε".fmt(f),
            Unknown => "?".fmt(f),
        }
    }
}

impl std::cmp::PartialEq for Atom {
    /// In the current implementation, only [`Num`]s and [`Var`]s can be meaningfully tested for equality.
    ///
    /// [`Complex`] is planned for meaningful comparison in the future, but is not currently implemented.
    ///
    /// [`Huge`], [`NegHuge`], [`Epsilon`], [`NegEpsilon`], and [`Unknown`]
    /// do not store distinguishing information, despite technically representing defined, finite, real numbers.
    ///
    /// [`Undefined`] equality however, is **not** mathematically defined.\
    /// Two instances of 1/0 aren't meaningfully equal; similar to [`NAN`][std::f32::NAN].
    fn eq(&self, other: &Atom) -> bool {
        use Atom::*;
        match (self, other) {
            // TODO: implement for Complex | delegated until Complex is implemented.
            (Num(n1), Num(n2)) => n1 == n2,
            (Var(v1), Var(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<i32> for Atom {
    fn eq(&self, other: &i32) -> bool {
        if let Num(n) = self {
            n == other
        } else {
            false
        }
    }
}
