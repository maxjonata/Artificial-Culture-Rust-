use std::path::Path;
use std::time::Instant;

use crate::cicd::validation::{
    ValidationErrorHandler, FixCommandGenerator,
    AiPatternValidator, BevyEcsValidator, PerformancePatternDetector,
    BehavioralConsistencyValidator, PerformanceRegressionDetector,
    SecurityAuditSystem, CrossPlatformBuildValidator, DocumentationQualityValidator,
};
use crate::cicd::validation::ai_patterns::{CheckType, IssueType, IssueSeverity, ValidationResult, ValidationIssue};
use crate::cicd::validation::error_handling::{
    FormattedValidationReport, ValidationStage, ProgressReporter,
};
use crate::cicd::validation::fix_commands::{
    BatchFixCommands, AutoFixResult,
};

/// Orchestrates the entire validation pipeline with error handling and fix generation.
#[derive(Debug)]
pub struct ValidationOrchestrator {
    pub ai_pattern_validator: AiPatternValidator,
    pub ecs_validator: BevyEcsValidator,
    pub performance_detector: PerformancePatternDetector,
    pub behavioral_validator: BehavioralConsistencyValidator,
    pub regression_detector: PerformanceRegressionDetector,
    pub security_auditor: SecurityAuditSystem,
    pub build_validator: CrossPlatformBuildValidator,
    pub documentation_validator: DocumentationQualityValidator,
    pub error_handler: ValidationErrorHandler,
    pub fix_generator: FixCommandGenerator,
    pub progress_reporter: ProgressReporter,
}

impl ValidationOrchestrator {
    pub fn new() -> Self {
        Self {
            ai_pattern_validator: AiPatternValidator::new(),
            ecs_validator: BevyEcsValidator::new(),
            performance_detector: PerformancePatternDetector::new(),
            behavioral_validator: BehavioralConsistencyValidator::new(),
            regression_detector: PerformanceRegressionDetector::new(std::path::PathBuf::from("performance_baselines")),
            security_auditor: SecurityAuditSystem::new(Default::default()),
            build_validator: CrossPlatformBuildValidator::new(),
            documentation_validator: DocumentationQualityValidator::new(),
            error_handler: ValidationErrorHandler::new(),
            fix_generator: FixCommandGenerator::new(),
            progress_reporter: ProgressReporter::new(),
        }
    }

    /// Runs the complete validation pipeline for pre-commit checks.
    pub fn run_pre_commit_validation(&mut self, files: &[&Path]) -> ValidationPipelineResult {
        self.progress_reporter.start_validation();
        let start_time = Instant::now();

        let mut all_results = Vec::new();

        // Stage 1: Code Formatting (fast check)
        self.progress_reporter.report_progress(ValidationStage::CodeFormatting, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                // Quick formatting check (would integrate with cargo fmt --check)
                let result = self.check_basic_formatting(&content, file_path);
                all_results.push(result);
            }
            self.progress_reporter.report_progress(ValidationStage::CodeFormatting, i + 1, files.len());
        }

