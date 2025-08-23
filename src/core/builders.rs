//! Generic Type-Safe Builder System for ECS Components
//!
//! This module provides a foundational trait-based architecture that transforms the traditional
//! setter-based builder pattern into a comprehensive component foundation system. It embeds
//! common functionality directly into components while maintaining ECS compatibility and
//! performance characteristics suitable for real-time simulation.
//!
//! # Architecture Overview
//!
//! The system is built around three core traits:
//! - `ComponentCore`: Provides built-in event generation, validation, and telemetry
//! - `ComponentBuilder`: Type-safe builder pattern with automatic state management
//! - `ComponentBehavior`: Common behaviors like performance monitoring and diagnostics
//!
//! # Design Principles
//!
//! 1. **ECS Compatibility**: All traits work seamlessly with Bevy's Component system
//! 2. **Zero-Cost Abstractions**: Default implementations compile to optimal code
//! 3. **Domain Separation**: Components remain within their AI domains
//! 4. **Performance First**: Built-in monitoring with minimal overhead
//! 5. **Type Safety**: Compile-time guarantees for component construction

use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::presentation::performance_alerts::PerformanceAlert;

/// Core trait that all enhanced components must implement.
/// Provides built-in event generation, validation, and telemetry capabilities.
pub trait ComponentCore: Component + Debug + Clone + Send + Sync + 'static {
    /// The type of events this component can generate
    type EventType: Event + Clone + Debug;
    
    /// The type of validation errors this component can produce
    type ValidationError: Debug + Clone;
    
    /// Unique identifier for this component type (used for telemetry)
    const COMPONENT_ID: &'static str;
    
    /// Performance budget in microseconds for component operations
    const PERFORMANCE_BUDGET_US: u64 = 100; // 0.1ms default budget
    
    /// Validate the component's current state
    fn validate(&self) -> Result<(), Self::ValidationError> {
        Ok(()) // Default: always valid
    }
    
    /// Generate events based on component state changes
    fn generate_events(&self, previous_state: Option<&Self>) -> Vec<Self::EventType> {
        Vec::new() // Default: no events
    }
    
    /// Get telemetry data for performance monitoring
    fn get_telemetry(&self) -> ComponentTelemetry {
        ComponentTelemetry {
            component_id: Self::COMPONENT_ID,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            memory_size: std::mem::size_of::<Self>(),
            validation_status: self.validate().is_ok(),
            performance_budget_us: Self::PERFORMANCE_BUDGET_US,
        }
    }
    
    /// Check if component operation exceeds performance budget
    fn check_performance_budget(&self, operation_duration_us: u64) -> Option<PerformanceAlert> {
        if operation_duration_us > Self::PERFORMANCE_BUDGET_US {
            Some(PerformanceAlert::SlowSystemExecution {
                system_name: format!("{}_operation", Self::COMPONENT_ID),
                execution_time_ms: operation_duration_us as f32 / 1000.0,
                threshold_ms: Self::PERFORMANCE_BUDGET_US as f32 / 1000.0,
                frame_percentage: (operation_duration_us as f32 / 16670.0) * 100.0, // % of 60 FPS frame
            })
        } else {
            None
        }
    }
    
    /// Apply bounds checking and normalization
    fn normalize_values(&mut self) {
        // Default: no normalization needed
    }
    
    /// Get debug information for logging
    fn debug_info(&self) -> ComponentDebugInfo<Self::ValidationError> {
        ComponentDebugInfo {
            component_id: Self::COMPONENT_ID,
            state_summary: format!("{self:?}"),
            validation_result: self.validate(),
            telemetry: self.get_telemetry(),
        }
    }
}

/// Telemetry data structure for component monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTelemetry {
    pub component_id: &'static str,
    pub timestamp: u64,
    pub memory_size: usize,
    pub validation_status: bool,
    pub performance_budget_us: u64,
}

/// Debug information structure for component diagnostics
#[derive(Debug, Clone)]
pub struct ComponentDebugInfo<E: Debug + Clone> {
    pub component_id: &'static str,
    pub state_summary: String,
    pub validation_result: Result<(), E>,
    pub telemetry: ComponentTelemetry,
}

/// Common behaviors that components can implement for enhanced functionality
pub trait ComponentBehavior: ComponentCore {
    /// Called when component is first created
    fn on_create(&mut self, entity: Entity, world: &mut World) {
        self.normalize_values();
        if let Err(e) = self.validate() {
            warn!("Component {} created with invalid state: {:?}", Self::COMPONENT_ID, e);
        }
        self.emit_creation_event(entity, world);
    }
    
