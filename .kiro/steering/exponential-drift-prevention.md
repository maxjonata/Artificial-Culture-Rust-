---
inclusion: fileMatch
fileMatchPattern: "src/ai/**/*.rs"
---

# Exponential Drift Prevention in Cascading AI Systems

## The Problem: Cascading Exponential Growth

When multiple systems influence each other over time, small errors or feedback loops can compound exponentially, leading to:
- Values drifting outside valid ranges despite clamping
- Unrealistic behavior amplification over long simulations
- System instability during high time scaling
- Loss of behavioral believability over virtual weeks/months

## Critical Risk Areas

### 1. Emotional Contagion Feedback Loops
```rust
// DANGEROUS: Unchecked positive feedback
fn emotional_contagion_unsafe(
    mut agents: Query<&mut EmotionalState>,
    world_time: Res<WorldTime>,
) {
    // If agents continuously amplify each other's emotions...
    for mut emotion in agents.iter_mut() {
        let amplification = 1.1_f32.powf(world_time.delta_time); // Exponential growth!
        emotion.valence *= amplification; // Will explode over time
    }
}

// SAFE: Bounded influence with decay
fn emotional_contagion_safe(
    mut agents: Query<(&mut EmotionalState, &Transform)>,
    world_time: Res<WorldTime>,
) {
    let contagion_rate = 0.1; // Fixed rate per hour
    let max_influence = 0.3;  // Maximum change per interaction
    let decay_rate = 0.05;    // Natural emotional decay
    
    for (mut emotion, transform) in agents.iter_mut() {
        // Apply natural decay first (prevents runaway growth)
        let hours_elapsed = world_time.delta_time / 3600.0;
        emotion.valence *= (1.0 - decay_rate * hours_elapsed);
        
        // Then apply bounded contagion influence
        let influence = calculate_bounded_influence(transform, max_influence);
        emotion.valence = (emotion.valence + influence).clamp(-1.0, 1.0);
    }
}
```

### 2. Stress-Personality Feedback Loops
```rust
// DANGEROUS: Stress permanently altering personality
fn stress_personality_unsafe(
    mut agents: Query<(&mut Personality, &StressSystem)>,
    world_time: Res<WorldTime>,
) {
    for (mut personality, stress) in agents.iter_mut() {
        // Stress increases neuroticism, which increases stress sensitivity...
        let stress_impact = stress.chronic_load.value() * world_time.delta_time;
        personality.neuroticism = (personality.neuroticism.value() + stress_impact).into();
        // This creates runaway neuroticism growth!
    }
}

// SAFE: Bounded personality shifts with recovery
fn stress_personality_safe(
    mut agents: Query<(&mut Personality, &StressSystem, &mut PersonalityShift)>,
    world_time: Res<WorldTime>,
) {
    for (mut personality, stress, mut shift) in agents.iter_mut() {
        let hours_elapsed = world_time.delta_time / 3600.0;
        
        // Calculate temporary shift (not permanent change)
        let max_shift = 0.2; // Maximum 20% personality shift
        let target_shift = stress.chronic_load.value() * max_shift;
        
        // Gradual approach to target with recovery
        let shift_rate = 0.01; // 1% per hour
        if target_shift > shift.current_neuroticism_shift {
            shift.current_neuroticism_shift += shift_rate * hours_elapsed;
        } else {
            shift.current_neuroticism_shift -= shift_rate * hours_elapsed * 2.0; // Faster recovery
        }
        
        shift.current_neuroticism_shift = shift.current_neuroticism_shift.clamp(0.0, max_shift);
        
        // Apply temporary shift (original personality remains unchanged)
        let effective_neuroticism = (personality.neuroticism.value() + shift.current_neuroticism_shift).clamp(0.0, 1.0);
        // Use effective_neuroticism for calculations, but don't modify personality component
    }
}
```

### 3. Memory-Belief Reinforcement Spirals
```rust
// DANGEROUS: Confirmation bias creating extreme beliefs
fn belief_update_unsafe(
    mut agents: Query<&mut BeliefSystem>,
    interaction_events: EventReader<SocialInteractionComplete>,
) {
    for event in interaction_events.read() {
        if let Ok(mut beliefs) = agents.get_mut(event.observer) {
            let existing_belief = beliefs.get_belief(event.target);
            
            // Confirmation bias amplifies existing beliefs
            let bias_multiplier = 1.0 + existing_belief.confidence * 0.5;
            let new_evidence_weight = event.evidence_strength * bias_multiplier;
            
            // This can create runaway belief extremism!
            existing_belief.strength += new_evidence_weight;
        }
    }
}

// SAFE: Bounded belief updates with regression to mean
fn belief_update_safe(
    mut agents: Query<&mut BeliefSystem>,
    interaction_events: EventReader<SocialInteractionComplete>,
    world_time: Res<WorldTime>,
) {
    for event in interaction_events.read() {
        if let Ok(mut beliefs) = agents.get_mut(event.observer) {
            let mut belief = beliefs.get_belief_mut(event.target);
            
            // Natural regression toward neutral over time
            let hours_elapsed = world_time.delta_time / 3600.0;
            let regression_rate = 0.001; // Very slow drift toward neutral
            belief.strength *= 1.0 - (regression_rate * hours_elapsed);
            
            // Bounded belief updates with diminishing returns
            let max_change = 0.1; // Maximum change per interaction
            let confidence_dampening = 1.0 - belief.confidence * 0.5; // High confidence resists change
            let evidence_impact = (event.evidence_strength * confidence_dampening).clamp(-max_change, max_change);
            
            belief.strength = (belief.strength + evidence_impact).clamp(-1.0, 1.0);
            
            // Confidence grows slowly and has upper bound
            belief.confidence = (belief.confidence + 0.01).clamp(0.0, 0.95); // Never 100% certain
        }
    }
}
```

