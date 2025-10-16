use std::collections::HashMap;
use std::path::PathBuf;

use crate::cicd::validation::ai_patterns::{ValidationIssue, IssueType, IssueSeverity, CheckType};

/// Generates automatic fix commands and provides guidance for validation issues.
#[derive(Debug)]
pub struct FixCommandGenerator {
    pub command_generators: HashMap<IssueType, Box<dyn CommandGenerator>>,
    pub severity_handlers: HashMap<IssueSeverity, SeverityHandler>,
    pub bypass_manager: BypassManager,
}

impl FixCommandGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            command_generators: HashMap::new(),
            severity_handlers: HashMap::new(),
            bypass_manager: BypassManager::new(),
        };
        
        generator.register_command_generators();
        generator.register_severity_handlers();
        generator
    }

    /// Generates fix commands for a validation issue.
    pub fn generate_fix_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        let mut commands = FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(),
            manual_fixes: Vec::new(),
            guidance_steps: Vec::new(),
            code_examples: Vec::new(),
            bypass_options: Vec::new(),
        };

        // Generate issue-specific commands
        if let Some(generator) = self.command_generators.get(&issue.issue_type) {
            let generated_commands = generator.generate_commands(issue);
            commands.automatic_fixes.extend(generated_commands.automatic_fixes);
            commands.manual_fixes.extend(generated_commands.manual_fixes);
            commands.guidance_steps.extend(generated_commands.guidance_steps);
            commands.code_examples.extend(generated_commands.code_examples);
        }

        // Add severity-specific handling
        if let Some(handler) = self.severity_handlers.get(&issue.severity) {
            commands.bypass_options = handler.get_bypass_options(issue);
        }

        commands
    }

    /// Generates fix commands for multiple issues with prioritization.
    pub fn generate_batch_fix_commands(&self, issues: &[ValidationIssue]) -> BatchFixCommands {
        let mut batch = BatchFixCommands {
            total_issues: issues.len(),
            critical_fixes: Vec::new(),
            high_priority_fixes: Vec::new(),
            medium_priority_fixes: Vec::new(),
            low_priority_fixes: Vec::new(),
            info_fixes: Vec::new(),
            global_commands: Vec::new(),
            execution_order: Vec::new(),
        };

        // Categorize issues by severity
        for issue in issues {
            let fix_commands = self.generate_fix_commands(issue);
            
            match issue.severity {
                IssueSeverity::Critical => batch.critical_fixes.push(fix_commands),
                IssueSeverity::High => batch.high_priority_fixes.push(fix_commands),
                IssueSeverity::Medium => batch.medium_priority_fixes.push(fix_commands),
                IssueSeverity::Low => batch.low_priority_fixes.push(fix_commands),
                IssueSeverity::Info => batch.info_fixes.push(fix_commands),
            }
        }

        // Generate global commands that can fix multiple issues
        batch.global_commands = self.generate_global_commands(issues);
        
        // Determine optimal execution order
        batch.execution_order = self.determine_execution_order(issues);

        batch
    }

    /// Determines if an issue can be automatically fixed.
    pub fn can_auto_fix(&self, issue: &ValidationIssue) -> bool {
        self.command_generators.get(&issue.issue_type)
            .map(|generator| generator.supports_auto_fix())
            .unwrap_or(false)
    }

    /// Executes automatic fixes for issues that support it.
    pub fn execute_auto_fixes(&self, issues: &[ValidationIssue]) -> AutoFixResult {
        let mut result = AutoFixResult {
            total_attempted: 0,
            successful_fixes: 0,
            failed_fixes: 0,
            fixed_issues: Vec::new(),
            failed_issues: Vec::new(),
            commands_executed: Vec::new(),
        };

        for issue in issues {
            if self.can_auto_fix(issue) {
                result.total_attempted += 1;
                
                if let Some(generator) = self.command_generators.get(&issue.issue_type) {
                    match generator.execute_auto_fix(issue) {
                        Ok(commands) => {
                            result.successful_fixes += 1;
                            result.fixed_issues.push(issue.clone());
                            result.commands_executed.extend(commands);
                        }
                        Err(error) => {
                            result.failed_fixes += 1;
                            result.failed_issues.push((issue.clone(), error));
                        }
                    }
                }
            }
        }

        result
    }

    fn register_command_generators(&mut self) {
        // AI Pattern Command Generators
        self.command_generators.insert(
            IssueType::IncorrectPersonalityType,
            Box::new(PersonalityTypeCommandGenerator),
        );
        self.command_generators.insert(
            IssueType::MissingRangeDocumentation,
            Box::new(RangeDocumentationCommandGenerator),
        );
        self.command_generators.insert(
            IssueType::MissingBipolarDocumentation,
            Box::new(BipolarDocumentationCommandGenerator),
        );
        self.command_generators.insert(
            IssueType::MissingPersonalityModulation,
            Box::new(PersonalityModulationCommandGenerator),
        );
        self.command_generators.insert(
            IssueType::IncorrectTemporalUsage,
            Box::new(TemporalUsageCommandGenerator),
        );
        
        // ECS Architecture Command Generators
        self.command_generators.insert(
            IssueType::DirectComponentAccess,
            Box::new(ComponentAccessCommandGenerator),
        );
        
        // Behavioral Consistency Command Generators
        self.command_generators.insert(
            IssueType::MissingPersonalityConsistency,
            Box::new(PersonalityConsistencyCommandGenerator),
        );
        self.command_generators.insert(
            IssueType::HardcodedBehavioralValue,
            Box::new(HardcodedValueCommandGenerator),
        );
    }

    fn register_severity_handlers(&mut self) {
        self.severity_handlers.insert(
            IssueSeverity::Critical,
            SeverityHandler {
                blocking: true,
                bypass_allowed: true,
                auto_fix_preferred: true,
                escalation_required: true,
            },
        );
        self.severity_handlers.insert(
            IssueSeverity::High,
            SeverityHandler {
                blocking: true,
                bypass_allowed: true,
                auto_fix_preferred: true,
                escalation_required: false,
            },
        );
        self.severity_handlers.insert(
            IssueSeverity::Medium,
            SeverityHandler {
                blocking: false,
                bypass_allowed: true,
                auto_fix_preferred: false,
                escalation_required: false,
            },
        );
        self.severity_handlers.insert(
            IssueSeverity::Low,
            SeverityHandler {
                blocking: false,
                bypass_allowed: true,
                auto_fix_preferred: false,
                escalation_required: false,
            },
        );
        self.severity_handlers.insert(
            IssueSeverity::Info,
            SeverityHandler {
                blocking: false,
                bypass_allowed: true,
                auto_fix_preferred: false,
                escalation_required: false,
            },
        );
    }

    fn generate_global_commands(&self, issues: &[ValidationIssue]) -> Vec<GlobalFixCommand> {
        let mut global_commands = Vec::new();

        // Check if cargo fmt can fix multiple formatting issues
        let formatting_issues = issues.iter()
            .filter(|issue| matches!(issue.issue_type, IssueType::IncorrectPersonalityType))
            .count();
        
        if formatting_issues > 1 {
            global_commands.push(GlobalFixCommand {
                command: "cargo fmt".to_string(),
                description: "Fix all code formatting issues".to_string(),
                affected_issues: formatting_issues,
                estimated_time: "5 seconds".to_string(),
            });
        }

        // Check if clippy can fix multiple linting issues
        let linting_issues = issues.iter()
            .filter(|issue| matches!(
                issue.issue_type, 
                IssueType::DirectComponentAccess | IssueType::HardcodedBehavioralValue
            ))
            .count();
        
        if linting_issues > 1 {
            global_commands.push(GlobalFixCommand {
                command: "cargo clippy --fix --allow-dirty".to_string(),
                description: "Automatically fix clippy suggestions".to_string(),
                affected_issues: linting_issues,
                estimated_time: "30 seconds".to_string(),
            });
        }

        global_commands
    }

    fn determine_execution_order(&self, issues: &[ValidationIssue]) -> Vec<ExecutionStep> {
        let mut steps = Vec::new();

        // Step 1: Run global formatting fixes first
        steps.push(ExecutionStep {
            step_number: 1,
            description: "Run global formatting fixes".to_string(),
            commands: vec!["cargo fmt".to_string()],
            estimated_time: "5 seconds".to_string(),
            can_run_parallel: false,
        });

        // Step 2: Fix critical issues that block other fixes
        let critical_issues = issues.iter()
            .filter(|issue| issue.severity == IssueSeverity::Critical)
            .count();
        
        if critical_issues > 0 {
            steps.push(ExecutionStep {
                step_number: 2,
                description: format!("Fix {} critical issues", critical_issues),
                commands: vec!["Review and fix critical issues manually".to_string()],
                estimated_time: format!("{} minutes", critical_issues * 2),
                can_run_parallel: false,
            });
        }

        // Step 3: Run automated fixes for high priority issues
        let high_auto_fixable = issues.iter()
            .filter(|issue| issue.severity == IssueSeverity::High && self.can_auto_fix(issue))
            .count();
        
        if high_auto_fixable > 0 {
            steps.push(ExecutionStep {
                step_number: 3,
                description: format!("Auto-fix {} high priority issues", high_auto_fixable),
                commands: vec!["cargo clippy --fix --allow-dirty".to_string()],
                estimated_time: "30 seconds".to_string(),
                can_run_parallel: true,
            });
        }

        // Step 4: Address remaining issues
        let remaining_issues = issues.len() - critical_issues - high_auto_fixable;
        if remaining_issues > 0 {
            steps.push(ExecutionStep {
                step_number: 4,
                description: format!("Address {} remaining issues", remaining_issues),
                commands: vec!["Review manual fix suggestions".to_string()],
                estimated_time: format!("{} minutes", remaining_issues),
                can_run_parallel: true,
            });
        }

        steps
    }
}

