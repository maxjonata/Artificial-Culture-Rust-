use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Validates AI-specific coding patterns and conventions.
#[derive(Debug)]
pub struct AiPatternValidator {
    pub personality_trait_checker: PersonalityTraitChecker,
    pub emotional_state_checker: EmotionalStateChecker,
    pub system_modulation_checker: SystemModulationChecker,
    pub temporal_consistency_checker: TemporalConsistencyChecker,
    pub architecture_checker: ArchitectureChecker,
}

impl AiPatternValidator {
    pub fn new() -> Self {
        Self {
            personality_trait_checker: PersonalityTraitChecker::new(),
            emotional_state_checker: EmotionalStateChecker::new(),
            system_modulation_checker: SystemModulationChecker::new(),
            temporal_consistency_checker: TemporalConsistencyChecker::new(),
            architecture_checker: ArchitectureChecker::new(),
        }
    }

    pub fn validate_file(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Run all validation checks
        let personality_result = self.personality_trait_checker.validate_personality_component(file_content, file_path);
        all_issues.extend(personality_result.issues);
        all_suggestions.extend(personality_result.suggestions);

        let emotional_result = self.emotional_state_checker.validate_emotional_component(file_content, file_path);
        all_issues.extend(emotional_result.issues);
        all_suggestions.extend(emotional_result.suggestions);

        let modulation_result = self.system_modulation_checker.validate_system_modulation(file_content, file_path);
        all_issues.extend(modulation_result.issues);
        all_suggestions.extend(modulation_result.suggestions);

        let temporal_result = self.temporal_consistency_checker.validate_temporal_consistency(file_content, file_path);
        all_issues.extend(temporal_result.issues);
        all_suggestions.extend(temporal_result.suggestions);

        let architecture_result = self.architecture_checker.validate_ai_architecture(file_content, file_path);
        all_issues.extend(architecture_result.issues);
        all_suggestions.extend(architecture_result.suggestions);

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }
}

impl Default for AiPatternValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates personality trait patterns and conventions.
#[derive(Debug)]
pub struct PersonalityTraitChecker {
    pub valid_trait_types: HashSet<String>,
    pub required_range_documentation: bool,
    pub normalized_type_enforcement: bool,
}

impl ValidationIssue {
    pub fn new(
        issue_type: IssueType,
        message: String,
        file_path: PathBuf,
        line_number: usize,
        severity: IssueSeverity,
        suggestion: Option<String>,
    ) -> Self {
        Self {
            issue_type,
            message,
            file_path,
            line_number,
            column_number: None,
            severity,
            suggestion,
            code_example: None,
        }
    }
}

impl PersonalityTraitChecker {
    pub fn new() -> Self {
        let mut valid_trait_types = HashSet::new();
        valid_trait_types.insert("openness".to_string());
        valid_trait_types.insert("conscientiousness".to_string());
        valid_trait_types.insert("extraversion".to_string());
        valid_trait_types.insert("agreeableness".to_string());
        valid_trait_types.insert("neuroticism".to_string());
        valid_trait_types.insert("leadership_tendency".to_string());
        valid_trait_types.insert("cooperation_drive".to_string());
        valid_trait_types.insert("exploration_urge".to_string());
        valid_trait_types.insert("protection_instinct".to_string());

        Self {
            valid_trait_types,
            required_range_documentation: true,
            normalized_type_enforcement: true,
        }
    }

    pub fn validate_personality_component(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Check for personality trait definitions
        let trait_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?(\w+):\s*([^,\n}]+)").unwrap();
        
