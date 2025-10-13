---
inclusion: fileMatch
fileMatchPattern: "src/ai/**/*.rs"
---

# Performance Optimization Patterns for Artificial Society

## Core Philosophy: "Feel Over Precision"

Optimize for the minimum computational cost that maintains behavioral believability. If players can't perceive the difference, use the simpler calculation.

## Decimal Precision Optimization

### Memory-Efficient Component Storage
```rust
// OPTIMIZED: Use smallest types that maintain believability
#[derive(Component, Debug)]
pub struct PersonalityCompact {
    // Store as u8 (1 byte each), convert to f32 for calculations
    pub openness: u8,         // 0-255 mapped to 0.0-1.0
    pub conscientiousness: u8,
    pub extraversion: u8,
    pub agreeableness: u8,
    pub neuroticism: u8,
}

impl PersonalityCompact {
    pub fn openness_f32(&self) -> f32 {
        self.openness as f32 / 255.0
    }
    
    pub fn set_openness(&mut self, value: f32) {
        self.openness = (value.clamp(0.0, 1.0) * 255.0) as u8;
    }
}

// OPTIMIZED: Bipolar values using i16 (2 bytes)
#[derive(Component, Debug)]
pub struct EmotionalStateCompact {
    pub valence: i16,    // -32768 to 32767 mapped to -1.0 to 1.0
    pub arousal: i16,
    pub dominance: i16,
}

impl EmotionalStateCompact {
    pub fn valence_f32(&self) -> f32 {
        self.valence as f32 / 32767.0
    }
    
    pub fn set_valence(&mut self, value: f32) {
        self.valence = (value.clamp(-1.0, 1.0) * 32767.0) as i16;
    }
}
```

### Quantized Calculations
```rust
// OPTIMIZED: Quantize intermediate calculations
const EMOTION_QUANTIZATION: f32 = 0.01; // 1% precision

fn quantize_emotion(value: f32) -> f32 {
    (value / EMOTION_QUANTIZATION).round() * EMOTION_QUANTIZATION
}

fn calculate_emotional_influence(base: f32, influence: f32) -> f32 {
    let result = base + influence;
    quantize_emotion(result.clamp(-1.0, 1.0))
}
```

## Event-Driven vs Polling Patterns

### Event-Driven for Immediate Response
```rust
// Use events for immediate social responses
#[derive(Event)]
pub struct SocialInteractionAttempt {
    pub initiator: Entity,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

fn immediate_social_response_system(
    mut events: EventReader<SocialInteractionAttempt>,
    mut agents: Query<&mut EmotionalState>,
) {
    for event in events.read() {
        // Immediate emotional response to social interaction
        if let Ok(mut emotion) = agents.get_mut(event.target) {
            let response = calculate_immediate_response(&event);
            emotion.set_valence(emotion.valence_f32() + response);
        }
    }
}
```

### Polling for Gradual Changes
```rust
// Use polling for gradual decay/growth
#[derive(Component)]
pub struct PollingSchedule {
    pub last_update: f32,
    pub interval: f32,
    pub priority: u8, // 0 = highest priority, 255 = lowest
}

fn needs_decay_polling_system(
    mut agents: Query<(&mut Needs, &mut PollingSchedule)>,
    world_time: Res<WorldTime>,
) {
    for (mut needs, mut schedule) in agents.iter_mut() {
        let time_since_update = world_time.current_time as f32 - schedule.last_update;
        
        if time_since_update >= schedule.interval {
            // Batch multiple intervals if system was skipped
            let intervals_elapsed = (time_since_update / schedule.interval).floor();
            
            // Simple linear decay per interval
            let decay_per_interval = 0.01; // 1% per interval
            let total_decay = decay_per_interval * intervals_elapsed;
            
            needs.hunger = (needs.hunger.value() + total_decay).clamp(0.0, 1.0).into();
            schedule.last_update = world_time.current_time as f32;
        }
    }
}
```

### Adaptive Scheduling
```rust
#[derive(Resource)]
pub struct AdaptiveScheduler {
    pub frame_budget_ms: f32,
    pub current_frame_time: f32,
    pub system_priorities: HashMap<String, u8>,
}

impl AdaptiveScheduler {
    pub fn should_run_system(&self, system_name: &str, base_interval: f32) -> bool {
        let priority = self.system_priorities.get(system_name).unwrap_or(&128);
        let budget_factor = (self.frame_budget_ms - self.current_frame_time) / self.frame_budget_ms;
        
        // Higher priority systems run more often when budget is tight
        let adjusted_interval = base_interval * (1.0 + (255 - *priority as u32) as f32 * budget_factor);
        
        // Simple time-based check (implement actual timing logic)
        true // Simplified for example
    }
}
```