impl Default for FixCommandGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages bypass mechanisms for emergency situations.
#[derive(Debug)]
pub struct BypassManager {
    pub emergency_bypasses: Vec<BypassOption>,
    pub temporary_bypasses: Vec<BypassOption>,
    pub permanent_bypasses: Vec<BypassOption>,
}

impl BypassManager {
    pub fn new() -> Self {
        Self {
            emergency_bypasses: vec![
                BypassOption {
                    command: "git commit -m \"EMERGENCY_COMMIT: <description>\"".to_string(),
                    description: "Emergency commit bypassing all checks".to_string(),
                    risk_level: RiskLevel::High,
                    time_limit: Some("24 hours".to_string()),
                    requires_approval: true,
                },
                BypassOption {
                    command: "git commit -m \"HOTFIX: <description>\"".to_string(),
                    description: "Hotfix commit for critical production issues".to_string(),
                    risk_level: RiskLevel::Medium,
                    time_limit: Some("4 hours".to_string()),
                    requires_approval: false,
                },
            ],
            temporary_bypasses: vec![
                BypassOption {
                    command: "git commit -m \"BYPASS_PRECOMMIT: <reason>\"".to_string(),
                    description: "Bypass pre-commit checks only".to_string(),
                    risk_level: RiskLevel::Low,
                    time_limit: None,
                    requires_approval: false,
                },
                BypassOption {
                    command: "git commit -m \"BYPASS_PREPUSH: <reason>\"".to_string(),
                    description: "Bypass pre-push checks only".to_string(),
                    risk_level: RiskLevel::Medium,
                    time_limit: None,
                    requires_approval: false,
                },
            ],
            permanent_bypasses: vec![
                BypassOption {
                    command: "# Add to .gitignore or use --allow in CI config".to_string(),
                    description: "Permanently exclude files from validation".to_string(),
                    risk_level: RiskLevel::High,
                    time_limit: None,
                    requires_approval: true,
                },
            ],
        }
    }

