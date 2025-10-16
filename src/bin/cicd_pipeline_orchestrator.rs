use std::path::{Path, PathBuf};
use std::env;
use clap::{Arg, Command};
use tokio;

use artificial_society::cicd::pipeline::{
    CiCdPipeline, CiCdConfiguration, ExecutionTrigger, PipelineError,
};
use artificial_society::cicd::pipeline::failure_handling::{
    CiFailureHandler, ExtendedValidationSystem, SummaryDashboard, PipelineCompletionReporter,
    FailureContext, EnvironmentInfo,
};

/// Main entry point for the CI/CD pipeline orchestrator.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("CI/CD Pipeline Orchestrator")
        .version("1.0")
        .author("Artificial Society Team")
        .about("Orchestrates CI/CD pipeline stages with comprehensive validation and reporting")
        .arg(
            Arg::new("trigger")
                .short('t')
                .long("trigger")
                .value_name("TRIGGER")
                .help("Pipeline trigger type")
                .value_parser(["pre-commit", "pre-push", "ci", "manual", "scheduled"])
                .default_value("manual")
        )
        .arg(
            Arg::new("files")
                .short('f')
                .long("files")
                .value_name("FILES")
                .help("Files to validate (comma-separated)")
                .default_value(".")
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to CI/CD configuration file")
        )
        .arg(
            Arg::new("extended-validation")
                .long("extended-validation")
                .help("Run extended validation for release deployments")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("generate-dashboard")
                .long("generate-dashboard")
                .help("Generate summary dashboard with metrics and trends")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("parallel")
                .short('p')
                .long("parallel")
                .help("Enable parallel execution of stages")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("Global timeout for pipeline execution")
                .value_parser(clap::value_parser!(u64))
                .default_value("3600")
        )
        .arg(
            Arg::new("output-dir")
                .short('o')
                .long("output-dir")
                .value_name("DIR")
                .help("Output directory for reports")
                .default_value("reports")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Initialize logging
    if matches.get_flag("verbose") {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    println!("🚀 Starting CI/CD Pipeline Orchestrator");

    // Parse command line arguments
    let trigger = parse_trigger(matches.get_one::<String>("trigger").unwrap())?;
    let files = parse_files(matches.get_one::<String>("files").unwrap())?;
    let config_path = matches.get_one::<String>("config");
    let extended_validation = matches.get_flag("extended-validation");
    let generate_dashboard = matches.get_flag("generate-dashboard");
    let timeout_seconds = *matches.get_one::<u64>("timeout").unwrap();
    let output_dir = PathBuf::from(matches.get_one::<String>("output-dir").unwrap());

    // Ensure output directory exists
    tokio::fs::create_dir_all(&output_dir).await?;

    // Load configuration
    let mut configuration = if let Some(config_path) = config_path {
        load_configuration(config_path).await?
    } else {
        CiCdConfiguration::default()
    };

    // Apply command line overrides
    configuration.timeout_config.global_timeout_seconds = timeout_seconds;
    if matches.get_flag("parallel") {
        configuration.parallel_execution.enable_stage_parallelism = true;
        configuration.parallel_execution.enable_validation_parallelism = true;
    }

    // Initialize pipeline components
    let mut pipeline = CiCdPipeline::new(configuration);
    let mut failure_handler = CiFailureHandler::new();
    let mut extended_validator = ExtendedValidationSystem::new();
    let mut dashboard = SummaryDashboard::new();
    let mut completion_reporter = PipelineCompletionReporter::new();

    // Generate dashboard if requested
    if generate_dashboard {
        println!("📊 Generating summary dashboard...");
        let dashboard_report = generate_summary_dashboard(&mut dashboard, &output_dir).await?;
        println!("✅ Dashboard generated: {:?}", dashboard_report.generated_at);
        return Ok(());
    }

    // Run extended validation if requested
    if extended_validation {
        println!("🔍 Running extended validation for release deployment...");
        let extended_result = extended_validator.run_extended_validation().await;
        
        println!("📋 EXTENDED VALIDATION RESULTS:");
        println!("   Duration: {:?}", extended_result.validation_time);
        println!("   Stability Score: {:.1}", extended_result.stability_results.overall_stability_score);
        println!("   Long-term Score: {:.1}", extended_result.long_term_results.long_term_stability_score);
        println!("   Overall Passed: {}", extended_result.overall_passed);
        
        if !extended_result.recommendations.is_empty() {
            println!("   Recommendations:");
            for rec in &extended_result.recommendations {
                println!("     • {}", rec);
            }
        }

        // Save extended validation report
        let report_path = output_dir.join("extended_validation_report.json");
        save_extended_validation_report(&extended_result, &report_path).await?;
        println!("📄 Extended validation report saved to: {}", report_path.display());

        return Ok(());
    }

    // Prepare file paths for validation
    let file_paths: Vec<&Path> = files.iter().map(|p| p.as_path()).collect();

    println!("🔧 Executing pipeline with trigger: {:?}", trigger);
    println!("📁 Validating {} files", file_paths.len());

    // Execute the main pipeline
    let pipeline_result = match pipeline.execute_pipeline(trigger.clone(), &file_paths).await {
        Ok(result) => result,
        Err(PipelineError::StageExecutionError(msg)) => {
            eprintln!("❌ Pipeline execution failed: {}", msg);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Pipeline error: {}", e);
            std::process::exit(1);
        }
    };

    // Report pipeline completion
    let completion_report = completion_reporter.report_pipeline_completion(
        &create_pipeline_execution_from_result(&pipeline_result)
    ).await;

    // Handle failures if any
    if !matches!(pipeline_result.overall_result, artificial_society::cicd::pipeline::ExecutionResult::Success) {
        println!("\n🚨 HANDLING PIPELINE FAILURES");
        
        let failed_stages: Vec<_> = pipeline_result.stage_results.values()
            .filter(|stage| !matches!(stage.result, artificial_society::cicd::pipeline::ExecutionResult::Success))
            .cloned()
            .collect();

        if !failed_stages.is_empty() {
            let failure_context = create_failure_context().await;
            
            for failed_stage in &failed_stages {
                let failure_result = failure_handler.handle_stage_failure(failed_stage, &failure_context).await;
                
                println!("\n🔍 FAILURE ANALYSIS FOR STAGE: {}", failed_stage.stage_name);
                println!("   Type: {:?}", failure_result.failure_analysis.failure_type);
                println!("   Severity: {:?}", failure_result.failure_analysis.severity);
                
                if let Some(cause) = &failure_result.failure_analysis.root_cause {
                    println!("   Root Cause: {}", cause);
                }
                
                println!("   Immediate Actions:");
                for action in &failure_result.guidance.immediate_actions {
                    println!("     • {}", action);
                }
                
                println!("   Diagnostic Commands:");
                for command in &failure_result.guidance.diagnostic_commands {
                    println!("     $ {}", command);
                }
            }

            // Generate comprehensive failure report
            let pipeline_execution = create_pipeline_execution_from_result(&pipeline_result);
            let comprehensive_report = failure_handler.generate_failure_report(
                &failed_stages,
                &pipeline_execution
            ).await;

            println!("\n📋 COMPREHENSIVE FAILURE REPORT:");
            println!("   Pipeline ID: {}", comprehensive_report.pipeline_id);
            println!("   Failed Stages: {}", comprehensive_report.total_failed_stages);
            println!("   Estimated Fix Time: {:?}", comprehensive_report.estimated_fix_time);
            
            if !comprehensive_report.recommendations.is_empty() {
                println!("   Recommendations:");
                for rec in &comprehensive_report.recommendations {
                    println!("     • {} (Priority: {:?})", rec.title, rec.priority);
                    println!("       {}", rec.description);
                    println!("       Estimated Effort: {:?}", rec.estimated_effort);
                }
            }

            // Save failure report
            let failure_report_path = output_dir.join(format!("failure_report_{}.json", pipeline_result.execution_id));
            save_failure_report(&comprehensive_report, &failure_report_path).await?;
            println!("📄 Failure report saved to: {}", failure_report_path.display());
        }
    }

    // Print final results
    println!("\n📊 PIPELINE EXECUTION SUMMARY:");
    println!("   Execution ID: {}", pipeline_result.execution_id);
    println!("   Overall Result: {:?}", pipeline_result.overall_result);
    println!("   Execution Time: {:?}", pipeline_result.execution_time);
    println!("   Quality Score: {:.1}", pipeline_result.quality_metrics.code_quality_score);
    println!("   Total Issues: {}", pipeline_result.quality_metrics.total_issues);
    println!("   Critical Issues: {}", pipeline_result.quality_metrics.critical_issues);
    println!("   High Issues: {}", pipeline_result.quality_metrics.high_issues);

    // Save comprehensive report
    let comprehensive_report_path = output_dir.join(format!("pipeline_report_{}.json", pipeline_result.execution_id));
    save_comprehensive_report(&pipeline_result.comprehensive_report, &comprehensive_report_path).await?;
    println!("📄 Comprehensive report saved to: {}", comprehensive_report_path.display());

    // Print recommendations from comprehensive report
    if !pipeline_result.comprehensive_report.recommendations.is_empty() {
        println!("\n💡 RECOMMENDATIONS:");
        for rec in &pipeline_result.comprehensive_report.recommendations {
            println!("   • {}", rec);
        }
    }

    // Exit with appropriate code
    match pipeline_result.overall_result {
        artificial_society::cicd::pipeline::ExecutionResult::Success => {
            println!("\n✅ Pipeline completed successfully!");
            Ok(())
        }
        artificial_society::cicd::pipeline::ExecutionResult::Warning => {
            println!("\n⚠️ Pipeline completed with warnings.");
            std::process::exit(1);
        }
        _ => {
            println!("\n❌ Pipeline failed.");
            std::process::exit(1);
        }
    }
}

/// Parses the trigger type from command line argument.
fn parse_trigger(trigger_str: &str) -> Result<ExecutionTrigger, Box<dyn std::error::Error>> {
    match trigger_str {
        "pre-commit" => Ok(ExecutionTrigger::PreCommit),
        "pre-push" => Ok(ExecutionTrigger::PrePush),
        "ci" => Ok(ExecutionTrigger::ContinuousIntegration),
        "manual" => Ok(ExecutionTrigger::Manual),
        "scheduled" => Ok(ExecutionTrigger::Scheduled),
        _ => Err(format!("Invalid trigger type: {}", trigger_str).into()),
    }
}

/// Parses the files list from command line argument.
fn parse_files(files_str: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if files_str == "." {
        // Scan current directory for Rust files
        let mut files = Vec::new();
        scan_rust_files(Path::new("."), &mut files)?;
        Ok(files)
    } else {
        // Parse comma-separated file list
        Ok(files_str.split(',')
            .map(|s| PathBuf::from(s.trim()))
            .collect())
    }
}

/// Recursively scans for Rust files in a directory.
fn scan_rust_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip target and .git directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "target" && name != ".git" && !name.starts_with('.') {
                        scan_rust_files(&path, files)?;
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    Ok(())
}

/// Loads configuration from a file.
async fn load_configuration(config_path: &str) -> Result<CiCdConfiguration, Box<dyn std::error::Error>> {
    let config_content = tokio::fs::read_to_string(config_path).await?;
    let configuration: CiCdConfiguration = serde_json::from_str(&config_content)?;
    Ok(configuration)
}

/// Generates a summary dashboard.
async fn generate_summary_dashboard(
    dashboard: &mut SummaryDashboard,
    output_dir: &Path,
) -> Result<artificial_society::cicd::pipeline::failure_handling::DashboardReport, Box<dyn std::error::Error>> {
    // Load historical pipeline executions (placeholder)
    let pipeline_executions = Vec::new(); // In real implementation, load from storage
    
    let dashboard_report = dashboard.generate_dashboard(&pipeline_executions).await;
    
    // Save dashboard report
    let dashboard_path = output_dir.join("dashboard_report.json");
    save_dashboard_report(&dashboard_report, &dashboard_path).await?;
    
    println!("📊 SUMMARY DASHBOARD:");
    println!("   Total Executions: {}", dashboard_report.total_executions);
    println!("   Current Health Score: {:.1}%", dashboard_report.current_health_score);
    println!("   Quality Trend: {}", dashboard_report.quality_trends.trend_direction);
    println!("   Performance Trend: {}", dashboard_report.performance_trends.performance_trend);
    println!("   Success Rate: {}", dashboard_report.success_rate_trends.success_rate_trend);
    
    if !dashboard_report.alerts.is_empty() {
        println!("   Active Alerts:");
        for alert in &dashboard_report.alerts {
            println!("     • {} ({}): {}", alert.title, format!("{:?}", alert.severity), alert.description);
        }
    }
    
    if !dashboard_report.recommendations.is_empty() {
        println!("   Recommendations:");
        for rec in &dashboard_report.recommendations {
            println!("     • {}", rec);
        }
    }
    
    Ok(dashboard_report)
}

/// Creates a failure context for the current environment.
async fn create_failure_context() -> FailureContext {
    FailureContext {
        git_commit: get_git_commit().unwrap_or_else(|| "unknown".to_string()),
        branch_name: get_git_branch().unwrap_or_else(|| "unknown".to_string()),
        author: get_git_author().unwrap_or_else(|| "unknown".to_string()),
        changed_files: get_changed_files().unwrap_or_default(),
        environment_info: get_environment_info(),
        previous_success: None, // Would be loaded from history
    }
}

/// Gets the current git commit hash.
fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Gets the current git branch name.
fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Gets the git author of the latest commit.
fn get_git_author() -> Option<String> {
    std::process::Command::new("git")
        .args(&["log", "-1", "--pretty=format:%an"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Gets the list of changed files.
fn get_changed_files() -> Option<Vec<PathBuf>> {
    std::process::Command::new("git")
        .args(&["diff", "--name-only", "HEAD~1"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| {
                    s.lines()
                        .filter(|line| !line.is_empty())
                        .map(PathBuf::from)
                        .collect()
                })
            } else {
                None
            }
        })
}

/// Gets environment information.
fn get_environment_info() -> EnvironmentInfo {
    EnvironmentInfo {
        os: env::consts::OS.to_string(),
        rust_version: get_rust_version().unwrap_or_else(|| "unknown".to_string()),
        cargo_version: get_cargo_version().unwrap_or_else(|| "unknown".to_string()),
        available_memory: get_available_memory(),
        cpu_cores: num_cpus::get(),
        disk_space: get_available_disk_space(),
    }
}

/// Gets the Rust version.
fn get_rust_version() -> Option<String> {
    std::process::Command::new("rustc")
        .args(&["--version"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Gets the Cargo version.
fn get_cargo_version() -> Option<String> {
    std::process::Command::new("cargo")
        .args(&["--version"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Gets available memory (placeholder implementation).
fn get_available_memory() -> u64 {
    // In a real implementation, this would query system memory
    8_000_000_000 // 8GB placeholder
}

/// Gets available disk space (placeholder implementation).
fn get_available_disk_space() -> u64 {
    // In a real implementation, this would query disk space
    100_000_000_000 // 100GB placeholder
}

/// Creates a pipeline execution from the result.
fn create_pipeline_execution_from_result(
    result: &artificial_society::cicd::pipeline::PipelineExecutionResult,
) -> artificial_society::cicd::pipeline::PipelineExecution {
    use std::time::Instant;
    
    artificial_society::cicd::pipeline::PipelineExecution {
        execution_id: result.execution_id.clone(),
        start_time: Instant::now() - result.execution_time,
        end_time: Some(Instant::now()),
        trigger: ExecutionTrigger::Manual, // Placeholder
        stages: result.stage_results.clone(),
        overall_result: result.overall_result.clone(),
        quality_metrics: result.quality_metrics.clone(),
    }
}

/// Saves extended validation report to file.
async fn save_extended_validation_report(
    report: &artificial_society::cicd::pipeline::failure_handling::ExtendedValidationResult,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&serde_json::json!({
        "validation_time": format!("{:?}", report.validation_time),
        "overall_passed": report.overall_passed,
        "stability_score": report.stability_results.overall_stability_score,
        "long_term_score": report.long_term_results.long_term_stability_score,
        "recommendations": report.recommendations
    }))?;
    
    tokio::fs::write(path, json).await?;
    Ok(())
}

/// Saves failure report to file.
async fn save_failure_report(
    report: &artificial_society::cicd::pipeline::failure_handling::ComprehensiveFailureReport,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&serde_json::json!({
        "pipeline_id": report.pipeline_id,
        "failure_timestamp": format!("{:?}", report.failure_timestamp),
        "total_failed_stages": report.total_failed_stages,
        "estimated_fix_time": format!("{:?}", report.estimated_fix_time),
        "stage_failures": report.stage_failures.iter().map(|s| serde_json::json!({
            "stage_name": s.stage_name,
            "failure_type": format!("{:?}", s.failure_type),
            "critical_issues": s.critical_issues,
            "high_issues": s.high_issues,
            "suggested_actions": s.suggested_actions
        })).collect::<Vec<_>>(),
        "recommendations": report.recommendations.iter().map(|r| serde_json::json!({
            "priority": format!("{:?}", r.priority),
            "title": r.title,
            "description": r.description,
            "estimated_effort": format!("{:?}", r.estimated_effort),
            "success_probability": r.success_probability
        })).collect::<Vec<_>>()
    }))?;
    
    tokio::fs::write(path, json).await?;
    Ok(())
}

/// Saves comprehensive report to file.
async fn save_comprehensive_report(
    report: &artificial_society::cicd::pipeline::ComprehensiveReport,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&serde_json::json!({
        "execution_id": report.execution_id,
        "execution_time": format!("{:?}", report.execution_time),
        "overall_result": format!("{:?}", report.overall_result),
        "quality_metrics": {
            "total_issues": report.quality_metrics.total_issues,
            "critical_issues": report.quality_metrics.critical_issues,
            "high_issues": report.quality_metrics.high_issues,
            "medium_issues": report.quality_metrics.medium_issues,
            "low_issues": report.quality_metrics.low_issues,
            "code_quality_score": report.quality_metrics.code_quality_score,
            "performance_score": report.quality_metrics.performance_score,
            "behavioral_consistency_score": report.quality_metrics.behavioral_consistency_score,
            "test_coverage": report.quality_metrics.test_coverage
        },
        "stage_summaries": report.stage_summaries.iter().map(|s| serde_json::json!({
            "stage_name": s.stage_name,
            "execution_time": format!("{:?}", s.execution_time),
            "result": format!("{:?}", s.result),
            "issues_found": s.issues_found,
            "retry_count": s.retry_count
        })).collect::<Vec<_>>(),
        "validation_summary": {
            "total_files_validated": report.validation_summary.total_files_validated,
            "total_issues": report.validation_summary.total_issues,
            "validation_types_run": report.validation_summary.validation_types_run,
            "average_validation_time": format!("{:?}", report.validation_summary.average_validation_time)
        },
        "trends_analysis": {
            "quality_trend": report.trends_analysis.quality_trend,
            "performance_trend": report.trends_analysis.performance_trend,
            "success_rate_trend": report.trends_analysis.success_rate_trend
        },
        "recommendations": report.recommendations
    }))?;
    
    tokio::fs::write(path, json).await?;
    Ok(())
}

/// Saves dashboard report to file.
async fn save_dashboard_report(
    report: &artificial_society::cicd::pipeline::failure_handling::DashboardReport,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&serde_json::json!({
        "generated_at": format!("{:?}", report.generated_at),
        "total_executions": report.total_executions,
        "current_health_score": report.current_health_score,
        "quality_trends": {
            "trend_direction": report.quality_trends.trend_direction,
            "average_quality_score": report.quality_trends.average_quality_score,
            "quality_score_change": report.quality_trends.quality_score_change,
            "issue_count_trend": report.quality_trends.issue_count_trend
        },
        "performance_trends": {
            "average_execution_time": format!("{:?}", report.performance_trends.average_execution_time),
            "execution_time_trend": report.performance_trends.execution_time_trend,
            "performance_score": report.performance_trends.performance_score,
            "performance_change": report.performance_trends.performance_change
        },
        "success_rate_trends": {
            "current_success_rate": report.success_rate_trends.current_success_rate,
            "success_rate_trend": report.success_rate_trends.success_rate_trend,
            "consecutive_successes": report.success_rate_trends.consecutive_successes,
            "consecutive_failures": report.success_rate_trends.consecutive_failures
        },
        "alerts": report.alerts.iter().map(|a| serde_json::json!({
            "severity": format!("{:?}", a.severity),
            "title": a.title,
            "description": a.description,
            "action_required": a.action_required
        })).collect::<Vec<_>>(),
        "recommendations": report.recommendations
    }))?;
    
    tokio::fs::write(path, json).await?;
    Ok(())
}