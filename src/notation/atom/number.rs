//! A single number.

/// A single integer number.
pub struct Number {
    /// The value the number represents.
    pub value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Number {
    /// Create a new Number object from a value.
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
