//! Tests for performance alert logging functionality.
//!
//! This module tests the JSON logging system and serialization for the performance monitoring system.

use bevy::prelude::*;
use serde_json;

use crate::presentation::performance_alerts::{
    PerformanceAlert, AlertSeverity
};

/// Helper to create a test alert
fn create_test_alert() -> PerformanceAlert {
    PerformanceAlert::HighFrameTime {
        current_ms: 25.0,
        target_ms: 16.67,
        fps_equivalent: 40.0,
    }
}

#[test]
fn test_alert_severity_serialization() {
    // Test all severity levels
    let severities = vec![
        AlertSeverity::Warning,
        AlertSeverity::Critical,
        AlertSeverity::Severe,
    ];

    for severity in severities {
        let json_result = serde_json::to_string(&severity);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        let deserialized: Result<AlertSeverity, _> = serde_json::from_str(&json_string);
        assert!(deserialized.is_ok());

        // Verify round-trip serialization
        match (severity, deserialized.unwrap()) {
            (AlertSeverity::Warning, AlertSeverity::Warning) => {},
            (AlertSeverity::Critical, AlertSeverity::Critical) => {},
            (AlertSeverity::Severe, AlertSeverity::Severe) => {},
            _ => panic!("Severity serialization mismatch"),
        }
    }
}

#[test]
fn test_performance_alert_serialization() {
    let alerts = vec![
        PerformanceAlert::HighFrameTime {
            current_ms: 25.0,
            target_ms: 16.67,
            fps_equivalent: 40.0,
        },
        PerformanceAlert::HighCpuUsage {
            current: 85.0,
            threshold: 80.0,
            duration_ms: 1500,
        },
        PerformanceAlert::HighMemoryUsage {
            current_mb: 14000.0,
            total_mb: 16384.0,
            percentage: 85.4,
            threshold: 85.0,
        },
        PerformanceAlert::LowFpsDrops {
            current_fps: 30.0,
            target_fps: 60.0,
            duration_ms: 2000,
        },
    ];
    
    for alert in alerts {
        let json_result = serde_json::to_string(&alert);
        assert!(json_result.is_ok(), "Failed to serialize alert: {:?}", alert);
        
        let json_string = json_result.unwrap();
        let deserialized: Result<PerformanceAlert, _> = serde_json::from_str(&json_string);
        assert!(deserialized.is_ok(), "Failed to deserialize alert JSON: {}", json_string);
    }
}
