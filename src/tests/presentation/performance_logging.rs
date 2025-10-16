//! Tests for performance alert logging functionality.
//!
//! This module tests the performance monitoring system alert types.

use crate::presentation::performance_alerts::PerformanceAlert;

/// Helper to create a test alert
fn create_test_alert() -> PerformanceAlert {
    PerformanceAlert::HighFrameTime {
        current_ms: 25.0,
        target_ms: 16.67,
        agent_count: 15,
    }
}

#[test]
fn test_performance_alert_types_exist() {
    // Test that all expected alert types can be created
    let _high_frame_time = PerformanceAlert::HighFrameTime {
        current_ms: 25.0,
        target_ms: 16.67,
        agent_count: 15,
    };

    let _low_fps = PerformanceAlert::LowFpsDrops {
        current_fps: 30.0,
        target_fps: 60.0,
        agent_count: 20,
    };

    let _high_agent_count = PerformanceAlert::HighAgentCount {
        agent_count: 150,
        performance_impact: 75.0,
    };

    // Test passes if all alert types compile and can be created
    assert!(true);
}

#[test]
fn test_helper_function() {
    // Test that the helper function creates a valid alert
    let alert = create_test_alert();
    
    // Verify it's the correct variant
    match alert {
        PerformanceAlert::HighFrameTime { current_ms, target_ms, agent_count } => {
            assert_eq!(current_ms, 25.0);
            assert_eq!(target_ms, 16.67);
            assert_eq!(agent_count, 15);
        }
        _ => panic!("Helper function created wrong alert type"),
    }
}
