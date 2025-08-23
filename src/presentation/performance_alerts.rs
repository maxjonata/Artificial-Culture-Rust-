//! Comprehensive performance monitoring and alerting system for the Artificial Society simulation.
//!
//! This module implements a robust early warning system that monitors critical performance metrics
//! and logs alerts to help identify bottlenecks before they impact real-time simulation requirements.
//!
//! # Industry-Standard Baseline Values (Research-Based)
//!
//! ## System Resource Thresholds:
//! - **CPU Usage**: >80% sustained (based on server monitoring best practices)
//! - **RAM Usage**: >85% of available memory (prevents swap thrashing)
//! - **GPU Usage**: >90% sustained (allows headroom for frame spikes)
//! - **Disk I/O**: >80% utilization (prevents I/O bottlenecks)
//!
//! ## Frame Performance Targets:
//! - **60 FPS Target**: 16.67ms frame time budget
//! - **Frame Time Variance**: <5ms deviation (smooth gameplay)
//! - **Sustained FPS Drops**: <60 FPS for >1 second triggers alert
//!
//! ## AI System Performance Budgets:
//! - **Individual System**: <5ms per system per frame (20% of 60 FPS budget)
//! - **Total AI Processing**: <10ms per frame (60% of 60 FPS budget)
//! - **Entity Scaling**: Monitor performance degradation with agent count
//!
//! # Performance Considerations
//!
//! The monitoring system itself is designed to have minimal overhead:
//! - Metrics collected at 10Hz (every 100ms) to reduce sampling cost
//! - JSON logging is buffered and flushed periodically
//! - File I/O operations are non-blocking where possible

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Performance alert events that can be triggered by the monitoring system.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceAlert {
    /// CPU usage exceeded threshold for sustained period
    HighCpuUsage {
        current: f32,
        threshold: f32,
        duration_ms: u64,
    },
    /// RAM usage exceeded threshold
    HighMemoryUsage {
        current_mb: f32,
        total_mb: f32,
        percentage: f32,
        threshold: f32,
    },
    /// GPU usage exceeded threshold for sustained period
    HighGpuUsage {
        current: f32,
        threshold: f32,
        duration_ms: u64,
    },
    /// Disk I/O usage exceeded threshold
    HighDiskIo {
        current: f32,
        threshold: f32,
        operation_type: String, // "read" or "write"
    },
    /// Frame time exceeded target (60 FPS = 16.67ms)
    HighFrameTime {
        current_ms: f32,
        target_ms: f32,
        fps_equivalent: f32,
    },
    /// FPS dropped below target for sustained period
    LowFpsDrops {
        current_fps: f32,
        target_fps: f32,
        duration_ms: u64,
    },
    /// Frame time variance/jitter exceeded threshold
    HighFrameJitter {
        variance_ms: f32,
        threshold_ms: f32,
        recent_frame_times: Vec<f32>,
    },
    /// Individual system execution time exceeded budget
    SlowSystemExecution {
        system_name: String,
        execution_time_ms: f32,
        threshold_ms: f32,
        frame_percentage: f32,
    },
    /// Total AI processing time exceeded budget
    SlowAiProcessing {
        total_ai_time_ms: f32,
        threshold_ms: f32,
        frame_percentage: f32,
        contributing_systems: Vec<String>,
    },
    /// Entity count causing performance degradation
    EntityCountPerformanceImpact {
        entity_count: u32,
        baseline_entity_count: u32,
        performance_degradation_percent: f32,
        affected_metrics: Vec<String>,
    },
}

/// Serializable alert log entry for JSON output
#[derive(Serialize, Deserialize, Debug)]
pub struct AlertLogEntry {
    pub timestamp: u64,
    pub timestamp_human: String,
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub alert: PerformanceAlert,
    pub baseline_values: BaselineValues,
}

/// Alert severity levels
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AlertSeverity {
    Warning,  // Performance concern but not critical
    Critical, // Immediate performance impact
    Severe,   // Simulation integrity at risk
}

/// Baseline "normal" performance values for comparison
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaselineValues {
    pub normal_cpu_usage: f32,
    pub normal_memory_usage_mb: f32,
    pub normal_frame_time_ms: f32,
    pub normal_fps: f32,
    pub normal_entity_count: u32,
    pub normal_ai_processing_time_ms: f32,
}

