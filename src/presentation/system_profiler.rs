//! System performance profiler for measuring and comparing event-driven vs polling approaches.
//!
//! This module provides tools to measure system execution times and compare different
//! architectural approaches (event-driven vs polling) with quantitative data.

use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Execution mode for systems being profiled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemExecutionMode {
    EventDriven,
    Polling,
}

/// Performance metrics for a single system execution
#[derive(Debug, Clone)]
pub struct SystemExecutionMetrics {
    pub system_name: String,
    pub execution_mode: SystemExecutionMode,
    pub execution_time: Duration,
    pub timestamp: Instant,
    pub entity_count: u32,
    pub events_processed: u32,
}

/// Aggregated performance statistics for a system
#[derive(Debug, Clone)]
pub struct SystemPerformanceStats {
    pub system_name: String,
    pub execution_mode: SystemExecutionMode,
    pub total_executions: u32,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub last_execution: Option<Instant>,
    pub average_entities_processed: f32,
    pub average_events_processed: f32,
}

impl SystemPerformanceStats {
    pub fn new(system_name: String, execution_mode: SystemExecutionMode) -> Self {
        Self {
            system_name,
            execution_mode,
            total_executions: 0,
            total_time: Duration::ZERO,
            average_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            last_execution: None,
            average_entities_processed: 0.0,
            average_events_processed: 0.0,
        }
    }

    pub fn add_execution(&mut self, metrics: &SystemExecutionMetrics) {
        self.total_executions += 1;
        self.total_time += metrics.execution_time;
        self.average_time = self.total_time / self.total_executions;
        
        if metrics.execution_time < self.min_time {
            self.min_time = metrics.execution_time;
        }
        if metrics.execution_time > self.max_time {
            self.max_time = metrics.execution_time;
        }
        
        self.last_execution = Some(metrics.timestamp);
        
        // Update running averages
        let n = self.total_executions as f32;
        self.average_entities_processed = 
            (self.average_entities_processed * (n - 1.0) + metrics.entity_count as f32) / n;
        self.average_events_processed = 
            (self.average_events_processed * (n - 1.0) + metrics.events_processed as f32) / n;
    }

    /// Calculate performance efficiency (entities processed per microsecond)
    pub fn efficiency(&self) -> f32 {
        if self.average_time.as_micros() == 0 {
            return 0.0;
        }
        self.average_entities_processed / self.average_time.as_micros() as f32
    }
}

/// Comparison result between event-driven and polling approaches
#[derive(Debug, Clone)]
pub struct SystemPerformanceComparison {
    pub system_name: String,
    pub event_driven_stats: Option<SystemPerformanceStats>,
    pub polling_stats: Option<SystemPerformanceStats>,
    pub recommended_approach: Option<SystemExecutionMode>,
    pub performance_difference_percent: f32,
    pub confidence_level: f32,
}

impl SystemPerformanceComparison {
    pub fn new(system_name: String) -> Self {
        Self {
            system_name,
            event_driven_stats: None,
            polling_stats: None,
            recommended_approach: None,
            performance_difference_percent: 0.0,
            confidence_level: 0.0,
        }
    }

    pub fn analyze(&mut self) {
        if let (Some(event_stats), Some(polling_stats)) = 
            (&self.event_driven_stats, &self.polling_stats) {
            
            let event_avg_micros = event_stats.average_time.as_micros() as f32;
            let polling_avg_micros = polling_stats.average_time.as_micros() as f32;
            
            if event_avg_micros > 0.0 && polling_avg_micros > 0.0 {
                // Calculate percentage difference (positive means event-driven is faster)
                self.performance_difference_percent = 
                    ((polling_avg_micros - event_avg_micros) / polling_avg_micros) * 100.0;
                
                // Determine recommendation based on performance and other factors
                self.recommended_approach = if self.performance_difference_percent.abs() < 5.0 {
                    // Less than 5% difference - recommend event-driven for better architecture
                    Some(SystemExecutionMode::EventDriven)
                } else if self.performance_difference_percent > 0.0 {
                    // Event-driven is significantly faster
                    Some(SystemExecutionMode::EventDriven)
                } else {
                    // Polling is significantly faster
                    Some(SystemExecutionMode::Polling)
                };
                
                // Calculate confidence based on sample size and consistency
                let min_samples = event_stats.total_executions.min(polling_stats.total_executions);
                let sample_confidence = (min_samples as f32 / 100.0).min(1.0); // Max confidence at 100 samples
                
                // Factor in consistency (lower variance = higher confidence)
                let event_variance = (event_stats.max_time.as_micros() as f32 - event_stats.min_time.as_micros() as f32) / event_avg_micros;
                let polling_variance = (polling_stats.max_time.as_micros() as f32 - polling_stats.min_time.as_micros() as f32) / polling_avg_micros;
                let consistency_confidence = 1.0 - ((event_variance + polling_variance) / 2.0).min(1.0);
                
                self.confidence_level = (sample_confidence + consistency_confidence) / 2.0;
            }
        }
    }
}