## Drift Prevention Strategies

### 1. Natural Decay and Regression
```rust
#[derive(Component)]
pub struct NaturalDecay {
    pub decay_rate: f32,        // Rate of return to baseline per hour
    pub baseline_value: f32,    // Target value for regression
    pub min_decay_rate: f32,    // Minimum decay to prevent stagnation
}

fn apply_natural_decay(
    value: f32,
    decay: &NaturalDecay,
    hours_elapsed: f32,
) -> f32 {
    let distance_from_baseline = value - decay.baseline_value;
    let decay_amount = distance_from_baseline * decay.decay_rate * hours_elapsed;
    value - decay_amount
}
```

### 2. Bounded Influence Systems
```rust
#[derive(Component)]
pub struct InfluenceBounds {
    pub max_change_per_hour: f32,     // Maximum rate of change
    pub max_total_deviation: f32,     // Maximum deviation from baseline
    pub influence_saturation: f32,    // Point where additional influence has diminishing returns
}

fn apply_bounded_influence(
    current_value: f32,
    influence: f32,
    bounds: &InfluenceBounds,
    hours_elapsed: f32,
) -> f32 {
    // Apply saturation curve to prevent runaway growth
    let saturated_influence = influence / (1.0 + influence.abs() / bounds.influence_saturation);
    
    // Limit rate of change
    let max_change = bounds.max_change_per_hour * hours_elapsed;
    let bounded_change = saturated_influence.clamp(-max_change, max_change);
    
    // Apply change and enforce total deviation bounds
    let new_value = current_value + bounded_change;
    new_value.clamp(
        bounds.max_total_deviation * -1.0,
        bounds.max_total_deviation
    )
}
```

### 3. Stability Monitoring
```rust
#[derive(Resource)]
pub struct StabilityMonitor {
    pub value_history: HashMap<Entity, VecDeque<f32>>,
    pub drift_alerts: Vec<DriftAlert>,
    pub max_history_length: usize,
}

#[derive(Debug)]
pub struct DriftAlert {
    pub entity: Entity,
    pub component_name: String,
    pub drift_rate: f32,
    pub timestamp: f64,
}

impl StabilityMonitor {
    pub fn check_for_drift(&mut self, entity: Entity, value: f32, component_name: &str, world_time: &WorldTime) {
        let history = self.value_history.entry(entity).or_insert_with(VecDeque::new);
        history.push_back(value);
        
        if history.len() > self.max_history_length {
            history.pop_front();
        }
        
        // Check for exponential drift
        if history.len() >= 10 {
            let recent_values: Vec<f32> = history.iter().rev().take(10).cloned().collect();
            let drift_rate = calculate_drift_rate(&recent_values);
            
            if drift_rate.abs() > 0.1 { // Alert if drifting more than 10% per measurement
                self.drift_alerts.push(DriftAlert {
                    entity,
                    component_name: component_name.to_string(),
                    drift_rate,
                    timestamp: world_time.current_time,
                });
                
                warn!(
                    "Exponential drift detected: Entity {:?}, Component {}, Rate: {:.3}",
                    entity, component_name, drift_rate
                );
            }
        }
    }
}

fn calculate_drift_rate(values: &[f32]) -> f32 {
    if values.len() < 2 {
        return 0.0;
    }
    
    // Calculate average rate of change
    let mut total_change = 0.0;
    for i in 1..values.len() {
        total_change += values[i] - values[i-1];
    }
    
    total_change / (values.len() - 1) as f32
}
```

### 4. Time-Scale Invariant Calculations
```rust
// WRONG: Time-scale dependent calculation
fn update_wrong(value: f32, world_time: &WorldTime) -> f32 {
    let change = 0.1 * world_time.time_scale; // Grows with time scale!
    value + change
}

// CORRECT: Time-scale invariant calculation
fn update_correct(value: f32, world_time: &WorldTime) -> f32 {
    let change_per_hour = 0.1;
    let hours_elapsed = world_time.delta_time / 3600.0;
    let change = change_per_hour * hours_elapsed; // Consistent regardless of time scale
    value + change
}
```

## Validation and Testing

### Drift Detection Tests
```rust
#[cfg(test)]
mod drift_tests {
    #[test]
    fn test_no_exponential_drift_over_time() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AiPlugin));
        
        // Spawn test agent
        let entity = app.world.spawn((
            EmotionalState { valence: 0.1, ..default() },
            StressSystem::default(),
            Personality::default(),
        )).id();
        
        let mut world_time = WorldTime::default();
        world_time.time_scale = 100.0; // High time scale to amplify drift
        
        // Run simulation for virtual months
        for _ in 0..1000 {
            world_time.advance(1.0); // 100 virtual seconds per iteration
            app.world.insert_resource(world_time.clone());
            app.update();
            
            // Check that values remain bounded
            let emotion = app.world.get::<EmotionalState>(entity).unwrap();
            assert!(emotion.valence >= -1.0 && emotion.valence <= 1.0);
            assert!(emotion.valence.abs() < 2.0); // Should not grow exponentially
        }
    }
    
    #[test]
    fn test_stability_under_time_jumps() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AiPlugin));
        
        let entity = app.world.spawn((
            Needs::default(),
            StressSystem::default(),
        )).id();
        
        // Jump forward by years
        let mut world_time = WorldTime::default();
        world_time.jump_to(365.0 * 24.0 * 3600.0); // 1 year
        app.world.insert_resource(world_time);
        app.update();
        
        // Values should remain reasonable
        let needs = app.world.get::<Needs>(entity).unwrap();
        assert!(needs.hunger.value() <= 1.0);
        assert!(needs.energy.value() >= 0.0);
    }
}
```