    pub fn get_bypass_options(&self, severity: &IssueSeverity) -> Vec<BypassOption> {
        match severity {
            IssueSeverity::Critical => self.emergency_bypasses.clone(),
            IssueSeverity::High => {
                let mut options = self.emergency_bypasses.clone();
                options.extend(self.temporary_bypasses.clone());
                options
            }
            _ => self.temporary_bypasses.clone(),
        }
    }
}

/// Configuration for handling different severity levels.
#[derive(Debug, Clone)]
pub struct SeverityHandler {
    pub blocking: bool,
    pub bypass_allowed: bool,
    pub auto_fix_preferred: bool,
    pub escalation_required: bool,
}

impl SeverityHandler {
    pub fn get_bypass_options(&self, issue: &ValidationIssue) -> Vec<BypassOption> {
        if !self.bypass_allowed {
            return Vec::new();
        }

        let bypass_manager = BypassManager::new();
        bypass_manager.get_bypass_options(&issue.severity)
    }
}

/// Set of fix commands for a specific validation issue.
#[derive(Debug, Clone)]
pub struct FixCommandSet {
    pub issue_type: IssueType,
    pub severity: IssueSeverity,
    pub automatic_fixes: Vec<AutomaticFix>,
    pub manual_fixes: Vec<ManualFix>,
    pub guidance_steps: Vec<GuidanceStep>,
    pub code_examples: Vec<CodeExample>,
    pub bypass_options: Vec<BypassOption>,
}

