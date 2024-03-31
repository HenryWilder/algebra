//! Algebraic exponentiation

use crate::notation::Notation;

impl Notation {
    /// Puts one value to the power of another.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// If the result has a [`Huge`] denominator, returns [`Epsilon`].\
    /// If the result has a denominator of 0, returns [`Undefined`].\
    /// If the base and exponent are both negative, returns [`Complex`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    pub fn pow(self, _rhs: Self) -> Self {
        todo!()
    }
}
