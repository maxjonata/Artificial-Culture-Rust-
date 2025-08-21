//! Utility macros for reducing boilerplate in Bevy applications.
//!
//! This module provides minimal, useful macros that extend Bevy's native patterns
//! without overengineering. The focus is on ergonomic improvements that follow
//! Bevy's established conventions.

/// Generates clamped setter methods for f32 fields in structs.
///
/// This macro creates setter methods that automatically clamp values between
/// specified minimum and maximum bounds. This is essential for physiological
/// and psychological components where values must remain within scientifically
/// valid ranges (e.g., needs levels, emotional states, cognitive loads).
///
/// # Generated Methods
///
/// For each field, the macro generates two types of methods:
/// - `set_{field}(value: f32)` - Sets the field to a specific value, clamped to bounds
/// - `add_{field}(delta: f32)` - Adds a delta to the current value, then clamps the result
///
/// # Default Behavior
///
/// If no min/max values are specified, the default range is [0.0, 1.0] inclusive,
/// which is appropriate for normalized values like satisfaction levels, completion
/// percentages, and probability weights.
///
/// # Syntax Variations
///
/// - `field_name` - Uses default range [0.0, 1.0]
/// - `field_name, min, max` - Uses custom range [min, max]
///
/// # Examples
///
/// ```rust
/// use crate::clamped_setters;
///
/// #[derive(Component, Debug, Reflect, Default)]
/// #[reflect(Component)]
/// pub struct Needs {
///     pub hunger: f32,
///     pub thirst: f32,
///     pub energy: f32,
///     pub temperature: f32,
/// }
///
/// impl Needs {
///     clamped_setters! {
///         // Uses default [0.0, 1.0] range for normalized need levels
///         hunger,
///         thirst,
///         energy,
///         // Custom range for body temperature in Celsius
///         temperature, 35.0, 42.0
///     }
/// }
///
/// // Usage examples:
/// let mut needs = Needs::default();
///
/// // Setting absolute values (clamped to bounds)
/// needs.set_hunger(1.5);        // Clamped to 1.0
/// needs.set_temperature(30.0);  // Clamped to 35.0
///
/// // Adding incremental changes (result clamped to bounds)
/// needs.add_hunger(0.3);        // Adds 0.3, result clamped to [0.0, 1.0]
/// needs.add_temperature(-2.0);  // Subtracts 2.0, result clamped to [35.0, 42.0]
/// needs.add_energy(-0.5);       // Can use negative values to decrease
/// ```
///
/// # Scientific Justification
///
/// Value clamping is critical for maintaining biological realism in agent
/// simulations. Physiological parameters have natural bounds (e.g., blood
/// glucose levels, core body temperature) that when exceeded lead to death
/// or system failure. This macro ensures all f32 components respect these
/// physical constraints automatically.
///
/// The `add_` methods are particularly useful for modeling continuous processes
/// like metabolic decay, stress accumulation, or gradual recovery, where changes
/// occur incrementally over time but must remain within physiological limits.
#[macro_export]
macro_rules! clamped_setters {
    // Pattern for fields with custom min/max values
    ($($field:ident, $min:expr, $max:expr),* $(,)?) => {
        $(
            paste::paste! {
                #[doc = "Sets the " $field " value, clamping it between " $min " and " $max "."]
                #[doc = ""]
                #[doc = "This setter ensures the value remains within scientifically valid bounds."]
                #[doc = "Values outside the range are automatically clamped to the nearest boundary."]
                pub fn [<set_ $field>](&mut self, value: f32) {
                    self.$field = value.clamp($min, $max);
                }

                #[doc = "Adds the given delta to the " $field " value and clamps the result between " $min " and " $max "."]
                #[doc = ""]
                #[doc = "This method allows for incremental changes while maintaining scientific bounds."]
                #[doc = "The resulting value after addition is automatically clamped to the valid range."]
                pub fn [<add_ $field>](&mut self, delta: f32) {
                    self.$field = (self.$field + delta).clamp($min, $max);
                }
            }
        )*
    };

    // Pattern for fields with default [0.0, 1.0] range
    ($($field:ident),* $(,)?) => {
        $(
            paste::paste! {
                #[doc = "Sets the " $field " value, clamping it between 0.0 and 1.0."]
                #[doc = ""]
                #[doc = "This setter ensures the value remains within the default normalized range."]
                #[doc = "Values outside [0.0, 1.0] are automatically clamped to the nearest boundary."]
                pub fn [<set_ $field>](&mut self, value: f32) {
                    self.$field = value.clamp(0.0, 1.0);
                }

                #[doc = "Adds the given delta to the " $field " value and clamps the result between 0.0 and 1.0."]
                #[doc = ""]
                #[doc = "This method allows for incremental changes while maintaining the normalized range."]
                #[doc = "The resulting value after addition is automatically clamped to [0.0, 1.0]."]
                pub fn [<add_ $field>](&mut self, delta: f32) {
                    self.$field = (self.$field + delta).clamp(0.0, 1.0);
                }
            }
        )*
    };

    // Mixed pattern - allows combining default and custom ranges
    ($(default: $default_field:ident),* $(,)? $(custom: $custom_field:ident, $min:expr, $max:expr),* $(,)?) => {
        $(
            paste::paste! {
                #[doc = "Sets the " $default_field " value, clamping it between 0.0 and 1.0."]
                #[doc = ""]
                #[doc = "This setter ensures the value remains within the default normalized range."]
                pub fn [<set_ $default_field>](&mut self, value: f32) {
                    self.$default_field = value.clamp(0.0, 1.0);
                }

                #[doc = "Adds the given delta to the " $default_field " value and clamps the result between 0.0 and 1.0."]
                #[doc = ""]
                #[doc = "This method allows for incremental changes while maintaining the normalized range."]
                pub fn [<add_ $default_field>](&mut self, delta: f32) {
                    self.$default_field = (self.$default_field + delta).clamp(0.0, 1.0);
                }
            }
        )*
        $(
            paste::paste! {
                #[doc = "Sets the " $custom_field " value, clamping it between " $min " and " $max "."]
                #[doc = ""]
                #[doc = "This setter ensures the value remains within scientifically valid bounds."]
                pub fn [<set_ $custom_field>](&mut self, value: f32) {
                    self.$custom_field = value.clamp($min, $max);
                }

                #[doc = "Adds the given delta to the " $custom_field " value and clamps the result between " $min " and " $max "."]
                #[doc = ""]
                #[doc = "This method allows for incremental changes while maintaining scientific bounds."]
                pub fn [<add_ $custom_field>](&mut self, delta: f32) {
                    self.$custom_field = (self.$custom_field + delta).clamp($min, $max);
                }
            }
        )*
    };
}

/// Generates implementations of `RegisterTypesTuple` for tuples of different sizes.
///
/// This macro supports the ergonomic `app.register_types::<(TypeA, TypeB, TypeC)>()`
/// syntax, which follows the same pattern as Bevy's native `app.register_type::<T>()`
/// but allows registering multiple types in a single call.
///
/// # Usage Context
///
/// This macro is used internally by the `AppRegisterTypesExt` trait to support
/// bulk type registration while maintaining Bevy's established patterns.
#[macro_export]
macro_rules! impl_register_types_tuple {
    ($($ty:ident),*) => {
        impl<$($ty: bevy::reflect::Reflect + bevy::reflect::TypePath + 'static),*>
        crate::utils::helpers::RegisterTypesTuple for ($($ty,)*) {
            fn register(app: &mut bevy::prelude::App) {
                $(
                    app.register_type::<$ty>();
                )*
            }
        }
    };
}
