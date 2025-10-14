# System Profiling Guide

## Overview

The System Profiler provides quantitative analysis to determine whether systems should use **event-driven** or **polling** approaches. It measures execution times, processes counts, and provides data-driven recommendations.

## Key Features

- **Automatic Performance Comparison**: Measures both event-driven and polling implementations
- **Statistical Analysis**: Provides confidence levels and performance differences
- **Multiple Integration Methods**: Macro, trait, and manual integration options
- **Real-time Recommendations**: Periodic reports with actionable insights
- **Comprehensive Metrics**: Execution time, entity count, events processed, efficiency

## Integration Methods

### 1. Macro Integration (Recommended)

```rust
use crate::profile_system;
use crate::presentation::system_profiler::{SystemExecutionMode, SystemProfiler};

fn my_system(
    mut agents: Query<&mut MyComponent>,
    mut profiler: ResMut<SystemProfiler>,
) {
    let entity_count = agents.iter().count() as u32;
    
    profile_system!(
        profiler,
        "my_system_name",
        SystemExecutionMode::EventDriven, // or Polling
        entity_count,
        0, // events_processed
        {
            // Your system logic here
            for mut agent in agents.iter_mut() {
                agent.update();
            }
        }
    );
}
```

### 2. Trait Integration

```rust
use crate::presentation::system_profiler::{ProfiledSystem, SystemExecutionMode};

fn my_system(
    agents: Query<&MyComponent>,
    mut profiler: ResMut<SystemProfiler>,
) {
    let entity_count = agents.iter().count() as u32;
    
    let result = ().with_profiling(
        &mut profiler,
        "my_system_name",
        SystemExecutionMode::Polling,
        entity_count,
        0,
        || {
            // Your system logic here
            agents.iter().count()
        },
    );
}
```

### 3. Manual Integration

```rust
use crate::presentation::system_profiler::{SystemExecutionMetrics, SystemExecutionMode};
use std::time::Instant;

fn my_system(
    agents: Query<&MyComponent>,
    mut profiler: ResMut<SystemProfiler>,
) {
    let start = Instant::now();
    let entity_count = agents.iter().count() as u32;
    
    // Your system logic here
    for agent in agents.iter() {
        // Process agent
    }
    
    // Record metrics
    profiler.record_execution(SystemExecutionMetrics {
        system_name: "my_system_name".to_string(),
        execution_mode: SystemExecutionMode::EventDriven,
        execution_time: start.elapsed(),
        timestamp: start,
        entity_count,
        events_processed: 0,
    });
}
```

## Comparing Event-Driven vs Polling

### Step 1: Implement Both Approaches

```rust
// Event-driven version
fn needs_decay_event_driven(
    mut events: EventReader<NeedDecayTrigger>,
    mut agents: Query<&mut Needs>,
    mut profiler: ResMut<SystemProfiler>,
) {
    let entity_count = agents.iter().count() as u32;
    let mut events_processed = 0;
    
    profile_system!(
        profiler,
        "needs_decay",
        SystemExecutionMode::EventDriven,
        entity_count,
        events_processed,
        {
            for event in events.read() {
                if let Ok(mut needs) = agents.get_mut(event.entity) {
                    needs.decay(event.amount);
                    events_processed += 1;
                }
            }
        }
    );
}

// Polling version
fn needs_decay_polling(
    time: Res<Time>,
    mut agents: Query<&mut Needs>,
    mut profiler: ResMut<SystemProfiler>,
) {
    let entity_count = agents.iter().count() as u32;
    let mut entities_processed = 0;
    
    profile_system!(
        profiler,
        "needs_decay",
        SystemExecutionMode::Polling,
        entity_count,
        entities_processed,
        {
            for mut needs in agents.iter_mut() {
                if needs.should_decay(time.delta_secs()) {
                    needs.decay_over_time(time.delta_secs());
                    entities_processed += 1;
                }
            }
        }
    );
}
```

### Step 2: Run Both Systems

Add both systems to your app and let them run for a while:

```rust
app.add_systems(Update, (
    needs_decay_event_driven,
    needs_decay_polling,
    // ... other systems
));
```

### Step 3: Get Recommendations

The profiler will automatically analyze performance and provide recommendations:

```rust
fn check_recommendations(profiler: Res<SystemProfiler>) {
    if let Some(comparison) = profiler.get_comparison("needs_decay") {
        if let Some(recommendation) = comparison.recommended_approach {
            info!(
                "Recommendation for needs_decay: Use {:?} ({:.1}% better, {:.1}% confidence)",
                recommendation,
                comparison.performance_difference_percent.abs(),
                comparison.confidence_level * 100.0
            );
        }
    }
}
```

