//! Functions for handling algebraic math.

pub mod add;
pub mod div;
pub mod mul;
pub mod pow;

#[cfg(test)]
mod assumption_tests {

    // Positive huge

    #[test]
    fn test_huge_plus_positive_does_overflow() {
        assert_eq!(i32::MAX.checked_add(1), None);
    }

    #[test]
    fn test_huge_plus_zero_does_not_overflow() {
        assert_ne!(i32::MAX.checked_add(0), None);
    }

    #[test]
    fn test_huge_plus_negative_does_not_overflow() {
        assert_ne!(i32::MAX.checked_add(-1), None);
    }

    // Negative huge

    #[test]
    fn test_negative_huge_plus_negative_does_underflow() {
        assert_eq!(i32::MIN.checked_add(-1), None);
    }

    #[test]
    fn test_negative_huge_plus_zero_does_not_underflow() {
        assert_ne!(i32::MIN.checked_add(0), None);
    }

    #[test]
    fn test_negative_huge_plus_positive_does_not_underflow() {
        assert_ne!(i32::MIN.checked_add(1), None);
    }

    // Positive and negative huge

    #[test]
    fn test_negative_huge_plus_positive_huge_does_neither_overflow_nor_underflow() {
        assert_eq!(i32::MIN.checked_add(i32::MAX), Some(-1));
    }

    // Negation of huge

    #[test]
    fn test_negating_positive_huge_ne_negative_huge() {
        assert_ne!(-i32::MAX, i32::MIN);
    }

    #[test]
    fn test_negating_negative_huge_does_overflow() {
        assert_eq!(i32::MIN.checked_neg(), None);
    }

    // Saturation

    #[test]
    fn test_overflow_is_max() {
        assert_eq!(i32::MAX.saturating_add(1), i32::MAX);
    }

    #[test]
    fn test_underflow_is_min() {
        assert_eq!(i32::MIN.saturating_add(-1), i32::MIN);
    }
}
