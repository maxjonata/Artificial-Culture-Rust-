# ⚠️ IMPLEMENTATION GUIDE - CURRENT DEVELOPMENT

**Status**: This document provides current implementation guidance for the `Normalized` type system and is aligned with the main specification.

**For Current Development**: This guide supports the **Primary Documentation Sources**:
- `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md`
- `Docs/Fundaments/DETAILED_ROADMAP.md`

---

# Type-Safe Normalized Values Guide

## Overview

The `Normalized` type is a zero-cost wrapper around `f32` that enforces the [0.0, 1.0] range constraint at the type level. This prevents invalid states from being stored in your components, eliminating a whole class of bugs while maintaining optimal performance.

## Why Use Normalized Instead of Raw f32?

### ❌ Problems with Raw f32

```rust
#[derive(Component)]
pub struct Needs {
    pub hunger: f32,  // Can be set to -5.0 or 10.0 by mistake
    pub energy: f32,  // No compile-time protection
}

// Easy to introduce bugs:
let mut needs = Needs { hunger: 0.5, energy: 0.8 };
needs.hunger = 2.0;  // Invalid! But compiles fine
needs.energy = -1.0; // Invalid! But compiles fine

// Calculations can produce invalid results:
needs.hunger = needs.hunger + needs.energy; // Could be 1.3, invalid!
```

### ✅ Benefits of Normalized Type

```rust
#[derive(Component)]
pub struct Needs {
    pub hunger: Normalized,  // Cannot store invalid values
    pub energy: Normalized,  // Type-level protection
}

// Type safety prevents bugs:
let mut needs = Needs {
    hunger: Normalized::new(0.5),
    energy: Normalized::new(0.8),
};

// All operations maintain bounds automatically:
needs.hunger += 1.0;  // Result: 1.0 (clamped)
needs.energy -= 2.0;  // Result: 0.0 (clamped)

// Arithmetic operations are safe:
let total = needs.hunger + needs.energy; // Always in [0.0, 1.0]
```

## Performance Characteristics

- **Zero-cost abstraction**: Same memory layout as `f32` (`#[repr(transparent)]`)
- **Inlined operations**: All methods marked with `#[inline]` for optimal performance
- **No heap allocations**: Stack-allocated value type
- **SIMD-friendly**: Compatible with vectorized operations
- **Cache-efficient**: Same size and alignment as `f32`

## Construction Methods

### Safe Construction (Recommended)

```rust
// Clamps automatically, never fails
let hunger = Normalized::new(1.5);    // → 1.0
let energy = Normalized::new(-0.2);   // → 0.0
let mood = Normalized::new(0.7);      // → 0.7
```

### Checked Construction

```rust
// Returns Result, useful when you need to know if clamping occurred
match Normalized::try_new(1.5) {
    Ok(value) => println!("Valid: {}", value.get()),
    Err(e) => println!("Out of range: {}", e),
}
```

### Unsafe Construction (Performance Critical)

```rust
// For when you know the value is valid (e.g., from quantized conversion)
let value = unsafe { Normalized::new_unchecked(0.5) };
```

### Constants

```rust
let zero = Normalized::ZERO;           // 0.0
let one = Normalized::ONE;             // 1.0
let half = Normalized::HALF;           // 0.5
let quarter = Normalized::QUARTER;     // 0.25
let three_quarters = Normalized::THREE_QUARTERS; // 0.75
```

## Arithmetic Operations

All arithmetic operations automatically maintain the [0.0, 1.0] bounds:

```rust
let a = Normalized::new(0.8);
let b = Normalized::new(0.7);

// Addition (clamped)
let sum = a + b;           // 1.0 (clamped from 1.5)
let sum_f32 = a + 0.5;     // Works with f32 too

// Subtraction (clamped)
let diff = a - b;          // 0.1
let diff_negative = b - a; // 0.0 (clamped from -0.1)

// Multiplication
let product = a * b;       // 0.56

// Division (safe)
let quotient = a / b;      // ~1.14 → 1.0 (clamped)
let div_by_zero = a / 0.0; // 1.0 (safe fallback)

// Assignment operations
let mut value = Normalized::new(0.3);
value += 0.2;  // 0.5
value *= 2.0;  // 1.0 (clamped)
value -= 0.5;  // 0.5
```

## Integration with Bevy

### Component Registration

```rust
// In your plugin's build method:
app.register_type::<Normalized>();
```

### Reflection Support

