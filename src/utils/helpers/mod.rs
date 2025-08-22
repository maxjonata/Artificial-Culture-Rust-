pub mod types;
pub mod overrides;

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