## Parallel Processing Patterns

### Parallel Agent Processing
```rust
use bevy::tasks::ParallelIterator;

fn parallel_emotional_contagion_system(
    mut agents: Query<(&mut EmotionalState, &Transform, Entity)>,
    spatial_query: Res<RapierContext>,
) {
    // Collect nearby agents data first (avoid borrowing issues)
    let agent_data: Vec<(Entity, Vec3, EmotionalState)> = agents
        .iter()
        .map(|(emotion, transform, entity)| (entity, transform.translation, *emotion))
        .collect();
    
    // Process in parallel batches
    agents.par_iter_mut().for_each(|(mut emotion, transform, entity)| {
        let mut total_influence = 0.0;
        let mut influence_count = 0;
        
        // Find nearby agents (simplified spatial query)
        for (other_entity, other_pos, other_emotion) in &agent_data {
            if *other_entity == entity { continue; }
            
            let distance = transform.translation.distance(*other_pos);
            if distance < CONTAGION_RADIUS {
                let influence = other_emotion.valence_f32() * (1.0 - distance / CONTAGION_RADIUS);
                total_influence += influence;
                influence_count += 1;
            }
        }
        
        if influence_count > 0 {
            let average_influence = total_influence / influence_count as f32;
            let contagion_strength = 0.1; // 10% influence per frame
            let new_valence = emotion.valence_f32() + (average_influence * contagion_strength);
            emotion.set_valence(new_valence.clamp(-1.0, 1.0));
        }
    });
}
```

### Batch Processing for Efficiency
```rust
const BATCH_SIZE: usize = 32;

fn batched_memory_decay_system(
    mut agents: Query<&mut SocialMemory>,
    world_time: Res<WorldTime>,
) {
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    
    for mut memory in agents.iter_mut() {
        batch.push(memory.as_mut());
        
        if batch.len() >= BATCH_SIZE {
            process_memory_batch(&mut batch, &world_time);
            batch.clear();
        }
    }
    
    // Process remaining agents
    if !batch.is_empty() {
        process_memory_batch(&mut batch, &world_time);
    }
}

fn process_memory_batch(batch: &mut [Mut<SocialMemory>], world_time: &WorldTime) {
    // Process entire batch with optimized calculations
    let hours_elapsed = world_time.delta_time / 3600.0;
    let decay_factor = (-0.001 * hours_elapsed).exp(); // Exponential decay
    
    for memory in batch.iter_mut() {
        // Vectorized decay operation
        memory.interactions.iter_mut().for_each(|interaction| {
            interaction.strength *= decay_factor;
        });
        
        // Remove very weak memories (batch cleanup)
        memory.interactions.retain(|interaction| interaction.strength > 0.01);
    }
}
```

## Simplified Calculation Models

### Linear Interpolation Over Complex Functions
```rust
// SIMPLE: Linear personality influence
fn apply_personality_influence(base_value: f32, trait_value: f32, influence_strength: f32) -> f32 {
    let trait_modifier = (trait_value - 0.5) * 2.0; // Convert 0-1 to -1 to 1
    let influence = trait_modifier * influence_strength;
    (base_value + influence).clamp(-1.0, 1.0)
}

// AVOID: Complex psychological models
fn complex_personality_influence(base: f32, traits: &[f32]) -> f32 {
    // Don't do this - too expensive for minimal behavioral difference
    let mut result = base;
    for (i, trait_val) in traits.iter().enumerate() {
        result += trait_val.sin() * (i as f32).cos() * 0.1;
    }
    result.tanh() // Expensive activation function
}
```

### Lookup Tables for Expensive Calculations
```rust
// Pre-computed lookup table for expensive functions
lazy_static! {
    static ref STRESS_RESPONSE_LUT: Vec<f32> = {
        (0..=1000).map(|i| {
            let stress = i as f32 / 1000.0;
            // Pre-compute expensive stress response function
            1.0 / (1.0 + (-10.0 * (stress - 0.5)).exp()) // Sigmoid
        }).collect()
    };
}

fn get_stress_response(stress_level: f32) -> f32 {
    let index = (stress_level.clamp(0.0, 1.0) * 1000.0) as usize;
    STRESS_RESPONSE_LUT[index]
}
```

### Threshold-Based State Machines
```rust
#[derive(Debug, PartialEq)]
pub enum StressState {
    Calm,      // 0.0 - 0.3
    Stressed,  // 0.3 - 0.7
    Panic,     // 0.7 - 1.0
}

impl StressState {
    pub fn from_level(level: f32) -> Self {
        match level {
            x if x < 0.3 => StressState::Calm,
            x if x < 0.7 => StressState::Stressed,
            _ => StressState::Panic,
        }
    }
    
    pub fn decision_modifier(&self) -> f32 {
        match self {
            StressState::Calm => 1.0,      // Normal decision making
            StressState::Stressed => 0.7,  // Reduced cognitive clarity
            StressState::Panic => 0.3,     // Severely impaired decisions
        }
    }
}
```

