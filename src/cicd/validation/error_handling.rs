use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::cicd::validation::ai_patterns::{ValidationResult, ValidationIssue, IssueType, IssueSeverity, CheckType};

/// Handles validation errors with graceful formatting and suggestion generation.
#[derive(Debug)]
pub struct ValidationErrorHandler {
    pub issue_formatters: HashMap<IssueType, Box<dyn IssueFormatter>>,
    pub suggestion_generators: HashMap<IssueType, Box<dyn SuggestionGenerator>>,
    pub bypass_mechanisms: BypassConfiguration,
    pub progress_reporter: ProgressReporter,
}

impl ValidationErrorHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            issue_formatters: HashMap::new(),
            suggestion_generators: HashMap::new(),
            bypass_mechanisms: BypassConfiguration::default(),
            progress_reporter: ProgressReporter::new(),
        };
        
        handler.register_default_formatters();
        handler.register_default_suggestion_generators();
        handler
    }

    /// Formats validation results with clear error messages and suggestions.
    pub fn format_validation_results(&self, results: &[ValidationResult]) -> FormattedValidationReport {
        let start_time = Instant::now();
        
        let mut report = FormattedValidationReport {
            total_files_checked: results.len(),
            total_issues: 0,
            critical_issues: 0,
            high_issues: 0,
            medium_issues: 0,
            low_issues: 0,
            info_issues: 0,
            formatted_issues: Vec::new(),
            suggestions: Vec::new(),
            bypass_instructions: Vec::new(),
            execution_summary: ExecutionSummary::default(),
        };

        for result in results {
            report.total_issues += result.issues.len();
            
            for issue in &result.issues {
                match issue.severity {
                    IssueSeverity::Critical => report.critical_issues += 1,
                    IssueSeverity::High => report.high_issues += 1,
                    IssueSeverity::Medium => report.medium_issues += 1,
                    IssueSeverity::Low => report.low_issues += 1,
                    IssueSeverity::Info => report.info_issues += 1,
                }

                let formatted_issue = self.format_issue(issue);
                report.formatted_issues.push(formatted_issue);

                // Generate suggestions for this issue type
                if let Some(generator) = self.suggestion_generators.get(&issue.issue_type) {
                    let suggestions = generator.generate_suggestions(issue);
                    report.suggestions.extend(suggestions);
                }
            }
        }

        // Add bypass instructions if there are critical issues
        if report.critical_issues > 0 {
            report.bypass_instructions = self.bypass_mechanisms.get_bypass_instructions();
        }

        report.execution_summary.total_time = start_time.elapsed();
        report.execution_summary.files_per_second = 
            results.len() as f64 / report.execution_summary.total_time.as_secs_f64();

        report
    }

    /// Formats a single validation issue with appropriate styling and context.
    pub fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        let formatter = self.issue_formatters.get(&issue.issue_type)
            .unwrap_or_else(|| self.issue_formatters.get(&IssueType::IncorrectPersonalityType).unwrap());
        
        formatter.format_issue(issue)
    }

    /// Determines if validation should be bypassed based on configuration.
    pub fn should_bypass_validation(&self, check_type: CheckType, bypass_reason: &str) -> bool {
        self.bypass_mechanisms.should_bypass(check_type, bypass_reason)
    }

    /// Reports progress during validation execution.
    pub fn report_progress(&mut self, stage: ValidationStage, current: usize, total: usize) {
        self.progress_reporter.report_progress(stage, current, total);
    }

    fn register_default_formatters(&mut self) {
        // AI Pattern Issue Formatters
        self.issue_formatters.insert(
            IssueType::IncorrectPersonalityType,
            Box::new(PersonalityTypeFormatter),
        );
        self.issue_formatters.insert(
            IssueType::MissingRangeDocumentation,
            Box::new(RangeDocumentationFormatter),
        );
        self.issue_formatters.insert(
            IssueType::MissingBipolarDocumentation,
            Box::new(BipolarDocumentationFormatter),
        );
        self.issue_formatters.insert(
            IssueType::MissingPersonalityModulation,
            Box::new(PersonalityModulationFormatter),
        );
        self.issue_formatters.insert(
            IssueType::IncorrectTemporalUsage,
            Box::new(TemporalUsageFormatter),
        );
        
        // ECS Architecture Issue Formatters
        self.issue_formatters.insert(
            IssueType::DirectComponentAccess,
            Box::new(ComponentAccessFormatter),
        );
        
        // Behavioral Consistency Issue Formatters
        self.issue_formatters.insert(
            IssueType::MissingPersonalityConsistency,
            Box::new(PersonalityConsistencyFormatter),
        );
        self.issue_formatters.insert(
            IssueType::HardcodedBehavioralValue,
            Box::new(HardcodedValueFormatter),
        );
    }

    fn register_default_suggestion_generators(&mut self) {
        // AI Pattern Suggestion Generators
        self.suggestion_generators.insert(
            IssueType::IncorrectPersonalityType,
            Box::new(PersonalityTypeSuggestionGenerator),
        );
        self.suggestion_generators.insert(
            IssueType::MissingRangeDocumentation,
            Box::new(RangeDocumentationSuggestionGenerator),
        );
        self.suggestion_generators.insert(
            IssueType::MissingBipolarDocumentation,
            Box::new(BipolarDocumentationSuggestionGenerator),
        );
        self.suggestion_generators.insert(
            IssueType::MissingPersonalityModulation,
            Box::new(PersonalityModulationSuggestionGenerator),
        );
        self.suggestion_generators.insert(
            IssueType::IncorrectTemporalUsage,
            Box::new(TemporalUsageSuggestionGenerator),
        );
        
        // ECS Architecture Suggestion Generators
        self.suggestion_generators.insert(
            IssueType::DirectComponentAccess,
            Box::new(ComponentAccessSuggestionGenerator),
        );
        
        // Behavioral Consistency Suggestion Generators
        self.suggestion_generators.insert(
            IssueType::MissingPersonalityConsistency,
            Box::new(PersonalityConsistencySuggestionGenerator),
        );
        self.suggestion_generators.insert(
            IssueType::HardcodedBehavioralValue,
            Box::new(HardcodedValueSuggestionGenerator),
        );
    }
}

