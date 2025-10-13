---
inclusion: fileMatch
fileMatchPattern: "src/ai/physiology/*"
---

# Phase 2: Physiological Foundation Development Guide

## Current Development Focus

You are currently implementing **Phase 2: Physiological Foundation (Weeks 4-6)** of the Artificial Society project. This phase establishes the base biological simulation layer that drives all higher-level behaviors.

## Key Implementation Requirements

### 1. Core Components to Implement

**Needs Component** (`src/ai/physiology/needs.rs`):
```rust
// PERFORMANCE OPTIMIZED: Use u8 for memory efficiency
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct NeedsCompact {
    pub hunger: u8,     // 0-255 mapped to 0.0-1.0, saves 12 bytes per agent
    pub energy: u8,     // 0-255 mapped to 0.0-1.0
    pub safety: u8,     // 0-255 mapped to 0.0-1.0
    pub social: u8,     // 0-255 mapped to 0.0-1.0
}

impl NeedsCompact {
    pub fn hunger_f32(&self) -> f32 { self.hunger as f32 / 255.0 }
    pub fn set_hunger(&mut self, value: f32) { 
        self.hunger = (value.clamp(0.0, 1.0) * 255.0) as u8;
    }
    // Similar methods for other needs...
}

// Alternative: Keep f32 for easier debugging during development
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Needs {
    pub hunger: Normalized<f32>,    // 0.0 = satisfied, 1.0 = starving
    pub energy: Normalized<f32>,    // 0.0 = exhausted, 1.0 = fully rested  
    pub safety: Normalized<f32>,    // 0.0 = secure, 1.0 = terrified
    pub social: Normalized<f32>,    // 0.0 = fulfilled, 1.0 = lonely
}
```

**Stress System Component** (`src/ai/physiology/stress.rs`):
```rust
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct StressSystem {
    pub acute_stress: Normalized<f32>,    // 0.0 = calm, 1.0 = panic
    pub chronic_load: Normalized<f32>,    // 0.0 = fresh, 1.0 = burned out
    pub state: StressState,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct EmotionalState {
    pub valence: f32,        // -1.0 to 1.0 (pleasure/displeasure)
    pub arousal: f32,        // -1.0 to 1.0 (activation/calm)
    pub dominance: f32,      // -1.0 to 1.0 (control/submission)
}

#[derive(Debug, Reflect, PartialEq)]
pub enum StressState {
    Homeostasis,    // Normal, resilient state
    Allostasis,     // Alert, adaptive state  
    PostTraumatic,  // Broken, hypersensitive state
}
```

### 2. Essential Systems to Implement

**Needs Decay System** (POLLING - Every 5-30 seconds):
```rust
fn needs_decay_system(
    mut agents: Query<(&mut Needs, &mut PollingSchedule, &AgentImportance)>,
    world_time: Res<WorldTime>,
    quality_scaler: Res<QualityScaler>,
) {
    // Parallel processing for large agent populations
    agents.par_iter_mut().for_each(|(mut needs, mut schedule, importance)| {
        // Adaptive scheduling based on importance and performance
        let base_interval = match importance.level {
            ImportanceLevel::Critical => 5.0,   // Player-visible agents
            ImportanceLevel::High => 10.0,      // Nearby agents
            ImportanceLevel::Medium => 20.0,    // Background agents
            ImportanceLevel::Low => 30.0,       // Distant agents
        };
        
        let adjusted_interval = base_interval * quality_scaler.get_update_frequency_multiplier();
        
        if world_time.current_time - schedule.last_update >= adjusted_interval {
            // Simple linear decay - no complex calculations
            let hours_elapsed = (world_time.current_time - schedule.last_update) / 3600.0;
            let decay_rate = 0.05; // 5% per hour
            
            needs.hunger = (needs.hunger.value() + decay_rate * hours_elapsed).clamp(0.0, 1.0).into();
            schedule.last_update = world_time.current_time;
        }
    });
}
```

**Stress Response System** (EVENT-DRIVEN - Immediate response):
```rust
fn stress_response_system(
    mut stress_events: EventReader<NeedCritical>,
    mut agents: Query<(&mut StressSystem, &Personality)>,
    mut expression_events: EventWriter<ExpressionChanged>,
) {
    // Event-driven for immediate response to stress triggers
    for event in stress_events.read() {
        if let Ok((mut stress, personality)) = agents.get_mut(event.entity) {
            // Simple threshold-based stress calculation
            let stress_increase = match event.severity {
                s if s > 0.8 => 0.3,  // High severity
                s if s > 0.5 => 0.2,  // Medium severity
                _ => 0.1,             // Low severity
            };
            
            // Personality modulation (simple multiplier)
            let neuroticism_factor = 0.5 + personality.neuroticism.value();
            let final_increase = stress_increase * neuroticism_factor;
            
            stress.acute_stress = (stress.acute_stress.value() + final_increase).clamp(0.0, 1.0).into();
            
            // Simple state transitions with thresholds
            stress.state = match stress.acute_stress.value() {
                s if s > 0.9 => StressState::PostTraumatic,
                s if s > 0.7 => StressState::Allostasis,
                _ => StressState::Homeostasis,
            };
        }
    }
}
```

