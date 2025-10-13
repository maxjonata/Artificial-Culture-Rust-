---
inclusion: fileMatch
fileMatchPattern: "*.rs"
---

# Rust and Bevy Development Patterns for Artificial Society

## Core Type System Patterns

### Use Normalized<f32> for All Social/Emotional Values
```rust
use crate::core::types::Normalized;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Needs {
    pub hunger: Normalized<f32>,    // Always 0.0-1.0
    pub energy: Normalized<f32>,    // Always 0.0-1.0
    pub safety: Normalized<f32>,    // Always 0.0-1.0
    pub social: Normalized<f32>,    // Always 0.0-1.0
}
```

### Component Registration Pattern
```rust
impl Plugin for PhysiologyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Needs>()
           .register_type::<StressSystem>()
           .register_type::<EmotionalState>()
           .add_event::<NeedCritical>()
           .add_event::<StressThresholdCrossed>()
           .add_systems(Update, (
               needs_decay_system,
               stress_response_system,
               mood_calculation_system,
           ).chain());
    }
}
```

## System Design Patterns

### Event-Driven System Pattern
```rust
pub fn stress_response_system(
    mut stress_query: Query<&mut StressSystem>,
    mut need_events: EventReader<NeedCritical>,
    mut stress_events: EventWriter<StressThresholdCrossed>,
) {
    for event in need_events.read() {
        if let Ok(mut stress) = stress_query.get_mut(event.entity) {
            stress.acute_stress = (stress.acute_stress + event.severity).clamp(0.0, 1.0);
            
            if stress.acute_stress > 0.7 && stress.state == StressState::Homeostasis {
                stress.state = StressState::Allostasis;
                stress_events.send(StressThresholdCrossed {
                    entity: event.entity,
                    new_state: StressState::Allostasis,
                });
            }
        }
    }
}
```

### Personality Modulation Pattern
```rust
pub fn calculate_with_personality(
    base_value: f32,
    personality: &Personality,
    trait_influence: f32,
) -> f32 {
    let modifier = 1.0 + (personality.neuroticism - 0.5) * trait_influence;
    (base_value * modifier).clamp(0.0, 1.0)
}

// Usage in systems
let stress_modifier = calculate_with_personality(
    base_stress,
    &personality,
    0.4, // neuroticism influence strength
);
```

## Component Design Patterns

### Reflectable Components for Debug UI
```rust
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct EmotionalState {
    pub valence: f32,        // -1.0 to 1.0 (pleasure/displeasure)
    pub arousal: f32,        // -1.0 to 1.0 (activation/calm)
    pub dominance: f32,      // -1.0 to 1.0 (control/submission)
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct ApparentStateVector {
    pub tension_relaxation: f32,    // -1.0 to 1.0 (tense/relaxed)
    pub openness_closure: f32,      // -1.0 to 1.0 (open/closed)
    pub dominance_submission: f32,  // -1.0 to 1.0 (dominant/submissive)
    pub focus_distraction: f32,     // -1.0 to 1.0 (focused/distracted)
}
```

### Event Structures
```rust
#[derive(Event, Debug)]
pub struct NeedCritical {
    pub entity: Entity,
    pub need_type: NeedType,
    pub severity: f32,
    pub timestamp: f32,
}

#[derive(Event, Debug)]
pub struct MoodChange {
    pub entity: Entity,
    pub old_valence: f32,
    pub new_valence: f32,
    pub change_magnitude: f32,
}
```

## Performance Optimization Patterns

### System Scheduling for Performance
```rust
impl Plugin for PhysiologyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // High frequency systems (every frame)
            social_expression_system,
            
            // Medium frequency systems (every few seconds)
            needs_decay_system.run_if(on_timer(Duration::from_secs(5))),
            
            // Low frequency systems (every 30 seconds)
            isolation_detection_system.run_if(on_timer(Duration::from_secs(30))),
        ));
    }
}
```

### Efficient Queries with Filters
```rust
pub fn emotional_contagion_system(
    mut agents: Query<(&mut EmotionalState, &Transform, &Personality)>,
    nearby_agents: Query<(&EmotionalState, &Transform), (With<Agent>, Without<Player>)>,
    spatial_query: Res<RapierContext>,
) {
    for (mut emotion, transform, personality) in agents.iter_mut() {
        // Use spatial queries for efficient neighbor finding
        spatial_query.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            &Collider::ball(CONTAGION_RADIUS),
            QueryFilter::default(),
            |entity| {
                // Process emotional influence
                true // Continue iteration
            },
        );
    }
}
```

## Error Handling Patterns

### Graceful Component Access
```rust
pub fn social_interaction_system(
    mut commands: Commands,
    agents: Query<(Entity, &Transform, &Personality, Option<&SocialMemory>)>,
) {
    for (entity, transform, personality, memory) in agents.iter() {
        // Handle optional components gracefully
        let social_history = memory.map(|m| &m.interactions).unwrap_or(&Vec::new());
        
        // Safe component access with error handling
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(SocialMemory::default());
        }
    }
}
```

### Validation Patterns
```rust
impl Needs {
    pub fn validate(&self) -> Result<(), String> {
        if self.hunger.value() > 1.0 || self.hunger.value() < 0.0 {
            return Err(format!("Invalid hunger value: {}", self.hunger.value()));
        }
        Ok(())
    }
    
    pub fn get_most_urgent(&self) -> (NeedType, f32) {
        let needs = [
            (NeedType::Hunger, self.hunger.value()),
            (NeedType::Energy, self.energy.value()),
            (NeedType::Safety, self.safety.value()),
            (NeedType::Social, self.social.value()),
        ];
        
        needs.iter()
             .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
             .map(|(need_type, value)| (*need_type, *value))
             .unwrap_or((NeedType::Hunger, 0.0))
    }
}
```

## Testing Patterns

### Component Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_needs_calculation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(PhysiologyPlugin);

        let entity = app.world.spawn((
            Needs::default(),
            Personality::default(),
        )).id();

        // Run systems
        app.update();

        // Validate results
        let needs = app.world.get::<Needs>(entity).unwrap();
        assert!(needs.validate().is_ok());
    }
}
```

### Integration Testing
```rust
#[test]
fn test_stress_response_integration() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, PhysiologyPlugin));
    
    let entity = app.world.spawn((
        Needs { hunger: Normalized::new(0.9), ..default() },
        StressSystem::default(),
    )).id();
    
    // Trigger need critical event
    app.world.send_event(NeedCritical {
        entity,
        need_type: NeedType::Hunger,
        severity: 0.8,
        timestamp: 0.0,
    });
    
    app.update();
    
    let stress = app.world.get::<StressSystem>(entity).unwrap();
    assert!(stress.acute_stress > 0.5);
}
```

## Debug and Development Patterns

### Inspector Integration
```rust
use bevy_inspector_egui::prelude::*;

#[derive(Component, Debug, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Personality {
    #[inspector(min = 0.0, max = 1.0)]
    pub neuroticism: f32,
    #[inspector(min = 0.0, max = 1.0)]
    pub agreeableness: f32,
    // ... other traits
}
```

### Logging Patterns
```rust
use tracing::{info, warn, debug, trace};

pub fn decision_making_system(
    agents: Query<(Entity, &Personality, &EmotionalState, &Needs)>,
) {
    for (entity, personality, emotion, needs) in agents.iter() {
        debug!(
            entity = ?entity,
            neuroticism = personality.neuroticism,
            valence = emotion.valence.value(),
            "Processing agent decision"
        );
        
        if needs.hunger.value() > 0.8 {
            warn!(entity = ?entity, "Agent critically hungry");
        }
    }
}
```