        for captures in trait_regex.captures_iter(file_content) {
            let field_name = &captures[1];
            let field_type = captures[2].trim();
            
            // Check if it's a personality trait
            if self.is_personality_trait(field_name) {
                // Verify it uses Normalized<f32> or just Normalized
                if !field_type.contains("Normalized") {
                    issues.push(ValidationIssue::new(
                        IssueType::IncorrectPersonalityType,
                        format!(
                            "Personality trait '{}' should use Normalized<f32> or Normalized, found: {}",
                            field_name, field_type
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::High,
                        Some(format!(
                            "Change '{}' to 'Normalized' and add documentation: /// {}: 0.0 (low) to 1.0 (high)",
                            field_type, field_name
                        )),
                    ));
                }
                
                // Check for range documentation
                if self.required_range_documentation && !self.has_range_documentation(file_content, field_name) {
                    issues.push(ValidationIssue::new(
                        IssueType::MissingRangeDocumentation,
                        format!(
                            "Personality trait '{}' missing range documentation",
                            field_name
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::Medium,
                        Some(format!(
                            "Add documentation: /// {}: 0.0 (low) to 1.0 (high)",
                            field_name
                        )),
                    ));
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }
    
    fn is_personality_trait(&self, field_name: &str) -> bool {
        self.valid_trait_types.contains(&field_name.to_lowercase()) ||
        field_name.to_lowercase().contains("personality") ||
        ["openness", "conscientiousness", "extraversion", "agreeableness", "neuroticism"]
            .contains(&field_name.to_lowercase().as_str())
    }

    fn has_range_documentation(&self, file_content: &str, field_name: &str) -> bool {
        // Look for documentation comment above the field
        let doc_pattern = format!(r"///.*{}.*(?:0\.0|1\.0)", regex::escape(field_name));
        let doc_regex = Regex::new(&doc_pattern).unwrap();
        doc_regex.is_match(file_content)
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::IncorrectPersonalityType => {
                    suggestions.push("Use Normalized type for all personality traits to ensure 0.0-1.0 range".to_string());
                }
                IssueType::MissingRangeDocumentation => {
                    suggestions.push("Add documentation comments explaining the 0.0-1.0 range for personality traits".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for PersonalityTraitChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates emotional state patterns and conventions.
#[derive(Debug)]
pub struct EmotionalStateChecker {
    pub valid_emotional_dimensions: HashSet<String>,
    pub required_bipolar_range: (f32, f32), // -1.0 to 1.0
}

impl EmotionalStateChecker {
    pub fn new() -> Self {
        let mut valid_emotional_dimensions = HashSet::new();
        valid_emotional_dimensions.insert("valence".to_string());
        valid_emotional_dimensions.insert("arousal".to_string());
        valid_emotional_dimensions.insert("dominance".to_string());
        valid_emotional_dimensions.insert("pleasure".to_string());
        valid_emotional_dimensions.insert("activation".to_string());
        valid_emotional_dimensions.insert("control".to_string());

        Self {
            valid_emotional_dimensions,
            required_bipolar_range: (-1.0, 1.0),
        }
    }

    pub fn validate_emotional_component(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Check for emotional state definitions
        let emotion_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?(\w+):\s*f32").unwrap();
        
        for captures in emotion_regex.captures_iter(file_content) {
            let field_name = &captures[1];
            
            if self.is_emotional_dimension(field_name) {
                // Check for proper range documentation
                if !self.has_bipolar_documentation(file_content, field_name) {
                    issues.push(ValidationIssue::new(
                        IssueType::MissingBipolarDocumentation,
                        format!(
                            "Emotional dimension '{}' missing bipolar range documentation",
                            field_name
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::Medium,
                        Some(format!(
                            "Add documentation: /// {}: -1.0 (negative) to 1.0 (positive)",
                            field_name
                        )),
                    ));
                }
                
                // Check for validation methods
                if !self.has_validation_method(file_content, field_name) {
                    issues.push(ValidationIssue::new(
                        IssueType::MissingValidationMethod,
                        format!(
                            "Emotional dimension '{}' should have validation method",
                            field_name
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::Low,
                        Some(format!(
                            "Add validation: pub fn validate_{}(&self) -> Result<(), String> {{ if !(-1.0..=1.0).contains(&self.{}) {{ Err(\"Invalid {} range\".to_string()) }} else {{ Ok(()) }} }}",
                            field_name, field_name, field_name
                        )),
                    ));
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }
    
    fn is_emotional_dimension(&self, field_name: &str) -> bool {
        self.valid_emotional_dimensions.contains(&field_name.to_lowercase()) ||
        ["valence", "arousal", "dominance", "pleasure", "activation", "control"]
            .contains(&field_name.to_lowercase().as_str())
    }

    fn has_bipolar_documentation(&self, file_content: &str, field_name: &str) -> bool {
        // Look for documentation comment mentioning -1.0 to 1.0 range
        let doc_pattern = format!(r"///.*{}.*(?:-1\.0.*1\.0|bipolar)", regex::escape(field_name));
        let doc_regex = Regex::new(&doc_pattern).unwrap();
        doc_regex.is_match(file_content)
    }

    fn has_validation_method(&self, file_content: &str, field_name: &str) -> bool {
        // Look for validation method for this field
        let validation_pattern = format!(r"fn validate_{}|fn validate.*{}.*-1\.0.*1\.0", regex::escape(field_name), regex::escape(field_name));
        let validation_regex = Regex::new(&validation_pattern).unwrap();
        validation_regex.is_match(file_content)
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::MissingBipolarDocumentation => {
                    suggestions.push("Add documentation comments explaining the -1.0 to 1.0 range for emotional dimensions".to_string());
                }
                IssueType::MissingValidationMethod => {
                    suggestions.push("Add validation methods to ensure emotional values stay within -1.0 to 1.0 range".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for EmotionalStateChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates system modulation patterns for personality-driven behavior.
#[derive(Debug)]
pub struct SystemModulationChecker {
    pub required_personality_modulation: bool,
}

impl SystemModulationChecker {
    pub fn new() -> Self {
        Self {
            required_personality_modulation: true,
        }
    }

    pub fn validate_system_modulation(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Find system functions
        let system_regex = Regex::new(r"fn (\w+_system)\s*\(").unwrap();
        
        for captures in system_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check if system uses personality modulation
                if !self.has_personality_modulation(&system_body) {
                    issues.push(ValidationIssue::new(
                        IssueType::MissingPersonalityModulation,
                        format!(
                            "System '{}' should be modulated by personality traits",
                            system_name
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::High,
                        Some(
                            "Add personality parameter to system and use personality traits to modulate behavior".to_string()
                        ),
                    ));
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }

    fn find_system_body(&self, content: &str, system_name: &str) -> Option<String> {
        let system_start_pattern = format!(r"fn {}\s*\([^{{]*\{{", regex::escape(system_name));
        let system_start_regex = Regex::new(&system_start_pattern).unwrap();
        
        if let Some(start_match) = system_start_regex.find(content) {
            let start_pos = start_match.end() - 1; // Position of opening brace
            let mut brace_count = 1;
            let current_pos = start_pos + 1;
            
            for (i, ch) in content[current_pos..].char_indices() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some(content[start_pos..current_pos + i + 1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        None
    }

    fn has_personality_modulation(&self, system_body: &str) -> bool {
        // Look for personality usage patterns
        let personality_patterns = [
            r"personality\.",
            r"Personality",
            r"\.openness",
            r"\.conscientiousness",
            r"\.extraversion",
            r"\.agreeableness",
            r"\.neuroticism",
        ];
        
        personality_patterns.iter().any(|pattern| {
            Regex::new(pattern).unwrap().is_match(system_body)
        })
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::MissingPersonalityModulation => {
                    suggestions.push("All AI systems should be modulated by personality traits to create consistent character behavior".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for SystemModulationChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates temporal consistency patterns for WorldTime usage.
#[derive(Debug)]
pub struct TemporalConsistencyChecker {
    pub require_world_time: bool,
}

impl TemporalConsistencyChecker {
    pub fn new() -> Self {
        Self {
            require_world_time: true,
        }
    }

    pub fn validate_temporal_consistency(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Check for real-world time usage patterns that should use WorldTime instead
        let real_time_patterns = [
            r"std::time::Instant",
            r"std::time::SystemTime",
            r"chrono::",
            r"\.elapsed\(\)",
            r"\.now\(\)",
        ];
        
        for pattern_str in &real_time_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            
            for mat in pattern.find_iter(file_content) {
                issues.push(ValidationIssue::new(
                    IssueType::IncorrectTemporalUsage,
                    format!(
                        "Real-world time usage detected: '{}'. Use WorldTime resource instead.",
                        mat.as_str()
                    ),
                    file_path.to_path_buf(),
                    self.find_line_number(file_content, mat.as_str()),
                    IssueSeverity::Critical,
                    Some(
                        "Replace real-world time with WorldTime resource for simulation consistency".to_string()
                    ),
                ));
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::IncorrectTemporalUsage => {
                    suggestions.push("Use WorldTime resource for all temporal calculations to support time scaling and simulation consistency".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for TemporalConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates AI system architecture patterns.
#[derive(Debug)]
pub struct ArchitectureChecker {
    pub require_event_communication: bool,
    pub require_behavioral_documentation: bool,
}

impl ArchitectureChecker {
    pub fn new() -> Self {
        Self {
            require_event_communication: true,
            require_behavioral_documentation: true,
        }
    }

    pub fn validate_ai_architecture(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Check for direct component access violations
        let direct_access_patterns = [
            r"\.get_component::<",
            r"\.component::<",
            r"world\.get::<",
        ];
        
        for pattern_str in &direct_access_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            
            for mat in pattern.find_iter(file_content) {
                issues.push(ValidationIssue::new(
                    IssueType::DirectComponentAccess,
                    format!(
                        "Direct component access detected: '{}'. Use event-driven communication instead.",
                        mat.as_str()
                    ),
                    file_path.to_path_buf(),
                    self.find_line_number(file_content, mat.as_str()),
                    IssueSeverity::High,
                    Some(
                        "Replace direct component access with event-driven communication between AI systems".to_string()
                    ),
                ));
            }
        }

        // Check for behavioral purpose documentation in AI systems
        if self.require_behavioral_documentation {
            let system_regex = Regex::new(r"fn (\w+_system)\s*\(").unwrap();
            
            for captures in system_regex.captures_iter(file_content) {
                let system_name = &captures[1];
                
                if !self.has_behavioral_documentation(file_content, system_name) {
                    issues.push(ValidationIssue::new(
                        IssueType::MissingBehavioralDocumentation,
                        format!(
                            "AI system '{}' missing behavioral purpose documentation",
                            system_name
                        ),
                        file_path.to_path_buf(),
                        self.find_line_number(file_content, &captures[0]),
                        IssueSeverity::Medium,
                        Some(
                            "Add documentation explaining the behavioral purpose and AI impact of this system".to_string()
                        ),
                    ));
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: Duration::from_secs(0),
            check_type: CheckType::Manual,
        }
    }

    fn has_behavioral_documentation(&self, file_content: &str, system_name: &str) -> bool {
        // Look for documentation comment above the system function
        let doc_pattern = format!(r"///.*(?:behavior|AI|agent|personality|emotion|social).*\n.*fn {}", regex::escape(system_name));
        let doc_regex = Regex::new(&doc_pattern).unwrap();
        doc_regex.is_match(file_content)
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::DirectComponentAccess => {
                    suggestions.push("Use event-driven architecture for communication between AI systems".to_string());
                }
                IssueType::MissingBehavioralDocumentation => {
                    suggestions.push("Document the behavioral purpose of AI systems to explain their impact on agent behavior".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for ArchitectureChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a validation check.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub file_path: PathBuf,
    pub passed: bool,
    pub issues: Vec<ValidationIssue>,
    pub suggestions: Vec<String>,
    pub execution_time: Duration,
    pub check_type: CheckType,
}

#[derive(Debug, Clone)]
pub enum CheckType {
    PreCommit,
    PrePush,
    ContinuousIntegration,
    Manual,
    PerformanceRegression,
}

/// Individual validation issue found in code.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub issue_type: IssueType,
    pub message: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column_number: Option<usize>,
    pub severity: IssueSeverity,
    pub suggestion: Option<String>,
    pub code_example: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IssueSeverity {
    Critical,  // Blocks commit/push
    High,      // Should be fixed soon
    Medium,    // Should be addressed
    Low,       // Nice to fix
    Info,      // Informational only
}

/// Types of validation issues that can be detected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueType {
    // AI Pattern Issues
    IncorrectPersonalityType,
    MissingRangeDocumentation,
    MissingBipolarDocumentation,
    MissingPersonalityModulation,
    IncorrectTemporalUsage,
    MissingValidationMethod,
    DirectComponentAccess,
    MissingBehavioralDocumentation,
    
    // Behavioral Consistency Issues
    MissingPersonalityConsistency,
    HardcodedBehavioralValue,
    MissingMisunderstandingMechanics,
    PerfectCommunication,
    UnrealisticContagionSpeed,
    MissingInformationLoss,
    MissingConfirmationBias,
    MissingSubjectivePerception,
    OptimalDecisionMaking,
    MissingEmotionalDecisionInfluence,
    MissingEmotionalLogic,
    MissingStressMoodImpact,
    MissingBehavioralFeedback,
    MissingDecisionPersonalityConsistency,
    
    // Performance Regression Issues
    PerformanceRegressionSetupFailed,
    BaselineEstablishmentFailed,
    PerformanceTestFailed,
    FpsRegression,
    MemoryRegression,
    CpuRegression,
    FpsTargetNotMet,
    
    // Documentation and Code Quality Issues
    MissingDocumentation,
    LowQualityDocumentation,
    ExcessiveComplexity,
    ExcessiveFunctionLength,
    CodeDuplication,
    MissingTestCoverage,
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::IncorrectPersonalityType => write!(f, "IncorrectPersonalityType"),
            IssueType::MissingRangeDocumentation => write!(f, "MissingRangeDocumentation"),
            IssueType::MissingBipolarDocumentation => write!(f, "MissingBipolarDocumentation"),
            IssueType::MissingPersonalityModulation => write!(f, "MissingPersonalityModulation"),
            IssueType::IncorrectTemporalUsage => write!(f, "IncorrectTemporalUsage"),
            IssueType::MissingValidationMethod => write!(f, "MissingValidationMethod"),
            IssueType::DirectComponentAccess => write!(f, "DirectComponentAccess"),
            IssueType::MissingBehavioralDocumentation => write!(f, "MissingBehavioralDocumentation"),
            
            // Behavioral Consistency Issues
            IssueType::MissingPersonalityConsistency => write!(f, "MissingPersonalityConsistency"),
            IssueType::HardcodedBehavioralValue => write!(f, "HardcodedBehavioralValue"),
            IssueType::MissingMisunderstandingMechanics => write!(f, "MissingMisunderstandingMechanics"),
            IssueType::PerfectCommunication => write!(f, "PerfectCommunication"),
            IssueType::UnrealisticContagionSpeed => write!(f, "UnrealisticContagionSpeed"),
            IssueType::MissingInformationLoss => write!(f, "MissingInformationLoss"),
            IssueType::MissingConfirmationBias => write!(f, "MissingConfirmationBias"),
            IssueType::MissingSubjectivePerception => write!(f, "MissingSubjectivePerception"),
            IssueType::OptimalDecisionMaking => write!(f, "OptimalDecisionMaking"),
            IssueType::MissingEmotionalDecisionInfluence => write!(f, "MissingEmotionalDecisionInfluence"),
            IssueType::MissingEmotionalLogic => write!(f, "MissingEmotionalLogic"),
            IssueType::MissingStressMoodImpact => write!(f, "MissingStressMoodImpact"),
            IssueType::MissingBehavioralFeedback => write!(f, "MissingBehavioralFeedback"),
            IssueType::MissingDecisionPersonalityConsistency => write!(f, "MissingDecisionPersonalityConsistency"),
            
            // Performance Regression Issues
            IssueType::PerformanceRegressionSetupFailed => write!(f, "PerformanceRegressionSetupFailed"),
            IssueType::BaselineEstablishmentFailed => write!(f, "BaselineEstablishmentFailed"),
            IssueType::PerformanceTestFailed => write!(f, "PerformanceTestFailed"),
            IssueType::FpsRegression => write!(f, "FpsRegression"),
            IssueType::MemoryRegression => write!(f, "MemoryRegression"),
            IssueType::CpuRegression => write!(f, "CpuRegression"),
            IssueType::FpsTargetNotMet => write!(f, "FpsTargetNotMet"),
            IssueType::MissingDocumentation => write!(f, "MissingDocumentation"),
            IssueType::LowQualityDocumentation => write!(f, "LowQualityDocumentation"),
            IssueType::ExcessiveComplexity => write!(f, "ExcessiveComplexity"),
            IssueType::ExcessiveFunctionLength => write!(f, "ExcessiveFunctionLength"),
            IssueType::CodeDuplication => write!(f, "CodeDuplication"),
            IssueType::MissingTestCoverage => write!(f, "MissingTestCoverage"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_personality_trait_validation() {
        let checker = PersonalityTraitChecker::new();
        
        // Test code with correct personality trait usage
        let correct_code = r#"
        #[derive(Component)]
        pub struct PersonalityVector {
            /// openness: 0.0 (low) to 1.0 (high)
            pub openness: Normalized,
            /// conscientiousness: 0.0 (low) to 1.0 (high)
            pub conscientiousness: Normalized,
        }
        "#;
        
        let result = checker.validate_personality_component(correct_code, Path::new("test.rs"));
        assert!(result.passed, "Correct personality trait usage should pass validation");
        
        // Test code with incorrect personality trait usage
        let incorrect_code = r#"
        #[derive(Component)]
        pub struct PersonalityVector {
            pub openness: f32,
            pub conscientiousness: f32,
        }
        "#;
        
        let result = checker.validate_personality_component(incorrect_code, Path::new("test.rs"));
        assert!(!result.passed, "Incorrect personality trait usage should fail validation");
        assert_eq!(result.issues.len(), 4); // 2 type issues + 2 documentation issues
    }

    #[test]
    fn test_emotional_state_validation() {
        let checker = EmotionalStateChecker::new();
        
        // Test code with correct emotional state usage
        let correct_code = r#"
        #[derive(Component)]
        pub struct EmotionalState {
            /// valence: -1.0 (negative) to 1.0 (positive)
            pub valence: f32,
        }
        
        impl EmotionalState {
            pub fn validate_valence(&self) -> Result<(), String> {
                if !(-1.0..=1.0).contains(&self.valence) {
                    Err("Invalid valence range".to_string())
                } else {
                    Ok(())
                }
            }
        }
        "#;
        
        let result = checker.validate_emotional_component(correct_code, Path::new("test.rs"));
        assert!(result.passed, "Correct emotional state usage should pass validation");
        
        // Test code with missing documentation
        let incorrect_code = r#"
        #[derive(Component)]
        pub struct EmotionalState {
            pub valence: f32,
        }
        "#;
        
        let result = checker.validate_emotional_component(incorrect_code, Path::new("test.rs"));
        assert!(!result.passed, "Missing emotional state documentation should fail validation");
        assert_eq!(result.issues.len(), 2); // Missing documentation + missing validation method
    }

    #[test]
    fn test_system_modulation_validation() {
        let checker = SystemModulationChecker::new();
        
        // Test system with personality modulation
        let correct_code = r#"
        fn emotional_contagion_system(
            mut agents: Query<(&mut EmotionalState, &Personality)>,
        ) {
            for (mut emotion, personality) in agents.iter_mut() {
                let openness_factor = personality.openness.value();
                emotion.valence *= openness_factor;
            }
        }
        "#;
        
        let result = checker.validate_system_modulation(correct_code, Path::new("test.rs"));
        assert!(result.passed, "System with personality modulation should pass validation");
        
        // Test system without personality modulation
        let incorrect_code = r#"
        fn simple_system(
            mut agents: Query<&mut EmotionalState>,
        ) {
            for mut emotion in agents.iter_mut() {
                emotion.valence += 0.1;
            }
        }
        "#;
        
        let result = checker.validate_system_modulation(incorrect_code, Path::new("test.rs"));
        assert!(!result.passed, "System without personality modulation should fail validation");
        assert_eq!(result.issues.len(), 1);
    }

    #[test]
    fn test_temporal_consistency_validation() {
        let checker = TemporalConsistencyChecker::new();
        
        // Test code with correct WorldTime usage
        let correct_code = r#"
        fn memory_decay_system(
            mut agents: Query<&mut SocialMemory>,
            world_time: Res<WorldTime>,
        ) {
            let hours_elapsed = world_time.delta_time / 3600.0;
        }
        "#;
        
        let result = checker.validate_temporal_consistency(correct_code, Path::new("test.rs"));
        assert!(result.passed, "Correct WorldTime usage should pass validation");
        
        // Test code with real-world time usage
        let incorrect_code = r#"
        fn bad_system() {
            let now = std::time::Instant::now();
            let elapsed = now.elapsed();
        }
        "#;
        
        let result = checker.validate_temporal_consistency(incorrect_code, Path::new("test.rs"));
        assert!(!result.passed, "Real-world time usage should fail validation");
        assert_eq!(result.issues.len(), 2); // Instant::now() + elapsed()
    }

    #[test]
    fn test_architecture_validation() {
        let checker = ArchitectureChecker::new();
        
        // Test code with event-driven communication
        let correct_code = r#"
        /// This system handles emotional contagion between agents
        fn emotional_contagion_system(
            mut events: EventWriter<EmotionChanged>,
        ) {
            events.send(EmotionChanged { entity: Entity::PLACEHOLDER });
        }
        "#;
        
        let result = checker.validate_ai_architecture(correct_code, Path::new("test.rs"));
        assert!(result.passed, "Event-driven architecture should pass validation");
        
        // Test code with direct component access
        let incorrect_code = r#"
        fn bad_system(world: &mut World) {
            let component = world.get::<EmotionalState>(Entity::PLACEHOLDER);
        }
        "#;
        
        let result = checker.validate_ai_architecture(incorrect_code, Path::new("test.rs"));
        assert!(!result.passed, "Direct component access should fail validation");
        assert_eq!(result.issues.len(), 2); // Direct access + missing behavioral documentation
    }

    #[test]
    fn test_full_ai_pattern_validator() {
        let validator = AiPatternValidator::new();
        
        // Test a comprehensive AI component file
        let test_code = r#"
        use bevy::prelude::*;
        use crate::core::types::Normalized;
        
        #[derive(Component)]
        pub struct PersonalityVector {
            /// openness: 0.0 (low) to 1.0 (high)
            pub openness: Normalized,
            /// conscientiousness: 0.0 (low) to 1.0 (high)
            pub conscientiousness: Normalized,
        }
        
        #[derive(Component)]
        pub struct EmotionalState {
            /// valence: -1.0 (negative) to 1.0 (positive)
            pub valence: f32,
        }
        
        impl EmotionalState {
            pub fn validate_valence(&self) -> Result<(), String> {
                if !(-1.0..=1.0).contains(&self.valence) {
                    Err("Invalid valence range".to_string())
                } else {
                    Ok(())
                }
            }
        }
        
        /// This system handles emotional contagion between agents based on personality
        fn emotional_contagion_system(
            mut agents: Query<(&mut EmotionalState, &Personality)>,
            world_time: Res<WorldTime>,
            mut events: EventWriter<EmotionChanged>,
        ) {
            for (mut emotion, personality) in agents.iter_mut() {
                let openness_factor = personality.openness.value();
                let time_factor = world_time.delta_time;
                emotion.valence *= openness_factor * time_factor;
                
                events.send(EmotionChanged { entity: Entity::PLACEHOLDER });
            }
        }
        "#;
        
        let result = validator.validate_file(test_code, Path::new("test.rs"));
        assert!(result.passed, "Well-structured AI code should pass all validations");
    }
}
