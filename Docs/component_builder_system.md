# Generic Type-Safe Builder System for ECS Components

## Overview

The Generic Type-Safe Builder System transforms the traditional setter-based builder pattern into a comprehensive component foundation system. It embeds common functionality directly into components while maintaining ECS compatibility and performance characteristics suitable for real-time simulation.

## Architecture

### Core Traits

#### 1. `ComponentCore`
The foundational trait that all enhanced components must implement:

```rust
pub trait ComponentCore: Component + Debug + Clone + Send + Sync + 'static {
    type EventType: Event + Clone + Debug;
    type ValidationError: Debug + Clone;
    const COMPONENT_ID: &'static str;
    const PERFORMANCE_BUDGET_US: u64 = 100; // 0.1ms default budget
    
    // Built-in functionality
    fn validate(&self) -> Result<(), Self::ValidationError>;
    fn generate_events(&self, previous_state: Option<&Self>) -> Vec<Self::EventType>;
    fn get_telemetry(&self) -> ComponentTelemetry;
    fn check_performance_budget(&self, operation_duration_us: u64) -> Option<PerformanceAlert>;
    fn normalize_values(&mut self);
}
```

#### 2. `ComponentBehavior`
Provides automatic lifecycle management:

```rust
pub trait ComponentBehavior: ComponentCore {
    fn on_create(&mut self, entity: Entity, world: &mut World);
    fn on_update(&mut self, previous_state: &Self, entity: Entity, world: &mut World);
    fn on_destroy(&self, entity: Entity, world: &mut World);
    // Event emission helpers
}
```

#### 3. `BoundsChecked` & `Normalizable`
Helper traits for automatic value management:

```rust
pub trait BoundsChecked {
    fn apply_bounds(&mut self);
    fn check_bounds(&self) -> bool;
}

pub trait Normalizable {
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}
```

### Type-Safe Builder

The builder uses a state machine to ensure correct construction:

```rust
ComponentBuilder<T, Initial>    // Can configure
    ↓
ComponentBuilder<T, Configured> // Can add more config or finalize
    ↓
ComponentBuilder<T, Finalized>  // Can build
```

## Quick Start

### 1. Define Your Component

```rust
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct PersonalityComponent {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

#[derive(Event, Debug, Clone)]
pub enum PersonalityEvent {
    TraitChanged { entity: Entity, trait_name: String, old_value: f32, new_value: f32 },
    ExtremeValueDetected { entity: Entity, trait_name: String, value: f32 },
}

#[derive(Debug, Clone)]
pub enum PersonalityError {
    TraitOutOfBounds { trait_name: String, value: f32 },
}
```

### 2. Implement Core Traits

Use the convenience macro:

```rust
impl_component_core!(PersonalityComponent, PersonalityEvent, PersonalityError, "personality", 50);
```

Or implement manually for custom behavior:

```rust
impl ComponentCore for PersonalityComponent {
    type EventType = PersonalityEvent;
    type ValidationError = PersonalityError;
    const COMPONENT_ID: &'static str = "personality";
    const PERFORMANCE_BUDGET_US: u64 = 50;
    
    fn validate(&self) -> Result<(), Self::ValidationError> {
        if !self.check_bounds() {
            return Err(PersonalityError::TraitOutOfBounds { 
                trait_name: "multiple".to_string(), 
                value: -1.0 
            });
        }
        Ok(())
    }
    
    fn generate_events(&self, previous_state: Option<&Self>) -> Vec<Self::EventType> {
        // Custom event generation logic
        Vec::new()
    }
}

impl ComponentBehavior for PersonalityComponent {}
```

### 3. Implement Helper Traits (Optional)

```rust
impl BoundsChecked for PersonalityComponent {
    fn apply_bounds(&mut self) {
        self.openness = self.openness.clamp(0.0, 1.0);
        self.conscientiousness = self.conscientiousness.clamp(0.0, 1.0);
        // ... other traits
    }
    
    fn check_bounds(&self) -> bool {
        [self.openness, self.conscientiousness, /* ... */]
            .iter()
            .all(|&x| x >= 0.0 && x <= 1.0)
    }
}

impl Normalizable for PersonalityComponent {
    fn normalize(&mut self) {
        // Custom normalization logic
        let sum = self.openness + self.conscientiousness + /* ... */;
        if sum > 0.0 {
            let factor = 2.5 / sum; // Target average of 0.5 per trait
            if factor < 0.8 || factor > 1.2 {
                self.openness *= factor;
                // ... normalize other traits
                self.apply_bounds();
            }
        }
    }
    
    fn is_normalized(&self) -> bool {
        let sum = self.openness + self.conscientiousness + /* ... */;
        sum >= 2.0 && sum <= 3.0
    }
}
```

### 4. Create Builder Extension Methods

```rust
impl ComponentBuilder<PersonalityComponent, Initial> {
    pub fn with_openness(self, value: f32) -> ComponentBuilder<PersonalityComponent, Configured> {
        self.configure(|comp| comp.openness = value)
    }
    
    pub fn random(self) -> ComponentBuilder<PersonalityComponent, Configured> {
        self.configure(|comp| {
            let mut rng = rand::thread_rng();
            comp.openness = rng.gen_range(0.0..1.0);
            // ... set other traits randomly
        })
    }
}

impl ComponentBuilder<PersonalityComponent, Configured> {
    pub fn with_conscientiousness(self, value: f32) -> Self {
        self.and_configure(|comp| comp.conscientiousness = value)
    }
}
```

