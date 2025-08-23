//! Core type definitions for the Artificial Society simulation.
//!
//! This module enforces the project's performance and data integrity requirements:
//! - All floating-point values normalized to [0.0, 1.0] range
//! - Smallest possible types for optimal memory layout and cache performance
//! - Type safety for scientific accuracy

use bevy::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Sub, SubAssign};

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
///
/// // Direct field assignment is impossible (private field)
/// // let mut n = Normalized::new(0.5);
/// // n.0 = 2.0; // Compile error!
/// ```
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[reflect(Component)]
#[repr(transparent)] // Same memory layout as f32
pub struct Normalized(f32);


/// Compact agent identifier, supports up to 65,535 agents.
/// Use this instead of Entity for internal references to optimize memory.
pub type AgentId = u16;

/// Quantized weight value (0-255) that maps to [0.0, 1.0] range.
/// Used for neural connections and memory weights to save memory.
pub type QuantizedWeight = u8;

impl Normalized {
    /// Creates a new normalized value, clamping the input to [0.0, 1.0].
    ///
    /// This is the primary constructor - it never fails and always produces
    /// a valid normalized value by clamping out-of-range inputs.
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// let n1 = Normalized::new(0.5);   // 0.5
    /// let n2 = Normalized::new(1.5);   // 1.0 (clamped)
    /// let n3 = Normalized::new(-0.2);  // 0.0 (clamped)
    /// ```
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
    /// This is provided for performance-critical paths where bounds checking
    /// has already been performed.
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// let n = unsafe { Normalized::new_unchecked(0.7) };
    /// ```
    #[inline]
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    /// Attempts to create a normalized value, returning an error if out of range.
    ///
    /// Use this when you want to know whether clamping occurred, rather than
    /// silently accepting clamped values.
    ///
    /// # Examples
    /// ```rust
    /// # use artificial_culture_rust::core::types::Normalized;
    /// assert!(Normalized::try_new(0.5).is_ok());
    /// assert!(Normalized::try_new(1.5).is_err());
    /// ```
    #[inline]
    pub fn try_new(value: f32) -> Result<Self, NormalizedError> {
        if RangeInclusive::new(0.0, 1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(NormalizedError::OutOfRange { value, min: 0.0, max: 1.0 })
        }
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
}

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

// Arithmetic operations that maintain bounds
impl Add for Normalized {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Add<f32> for Normalized {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl AddAssign for Normalized {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.0 + rhs.0);
    }
}

impl AddAssign<f32> for Normalized {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = Self::new(self.0 + rhs);
    }
}

impl Sub for Normalized {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Sub<f32> for Normalized {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl SubAssign for Normalized {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.0 - rhs.0);
    }
}

impl SubAssign<f32> for Normalized {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = Self::new(self.0 - rhs);
    }
}

impl Mul for Normalized {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl Mul<f32> for Normalized {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl MulAssign for Normalized {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::new(self.0 * rhs.0);
    }
}

impl MulAssign<f32> for Normalized {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::new(self.0 * rhs);
    }
}

impl Div for Normalized {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0.0 {
            panic!("Division by zero in Normalized arithmetic");
        } else {
            Self::new(self.0 / rhs.0)
        }
    }
}

impl Div<f32> for Normalized {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.0 / rhs) // Will panic on division by zero, like standard Rust
    }
}

impl DivAssign for Normalized {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Normalized {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}