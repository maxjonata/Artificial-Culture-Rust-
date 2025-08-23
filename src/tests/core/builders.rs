//! Tests for the generic type-safe builder system.
//!
//! This module provides comprehensive tests for the component builder architecture,
//! including trait implementations, type safety, validation, and ECS integration.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::builders::{
    ComponentCore, ComponentBehavior, ComponentBuilder, ComponentTelemetryEvent,
    BoundsChecked, Normalizable, AutoComponentCore, GenericComponentEvent,
    ComponentBuilderPlugin, Initial, Configured, BuilderState
};

/// Test component for builder system validation
#[derive(Component, Debug, Clone, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TestComponent {
    pub value: f32,
    pub name: String,
    pub active: bool,
}

/// Test events for TestComponent
#[derive(Event, Debug, Clone)]
pub enum TestEvent {
    ValueChanged { old: f32, new: f32 },
    Activated,
    Deactivated,
}

/// Test validation errors
#[derive(Debug, Clone)]
pub enum TestError {
    ValueOutOfRange(f32),
    InvalidName,
}

// Implement ComponentCore for TestComponent
impl ComponentCore for TestComponent {
    type EventType = TestEvent;
    type ValidationError = TestError;
    const COMPONENT_ID: &'static str = "test_component";
    const PERFORMANCE_BUDGET_US: u64 = 100;
    
    fn validate(&self) -> Result<(), Self::ValidationError> {
        if self.value < 0.0 || self.value > 1.0 {
            return Err(TestError::ValueOutOfRange(self.value));
        }
        if self.name.is_empty() {
            return Err(TestError::InvalidName);
        }
        Ok(())
    }
    
    fn generate_events(&self, previous_state: Option<&Self>) -> Vec<Self::EventType> {
        let mut events = Vec::new();
        
        if let Some(prev) = previous_state {
            if (self.value - prev.value).abs() > 0.1 {
                events.push(TestEvent::ValueChanged {
                    old: prev.value,
                    new: self.value,
                });
            }
            
            if self.active && !prev.active {
                events.push(TestEvent::Activated);
            } else if !self.active && prev.active {
                events.push(TestEvent::Deactivated);
            }
        }
        
        events
    }
    
    fn normalize_values(&mut self) {
        self.value = self.value.clamp(0.0, 1.0);
        if self.name.is_empty() {
            self.name = "default".to_string();
        }
    }
}

impl ComponentBehavior for TestComponent {}

impl BoundsChecked for TestComponent {
    fn apply_bounds(&mut self) {
        self.value = self.value.clamp(0.0, 1.0);
    }
    
    fn check_bounds(&self) -> bool {
        self.value >= 0.0 && self.value <= 1.0
    }
}

impl Normalizable for TestComponent {
    fn normalize(&mut self) {
        if self.name.is_empty() {
            self.name = "normalized".to_string();
        }
    }
    
    fn is_normalized(&self) -> bool {
        !self.name.is_empty()
    }
}

/// Extension methods for TestComponent builder on Initial state
impl ComponentBuilder<TestComponent, Initial> {
    /// Set value with validation
    pub fn with_value(self, value: f32) -> ComponentBuilder<TestComponent, Configured> {
        self.configure(|comp| comp.value = value)
    }

    /// Set name
    pub fn with_name(self, name: String) -> ComponentBuilder<TestComponent, Configured> {
        self.configure(|comp| comp.name = name)
    }

    /// Set active state
    pub fn with_active(self, active: bool) -> ComponentBuilder<TestComponent, Configured> {
        self.configure(|comp| comp.active = active)
    }

    /// Custom builder method for testing
    pub fn with_defaults(self) -> ComponentBuilder<TestComponent, Configured> {
        self.configure(|comp| {
            comp.value = 0.5;
            comp.name = "test".to_string();
            comp.active = true;
        })
    }
}

/// Extension methods for TestComponent builder on Configured state
impl ComponentBuilder<TestComponent, Configured> {
    /// Set value with validation
    pub fn with_value(self, value: f32) -> Self {
        self.and_configure(|comp| comp.value = value)
    }

    /// Set name
    pub fn with_name(self, name: String) -> Self {
        self.and_configure(|comp| comp.name = name)
    }

    /// Set active state
    pub fn with_active(self, active: bool) -> Self {
        self.and_configure(|comp| comp.active = active)
    }
}