    /// Called when component state changes
    fn on_update(&mut self, previous_state: &Self, entity: Entity, world: &mut World) {
        let start_time = std::time::Instant::now();
        
        self.normalize_values();
        
        // Generate events for state changes
        let events = self.generate_events(Some(previous_state));
        for event in events {
            self.emit_event(event, entity, world);
        }
        
        // Check performance budget
        let duration_us = start_time.elapsed().as_micros() as u64;
        if let Some(alert) = self.check_performance_budget(duration_us) {
            self.emit_performance_alert(alert, world);
        }
        
        // Validate new state
        if let Err(e) = self.validate() {
            warn!("Component {} updated to invalid state: {:?}", Self::COMPONENT_ID, e);
        }
    }
    
    /// Called when component is removed
    fn on_destroy(&self, entity: Entity, world: &mut World) {
        self.emit_destruction_event(entity, world);
    }
    
    /// Emit a component-specific event
    fn emit_event(&self, event: Self::EventType, entity: Entity, world: &mut World) {
        if let Some(mut events) = world.get_resource_mut::<Events<Self::EventType>>() {
            events.send(event);
        }
    }
    
    /// Emit creation event
    fn emit_creation_event(&self, entity: Entity, world: &mut World) {
        debug!("Component {} created for entity {:?}", Self::COMPONENT_ID, entity);
    }
    
    /// Emit destruction event
    fn emit_destruction_event(&self, entity: Entity, world: &mut World) {
        debug!("Component {} destroyed for entity {:?}", Self::COMPONENT_ID, entity);
    }
    
    /// Emit performance alert
    fn emit_performance_alert(&self, alert: PerformanceAlert, world: &mut World) {
        if let Some(mut events) = world.get_resource_mut::<Events<PerformanceAlert>>() {
            events.send(alert);
        }
    }
}

/// Generic type-safe builder with state machine and automatic component enhancement
pub struct ComponentBuilder<T, S> 
where 
    T: ComponentCore,
    S: BuilderState,
{
    component: T,
    _state: PhantomData<S>,
    creation_timestamp: u64,
    validation_enabled: bool,
}

/// Marker trait for builder states
pub trait BuilderState: Send + Sync + 'static {}

/// Initial builder state
pub struct Initial;
impl BuilderState for Initial {}

/// Configured builder state
pub struct Configured;
impl BuilderState for Configured {}

/// Finalized builder state
pub struct Finalized;
impl BuilderState for Finalized {}

impl<T> ComponentBuilder<T, Initial>
where
    T: ComponentCore + Default,
{
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            component: T::default(),
            _state: PhantomData,
            creation_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            validation_enabled: true,
        }
    }
    
    /// Create builder with custom initial component
    pub fn with_component(component: T) -> Self {
        Self {
            component,
            _state: PhantomData,
            creation_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            validation_enabled: true,
        }
    }
}

impl<T> ComponentBuilder<T, Initial>
where
    T: ComponentCore,
{
    /// Enable or disable validation during building
    pub fn with_validation(mut self, enabled: bool) -> ComponentBuilder<T, Configured> {
        self.validation_enabled = enabled;
        ComponentBuilder {
            component: self.component,
            _state: PhantomData,
            creation_timestamp: self.creation_timestamp,
            validation_enabled: enabled,
        }
    }
    
    /// Apply a configuration function to the component
    pub fn configure<F>(mut self, config_fn: F) -> ComponentBuilder<T, Configured>
    where
        F: FnOnce(&mut T),
    {
        config_fn(&mut self.component);
        ComponentBuilder {
            component: self.component,
            _state: PhantomData,
            creation_timestamp: self.creation_timestamp,
            validation_enabled: self.validation_enabled,
        }
    }
}

impl<T> ComponentBuilder<T, Configured>
where
    T: ComponentCore,
{
    /// Apply additional configuration
    pub fn and_configure<F>(mut self, config_fn: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        config_fn(&mut self.component);
        self
    }
    
    /// Finalize the builder and prepare for component creation
    pub fn finalize(mut self) -> ComponentBuilder<T, Finalized> {
        // Apply normalization
        self.component.normalize_values();
        
        // Validate if enabled
        if self.validation_enabled {
            if let Err(e) = self.component.validate() {
                panic!("Component {} validation failed during build: {:?}", T::COMPONENT_ID, e);
            }
        }
        
        ComponentBuilder {
            component: self.component,
            _state: PhantomData,
            creation_timestamp: self.creation_timestamp,
            validation_enabled: self.validation_enabled,
        }
    }
}

