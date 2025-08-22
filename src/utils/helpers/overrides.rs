use bevy::prelude::{App, Reflect, Resource};
use bevy::reflect::GetTypeRegistration;

/// Trait implemented for tuples of types that should be registered for reflection.
pub trait RegisterTypesTuple {
    fn register(app: &mut App);
}

/// Extension trait adding `register_types` to `App`.
///
/// This follows Bevy's native pattern where `app.register_type::<T>()` registers
/// a single type, but extends it to allow `app.register_types::<(A, B, C)>()`
/// for bulk registration while maintaining the same ergonomic API style.
pub trait AppRegisterTypesExt {
    /// Register all types inside the provided tuple type parameter.
    ///
    /// Usage:
    /// ```rust
    /// app.register_types::<(Needs, MetabolicConstants, DesireState)>();
    /// ```
    fn register_types<T: RegisterTypesTuple>(&mut self) -> &mut Self;
}

impl AppRegisterTypesExt for App {
    #[inline]
    fn register_types<T: RegisterTypesTuple>(&mut self) -> &mut Self {
        T::register(self);
        self
    }
}