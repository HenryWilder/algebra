//! A single number.

/// A single integer number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    /// The value the number represents.
    pub value: i32,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

// # Conversion

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

// # Equality

impl std::cmp::PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}
