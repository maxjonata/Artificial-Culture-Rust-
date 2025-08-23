//! Utility macros for reducing boilerplate in Bevy applications.
//!
//! This module provides minimal, useful macros that extend Bevy's native patterns
//! without overengineering. The focus is on ergonomic improvements that follow
//! Bevy's established conventions.

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
        impl<$($ty: bevy::reflect::Reflect + bevy::reflect::TypePath + bevy::reflect::GetTypeRegistration + 'static),*>
        $crate::utils::helpers::overrides::RegisterTypesTuple for ($($ty,)*) {
            fn register(app: &mut bevy::prelude::App) {
                $(
                    app.register_type::<$ty>();
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_register_events_tuple {
    ($($ty:ident),*) => {
        impl<$($ty: bevy::prelude::Event + 'static),*>
        $crate::utils::helpers::overrides::RegisterEventsTuple for ($($ty,)*) {
            fn register(app: &mut bevy::prelude::App) {
                $(
                    app.add_event::<$ty>();
                )*
            }
        }
    }
}