/// Batch of fix commands for multiple issues.
#[derive(Debug)]
pub struct BatchFixCommands {
    pub total_issues: usize,
    pub critical_fixes: Vec<FixCommandSet>,
    pub high_priority_fixes: Vec<FixCommandSet>,
    pub medium_priority_fixes: Vec<FixCommandSet>,
    pub low_priority_fixes: Vec<FixCommandSet>,
    pub info_fixes: Vec<FixCommandSet>,
    pub global_commands: Vec<GlobalFixCommand>,
    pub execution_order: Vec<ExecutionStep>,
}

/// Result of executing automatic fixes.
#[derive(Debug)]
pub struct AutoFixResult {
    pub total_attempted: usize,
    pub successful_fixes: usize,
    pub failed_fixes: usize,
    pub fixed_issues: Vec<ValidationIssue>,
    pub failed_issues: Vec<(ValidationIssue, String)>,
    pub commands_executed: Vec<String>,
}

/// Automatic fix that can be executed without user intervention.
#[derive(Debug, Clone)]
pub struct AutomaticFix {
    pub command: String,
    pub description: String,
    pub estimated_time: String,
    pub risk_level: RiskLevel,
    pub requires_backup: bool,
}

/// Manual fix that requires user intervention.
#[derive(Debug, Clone)]
pub struct ManualFix {
    pub description: String,
    pub steps: Vec<String>,
    pub estimated_time: String,
    pub difficulty: DifficultyLevel,
    pub references: Vec<String>,
}

/// Step-by-step guidance for fixing an issue.
#[derive(Debug, Clone)]
pub struct GuidanceStep {
    pub step_number: usize,
    pub description: String,
    pub command: Option<String>,
    pub expected_outcome: String,
    pub troubleshooting: Vec<String>,
}

/// Code example showing correct implementation.
#[derive(Debug, Clone)]
pub struct CodeExample {
    pub title: String,
    pub incorrect_code: Option<String>,
    pub correct_code: String,
    pub explanation: String,
    pub file_path: Option<PathBuf>,
}

/// Bypass option for emergency situations.
#[derive(Debug, Clone)]
pub struct BypassOption {
    pub command: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub time_limit: Option<String>,
    pub requires_approval: bool,
}

/// Global command that can fix multiple issues.
#[derive(Debug, Clone)]
pub struct GlobalFixCommand {
    pub command: String,
    pub description: String,
    pub affected_issues: usize,
    pub estimated_time: String,
}

/// Step in the execution order for fixing issues.
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub step_number: usize,
    pub description: String,
    pub commands: Vec<String>,
    pub estimated_time: String,
    pub can_run_parallel: bool,
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Trait for generating fix commands for specific issue types.
pub trait CommandGenerator: std::fmt::Debug + Send + Sync {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet;
    fn supports_auto_fix(&self) -> bool;
    fn execute_auto_fix(&self, issue: &ValidationIssue) -> Result<Vec<String>, String>;
}

// Command Generator Implementations

#[derive(Debug)]
pub struct PersonalityTypeCommandGenerator;

