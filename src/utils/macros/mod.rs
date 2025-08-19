//! Reusable macros for reducing boilerplate code in the Artificial Society simulation.
//!
//! This module contains utility macros that help reduce repetitive code patterns
//! throughout the simulation. These macros follow Rust best practices and
//! maintain type safety while improving code maintainability and readability.
//!
//! # Macro Categories
//!
//! - Component registration macros for Bevy plugins
//! - Event handling pattern macros
//! - System parameter validation macros
//! - Debug logging and instrumentation macros

/// Macro for registering multiple component types in a Bevy plugin.
///
/// This macro reduces boilerplate when registering many components and events
/// in plugin build methods, ensuring consistent registration patterns across
/// all domain plugins.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::register_components;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         register_components!(app,
///             MyComponent,
///             MyOtherComponent,
///             MyEvent
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! register_components {
    ($app:expr, $($component:ty),* $(,)?) => {
        $app$(
            .register_type::<$component>()
        )*
    };
}

/// Macro for adding multiple events to a Bevy application.
///
/// This macro streamlines the process of registering multiple event types
/// in plugin configurations, maintaining consistency across domain modules.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::add_events;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         add_events!(app,
///             NeedsChanged,
///             DesireTriggered,
///             ResourceConsumed
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! add_events {
    ($app:expr, $($event:ty),* $(,)?) => {
        $app$(
            .add_event::<$event>()
        )*
    };
}

/// Macro for adding multiple systems to a specific schedule.
///
/// This macro simplifies the process of registering multiple systems
/// to the same schedule, reducing verbosity in plugin configurations.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::add_systems_to_schedule;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         add_systems_to_schedule!(app, Update,
///             needs_decay_system,
///             desire_generation_system,
///             resource_interaction_system
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! add_systems_to_schedule {
    ($app:expr, $schedule:expr, $($system:expr),* $(,)?) => {
        $app.add_systems($schedule, (
            $($system,)*
        ))
    };
}

/// Macro for creating debug logging with component state information.
///
/// This macro provides consistent debug output formatting for agent
/// state inspection during development and testing phases.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::debug_agent_state;
///
/// fn my_system(query: Query<(Entity, &Needs, &Desires)>) {
///     for (entity, needs, desires) in query.iter() {
///         debug_agent_state!(entity, "needs" => needs, "desires" => desires);
///     }
/// }
/// ```
#[macro_export]
macro_rules! debug_agent_state {
    ($entity:expr, $($field:literal => $value:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        {
            bevy::log::debug!(
                "Agent {:?}: {}",
                $entity,
                format!($($field = "{{:?}}", )* $($value),*)
            );
        }
    };
}

/// Macro for validating system parameters and providing early returns.
///
/// This macro standardizes parameter validation in systems, ensuring
/// consistent error handling and preventing invalid state processing.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::validate_system_params;
///
/// fn my_system(
///     mut query: Query<&mut Needs>,
///     time: Res<Time>,
/// ) {
///     validate_system_params!(
///         query.is_empty() => return,
///         time.delta_seconds() <= 0.0 => return
///     );
///
///     // System logic continues here...
/// }
/// ```
#[macro_export]
macro_rules! validate_system_params {
    ($($condition:expr => $action:stmt),* $(,)?) => {
        $(
            if $condition {
                $action;
            }
        )*
    };
}

/// Macro for ML observation space data collection.
///
/// This macro standardizes the collection of quantifiable metrics
/// for machine learning integration, ensuring consistent data formats
/// across all observation systems.
///
/// # Examples
///
/// ```rust
/// use crate::utils::macros::collect_ml_observation;
///
/// fn observation_system(query: Query<(Entity, &Needs, &Position)>) {
///     for (entity, needs, position) in query.iter() {
///         collect_ml_observation!(entity,
///             "hunger" => needs.hunger,
///             "thirst" => needs.thirst,
///             "x_position" => position.x,
///             "y_position" => position.y
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! collect_ml_observation {
    ($entity:expr, $($metric:literal => $value:expr),* $(,)?) => {
        // ML-HOOK: Observation data collection point
        #[cfg(feature = "ml_integration")]
        {
            let observation_data = std::collections::HashMap::from([
                $(($metric.to_string(), $value as f32),)*
            ]);

            // Future ML integration point - send to observation buffer
            bevy::log::trace!("ML Observation for {:?}: {:?}", $entity, observation_data);
        }
    };
}