### 5. Register Component with App

```rust
use artificial_culture_rust::core::builders::ComponentSystemExt;

app.register_component_with_behaviors::<PersonalityComponent>();
```

### 6. Use the Builder

```rust
// Create component with builder
let personality = ComponentBuilder::<PersonalityComponent, _>::new()
    .with_validation(true)
    .random()
    .with_openness(0.8)
    .finalize()
    .build();

// Spawn entity with component
let entity = ComponentBuilder::<PersonalityComponent, _>::new()
    .random()
    .finalize()
    .spawn(&mut commands);

// Add to existing entity
ComponentBuilder::<PersonalityComponent, _>::new()
    .with_openness(0.7)
    .with_conscientiousness(0.6)
    .finalize()
    .add_to_entity(&mut commands, entity);
```

## Built-in Features

### Automatic Performance Monitoring
- Components track execution time against performance budgets
- Automatic alerts when operations exceed thresholds
- Integration with the performance monitoring system

### Event Generation
- Components automatically generate events on state changes
- Configurable event types per component
- Automatic lifecycle events (creation, destruction)

### Validation & Normalization
- Built-in validation with custom error types
- Automatic bounds checking and value normalization
- Validation can be enabled/disabled during building

### Telemetry & Debugging
- Automatic telemetry collection (memory usage, instance counts)
- Debug information generation
- Performance impact monitoring

### ECS Integration
- Seamless integration with Bevy's ECS system
- Automatic system registration for component behaviors
- Type-safe builder pattern with compile-time guarantees

## Advanced Usage

### Custom Validation Logic

```rust
impl ComponentCore for MyComponent {
    fn validate(&self) -> Result<(), Self::ValidationError> {
        if self.value < 0.0 {
            return Err(MyError::NegativeValue(self.value));
        }
        if self.name.is_empty() {
            return Err(MyError::EmptyName);
        }
        Ok(())
    }
}
```

### Complex Event Generation

```rust
impl ComponentCore for MyComponent {
    fn generate_events(&self, previous_state: Option<&Self>) -> Vec<Self::EventType> {
        let mut events = Vec::new();
        
        if let Some(prev) = previous_state {
            if (self.value - prev.value).abs() > 0.1 {
                events.push(MyEvent::ValueChanged {
                    old: prev.value,
                    new: self.value,
                });
            }
        }
        
        if self.value > 0.9 {
            events.push(MyEvent::HighValue { value: self.value });
        }
        
        events
    }
}
```

### Performance Budget Customization

```rust
impl ComponentCore for MyComponent {
    const PERFORMANCE_BUDGET_US: u64 = 200; // 0.2ms budget for complex components
    
    // Custom performance checking
    fn check_performance_budget(&self, duration_us: u64) -> Option<PerformanceAlert> {
        if duration_us > Self::PERFORMANCE_BUDGET_US {
            Some(PerformanceAlert::SlowSystemExecution {
                system_name: format!("{}_custom_operation", Self::COMPONENT_ID),
                execution_time_ms: duration_us as f32 / 1000.0,
                threshold_ms: Self::PERFORMANCE_BUDGET_US as f32 / 1000.0,
                frame_percentage: (duration_us as f32 / 16670.0) * 100.0,
            })
        } else {
            None
        }
    }
}
```

## Testing

The system includes comprehensive tests demonstrating all features:

```bash
cargo test tests::core::builders --lib
```

Tests cover:
- Component validation and normalization
- Event generation
- Performance budget checking
- Builder type safety
- ECS integration
- Telemetry collection

## Performance Characteristics

- **Zero-cost abstractions**: Default implementations compile to optimal code
- **Minimal overhead**: <1% frame time impact for monitoring
- **Memory efficient**: Compact data structures with configurable budgets
- **Cache-friendly**: Data-oriented design principles

## Migration Guide

### From Setter-Based Builders

**Before:**
```rust
let component = MyComponentBuilder::new()
    .set_value(0.5)
    .set_name("test")
    .build();
```

**After:**
```rust
let component = ComponentBuilder::<MyComponent, _>::new()
    .with_value(0.5)
    .with_name("test".to_string())
    .finalize()
    .build();
```

### From Manual Component Creation

**Before:**
```rust
let mut component = MyComponent::default();
component.value = 0.5;
component.name = "test".to_string();
// Manual validation
if component.value < 0.0 { panic!("Invalid value"); }
```

**After:**
```rust
let component = ComponentBuilder::<MyComponent, _>::new()
    .with_validation(true)
    .with_value(0.5)
    .with_name("test".to_string())
    .finalize() // Automatic validation and normalization
    .build();
```

## Best Practices

1. **Use validation during development**: Enable validation in debug builds
2. **Set appropriate performance budgets**: Based on component complexity
3. **Implement meaningful events**: For state changes that matter to gameplay
4. **Use bounds checking**: For components with constrained value ranges
5. **Test component behavior**: Use the provided test utilities
6. **Monitor telemetry**: Watch for performance impacts in production

The Generic Type-Safe Builder System provides a robust foundation for creating maintainable, performant, and feature-rich ECS components while preserving the flexibility and performance characteristics that make Bevy's ECS system powerful.
