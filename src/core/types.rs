//! Core type definitions for the Artificial Society simulation.
//!
//! This module enforces the project's performance and data integrity requirements:
//! - All floating-point values normalized to configurable ranges
//! - Smallest possible types for optimal memory layout and cache performance
//! - Type safety for scientific accuracy

use bevy::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Sub, SubAssign};

/// Compact agent identifier, supports up to 65,535 agents.
/// Use this instead of Entity for internal references to optimize memory.
pub type EntityId = u16;

/// Quantized weight value (0-255) that maps to [0.0, 1.0] range.
/// Used for neural connections and memory weights to save memory.
pub type QuantizedWeight = u8;

/// Error type for normalized value operations.
#[derive(Debug, Clone, PartialEq)]
pub enum NormalizedError {
    OutOfRange { value: f32, min: f32, max: f32 },
}

impl std::fmt::Display for NormalizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalizedError::OutOfRange { value, min, max } => {
                write!(f, "Value {value} is out of range [{min}, {max}]")
            }
        }
    }
}

impl std::error::Error for NormalizedError {}

/// Builder for creating normalized values with custom min/max ranges.
///
/// Provides a flexible way to create normalized types with different bounds
/// while maintaining type safety and performance. This addresses task 1.2.2
/// from the roadmap for flexible configuration of custom ranges.
///
/// # Examples
/// ```rust
/// use crate::core::types::NormalizedBuilder;
///
/// // Temperature range [0.0, 100.0] normalized to [0.0, 1.0]
/// let temp_builder = NormalizedBuilder::new()
///     .input_range(0.0, 100.0);
/// let temp = temp_builder.build(25.0); // Results in 0.25
///
/// // Custom output range [-1.0, 1.0] for severity values
/// let severity_builder = NormalizedBuilder::new()
///     .output_range(-1.0, 1.0)
///     .input_range(-10.0, 10.0);
/// let severity = severity_builder.build(-3.0); // Results in -0.3
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NormalizedBuilder {
    output_min: f32,
    output_max: f32,
    input_min: Option<f32>,
    input_max: Option<f32>,
}

impl Default for NormalizedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NormalizedBuilder {
    /// Creates a new builder with default [0.0, 1.0] output range.
    #[inline]
    pub const fn new() -> Self {
        Self {
            output_min: 0.0,
            output_max: 1.0,
            input_min: None,
            input_max: None,
        }
    }

    /// Sets the output range for the normalized value.
    ///
    /// # Examples
    /// ```rust
    /// # use crate::core::types::NormalizedBuilder;
    /// let builder = NormalizedBuilder::new().output_range(-1.0, 1.0);
    /// ```
    #[inline]
    pub const fn output_range(mut self, min: f32, max: f32) -> Self {
        self.output_min = min;
        self.output_max = max;
        self
    }

    /// Sets an input range that will be normalized to the output range.
    /// Useful for converting external values (e.g., temperature in Celsius) to normalized form.
    ///
    /// # Examples
    /// ```rust
    /// # use crate::core::types::NormalizedBuilder;
    /// let builder = NormalizedBuilder::new().input_range(0.0, 100.0);
    /// ```
    #[inline]
    pub const fn input_range(mut self, input_min: f32, input_max: f32) -> Self {
        self.input_min = Some(input_min);
        self.input_max = Some(input_max);
        self
    }

    /// Builds a clamped f32 value with the configured ranges.
    #[inline]
    pub fn build(self, value: f32) -> f32 {
        let normalized_value = if let (Some(input_min), Some(input_max)) = (self.input_min, self.input_max) {
            // Convert from input range to output range
            let input_range = input_max - input_min;
            let output_range = self.output_max - self.output_min;
            let normalized = (value - input_min) / input_range;
            self.output_min + normalized * output_range
        } else {
            value
        };

        normalized_value.clamp(self.output_min, self.output_max)
    }