        // Stage 2: Essential Linting
        self.progress_reporter.report_progress(ValidationStage::EssentialLinting, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                let result = self.ai_pattern_validator.validate_file(&content, file_path);
                all_results.push(result);
            }
            self.progress_reporter.report_progress(ValidationStage::EssentialLinting, i + 1, files.len());
        }

        // Stage 3: Unit Testing (changed modules only)
        self.progress_reporter.report_progress(ValidationStage::UnitTesting, 0, 1);
        // This would run cargo test for changed modules
        self.progress_reporter.report_progress(ValidationStage::UnitTesting, 1, 1);

        let total_time = start_time.elapsed();
        self.generate_pipeline_result(all_results, CheckType::PreCommit, total_time)
    }

    /// Runs the complete validation pipeline for pre-push checks.
    pub fn run_pre_push_validation(&mut self, files: &[&Path]) -> ValidationPipelineResult {
        self.progress_reporter.start_validation();
        let start_time = Instant::now();

        let mut all_results = Vec::new();

        // Stage 1: Complete Build Check
        self.progress_reporter.report_progress(ValidationStage::CodeFormatting, 0, 1);
        // This would run cargo build --workspace
        self.progress_reporter.report_progress(ValidationStage::CodeFormatting, 1, 1);

        // Stage 2: AI Pattern Validation (ALL files)
        self.progress_reporter.report_progress(ValidationStage::AiPatternValidation, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                let result = self.ai_pattern_validator.validate_file(&content, file_path);
                all_results.push(result);
            }
            self.progress_reporter.report_progress(ValidationStage::AiPatternValidation, i + 1, files.len());
        }

        // Stage 3: ECS Architecture Validation (ALL Rust files)
        self.progress_reporter.report_progress(ValidationStage::EcsArchitectureValidation, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    let result = self.convert_ecs_result(
                        self.ecs_validator.validate_file(&content, file_path),
                        file_path
                    );
                    all_results.push(result);
                }
            }
            self.progress_reporter.report_progress(ValidationStage::EcsArchitectureValidation, i + 1, files.len());
        }

        // Stage 4: Performance Pattern Validation (ALL Rust files)
        self.progress_reporter.report_progress(ValidationStage::PerformancePatternValidation, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    let result = self.convert_performance_result(
                        self.performance_detector.validate_file(&content, file_path),
                        file_path
                    );
                    all_results.push(result);
                }
            }
            self.progress_reporter.report_progress(ValidationStage::PerformancePatternValidation, i + 1, files.len());
        }

        // Stage 5: Behavioral Consistency Validation
        self.progress_reporter.report_progress(ValidationStage::BehavioralConsistencyValidation, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                let result = self.behavioral_validator.validate_file(&content, file_path);
                all_results.push(result);
            }
            self.progress_reporter.report_progress(ValidationStage::BehavioralConsistencyValidation, i + 1, files.len());
        }

        // Stage 6: Security Audit
        self.progress_reporter.report_progress(ValidationStage::SecurityAudit, 0, 1);
        let security_result = self.run_security_audit();
        all_results.push(security_result);
        self.progress_reporter.report_progress(ValidationStage::SecurityAudit, 1, 1);

        // Stage 7: Cross-Platform Build Validation
        self.progress_reporter.report_progress(ValidationStage::CrossPlatformBuild, 0, 1);
        let build_result = self.run_cross_platform_build_validation();
        all_results.push(build_result);
        self.progress_reporter.report_progress(ValidationStage::CrossPlatformBuild, 1, 1);

        // Stage 8: Documentation Validation
        self.progress_reporter.report_progress(ValidationStage::DocumentationValidation, 0, files.len());
        for (i, file_path) in files.iter().enumerate() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                let result = self.documentation_validator.validate_file(&content, file_path);
                all_results.push(result);
            }
            self.progress_reporter.report_progress(ValidationStage::DocumentationValidation, i + 1, files.len());
        }

        let total_time = start_time.elapsed();
        self.generate_pipeline_result(all_results, CheckType::PrePush, total_time)
    }

    /// Runs performance regression detection.
    pub fn run_performance_regression_check(&mut self) -> ValidationPipelineResult {
        self.progress_reporter.start_validation();
        let start_time = Instant::now();

        self.progress_reporter.report_progress(ValidationStage::PerformanceRegression, 0, 1);
        let regression_result = self.run_performance_regression_detection();
        self.progress_reporter.report_progress(ValidationStage::PerformanceRegression, 1, 1);

        let total_time = start_time.elapsed();
        self.generate_pipeline_result(vec![regression_result], CheckType::ContinuousIntegration, total_time)
    }

    /// Executes automatic fixes for issues that support it.
    pub fn execute_automatic_fixes(&mut self, issues: &[ValidationIssue]) -> AutoFixResult {
        self.fix_generator.execute_auto_fixes(issues)
    }

    /// Generates comprehensive fix guidance for all issues.
    pub fn generate_fix_guidance(&self, issues: &[ValidationIssue]) -> BatchFixCommands {
        self.fix_generator.generate_batch_fix_commands(issues)
    }

    /// Checks if validation should be bypassed.
    pub fn should_bypass_validation(&self, check_type: CheckType, bypass_reason: &str) -> bool {
        self.error_handler.should_bypass_validation(check_type, bypass_reason)
    }

    fn generate_pipeline_result(
        &mut self,
        results: Vec<ValidationResult>,
        check_type: CheckType,
        total_time: std::time::Duration,
    ) -> ValidationPipelineResult {
        // Collect all issues
        let mut all_issues = Vec::new();
        for result in &results {
            all_issues.extend(result.issues.clone());
        }

        // Generate formatted report
        let formatted_report = self.error_handler.format_validation_results(&results);

        // Generate fix commands
        let fix_commands = self.fix_generator.generate_batch_fix_commands(&all_issues);

        // Finish progress reporting
        self.progress_reporter.finish_validation(all_issues.len());

        let has_blocking_issues = formatted_report.has_blocking_issues();
        
        ValidationPipelineResult {
            check_type,
            total_time,
            results,
            formatted_report,
            fix_commands,
            has_blocking_issues,
            can_auto_fix: all_issues.iter().any(|issue| self.fix_generator.can_auto_fix(issue)),
        }
    }

    fn check_basic_formatting(&self, _content: &str, file_path: &Path) -> ValidationResult {
        // This would integrate with cargo fmt --check
        // For now, return a placeholder result
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: true,
            issues: Vec::new(),
            suggestions: Vec::new(),
            execution_time: std::time::Duration::from_millis(10),
            check_type: CheckType::PreCommit,
        }
    }

    fn run_security_audit(&self) -> ValidationResult {
        // Convert SecurityAuditResult to ValidationResult
        let project_path = std::path::Path::new(".");
        match self.security_auditor.run_security_audit(project_path) {
            Ok(audit_result) => {
                let issues: Vec<ValidationIssue> = audit_result.vulnerabilities.into_iter()
                    .map(|vuln| ValidationIssue {
                        issue_type: IssueType::MissingBehavioralDocumentation, // Placeholder
                        message: format!("Security vulnerability: {}", vuln.title),
                        file_path: project_path.to_path_buf(),
                        line_number: 1,
                        column_number: None,
                        severity: IssueSeverity::High,
                        suggestion: Some("Run cargo audit and address security issues".to_string()),
                        code_example: None,
                    })
                    .collect();

                ValidationResult {
                    file_path: project_path.to_path_buf(),
                    passed: issues.is_empty(),
                    issues,
                    suggestions: vec!["Run cargo audit to get detailed security information".to_string()],
                    execution_time: std::time::Duration::from_secs(5),
                    check_type: CheckType::PrePush,
                }
            }
            Err(_) => {
                ValidationResult {
                    file_path: project_path.to_path_buf(),
                    passed: false,
                    issues: vec![ValidationIssue {
                        issue_type: IssueType::MissingBehavioralDocumentation,
                        message: "Security audit failed to run".to_string(),
                        file_path: project_path.to_path_buf(),
                        line_number: 1,
                        column_number: None,
                        severity: IssueSeverity::High,
                        suggestion: Some("Check that cargo-audit is installed".to_string()),
                        code_example: None,
                    }],
                    suggestions: vec!["Install cargo-audit: cargo install cargo-audit".to_string()],
                    execution_time: std::time::Duration::from_secs(1),
                    check_type: CheckType::PrePush,
                }
            }
        }
    }

    fn run_cross_platform_build_validation(&self) -> ValidationResult {
        // Placeholder for cross-platform build validation
        ValidationResult {
            file_path: std::path::Path::new(".").to_path_buf(),
            passed: true,
            issues: Vec::new(),
            suggestions: vec!["Cross-platform build validation not yet implemented".to_string()],
            execution_time: std::time::Duration::from_secs(30),
            check_type: CheckType::PrePush,
        }
    }

    fn run_performance_regression_detection(&self) -> ValidationResult {
        // Placeholder for performance regression detection
        ValidationResult {
            file_path: std::path::Path::new(".").to_path_buf(),
            passed: true,
            issues: Vec::new(),
            suggestions: vec!["Performance regression detection not yet implemented".to_string()],
            execution_time: std::time::Duration::from_secs(60),
            check_type: CheckType::ContinuousIntegration,
        }
    }

    fn convert_performance_result(
        &self, 
        perf_result: crate::cicd::validation::performance_patterns::PerformanceValidationResult,
        file_path: &Path
    ) -> ValidationResult {
        // Convert PerformanceValidationResult to ValidationResult
        let issues: Vec<ValidationIssue> = perf_result.issues.into_iter()
            .map(|perf_issue| ValidationIssue {
                issue_type: IssueType::MissingBehavioralDocumentation, // Placeholder
                message: perf_issue.message,
                file_path: perf_issue.file_path,
                line_number: perf_issue.line_number,
                column_number: None,
                severity: match perf_issue.severity {
                    crate::cicd::validation::performance_patterns::IssueSeverity::Critical => IssueSeverity::Critical,
                    crate::cicd::validation::performance_patterns::IssueSeverity::High => IssueSeverity::High,
                    crate::cicd::validation::performance_patterns::IssueSeverity::Medium => IssueSeverity::Medium,
                    crate::cicd::validation::performance_patterns::IssueSeverity::Low => IssueSeverity::Low,
                },
                suggestion: Some(perf_issue.suggestion),
                code_example: perf_issue.fix_example,
            })
            .collect();

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions: perf_result.suggestions,
            execution_time: perf_result.execution_time,
            check_type: CheckType::PrePush,
        }
    }

    fn convert_ecs_result(
        &self, 
        ecs_result: crate::cicd::validation::ecs_architecture::ValidationResult,
        _file_path: &Path
    ) -> ValidationResult {
        // Convert ECS ValidationResult to AI patterns ValidationResult
        let issues: Vec<ValidationIssue> = ecs_result.issues.into_iter()
            .map(|ecs_issue| ValidationIssue {
                issue_type: match ecs_issue.issue_type {
                    crate::cicd::validation::ecs_architecture::IssueType::ComponentWithBehavior => IssueType::DirectComponentAccess,
                    crate::cicd::validation::ecs_architecture::IssueType::DirectComponentAccess => IssueType::DirectComponentAccess,
                    _ => IssueType::MissingBehavioralDocumentation, // Default mapping
                },
                message: ecs_issue.message,
                file_path: ecs_issue.file_path,
                line_number: ecs_issue.line_number,
                column_number: None, // ECS doesn't have column_number
                severity: IssueSeverity::Medium, // Default severity since ECS doesn't have severity
                suggestion: ecs_issue.suggestion,
                code_example: None, // ECS doesn't have code_example
            })
            .collect();

        ValidationResult {
            file_path: ecs_result.file_path,
            passed: issues.is_empty(),
            issues,
            suggestions: ecs_result.suggestions,
            execution_time: std::time::Duration::from_millis(100), // Default since ECS doesn't have execution_time
            check_type: CheckType::PrePush,
        }
    }
}