impl<T> ComponentBuilder<T, Finalized>
where
    T: ComponentCore + ComponentBehavior,
{
    /// Build the final component with all enhancements
    pub fn build(self) -> T {
        info!("Built component {} in {}ms", 
              T::COMPONENT_ID, 
              SystemTime::now()
                  .duration_since(UNIX_EPOCH)
                  .unwrap_or_default()
                  .as_millis() as u64 - self.creation_timestamp);
        
        self.component
    }
    
    /// Build and spawn the component on an entity
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let component = self.build();
        let entity = commands.spawn(component).id();
        
        info!("Spawned entity {:?} with component {}", entity, T::COMPONENT_ID);
        entity
    }
    
    /// Build and add to existing entity
    pub fn add_to_entity(self, commands: &mut Commands, entity: Entity) {
        let component = self.build();
        commands.entity(entity).insert(component);
        
        info!("Added component {} to entity {:?}", T::COMPONENT_ID, entity);
    }
}

/// Extension trait for automatic component behavior integration
pub trait ComponentSystemExt {
    /// Register component with automatic behavior systems
    fn register_component_with_behaviors<T>(&mut self) -> &mut Self
    where
        T: ComponentCore + ComponentBehavior + Component + Reflect + GetTypeRegistration;

    /// Register multiple components with automatic behavior systems
    ///
    /// This provides bulk registration functionality embedded directly in the
    /// builder system following the component-private functionality principle.
    fn register_components_with_behaviors<T: ComponentTupleRegistration>(&mut self) -> &mut Self;
}

/// Trait for registering tuples of components with behaviors
pub trait ComponentTupleRegistration {
    fn register_all(app: &mut App);
}

impl ComponentSystemExt for App {
    fn register_component_with_behaviors<T>(&mut self) -> &mut Self
    where
        T: ComponentCore + ComponentBehavior + Component + Reflect + GetTypeRegistration,
    {
        self.register_type::<T>()
            .add_event::<T::EventType>()
            .add_systems(Update, (
                component_validation_system::<T>,
                component_telemetry_system::<T>,
                component_performance_monitoring_system::<T>,
            ))
    }

    fn register_components_with_behaviors<T: ComponentTupleRegistration>(&mut self) -> &mut Self {
        T::register_all(self);
        self
    }
}

/// System that validates all components of type T each frame
pub fn component_validation_system<T>(
    mut query: Query<(Entity, &T), Changed<T>>,
    mut commands: Commands,
) where
    T: ComponentCore + Component,
{
    for (entity, component) in query.iter_mut() {
        if let Err(e) = component.validate() {
            warn!("Entity {:?} has invalid component {}: {:?}", entity, T::COMPONENT_ID, e);

            // Optionally remove invalid components
            // commands.entity(entity).remove::<T>();
        }
    }
}

/// System that collects telemetry from all components of type T
pub fn component_telemetry_system<T>(
    query: Query<&T>,
    mut telemetry_events: EventWriter<ComponentTelemetryEvent>,
) where
    T: ComponentCore + Component,
{
    // Collect telemetry every 60 frames (1 second at 60 FPS)
    static mut FRAME_COUNTER: u32 = 0;
    unsafe {
        FRAME_COUNTER += 1;
        if FRAME_COUNTER % 60 != 0 {
            return;
        }
    }

    let component_count = query.iter().count();
    if component_count > 0 {
        let sample_telemetry = query.iter().next().unwrap().get_telemetry();

        telemetry_events.write(ComponentTelemetryEvent {
            component_id: T::COMPONENT_ID,
            instance_count: component_count,
            total_memory_bytes: component_count * sample_telemetry.memory_size,
            sample_telemetry,
        });
    }
}

/// System that monitors component performance and generates alerts
pub fn component_performance_monitoring_system<T>(
    query: Query<&T>,
    mut performance_alerts: EventWriter<PerformanceAlert>,
) where
    T: ComponentCore + Component,
{
    let component_count = query.iter().count();

    // Alert if too many components of this type exist
    const MAX_COMPONENTS_WARNING: usize = 10000;
    const MAX_COMPONENTS_CRITICAL: usize = 50000;

    if component_count > MAX_COMPONENTS_CRITICAL {
        performance_alerts.write(PerformanceAlert::EntityCountPerformanceImpact {
            entity_count: component_count as u32,
            baseline_entity_count: 1000,
            performance_degradation_percent: ((component_count as f32 / 1000.0) - 1.0) * 100.0,
            affected_metrics: vec![
                format!("{}_memory_usage", T::COMPONENT_ID),
                format!("{}_processing_time", T::COMPONENT_ID),
            ],
        });
    } else if component_count > MAX_COMPONENTS_WARNING {
        warn!("High component count for {}: {} instances", T::COMPONENT_ID, component_count);
    }
}