    /// Builds a value, returning an error if the final value is out of range.
    #[inline]
    pub fn try_build(self, value: f32) -> Result<f32, NormalizedError> {
        let result = self.build(value);
        if RangeInclusive::new(self.output_min, self.output_max).contains(&result) {
            Ok(result)
        } else {
            Err(NormalizedError::OutOfRange {
                value: result,
                min: self.output_min,
                max: self.output_max,
            })
        }
    }

    /// Returns the configured output range.
    #[inline]
    pub const fn get_output_range(self) -> (f32, f32) {
        (self.output_min, self.output_max)
    }

    /// Returns the configured input range if set.
    #[inline]
    pub const fn get_input_range(self) -> Option<(f32, f32)> {
        if let (Some(min), Some(max)) = (self.input_min, self.input_max) {
            Some((min, max))
        } else {
            None
        }
    }
}

/// Macro to generate all arithmetic trait implementations for clamped types.
///
/// This eliminates code duplication by generating all arithmetic operations
/// that maintain the type's specific range constraints.
///
/// # Usage
/// ```rust
/// impl_clamped_arithmetic!(TypeName, min_value, max_value);
/// ```
macro_rules! impl_clamped_arithmetic {
    ($type_name:ident, $min:expr, $max:expr) => {
        // Addition operations
        impl Add for $type_name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.0 + rhs.0)
            }
        }

        impl Add<f32> for $type_name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: f32) -> Self::Output {
                Self::new(self.0 + rhs)
            }
        }

        impl AddAssign for $type_name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = Self::new(self.0 + rhs.0);
            }
        }

        impl AddAssign<f32> for $type_name {
            #[inline]
            fn add_assign(&mut self, rhs: f32) {
                *self = Self::new(self.0 + rhs);
            }
        }

        // Subtraction operations
        impl Sub for $type_name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.0 - rhs.0)
            }
        }

        impl Sub<f32> for $type_name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: f32) -> Self::Output {
                Self::new(self.0 - rhs)
            }
        }

        impl SubAssign for $type_name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = Self::new(self.0 - rhs.0);
            }
        }

        impl SubAssign<f32> for $type_name {
            #[inline]
            fn sub_assign(&mut self, rhs: f32) {
                *self = Self::new(self.0 - rhs);
            }
        }

        // Multiplication operations
        impl Mul for $type_name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.0 * rhs.0)
            }
        }

        impl Mul<f32> for $type_name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: f32) -> Self::Output {
                Self::new(self.0 * rhs)
            }
        }

        impl MulAssign for $type_name {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = Self::new(self.0 * rhs.0);
            }
        }

        impl MulAssign<f32> for $type_name {
            #[inline]
            fn mul_assign(&mut self, rhs: f32) {
                *self = Self::new(self.0 * rhs);
            }
        }

        // Division operations
        impl Div for $type_name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                if rhs.0 == 0.0 {
                    panic!("Division by zero in {} arithmetic", stringify!($type_name));
                }
                Self::new(self.0 / rhs.0)
            }
        }

        impl Div<f32> for $type_name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: f32) -> Self::Output {
                Self::new(self.0 / rhs)
            }
        }

        impl DivAssign for $type_name {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                if rhs.0 == 0.0 {
                    panic!("Division by zero in {} arithmetic", stringify!($type_name));
                }
                *self = Self::new(self.0 / rhs.0);
            }
        }

        impl DivAssign<f32> for $type_name {
            #[inline]
            fn div_assign(&mut self, rhs: f32) {
                *self = Self::new(self.0 / rhs);
            }
        }
    };
}