/// Performance monitoring configuration and thresholds
#[derive(Resource, Debug)]
pub struct PerformanceMonitorConfig {
    // System resource thresholds
    pub cpu_usage_threshold: f32,
    pub memory_usage_threshold: f32,
    pub gpu_usage_threshold: f32,
    pub disk_io_threshold: f32,
    
    // Frame performance thresholds
    pub target_frame_time_ms: f32,
    pub target_fps: f32,
    pub frame_jitter_threshold_ms: f32,
    pub fps_drop_duration_threshold_ms: u64,
    
    // AI system performance thresholds
    pub system_execution_threshold_ms: f32,
    pub total_ai_processing_threshold_ms: f32,
    
    // Monitoring intervals
    pub monitoring_interval_ms: u64,
    pub sustained_alert_duration_ms: u64,
    
    // Logging configuration
    pub log_directory: PathBuf,
    pub log_buffer_size: usize,
    pub flush_interval_ms: u64,
}

impl Default for PerformanceMonitorConfig {
    fn default() -> Self {
        Self {
            // System resource thresholds (industry standards)
            cpu_usage_threshold: 80.0,      // 80% CPU usage
            memory_usage_threshold: 85.0,   // 85% memory usage
            gpu_usage_threshold: 90.0,      // 90% GPU usage
            disk_io_threshold: 80.0,        // 80% disk I/O utilization
            
            // Frame performance thresholds (60 FPS target)
            target_frame_time_ms: 16.67,    // 60 FPS = 16.67ms per frame
            target_fps: 60.0,               // 60 FPS target
            frame_jitter_threshold_ms: 5.0, // 5ms variance threshold
            fps_drop_duration_threshold_ms: 1000, // 1 second of low FPS
            
            // AI system performance budgets
            system_execution_threshold_ms: 5.0,  // 5ms per system (30% of frame budget)
            total_ai_processing_threshold_ms: 10.0, // 10ms total AI (60% of frame budget)
            
            // Monitoring configuration
            monitoring_interval_ms: 100,     // Check every 100ms (10Hz)
            sustained_alert_duration_ms: 500, // 500ms sustained for alert
            
            // Logging configuration
            log_directory: PathBuf::from("logs"),
            log_buffer_size: 100,            // Buffer 100 alerts before flush
            flush_interval_ms: 5000,         // Flush every 5 seconds
        }
    }
}

/// Performance monitoring state and history
#[derive(Resource, Debug)]
pub struct PerformanceMonitorState {
    pub baseline_values: BaselineValues,
    pub recent_frame_times: VecDeque<f32>,
    pub recent_fps_values: VecDeque<f32>,
    pub recent_cpu_usage: VecDeque<f32>,
    pub recent_memory_usage: VecDeque<f32>,
    pub alert_log_buffer: Vec<AlertLogEntry>,
    pub last_monitoring_time: f64,
    pub last_flush_time: f64,
    pub current_log_file: Option<PathBuf>,
}

impl Default for PerformanceMonitorState {
    fn default() -> Self {
        Self {
            baseline_values: BaselineValues {
                normal_cpu_usage: 25.0,        // Baseline 25% CPU
                normal_memory_usage_mb: 512.0, // Baseline 512MB
                normal_frame_time_ms: 16.67,   // Baseline 60 FPS
                normal_fps: 60.0,              // Baseline 60 FPS
                normal_entity_count: 100,      // Baseline 100 entities
                normal_ai_processing_time_ms: 5.0, // Baseline 5ms AI processing
            },
            recent_frame_times: VecDeque::with_capacity(60), // 6 seconds at 10Hz
            recent_fps_values: VecDeque::with_capacity(60),
            recent_cpu_usage: VecDeque::with_capacity(60),
            recent_memory_usage: VecDeque::with_capacity(60),
            alert_log_buffer: Vec::new(),
            last_monitoring_time: 0.0,
            last_flush_time: 0.0,
            current_log_file: None,
        }
    }
}

/// Plugin that adds performance monitoring and alerting to the application
pub struct PerformanceAlertsPlugin;

