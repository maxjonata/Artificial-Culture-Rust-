## Project Overview

Agent-based AI simulation in Rust/Bevy modeling emergent social dynamics. Agents have internal cognitive systems that interact to produce complex behaviors through local interactions.

## Core Principles

1. **Information Isolation:** Agents cannot directly access other entities' internal state
2. **Cognitive Architecture:** Each agent has competing/cooperating internal sub-systems
3. **Performance-Driven Communication:** Choose polling vs events based on scalability needs
4. **Data-Oriented Design:** Compact types, flat structures, debugger-accessible values
5. **Emergent Behavior:** Complex patterns arise from simple local interactions
6. **Continuous State Space:** No enums - all states represented as normalized mathematical values

## Mathematical Philosophy

**Never use enums.** All states, decisions, and classifications must be continuous values normalized to ranges (typically 0.0-1.0). This creates realistic "grey areas" where agents can have partial, conflicting, or uncertain states.

Examples:
- Instead of `enum Mood { Happy, Sad, Angry }` → `mood_valence: f32, mood_arousal: f32`
- Instead of `enum Action { Walk, Run, Rest }` → `movement_intensity: f32, energy_expenditure: f32`
- Instead of `enum Relationship { Friend, Enemy }` → `trust_level: f32, hostility_level: f32`

## Project Structure

```
src/ai/{domain}/
├── mod.rs          # Plugin definition and exports
├── components.rs   # Component definitions
├── bundles.rs      # Entity bundle definitions
├── systems.rs      # System implementations
├── factories.rs    # Entity spawning functions
└── helpers.rs      # Domain utilities
```

**Domains:** `physiology`, `cognition`, `perception`, `social`, `navigation`, `environment`

## File Responsibilities

### mod.rs
- Plugin struct implementing `Plugin` trait
- Public exports of domain types
- System and type registration

### components.rs
- Component structs with normalized f32 fields
- Arithmetic operator overrides for value constraints
- Pure data containers only

### bundles.rs
- Bundle structs grouping related components
- Logical entity compositions

### systems.rs
- System functions with clear naming
- Mathematical transformations and normalizations
- Communication pattern chosen by scalability needs

### factories.rs
- Entity creation functions
- Parameter validation and normalization
- Return spawned `Entity` IDs

### helpers.rs
- Mathematical normalization functions
- Continuous value transformations
- Configuration constants

## Coding Standards

### Component Definition
All fields must be continuous values:
```rust
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ComponentName {
    pub primary_value: f32,      // 0.0-1.0 normalized
    pub secondary_value: f32,    // 0.0-1.0 normalized
    pub intensity: f32,          // 0.0-1.0 normalized
}
```

### Value Constraints via Operator Overrides
```rust
use std::ops::{Add, Mul, Sub};

impl Add<f32> for ComponentName {
    type Output = Self;
    
    fn add(mut self, rhs: f32) -> Self::Output {
        self.primary_value = (self.primary_value + rhs).clamp(0.0, 1.0);
        self
    }
}

impl Mul<f32> for ComponentName {
    type Output = Self;
    
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.primary_value = (self.primary_value * rhs).clamp(0.0, 1.0);
        self
    }
}
```

### Mathematical Normalizations
Use helper functions for common transformations:
```rust
// In helpers.rs
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn normalize_to_range(value: f32, min: f32, max: f32) -> f32 {
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
```

### Plugin Registration
All types must be registered for debugger access:
```rust
impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ComponentName>()
           .register_type::<BundleName>()
           .register_type::<f32>()
           .add_systems(Update, system_name);
    }
}
```

### System Communication
Choose based on performance requirements:
- **Events:** Low-frequency, decoupled communication
- **Polling:** High-frequency, performance-critical operations

### Naming Conventions
- Systems: `{action}_{target}_system`
- Components: `{Domain}{Concept}`
- Bundles: `{Purpose}Bundle`
- Factories: `spawn_{entity_type}`
- Events: `{Domain}{Action}Event`
- Fields: `{concept}_{dimension}` (e.g., `mood_valence`, `trust_level`)

## Continuous State Design

### Multi-Dimensional States
Replace discrete categories with continuous dimensions:
```rust
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct EmotionalState {
    pub valence: f32,        // pleasure/displeasure axis
    pub arousal: f32,        // activation/deactivation axis
    pub dominance: f32,      // control/submission axis
}
```

### Decision Making
Use weighted combinations instead of discrete choices:
```rust
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ActionTendencies {
    pub exploration_drive: f32,
    pub social_seeking: f32,
    pub resource_gathering: f32,
    pub rest_inclination: f32,
}
```

### Relationship Modeling
Multiple continuous dimensions for complex relationships:
```rust
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct SocialBond {
    pub trust_level: f32,
    pub familiarity: f32,
    pub emotional_attachment: f32,
    pub perceived_status: f32,
}
```

## Type Registration

All values must be registered for debugger inspection:
```rust
app.register_type::<f32>()
   .register_type::<Vec3>()
   .register_type::<CustomType>();
```

## Terminology

**Environment Features:** Items in game world (food, materials, terrain)
**Bevy Resource:** Global state accessed via `Res<T>`/`ResMut<T>`

## Performance Guidelines

- Use compact types: `u16`, `u32`, `f16` when appropriate
- Batch mathematical operations in systems
- Choose communication pattern by scalability needs
- Profile before optimizing

## Scientific Inspiration

Reference concepts in file headers only:
```rust
//! Based on: Author "Book Title"
```

## ML Integration

- All normalized f32 fields create natural observation space
- Mark decision points with `// ML-HOOK:` comments
- Continuous values ideal for neural network training

## Development Workflow

### Entity Creation
1. Define components with normalized f32 fields in `components.rs`
2. Implement arithmetic operator overrides for constraints
3. Group into bundles in `bundles.rs`
4. Create factory functions with normalization in `factories.rs`
5. Register all types in plugin

### System Development
1. Implement mathematical transformations in `systems.rs`
2. Use helper functions for common normalizations
3. Choose communication pattern by performance needs
4. Add to plugin registration
5. Test with debugger inspection

### Domain Integration
1. Export public API in `mod.rs`
2. Register plugin in `main.rs`
3. Ensure all types are debugger-accessible

## Command Guidelines

Single commands only:
```bash
cargo build
cargo test
cargo run
```

Never read, write or interact in any way with files inside Docs\Backstory