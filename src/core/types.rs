//! Core type definitions for the Artificial Society simulation.
//!
//! This module enforces the project's performance and data integrity requirements:
//! - All floating-point values normalized to [0.0, 1.0] range
//! - Smallest possible types for optimal memory layout and cache performance
//! - Type safety for scientific accuracy

use bevy::prelude::*;

/// Normalized floating-point value, always constrained to [0.0, 1.0].
/// Used for all physiological, cognitive, and social measurements.
pub type NormalizedFloat = f32;

/// Compact agent identifier, supports up to 65,535 agents.
/// Use this instead of Entity for internal references to optimize memory.
pub type AgentId = u16;

/// Quantized weight value (0-255) that maps to [0.0, 1.0] range.
/// Used for neural connections and memory weights to save memory.
pub type QuantizedWeight = u8;

/// Converts a QuantizedWeight to NormalizedFloat.
#[inline]
pub fn quantized_to_float(weight: QuantizedWeight) -> NormalizedFloat {
    weight as f32 / 255.0
}

/// Converts a NormalizedFloat to QuantizedWeight.
#[inline]
pub fn float_to_quantized(value: NormalizedFloat) -> QuantizedWeight {
    (value.clamp(0.0, 1.0) * 255.0) as u8
}

/// Validation macro to ensure values stay within [0.0, 1.0] range.
/// Usage: validate_normalized!(my_value);
#[macro_export]
macro_rules! validate_normalized {
    ($value:expr) => {
        debug_assert!(
            $value >= 0.0 && $value <= 1.0,
            "Value {} must be in range [0.0, 1.0], got: {}",
            stringify!($value),
            $value
        );
    };
}

/// Utility functions for safe floating-point operations.
pub mod utils {
    use super::NormalizedFloat;

    /// Linear interpolation between two normalized values.
    #[inline]
    pub fn lerp_normalized(a: NormalizedFloat, b: NormalizedFloat, t: NormalizedFloat) -> NormalizedFloat {
        (a + t * (b - a)).clamp(0.0, 1.0)
    }

    /// Converts Big Five personality trait (1-5 scale) to normalized [0.0, 1.0].
    #[inline]
    pub fn big_five_to_normalized(big_five_score: f32) -> NormalizedFloat {
        ((big_five_score - 1.0) / 4.0).clamp(0.0, 1.0)
    }
}