/// Resource that tracks system performance metrics
#[derive(Resource, Debug)]
pub struct SystemProfiler {
    pub metrics_history: Vec<SystemExecutionMetrics>,
    pub system_stats: HashMap<(String, SystemExecutionMode), SystemPerformanceStats>,
    pub comparisons: HashMap<String, SystemPerformanceComparison>,
    pub max_history_size: usize,
    pub profiling_enabled: bool,
}

impl Default for SystemProfiler {
    fn default() -> Self {
        Self {
            metrics_history: Vec::new(),
            system_stats: HashMap::new(),
            comparisons: HashMap::new(),
            max_history_size: 1000,
            profiling_enabled: true,
        }
    }
}

impl SystemProfiler {
    /// Record a system execution
    pub fn record_execution(&mut self, metrics: SystemExecutionMetrics) {
        if !self.profiling_enabled {
            return;
        }

        let key = (metrics.system_name.clone(), metrics.execution_mode);
        
        // Update or create stats for this system/mode combination
        let stats = self.system_stats.entry(key).or_insert_with(|| {
            SystemPerformanceStats::new(metrics.system_name.clone(), metrics.execution_mode)
        });
        stats.add_execution(&metrics);
        
        // Update comparison data
        let comparison = self.comparisons.entry(metrics.system_name.clone())
            .or_insert_with(|| SystemPerformanceComparison::new(metrics.system_name.clone()));
        
        match metrics.execution_mode {
            SystemExecutionMode::EventDriven => {
                comparison.event_driven_stats = Some(stats.clone());
            }
            SystemExecutionMode::Polling => {
                comparison.polling_stats = Some(stats.clone());
            }
        }
        comparison.analyze();
        
        // Add to history
        self.metrics_history.push(metrics);
        
        // Trim history if needed
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.remove(0);
        }
    }

    /// Get performance comparison for a system
    pub fn get_comparison(&self, system_name: &str) -> Option<&SystemPerformanceComparison> {
        self.comparisons.get(system_name)
    }

    /// Get all comparisons with recommendations
    pub fn get_recommendations(&self) -> Vec<&SystemPerformanceComparison> {
        self.comparisons.values()
            .filter(|comp| comp.recommended_approach.is_some())
            .collect()
    }

    /// Clear all profiling data
    pub fn clear(&mut self) {
        self.metrics_history.clear();
        self.system_stats.clear();
        self.comparisons.clear();
    }

    /// Generate a performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== System Performance Report ===\n\n");
        
        for comparison in self.comparisons.values() {
            report.push_str(&format!("System: {}\n", comparison.system_name));
            
            if let Some(event_stats) = &comparison.event_driven_stats {
                report.push_str(&format!(
                    "  Event-Driven: {:.2}μs avg ({} executions, {:.2} entities/exec)\n",
                    event_stats.average_time.as_micros(),
                    event_stats.total_executions,
                    event_stats.average_entities_processed
                ));
            }
            
            if let Some(polling_stats) = &comparison.polling_stats {
                report.push_str(&format!(
                    "  Polling: {:.2}μs avg ({} executions, {:.2} entities/exec)\n",
                    polling_stats.average_time.as_micros(),
                    polling_stats.total_executions,
                    polling_stats.average_entities_processed
                ));
            }
            
            if let Some(recommendation) = comparison.recommended_approach {
                report.push_str(&format!(
                    "  Recommendation: {:?} ({:.1}% difference, {:.1}% confidence)\n",
                    recommendation,
                    comparison.performance_difference_percent,
                    comparison.confidence_level * 100.0
                ));
            }
            
            report.push('\n');
        }
        
        report
    }
}

/// Macro for easy system profiling
#[macro_export]
macro_rules! profile_system {
    ($profiler:expr, $system_name:expr, $mode:expr, $entity_count:expr, $events_processed:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let execution_time = start.elapsed();
        
        {
            $profiler.record_execution($crate::presentation::system_profiler::SystemExecutionMetrics {
                system_name: $system_name.to_string(),
                execution_mode: $mode,
                execution_time,
                timestamp: start,
                entity_count: $entity_count,
                events_processed: $events_processed,
            });
        }
        
        result
    }};
}

/// Events for system profiling
#[derive(Event, Debug)]
pub enum SystemProfilerEvent {
    StartProfiling,
    StopProfiling,
    ClearData,
    GenerateReport,
    PrintRecommendations,
}