/// Event for component telemetry reporting
#[derive(Event, Debug, Clone)]
pub struct ComponentTelemetryEvent {
    pub component_id: &'static str,
    pub instance_count: usize,
    pub total_memory_bytes: usize,
    pub sample_telemetry: ComponentTelemetry,
}

/// Macro for implementing ComponentCore with sensible defaults
#[macro_export]
macro_rules! impl_component_core {
    ($component:ty, $event:ty, $error:ty, $id:expr) => {
        impl ComponentCore for $component {
            type EventType = $event;
            type ValidationError = $error;
            const COMPONENT_ID: &'static str = $id;
        }

        impl ComponentBehavior for $component {}
    };

    ($component:ty, $event:ty, $error:ty, $id:expr, $budget_us:expr) => {
        impl ComponentCore for $component {
            type EventType = $event;
            type ValidationError = $error;
            const COMPONENT_ID: &'static str = $id;
            const PERFORMANCE_BUDGET_US: u64 = $budget_us;
        }

        impl ComponentBehavior for $component {}
    };
}

/// Macro for creating type-safe builder methods
#[macro_export]
macro_rules! builder_method {
    ($method:ident, $field:ident, $type:ty) => {
        pub fn $method(mut self, value: $type) -> Self {
            self.component.$field = value;
            self
        }
    };

    ($method:ident, $field:ident, $type:ty, validate) => {
        pub fn $method(mut self, value: $type) -> Self {
            self.component.$field = value;
            if self.validation_enabled {
                if let Err(e) = self.component.validate() {
                    panic!("Validation failed after setting {}: {:?}", stringify!($field), e);
                }
            }
            self
        }
    };
}

/// Macro for registering multiple components with behaviors
///
/// This macro is embedded in the builder system following the component-private
/// functionality principle, providing bulk registration for enhanced components.
#[macro_export]
macro_rules! impl_component_tuple_registration {
    ($($ty:ident),*) => {
        impl<$($ty),*> ComponentTupleRegistration for ($($ty,)*)
        where
            $($ty: ComponentCore + ComponentBehavior + Component + Reflect + GetTypeRegistration,)*
        {
            fn register_all(app: &mut App) {
                $(
                    app.register_component_with_behaviors::<$ty>();
                )*
            }
        }
    };
}

/// Helper trait for components that need bounds checking
pub trait BoundsChecked {
    /// Apply bounds checking to all fields
    fn apply_bounds(&mut self);

    /// Check if all values are within acceptable bounds
    fn check_bounds(&self) -> bool;
}

/// Helper trait for components that need normalization
pub trait Normalizable {
    /// Normalize all values to their expected ranges
    fn normalize(&mut self);

    /// Check if all values are properly normalized
    fn is_normalized(&self) -> bool;
}

/// Helper implementation for components that implement BoundsChecked + Normalizable
/// Note: Components must still manually implement ComponentCore, but can use these helpers
pub trait AutoComponentCore: BoundsChecked + Normalizable {
    fn auto_normalize_values(&mut self) {
        self.apply_bounds();
        self.normalize();
    }

    fn auto_validate(&self) -> Result<(), String> {
        if !self.check_bounds() {
            return Err("Values out of bounds".to_string());
        }
        if !self.is_normalized() {
            return Err("Values not normalized".to_string());
        }
        Ok(())
    }
}

// Blanket implementation for all types that implement the required traits
impl<T> AutoComponentCore for T where T: BoundsChecked + Normalizable {}

/// Generic event type for components that don't need specific events
#[derive(Event, Debug, Clone)]
pub enum GenericComponentEvent {
    Created(Entity),
    Updated(Entity),
    Destroyed(Entity),
    ValidationFailed(Entity, String),
}

/// Plugin that registers the component builder system
pub struct ComponentBuilderPlugin;

impl Plugin for ComponentBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ComponentTelemetryEvent>()
           .add_event::<GenericComponentEvent>()
           .add_systems(Update, (
               telemetry_reporting_system,
               component_debug_system,
           ));
    }
}

/// System that reports telemetry events to logs
fn telemetry_reporting_system(
    mut telemetry_events: EventReader<ComponentTelemetryEvent>,
) {
    for event in telemetry_events.read() {
        debug!("Component telemetry - {}: {} instances, {}KB total memory",
               event.component_id,
               event.instance_count,
               event.total_memory_bytes / 1024);
    }
}

/// System that provides debug information for components
fn component_debug_system(
    mut generic_events: EventReader<GenericComponentEvent>,
) {
    for event in generic_events.read() {
        match event {
            GenericComponentEvent::ValidationFailed(entity, error) => {
                error!("Component validation failed for entity {:?}: {}", entity, error);
            },
            _ => {
                trace!("Component event: {:?}", event);
            }
        }
    }
}
