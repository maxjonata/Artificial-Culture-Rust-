//! Performance tests for comparing event-driven vs polling system approaches.
//!
//! These tests are optional and should be run explicitly to measure system performance
//! characteristics and determine optimal architectural approaches.
//!
//! Run with: `cargo test --test system_performance_tests --release`

use artificial_culture_rust::presentation::system_profiler::{
    SystemExecutionMetrics, SystemExecutionMode, SystemProfiler, SystemProfilerPlugin,
};
use bevy::prelude::*;
use std::time::{Duration, Instant};

/// Test component for performance testing
#[derive(Component, Debug)]
struct TestAgent {
    value: f32,
    last_update: f32,
}

/// Test event for event-driven performance testing
#[derive(Event, Debug)]
struct TestAgentUpdate {
    entity: Entity,
    new_value: f32,
}

/// Event-driven system implementation for testing
fn test_system_event_driven(
    mut events: EventReader<TestAgentUpdate>,
    mut agents: Query<&mut TestAgent>,
    mut profiler: ResMut<SystemProfiler>,
) {
    if events.is_empty() {
        return;
    }

    let start = Instant::now();
    let mut events_processed = 0;
    let entity_count = agents.iter().count() as u32;

    for event in events.read() {
        if let Ok(mut agent) = agents.get_mut(event.entity) {
            agent.value = event.new_value;
            agent.last_update = 0.0;
            events_processed += 1;
        }
    }

    let execution_time = start.elapsed();
    profiler.record_execution(SystemExecutionMetrics {
        system_name: "test_agent_update".to_string(),
        execution_mode: SystemExecutionMode::EventDriven,
        execution_time,
        timestamp: start,
        entity_count,
        events_processed,
    });
}

/// Polling system implementation for testing
fn test_system_polling(
    time: Res<Time>,
    mut agents: Query<&mut TestAgent>,
    mut profiler: ResMut<SystemProfiler>,
    mut last_poll_time: Local<f32>,
) {
    let current_time = time.elapsed_secs();

    // Poll every 200ms
    if current_time - *last_poll_time < 0.2 {
        return;
    }
    *last_poll_time = current_time;

    let start = Instant::now();
    let mut entities_processed = 0;
    let entity_count = agents.iter().count() as u32;

    for mut agent in agents.iter_mut() {
        agent.last_update += time.delta_secs();

        if agent.last_update > 1.0 {
            agent.value += 0.1;
            agent.last_update = 0.0;
            entities_processed += 1;
        }
    }

    let execution_time = start.elapsed();
    profiler.record_execution(SystemExecutionMetrics {
        system_name: "test_agent_update".to_string(),
        execution_mode: SystemExecutionMode::Polling,
        execution_time,
        timestamp: start,
        entity_count,
        events_processed: entities_processed,
    });
}

/// Event generator for testing
fn generate_test_events(
    time: Res<Time>,
    agents: Query<Entity, With<TestAgent>>,
    mut events: EventWriter<TestAgentUpdate>,
    mut last_event_time: Local<f32>,
    mut event_count: Local<u32>,
) {
    let current_time = time.elapsed_secs();

    // Generate events every 1.5 seconds
    if current_time - *last_event_time > 1.5 {
        *last_event_time = current_time;

        // Send events for 10 random agents
        let agent_list: Vec<Entity> = agents.iter().collect();
        for _ in 0..10.min(agent_list.len()) {
            let random_index = (*event_count as usize) % agent_list.len();
            if let Some(&entity) = agent_list.get(random_index) {
                events.write(TestAgentUpdate {
                    entity,
                    new_value: rand::random::<f32>() * 100.0,
                });
            }
            *event_count += 1;
        }
    }
}