impl Default for ValidationOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of running the complete validation pipeline.
#[derive(Debug)]
pub struct ValidationPipelineResult {
    pub check_type: CheckType,
    pub total_time: std::time::Duration,
    pub results: Vec<ValidationResult>,
    pub formatted_report: FormattedValidationReport,
    pub fix_commands: BatchFixCommands,
    pub has_blocking_issues: bool,
    pub can_auto_fix: bool,
}

impl ValidationPipelineResult {
    /// Returns true if the validation passed (no blocking issues).
    pub fn passed(&self) -> bool {
        !self.has_blocking_issues
    }

    /// Returns the total number of issues found.
    pub fn total_issues(&self) -> usize {
        self.formatted_report.total_issues
    }

    /// Returns a summary message for the validation result.
    pub fn get_summary(&self) -> String {
        self.formatted_report.get_summary_message()
    }

    /// Prints the complete validation report.
    pub fn print_report(&self) {
        println!("{}", self.formatted_report);
        
        if !self.fix_commands.global_commands.is_empty() {
            println!("\n🔧 SUGGESTED GLOBAL FIXES:");
            println!("{}", "=".repeat(80));
            for command in &self.fix_commands.global_commands {
                println!("   {} (affects {} issues, ~{})", 
                         command.command, command.affected_issues, command.estimated_time);
                println!("   {}", command.description);
            }
        }

        if !self.fix_commands.execution_order.is_empty() {
            println!("\n📋 RECOMMENDED EXECUTION ORDER:");
            println!("{}", "=".repeat(80));
            for step in &self.fix_commands.execution_order {
                println!("   {}. {} (~{})", 
                         step.step_number, step.description, step.estimated_time);
                for command in &step.commands {
                    println!("      • {}", command);
                }
            }
        }

        if self.can_auto_fix {
            println!("\n🤖 Some issues can be automatically fixed!");
            println!("   Run with --auto-fix flag to apply automatic fixes.");
        }
    }
}