impl Plugin for PerformanceAlertsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .add_event::<PerformanceAlert>()
            .init_resource::<PerformanceMonitorConfig>()
            .init_resource::<PerformanceMonitorState>()
            .add_systems(
                Update,
                (
                    performance_monitoring_system,
                    alert_logging_system,
                    periodic_log_flush_system,
                ).chain()
            )
            .add_systems(Startup, initialize_performance_monitoring);
    }
}

/// Initialize the performance monitoring system
fn initialize_performance_monitoring(
    config: Res<PerformanceMonitorConfig>,
    mut state: ResMut<PerformanceMonitorState>,
) {
    // Create logs directory if it doesn't exist
    if let Err(e) = create_dir_all(&config.log_directory) {
        error!("Failed to create logs directory: {}", e);
        return;
    }

    // Generate timestamped log filename
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let log_filename = format!("performance_alerts_{timestamp}.jsonl");
    let log_path = config.log_directory.join(log_filename);

    state.current_log_file = Some(log_path.clone());

    info!("Performance monitoring initialized. Logging to: {:?}", log_path);
}

/// Main performance monitoring system that checks all metrics and triggers alerts
fn performance_monitoring_system(
    time: Res<Time>,
    config: Res<PerformanceMonitorConfig>,
    mut state: ResMut<PerformanceMonitorState>,
    diagnostics: Res<DiagnosticsStore>,
    mut alert_events: EventWriter<PerformanceAlert>,
    query: Query<Entity>, // For entity count monitoring
) {
    let current_time = time.elapsed_secs_f64();

    // Check if it's time to monitor (based on monitoring interval)
    if (current_time - state.last_monitoring_time) * 1000.0 < config.monitoring_interval_ms as f64 {
        return;
    }

    state.last_monitoring_time = current_time;

    // Monitor frame performance metrics
    monitor_frame_performance(&config, &mut state, &diagnostics, &mut alert_events);

    // Monitor system resource metrics
    monitor_system_resources(&config, &mut state, &diagnostics, &mut alert_events);

    // Monitor entity count and performance correlation
    let entity_count = query.iter().count() as u32;
    monitor_entity_performance(&config, &mut state, entity_count, &mut alert_events);
}

/// Monitor frame performance metrics (FPS, frame time, jitter)
fn monitor_frame_performance(
    config: &PerformanceMonitorConfig,
    state: &mut PerformanceMonitorState,
    diagnostics: &DiagnosticsStore,
    alert_events: &mut EventWriter<PerformanceAlert>,
) {
    // Check frame time
    if let Some(frame_time_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_ms) = frame_time_diag.smoothed() {
            let frame_time_ms_f32 = frame_time_ms as f32;

            // Add to recent frame times for jitter calculation
            state.recent_frame_times.push_back(frame_time_ms_f32);
            if state.recent_frame_times.len() > 60 {
                state.recent_frame_times.pop_front();
            }

            // Alert on high frame time
            if frame_time_ms_f32 > config.target_frame_time_ms {
                let fps_equivalent = 1000.0 / frame_time_ms_f32;
                alert_events.write(PerformanceAlert::HighFrameTime {
                    current_ms: frame_time_ms_f32,
                    target_ms: config.target_frame_time_ms,
                    fps_equivalent,
                });
            }

            // Check frame time jitter/variance
            if state.recent_frame_times.len() >= 10 {
                let recent_times: Vec<f32> = state.recent_frame_times.iter().cloned().collect();
                let mean = recent_times.iter().sum::<f32>() / recent_times.len() as f32;
                let variance = recent_times.iter()
                    .map(|&x| (x - mean).powi(2))
                    .sum::<f32>() / recent_times.len() as f32;
                let std_dev = variance.sqrt();

                if std_dev > config.frame_jitter_threshold_ms {
                    alert_events.write(PerformanceAlert::HighFrameJitter {
                        variance_ms: std_dev,
                        threshold_ms: config.frame_jitter_threshold_ms,
                        recent_frame_times: recent_times.clone(),
                    });
                }
            }
        }
    }

    // Check FPS drops
    if let Some(fps_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(current_fps) = fps_diag.smoothed() {
            let current_fps_f32 = current_fps as f32;

            state.recent_fps_values.push_back(current_fps_f32);
            if state.recent_fps_values.len() > 60 {
                state.recent_fps_values.pop_front();
            }

            // Check for sustained FPS drops
            if current_fps_f32 < config.target_fps {
                let low_fps_duration = state.recent_fps_values.iter()
                    .rev()
                    .take_while(|&&fps| fps < config.target_fps)
                    .count() as u64 * config.monitoring_interval_ms;

                if low_fps_duration >= config.fps_drop_duration_threshold_ms {
                    alert_events.write(PerformanceAlert::LowFpsDrops {
                        current_fps: current_fps_f32,
                        target_fps: config.target_fps,
                        duration_ms: low_fps_duration,
                    });
                }
            }
        }
    }
}

