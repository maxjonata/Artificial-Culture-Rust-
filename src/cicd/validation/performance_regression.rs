use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use crate::cicd::validation::ai_patterns::{ValidationResult, ValidationIssue, IssueType, IssueSeverity, CheckType};

/// Hardware simulation and performance regression detection system
#[derive(Debug)]
pub struct PerformanceRegressionDetector {
    pub hardware_simulator: HardwareSimulator,
    pub baseline_manager: BaselineManager,
    pub performance_monitor: PerformanceMonitor,
    pub regression_analyzer: RegressionAnalyzer,
}

impl PerformanceRegressionDetector {
    pub fn new(baseline_path: PathBuf) -> Self {
        Self {
            hardware_simulator: HardwareSimulator::new(),
            baseline_manager: BaselineManager::new(baseline_path),
            performance_monitor: PerformanceMonitor::new(),
            regression_analyzer: RegressionAnalyzer::new(),
        }
    }

    /// Run complete performance regression detection
    pub fn detect_regressions(&mut self, project_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();

        // 1. Set up hardware simulation
        if let Err(e) = self.hardware_simulator.configure_simulation() {
            issues.push(ValidationIssue::new(
                IssueType::PerformanceRegressionSetupFailed,
                format!("Failed to configure hardware simulation: {}", e),
                project_path.to_path_buf(),
                0,
                IssueSeverity::Critical,
                Some("Ensure system has sufficient resources for performance testing".to_string()),
            ));
        }

        // 2. Run performance tests and collect metrics
        match self.run_performance_tests(project_path) {
            Ok(current_metrics) => {
                // 3. Compare against baselines
                if let Ok(baseline_metrics) = self.baseline_manager.load_baseline() {
                    let regression_issues = self.regression_analyzer.analyze_regressions(
                        &baseline_metrics,
                        &current_metrics,
                        project_path,
                    );
                    issues.extend(regression_issues);
                } else {
                    // No baseline exists, establish new baseline
                    if let Err(e) = self.baseline_manager.establish_baseline(&current_metrics) {
                        issues.push(ValidationIssue {
                            issue_type: IssueType::BaselineEstablishmentFailed,
                            message: format!("Failed to establish performance baseline: {}", e),
                            file_path: project_path.to_path_buf(),
                            line_number: 0,
                            column_number: None,
                            severity: crate::cicd::validation::ai_patterns::IssueSeverity::Medium,
                            suggestion: Some("Check write permissions for baseline storage".to_string()),
                            code_example: None,
                        });
                    }
                }
            }
            Err(e) => {
                issues.push(ValidationIssue {
                    issue_type: IssueType::PerformanceTestFailed,
                    message: format!("Performance tests failed: {}", e),
                    file_path: project_path.to_path_buf(),
                    line_number: 0,
                    column_number: None,
                    severity: crate::cicd::validation::ai_patterns::IssueSeverity::Critical,
                    suggestion: Some("Fix compilation errors or test failures before performance validation".to_string()),
                    code_example: None,
                });
            }
        }

        ValidationResult {
            file_path: project_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions: self.generate_optimization_suggestions(),
            execution_time: Duration::from_secs(0), // Will be set by caller
            check_type: CheckType::PerformanceRegression,
        }
    }

    fn run_performance_tests(&mut self, project_path: &Path) -> Result<PerformanceMetrics, String> {
        // Start performance monitoring
        self.performance_monitor.start_monitoring();

        // Run the performance test suite
        let test_result = std::process::Command::new("cargo")
            .args(&["test", "--release", "--", "performance_", "--nocapture"])
            .current_dir(project_path)
            .output()
            .map_err(|e| format!("Failed to run performance tests: {}", e))?;

        if !test_result.status.success() {
            return Err(format!(
                "Performance tests failed: {}",
                String::from_utf8_lossy(&test_result.stderr)
            ));
        }

        // Stop monitoring and collect metrics
        let metrics = self.performance_monitor.stop_and_collect();
        Ok(metrics)
    }

    fn generate_optimization_suggestions(&self) -> Vec<String> {
        vec![
            "Consider using parallel processing for agent updates".to_string(),
            "Profile memory allocations in hot paths".to_string(),
            "Optimize query patterns in Bevy systems".to_string(),
            "Use object pooling for frequently allocated objects".to_string(),
        ]
    }
}

/// Simulates medium-low hardware specifications for consistent testing
#[derive(Debug)]
pub struct HardwareSimulator {
    pub target_specs: HardwareSpecs,
    pub current_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSpecs {
    pub cpu_cores: u32,
    pub memory_gb: f32,
    pub target_fps: f32,
    pub max_agents: u32,
}

impl Default for HardwareSpecs {
    fn default() -> Self {
        Self {
            cpu_cores: 2,
            memory_gb: 4.0,
            target_fps: 60.0,
            max_agents: 100,
        }
    }
}

#[derive(Debug)]
pub struct ResourceLimits {
    pub memory_limit_bytes: usize,
    pub cpu_affinity_mask: u64,
}

impl HardwareSimulator {
    pub fn new() -> Self {
        Self {
            target_specs: HardwareSpecs::default(),
            current_limits: ResourceLimits {
                memory_limit_bytes: (4.0 * 1024.0 * 1024.0 * 1024.0) as usize, // 4GB
                cpu_affinity_mask: 0b11, // 2 cores
            },
        }
    }