/// Type-safe normalized floating-point value, always constrained to [0.0, 1.0].
///
/// This wrapper type prevents invalid values from being stored at the type level,
/// automatically clamping all operations to maintain the [0.0, 1.0] invariant.
/// Used for all physiological, cognitive, and social measurements.
///
/// # Performance
/// - Zero-cost abstraction: same memory layout as f32
/// - All operations are inlined for optimal performance
/// - Suitable for 60fps simulation with 100+ agents
///
/// # Examples
/// ```rust
/// use crate::core::types::Normalized;
///
/// // Safe construction (clamps automatically)
/// let hunger = Normalized::new(1.5); // Clamped to 1.0
/// let energy = Normalized::new(-0.2); // Clamped to 0.0
///
/// // Arithmetic operations maintain bounds
/// let total = hunger + energy; // Result is clamped to [0.0, 1.0]
/// ```
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[reflect(Component)]
#[repr(transparent)] // Same memory layout as f32
pub struct Normalized(f32);

/// Type-safe severity/polarity value, always constrained to [-1.0, 1.0].
///
/// This specialized normalized type represents bipolar measurements like:
/// - Emotional polarity (negative to positive)
/// - Political alignment (left to right)
/// - Social stance (hostile to friendly)
/// - Environmental conditions (harmful to beneficial)
///
/// Implements task 1.2.3 from the roadmap for bipolar measurements.
///
/// # Performance
/// - Zero-cost abstraction: same memory layout as f32
/// - All operations maintain [-1.0, 1.0] bounds
/// - Suitable for high-frequency emotional and social calculations
///
/// # Examples
/// ```rust
/// use crate::core::types::Severity;
///
/// // Safe construction (clamps automatically)
/// let hostility = Severity::new(-0.8);  // Hostile
/// let neutrality = Severity::new(0.0);   // Neutral
/// let friendliness = Severity::new(0.7); // Friendly
/// let clamped = Severity::new(2.0);      // Clamped to 1.0
///
/// // Arithmetic operations maintain bounds
/// let interaction = hostility + friendliness; // Result clamped to [-1.0, 1.0]
///
/// // Convert to/from standard normalized values
/// let normalized = friendliness.to_normalized(); // Maps to [0.0, 1.0]
/// let severity = Severity::from_normalized(normalized); // Maps back to [-1.0, 1.0]
/// ```
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[reflect(Component)]
#[repr(transparent)] // Same memory layout as f32
pub struct Severity(f32);

impl Normalized {
    /// Creates a new normalized value, clamping the input to [0.0, 1.0].
    #[inline]
    pub const fn new(value: f32) -> Self {
        if value < 0.0 {
            Self(0.0)
        } else if value > 1.0 {
            Self(1.0)
        } else {
            Self(value)
        }
    }

    /// Creates a new normalized value from a value known to be in range.
    ///
    /// # Safety
    /// The caller must ensure that `value` is in the range [0.0, 1.0].
    #[inline]
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    /// Creates a builder for configuring custom input ranges that map to [0.0, 1.0].
    /// This implements the flexible configuration from task 1.2.2.
    #[inline]
    pub fn builder() -> NormalizedBuilder {
        NormalizedBuilder::new()
    }

    /// Attempts to create a normalized value, returning an error if out of range.
    #[inline]
    pub fn try_new(value: f32) -> Result<Self, NormalizedError> {
        if RangeInclusive::new(0.0, 1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(NormalizedError::OutOfRange { value, min: 0.0, max: 1.0 })
        }
    }

    /// Creates a normalized value from an input range, mapping it to [0.0, 1.0].
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// let temp = Normalized::from_range(25.0, 0.0, 100.0); // 0.25
    /// let percentage = Normalized::from_range(75.0, 0.0, 100.0); // 0.75
    /// ```
    #[inline]
    pub fn from_range(value: f32, input_min: f32, input_max: f32) -> Self {
        let builder_result = Self::builder()
            .input_range(input_min, input_max)
            .build(value);
        Self(builder_result)
    }

    /// Returns the inner f32 value.
    ///
    /// This is a zero-cost conversion that extracts the underlying float.
    /// The returned value is guaranteed to be in [0.0, 1.0].
    #[inline]
    pub const fn get(self) -> f32 {
        self.0
    }

    /// Returns the inner f32 value (alias for `get`).
    #[inline]
    pub const fn value(self) -> f32 {
        self.0
    }