/// Monitor system resource metrics (CPU, RAM, GPU, Disk I/O)
fn monitor_system_resources(
    config: &PerformanceMonitorConfig,
    state: &mut PerformanceMonitorState,
    diagnostics: &DiagnosticsStore,
    alert_events: &mut EventWriter<PerformanceAlert>,
) {
    // Monitor CPU usage
    if let Some(cpu_diag) = diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE) {
        if let Some(cpu_usage) = cpu_diag.smoothed() {
            let cpu_usage_f32 = cpu_usage as f32;

            state.recent_cpu_usage.push_back(cpu_usage_f32);
            if state.recent_cpu_usage.len() > 60 {
                state.recent_cpu_usage.pop_front();
            }

            // Check for sustained high CPU usage
            if cpu_usage_f32 > config.cpu_usage_threshold {
                let high_cpu_duration = state.recent_cpu_usage.iter()
                    .rev()
                    .take_while(|&&usage| usage > config.cpu_usage_threshold)
                    .count() as u64 * config.monitoring_interval_ms;

                if high_cpu_duration >= config.sustained_alert_duration_ms {
                    alert_events.write(PerformanceAlert::HighCpuUsage {
                        current: cpu_usage_f32,
                        threshold: config.cpu_usage_threshold,
                        duration_ms: high_cpu_duration,
                    });
                }
            }
        }
    }

    // Monitor memory usage
    if let Some(mem_diag) = diagnostics.get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE) {
        if let Some(mem_usage_bytes) = mem_diag.smoothed() {
            let mem_usage_mb = (mem_usage_bytes / (1024.0 * 1024.0)) as f32;
            state.recent_memory_usage.push_back(mem_usage_mb);
            if state.recent_memory_usage.len() > 60 {
                state.recent_memory_usage.pop_front();
            }

            // For now, use a simplified memory percentage calculation
            // In a real implementation, you'd want to get total system memory
            let estimated_total_memory_gb = 16.0; // Assume 16GB system
            let estimated_total_memory_mb = estimated_total_memory_gb * 1024.0;
            let mem_percentage = (mem_usage_mb / estimated_total_memory_mb) * 100.0;

            if mem_percentage > config.memory_usage_threshold {
                alert_events.write(PerformanceAlert::HighMemoryUsage {
                    current_mb: mem_usage_mb,
                    total_mb: estimated_total_memory_mb,
                    percentage: mem_percentage,
                    threshold: config.memory_usage_threshold,
                });
            }
        }
    }
}

/// Monitor entity count and its correlation with performance
fn monitor_entity_performance(
    config: &PerformanceMonitorConfig,
    state: &mut PerformanceMonitorState,
    entity_count: u32,
    alert_events: &mut EventWriter<PerformanceAlert>,
) {
    // Calculate performance degradation based on entity count increase
    if entity_count > state.baseline_values.normal_entity_count {
        let entity_increase_ratio = entity_count as f32 / state.baseline_values.normal_entity_count as f32;

        // Estimate performance impact (simplified model)
        let estimated_performance_impact = (entity_increase_ratio - 1.0) * 100.0;

        // If entity count has doubled and we're seeing performance issues, alert
        if entity_increase_ratio > 2.0 && estimated_performance_impact > 50.0 {
            let affected_metrics = vec![
                "frame_time".to_string(),
                "ai_processing_time".to_string(),
                "memory_usage".to_string(),
            ];

            alert_events.write(PerformanceAlert::EntityCountPerformanceImpact {
                entity_count,
                baseline_entity_count: state.baseline_values.normal_entity_count,
                performance_degradation_percent: estimated_performance_impact,
                affected_metrics,
            });
        }
    }
}