impl CommandGenerator for PersonalityTypeCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: vec![
                AutomaticFix {
                    command: "sed -i 's/pub \\([a-z_]*\\): f32,/pub \\1: Normalized<f32>,/g'".to_string(),
                    description: "Replace f32 with Normalized<f32> for personality traits".to_string(),
                    estimated_time: "1 second".to_string(),
                    risk_level: RiskLevel::Low,
                    requires_backup: true,
                },
            ],
            manual_fixes: vec![
                ManualFix {
                    description: "Replace f32 with Normalized<f32> for personality traits".to_string(),
                    steps: vec![
                        "1. Open the file in your editor".to_string(),
                        "2. Find personality trait fields (openness, conscientiousness, etc.)".to_string(),
                        "3. Change 'pub trait_name: f32,' to 'pub trait_name: Normalized<f32>,'".to_string(),
                        "4. Add 'use crate::core::types::Normalized;' to imports".to_string(),
                        "5. Update any initialization code to use .into() or Normalized::new()".to_string(),
                    ],
                    estimated_time: "2-3 minutes".to_string(),
                    difficulty: DifficultyLevel::Beginner,
                    references: vec![
                        "See src/core/types.rs for Normalized<f32> usage".to_string(),
                        "Big Five personality model documentation".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Identify personality trait fields".to_string(),
                    command: Some("grep -n 'pub.*: f32' src/ai/cognition/personality.rs".to_string()),
                    expected_outcome: "List of personality trait fields using f32".to_string(),
                    troubleshooting: vec![
                        "If no matches found, check if traits are already using Normalized<f32>".to_string(),
                    ],
                },
                GuidanceStep {
                    step_number: 2,
                    description: "Replace f32 with Normalized<f32>".to_string(),
                    command: Some("Replace each 'f32' with 'Normalized<f32>' for personality traits".to_string()),
                    expected_outcome: "All personality traits use Normalized<f32> type".to_string(),
                    troubleshooting: vec![
                        "Ensure you only change personality traits, not other f32 fields".to_string(),
                        "Add import for Normalized type if missing".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Personality Trait Type Correction".to_string(),
                    incorrect_code: Some("pub openness: f32,\npub conscientiousness: f32,".to_string()),
                    correct_code: "pub openness: Normalized<f32>,\npub conscientiousness: Normalized<f32>,".to_string(),
                    explanation: "Personality traits should use Normalized<f32> to ensure 0.0-1.0 range".to_string(),
                    file_path: Some(PathBuf::from("src/ai/cognition/personality.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }

    fn execute_auto_fix(&self, issue: &ValidationIssue) -> Result<Vec<String>, String> {
        // This would execute the actual fix command
        // For now, return the commands that would be executed
        Ok(vec![
            format!("sed -i 's/pub \\([a-z_]*\\): f32,/pub \\1: Normalized<f32>,/g' {}", 
                    issue.file_path.display()),
        ])
    }
}

#[derive(Debug)]
pub struct RangeDocumentationCommandGenerator;

impl CommandGenerator for RangeDocumentationCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(), // Documentation requires manual intervention
            manual_fixes: vec![
                ManualFix {
                    description: "Add range documentation for personality traits".to_string(),
                    steps: vec![
                        "1. Add doc comment above each personality trait field".to_string(),
                        "2. Use format: /// Trait name: 0.0 (low meaning) to 1.0 (high meaning)".to_string(),
                        "3. Include behavioral implications of low and high values".to_string(),
                        "4. Reference Big Five model for standard interpretations".to_string(),
                    ],
                    estimated_time: "1-2 minutes per trait".to_string(),
                    difficulty: DifficultyLevel::Beginner,
                    references: vec![
                        "Big Five personality model documentation".to_string(),
                        "Project personality trait conventions".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Identify undocumented personality traits".to_string(),
                    command: Some("grep -B1 -A1 'pub.*Normalized<f32>' src/ai/cognition/personality.rs".to_string()),
                    expected_outcome: "List of personality traits and their current documentation".to_string(),
                    troubleshooting: vec![
                        "Look for traits without /// comments above them".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Personality Trait Documentation".to_string(),
                    incorrect_code: Some("pub openness: Normalized<f32>,".to_string()),
                    correct_code: "/// Openness to experience: 0.0 (closed-minded, traditional) to 1.0 (very open, creative)\npub openness: Normalized<f32>,".to_string(),
                    explanation: "Document the range and behavioral meaning of personality traits".to_string(),
                    file_path: Some(PathBuf::from("src/ai/cognition/personality.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false // Documentation requires human judgment
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Documentation requires manual intervention".to_string())
    }
}

#[derive(Debug)]
pub struct BipolarDocumentationCommandGenerator;

impl CommandGenerator for BipolarDocumentationCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(),
            manual_fixes: vec![
                ManualFix {
                    description: "Add bipolar range documentation for emotional dimensions".to_string(),
                    steps: vec![
                        "1. Add doc comment above each emotional dimension field".to_string(),
                        "2. Use format: /// Dimension: -1.0 (negative pole) to 1.0 (positive pole)".to_string(),
                        "3. Explain the meaning of negative, neutral (0.0), and positive values".to_string(),
                        "4. Add validation method to ensure values stay in range".to_string(),
                    ],
                    estimated_time: "2-3 minutes per dimension".to_string(),
                    difficulty: DifficultyLevel::Beginner,
                    references: vec![
                        "Emotional dimension theory documentation".to_string(),
                        "Valence-Arousal-Dominance model".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Identify emotional dimensions".to_string(),
                    command: Some("grep -n 'valence\\|arousal\\|dominance' src/ai/".to_string()),
                    expected_outcome: "List of emotional dimension fields".to_string(),
                    troubleshooting: vec![
                        "Look for f32 fields that represent emotional states".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Emotional Dimension Documentation".to_string(),
                    incorrect_code: Some("pub valence: f32,".to_string()),
                    correct_code: "/// Emotional valence: -1.0 (very negative) to 1.0 (very positive)\npub valence: f32,".to_string(),
                    explanation: "Document bipolar emotional dimensions with clear range meanings".to_string(),
                    file_path: Some(PathBuf::from("src/ai/physiology/emotional_state.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Emotional documentation requires manual intervention".to_string())
    }
}

#[derive(Debug)]
pub struct PersonalityModulationCommandGenerator;

impl CommandGenerator for PersonalityModulationCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(),
            manual_fixes: vec![
                ManualFix {
                    description: "Add personality modulation to AI systems".to_string(),
                    steps: vec![
                        "1. Add Personality component to system query".to_string(),
                        "2. Identify base behavior values that should be modulated".to_string(),
                        "3. Multiply base values by relevant personality trait values".to_string(),
                        "4. Use personality.trait_name.value() to get f32 from Normalized<f32>".to_string(),
                        "5. Document which personality traits affect this behavior".to_string(),
                    ],
                    estimated_time: "5-10 minutes per system".to_string(),
                    difficulty: DifficultyLevel::Intermediate,
                    references: vec![
                        "Personality-driven behavior documentation".to_string(),
                        "Big Five trait behavioral implications".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Identify systems that need personality modulation".to_string(),
                    command: Some("grep -r 'fn.*_system' src/ai/ | grep -v personality".to_string()),
                    expected_outcome: "List of AI systems that might need personality modulation".to_string(),
                    troubleshooting: vec![
                        "Look for systems that make behavioral decisions".to_string(),
                        "Skip utility systems that don't affect agent behavior".to_string(),
                    ],
                },
                GuidanceStep {
                    step_number: 2,
                    description: "Add personality parameter to system".to_string(),
                    command: Some("Add &Personality to Query parameters".to_string()),
                    expected_outcome: "System can access personality traits".to_string(),
                    troubleshooting: vec![
                        "Ensure Personality component is imported".to_string(),
                        "Check that entities have Personality component".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Personality Modulation in Systems".to_string(),
                    incorrect_code: Some("fn social_interaction_system(\n    mut agents: Query<&mut SocialBehavior>,\n) {\n    for mut behavior in agents.iter_mut() {\n        behavior.approach_tendency = 0.5; // Fixed value\n    }\n}".to_string()),
                    correct_code: "fn social_interaction_system(\n    mut agents: Query<(&mut SocialBehavior, &Personality)>,\n) {\n    for (mut behavior, personality) in agents.iter_mut() {\n        let extraversion_factor = personality.extraversion.value();\n        behavior.approach_tendency = 0.3 + (extraversion_factor * 0.4); // Modulated by personality\n    }\n}".to_string(),
                    explanation: "Modulate behavior based on personality traits for consistent character differences".to_string(),
                    file_path: Some(PathBuf::from("src/ai/social/interaction_systems.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false // Requires understanding of behavioral logic
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Personality modulation requires manual behavioral analysis".to_string())
    }
}

#[derive(Debug)]
pub struct TemporalUsageCommandGenerator;

impl CommandGenerator for TemporalUsageCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: vec![
                AutomaticFix {
                    command: "sed -i 's/std::time::Instant::now()/world_time.current_time/g'".to_string(),
                    description: "Replace real-world time with WorldTime".to_string(),
                    estimated_time: "1 second".to_string(),
                    risk_level: RiskLevel::Medium,
                    requires_backup: true,
                },
            ],
            manual_fixes: vec![
                ManualFix {
                    description: "Replace real-world time usage with WorldTime resource".to_string(),
                    steps: vec![
                        "1. Add world_time: Res<WorldTime> to system parameters".to_string(),
                        "2. Replace std::time::Instant::now() with world_time.current_time".to_string(),
                        "3. Replace duration calculations with world_time.delta_time".to_string(),
                        "4. Ensure WorldTime resource is available in the system".to_string(),
                        "5. Test that time scaling works correctly".to_string(),
                    ],
                    estimated_time: "3-5 minutes per system".to_string(),
                    difficulty: DifficultyLevel::Intermediate,
                    references: vec![
                        "WorldTime resource documentation".to_string(),
                        "Temporal consistency guidelines".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Find real-world time usage".to_string(),
                    command: Some("grep -r 'std::time::Instant\\|SystemTime' src/ai/".to_string()),
                    expected_outcome: "List of files using real-world time".to_string(),
                    troubleshooting: vec![
                        "Also check for Duration::from_secs and similar patterns".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "WorldTime Usage".to_string(),
                    incorrect_code: Some("let now = std::time::Instant::now();\nlet elapsed = now.duration_since(last_update);".to_string()),
                    correct_code: "fn system(world_time: Res<WorldTime>) {\n    let current_time = world_time.current_time;\n    let elapsed = world_time.delta_time;\n}".to_string(),
                    explanation: "Use WorldTime resource for simulation consistency and time scaling support".to_string(),
                    file_path: Some(PathBuf::from("src/ai/systems.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        true // Simple text replacement
    }

    fn execute_auto_fix(&self, issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Ok(vec![
            format!("sed -i 's/std::time::Instant::now()/world_time.current_time/g' {}", 
                    issue.file_path.display()),
            format!("sed -i 's/std::time::SystemTime::now()/world_time.current_time/g' {}", 
                    issue.file_path.display()),
        ])
    }
}

#[derive(Debug)]
pub struct ComponentAccessCommandGenerator;

impl CommandGenerator for ComponentAccessCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(),
            manual_fixes: vec![
                ManualFix {
                    description: "Replace direct component access with proper Query usage".to_string(),
                    steps: vec![
                        "1. Identify the component being accessed directly".to_string(),
                        "2. Add Query<&ComponentType> to system parameters".to_string(),
                        "3. Replace world.get::<Component>(entity) with query.get(entity)".to_string(),
                        "4. Use query.iter() for iterating over all components".to_string(),
                        "5. Handle Result from query.get() properly".to_string(),
                    ],
                    estimated_time: "3-5 minutes per occurrence".to_string(),
                    difficulty: DifficultyLevel::Intermediate,
                    references: vec![
                        "Bevy ECS Query documentation".to_string(),
                        "ECS architecture guidelines".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Find direct component access patterns".to_string(),
                    command: Some("grep -r 'world\\.get::<\\|world\\.entity(' src/ai/".to_string()),
                    expected_outcome: "List of direct component access violations".to_string(),
                    troubleshooting: vec![
                        "Also check for .component::<> patterns".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Proper ECS Query Usage".to_string(),
                    incorrect_code: Some("fn system(world: &mut World) {\n    let component = world.get::<MyComponent>(entity).unwrap();\n}".to_string()),
                    correct_code: "fn system(query: Query<&MyComponent>) {\n    if let Ok(component) = query.get(entity) {\n        // Use component\n    }\n}".to_string(),
                    explanation: "Use Query parameters instead of direct world access for ECS compliance".to_string(),
                    file_path: Some(PathBuf::from("src/ai/systems.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false // Requires understanding of system architecture
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Component access patterns require manual refactoring".to_string())
    }
}

#[derive(Debug)]
pub struct PersonalityConsistencyCommandGenerator;

impl CommandGenerator for PersonalityConsistencyCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(),
            manual_fixes: vec![
                ManualFix {
                    description: "Ensure personality traits consistently affect behavior".to_string(),
                    steps: vec![
                        "1. Review all behavioral systems for personality modulation".to_string(),
                        "2. Add personality parameters where missing".to_string(),
                        "3. Ensure same personality traits affect related behaviors consistently".to_string(),
                        "4. Test that personality differences produce observable changes".to_string(),
                        "5. Document personality-behavior relationships".to_string(),
                    ],
                    estimated_time: "10-20 minutes per behavioral area".to_string(),
                    difficulty: DifficultyLevel::Advanced,
                    references: vec![
                        "Personality consistency guidelines".to_string(),
                        "Behavioral validation tests".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Audit personality usage across systems".to_string(),
                    command: Some("grep -r 'personality\\.' src/ai/ | wc -l".to_string()),
                    expected_outcome: "Count of personality trait usage in AI systems".to_string(),
                    troubleshooting: vec![
                        "Low count may indicate missing personality modulation".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Consistent Personality Effects".to_string(),
                    incorrect_code: Some("// System A uses extraversion\nbehavior.social_approach = base * personality.extraversion.value();\n\n// System B ignores extraversion\nbehavior.group_joining = 0.5; // Fixed value".to_string()),
                    correct_code: "// Both systems use extraversion consistently\nbehavior.social_approach = base * personality.extraversion.value();\nbehavior.group_joining = base_joining * personality.extraversion.value();".to_string(),
                    explanation: "Ensure related behaviors are consistently affected by the same personality traits".to_string(),
                    file_path: Some(PathBuf::from("src/ai/social/")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false // Requires behavioral analysis
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Personality consistency requires manual behavioral review".to_string())
    }
}

#[derive(Debug)]
pub struct HardcodedValueCommandGenerator;

impl CommandGenerator for HardcodedValueCommandGenerator {
    fn generate_commands(&self, issue: &ValidationIssue) -> FixCommandSet {
        FixCommandSet {
            issue_type: issue.issue_type.clone(),
            severity: issue.severity.clone(),
            automatic_fixes: Vec::new(), // Requires understanding of value meaning
            manual_fixes: vec![
                ManualFix {
                    description: "Replace magic numbers with named constants".to_string(),
                    steps: vec![
                        "1. Identify the hardcoded value and its purpose".to_string(),
                        "2. Create a descriptive constant name".to_string(),
                        "3. Add the constant to appropriate constants module".to_string(),
                        "4. Replace the hardcoded value with the constant".to_string(),
                        "5. Document why this specific value was chosen".to_string(),
                    ],
                    estimated_time: "2-3 minutes per value".to_string(),
                    difficulty: DifficultyLevel::Beginner,
                    references: vec![
                        "Constants organization guidelines".to_string(),
                        "Naming conventions documentation".to_string(),
                    ],
                },
            ],
            guidance_steps: vec![
                GuidanceStep {
                    step_number: 1,
                    description: "Find hardcoded values".to_string(),
                    command: Some("grep -r '\\b0\\.[0-9]\\+\\b\\|\\b[1-9][0-9]*\\.[0-9]\\+\\b' src/ai/".to_string()),
                    expected_outcome: "List of potential magic numbers in AI code".to_string(),
                    troubleshooting: vec![
                        "Filter out obvious values like 0.0, 1.0 that don't need constants".to_string(),
                    ],
                },
            ],
            code_examples: vec![
                CodeExample {
                    title: "Named Constants".to_string(),
                    incorrect_code: Some("if stress_level > 0.8 {\n    trigger_stress_response();\n}".to_string()),
                    correct_code: "const STRESS_CRITICAL_THRESHOLD: f32 = 0.8;\n\nif stress_level > STRESS_CRITICAL_THRESHOLD {\n    trigger_stress_response();\n}".to_string(),
                    explanation: "Use named constants to make code more readable and maintainable".to_string(),
                    file_path: Some(PathBuf::from("src/ai/constants.rs")),
                },
            ],
            bypass_options: Vec::new(),
        }
    }

    fn supports_auto_fix(&self) -> bool {
        false // Requires understanding of value semantics
    }

    fn execute_auto_fix(&self, _issue: &ValidationIssue) -> Result<Vec<String>, String> {
        Err("Hardcoded values require manual analysis for appropriate constant names".to_string())
    }
}