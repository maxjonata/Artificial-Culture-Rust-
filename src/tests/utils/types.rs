//! Tests for utility helper functions.
use crate::utils::helpers::types::*;

#[test]
fn test_assert_range_function() {
    assert!(assert_range(5.0, "temperature", 0.0..=10.0));
    assert!(assert_range(25, "age", 18..=65));
}

#[test]
#[should_panic(expected = "temperature must be in range [0, 10], got: 15")]
fn test_assert_range_panic() {
        assert_range(15.0, "temperature", 0.0..=10.0);
    }

#[test]
fn test_big_five_to_normalized() {
    // Test Big Five scale (1-5) conversion
    let low_score = big_five_to_normalized(1.0);
    assert_eq!(low_score.get(), 0.0);
    
    let mid_score = big_five_to_normalized(3.0);
    assert_eq!(mid_score.get(), 0.5);
    
    let high_score = big_five_to_normalized(5.0);
    assert_eq!(high_score.get(), 1.0);
    
    // Out of range should be clamped
    let out_of_range = big_five_to_normalized(6.0);
    assert_eq!(out_of_range.get(), 1.0);
}