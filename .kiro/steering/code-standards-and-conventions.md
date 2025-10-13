---
inclusion: always
---

# Code Standards and Conventions for Artificial Society

## Core Philosophy: "Consistency Enables Collaboration"

Consistent code standards ensure that all contributors can understand, maintain, and extend the AI systems effectively. These conventions support the "Feel Over Science" philosophy while maintaining high technical quality.

## Naming Conventions

### Rust Language Standards
```rust
// CORRECT: Follow Rust naming conventions
pub struct PersonalityVector {
    pub openness: Normalized<f32>,
    pub conscientiousness: Normalized<f32>,
}

pub fn calculate_emotional_influence(
    base_emotion: f32,
    personality_modifier: f32,
) -> f32 {
    // Implementation
}

// CORRECT: Constants use SCREAMING_SNAKE_CASE
const MAX_SOCIAL_INFLUENCE_RADIUS: f32 = 10.0;
const DEFAULT_PERSONALITY_DISTRIBUTION: PersonalityDistribution = PersonalityDistribution::Normal;
```

### AI-Specific Naming Patterns
```rust
// CORRECT: AI components describe behavioral purpose
#[derive(Component)]
pub struct SocialMemory {
    pub recent_interactions: Vec<InteractionRecord>,
    pub relationship_strengths: HashMap<Entity, f32>,
}

#[derive(Component)]
pub struct EmotionalContagionRadius {
    pub influence_range: f32,
    pub contagion_strength: f32,
}

// CORRECT: Systems describe what they do behaviorally
fn emotional_contagion_system() {}
fn personality_expression_system() {}
fn social_memory_decay_system() {}

// AVOID: Technical names that don't describe behavior
fn update_ai_state() {}  // Too generic
fn process_data() {}     // Doesn't describe purpose
```

### Event Naming Conventions
```rust
// CORRECT: Events describe what happened behaviorally
#[derive(Event)]
pub struct SocialInteractionAttempted {
    pub initiator: Entity,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

#[derive(Event)]
pub struct PersonalityShiftTriggered {
    pub entity: Entity,
    pub trigger_cause: StressTrigger,
    pub shift_magnitude: f32,
}

#[derive(Event)]
pub struct EmotionalStateChanged {
    pub entity: Entity,
    pub old_valence: f32,
    pub new_valence: f32,
}
```

## File and Directory Organization

### Domain-Based Structure
```
src/
├── main.rs                    # Entry point only
├── lib.rs                     # Public API exports
├── ai/                        # AI domain - all AI-related code
│   ├── mod.rs                 # AI plugin coordinator
│   ├── physiology/            # Needs, stress, energy systems
│   │   ├── mod.rs
│   │   ├── components.rs      # Physiological components
│   │   ├── systems.rs         # Physiological systems
│   │   └── events.rs          # Physiological events
│   ├── cognition/             # Decision making, memory, learning
│   │   ├── mod.rs
│   │   ├── personality.rs     # Personality traits and modulation
│   │   ├── decision_making.rs # Decision systems
│   │   ├── memory.rs          # Memory and learning systems
│   │   └── beliefs.rs         # Belief formation and bias
│   ├── social/                # Social interaction and communication
│   │   ├── mod.rs
│   │   ├── expression.rs      # Social expression systems
│   │   ├── perception.rs      # Social perception and attention
│   │   ├── communication.rs   # Communication pipeline
│   │   └── relationships.rs   # Relationship management
│   └── perception/            # Environmental perception
│       ├── mod.rs
│       ├── vision.rs          # Visual perception systems
│       └── spatial.rs         # Spatial awareness
├── world/                     # World domain - environment and objects
│   ├── mod.rs
│   ├── environment/           # Environmental systems
│   └── objects/               # Interactive objects
├── presentation/              # Presentation domain - UI, debug, rendering
│   ├── mod.rs
│   ├── debug_ui.rs           # Debug interfaces
│   ├── profiler.rs           # Performance monitoring
│   └── rendering/            # Visual representation
└── core/                     # Core utilities and types
    ├── mod.rs
    ├── types.rs              # Core type definitions
    ├── builders.rs           # Entity builders
    └── constants.rs          # Global constants
```

### Module Organization Rules
```rust
// CORRECT: Each domain module structure
// src/ai/physiology/mod.rs
pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use systems::*;
pub use events::*;

use bevy::prelude::*;

pub struct PhysiologyPlugin;

impl Plugin for PhysiologyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Needs>()
           .register_type::<StressSystem>()
           .add_event::<NeedCritical>()
           .add_systems(Update, (
               needs_decay_system,
               stress_response_system,
           ));
    }
}
```

## Component Design Standards