    /// Converts to a quantized u8 value (0-255 range).
    ///
    /// This is useful for memory-efficient storage of normalized values.
    #[inline]
    pub fn to_quantized(self) -> QuantizedWeight {
        (self.0 * 255.0) as u8
    }

    /// Creates a normalized value from a quantized u8 (0-255 range).
    #[inline]
    pub fn from_quantized(weight: QuantizedWeight) -> Self {
        // Safe because weight/255.0 is always in [0.0, 1.0]
        unsafe { Self::new_unchecked(weight as f32 / 255.0) }
    }

    /// Linear interpolation between two normalized values.
    #[inline]
    pub fn lerp(self, other: Self, t: Self) -> Self {
        Self::new(self.0 + t.0 * (other.0 - self.0))
    }

    /// Clamps this value between two normalized bounds.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.0.clamp(min.0, max.0))
    }

    /// Creates a normalized value from a Big Five personality trait score (1-5 scale).
    ///
    /// This is a domain-specific conversion utility embedded directly in the type
    /// following the component-private functionality principle.
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// let low_openness = Normalized::from_big_five(1.0);   // 0.0
    /// let mid_openness = Normalized::from_big_five(3.0);   // 0.5
    /// let high_openness = Normalized::from_big_five(5.0);  // 1.0
    /// let clamped = Normalized::from_big_five(6.0);        // 1.0 (clamped)
    /// ```
    #[inline]
    pub fn from_big_five(big_five_score: f32) -> Self {
        Self::new((big_five_score - 1.0) / 4.0)
    }

    /// Validates that a raw f32 value is within the normalized range [0.0, 1.0].
    ///
    /// This is a utility for validating external input before conversion to Normalized.
    /// Panics in debug builds if the value is out of range.
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// assert!(Normalized::validate_raw(0.5, "mood"));
    /// // Normalized::validate_raw(1.5, "invalid"); // Panics in debug
    /// ```
    #[inline]
    pub fn validate_raw(value: f32, name: &str) -> bool {
        Self::assert_range(value, name, 0.0..=1.0)
    }

    /// Generic range assertion utility for validation.
    ///
    /// This is embedded in the Normalized type as it's primarily used for
    /// validating values before normalization.
    #[inline]
    pub fn assert_range<T>(value: T, name: &str, range: RangeInclusive<T>) -> bool
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
}

// === Constants for common normalized values ===
impl Normalized {
    /// Normalized value representing 0.0 (minimum).
    pub const ZERO: Self = Self(0.0);

    /// Normalized value representing 1.0 (maximum).
    pub const ONE: Self = Self(1.0);

    /// Normalized value representing 0.5 (middle).
    pub const HALF: Self = Self(0.5);

    /// Normalized value representing 0.25 (quarter).
    pub const QUARTER: Self = Self(0.25);

    /// Normalized value representing 0.75 (three quarters).
    pub const THREE_QUARTERS: Self = Self(0.75);
}

// === Trait Implementations for Normalized ===

impl Default for Normalized {
    #[inline]
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<QuantizedWeight> for Normalized {
    #[inline]
    fn from(weight: QuantizedWeight) -> Self {
        Self::from_quantized(weight)
    }
}

impl From<Normalized> for f32 {
    #[inline]
    fn from(normalized: Normalized) -> Self {
        normalized.0
    }
}

impl From<Normalized> for QuantizedWeight {
    #[inline]
    fn from(normalized: Normalized) -> Self {
        normalized.to_quantized()
    }
}

impl Severity {
    /// Creates a new severity value, clamping the input to [-1.0, 1.0].
    #[inline]
    pub const fn new(value: f32) -> Self {
        if value < -1.0 {
            Self(-1.0)
        } else if value > 1.0 {
            Self(1.0)
        } else {
            Self(value)
        }
    }

