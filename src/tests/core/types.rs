//! Tests for core type definitions.

use crate::core::types::*;

#[cfg(test)]
mod normalized_tests {
    use super::*;

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
}

#[cfg(test)]
mod validation_tests {
    use super::*;

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
}

#[cfg(test)]
mod utility_tests {
    use super::*;

    #[test]
    fn test_big_five_to_normalized() {
        // Test Big Five scale (1-5) conversion
        let low_score = utils::big_five_to_normalized(1.0);
        assert_eq!(low_score.get(), 0.0);
        
        let mid_score = utils::big_five_to_normalized(3.0);
        assert_eq!(mid_score.get(), 0.5);
        
        let high_score = utils::big_five_to_normalized(5.0);
        assert_eq!(high_score.get(), 1.0);
        
        // Out of range should be clamped
        let out_of_range = utils::big_five_to_normalized(6.0);
        assert_eq!(out_of_range.get(), 1.0);
    }
}