/// System that handles profiler events
pub fn system_profiler_event_handler(
    mut events: EventReader<SystemProfilerEvent>,
    mut profiler: ResMut<SystemProfiler>,
) {
    for event in events.read() {
        match event {
            SystemProfilerEvent::StartProfiling => {
                profiler.profiling_enabled = true;
                info!("System profiling enabled");
            }
            SystemProfilerEvent::StopProfiling => {
                profiler.profiling_enabled = false;
                info!("System profiling disabled");
            }
            SystemProfilerEvent::ClearData => {
                profiler.clear();
                info!("System profiling data cleared");
            }
            SystemProfilerEvent::GenerateReport => {
                let report = profiler.generate_report();
                info!("System Performance Report:\n{}", report);
            }
            SystemProfilerEvent::PrintRecommendations => {
                let recommendations = profiler.get_recommendations();
                if recommendations.is_empty() {
                    info!("No performance recommendations available yet");
                } else {
                    info!("Performance Recommendations:");
                    for comp in recommendations {
                        if let Some(rec) = comp.recommended_approach {
                            info!(
                                "  {}: Use {:?} ({:.1}% better, {:.1}% confidence)",
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
}

/// System that periodically prints performance recommendations
pub fn periodic_performance_recommendations(
    time: Res<Time>,
    profiler: Res<SystemProfiler>,
    mut last_report_time: Local<f32>,
) {
    let current_time = time.elapsed_secs();
    
    // Print recommendations every 60 seconds
    if current_time - *last_report_time > 60.0 {
        *last_report_time = current_time;
        
        let recommendations = profiler.get_recommendations();
        if !recommendations.is_empty() {
            info!("=== Periodic Performance Recommendations ===");
            for comp in recommendations {
                if comp.confidence_level > 0.7 { // Only show high-confidence recommendations
                    if let Some(rec) = comp.recommended_approach {
                        info!(
                            "{}: Use {:?} ({:.1}% performance difference)",
                            comp.system_name,
                            rec,
                            comp.performance_difference_percent.abs()
                        );
                    }
                }
            }
        }
    }
}

/// Plugin for system profiling
pub struct SystemProfilerPlugin;

impl Plugin for SystemProfilerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SystemProfiler>()
            .add_event::<SystemProfilerEvent>()
            .add_systems(Update, (
                system_profiler_event_handler,
                periodic_performance_recommendations,
            ));
    }
}

/// Helper trait for easy system profiling integration
pub trait ProfiledSystem {
    fn with_profiling<T>(
        &self,
        profiler: &mut ResMut<SystemProfiler>,
        system_name: &str,
        mode: SystemExecutionMode,
        entity_count: u32,
        events_processed: u32,
        f: impl FnOnce() -> T,
    ) -> T {
        let start = Instant::now();
        let result = f();
        let execution_time = start.elapsed();
        
        profiler.record_execution(SystemExecutionMetrics {
            system_name: system_name.to_string(),
            execution_mode: mode,
            execution_time,
            timestamp: start,
            entity_count,
            events_processed,
        });
        
        result
    }
}

// Implement the trait for any type (blanket implementation)
impl<T> ProfiledSystem for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_performance_stats() {
        let mut stats = SystemPerformanceStats::new(
            "test_system".to_string(),
            SystemExecutionMode::EventDriven
        );
        
        let metrics = SystemExecutionMetrics {
            system_name: "test_system".to_string(),
            execution_mode: SystemExecutionMode::EventDriven,
            execution_time: Duration::from_micros(100),
            timestamp: Instant::now(),
            entity_count: 50,
            events_processed: 5,
        };
        
        stats.add_execution(&metrics);
        
        assert_eq!(stats.total_executions, 1);
        assert_eq!(stats.average_time, Duration::from_micros(100));
        assert_eq!(stats.average_entities_processed, 50.0);
    }

    #[test]
    fn test_performance_comparison() {
        let mut comparison = SystemPerformanceComparison::new("test_system".to_string());
        
        let mut event_stats = SystemPerformanceStats::new(
            "test_system".to_string(),
            SystemExecutionMode::EventDriven
        );
        event_stats.average_time = Duration::from_micros(100);
        event_stats.total_executions = 10;
        
        let mut polling_stats = SystemPerformanceStats::new(
            "test_system".to_string(),
            SystemExecutionMode::Polling
        );
        polling_stats.average_time = Duration::from_micros(150);
        polling_stats.total_executions = 10;
        
        comparison.event_driven_stats = Some(event_stats);
        comparison.polling_stats = Some(polling_stats);
        comparison.analyze();
        
        assert_eq!(comparison.recommended_approach, Some(SystemExecutionMode::EventDriven));
        assert!(comparison.performance_difference_percent > 0.0);
    }
}