### Pure Data Components
```rust
// CORRECT: Components contain only data, no behavior
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Personality {
    pub openness: Normalized<f32>,
    pub conscientiousness: Normalized<f32>,
    pub extraversion: Normalized<f32>,
    pub agreeableness: Normalized<f32>,
    pub neuroticism: Normalized<f32>,
}

// CORRECT: Helper methods for data access only
impl Personality {
    pub fn get_trait(&self, trait_type: PersonalityTrait) -> f32 {
        match trait_type {
            PersonalityTrait::Openness => self.openness.value(),
            PersonalityTrait::Conscientiousness => self.conscientiousness.value(),
            // ... other traits
        }
    }
    
    pub fn is_introverted(&self) -> bool {
        self.extraversion.value() < 0.3
    }
}

// AVOID: Components with behavior methods
impl Personality {
    // DON'T DO THIS - behavior belongs in systems
    pub fn update_from_stress(&mut self, stress_level: f32) {
        // This should be in a system, not a component
    }
}
```

### Value Range Documentation
```rust
// CORRECT: Clear documentation of value ranges and meanings
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct EmotionalState {
    /// Pleasure/displeasure dimension: -1.0 (very unpleasant) to 1.0 (very pleasant)
    pub valence: f32,
    
    /// Activation/deactivation dimension: -1.0 (very calm) to 1.0 (very excited)
    pub arousal: f32,
    
    /// Control/submission dimension: -1.0 (very submissive) to 1.0 (very dominant)
    pub dominance: f32,
}

// CORRECT: Validation methods
impl EmotionalState {
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(-1.0, 1.0),
            dominance: dominance.clamp(-1.0, 1.0),
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if !(-1.0..=1.0).contains(&self.valence) {
            return Err(format!("Invalid valence: {}", self.valence));
        }
        if !(-1.0..=1.0).contains(&self.arousal) {
            return Err(format!("Invalid arousal: {}", self.arousal));
        }
        if !(-1.0..=1.0).contains(&self.dominance) {
            return Err(format!("Invalid dominance: {}", self.dominance));
        }
        Ok(())
    }
}
```

## System Design Standards

### Event-Driven Communication
```rust
// CORRECT: Systems communicate through events
fn stress_response_system(
    mut stress_events: EventReader<NeedCritical>,
    mut agents: Query<&mut StressSystem>,
    mut expression_events: EventWriter<ExpressionChanged>,
) {
    for event in stress_events.read() {
        if let Ok(mut stress) = agents.get_mut(event.entity) {
            // Update stress based on event
            stress.acute_stress = (stress.acute_stress.value() + event.severity).clamp(0.0, 1.0).into();
            
            // Emit event for other systems
            expression_events.send(ExpressionChanged {
                entity: event.entity,
                stress_change: event.severity,
            });
        }
    }
}

// AVOID: Direct component access between domains
fn bad_system(
    mut stress_query: Query<&mut StressSystem>,
    mut expression_query: Query<&mut SocialExpression>,  // DON'T DO THIS
) {
    // Directly modifying components from different domains breaks encapsulation
}
```

### Personality Modulation Pattern
```rust
// CORRECT: All AI systems should be modulated by personality
fn emotional_contagion_system(
    mut agents: Query<(&mut EmotionalState, &Transform, &Personality)>,
    nearby_agents: Query<(&EmotionalState, &Transform), Without<Player>>,
) {
    for (mut emotion, transform, personality) in agents.iter_mut() {
        // Base contagion calculation
        let mut influence_sum = 0.0;
        let mut influence_count = 0;
        
        // Find nearby agents and calculate influence
        for (other_emotion, other_transform) in nearby_agents.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if distance < CONTAGION_RADIUS {
                let influence = other_emotion.valence * (1.0 - distance / CONTAGION_RADIUS);
                influence_sum += influence;
                influence_count += 1;
            }
        }
        
        if influence_count > 0 {
            let average_influence = influence_sum / influence_count as f32;
            
            // IMPORTANT: Modulate by personality
            let openness_factor = personality.openness.value();
            let extraversion_factor = personality.extraversion.value();
            let susceptibility = (openness_factor + extraversion_factor) / 2.0;
            
            let final_influence = average_influence * susceptibility * CONTAGION_STRENGTH;
            emotion.valence = (emotion.valence + final_influence).clamp(-1.0, 1.0);
        }
    }
}
```

### Temporal Consistency Pattern
```rust
// CORRECT: Use WorldTime for all temporal calculations
fn memory_decay_system(
    mut agents: Query<&mut SocialMemory>,
    world_time: Res<WorldTime>,
) {
    for mut memory in agents.iter_mut() {
        let hours_elapsed = world_time.delta_time / 3600.0;
        
        // Apply time-based decay
        memory.interactions.retain_mut(|interaction| {
            let age_hours = (world_time.current_time - interaction.timestamp) / 3600.0;
            let decay_factor = (-0.001 * age_hours).exp(); // Exponential decay
            interaction.strength *= decay_factor;
            
            interaction.strength > 0.01 // Remove very weak memories
        });
    }
}

// AVOID: Using real-world time
fn bad_temporal_system(
    mut agents: Query<&mut SocialMemory>,
) {
    let now = std::time::Instant::now(); // DON'T DO THIS
    // Real-world time breaks time scaling and simulation consistency
}
```