    /// Configure system to simulate target hardware specifications
    pub fn configure_simulation(&self) -> Result<(), String> {
        // On Windows, we can use job objects to limit resources
        #[cfg(target_os = "windows")]
        {
            self.configure_windows_limits()
        }

        // On Unix systems, we can use cgroups or process limits
        #[cfg(unix)]
        {
            self.configure_unix_limits()
        }

        // For other platforms, just validate we can run the tests
        #[cfg(not(any(target_os = "windows", unix)))]
        {
            Ok(())
        }
    }

    #[cfg(target_os = "windows")]
    fn configure_windows_limits(&self) -> Result<(), String> {
        // Note: This would require Windows API calls to create job objects
        // For now, we'll just validate the system meets minimum requirements
        self.validate_system_requirements()
    }

    #[cfg(unix)]
    fn configure_unix_limits(&self) -> Result<(), String> {
        // Note: This would require setting up cgroups or using setrlimit
        // For now, we'll just validate the system meets minimum requirements
        self.validate_system_requirements()
    }

    fn validate_system_requirements(&self) -> Result<(), String> {
        // Check available memory
        let available_memory = self.get_available_memory()?;
        if available_memory < self.target_specs.memory_gb {
            return Err(format!(
                "Insufficient memory: {} GB available, {} GB required",
                available_memory, self.target_specs.memory_gb
            ));
        }

        // Check CPU cores
        let available_cores = num_cpus::get() as u32;
        if available_cores < self.target_specs.cpu_cores {
            return Err(format!(
                "Insufficient CPU cores: {} available, {} required",
                available_cores, self.target_specs.cpu_cores
            ));
        }

        Ok(())
    }