/// Create a test app with profiling systems
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(SystemProfilerPlugin)
        .add_event::<TestAgentUpdate>()
        .add_systems(
            Update,
            (
                test_system_event_driven,
                test_system_polling,
                generate_test_events,
            ),
        );

    // Spawn test agents
    for _ in 0..100 {
        app.world_mut().spawn(TestAgent {
            value: rand::random::<f32>() * 100.0,
            last_update: 0.0,
        });
    }

    app
}

#[test]
#[ignore] // Use --ignored to run performance tests
fn test_event_driven_vs_polling_performance() {
    let mut app = create_test_app();

    // Run for 10 seconds to collect performance data
    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(10) {
        app.update();
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    let profiler = app.world().resource::<SystemProfiler>();

    if let Some(comparison) = profiler.get_comparison("test_agent_update") {
        println!("=== Performance Test Results ===");

        if let Some(event_stats) = &comparison.event_driven_stats {
            println!(
                "Event-Driven: {:.2}μs avg, {} executions, {:.2} events/exec",
                event_stats.average_time.as_micros(),
                event_stats.total_executions,
                event_stats.average_events_processed
            );
        }

        if let Some(polling_stats) = &comparison.polling_stats {
            println!(
                "Polling: {:.2}μs avg, {} executions, {:.2} entities/exec",
                polling_stats.average_time.as_micros(),
                polling_stats.total_executions,
                polling_stats.average_entities_processed
            );
        }

        if let Some(recommendation) = comparison.recommended_approach {
            println!(
                "Recommendation: {:?} ({:.1}% difference, {:.1}% confidence)",
                recommendation,
                comparison.performance_difference_percent,
                comparison.confidence_level * 100.0
            );
        }

        // Assert that we collected meaningful data
        assert!(
            comparison.event_driven_stats.is_some(),
            "Event-driven stats should be collected"
        );
        assert!(
            comparison.polling_stats.is_some(),
            "Polling stats should be collected"
        );

        if let (Some(event_stats), Some(polling_stats)) =
            (&comparison.event_driven_stats, &comparison.polling_stats)
        {
            // Event-driven should have fewer executions but more events per execution
            assert!(
                event_stats.total_executions < polling_stats.total_executions,
                "Event-driven should have fewer total executions"
            );

            assert!(
                event_stats.average_events_processed > 1.0,
                "Event-driven should process multiple events per execution"
            );
        }
    } else {
        panic!("No performance comparison data collected");
    }
}

#[test]
#[ignore] // Use --ignored to run performance tests
fn test_polling_frequency_comparison() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(SystemProfilerPlugin);

    // Spawn test agents
    for _ in 0..50 {
        app.world_mut().spawn(TestAgent {
            value: rand::random::<f32>() * 100.0,
            last_update: 0.0,
        });
    }

    // Test different polling frequencies
    let frequencies = [
        ("fast_polling", 0.05),  // 50ms
        ("medium_polling", 0.2), // 200ms
        ("slow_polling", 1.0),   // 1000ms
    ];

    for (name, interval) in &frequencies {
        let mut last_poll = 0.0;
        let start_time = Instant::now();

        // Run for 5 seconds
        while start_time.elapsed() < Duration::from_secs(5) {
            app.update();

            let current_time = app.world().resource::<Time>().elapsed_secs();
            if current_time - last_poll >= *interval {
                last_poll = current_time;

                // Simulate polling system
                let system_start = Instant::now();
                let mut entities_processed = 0;

                let mut agents = app.world_mut().query::<&mut TestAgent>();
                for mut agent in agents.iter_mut(app.world_mut()) {
                    agent.last_update += *interval;
                    if agent.last_update > 1.0 {
                        agent.value += 0.1;
                        agent.last_update = 0.0;
                        entities_processed += 1;
                    }
                }

                let execution_time = system_start.elapsed();
                let mut profiler = app.world_mut().resource_mut::<SystemProfiler>();
                profiler.record_execution(SystemExecutionMetrics {
                    system_name: name.to_string(),
                    execution_mode: SystemExecutionMode::Polling,
                    execution_time,
                    timestamp: system_start,
                    entity_count: 50,
                    events_processed: entities_processed,
                });
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    }

    // Analyze results
    let profiler = app.world().resource::<SystemProfiler>();
    println!("=== Polling Frequency Comparison ===");

    for (name, interval) in &frequencies {
        if let Some(comparison) = profiler.get_comparison(name) {
            if let Some(stats) = &comparison.polling_stats {
                let frequency = 1.0 / interval;
                println!(
                    "{} ({:.1} Hz): {:.2}μs avg, {} executions, {:.2} entities/exec",
                    name,
                    frequency,
                    stats.average_time.as_micros(),
                    stats.total_executions,
                    stats.average_entities_processed
                );

                // Assert reasonable execution counts based on frequency
                let expected_executions = (5.0 / interval) as u32;
                let tolerance = expected_executions / 4; // 25% tolerance
                assert!(
                    stats.total_executions >= expected_executions - tolerance
                        && stats.total_executions <= expected_executions + tolerance,
                    "Execution count {} should be near expected {} for {}",
                    stats.total_executions,
                    expected_executions,
                    name
                );
            }
        }
    }
}

#[test]
#[ignore] // Use --ignored to run performance tests
fn test_system_profiler_overhead() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(SystemProfilerPlugin);

    // Spawn test agents
    for _ in 0..1000 {
        app.world_mut().spawn(TestAgent {
            value: rand::random::<f32>() * 100.0,
            last_update: 0.0,
        });
    }

    // Measure system execution without profiling
    let start_time = Instant::now();
    let mut agents = app.world_mut().query::<&mut TestAgent>();
    for mut agent in agents.iter_mut(app.world_mut()) {
        agent.value += 0.1;
    }
    let baseline_time = start_time.elapsed();

    // Measure system execution with profiling
    let profiled_start = Instant::now();
    let mut profiler = app.world_mut().resource_mut::<SystemProfiler>();
    profiler.record_execution(SystemExecutionMetrics {
        system_name: "overhead_test".to_string(),
        execution_mode: SystemExecutionMode::Polling,
        execution_time: baseline_time,
        timestamp: profiled_start,
        entity_count: 1000,
        events_processed: 0,
    });
    let profiled_time = profiled_start.elapsed();

    // Profiling overhead should be minimal (< 10μs)
    let overhead = profiled_time.saturating_sub(baseline_time);
    println!("Profiling overhead: {:.2}μs", overhead.as_micros());

    assert!(
        overhead < Duration::from_micros(10),
        "Profiling overhead should be less than 10μs, got {:.2}μs",
        overhead.as_micros()
    );
}

#[test]
#[ignore] // Use --ignored to run performance tests  
fn test_profiler_memory_usage() {
    let mut profiler = SystemProfiler::default();

    // Fill profiler with data
    for i in 0..2000 {
        profiler.record_execution(SystemExecutionMetrics {
            system_name: format!("test_system_{}", i % 10),
            execution_mode: if i % 2 == 0 {
                SystemExecutionMode::EventDriven
            } else {
                SystemExecutionMode::Polling
            },
            execution_time: Duration::from_micros(100 + (i % 50) as u64),
            timestamp: Instant::now(),
            entity_count: 100,
            events_processed: i % 20,
        });
    }

    // Check that history is properly limited
    assert!(
        profiler.metrics_history.len() <= profiler.max_history_size,
        "History size should be limited to max_history_size"
    );

    // Check that we have reasonable number of system stats
    assert!(
        profiler.system_stats.len() <= 20, // 10 systems * 2 modes
        "System stats should not grow unbounded"
    );

    println!("Memory usage test passed:");
    println!("  History entries: {}", profiler.metrics_history.len());
    println!("  System stats: {}", profiler.system_stats.len());
    println!("  Comparisons: {}", profiler.comparisons.len());
}
