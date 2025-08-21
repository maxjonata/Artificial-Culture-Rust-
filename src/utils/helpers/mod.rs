pub mod types;

use bevy::prelude::App;

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

// Import the macro from the macros module
use crate::impl_register_types_tuple;

// Generate impls for tuple sizes 1..=20 (covers most practical use cases)
impl_register_types_tuple!(A);
impl_register_types_tuple!(A, B);
impl_register_types_tuple!(A, B, C);
impl_register_types_tuple!(A, B, C, D);
impl_register_types_tuple!(A, B, C, D, E);
impl_register_types_tuple!(A, B, C, D, E, F);
impl_register_types_tuple!(A, B, C, D, E, F, G);
impl_register_types_tuple!(A, B, C, D, E, F, G, H);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_register_types_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
