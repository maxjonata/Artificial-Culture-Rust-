use super::*;
use std::path::PathBuf;
use tokio;

#[tokio::test]
async fn test_pipeline_execution() {
    let config = CiCdConfiguration::default();
    let mut pipeline = CiCdPipeline::new(config);
    
    // Test files
    let test_files = vec![PathBuf::from("src/lib.rs")];
    let file_paths: Vec<&Path> = test_files.iter().map(|p| p.as_path()).collect();
    
    // Execute pipeline
    let result = pipeline.execute_pipeline(ExecutionTrigger::Manual, &file_paths).await;
    
    assert!(result.is_ok());
    let pipeline_result = result.unwrap();
    
    // Verify basic structure
    assert!(!pipeline_result.execution_id.is_empty());
    assert!(pipeline_result.execution_time > Duration::from_secs(0));
    
    // Should have some stages executed
    assert!(!pipeline_result.stage_results.is_empty());
    
    println!("Pipeline execution completed successfully!");
    println!("Execution ID: {}", pipeline_result.execution_id);
    println!("Overall Result: {:?}", pipeline_result.overall_result);
    println!("Quality Score: {:.1}", pipeline_result.quality_metrics.code_quality_score);
}

#[tokio::test]
async fn test_failure_handling() {
    let mut failure_handler = CiFailureHandler::new();
    
    // Create a mock failed stage execution
    let failed_stage = StageExecution {
        stage_name: "test-stage".to_string(),
        start_time: Instant::now(),
        end_time: Some(Instant::now()),
        result: ExecutionResult::Failure,
        validation_results: vec![ValidationResult {
            file_path: PathBuf::from("test.rs"),
            passed: false,
            issues: vec![crate::cicd::validation::ai_patterns::ValidationIssue {
                issue_type: crate::cicd::validation::ai_patterns::IssueType::MissingBehavioralDocumentation,
                message: "Test failure".to_string(),
                file_path: PathBuf::from("test.rs"),
                line_number: 1,
                column_number: None,
                severity: IssueSeverity::Critical,
                suggestion: Some("Fix the test issue".to_string()),
                code_example: None,
            }],
            suggestions: vec!["Fix the issue".to_string()],
            execution_time: Duration::from_millis(100),
            check_type: CheckType::Manual,
        }],
        retry_count: 0,
        error_message: Some("Stage failed".to_string()),
    };
    
    // Create failure context
    let context = failure_handling::FailureContext {
        git_commit: "abc123".to_string(),
        branch_name: "main".to_string(),
        author: "test-author".to_string(),
        changed_files: vec![PathBuf::from("test.rs")],
        environment_info: failure_handling::EnvironmentInfo {
            os: "test-os".to_string(),
            rust_version: "1.70.0".to_string(),
            cargo_version: "1.70.0".to_string(),
            available_memory: 8_000_000_000,
            cpu_cores: 4,
            disk_space: 100_000_000_000,
        },
        previous_success: None,
    };
    
    // Handle the failure
    let result = failure_handler.handle_stage_failure(&failed_stage, &context).await;
    
    // Verify failure handling
    assert!(result.handling_time > Duration::from_secs(0));
    assert!(!result.guidance.immediate_actions.is_empty());
    assert!(!result.guidance.diagnostic_commands.is_empty());
    
    println!("Failure handling completed successfully!");
    println!("Failure Type: {:?}", result.failure_analysis.failure_type);
    println!("Immediate Actions: {:?}", result.guidance.immediate_actions);
}

#[tokio::test]
async fn test_extended_validation() {
    let mut extended_validator = ExtendedValidationSystem::new();
    
    // Run extended validation
    let result = extended_validator.run_extended_validation().await;
    
    // Verify extended validation results
    assert!(result.validation_time > Duration::from_secs(0));
    assert!(result.stability_results.overall_stability_score > 0.0);
    assert!(result.long_term_results.long_term_stability_score > 0.0);
    assert!(!result.recommendations.is_empty());
    
    println!("Extended validation completed successfully!");
    println!("Stability Score: {:.1}", result.stability_results.overall_stability_score);
    println!("Long-term Score: {:.1}", result.long_term_results.long_term_stability_score);
}