/// Helper to create test app with builder system
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(ComponentBuilderPlugin)
       .add_event::<TestEvent>()
       .register_type::<TestComponent>();
    app
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_core_validation() {
        let valid_component = TestComponent {
            value: 0.5,
            name: "test".to_string(),
            active: true,
        };
        
        assert!(valid_component.validate().is_ok());
        
        let invalid_value = TestComponent {
            value: 1.5, // Out of range
            name: "test".to_string(),
            active: true,
        };
        
        assert!(invalid_value.validate().is_err());
        
        let invalid_name = TestComponent {
            value: 0.5,
            name: "".to_string(), // Empty name
            active: true,
        };
        
        assert!(invalid_name.validate().is_err());
    }

    #[test]
    fn test_component_normalization() {
        let mut component = TestComponent {
            value: 1.5, // Out of bounds
            name: "".to_string(), // Empty
            active: true,
        };
        
        component.normalize_values();
        
        assert_eq!(component.value, 1.0); // Clamped to bounds
        assert_eq!(component.name, "default"); // Normalized
        assert!(component.validate().is_ok());
    }

    #[test]
    fn test_event_generation() {
        let old_component = TestComponent {
            value: 0.3,
            name: "test".to_string(),
            active: false,
        };
        
        let new_component = TestComponent {
            value: 0.8, // Significant change
            name: "test".to_string(),
            active: true, // State change
        };
        
        let events = new_component.generate_events(Some(&old_component));
        
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], TestEvent::ValueChanged { .. }));
        assert!(matches!(events[1], TestEvent::Activated));
    }

    #[test]
    fn test_component_telemetry() {
        let component = TestComponent::default();
        let telemetry = component.get_telemetry();
        
        assert_eq!(telemetry.component_id, "test_component");
        assert!(telemetry.timestamp > 0);
        assert!(telemetry.memory_size > 0);
        assert_eq!(telemetry.performance_budget_us, 100);
    }

    #[test]
    fn test_performance_budget_checking() {
        let component = TestComponent::default();
        
        // Within budget
        let no_alert = component.check_performance_budget(50);
        assert!(no_alert.is_none());
        
        // Exceeds budget
        let alert = component.check_performance_budget(200);
        assert!(alert.is_some());
        
        if let Some(alert) = alert {
            match alert {
                crate::presentation::performance_alerts::PerformanceAlert::SlowSystemExecution { 
                    system_name, execution_time_ms, threshold_ms, .. 
                } => {
                    assert_eq!(system_name, "test_component_operation");
                    assert_eq!(execution_time_ms, 0.2);
                    assert_eq!(threshold_ms, 0.1);
                },
                _ => panic!("Expected SlowSystemExecution alert"),
            }
        }
    }

    #[test]
    fn test_bounds_checked_trait() {
        let mut component = TestComponent {
            value: 1.5,
            name: "test".to_string(),
            active: true,
        };
        
        assert!(!component.check_bounds());
        
        component.apply_bounds();
        
        assert!(component.check_bounds());
        assert_eq!(component.value, 1.0);
    }

    #[test]
    fn test_normalizable_trait() {
        let mut component = TestComponent {
            value: 0.5,
            name: "".to_string(),
            active: true,
        };
        
        assert!(!component.is_normalized());
        
        component.normalize();
        
        assert!(component.is_normalized());
        assert_eq!(component.name, "normalized");
    }

    #[test]
    fn test_auto_component_core_helpers() {
        let mut component = TestComponent {
            value: 1.5,
            name: "".to_string(),
            active: true,
        };
        
        // Test auto normalization
        component.auto_normalize_values();
        assert_eq!(component.value, 1.0);
        assert_eq!(component.name, "normalized");
        
        // Test auto validation
        let result = component.auto_validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_type_safety() {
        // Test that builder enforces correct state transitions
        let builder = ComponentBuilder::<TestComponent, Initial>::new();
        
        // Can configure from Initial
        let configured = builder.with_validation(true);
        
        // Can finalize from Configured
        let finalized = configured.finalize();
        
        // Can build from Finalized
        let component = finalized.build();
        
        assert!(component.validate().is_ok());
    }

    #[test]
    fn test_builder_validation_enforcement() {
        // Test that validation works with valid values
        let component = ComponentBuilder::<TestComponent, Initial>::new()
            .with_validation(true)
            .with_value(0.5)
            .with_name("test".to_string())
            .finalize()
            .build();

        assert!(component.validate().is_ok());
    }

    #[test]
    fn test_builder_custom_methods() {
        let component = ComponentBuilder::<TestComponent, Initial>::new()
            .with_defaults()
            .with_value(0.8)
            .with_name("custom".to_string())
            .with_active(false)
            .finalize()
            .build();
        
        assert_eq!(component.value, 0.8);
        assert_eq!(component.name, "custom");
        assert!(!component.active);
    }

    #[test]
    fn test_builder_normalization() {
        let component = ComponentBuilder::<TestComponent, Initial>::new()
            .configure(|comp| {
                comp.value = 1.5; // Out of bounds
                comp.name = "".to_string(); // Empty
            })
            .finalize() // Should normalize during finalization
            .build();
        
        assert_eq!(component.value, 1.0); // Normalized
        assert_eq!(component.name, "default"); // Normalized
        assert!(component.validate().is_ok());
    }

    #[test]
    fn test_component_builder_plugin() {
        let mut app = create_test_app();
        
        // Verify plugin is loaded
        assert!(app.world().contains_resource::<Events<GenericComponentEvent>>());
        assert!(app.world().contains_resource::<Events<ComponentTelemetryEvent>>());
        
        // Run one update to ensure systems don't panic
        app.update();
    }
}
