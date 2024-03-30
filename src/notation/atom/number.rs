//! A single number.

/// A single integer number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    /// The value the number represents.
    pub value: i32,
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
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