#[tokio::test]
async fn test_summary_dashboard() {
    let mut dashboard = SummaryDashboard::new();
    
    // Create mock pipeline executions
    let pipeline_executions = vec![
        PipelineExecution {
            execution_id: "test-1".to_string(),
            start_time: Instant::now(),
            end_time: Some(Instant::now()),
            trigger: ExecutionTrigger::Manual,
            stages: HashMap::new(),
            overall_result: ExecutionResult::Success,
            quality_metrics: QualityMetrics {
                total_issues: 5,
                critical_issues: 0,
                high_issues: 2,
                medium_issues: 3,
                low_issues: 0,
                test_coverage: 85.0,
                performance_score: 90.0,
                behavioral_consistency_score: 88.0,
                code_quality_score: 92.0,
            },
        },
    ];
    
    // Generate dashboard
    let dashboard_report = dashboard.generate_dashboard(&pipeline_executions).await;
    
    // Verify dashboard generation
    assert_eq!(dashboard_report.total_executions, 1);
    assert!(dashboard_report.current_health_score > 0.0);
    assert!(!dashboard_report.quality_trends.trend_direction.is_empty());
    assert!(!dashboard_report.recommendations.is_empty());
    
    println!("Dashboard generation completed successfully!");
    println!("Health Score: {:.1}%", dashboard_report.current_health_score);
    println!("Quality Trend: {}", dashboard_report.quality_trends.trend_direction);
}

#[tokio::test]
async fn test_pipeline_completion_reporting() {
    let mut completion_reporter = PipelineCompletionReporter::new();
    
    // Create mock pipeline execution
    let pipeline_execution = PipelineExecution {
        execution_id: "test-completion".to_string(),
        start_time: Instant::now(),
        end_time: Some(Instant::now()),
        trigger: ExecutionTrigger::ContinuousIntegration,
        stages: HashMap::new(),
        overall_result: ExecutionResult::Success,
        quality_metrics: QualityMetrics {
            total_issues: 2,
            critical_issues: 0,
            high_issues: 1,
            medium_issues: 1,
            low_issues: 0,
            test_coverage: 78.0,
            performance_score: 85.0,
            behavioral_consistency_score: 82.0,
            code_quality_score: 88.0,
        },
    };
    
    // Report completion
    let completion_report = completion_reporter.report_pipeline_completion(&pipeline_execution).await;
    
    // Verify completion reporting
    assert_eq!(completion_report.execution_id, "test-completion");
    assert!(matches!(completion_report.overall_result, ExecutionResult::Success));
    assert!(!completion_report.summary.is_empty());
    assert!(!completion_report.next_steps.is_empty());
    
    println!("Completion reporting completed successfully!");
    println!("Summary: {}", completion_report.summary);
    println!("Next Steps: {:?}", completion_report.next_steps);
}

#[test]
fn test_configuration_default() {
    let config = CiCdConfiguration::default();
    
    // Verify default configuration
    assert!(!config.stages.is_empty());
    assert!(config.parallel_execution.enable_stage_parallelism);
    assert!(config.parallel_execution.enable_validation_parallelism);
    assert!(config.notification_config.enable_progress_notifications);
    assert!(config.quality_gates.max_critical_issues == 0);
    assert!(config.reporting_config.generate_detailed_reports);
    
    println!("Default configuration is valid!");
    println!("Stages: {}", config.stages.len());
    println!("Max concurrent stages: {}", config.parallel_execution.max_concurrent_stages);
}

#[test]
fn test_dependency_resolution() {
    let resolver = DependencyResolver;
    let stage_names = vec!["stage1".to_string(), "stage2".to_string(), "stage3".to_string()];
    
    let result = resolver.resolve_execution_order(&stage_names);
    
    assert!(result.is_ok());
    let execution_order = result.unwrap();
    
    // Should have at least one batch
    assert!(!execution_order.is_empty());
    
    // All stages should be included
    let total_stages: usize = execution_order.iter().map(|batch| batch.len()).sum();
    assert_eq!(total_stages, stage_names.len());
    
    println!("Dependency resolution completed successfully!");
    println!("Execution order: {:?}", execution_order);
}