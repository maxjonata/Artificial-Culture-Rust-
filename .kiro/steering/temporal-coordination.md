---
inclusion: fileMatch
fileMatchPattern: "src/ai/**/*.rs"
---

# Temporal Coordination System for Artificial Society

## Core Principle: Virtual World Time

All AI systems must be synchronized to a virtual world time that can be scaled, paused, or jumped independently of real-world time. This ensures consistent behavior regardless of simulation speed or time manipulation.

## WorldTime Resource

### Central Time Authority
```rust
#[derive(Resource, Debug, Reflect)]
pub struct WorldTime {
    pub current_time: f64,        // Virtual world time in seconds since simulation start
    pub time_scale: f32,          // Multiplier for time passage (1.0 = normal, 10.0 = 10x speed)
    pub delta_time: f32,          // Time elapsed since last frame in virtual seconds
    pub is_paused: bool,          // Whether time is currently paused
    pub total_real_time: f64,     // Real-world time for debugging/profiling
}

impl WorldTime {
    pub fn advance(&mut self, real_delta: f32) {
        if !self.is_paused {
            let virtual_delta = real_delta * self.time_scale;
            self.current_time += virtual_delta as f64;
            self.delta_time = virtual_delta;
        } else {
            self.delta_time = 0.0;
        }
        self.total_real_time += real_delta as f64;
    }
    
    pub fn jump_to(&mut self, new_time: f64) {
        self.current_time = new_time;
        // Emit TimeJumpEvent for systems that need to recalculate
    }
    
    pub fn set_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }
}
```

## Time-Based Calculation Patterns

### Decay and Growth Rates
```rust
// CORRECT: Time-delta based calculation
fn apply_need_decay(
    need_value: f32,
    decay_rate_per_hour: f32,
    world_time: &WorldTime,
) -> f32 {
    let hours_elapsed = world_time.delta_time / 3600.0;
    let decay_amount = decay_rate_per_hour * hours_elapsed;
    (need_value + decay_amount).clamp(0.0, 1.0)
}

// INCORRECT: Frame-based calculation
fn apply_need_decay_wrong(need_value: f32, decay_per_frame: f32) -> f32 {
    need_value + decay_per_frame  // Breaks with variable time scales!
}
```

### Memory Decay with Time
```rust
#[derive(Component)]
pub struct SocialMemory {
    pub interactions: Vec<InteractionRecord>,
}

#[derive(Debug)]
pub struct InteractionRecord {
    pub timestamp: f64,           // WorldTime when interaction occurred
    pub interaction_type: InteractionType,
    pub emotional_intensity: f32,
    pub base_strength: f32,       // Initial memory strength
}

impl InteractionRecord {
    pub fn current_strength(&self, world_time: &WorldTime) -> f32 {
        let hours_elapsed = (world_time.current_time - self.timestamp) / 3600.0;
        let half_life_hours = if self.emotional_intensity > 0.7 { 168.0 } else { 72.0 }; // 1 week vs 3 days
        
        // Exponential decay: strength = base * (0.5)^(time/half_life)
        self.base_strength * 0.5_f32.powf(hours_elapsed as f32 / half_life_hours)
    }
}
```

### Scheduled System Updates
```rust
#[derive(Component)]
pub struct ScheduledUpdate {
    pub last_update: f64,         // WorldTime of last update
    pub update_interval: f32,     // Seconds between updates
}

fn needs_decay_system(
    mut agents: Query<(&mut Needs, &mut ScheduledUpdate)>,
    world_time: Res<WorldTime>,
) {
    for (mut needs, mut schedule) in agents.iter_mut() {
        let time_since_update = world_time.current_time - schedule.last_update;
        
        if time_since_update >= schedule.update_interval as f64 {
            // Calculate how many intervals have passed (handles time jumps)
            let intervals_elapsed = time_since_update / schedule.update_interval as f64;
            
            // Apply decay for all elapsed intervals
            let decay_per_interval = 0.05 / 12.0; // 0.05 per hour, 12 intervals per hour (5min each)
            let total_decay = decay_per_interval * intervals_elapsed as f32;
            
            needs.hunger = (needs.hunger.value() + total_decay).clamp(0.0, 1.0).into();
            
            schedule.last_update = world_time.current_time;
        }
    }
}
```

## Event-Based Time Synchronization

### Time Events
```rust
#[derive(Event, Debug)]
pub struct TimeScaleChanged {
    pub old_scale: f32,
    pub new_scale: f32,
}

#[derive(Event, Debug)]
pub struct TimeJumped {
    pub old_time: f64,
    pub new_time: f64,
    pub jump_amount: f64,
}

#[derive(Event, Debug)]
pub struct TimePaused {
    pub paused: bool,
}
```