## Understanding the Metrics

### Performance Metrics

- **Execution Time**: How long the system takes to run
- **Entity Count**: Number of entities processed
- **Events Processed**: Number of events handled (event-driven only)
- **Efficiency**: Entities processed per microsecond

### Recommendation Logic

The profiler recommends based on:

1. **Performance Difference**: If one approach is >5% faster, it's recommended
2. **Architectural Benefits**: Event-driven is preferred for <5% difference (better decoupling)
3. **Confidence Level**: Based on sample size and consistency

### Confidence Levels

- **High (>70%)**: Strong recommendation, act on it
- **Medium (40-70%)**: Moderate confidence, consider other factors
- **Low (<40%)**: Insufficient data, collect more samples

## Best Practices

### 1. System Naming

Use consistent, descriptive names:
```rust
// Good
"personality_development"
"social_interaction_processing"
"needs_decay"

// Bad
"system1"
"update"
"process"
```

### 2. Meaningful Metrics

Track relevant counts:
```rust
// For event-driven systems
events_processed = events.read().count();

// For polling systems  
entities_processed = agents.iter_mut()
    .filter(|agent| agent.needs_update())
    .count();
```

### 3. Fair Comparisons

Ensure both implementations:
- Process the same data
- Have similar complexity
- Run under similar conditions
- Use the same entity counts

### 4. Sample Size

- Collect at least 50-100 samples per approach
- Run for several minutes to get stable averages
- Consider different entity counts and load conditions

## Profiler Events

Control profiling via events:

```rust
fn control_profiling(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<SystemProfilerEvent>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        events.send(SystemProfilerEvent::PrintRecommendations);
    }
    
    if keyboard.just_pressed(KeyCode::R) {
        events.send(SystemProfilerEvent::GenerateReport);
    }
    
    if keyboard.just_pressed(KeyCode::C) {
        events.send(SystemProfilerEvent::ClearData);
    }
}
```

## Example Output

```
=== System Performance Report ===

System: needs_decay
  Event-Driven: 45.30μs avg (127 executions, 12.50 entities/exec)
  Polling: 67.80μs avg (89 executions, 100.00 entities/exec)
  Recommendation: EventDriven (33.2% difference, 85.4% confidence)

System: social_interaction
  Event-Driven: 123.45μs avg (45 executions, 8.20 entities/exec)
  Polling: 89.12μs avg (156 executions, 50.30 entities/exec)
  Recommendation: Polling (27.8% difference, 92.1% confidence)
```

## Performance Considerations

The profiler itself has minimal overhead:
- ~1-2μs per measurement
- Configurable history size (default: 1000 entries)
- Can be disabled in production builds
- Automatic cleanup of old data

## Performance Testing

### Running Performance Tests

Performance tests are separate from the profiler and should be run explicitly:

```bash
# Run all performance tests
cargo test --test system_performance_tests --release --ignored

# Run specific performance test
cargo test --test system_performance_tests test_event_driven_vs_polling_performance --release --ignored
```

### Available Performance Tests

1. **`test_event_driven_vs_polling_performance`**: Compares event-driven vs polling approaches
2. **`test_polling_frequency_comparison`**: Tests different polling intervals
3. **`test_system_profiler_overhead`**: Measures profiler overhead
4. **`test_profiler_memory_usage`**: Validates memory usage limits

### Integration with CI/CD

Add performance regression tests to your CI:

```yaml
# .github/workflows/performance.yml
- name: Run Performance Tests
  run: cargo test --test system_performance_tests --release --ignored
  
- name: Check Performance Regression
  run: |
    cargo test --test system_performance_tests test_system_profiler_overhead --release --ignored
```

## Production Usage

The profiler is designed for production use:

- **Minimal Overhead**: ~1-2μs per measurement
- **Configurable**: Can be disabled in release builds
- **Memory Efficient**: Automatic cleanup and size limits
- **Real-time Monitoring**: Monitor actual system performance

### Enabling in Production

```rust
// Only enable profiling in debug builds
#[cfg(debug_assertions)]
app.add_plugins(RealSystemProfilingPlugin);

// Or enable conditionally
if std::env::var("ENABLE_PROFILING").is_ok() {
    app.add_plugins(SystemProfilerPlugin);
}
```

This profiler enables data-driven decisions about system architecture, ensuring optimal performance while maintaining code quality.