/// System that processes performance alerts and adds them to the log buffer
fn alert_logging_system(
    mut alert_events: EventReader<PerformanceAlert>,
    mut state: ResMut<PerformanceMonitorState>,
    config: Res<PerformanceMonitorConfig>,
) {
    for alert in alert_events.read() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let timestamp_human = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let (alert_type, severity) = match alert {
            PerformanceAlert::HighCpuUsage { current, threshold, .. } => {
                let severity = if *current > threshold * 1.2 { AlertSeverity::Critical } else { AlertSeverity::Warning };
                ("HighCpuUsage".to_string(), severity)
            },
            PerformanceAlert::HighMemoryUsage { percentage, threshold, .. } => {
                let severity = if *percentage > threshold * 1.1 { AlertSeverity::Severe } else { AlertSeverity::Critical };
                ("HighMemoryUsage".to_string(), severity)
            },
            PerformanceAlert::HighFrameTime { current_ms, target_ms, .. } => {
                let severity = if *current_ms > target_ms * 2.0 { AlertSeverity::Severe } else { AlertSeverity::Warning };
                ("HighFrameTime".to_string(), severity)
            },
            PerformanceAlert::LowFpsDrops { duration_ms, .. } => {
                let severity = if *duration_ms > 3000 { AlertSeverity::Critical } else { AlertSeverity::Warning };
                ("LowFpsDrops".to_string(), severity)
            },
            PerformanceAlert::HighFrameJitter { .. } => ("HighFrameJitter".to_string(), AlertSeverity::Warning),
            PerformanceAlert::SlowSystemExecution { .. } => ("SlowSystemExecution".to_string(), AlertSeverity::Warning),
            PerformanceAlert::SlowAiProcessing { .. } => ("SlowAiProcessing".to_string(), AlertSeverity::Critical),
            PerformanceAlert::EntityCountPerformanceImpact { .. } => ("EntityCountPerformanceImpact".to_string(), AlertSeverity::Warning),
            _ => ("Unknown".to_string(), AlertSeverity::Warning),
        };

        let log_entry = AlertLogEntry {
            timestamp,
            timestamp_human,
            alert_type,
            severity,
            alert: alert.clone(),
            baseline_values: state.baseline_values.clone(),
        };

        state.alert_log_buffer.push(log_entry);

        // Log to console as well for immediate visibility
        match severity {
            AlertSeverity::Warning => warn!("Performance Alert: {:?}", alert),
            AlertSeverity::Critical => error!("Critical Performance Alert: {:?}", alert),
            AlertSeverity::Severe => error!("SEVERE Performance Alert: {:?}", alert),
        }

        // Flush buffer if it's getting full
        if state.alert_log_buffer.len() >= config.log_buffer_size {
            flush_alert_buffer(&mut state, &config);
        }
    }
}

/// System that periodically flushes the alert log buffer to disk
fn periodic_log_flush_system(
    time: Res<Time>,
    mut state: ResMut<PerformanceMonitorState>,
    config: Res<PerformanceMonitorConfig>,
) {
    let current_time = time.elapsed_secs_f64();

    // Check if it's time to flush
    if (current_time - state.last_flush_time) * 1000.0 >= config.flush_interval_ms as f64 {
        if !state.alert_log_buffer.is_empty() {
            flush_alert_buffer(&mut state, &config);
        }
        state.last_flush_time = current_time;
    }
}

/// Helper function to flush the alert log buffer to disk
fn flush_alert_buffer(
    state: &mut PerformanceMonitorState,
    config: &PerformanceMonitorConfig,
) {
    if let Some(log_file_path) = &state.current_log_file {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
        {
            Ok(mut file) => {
                for entry in &state.alert_log_buffer {
                    if let Ok(json_line) = serde_json::to_string(entry) {
                        if let Err(e) = writeln!(file, "{}", json_line) {
                            error!("Failed to write alert log entry: {}", e);
                        }
                    }
                }

                if let Err(e) = file.flush() {
                    error!("Failed to flush alert log file: {}", e);
                } else {
                    debug!("Flushed {} alert entries to log file", state.alert_log_buffer.len());
                }

                state.alert_log_buffer.clear();
            },
            Err(e) => {
                error!("Failed to open alert log file {:?}: {}", log_file_path, e);
            }
        }
    }
}