## Memory and Cache Optimization

### Cache-Friendly Component Layout
```rust
// OPTIMIZED: Pack related data together
#[derive(Component)]
pub struct AgentCoreState {
    // Frequently accessed together - good cache locality
    pub position: Vec3,           // 12 bytes
    pub current_emotion: i16,     // 2 bytes  
    pub energy_level: u8,         // 1 byte
    pub stress_state: u8,         // 1 byte (enum as u8)
    // Total: 16 bytes - fits in single cache line
}

// AVOID: Scattered component access
// Don't query many separate components in hot loops
```

### Object Pooling for Temporary Data
```rust
#[derive(Resource)]
pub struct TempDataPool {
    pub interaction_buffers: Vec<Vec<SocialInteraction>>,
    pub calculation_buffers: Vec<Vec<f32>>,
}

impl TempDataPool {
    pub fn get_interaction_buffer(&mut self) -> Vec<SocialInteraction> {
        self.interaction_buffers.pop().unwrap_or_else(Vec::new)
    }
    
    pub fn return_interaction_buffer(&mut self, mut buffer: Vec<SocialInteraction>) {
        buffer.clear();
        if buffer.capacity() <= 1024 { // Prevent unbounded growth
            self.interaction_buffers.push(buffer);
        }
    }
}
```

### Efficient Social Network Storage
```rust
// OPTIMIZED: Sparse matrix for relationships
#[derive(Resource)]
pub struct SocialNetwork {
    // Only store non-zero relationships
    pub relationships: HashMap<(Entity, Entity), f32>,
    pub entity_connections: HashMap<Entity, Vec<Entity>>, // Quick neighbor lookup
}

impl SocialNetwork {
    pub fn get_relationship(&self, a: Entity, b: Entity) -> f32 {
        self.relationships.get(&(a, b))
            .or_else(|| self.relationships.get(&(b, a)))
            .copied()
            .unwrap_or(0.0) // Default neutral relationship
    }
    
    pub fn set_relationship(&mut self, a: Entity, b: Entity, strength: f32) {
        if strength.abs() < 0.01 {
            // Remove weak relationships to save memory
            self.relationships.remove(&(a, b));
            self.relationships.remove(&(b, a));
        } else {
            self.relationships.insert((a, b), strength);
            // Update connection lists for fast neighbor queries
            self.entity_connections.entry(a).or_default().push(b);
            self.entity_connections.entry(b).or_default().push(a);
        }
    }
}
```

## Performance Monitoring Integration

### Micro-Benchmarking Systems
```rust
#[derive(Resource)]
pub struct SystemProfiler {
    pub system_times: HashMap<String, Vec<f32>>,
    pub frame_budget: f32,
    pub current_frame_start: Instant,
}

impl SystemProfiler {
    pub fn start_system(&mut self, name: &str) -> SystemTimer {
        SystemTimer {
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

pub struct SystemTimer {
    name: String,
    start: Instant,
}

impl Drop for SystemTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_secs_f32() * 1000.0; // Convert to ms
        // Record timing (implement actual recording logic)
        info!("System {} took {:.2}ms", self.name, elapsed);
    }
}

// Usage in systems
fn expensive_ai_system(mut profiler: ResMut<SystemProfiler>) {
    let _timer = profiler.start_system("expensive_ai_system");
    
    // System logic here
    // Timer automatically records when dropped
}
```

### Adaptive Quality Scaling
```rust
#[derive(Resource)]
pub struct QualityScaler {
    pub target_fps: f32,
    pub current_fps: f32,
    pub quality_level: f32, // 0.0 to 1.0
}

impl QualityScaler {
    pub fn update(&mut self, frame_time: f32) {
        self.current_fps = 1.0 / frame_time;
        
        if self.current_fps < self.target_fps * 0.9 {
            // Reduce quality
            self.quality_level = (self.quality_level - 0.1).max(0.1);
        } else if self.current_fps > self.target_fps * 1.1 {
            // Increase quality
            self.quality_level = (self.quality_level + 0.05).min(1.0);
        }
    }
    
    pub fn should_skip_expensive_calculation(&self) -> bool {
        self.quality_level < 0.5
    }
    
    pub fn get_update_frequency_multiplier(&self) -> f32 {
        0.5 + (self.quality_level * 0.5) // 0.5x to 1.0x frequency
    }
}
```