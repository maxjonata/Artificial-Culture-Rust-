//! Tests for core type definitions.
use crate::core::types::*;

#[test]
fn test_normalized_construction() {
    assert_eq!(Normalized::new(0.5).get(), 0.5);
    assert_eq!(Normalized::new(1.5).get(), 1.0); // Clamped
    assert_eq!(Normalized::new(-0.5).get(), 0.0); // Clamped
}

#[test]
fn test_normalized_arithmetic() {
    let a = Normalized::new(0.3);
    let b = Normalized::new(0.8);
    
    assert_eq!((a + b).get(), 1.0); // Clamped to 1.0
    assert_eq!((b - a).get(), 0.5);
    assert!((a * b).get() - 0.24 < 0.001); // Approximately 0.24
}

#[test]
fn test_normalized_constants() {
    assert_eq!(Normalized::ZERO.get(), 0.0);
    assert_eq!(Normalized::ONE.get(), 1.0);
    assert_eq!(Normalized::HALF.get(), 0.5);
    assert_eq!(Normalized::QUARTER.get(), 0.25);
    assert_eq!(Normalized::THREE_QUARTERS.get(), 0.75);
}

#[test]
fn test_quantized_conversion() {
    let n = Normalized::new(0.5);
    let quantized = n.to_quantized();
    let back = Normalized::from_quantized(quantized);
    
    // Should be very close (within quantization error)
    assert!((n.get() - back.get()).abs() < 0.01);
}

#[test]
fn test_normalized_assignment_operations() {
    let mut n = Normalized::new(0.3);
    n += 0.2;
    assert_eq!(n.get(), 0.5);
    
    n += 1.0; // Should clamp to 1.0
    assert_eq!(n.get(), 1.0);
    
    n -= 0.5;
    assert_eq!(n.get(), 0.5);
    
    n *= 2.0; // Should clamp to 1.0
    assert_eq!(n.get(), 1.0);
}

#[test]
fn test_normalized_try_new() {
    assert!(Normalized::try_new(0.5).is_ok());
    assert!(Normalized::try_new(1.5).is_err());
    assert!(Normalized::try_new(-0.5).is_err());
}

#[test]
fn test_normalized_lerp() {
    let a = Normalized::new(0.2);
    let b = Normalized::new(0.8);
    let t = Normalized::new(0.5);
    
    let result = a.lerp(b, t);
    assert_eq!(result.get(), 0.5); // Midpoint between 0.2 and 0.8
}

#[test]
fn test_normalized_clamp() {
    let value = Normalized::new(0.7);
    let min = Normalized::new(0.3);
    let max = Normalized::new(0.6);
    
    let clamped = value.clamp(min, max);
    assert_eq!(clamped.get(), 0.6); // Clamped to max
}

#[test]
fn test_normalized_division_by_zero_f32() {
    let a = Normalized::new(0.5);
    let result = a / 0.0; // f32 division by zero produces infinity, then gets clamped
    assert_eq!(result.get(), 1.0); // Infinity gets clamped to 1.0
}

#[test]
#[should_panic]
fn test_normalized_division_by_zero_normalized() {
    let a = Normalized::new(0.5);
    let b = Normalized::ZERO;
    let _result = a / b; // Should panic like standard Rust
}

#[test]
fn test_normalized_from_conversions() {
    // From u8
    let from_u8: Normalized = 127u8.into();
    assert!((from_u8.get() - 0.498).abs() < 0.01);
    
    // To f32
    let n = Normalized::new(0.7);
    let as_f32: f32 = n.into();
    assert_eq!(as_f32, 0.7);
    
    // To u8
    let as_u8: u8 = n.into();
    assert_eq!(as_u8, 178); // 0.7 * 255 ≈ 178
}