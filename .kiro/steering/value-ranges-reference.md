---
inclusion: fileMatch
fileMatchPattern: "src/ai/**/*.rs"
---

# Value Ranges Reference for Artificial Society

## Core Principle: Meaningful Zero Points

The distinction between unipolar (0.0-1.0) and bipolar (-1.0 to 1.0) ranges is crucial for proper social dynamics:

- **Unipolar (0.0-1.0)**: Represents absence to presence of something (needs, stress, energy)
- **Bipolar (-1.0 to 1.0)**: Represents opposing states with meaningful neutral point (emotions, social expressions)

## Physiological Systems (Unipolar: 0.0-1.0)

### Basic Needs
```rust
pub struct Needs {
    pub hunger: Normalized<f32>,    // 0.0 = satisfied, 1.0 = starving
    pub energy: Normalized<f32>,    // 0.0 = exhausted, 1.0 = fully rested
    pub safety: Normalized<f32>,    // 0.0 = secure, 1.0 = terrified
    pub social: Normalized<f32>,    // 0.0 = fulfilled, 1.0 = lonely
}
```

### Stress and Arousal
```rust
pub struct StressSystem {
    pub acute_stress: Normalized<f32>,    // 0.0 = calm, 1.0 = panic
    pub chronic_load: Normalized<f32>,    // 0.0 = fresh, 1.0 = burned out
}
```

### Personality Traits (Big Five)
```rust
pub struct Personality {
    pub openness: Normalized<f32>,         // 0.0 = conventional, 1.0 = creative
    pub conscientiousness: Normalized<f32>, // 0.0 = impulsive, 1.0 = disciplined
    pub extraversion: Normalized<f32>,     // 0.0 = introverted, 1.0 = extraverted
    pub agreeableness: Normalized<f32>,    // 0.0 = competitive, 1.0 = cooperative
    pub neuroticism: Normalized<f32>,      // 0.0 = stable, 1.0 = volatile
}
```

## Emotional and Social Systems (Bipolar: -1.0 to 1.0)

### Emotional State (PAD Model)
```rust
pub struct EmotionalState {
    pub valence: f32,     // -1.0 = displeasure, 0.0 = neutral, 1.0 = pleasure
    pub arousal: f32,     // -1.0 = calm/sleepy, 0.0 = neutral, 1.0 = excited/alert
    pub dominance: f32,   // -1.0 = submissive, 0.0 = neutral, 1.0 = dominant
}
```

### Social Expression Vector
```rust
pub struct ApparentStateVector {
    pub tension_relaxation: f32,    // -1.0 = relaxed, 0.0 = neutral, 1.0 = tense
    pub openness_closure: f32,      // -1.0 = closed, 0.0 = neutral, 1.0 = open
    pub dominance_submission: f32,  // -1.0 = submissive, 0.0 = neutral, 1.0 = dominant
    pub focus_distraction: f32,     // -1.0 = distracted, 0.0 = neutral, 1.0 = focused
}
```

### Social Relationships
```rust
pub struct RelationshipComponent {
    pub trust_level: f32,           // -1.0 = distrust, 0.0 = neutral, 1.0 = trust
    pub emotional_attachment: f32,  // -1.0 = aversion, 0.0 = neutral, 1.0 = attachment
    pub perceived_status: f32,      // -1.0 = low status, 0.0 = equal, 1.0 = high status
}
```

## Conversion and Validation Patterns

### Unipolar to Bipolar Conversion
```rust
// Convert need satisfaction (0.0-1.0) to mood valence (-1.0 to 1.0)
fn needs_to_valence(average_need_satisfaction: f32) -> f32 {
    (2.0 * average_need_satisfaction - 1.0).clamp(-1.0, 1.0)
}

// Example: 0.0 needs (all satisfied) -> 1.0 valence (very positive)
// Example: 0.5 needs (moderate) -> 0.0 valence (neutral)
// Example: 1.0 needs (all critical) -> -1.0 valence (very negative)
```

### Bipolar Value Validation
```rust
fn validate_bipolar(value: f32, name: &str) -> Result<f32, String> {
    if value >= -1.0 && value <= 1.0 {
        Ok(value)
    } else {
        Err(format!("{} must be between -1.0 and 1.0, got {}", name, value))
    }
}
```

### Personality Modulation of Bipolar Values
```rust
fn apply_personality_bias(
    base_value: f32,           // -1.0 to 1.0
    personality_trait: f32,    // 0.0 to 1.0
    influence_strength: f32,   // 0.0 to 1.0
) -> f32 {
    let bias = (personality_trait - 0.5) * 2.0; // Convert to -1.0 to 1.0
    let adjustment = bias * influence_strength;
    (base_value + adjustment).clamp(-1.0, 1.0)
}
```

## Threshold Definitions

### Emotional State Thresholds
- **Strong Positive**: valence > 0.6
- **Moderate Positive**: valence > 0.3
- **Neutral**: -0.3 <= valence <= 0.3
- **Moderate Negative**: valence < -0.3
- **Strong Negative**: valence < -0.6

### Arousal Thresholds
- **High Arousal**: arousal > 0.5
- **Moderate Arousal**: arousal > 0.2
- **Low Arousal**: arousal < -0.2
- **Very Low Arousal**: arousal < -0.5

### Social Expression Thresholds
- **Clearly Observable**: |expression_value| > 0.4
- **Subtly Observable**: |expression_value| > 0.2
- **Barely Noticeable**: |expression_value| > 0.1
- **Neutral/Ambiguous**: |expression_value| <= 0.1

## Scientific Basis for Ranges

### PAD Model (Mehrabian & Russell, 1974)
- Valence: Pleasure-Displeasure dimension
- Arousal: Activation-Deactivation dimension  
- Dominance: Control-Submission dimension
- All three dimensions are inherently bipolar with meaningful neutral points

### Big Five Personality Model
- Traits represent continuous distributions in population
- 0.0-1.0 range represents percentile within normal distribution
- Most individuals cluster around 0.5 (population mean)
- Extreme values (< 0.2 or > 0.8) represent notable personality characteristics

### Maslow's Hierarchy of Needs
- Needs represent deficiency states (absence to critical)
- 0.0 = need fully satisfied, no motivation from this need
- 1.0 = need critically unmet, dominates behavior
- Unipolar scale appropriate as needs don't have "negative" states

## Implementation Guidelines

### Component Design
```rust
// CORRECT: Bipolar emotional state
#[derive(Component)]
pub struct EmotionalState {
    pub valence: f32,  // -1.0 to 1.0, validated in setters
}

// INCORRECT: Unipolar emotional state
pub struct EmotionalState {
    pub happiness: Normalized<f32>,  // 0.0 to 1.0 - loses negative emotions!
}
```

### System Processing
```rust
// CORRECT: Respects bipolar nature
fn process_social_interaction(valence: f32) -> f32 {
    match valence {
        v if v > 0.3 => v * 1.2,      // Amplify positive
        v if v < -0.3 => v * 1.2,     // Amplify negative  
        v => v * 0.8,                 // Dampen neutral
    }
}

// INCORRECT: Treats as unipolar
fn process_social_interaction(valence: f32) -> f32 {
    valence * 1.2  // Doesn't account for negative values properly
}
```