**Mood Integration System** (EVENT-DRIVEN - Triggered by social events):
```rust
fn mood_calculation_system(
    mut mood_events: EventReader<SocialInteractionComplete>,
    mut agents: Query<(&mut EmotionalState, &Needs, &Personality)>,
) {
    for event in mood_events.read() {
        if let Ok((mut emotion, needs, personality)) = agents.get_mut(event.entity) {
            // Simple linear mapping from needs to valence
            let needs_satisfaction = 1.0 - (
                needs.hunger.value() + needs.energy.value() + 
                needs.safety.value() + needs.social.value()
            ) / 4.0;
            
            let base_valence = (needs_satisfaction * 2.0) - 1.0; // Map to -1.0 to 1.0
            
            // Simple personality modulation
            let personality_modifier = match personality.neuroticism.value() {
                n if n > 0.6 => -0.2,  // High neuroticism = more negative
                _ => 0.0,
            } + match personality.extraversion.value() {
                e if e > 0.6 => 0.1,   // High extraversion = more positive
                _ => 0.0,
            };
            
            // Apply social interaction influence
            let social_influence = match event.outcome {
                InteractionOutcome::Positive => 0.2,
                InteractionOutcome::Negative => -0.3,
                InteractionOutcome::Neutral => 0.0,
            };
            
            let final_valence = (base_valence + personality_modifier + social_influence).clamp(-1.0, 1.0);
            emotion.valence = final_valence;
        }
    }
}
```

### 3. Critical Events to Implement

```rust
#[derive(Event, Debug)]
pub struct NeedCritical {
    pub entity: Entity,
    pub need_type: NeedType,
    pub severity: f32,
    pub timestamp: f32,
}

#[derive(Event, Debug)]
pub struct StressThresholdCrossed {
    pub entity: Entity,
    pub old_state: StressState,
    pub new_state: StressState,
    pub trigger_cause: String,
}

#[derive(Event, Debug)]
pub struct MoodChange {
    pub entity: Entity,
    pub old_valence: f32,
    pub new_valence: f32,
    pub change_magnitude: f32,
}
```

## Development Priorities

### Week 4: Core Components
1. Implement Needs component with proper validation
2. Create StressSystem with state machine logic
3. Add basic energy/fatigue tracking
4. Set up event structures and emission

### Week 5: System Implementation
1. Create needs_decay_system with realistic time scales
2. Implement stress_response_system with threshold logic
3. Add mood_calculation_system with personality integration
4. Connect systems through event-driven architecture

### Week 6: Integration and Testing
1. Test system integration and behavioral coherence
2. Validate that personality differences create distinct patterns
3. Optimize for 60fps with 100+ agents
4. Ensure player-observable behavioral changes

## Key Success Criteria

### Player Experience Validation
- Can players tell when an agent is hungry, tired, or stressed?
- Do agent behaviors feel natural and relatable?
- Are personality differences clear without being cartoonish?
- Do agents react to social situations in believable ways?

### Technical Requirements
- Maintain 60fps with 100+ agents
- All values use Normalized<f32> types
- Systems communicate only through events
- Components are visible in debug UI
- Personality modulates all physiological responses

## Common Pitfalls to Avoid

1. **Perfect Information**: Don't let agents directly access each other's internal states
2. **Discrete States**: Avoid boolean flags or enums for emotional/social states
3. **Over-Complexity**: Keep systems simple enough for players to understand intuitively
4. **Performance Neglect**: Profile regularly and optimize for target frame rate
5. **Missing Personality**: Every system should be modulated by personality traits

## Integration Points

### With Existing Systems
- Use existing Personality component from core types
- Integrate with debug UI for real-time monitoring
- Connect to performance profiler for optimization
- Use established event patterns from other domains

### For Future Systems
- Needs and stress will drive decision-making in Phase 3
- Mood will influence social expression in Phase 4
- Physiological state will affect perception and communication
- Stress states will modulate learning and memory systems

## Testing Strategy

### Unit Tests
- Validate component value ranges and constraints
- Test event emission under various conditions
- Verify personality modulation effects
- Check system performance with large agent counts

### Integration Tests
- Spawn agents with different personalities
- Trigger various physiological states
- Observe emergent behavioral differences
- Validate player intuition about agent states

### Performance Tests
- Benchmark system execution times
- Profile memory usage patterns
- Test with 100+ agents for 60fps target
- Monitor for performance degradation over time