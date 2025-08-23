//! Legacy tests for migrated utility functions.
//!
//! These tests verify functionality that was migrated from src/utils/helpers/types.rs
//! to src/core/types.rs as methods on the Normalized type, following the
//! component-private functionality principle.

use crate::core::types::Normalized;

#[test]
fn test_assert_range_function() {
    assert!(Normalized::assert_range(5.0, "temperature", 0.0..=10.0));
    assert!(Normalized::assert_range(25, "age", 18..=65));
}

#[test]
#[should_panic(expected = "temperature must be in range [0, 10], got: 15")]
fn test_assert_range_panic() {
    Normalized::assert_range(15.0, "temperature", 0.0..=10.0);
}

#[test]
fn test_big_five_to_normalized() {
    // Test Big Five scale (1-5) conversion
    let low_score = Normalized::from_big_five(1.0);
    assert_eq!(low_score.get(), 0.0);

    let mid_score = Normalized::from_big_five(3.0);
    assert_eq!(mid_score.get(), 0.5);

    let high_score = Normalized::from_big_five(5.0);
    assert_eq!(high_score.get(), 1.0);

    // Out of range should be clamped
    let out_of_range = Normalized::from_big_five(6.0);
    assert_eq!(out_of_range.get(), 1.0);
}

#[test]
fn test_validate_normalized() {
    // Test validation of raw values
    assert!(Normalized::validate_raw(0.5, "test_value"));
    assert!(Normalized::validate_raw(0.0, "min_value"));
    assert!(Normalized::validate_raw(1.0, "max_value"));
}