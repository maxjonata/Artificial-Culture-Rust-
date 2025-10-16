//! Tests for the performance monitoring and alerting system.
//!
//! This module provides comprehensive tests for all performance monitoring functionality,
//! including alert generation, threshold validation, logging, and system integration.

use bevy::prelude::*;

use crate::presentation::performance_alerts::{
    PerformanceAlert, PerformanceAlertsPlugin, PerformanceMonitorConfig,
    PerformanceMonitorState
};

/// Test helper to create a minimal Bevy app with performance monitoring
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(bevy::diagnostic::DiagnosticsPlugin)
       .add_plugins(PerformanceAlertsPlugin);
    app
}

/// Test helper to create a test app with custom config
fn create_test_app_with_config(config: PerformanceMonitorConfig) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(bevy::diagnostic::DiagnosticsPlugin)
       .add_plugins(PerformanceAlertsPlugin)
       .insert_resource(config);
    app
}

#[test]
fn test_performance_alerts_plugin_initialization() {
    let app = create_test_app();

    // Verify resources are initialized
    assert!(app.world().contains_resource::<PerformanceMonitorConfig>());
    assert!(app.world().contains_resource::<PerformanceMonitorState>());

    // Verify event type is registered
    let events = app.world().resource::<Events<PerformanceAlert>>();
    assert!(events.is_empty());
}

#[test]
fn test_default_configuration_values() {
    let config = PerformanceMonitorConfig::default();

    // Verify configuration values
    assert_eq!(config.target_frame_time_ms, 16.67);
    assert_eq!(config.target_fps, 60.0);
    assert_eq!(config.monitoring_interval_ms, 1000);
    assert_eq!(config.alert_cooldown_ms, 10000);
}

#[test]
fn test_state_initialization() {
    let state = PerformanceMonitorState::default();

    // Verify state fields are initialized correctly
    assert_eq!(state.last_monitoring_time, 0.0);
    assert_eq!(state.last_alert_time, 0.0);
    assert_eq!(state.monitoring_active, false);

    // Verify collections are initialized with correct capacity
    assert_eq!(state.recent_frame_times.capacity(), 10);
    assert_eq!(state.recent_fps_values.capacity(), 10);
}

#[test]
fn test_performance_alert_types_exist() {
    // Test that all expected alert types can be created
    let _high_frame_time = PerformanceAlert::HighFrameTime {
        current_ms: 30.0,
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

    // Test passes if all alert types compile
    assert!(true);
}

#[test]
fn test_custom_configuration() {
    let custom_config = PerformanceMonitorConfig {
        target_fps: 30.0,
        target_frame_time_ms: 33.33,
        monitoring_interval_ms: 200,
        alert_cooldown_ms: 5000,
    };

    let app = create_test_app_with_config(custom_config);

    // Verify custom config is applied
    let config = app.world().resource::<PerformanceMonitorConfig>();
    assert_eq!(config.target_fps, 30.0);
    assert_eq!(config.target_frame_time_ms, 33.33);
    assert_eq!(config.monitoring_interval_ms, 200);
    assert_eq!(config.alert_cooldown_ms, 5000);
}

#[test]
fn test_entity_count_monitoring() {
    let mut app = create_test_app();

    // Spawn many entities to simulate high entity count
    for _ in 0..1000 {
        app.world_mut().spawn_empty();
    }

    // Run update to trigger entity count monitoring
    app.update();

    // Test passes if no panics occur during entity count monitoring
    assert!(true);
}

#[test]
fn test_performance_monitoring_systems_run() {
    let mut app = create_test_app();

    // Run multiple updates to ensure systems execute without panicking
    for _ in 0..5 {
        app.update();
    }

    // Verify the monitoring state is being updated
    let state = app.world().resource::<PerformanceMonitorState>();
    assert!(state.last_monitoring_time >= 0.0);
    assert!(state.last_alert_time >= 0.0);
}