## Documentation Standards

### Component Documentation
```rust
/// Represents an agent's basic physiological needs that drive behavior.
/// 
/// All needs are represented as values from 0.0 (completely satisfied) to 1.0 (critically unmet).
/// When needs exceed 0.8, they trigger stress responses and behavioral changes.
/// 
/// # Behavioral Impact
/// - High hunger (>0.6): Agents prioritize food-seeking behavior
/// - Low energy (<0.3): Agents seek rest and avoid complex social interactions
/// - High safety need (>0.7): Agents become cautious and avoid perceived threats
/// - High social need (>0.5): Agents actively seek social interaction
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Needs {
    /// Hunger level: 0.0 = well-fed, 1.0 = starving
    pub hunger: Normalized<f32>,
    
    /// Energy level: 0.0 = exhausted, 1.0 = fully rested
    pub energy: Normalized<f32>,
    
    /// Safety need: 0.0 = feels secure, 1.0 = feels threatened
    pub safety: Normalized<f32>,
    
    /// Social connection need: 0.0 = socially fulfilled, 1.0 = lonely
    pub social: Normalized<f32>,
}
```

### System Documentation
```rust
/// Processes emotional contagion between nearby agents.
/// 
/// This system implements the spread of emotions through social proximity,
/// modulated by personality traits and relationship strength. Agents with
/// high openness and extraversion are more susceptible to emotional influence.
/// 
/// # Behavioral Purpose
/// Creates realistic emotional dynamics where group moods emerge naturally
/// from individual emotional states, leading to phenomena like crowd excitement
/// or collective anxiety.
/// 
/// # Performance Notes
/// - Runs every frame for immediate emotional response
/// - Uses spatial partitioning for efficient neighbor queries
/// - Processes agents in parallel for scalability
/// 
/// # Scientific Basis
/// Based on research in emotional contagion (Hatfield et al., 1994) and
/// the role of personality in emotional susceptibility.
fn emotional_contagion_system(
    mut agents: Query<(&mut EmotionalState, &Transform, &Personality)>,
    spatial_query: Res<RapierContext>,
) {
    // Implementation...
}
```

### Error Handling Standards
```rust
// CORRECT: Graceful error handling with context
fn social_interaction_system(
    mut interaction_events: EventReader<SocialInteractionAttempt>,
    mut agents: Query<(&mut SocialMemory, &Personality)>,
) {
    for event in interaction_events.read() {
        // Graceful handling of missing components
        let Ok((mut memory, personality)) = agents.get_mut(event.initiator) else {
            warn!("Social interaction attempted by entity without required components: {:?}", event.initiator);
            continue;
        };
        
        // Validate data before processing
        if let Err(e) = memory.validate() {
            error!("Invalid social memory state for entity {:?}: {}", event.initiator, e);
            continue;
        }
        
        // Process interaction...
    }
}
```

## Testing Standards

### Unit Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    
    /// Test that personality traits properly modulate emotional contagion
    #[test]
    fn test_personality_modulates_emotional_contagion() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(AiPlugin);
        
        // Create agents with different personalities
        let introverted_agent = app.world.spawn((
            EmotionalState::new(0.0, 0.0, 0.0),
            Personality { extraversion: 0.1.into(), ..default() },
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        
        let extraverted_agent = app.world.spawn((
            EmotionalState::new(0.0, 0.0, 0.0),
            Personality { extraversion: 0.9.into(), ..default() },
            Transform::from_xyz(1.0, 0.0, 0.0),
        )).id();
        
        // Create emotional source
        app.world.spawn((
            EmotionalState::new(0.8, 0.0, 0.0), // High positive valence
            Transform::from_xyz(0.5, 0.0, 0.0), // Between the two agents
        ));
        
        // Run emotional contagion system
        app.update();
        
        // Verify that extraverted agent is more affected
        let introverted_emotion = app.world.get::<EmotionalState>(introverted_agent).unwrap();
        let extraverted_emotion = app.world.get::<EmotionalState>(extraverted_agent).unwrap();
        
        assert!(extraverted_emotion.valence > introverted_emotion.valence,
                "Extraverted agents should be more susceptible to emotional contagion");
    }
}
```

These standards ensure that the codebase remains maintainable, performant, and aligned with the project's behavioral goals while supporting effective collaboration among contributors.