impl Default for ValidationErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for bypass mechanisms in emergency situations.
#[derive(Debug, Clone)]
pub struct BypassConfiguration {
    pub emergency_bypass_enabled: bool,
    pub bypass_keywords: Vec<String>,
    pub bypass_instructions: Vec<String>,
    pub allowed_bypass_reasons: Vec<BypassReason>,
}

impl Default for BypassConfiguration {
    fn default() -> Self {
        Self {
            emergency_bypass_enabled: true,
            bypass_keywords: vec![
                "BYPASS_PRECOMMIT".to_string(),
                "BYPASS_PREPUSH".to_string(),
                "EMERGENCY_COMMIT".to_string(),
                "HOTFIX".to_string(),
            ],
            bypass_instructions: vec![
                "To bypass pre-commit checks: git commit -m \"BYPASS_PRECOMMIT: <reason>\"".to_string(),
                "To bypass pre-push checks: git commit -m \"BYPASS_PREPUSH: <reason>\"".to_string(),
                "For emergency situations: git commit -m \"EMERGENCY_COMMIT: <critical issue>\"".to_string(),
                "For hotfixes: git commit -m \"HOTFIX: <urgent fix description>\"".to_string(),
            ],
            allowed_bypass_reasons: vec![
                BypassReason::CriticalProduction,
                BypassReason::SecurityHotfix,
                BypassReason::BuildSystemFailure,
                BypassReason::DependencyIssue,
            ],
        }
    }
}

impl BypassConfiguration {
    pub fn should_bypass(&self, check_type: CheckType, bypass_reason: &str) -> bool {
        if !self.emergency_bypass_enabled {
            return false;
        }

        self.bypass_keywords.iter().any(|keyword| bypass_reason.contains(keyword))
    }

    pub fn get_bypass_instructions(&self) -> Vec<String> {
        self.bypass_instructions.clone()
    }
}

#[derive(Debug, Clone)]
pub enum BypassReason {
    CriticalProduction,
    SecurityHotfix,
    BuildSystemFailure,
    DependencyIssue,
    TestingInfrastructure,
}

/// Reports progress during validation execution with timing information.
#[derive(Debug)]
pub struct ProgressReporter {
    pub current_stage: Option<ValidationStage>,
    pub stage_start_time: Option<Instant>,
    pub total_start_time: Option<Instant>,
    pub show_progress: bool,
}

impl ProgressReporter {
    pub fn new() -> Self {
        Self {
            current_stage: None,
            stage_start_time: None,
            total_start_time: None,
            show_progress: true,
        }
    }

    pub fn start_validation(&mut self) {
        self.total_start_time = Some(Instant::now());
        if self.show_progress {
            println!("🚀 Starting validation pipeline...");
        }
    }

