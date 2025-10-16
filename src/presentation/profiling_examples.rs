//! Usage examples for the system profiler.
//!
//! This module shows how to integrate the profiler with real systems.
//! The actual performance tests are in tests/system_performance_tests.rs

use crate::presentation::system_profiler::{SystemExecutionMode, SystemProfiler};
use bevy::prelude::*;

/// Example of profiling using the macro - replace with your actual system
pub fn example_macro_profiling() {
    // This is just a template - replace with your actual system
    /*
    pub fn your_event_driven_system(
        mut events: EventReader<YourEvent>,
        mut agents: Query<&mut YourComponent>,
        mut profiler: ResMut<SystemProfiler>,
    ) {
        if events.is_empty() {
            return;
        }

        let entity_count = agents.iter().count() as u32;
        let events_processed = events.len() as u32;

        crate::profile_system!(
            profiler,
            "your_system_name",
            SystemExecutionMode::EventDriven,
            entity_count,
            events_processed,
            {
                // Your actual system logic here
                for event in events.read() {
                    // Process events
                }
            }
        );
    }
    */
}

/// Example of profiling using polling - replace with your actual system
pub fn example_polling_profiling() {
    // This is just a template - replace with your actual system
    /*
    pub fn your_polling_system(
        time: Res<Time>,
        mut agents: Query<&mut YourComponent>,
        mut profiler: ResMut<SystemProfiler>,
        mut last_poll_time: Local<f32>,
    ) {
        let current_time = time.elapsed_secs();

        // Poll every 500ms
        if current_time - *last_poll_time < 0.5 {
            return;
        }
        *last_poll_time = current_time;

        let entity_count = agents.iter().count() as u32;

        crate::profile_system!(
            profiler,
            "your_system_name",
            SystemExecutionMode::Polling,
            entity_count,
            0,
            {
                let mut entities_processed = 0;

                for mut component in agents.iter_mut() {
                    // Your polling logic here
                    entities_processed += 1;
                }

                entities_processed
            }
        );
    }
    */
}

/// Example of using the trait-based profiling
pub fn example_trait_profiling() {
    // This is just a template - replace with your actual system
    /*
    pub fn your_system_with_trait(
        agents: Query<&YourComponent>,
        mut profiler: ResMut<SystemProfiler>,
    ) {
        use crate::presentation::system_profiler::ProfiledSystem;

        let entity_count = agents.iter().count() as u32;

        let _result = ().with_profiling(
            &mut profiler,
            "your_system_name",
            SystemExecutionMode::EventDriven,
            entity_count,
            0,
            || {
                // Your actual system logic
                let mut processed = 0;

                for _agent in agents.iter() {
                    // Process agents
                    processed += 1;
                }

                processed
            },
        );
    }
    */
}

/// System that prints profiling results for real systems
pub fn print_real_system_profiling_results(
    time: Res<Time>,
    profiler: Res<SystemProfiler>,
    mut last_print_time: Local<f32>,
) {
    let current_time = time.elapsed_secs();

    // Print results every 60 seconds
    if current_time - *last_print_time > 60.0 {
        *last_print_time = current_time;

        let recommendations = profiler.get_recommendations();
        if !recommendations.is_empty() {
            info!("=== Real System Performance Recommendations ===");
            for comp in recommendations {
                if comp.confidence_level > 0.7 {
                    // Only show high-confidence recommendations
                    if let Some(rec) = comp.recommended_approach {
                        info!(
                            "{}: Use {:?} ({:.1}% performance difference, {:.1}% confidence)",
                            comp.system_name,
                            rec,
                            comp.performance_difference_percent.abs(),
                            comp.confidence_level * 100.0
                        );
                    }
                }
            }
        }
    }
}

/// Plugin that adds profiling to real systems (optional)
/// Enable this when you want to profile your actual systems
pub struct RealSystemProfilingPlugin;

impl Plugin for RealSystemProfilingPlugin {
    fn build(&self, app: &mut App) {
        // Only add in debug builds and when explicitly enabled
        #[cfg(debug_assertions)]
        {
            app.add_systems(
                Update,
                (
                    // Add your actual profiled systems here
                    // your_event_driven_system,
                    // your_polling_system,

                    // Results printing
                    print_real_system_profiling_results,
                ),
            );
        }
    }
}