    fn get_available_memory(&self) -> Result<f32, String> {
        // This is a simplified implementation
        // In a real implementation, we'd use platform-specific APIs
        #[cfg(target_os = "windows")]
        {
            // Use GlobalMemoryStatusEx on Windows
            Ok(8.0) // Placeholder
        }

        #[cfg(unix)]
        {
            // Parse /proc/meminfo on Linux or use sysctl on macOS
            Ok(8.0) // Placeholder
        }

        #[cfg(not(any(target_os = "windows", unix)))]
        {
            Ok(8.0) // Placeholder
        }
    }
}

/// Manages performance baselines for regression detection
#[derive(Debug)]
pub struct BaselineManager {
    pub baseline_path: PathBuf,
    pub current_baseline: Option<PerformanceBaseline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub timestamp: u64,
    pub git_commit: String,
    pub hardware_specs: HardwareSpecs,
    pub metrics: PerformanceMetrics,
    pub test_conditions: TestConditions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConditions {
    pub agent_count: u32,
    pub test_duration_seconds: f32,
    pub world_complexity: WorldComplexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldComplexity {
    Simple,
    Medium,
    Complex,
}

impl BaselineManager {
    pub fn new(baseline_path: PathBuf) -> Self {
        Self {
            baseline_path,
            current_baseline: None,
        }
    }

    /// Load existing performance baseline
    pub fn load_baseline(&mut self) -> Result<PerformanceBaseline, String> {
        if !self.baseline_path.exists() {
            return Err("No baseline file exists".to_string());
        }

        let baseline_content = std::fs::read_to_string(&self.baseline_path)
            .map_err(|e| format!("Failed to read baseline file: {}", e))?;

        let baseline: PerformanceBaseline = serde_json::from_str(&baseline_content)
            .map_err(|e| format!("Failed to parse baseline file: {}", e))?;

        self.current_baseline = Some(baseline.clone());
        Ok(baseline)
    }

    /// Establish new performance baseline
    pub fn establish_baseline(&mut self, metrics: &PerformanceMetrics) -> Result<(), String> {
        let git_commit = self.get_current_git_commit()?;
        
        let baseline = PerformanceBaseline {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            git_commit,
            hardware_specs: HardwareSpecs::default(),
            metrics: metrics.clone(),
            test_conditions: TestConditions {
                agent_count: 100,
                test_duration_seconds: 60.0,
                world_complexity: WorldComplexity::Medium,
            },
        };

        let baseline_json = serde_json::to_string_pretty(&baseline)
            .map_err(|e| format!("Failed to serialize baseline: {}", e))?;

        // Ensure directory exists
        if let Some(parent) = self.baseline_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create baseline directory: {}", e))?;
        }

        std::fs::write(&self.baseline_path, baseline_json)
            .map_err(|e| format!("Failed to write baseline file: {}", e))?;

        self.current_baseline = Some(baseline);
        Ok(())
    }

    fn get_current_git_commit(&self) -> Result<String, String> {
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .map_err(|e| format!("Failed to get git commit: {}", e))?;

        if !output.status.success() {
            return Err("Failed to get current git commit".to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

/// Monitors system performance during tests
#[derive(Debug)]
pub struct PerformanceMonitor {
    pub start_time: Option<Instant>,
    pub memory_samples: Vec<MemorySample>,
    pub cpu_samples: Vec<CpuSample>,
    pub frame_time_samples: Vec<f32>,
    pub memory_leak_detector: MemoryLeakDetector,
    pub cpu_usage_monitor: CpuUsageMonitor,
    pub system_profiler: SystemProfiler,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_fps: f32,
    pub min_fps: f32,
    pub max_fps: f32,
    pub frame_time_p95: f32,
    pub frame_time_p99: f32,
    pub memory_usage: MemoryMetrics,
    pub cpu_usage: CpuMetrics,
    pub agent_performance: AgentPerformanceMetrics,
    #[serde(skip)] // Skip serialization for complex types
    pub memory_issues: Vec<MemoryIssue>,
    #[serde(skip)] // Skip serialization for complex types
    pub cpu_issues: Vec<CpuIssue>,
    #[serde(skip)] // Skip serialization for complex types
    pub profiling_report: ProfilingReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub peak_usage_mb: f32,
    pub average_usage_mb: f32,
    pub allocation_rate_mb_per_sec: f32,
    pub gc_pressure: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub average_usage_percent: f32,
    pub peak_usage_percent: f32,
    pub system_time_percent: f32,
    pub user_time_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceMetrics {
    pub agents_processed_per_frame: f32,
    pub ai_system_time_ms: f32,
    pub social_system_time_ms: f32,
    pub physics_system_time_ms: f32,
}

#[derive(Debug, Clone)]
pub struct MemorySample {
    pub timestamp: Instant,
    pub usage_bytes: usize,
}

#[derive(Debug, Clone)]
pub struct CpuSample {
    pub timestamp: Instant,
    pub usage_percent: f32,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: None,
            memory_samples: Vec::new(),
            cpu_samples: Vec::new(),
            frame_time_samples: Vec::new(),
            memory_leak_detector: MemoryLeakDetector::new(),
            cpu_usage_monitor: CpuUsageMonitor::new(),
            system_profiler: SystemProfiler::new(),
        }
    }

    /// Start performance monitoring
    pub fn start_monitoring(&mut self) {
        self.start_time = Some(Instant::now());
        self.memory_samples.clear();
        self.cpu_samples.clear();
        self.frame_time_samples.clear();
        
        // Start advanced monitoring
        self.memory_leak_detector.start_detection();
        self.cpu_usage_monitor.start_monitoring();
        self.system_profiler.start_profiling();
    }

    /// Stop monitoring and collect final metrics
    pub fn stop_and_collect(&mut self) -> PerformanceMetrics {
        let _total_duration = self.start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();

        // Calculate FPS metrics from frame time samples
        let fps_samples: Vec<f32> = self.frame_time_samples
            .iter()
            .map(|&frame_time| if frame_time > 0.0 { 1.0 / frame_time } else { 0.0 })
            .collect();

        let average_fps = if !fps_samples.is_empty() {
            fps_samples.iter().sum::<f32>() / fps_samples.len() as f32
        } else {
            0.0
        };

        let min_fps = fps_samples.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_fps = fps_samples.iter().cloned().fold(0.0, f32::max);

        // Calculate percentiles
        let mut sorted_frame_times = self.frame_time_samples.clone();
        sorted_frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let frame_time_p95 = self.calculate_percentile(&sorted_frame_times, 0.95);
        let frame_time_p99 = self.calculate_percentile(&sorted_frame_times, 0.99);

        // Calculate memory metrics
        let memory_metrics = self.calculate_memory_metrics();
        let cpu_metrics = self.calculate_cpu_metrics();

        // Generate detailed profiling report
        let profiling_report = self.system_profiler.generate_profiling_report();
        
        // Analyze memory issues
        let current_memory = self.memory_samples.last()
            .map(|s| s.usage_bytes)
            .unwrap_or(0);
        let memory_issues = self.memory_leak_detector.analyze_memory_usage(current_memory);
        
        // Analyze CPU issues
        let cpu_issues = self.cpu_usage_monitor.analyze_cpu_usage();

        PerformanceMetrics {
            average_fps,
            min_fps: if min_fps.is_infinite() { 0.0 } else { min_fps },
            max_fps,
            frame_time_p95,
            frame_time_p99,
            memory_usage: memory_metrics,
            cpu_usage: cpu_metrics,
            agent_performance: AgentPerformanceMetrics {
                agents_processed_per_frame: 100.0, // Placeholder
                ai_system_time_ms: 5.0,           // Placeholder
                social_system_time_ms: 3.0,       // Placeholder
                physics_system_time_ms: 2.0,      // Placeholder
            },
            memory_issues,
            cpu_issues,
            profiling_report,
        }
    }

    fn calculate_percentile(&self, sorted_values: &[f32], percentile: f32) -> f32 {
        if sorted_values.is_empty() {
            return 0.0;
        }

        let index = (percentile * (sorted_values.len() - 1) as f32) as usize;
        sorted_values.get(index).copied().unwrap_or(0.0)
    }

    fn calculate_memory_metrics(&self) -> MemoryMetrics {
        if self.memory_samples.is_empty() {
            return MemoryMetrics {
                peak_usage_mb: 0.0,
                average_usage_mb: 0.0,
                allocation_rate_mb_per_sec: 0.0,
                gc_pressure: 0.0,
            };
        }

        let peak_usage_bytes = self.memory_samples
            .iter()
            .map(|sample| sample.usage_bytes)
            .max()
            .unwrap_or(0);

        let average_usage_bytes = self.memory_samples
            .iter()
            .map(|sample| sample.usage_bytes)
            .sum::<usize>() / self.memory_samples.len();

        MemoryMetrics {
            peak_usage_mb: peak_usage_bytes as f32 / (1024.0 * 1024.0),
            average_usage_mb: average_usage_bytes as f32 / (1024.0 * 1024.0),
            allocation_rate_mb_per_sec: 0.0, // Would need more sophisticated tracking
            gc_pressure: 0.0,                // Rust doesn't have GC, but could track allocator pressure
        }
    }

    fn calculate_cpu_metrics(&self) -> CpuMetrics {
        if self.cpu_samples.is_empty() {
            return CpuMetrics {
                average_usage_percent: 0.0,
                peak_usage_percent: 0.0,
                system_time_percent: 0.0,
                user_time_percent: 0.0,
            };
        }

        let average_usage = self.cpu_samples
            .iter()
            .map(|sample| sample.usage_percent)
            .sum::<f32>() / self.cpu_samples.len() as f32;

        let peak_usage = self.cpu_samples
            .iter()
            .map(|sample| sample.usage_percent)
            .fold(0.0, f32::max);

        CpuMetrics {
            average_usage_percent: average_usage,
            peak_usage_percent: peak_usage,
            system_time_percent: average_usage * 0.3, // Rough estimate
            user_time_percent: average_usage * 0.7,   // Rough estimate
        }
    }

    /// Record a frame time sample
    pub fn record_frame_time(&mut self, frame_time_seconds: f32) {
        self.frame_time_samples.push(frame_time_seconds);
    }

    /// Record a memory usage sample
    pub fn record_memory_usage(&mut self, usage_bytes: usize) {
        self.memory_samples.push(MemorySample {
            timestamp: Instant::now(),
            usage_bytes,
        });
    }

    /// Record a CPU usage sample
    pub fn record_cpu_usage(&mut self, usage_percent: f32) {
        self.cpu_samples.push(CpuSample {
            timestamp: Instant::now(),
            usage_percent,
        });
    }

    /// Record detailed CPU usage with system breakdown
    pub fn record_detailed_cpu_usage(&mut self, usage_percent: f32, system_breakdown: SystemCpuBreakdown) {
        self.cpu_usage_monitor.record_cpu_sample(usage_percent, system_breakdown);
    }

    /// Record system execution for profiling
    pub fn record_system_execution(&mut self, system_name: String, execution_data: SystemExecutionData) {
        self.system_profiler.record_system_execution(system_name, execution_data);
    }

    /// Record memory allocation for leak detection
    pub fn record_allocation(&mut self, size: usize, allocation_type: AllocationType) {
        self.memory_leak_detector.allocation_tracking.record_allocation(size, allocation_type);
    }
}

/// Analyzes performance regressions by comparing current metrics to baselines
#[derive(Debug)]
pub struct RegressionAnalyzer {
    pub fps_regression_threshold: f32,
    pub memory_regression_threshold: f32,
    pub cpu_regression_threshold: f32,
}

impl RegressionAnalyzer {
    pub fn new() -> Self {
        Self {
            fps_regression_threshold: 0.05,  // 5% FPS drop is a regression
            memory_regression_threshold: 0.10, // 10% memory increase is a regression
            cpu_regression_threshold: 0.15,  // 15% CPU increase is a regression
        }
    }

    /// Analyze performance regressions between baseline and current metrics
    pub fn analyze_regressions(
        &self,
        baseline: &PerformanceBaseline,
        current: &PerformanceMetrics,
        project_path: &Path,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check FPS regression
        let fps_regression = (baseline.metrics.average_fps - current.average_fps) / baseline.metrics.average_fps;
        if fps_regression > self.fps_regression_threshold {
            issues.push(ValidationIssue {
                issue_type: IssueType::FpsRegression,
                message: format!(
                    "FPS regression detected: {:.1} fps -> {:.1} fps ({:.1}% decrease)",
                    baseline.metrics.average_fps,
                    current.average_fps,
                    fps_regression * 100.0
                ),
                file_path: project_path.to_path_buf(),
                line_number: 0,
                column_number: None,
                severity: crate::cicd::validation::ai_patterns::IssueSeverity::High,
                suggestion: Some(format!(
                    "Profile the application to identify performance bottlenecks. Target: {:.1} fps, Current: {:.1} fps",
                    baseline.metrics.average_fps,
                    current.average_fps
                )),
                code_example: None,
            });
        }

        // Check memory regression
        let memory_regression = (current.memory_usage.peak_usage_mb - baseline.metrics.memory_usage.peak_usage_mb) 
            / baseline.metrics.memory_usage.peak_usage_mb;
        if memory_regression > self.memory_regression_threshold {
            issues.push(ValidationIssue {
                issue_type: IssueType::MemoryRegression,
                message: format!(
                    "Memory usage regression detected: {:.1} MB -> {:.1} MB ({:.1}% increase)",
                    baseline.metrics.memory_usage.peak_usage_mb,
                    current.memory_usage.peak_usage_mb,
                    memory_regression * 100.0
                ),
                file_path: project_path.to_path_buf(),
                line_number: 0,
                column_number: None,
                severity: crate::cicd::validation::ai_patterns::IssueSeverity::High,
                suggestion: Some(format!(
                    "Check for memory leaks or excessive allocations. Baseline: {:.1} MB, Current: {:.1} MB",
                    baseline.metrics.memory_usage.peak_usage_mb,
                    current.memory_usage.peak_usage_mb
                )),
                code_example: None,
            });
        }

        // Check CPU regression
        let cpu_regression = (current.cpu_usage.average_usage_percent - baseline.metrics.cpu_usage.average_usage_percent) 
            / baseline.metrics.cpu_usage.average_usage_percent;
        if cpu_regression > self.cpu_regression_threshold {
            issues.push(ValidationIssue {
                issue_type: IssueType::CpuRegression,
                message: format!(
                    "CPU usage regression detected: {:.1}% -> {:.1}% ({:.1}% increase)",
                    baseline.metrics.cpu_usage.average_usage_percent,
                    current.cpu_usage.average_usage_percent,
                    cpu_regression * 100.0
                ),
                file_path: project_path.to_path_buf(),
                line_number: 0,
                column_number: None,
                severity: crate::cicd::validation::ai_patterns::IssueSeverity::Medium,
                suggestion: Some(format!(
                    "Profile CPU usage to identify computational bottlenecks. Baseline: {:.1}%, Current: {:.1}%",
                    baseline.metrics.cpu_usage.average_usage_percent,
                    current.cpu_usage.average_usage_percent
                )),
                code_example: None,
            });
        }

        // Check if minimum FPS target is met
        if current.min_fps < 60.0 {
            issues.push(ValidationIssue {
                issue_type: IssueType::FpsTargetNotMet,
                message: format!(
                    "Minimum FPS target not met: {:.1} fps (target: 60 fps)",
                    current.min_fps
                ),
                file_path: project_path.to_path_buf(),
                line_number: 0,
                column_number: None,
                severity: crate::cicd::validation::ai_patterns::IssueSeverity::Critical,
                suggestion: Some("Optimize performance-critical systems to maintain 60 fps with 100+ agents".to_string()),
                code_example: None,
            });
        }

        issues
    }
}

/// Detects memory leaks and allocation pattern issues
#[derive(Debug)]
pub struct MemoryLeakDetector {
    pub baseline_memory: Option<usize>,
    pub allocation_tracking: AllocationTracker,
    pub leak_detection_threshold: f32, // Percentage increase that indicates a leak
    pub allocation_pattern_analyzer: AllocationPatternAnalyzer,
}

impl MemoryLeakDetector {
    pub fn new() -> Self {
        Self {
            baseline_memory: None,
            allocation_tracking: AllocationTracker::new(),
            leak_detection_threshold: 0.20, // 20% increase indicates potential leak
            allocation_pattern_analyzer: AllocationPatternAnalyzer::new(),
        }
    }

    /// Start memory leak detection
    pub fn start_detection(&mut self) {
        self.baseline_memory = Some(self.get_current_memory_usage());
        self.allocation_tracking.start_tracking();
    }

    /// Analyze memory usage for leaks and patterns
    pub fn analyze_memory_usage(&mut self, current_memory: usize) -> Vec<MemoryIssue> {
        let mut issues = Vec::new();

        // Check for memory leaks
        if let Some(baseline) = self.baseline_memory {
            let increase_ratio = (current_memory as f32 - baseline as f32) / baseline as f32;
            
            if increase_ratio > self.leak_detection_threshold {
                issues.push(MemoryIssue {
                    issue_type: MemoryIssueType::PotentialLeak,
                    message: format!(
                        "Memory usage increased by {:.1}% from baseline ({} MB -> {} MB)",
                        increase_ratio * 100.0,
                        baseline / (1024 * 1024),
                        current_memory / (1024 * 1024)
                    ),
                    severity: if increase_ratio > 0.5 { 
                        IssueSeverity::Critical 
                    } else { 
                        IssueSeverity::High 
                    },
                    suggestion: "Check for memory leaks in agent systems, particularly in social memory and relationship tracking".to_string(),
                });
            }
        }

        // Analyze allocation patterns
        let allocation_issues = self.allocation_pattern_analyzer.analyze_patterns(&self.allocation_tracking);
        issues.extend(allocation_issues);

        issues
    }

    fn get_current_memory_usage(&self) -> usize {
        // This is a simplified implementation
        // In a real implementation, we'd use platform-specific APIs
        #[cfg(target_os = "windows")]
        {
            // Use GetProcessMemoryInfo on Windows
            50 * 1024 * 1024 // Placeholder: 50MB
        }

        #[cfg(unix)]
        {
            // Parse /proc/self/status on Linux or use task_info on macOS
            50 * 1024 * 1024 // Placeholder: 50MB
        }

        #[cfg(not(any(target_os = "windows", unix)))]
        {
            50 * 1024 * 1024 // Placeholder: 50MB
        }
    }
}

/// Tracks memory allocations for pattern analysis
#[derive(Debug)]
pub struct AllocationTracker {
    pub allocations: Vec<AllocationRecord>,
    pub tracking_active: bool,
}

#[derive(Debug, Clone)]
pub struct AllocationRecord {
    pub timestamp: Instant,
    pub size: usize,
    pub allocation_type: AllocationType,
    pub stack_trace: Option<String>, // Simplified - would need backtrace crate
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllocationType {
    Agent,
    SocialMemory,
    Relationship,
    EventQueue,
    Other(String),
}

impl AllocationTracker {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
            tracking_active: false,
        }
    }

    pub fn start_tracking(&mut self) {
        self.tracking_active = true;
        self.allocations.clear();
    }

    pub fn record_allocation(&mut self, size: usize, allocation_type: AllocationType) {
        if self.tracking_active {
            self.allocations.push(AllocationRecord {
                timestamp: Instant::now(),
                size,
                allocation_type,
                stack_trace: None, // Would capture backtrace in real implementation
            });
        }
    }

    pub fn stop_tracking(&mut self) {
        self.tracking_active = false;
    }
}

/// Analyzes allocation patterns for performance issues
#[derive(Debug)]
pub struct AllocationPatternAnalyzer {
    pub excessive_allocation_threshold: usize, // Bytes per second
    pub fragmentation_threshold: f32,
}

impl AllocationPatternAnalyzer {
    pub fn new() -> Self {
        Self {
            excessive_allocation_threshold: 10 * 1024 * 1024, // 10MB/sec
            fragmentation_threshold: 0.3, // 30% fragmentation
        }
    }

    pub fn analyze_patterns(&self, tracker: &AllocationTracker) -> Vec<MemoryIssue> {
        let mut issues = Vec::new();

        if tracker.allocations.is_empty() {
            return issues;
        }

        // Calculate allocation rate
        let total_duration = tracker.allocations.last().unwrap().timestamp
            .duration_since(tracker.allocations.first().unwrap().timestamp);
        
        if total_duration.as_secs() > 0 {
            let total_allocated: usize = tracker.allocations.iter().map(|a| a.size).sum();
            let allocation_rate = total_allocated as f32 / total_duration.as_secs_f32();

            if allocation_rate > self.excessive_allocation_threshold as f32 {
                issues.push(MemoryIssue {
                    issue_type: MemoryIssueType::ExcessiveAllocation,
                    message: format!(
                        "High allocation rate detected: {:.2} MB/sec (threshold: {:.2} MB/sec)",
                        allocation_rate / (1024.0 * 1024.0),
                        self.excessive_allocation_threshold as f32 / (1024.0 * 1024.0)
                    ),
                    severity: IssueSeverity::High,
                    suggestion: "Consider object pooling for frequently allocated objects like social interactions".to_string(),
                });
            }
        }

        // Analyze allocation patterns by type
        let mut type_counts = std::collections::HashMap::new();
        for allocation in &tracker.allocations {
            *type_counts.entry(&allocation.allocation_type).or_insert(0) += 1;
        }

        // Check for excessive agent allocations
        if let Some(agent_count) = type_counts.get(&AllocationType::Agent) {
            if *agent_count > 1000 {
                issues.push(MemoryIssue {
                    issue_type: MemoryIssueType::ExcessiveAgentAllocation,
                    message: format!("Excessive agent allocations detected: {} allocations", agent_count),
                    severity: IssueSeverity::Medium,
                    suggestion: "Consider agent pooling or reducing agent creation frequency".to_string(),
                });
            }
        }

        issues
    }
}

#[derive(Debug, Clone)]
pub struct MemoryIssue {
    pub issue_type: MemoryIssueType,
    pub message: String,
    pub severity: IssueSeverity,
    pub suggestion: String,
}

#[derive(Debug, Clone)]
pub enum MemoryIssueType {
    PotentialLeak,
    ExcessiveAllocation,
    ExcessiveAgentAllocation,
    Fragmentation,
}

/// Monitors CPU usage and computational budget validation
#[derive(Debug)]
pub struct CpuUsageMonitor {
    pub cpu_samples: Vec<CpuUsageSample>,
    pub computational_budget: ComputationalBudget,
    pub system_cpu_tracker: SystemCpuTracker,
}

impl CpuUsageMonitor {
    pub fn new() -> Self {
        Self {
            cpu_samples: Vec::new(),
            computational_budget: ComputationalBudget::new(),
            system_cpu_tracker: SystemCpuTracker::new(),
        }
    }

    /// Start CPU monitoring
    pub fn start_monitoring(&mut self) {
        self.cpu_samples.clear();
        self.system_cpu_tracker.start_tracking();
    }

    /// Record CPU usage sample
    pub fn record_cpu_sample(&mut self, usage_percent: f32, system_breakdown: SystemCpuBreakdown) {
        self.cpu_samples.push(CpuUsageSample {
            timestamp: Instant::now(),
            total_usage_percent: usage_percent,
            system_breakdown,
        });
    }

    /// Analyze CPU usage for budget violations and bottlenecks
    pub fn analyze_cpu_usage(&self) -> Vec<CpuIssue> {
        let mut issues = Vec::new();

        if self.cpu_samples.is_empty() {
            return issues;
        }

        // Calculate average CPU usage
        let average_cpu = self.cpu_samples.iter()
            .map(|s| s.total_usage_percent)
            .sum::<f32>() / self.cpu_samples.len() as f32;

        // Check against computational budget
        if average_cpu > self.computational_budget.max_cpu_usage_percent {
            issues.push(CpuIssue {
                issue_type: CpuIssueType::BudgetExceeded,
                message: format!(
                    "CPU usage ({:.1}%) exceeds computational budget ({:.1}%)",
                    average_cpu,
                    self.computational_budget.max_cpu_usage_percent
                ),
                severity: IssueSeverity::High,
                suggestion: "Optimize AI systems or reduce agent count to stay within computational budget".to_string(),
            });
        }

        // Analyze system-specific CPU usage
        let system_issues = self.analyze_system_cpu_usage();
        issues.extend(system_issues);

        issues
    }

    fn analyze_system_cpu_usage(&self) -> Vec<CpuIssue> {
        let mut issues = Vec::new();

        // Calculate average CPU usage per system
        let mut ai_system_total = 0.0;
        let mut social_system_total = 0.0;
        let mut physics_system_total = 0.0;
        let sample_count = self.cpu_samples.len() as f32;

        for sample in &self.cpu_samples {
            ai_system_total += sample.system_breakdown.ai_systems_percent;
            social_system_total += sample.system_breakdown.social_systems_percent;
            physics_system_total += sample.system_breakdown.physics_systems_percent;
        }

        let ai_average = ai_system_total / sample_count;
        let social_average = social_system_total / sample_count;
        let _physics_average = physics_system_total / sample_count;

        // Check AI system budget (should be largest consumer)
        if ai_average > self.computational_budget.ai_systems_budget_percent {
            issues.push(CpuIssue {
                issue_type: CpuIssueType::AiSystemBudgetExceeded,
                message: format!(
                    "AI systems CPU usage ({:.1}%) exceeds budget ({:.1}%)",
                    ai_average,
                    self.computational_budget.ai_systems_budget_percent
                ),
                severity: IssueSeverity::High,
                suggestion: "Optimize personality calculations, decision making, or social processing systems".to_string(),
            });
        }

        // Check social system budget
        if social_average > self.computational_budget.social_systems_budget_percent {
            issues.push(CpuIssue {
                issue_type: CpuIssueType::SocialSystemBudgetExceeded,
                message: format!(
                    "Social systems CPU usage ({:.1}%) exceeds budget ({:.1}%)",
                    social_average,
                    self.computational_budget.social_systems_budget_percent
                ),
                severity: IssueSeverity::Medium,
                suggestion: "Optimize emotional contagion, relationship tracking, or communication systems".to_string(),
            });
        }

        issues
    }
}

#[derive(Debug, Clone)]
pub struct CpuUsageSample {
    pub timestamp: Instant,
    pub total_usage_percent: f32,
    pub system_breakdown: SystemCpuBreakdown,
}

#[derive(Debug, Clone)]
pub struct SystemCpuBreakdown {
    pub ai_systems_percent: f32,
    pub social_systems_percent: f32,
    pub physics_systems_percent: f32,
    pub rendering_systems_percent: f32,
    pub other_systems_percent: f32,
}

#[derive(Debug)]
pub struct ComputationalBudget {
    pub max_cpu_usage_percent: f32,
    pub ai_systems_budget_percent: f32,
    pub social_systems_budget_percent: f32,
    pub physics_systems_budget_percent: f32,
    pub rendering_systems_budget_percent: f32,
}

impl ComputationalBudget {
    pub fn new() -> Self {
        Self {
            max_cpu_usage_percent: 80.0, // 80% max CPU usage
            ai_systems_budget_percent: 40.0, // 40% for AI systems
            social_systems_budget_percent: 20.0, // 20% for social systems
            physics_systems_budget_percent: 10.0, // 10% for physics
            rendering_systems_budget_percent: 10.0, // 10% for rendering
        }
    }
}

#[derive(Debug)]
pub struct SystemCpuTracker {
    pub tracking_active: bool,
    pub system_timings: std::collections::HashMap<String, Vec<Duration>>,
}

impl SystemCpuTracker {
    pub fn new() -> Self {
        Self {
            tracking_active: false,
            system_timings: std::collections::HashMap::new(),
        }
    }

    pub fn start_tracking(&mut self) {
        self.tracking_active = true;
        self.system_timings.clear();
    }

    pub fn record_system_timing(&mut self, system_name: String, duration: Duration) {
        if self.tracking_active {
            self.system_timings.entry(system_name).or_insert_with(Vec::new).push(duration);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpuIssue {
    pub issue_type: CpuIssueType,
    pub message: String,
    pub severity: IssueSeverity,
    pub suggestion: String,
}

#[derive(Debug, Clone)]
pub enum CpuIssueType {
    BudgetExceeded,
    AiSystemBudgetExceeded,
    SocialSystemBudgetExceeded,
    PhysicsSystemBudgetExceeded,
    SystemBottleneck,
}

/// Profiles individual systems for performance impact identification
#[derive(Debug)]
pub struct SystemProfiler {
    pub system_profiles: std::collections::HashMap<String, SystemProfile>,
    pub profiling_active: bool,
}

impl SystemProfiler {
    pub fn new() -> Self {
        Self {
            system_profiles: std::collections::HashMap::new(),
            profiling_active: false,
        }
    }

    /// Start system profiling
    pub fn start_profiling(&mut self) {
        self.profiling_active = true;
        self.system_profiles.clear();
    }

    /// Record system execution data
    pub fn record_system_execution(&mut self, system_name: String, execution_data: SystemExecutionData) {
        if self.profiling_active {
            let profile = self.system_profiles.entry(system_name).or_insert_with(SystemProfile::new);
            profile.add_execution_data(execution_data);
        }
    }

    /// Generate profiling report with optimization suggestions
    pub fn generate_profiling_report(&self) -> ProfilingReport {
        let mut system_reports = Vec::new();

        for (system_name, profile) in &self.system_profiles {
            let report = SystemReport {
                system_name: system_name.clone(),
                average_execution_time_ms: profile.calculate_average_execution_time(),
                peak_execution_time_ms: profile.calculate_peak_execution_time(),
                total_execution_count: profile.execution_data.len(),
                performance_impact: profile.calculate_performance_impact(),
                optimization_suggestions: profile.generate_optimization_suggestions(),
            };
            system_reports.push(report);
        }

        // Sort by performance impact (highest first)
        system_reports.sort_by(|a, b| b.performance_impact.partial_cmp(&a.performance_impact).unwrap());

        ProfilingReport {
            system_reports,
            total_systems_profiled: self.system_profiles.len(),
            profiling_duration: Duration::from_secs(60), // Placeholder
        }
    }
}

#[derive(Debug)]
pub struct SystemProfile {
    pub execution_data: Vec<SystemExecutionData>,
}

impl SystemProfile {
    pub fn new() -> Self {
        Self {
            execution_data: Vec::new(),
        }
    }

    pub fn add_execution_data(&mut self, data: SystemExecutionData) {
        self.execution_data.push(data);
    }

    pub fn calculate_average_execution_time(&self) -> f32 {
        if self.execution_data.is_empty() {
            return 0.0;
        }

        let total_time: f32 = self.execution_data.iter()
            .map(|d| d.execution_time.as_secs_f32() * 1000.0)
            .sum();
        
        total_time / self.execution_data.len() as f32
    }

    pub fn calculate_peak_execution_time(&self) -> f32 {
        self.execution_data.iter()
            .map(|d| d.execution_time.as_secs_f32() * 1000.0)
            .fold(0.0, f32::max)
    }

    pub fn calculate_performance_impact(&self) -> f32 {
        // Performance impact = average_time * execution_frequency
        let avg_time = self.calculate_average_execution_time();
        let frequency = self.execution_data.len() as f32 / 60.0; // Executions per second (assuming 60 second test)
        avg_time * frequency
    }

    pub fn generate_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        let avg_time = self.calculate_average_execution_time();
        let peak_time = self.calculate_peak_execution_time();

        if avg_time > 5.0 {
            suggestions.push("Consider optimizing this system - average execution time exceeds 5ms".to_string());
        }

        if peak_time > 16.0 {
            suggestions.push("Peak execution time exceeds frame budget - investigate performance spikes".to_string());
        }

        if peak_time / avg_time > 3.0 {
            suggestions.push("High variance in execution time - check for conditional complexity".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("System performance is within acceptable limits".to_string());
        }

        suggestions
    }
}

#[derive(Debug, Clone)]
pub struct SystemExecutionData {
    pub timestamp: Instant,
    pub execution_time: Duration,
    pub entities_processed: usize,
    pub memory_allocated: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ProfilingReport {
    pub system_reports: Vec<SystemReport>,
    pub total_systems_profiled: usize,
    pub profiling_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct SystemReport {
    pub system_name: String,
    pub average_execution_time_ms: f32,
    pub peak_execution_time_ms: f32,
    pub total_execution_count: usize,
    pub performance_impact: f32,
    pub optimization_suggestions: Vec<String>,
}