```rust
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct AgentState {
    pub mood: Normalized,
    pub energy: Normalized,
    pub stress: Normalized,
}
```

### Inspector Integration

The `Normalized` type works seamlessly with `bevy-inspector-egui`:

```rust
// Values are displayed as sliders with 0.0-1.0 range
// Edits are automatically clamped to valid bounds
```

## Conversion and Interoperability

### To/From f32

```rust
let normalized = Normalized::new(0.7);
let as_float: f32 = normalized.into();     // 0.7
let as_float2 = normalized.get();          // 0.7
let as_float3 = normalized.value();        // 0.7 (alias)
```

### Quantized Storage

```rust
// Convert to u8 for memory-efficient storage
let normalized = Normalized::new(0.5);
let quantized: u8 = normalized.to_quantized();        // 127
let back = Normalized::from_quantized(quantized);     // ~0.498

// Direct conversion
let from_u8: Normalized = 200u8.into();  // ~0.784
let to_u8: u8 = normalized.into();       // 127
```

### Big Five Personality Conversion

```rust
// Convert from 1-5 scale to normalized
let openness_score = 4.2; // Big Five score
let openness = Normalized::new((openness_score - 1.0) / 4.0); // 0.8
```

## Common Usage Patterns

### Needs System

```rust
#[derive(Component, Reflect, Default)]
pub struct Needs {
    pub hunger: Normalized,
    pub thirst: Normalized,
    pub energy: Normalized,
    pub social: Normalized,
}

impl Needs {
    pub fn most_urgent(&self) -> Normalized {
        [self.hunger, self.thirst, self.energy, self.social]
            .iter()
            .max()
            .copied()
            .unwrap_or(Normalized::ZERO)
    }
    
    pub fn overall_satisfaction(&self) -> Normalized {
        // Inverse of average need level
        Normalized::ONE - ((self.hunger + self.thirst + self.energy + self.social) / 4.0)
    }
}
```

### Personality Traits

```rust
#[derive(Component, Reflect, Default)]
pub struct Personality {
    pub neuroticism: Normalized,
    pub agreeableness: Normalized,
    pub conscientiousness: Normalized,
    pub openness: Normalized,
    pub extraversion: Normalized,
}

impl Personality {
    pub fn emotional_stability(&self) -> Normalized {
        Normalized::ONE - self.neuroticism
    }
    
    pub fn social_confidence(&self) -> Normalized {
        (self.extraversion + self.agreeableness) / 2.0
    }
}
```

### System Implementation

```rust
fn stress_system(
    mut query: Query<(&mut StressLevel, &Personality, &Needs)>,
    time: Res<Time>,
) {
    let dt = Normalized::new(time.delta_seconds());
    
    for (mut stress, personality, needs) in query.iter_mut() {
        // Neurotic agents accumulate stress faster
        let stress_rate = personality.neuroticism * Normalized::new(0.1);
        
        // Unmet needs increase stress
        let need_stress = needs.most_urgent() * Normalized::new(0.05);
        
        // Apply stress (automatically clamped)
        stress.current += (stress_rate + need_stress) * dt;
        
        // Natural stress recovery
        let recovery = Normalized::new(0.02) * dt;
        stress.current -= recovery;
    }
}
```

## Migration Strategy

1. **Identify components** with f32 fields that should be [0.0, 1.0]
2. **Replace field types** with `Normalized`
3. **Update constructors** to use `Normalized::new()`
4. **Remove manual clamping** - it's now automatic
5. **Test thoroughly** - the type system will catch many issues

## Best Practices

- ✅ Use `Normalized::new()` for most construction
- ✅ Leverage constants like `Normalized::HALF`
- ✅ Let arithmetic operations handle clamping automatically
- ✅ Use `try_new()` when you need to detect out-of-range values
- ❌ Don't use `new_unchecked()` unless performance-critical and value is guaranteed valid
- ❌ Don't convert to f32 just to do arithmetic - work with Normalized directly

## Debugging and Inspection

```rust
// Debug output shows the inner value
println!("{:?}", normalized); // Normalized(0.7)

// Get the raw value for logging
info!("Hunger level: {:.2}", needs.hunger.get());

// Check for specific thresholds
if stress.current > Normalized::new(0.8) {
    warn!("Agent is highly stressed!");
}
```

This type-safe approach eliminates bounds-checking bugs while maintaining the performance characteristics essential for real-time simulation with 100+ agents at 60fps.