### System Response to Time Changes
```rust
fn handle_time_jumps(
    mut time_events: EventReader<TimeJumped>,
    mut agents: Query<&mut SocialMemory>,
) {
    for event in time_events.read() {
        // If time jumped forward significantly, age memories appropriately
        if event.jump_amount > 3600.0 { // More than 1 hour
            for mut memory in agents.iter_mut() {
                // Recalculate memory strengths based on new time
                memory.interactions.retain(|record| {
                    record.current_strength(&WorldTime { 
                        current_time: event.new_time,
                        ..default()
                    }) > 0.01 // Remove very weak memories
                });
            }
        }
    }
}
```

## Performance Optimization with Time

### Adaptive Update Frequencies
```rust
#[derive(Component)]
pub struct AdaptiveSchedule {
    pub base_interval: f32,       // Base update interval in seconds
    pub current_interval: f32,    // Current adaptive interval
    pub last_update: f64,         // Last update time
    pub importance: f32,          // 0.0-1.0, affects update frequency
}

impl AdaptiveSchedule {
    pub fn should_update(&self, world_time: &WorldTime) -> bool {
        let time_since_update = world_time.current_time - self.last_update;
        time_since_update >= self.current_interval as f64
    }
    
    pub fn adapt_frequency(&mut self, world_time: &WorldTime) {
        // More important agents update more frequently
        // Higher time scales reduce update frequency to maintain performance
        let time_scale_factor = (world_time.time_scale / 10.0).min(1.0);
        let importance_factor = 0.5 + (self.importance * 0.5);
        
        self.current_interval = self.base_interval / (importance_factor * time_scale_factor);
    }
}
```

## Integration with Existing Systems

### Needs System with Time
```rust
fn needs_decay_system(
    mut agents: Query<(&mut Needs, &Personality, &mut ScheduledUpdate)>,
    world_time: Res<WorldTime>,
) {
    for (mut needs, personality, mut schedule) in agents.iter_mut() {
        if !schedule.should_update(&world_time) {
            continue;
        }
        
        let hours_elapsed = (world_time.current_time - schedule.last_update) / 3600.0;
        
        // Base decay rates per hour
        let hunger_decay = 0.05 * hours_elapsed as f32;
        let energy_decay = 0.03 * hours_elapsed as f32;
        let social_decay = if personality.extraversion.value() > 0.6 {
            0.04 * hours_elapsed as f32  // Extraverts need more social contact
        } else {
            0.02 * hours_elapsed as f32
        };
        
        needs.hunger = (needs.hunger.value() + hunger_decay).clamp(0.0, 1.0).into();
        needs.energy = (needs.energy.value() - energy_decay).clamp(0.0, 1.0).into();
        needs.social = (needs.social.value() + social_decay).clamp(0.0, 1.0).into();
        
        schedule.last_update = world_time.current_time;
    }
}
```

### Stress System with Time
```rust
fn stress_recovery_system(
    mut agents: Query<(&mut StressSystem, &Needs)>,
    world_time: Res<WorldTime>,
) {
    for (mut stress, needs) in agents.iter_mut() {
        // Stress recovery rate depends on needs satisfaction
        let needs_satisfaction = 1.0 - (
            needs.hunger.value() + 
            needs.energy.value() + 
            needs.safety.value() + 
            needs.social.value()
        ) / 4.0;
        
        // Recovery rate: 0.1 per hour when all needs satisfied, 0.0 when all critical
        let recovery_rate_per_hour = 0.1 * needs_satisfaction;
        let hours_elapsed = world_time.delta_time / 3600.0;
        let recovery_amount = recovery_rate_per_hour * hours_elapsed;
        
        stress.acute_stress = (stress.acute_stress.value() - recovery_amount).clamp(0.0, 1.0).into();
        
        // Chronic stress recovers much more slowly (weeks to months)
        let chronic_recovery_rate = 0.001 * needs_satisfaction; // Very slow
        let chronic_recovery = chronic_recovery_rate * hours_elapsed;
        stress.chronic_load = (stress.chronic_load.value() - chronic_recovery).clamp(0.0, 1.0).into();
    }
}
```

## Debugging and Validation

### Time Consistency Checks
```rust
#[cfg(debug_assertions)]
fn validate_time_consistency(
    world_time: Res<WorldTime>,
    agents: Query<&ScheduledUpdate>,
) {
    for schedule in agents.iter() {
        if schedule.last_update > world_time.current_time {
            warn!(
                "Agent has future timestamp: {} > {}",
                schedule.last_update,
                world_time.current_time
            );
        }
    }
}
```

### Time Scale Testing
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_time_scale_consistency() {
        let mut world_time = WorldTime::default();
        let mut needs = Needs::default();
        
        // Test 1x speed
        world_time.time_scale = 1.0;
        world_time.advance(1.0); // 1 second real time = 1 second virtual
        assert_eq!(world_time.current_time, 1.0);
        
        // Test 10x speed
        world_time.time_scale = 10.0;
        world_time.advance(1.0); // 1 second real time = 10 seconds virtual
        assert_eq!(world_time.current_time, 11.0);
        
        // Test time jump
        world_time.jump_to(3600.0); // Jump to 1 hour
        assert_eq!(world_time.current_time, 3600.0);
    }
}
```