    /// Creates a new severity value from a value known to be in range.
    ///
    /// # Safety
    /// The caller must ensure that `value` is in the range [-1.0, 1.0].
    #[inline]
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    /// Creates a builder for configuring normal severity values.
    /// This uses the flexible configuration from task 1.2.2.
    #[inline]
    pub fn builder() -> NormalizedBuilder {
        NormalizedBuilder::new().output_range(-1.0, 1.0)
    }

    /// Attempts to create a severity value, returning an error if out of range.
    #[inline]
    pub fn try_new(value: f32) -> Result<Self, NormalizedError> {
        if RangeInclusive::new(-1.0, 1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(NormalizedError::OutOfRange { value, min: -1.0, max: 1.0 })
        }
    }

    /// Creates a severity value from an input range, mapping it to [-1.0, 1.0].
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Severity;
    /// let mood = Severity::from_range(-3.0, -10.0, 10.0); // -0.3
    /// let opinion = Severity::from_range(2.0, -5.0, 5.0); // 0.4
    /// ```
    #[inline]
    pub fn from_range(value: f32, input_min: f32, input_max: f32) -> Self {
        let builder_result = Self::builder()
            .input_range(input_min, input_max)
            .build(value);
        Self(builder_result)
    }

    /// Returns the inner f32 value.
    ///
    /// The returned value is guaranteed to be in [-1.0, 1.0].
    #[inline]
    pub const fn get(self) -> f32 {
        self.0
    }

    /// Returns the inner f32 value (alias for `get`).
    #[inline]
    pub const fn value(self) -> f32 {
        self.0
    }

    /// Converts this severity value to a standard normalized value [0.0, 1.0].
    ///
    /// Maps [-1.0, 1.0] to [0.0, 1.0] using linear transformation.
    #[inline]
    pub fn to_normalized(self) -> Normalized {
        Normalized::new((self.0 + 1.0) / 2.0)
    }

    /// Creates a severity value from a standard normalized value [0.0, 1.0].
    ///
    /// Maps [0.0, 1.0] to [-1.0, 1.0] using linear transformation.
    #[inline]
    pub fn from_normalized(normalized: Normalized) -> Self {
        Self::new(normalized.get() * 2.0 - 1.0)
    }

    /// Returns the absolute value of this severity.
    ///
    /// Useful for measuring intensity regardless of polarity.
    #[inline]
    pub fn abs(self) -> Normalized {
        Normalized::new(self.0.abs())
    }

    /// Returns the sign of this severity value.
    ///
    /// Returns -1.0 for negative, 0.0 for zero, 1.0 for positive.
    #[inline]
    pub fn signum(self) -> f32 {
        self.0.signum()
    }

    /// Linear interpolation between two severity values.
    #[inline]
    pub fn lerp(self, other: Self, t: Normalized) -> Self {
        Self::new(self.0 + t.get() * (other.0 - self.0))
    }

    /// Clamps this value between two severity bounds.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.0.clamp(min.0, max.0))
    }

    /// Creates a severity value representing complete negativity.
    #[inline]
    pub const fn min_value() -> Self {
        Self(-1.0)
    }

    /// Creates a severity value representing neutrality.
    #[inline]
    pub const fn neutral() -> Self {
        Self(0.0)
    }

    /// Creates a severity value representing complete positivity.
    #[inline]
    pub const fn max_value() -> Self {
        Self(1.0)
    }

    /// Returns true if this severity is negative (< 0.0).
    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0.0
    }

    /// Returns true if this severity is positive (> 0.0).
    #[inline]
    pub fn is_positive(self) -> bool {
        self.0 > 0.0
    }

    /// Returns true if this severity is neutral (== 0.0).
    #[inline]
    pub fn is_neutral(self) -> bool {
        self.0 == 0.0
    }
}

// Default implementations
impl Default for Severity {
    fn default() -> Self {
        Self::neutral()
    }
}

// Generate arithmetic implementations for both types
impl_clamped_arithmetic!(Normalized, 0.0, 1.0);
impl_clamped_arithmetic!(Severity, -1.0, 1.0);
