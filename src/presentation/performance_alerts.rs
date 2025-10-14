//! Lightweight performance monitoring for the Artificial Society simulation.
//!
//! This module provides performance monitoring that only activates when there are
//! agents in the simulation to avoid false alerts during development.
//!
//! # Performance Targets:
//! - **60 FPS Target**: 16.67ms frame time budget
//! - **Agent Scaling**: Monitor performance with agent count >10
//! - **Alert Cooldowns**: Prevent spam with reasonable cooldown periods
//!
//! # Design Philosophy:
//! - Only monitor when simulation is actually running (agents present)
//! - Minimal overhead monitoring system
//! - Clear, actionable alerts for developers

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::collections::VecDeque;

/// Performance alert events for simulation monitoring.
#[derive(Event, Debug, Clone)]
pub enum PerformanceAlert {
    HighFrameTime {
        current_ms: f32,
        target_ms: f32,
        agent_count: u32,
    },
    LowFpsDrops {
        current_fps: f32,
        target_fps: f32,
        agent_count: u32,
    },
    HighAgentCount {
        agent_count: u32,
        performance_impact: f32,
    },
}

/// Minimum agent count before performance monitoring activates
const MIN_AGENTS_FOR_MONITORING: u32 = 10;

/// Performance monitoring configuration
#[derive(Resource, Debug)]
pub struct PerformanceMonitorConfig {
    pub target_frame_time_ms: f32,
    pub target_fps: f32,
    pub monitoring_interval_ms: u64,
    pub alert_cooldown_ms: u64,
}

impl Default for PerformanceMonitorConfig {
    fn default() -> Self {
        Self {
            target_frame_time_ms: 16.67, // 60 FPS target
            target_fps: 60.0,
            monitoring_interval_ms: 1000, // Check every second
            alert_cooldown_ms: 10000,     // 10 second cooldown between alerts
        }
    }
}

/// Performance monitoring state
#[derive(Resource, Debug)]
pub struct PerformanceMonitorState {
    pub recent_frame_times: VecDeque<f32>,
    pub recent_fps_values: VecDeque<f32>,
    pub last_monitoring_time: f64,
    pub last_alert_time: f64,
    pub monitoring_active: bool,
}

impl Default for PerformanceMonitorState {
    fn default() -> Self {
        Self {
            recent_frame_times: VecDeque::with_capacity(10),
            recent_fps_values: VecDeque::with_capacity(10),
            last_monitoring_time: 0.0,
            last_alert_time: 0.0,
            monitoring_active: false,
        }
    }
}

/// Plugin that adds lightweight performance monitoring
pub struct PerformanceAlertsPlugin;

impl Plugin for PerformanceAlertsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerformanceAlert>()
            .init_resource::<PerformanceMonitorConfig>()
            .init_resource::<PerformanceMonitorState>()
            .add_systems(
                Update,
                (performance_monitoring_system, alert_handling_system),
            );
    }
}

/// Main performance monitoring system - only monitors when agents are present
fn performance_monitoring_system(
    time: Res<Time>,
    config: Res<PerformanceMonitorConfig>,
    mut state: ResMut<PerformanceMonitorState>,
    diagnostics: Res<DiagnosticsStore>,
    mut alert_events: EventWriter<PerformanceAlert>,
    // Query for any entity that could be an agent (excluding cameras and UI)
    agents: Query<Entity, (Without<Camera>, Without<bevy::ui::Node>)>,
) {
    let current_time = time.elapsed_secs_f64();
    let agent_count = agents.iter().count() as u32;

    // Only monitor if we have enough agents to make monitoring meaningful
    if agent_count < MIN_AGENTS_FOR_MONITORING {
        if state.monitoring_active {
            info!(
                "Performance monitoring deactivated - agent count below threshold ({} < {})",
                agent_count, MIN_AGENTS_FOR_MONITORING
            );
            state.monitoring_active = false;
        }
        return;
    }

    // Activate monitoring if we have enough agents
    if !state.monitoring_active {
        info!(
            "Performance monitoring activated - {} agents detected",
            agent_count
        );
        state.monitoring_active = true;
    }

    // Check if it's time to monitor (based on monitoring interval)
    if (current_time - state.last_monitoring_time) * 1000.0 < config.monitoring_interval_ms as f64 {
        return;
    }

    state.last_monitoring_time = current_time;

    // Monitor frame performance with agent context
    monitor_frame_performance(
        &config,
        &mut state,
        &diagnostics,
        &mut alert_events,
        current_time,
        agent_count,
    );
}

