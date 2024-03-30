//! The trait giving expressions the ability to be simplified.

use crate::Notation;

/// An expression capable of being simplified.
pub trait Simplify {
    /// Converts the expression to its simplest form.
    fn simplify(self) -> Notation;

    /// Returns the simplest form of the expression.
    fn simplified(&self) -> Notation
    where
        Self: Clone,
    {
        self.clone().simplify()
    }
}
