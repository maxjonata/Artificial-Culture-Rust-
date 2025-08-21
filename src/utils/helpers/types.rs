/// Converts Big Five personality trait (1-5 scale) to type-safe normalized value.
#[inline]
pub fn big_five_to_normalized(big_five_score: f32) -> crate::core::types::Normalized {
    crate::core::types::Normalized::new((big_five_score - 1.0) / 4.0)
}

/// Validates that a value is within the 0.0-1.0 range for normalized types.
#[inline]
pub fn validate_normalized(value: f32, name: &str) -> bool {
    assert_range(value, name, 0.0..=1.0)
}

/// Asserts that a value is within a given range.
#[inline]
pub fn assert_range<T>(value: T, name: &str, range: std::ops::RangeInclusive<T>) -> bool
where
    T: PartialOrd + std::fmt::Display + Copy,
{
    assert!(range.contains(&value),
            "{} must be in range [{}, {}], got: {}",
            name,
            range.start(),
            range.end(),
            value
    );
    true
}