/// Monitor frame performance metrics with agent context
fn monitor_frame_performance(
    config: &PerformanceMonitorConfig,
    state: &mut PerformanceMonitorState,
    diagnostics: &DiagnosticsStore,
    alert_events: &mut EventWriter<PerformanceAlert>,
    current_time: f64,
    agent_count: u32,
) {
    // Check if enough time has passed since last alert
    if (current_time - state.last_alert_time) * 1000.0 < config.alert_cooldown_ms as f64 {
        return;
    }

    // Monitor frame time
    if let Some(frame_time_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_ms) = frame_time_diag.smoothed() {
            let frame_time_ms_f32 = frame_time_ms as f32;

            state.recent_frame_times.push_back(frame_time_ms_f32);
            if state.recent_frame_times.len() > 10 {
                state.recent_frame_times.pop_front();
            }

            // Alert if frame time is consistently high
            if frame_time_ms_f32 > config.target_frame_time_ms * 1.5 {
                let high_frame_count = state
                    .recent_frame_times
                    .iter()
                    .filter(|&&time| time > config.target_frame_time_ms * 1.2)
                    .count();

                if high_frame_count >= 5 {
                    // 5 out of last 10 frames are slow
                    alert_events.write(PerformanceAlert::HighFrameTime {
                        current_ms: frame_time_ms_f32,
                        target_ms: config.target_frame_time_ms,
                        agent_count,
                    });
                    state.last_alert_time = current_time;
                }
            }
        }
    }

    // Monitor FPS
    if let Some(fps_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(current_fps) = fps_diag.smoothed() {
            let current_fps_f32 = current_fps as f32;

            state.recent_fps_values.push_back(current_fps_f32);
            if state.recent_fps_values.len() > 10 {
                state.recent_fps_values.pop_front();
            }

            // Alert if FPS is consistently low
            if current_fps_f32 < config.target_fps * 0.8 {
                let low_fps_count = state
                    .recent_fps_values
                    .iter()
                    .filter(|&&fps| fps < config.target_fps * 0.9)
                    .count();

                if low_fps_count >= 5 {
                    // 5 out of last 10 measurements are low
                    alert_events.write(PerformanceAlert::LowFpsDrops {
                        current_fps: current_fps_f32,
                        target_fps: config.target_fps,
                        agent_count,
                    });
                    state.last_alert_time = current_time;
                }
            }
        }
    }

    // Alert if agent count is getting high
    if agent_count > 100 {
        let performance_impact = (agent_count as f32 / 100.0 - 1.0) * 100.0;
        if performance_impact > 50.0 {
            alert_events.write(PerformanceAlert::HighAgentCount {
                agent_count,
                performance_impact,
            });
            state.last_alert_time = current_time;
        }
    }
}

/// System that handles performance alerts by logging them appropriately
fn alert_handling_system(mut alert_events: EventReader<PerformanceAlert>) {
    for alert in alert_events.read() {
        match alert {
            PerformanceAlert::HighFrameTime {
                current_ms,
                target_ms,
                agent_count,
            } => {
                warn!(
                    "Performance Alert: High frame time {:.2}ms (target: {:.2}ms) with {} agents",
                    current_ms, target_ms, agent_count
                );
            }
            PerformanceAlert::LowFpsDrops {
                current_fps,
                target_fps,
                agent_count,
            } => {
                warn!(
                    "Performance Alert: Low FPS {:.1} (target: {:.1}) with {} agents",
                    current_fps, target_fps, agent_count
                );
            }
            PerformanceAlert::HighAgentCount {
                agent_count,
                performance_impact,
            } => {
                warn!(
                    "Performance Alert: High agent count {} causing {:.1}% performance impact",
                    agent_count, performance_impact
                );
            }
        }
    }
}