    pub fn report_progress(&mut self, stage: ValidationStage, current: usize, total: usize) {
        if !self.show_progress {
            return;
        }

        // Start new stage if different from current
        if self.current_stage.as_ref() != Some(&stage) {
            if let Some(start_time) = self.stage_start_time {
                let elapsed = start_time.elapsed();
                println!("✅ Completed in {:.2}s", elapsed.as_secs_f64());
            }
            
            self.current_stage = Some(stage.clone());
            self.stage_start_time = Some(Instant::now());
            println!("{} {}", stage.icon(), stage.description());
        }

        // Show progress bar
        let percentage = (current as f64 / total as f64 * 100.0) as usize;
        let bar_length = 30;
        let filled = (current * bar_length / total).min(bar_length);
        let bar = "█".repeat(filled) + &"░".repeat(bar_length - filled);
        
        print!("\r[{}] {}% ({}/{})", bar, percentage, current, total);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        if current == total {
            println!(); // New line after completion
        }
    }

    pub fn finish_validation(&mut self, total_issues: usize) {
        if let Some(start_time) = self.total_start_time {
            let elapsed = start_time.elapsed();
            if self.show_progress {
                if total_issues == 0 {
                    println!("🎉 All validation checks passed! Total time: {:.2}s", elapsed.as_secs_f64());
                } else {
                    println!("⚠️  Validation completed with {} issues. Total time: {:.2}s", total_issues, elapsed.as_secs_f64());
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStage {
    CodeFormatting,
    EssentialLinting,
    UnitTesting,
    DocumentationValidation,
    AiPatternValidation,
    EcsArchitectureValidation,
    PerformancePatternValidation,
    BehavioralConsistencyValidation,
    SecurityAudit,
    CrossPlatformBuild,
    PerformanceRegression,
}

impl ValidationStage {
    pub fn icon(&self) -> &'static str {
        match self {
            ValidationStage::CodeFormatting => "📝",
            ValidationStage::EssentialLinting => "🔍",
            ValidationStage::UnitTesting => "🧪",
            ValidationStage::DocumentationValidation => "📚",
            ValidationStage::AiPatternValidation => "🧠",
            ValidationStage::EcsArchitectureValidation => "⚙️",
            ValidationStage::PerformancePatternValidation => "⚡",
            ValidationStage::BehavioralConsistencyValidation => "🎭",
            ValidationStage::SecurityAudit => "🔒",
            ValidationStage::CrossPlatformBuild => "🏗️",
            ValidationStage::PerformanceRegression => "📊",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ValidationStage::CodeFormatting => "Checking code formatting...",
            ValidationStage::EssentialLinting => "Running essential linting checks...",
            ValidationStage::UnitTesting => "Executing unit tests...",
            ValidationStage::DocumentationValidation => "Validating documentation...",
            ValidationStage::AiPatternValidation => "Validating AI patterns...",
            ValidationStage::EcsArchitectureValidation => "Validating ECS architecture...",
            ValidationStage::PerformancePatternValidation => "Checking performance patterns...",
            ValidationStage::BehavioralConsistencyValidation => "Validating behavioral consistency...",
            ValidationStage::SecurityAudit => "Running security audit...",
            ValidationStage::CrossPlatformBuild => "Testing cross-platform builds...",
            ValidationStage::PerformanceRegression => "Checking performance regressions...",
        }
    }
}

/// Formatted validation report with all issues and suggestions.
#[derive(Debug)]
pub struct FormattedValidationReport {
    pub total_files_checked: usize,
    pub total_issues: usize,
    pub critical_issues: usize,
    pub high_issues: usize,
    pub medium_issues: usize,
    pub low_issues: usize,
    pub info_issues: usize,
    pub formatted_issues: Vec<FormattedIssue>,
    pub suggestions: Vec<String>,
    pub bypass_instructions: Vec<String>,
    pub execution_summary: ExecutionSummary,
}

impl FormattedValidationReport {
    pub fn has_blocking_issues(&self) -> bool {
        self.critical_issues > 0 || self.high_issues > 0
    }

    pub fn get_summary_message(&self) -> String {
        if self.total_issues == 0 {
            "🎉 All validation checks passed!".to_string()
        } else if self.has_blocking_issues() {
            format!(
                "❌ Validation failed: {} critical, {} high priority issues found",
                self.critical_issues, self.high_issues
            )
        } else {
            format!(
                "⚠️  Validation completed with {} medium/low priority issues",
                self.medium_issues + self.low_issues + self.info_issues
            )
        }
    }
}

impl fmt::Display for FormattedValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", "=".repeat(80))?;
        writeln!(f, "📋 VALIDATION REPORT")?;
        writeln!(f, "{}", "=".repeat(80))?;
        
        writeln!(f, "📊 Summary:")?;
        writeln!(f, "   Files checked: {}", self.total_files_checked)?;
        writeln!(f, "   Total issues: {}", self.total_issues)?;
        writeln!(f, "   Critical: {} | High: {} | Medium: {} | Low: {} | Info: {}", 
                 self.critical_issues, self.high_issues, self.medium_issues, 
                 self.low_issues, self.info_issues)?;
        writeln!(f, "   Execution time: {:.2}s", self.execution_summary.total_time.as_secs_f64())?;
        writeln!(f, "   Files/second: {:.1}", self.execution_summary.files_per_second)?;
        
        if !self.formatted_issues.is_empty() {
            writeln!(f, "\n🔍 Issues Found:")?;
            writeln!(f, "{}", "-".repeat(80))?;
            
            for issue in &self.formatted_issues {
                writeln!(f, "{}", issue)?;
            }
        }
        
        if !self.suggestions.is_empty() {
            writeln!(f, "\n💡 Suggestions:")?;
            writeln!(f, "{}", "-".repeat(80))?;
            
            for suggestion in &self.suggestions {
                writeln!(f, "   • {}", suggestion)?;
            }
        }
        
        if !self.bypass_instructions.is_empty() {
            writeln!(f, "\n🚨 Emergency Bypass Instructions:")?;
            writeln!(f, "{}", "-".repeat(80))?;
            
            for instruction in &self.bypass_instructions {
                writeln!(f, "   {}", instruction)?;
            }
        }
        
        writeln!(f, "\n{}", self.get_summary_message())?;
        writeln!(f, "{}", "=".repeat(80))
    }
}

/// Individual formatted validation issue.
#[derive(Debug)]
pub struct FormattedIssue {
    pub severity_icon: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub issue_type: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub code_example: Option<String>,
}

impl fmt::Display for FormattedIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}:{} [{}]", 
                 self.severity_icon, 
                 self.file_path.display(), 
                 self.line_number,
                 self.issue_type)?;
        writeln!(f, "   {}", self.message)?;
        
        if let Some(suggestion) = &self.suggestion {
            writeln!(f, "   💡 {}", suggestion)?;
        }
        
        if let Some(code_example) = &self.code_example {
            writeln!(f, "   📝 Example:")?;
            for line in code_example.lines() {
                writeln!(f, "      {}", line)?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ExecutionSummary {
    pub total_time: Duration,
    pub files_per_second: f64,
}

/// Trait for formatting specific types of validation issues.
pub trait IssueFormatter: std::fmt::Debug + Send + Sync {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue;
}

/// Trait for generating suggestions for specific types of validation issues.
pub trait SuggestionGenerator: std::fmt::Debug + Send + Sync {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String>;
}

// Issue Formatters Implementation

#[derive(Debug)]
pub struct PersonalityTypeFormatter;

impl IssueFormatter for PersonalityTypeFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Personality Type".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ❌ Incorrect:\npub openness: f32,\n\n// ✅ Correct:\npub openness: Normalized<f32>,".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct RangeDocumentationFormatter;

impl IssueFormatter for RangeDocumentationFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Range Documentation".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "/// Openness to experience: 0.0 (closed-minded) to 1.0 (very open)\npub openness: Normalized<f32>,".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct BipolarDocumentationFormatter;

impl IssueFormatter for BipolarDocumentationFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Bipolar Documentation".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "/// Emotional valence: -1.0 (very negative) to 1.0 (very positive)\npub valence: f32,".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct PersonalityModulationFormatter;

impl IssueFormatter for PersonalityModulationFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Personality Modulation".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ✅ Correct: Modulate behavior by personality\nlet openness_factor = personality.openness.value();\nlet final_behavior = base_behavior * openness_factor;".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct TemporalUsageFormatter;

impl IssueFormatter for TemporalUsageFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Temporal Usage".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ❌ Incorrect:\nlet now = std::time::Instant::now();\n\n// ✅ Correct:\nfn system(world_time: Res<WorldTime>) {\n    let elapsed = world_time.delta_time;\n}".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct ComponentAccessFormatter;

impl IssueFormatter for ComponentAccessFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Component Access".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ❌ Incorrect:\nworld.get::<Component>(entity)\n\n// ✅ Correct:\nfn system(query: Query<&Component>) {\n    for component in query.iter() { ... }\n}".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct PersonalityConsistencyFormatter;

impl IssueFormatter for PersonalityConsistencyFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Personality Consistency".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ✅ Ensure personality affects behavior consistently\nlet extraversion_factor = personality.extraversion.value();\nlet social_approach = base_approach * extraversion_factor;".to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub struct HardcodedValueFormatter;

impl IssueFormatter for HardcodedValueFormatter {
    fn format_issue(&self, issue: &ValidationIssue) -> FormattedIssue {
        FormattedIssue {
            severity_icon: get_severity_icon(&issue.severity),
            file_path: issue.file_path.clone(),
            line_number: issue.line_number,
            issue_type: "Hardcoded Value".to_string(),
            message: issue.message.clone(),
            suggestion: issue.suggestion.clone(),
            code_example: Some(
                "// ❌ Incorrect:\nif emotion.valence > 0.7 { ... }\n\n// ✅ Correct:\nconst HIGH_VALENCE_THRESHOLD: f32 = 0.7;\nif emotion.valence > HIGH_VALENCE_THRESHOLD { ... }".to_string()
            ),
        }
    }
}

// Suggestion Generators Implementation

#[derive(Debug)]
pub struct PersonalityTypeSuggestionGenerator;

impl SuggestionGenerator for PersonalityTypeSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Replace f32 with Normalized<f32> for personality traits".to_string(),
            "Add use crate::core::types::Normalized; to imports".to_string(),
            "Ensure personality trait values are always in 0.0-1.0 range".to_string(),
            "Consider using PersonalityVector struct for grouped traits".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct RangeDocumentationSuggestionGenerator;

impl SuggestionGenerator for RangeDocumentationSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Add documentation comment describing the valid range (0.0-1.0)".to_string(),
            "Include behavioral meaning of low and high values".to_string(),
            "Use format: /// Trait name: 0.0 (low meaning) to 1.0 (high meaning)".to_string(),
            "Reference Big Five personality model for standard interpretations".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct BipolarDocumentationSuggestionGenerator;

impl SuggestionGenerator for BipolarDocumentationSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Add documentation comment describing the bipolar range (-1.0 to 1.0)".to_string(),
            "Include meaning of negative, neutral (0.0), and positive values".to_string(),
            "Use format: /// Dimension: -1.0 (negative pole) to 1.0 (positive pole)".to_string(),
            "Add validation method to ensure values stay in valid range".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct PersonalityModulationSuggestionGenerator;

impl SuggestionGenerator for PersonalityModulationSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Add personality parameter to system function signature".to_string(),
            "Multiply base behavior by relevant personality trait values".to_string(),
            "Use personality.trait_name.value() to get f32 from Normalized<f32>".to_string(),
            "Consider multiple personality traits for complex behaviors".to_string(),
            "Document which personality traits affect this behavior".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct TemporalUsageSuggestionGenerator;

impl SuggestionGenerator for TemporalUsageSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Replace std::time::Instant with WorldTime resource".to_string(),
            "Add world_time: Res<WorldTime> to system parameters".to_string(),
            "Use world_time.delta_time for time-based calculations".to_string(),
            "Use world_time.current_time for timestamps".to_string(),
            "This ensures time scaling and simulation consistency".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct ComponentAccessSuggestionGenerator;

impl SuggestionGenerator for ComponentAccessSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Replace direct component access with Query parameters".to_string(),
            "Add Query<&ComponentType> to system function signature".to_string(),
            "Use query.iter() to iterate over components".to_string(),
            "Use query.get(entity) for specific entity access".to_string(),
            "This follows ECS architecture principles".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct PersonalityConsistencySuggestionGenerator;

impl SuggestionGenerator for PersonalityConsistencySuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Ensure personality traits consistently affect behavior".to_string(),
            "Add personality modulation to all behavioral systems".to_string(),
            "Document which personality traits influence each behavior".to_string(),
            "Test that personality differences produce observable behavior changes".to_string(),
            "Consider interaction effects between multiple personality traits".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct HardcodedValueSuggestionGenerator;

impl SuggestionGenerator for HardcodedValueSuggestionGenerator {
    fn generate_suggestions(&self, issue: &ValidationIssue) -> Vec<String> {
        vec![
            "Replace magic numbers with named constants".to_string(),
            "Move constants to a dedicated constants module".to_string(),
            "Use descriptive names that explain the threshold's purpose".to_string(),
            "Consider making thresholds configurable based on personality".to_string(),
            "Document why this specific threshold value was chosen".to_string(),
        ]
    }
}

// Helper functions

fn get_severity_icon(severity: &IssueSeverity) -> String {
    match severity {
        IssueSeverity::Critical => "🚨".to_string(),
        IssueSeverity::High => "❌".to_string(),
        IssueSeverity::Medium => "⚠️".to_string(),
        IssueSeverity::Low => "💡".to_string(),
        IssueSeverity::Info => "ℹ️".to_string(),
    }
}