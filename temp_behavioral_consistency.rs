use regex::Regex;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::cicd::validation::{ValidationResult, ValidationIssue, IssueType};
use crate::cicd::validation::ai_patterns::{IssueSeverity, CheckType};

// Helper function to create ValidationIssue with default values
fn create_validation_issue(
    issue_type: IssueType,
    message: String,
    file_path: &Path,
    line_number: usize,
    suggestion: Option<String>,
) -> ValidationIssue {
    ValidationIssue {
        issue_type,
        message,
        file_path: file_path.to_path_buf(),
        line_number,
        column_number: None,
        severity: IssueSeverity::Medium,
        suggestion,
        code_example: None,
    }
}

/// Validates behavioral consistency patterns for AI believability.
#[derive(Debug)]
pub struct BehavioralConsistencyValidator {
    pub personality_consistency_validator: PersonalityConsistencyValidator,
    pub social_dynamics_validator: SocialDynamicsValidator,
    pub decision_system_validator: DecisionSystemValidator,
}

impl BehavioralConsistencyValidator {
    pub fn new() -> Self {
        Self {
            personality_consistency_validator: PersonalityConsistencyValidator::new(),
            social_dynamics_validator: SocialDynamicsValidator::new(),
            decision_system_validator: DecisionSystemValidator::new(),
        }
    }

    pub fn validate_file(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Run all behavioral consistency validation checks
        let personality_result = self.personality_consistency_validator.validate_personality_consistency(file_content, file_path);
        all_issues.extend(personality_result.issues);
        all_suggestions.extend(personality_result.suggestions);

        let social_result = self.social_dynamics_validator.validate_social_dynamics(file_content, file_path);
        all_issues.extend(social_result.issues);
        all_suggestions.extend(social_result.suggestions);

        let decision_result = self.decision_system_validator.validate_decision_system(file_content, file_path);
        all_issues.extend(decision_result.issues);
        all_suggestions.extend(decision_result.suggestions);

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }
}

impl Default for BehavioralConsistencyValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates personality consistency for observable personality differences.
#[derive(Debug)]
pub struct PersonalityConsistencyValidator {
    pub required_personality_impact: bool,
    pub personality_trait_patterns: Vec<Regex>,
    pub behavioral_outcome_patterns: Vec<Regex>,
}

impl PersonalityConsistencyValidator {
    pub fn new() -> Self {
        let personality_trait_patterns = vec![
            Regex::new(r"personality\.openness").unwrap(),
            Regex::new(r"personality\.conscientiousness").unwrap(),
            Regex::new(r"personality\.extraversion").unwrap(),
            Regex::new(r"personality\.agreeableness").unwrap(),
            Regex::new(r"personality\.neuroticism").unwrap(),
        ];

        let behavioral_outcome_patterns = vec![
            Regex::new(r"decision\.|make_decision|choose_action").unwrap(),
            Regex::new(r"social_interaction|communicate|respond").unwrap(),
            Regex::new(r"emotional_response|emotion\.|feeling").unwrap(),
            Regex::new(r"stress_response|anxiety|fear").unwrap(),
        ];

        Self {
            required_personality_impact: true,
            personality_trait_patterns,
            behavioral_outcome_patterns,
        }
    }

    pub fn validate_personality_consistency(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        
        // Find AI systems that should be personality-driven
        let system_regex = Regex::new(r"fn (\w+_system)\s*\(").unwrap();
        
        for captures in system_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            // Skip non-AI systems
            if !self.is_ai_system(system_name) {
                continue;
            }
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check if system produces behavioral outcomes
                if self.has_behavioral_outcomes(&system_body) {
                    // Verify personality traits impact the behavior
                    if !self.has_personality_impact(&system_body) {
                        issues.push(create_validation_issue(
                            IssueType::MissingPersonalityConsistency,
                            format!(
                                "AI system '{}' produces behavioral outcomes but lacks personality trait impact. Personality differences should be observable.",
                                system_name
                            ),
                            file_path,
                            self.find_line_number(file_content, &captures[0]),
                            Some("Add personality trait modulation to ensure consistent character behavior across different scenarios".to_string()),
                        ));
                    }
                    
                    // Check for consistent personality application
                    let consistency_issues = self.check_personality_consistency(&system_body, system_name, file_path);
                    issues.extend(consistency_issues);
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn is_ai_system(&self, system_name: &str) -> bool {
        let ai_system_indicators = [
            "decision", "emotion", "social", "personality", "behavior", 
            "cognitive", "memory", "learning", "perception", "expression"
        ];
        
        ai_system_indicators.iter().any(|indicator| 
            system_name.to_lowercase().contains(indicator)
        )
    }

    fn has_behavioral_outcomes(&self, system_body: &str) -> bool {
        self.behavioral_outcome_patterns.iter().any(|pattern| 
            pattern.is_match(system_body)
        )
    }

    fn has_personality_impact(&self, system_body: &str) -> bool {
        self.personality_trait_patterns.iter().any(|pattern| 
            pattern.is_match(system_body)
        )
    }

    fn check_personality_consistency(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for hardcoded behavioral values that should be personality-driven
        let hardcoded_patterns = [
            r"0\.[0-9]+", // Hardcoded probability/strength values
            r"[0-9]+\.[0-9]+", // Hardcoded thresholds
        ];
        
        for pattern_str in &hardcoded_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            
            for mat in pattern.find_iter(system_body) {
                let line_number = self.find_line_number(system_body, mat.as_str());
                let line_content = self.get_line_content(system_body, line_number);
                
                // Skip if this value is already personality-modulated
                if !line_content.contains("personality") && 
                   self.is_behavioral_context(line_content) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::HardcodedBehavioralValue,
                        message: format!(
                            "System '{}' uses hardcoded behavioral value '{}' that should be personality-driven",
                            system_name, mat.as_str()
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        suggestion: Some(
                            "Replace hardcoded values with personality-modulated calculations for consistent character behavior".to_string()
                        ),
                    });
                }
            }
        }
        
        issues
    }

    fn is_behavioral_context(&self, line_content: &str) -> bool {
        let behavioral_keywords = [
            "threshold", "probability", "strength", "intensity", "rate", 
            "factor", "weight", "influence", "response", "reaction"
        ];
        
        behavioral_keywords.iter().any(|keyword| 
            line_content.to_lowercase().contains(keyword)
        )
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

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn get_line_content<'a>(&self, content: &'a str, line_number: usize) -> &'a str {
        content.lines().nth(line_number - 1).unwrap_or("")
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::MissingPersonalityConsistency => {
                    suggestions.push("Ensure all AI behavioral systems are modulated by personality traits for observable character differences".to_string());
                }
                IssueType::HardcodedBehavioralValue => {
                    suggestions.push("Replace hardcoded behavioral values with personality-driven calculations".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for PersonalityConsistencyValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates social dynamics for believable interaction patterns.
#[derive(Debug)]
pub struct SocialDynamicsValidator {
    pub misunderstanding_rate_range: (f32, f32), // 20-40% target range
    pub emotional_contagion_patterns: Vec<Regex>,
    pub communication_pipeline_patterns: Vec<Regex>,
}

impl SocialDynamicsValidator {
    pub fn new() -> Self {
        let emotional_contagion_patterns = vec![
            Regex::new(r"emotional_contagion|contagion_system").unwrap(),
            Regex::new(r"emotion.*spread|spread.*emotion").unwrap(),
            Regex::new(r"influence.*emotion|emotion.*influence").unwrap(),
        ];

        let communication_pipeline_patterns = vec![
            Regex::new(r"expression.*perception|perception.*expression").unwrap(),
            Regex::new(r"communicate.*interpret|interpret.*communicate").unwrap(),
            Regex::new(r"message.*filter|filter.*message").unwrap(),
            Regex::new(r"plato.*cave|information.*loss").unwrap(),
        ];

        Self {
            misunderstanding_rate_range: (0.2, 0.4), // 20-40%
            emotional_contagion_patterns,
            communication_pipeline_patterns,
        }
    }

    pub fn validate_social_dynamics(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        
        // Check for misunderstanding rate validation
        let misunderstanding_issues = self.validate_misunderstanding_rates(file_content, file_path);
        issues.extend(misunderstanding_issues);
        
        // Check for emotional contagion speed validation
        let contagion_issues = self.validate_emotional_contagion_speed(file_content, file_path);
        issues.extend(contagion_issues);
        
        // Check for communication pipeline information loss
        let communication_issues = self.validate_communication_pipeline(file_content, file_path);
        issues.extend(communication_issues);
        
        // Check for social interaction believability
        let believability_issues = self.validate_social_believability(file_content, file_path);
        issues.extend(believability_issues);
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn validate_misunderstanding_rates(&self, file_content: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Look for social interaction systems
        let social_system_regex = Regex::new(r"fn (\w*(?:social|communication|interaction)\w*_system)\s*\(").unwrap();
        
        for captures in social_system_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check if system implements misunderstanding mechanics
                if !self.has_misunderstanding_mechanics(&system_body) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::MissingMisunderstandingMechanics,
                        message: format!(
                            "Social system '{}' lacks misunderstanding mechanics. Target 20-40% misunderstanding rate for believable social dynamics.",
                            system_name
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: self.find_line_number(file_content, &captures[0]),
                        suggestion: Some(
                            "Add misunderstanding mechanics with 20-40% failure rate to create believable social conflicts".to_string()
                        ),
                    });
                }
                
                // Check for hardcoded perfect communication
                if self.has_perfect_communication(&system_body) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::PerfectCommunication,
                        message: format!(
                            "Social system '{}' implements perfect communication. This eliminates drama and conflict.",
                            system_name
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: self.find_line_number(file_content, &captures[0]),
                        suggestion: Some(
                            "Add communication failures and misinterpretation to create interesting social dynamics".to_string()
                        ),
                    });
                }
            }
        }
        
        issues
    }

    fn validate_emotional_contagion_speed(&self, file_content: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Look for emotional contagion systems
        for pattern in &self.emotional_contagion_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.as_str());
                let system_context = self.get_system_context(file_content, line_number);
                
                // Check for believable contagion speed
                if !self.has_believable_contagion_speed(&system_context) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::UnrealisticContagionSpeed,
                        message: format!(
                            "Emotional contagion at line {} may have unrealistic speed. Ensure gradual, believable emotional spread.",
                            line_number
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        suggestion: Some(
                            "Implement gradual emotional contagion with realistic time delays and personality-based susceptibility".to_string()
                        ),
                    });
                }
            }
        }
        
        issues
    }

    fn validate_communication_pipeline(&self, file_content: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Look for communication pipeline systems
        for pattern in &self.communication_pipeline_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.as_str());
                let system_context = self.get_system_context(file_content, line_number);
                
                // Check for "Plato's Cave" information loss
                if !self.has_information_loss(&system_context) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::MissingInformationLoss,
                        message: format!(
                            "Communication pipeline at line {} lacks 'Plato's Cave' information loss. Perfect information transfer is unrealistic.",
                            line_number
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        suggestion: Some(
                            "Add information loss layers: expression filtering, perception bias, and interpretation errors".to_string()
                        ),
                    });
                }
            }
        }
        
        issues
    }

    fn validate_social_believability(&self, file_content: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for social interaction systems
        let social_regex = Regex::new(r"fn (\w*social\w*_system)\s*\(").unwrap();
        
        for captures in social_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check for confirmation bias implementation
                if !self.has_confirmation_bias(&system_body) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::MissingConfirmationBias,
                        message: format!(
                            "Social system '{}' lacks confirmation bias mechanics. Agents should interpret information through their existing beliefs.",
                            system_name
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: self.find_line_number(file_content, &captures[0]),
                        suggestion: Some(
                            "Add confirmation bias where agents interpret ambiguous information to confirm existing beliefs".to_string()
                        ),
                    });
                }
                
                // Check for subjective perception
                if !self.has_subjective_perception(&system_body) {
                    issues.push(create_validation_issue(
                        issue_type: IssueType::MissingSubjectivePerception,
                        message: format!(
                            "Social system '{}' lacks subjective perception. Same events should be perceived differently by different agents.",
                            system_name
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: self.find_line_number(file_content, &captures[0]),
                        suggestion: Some(
                            "Add personality and emotional state filtering to perception for subjective interpretation".to_string()
                        ),
                    });
                }
            }
        }
        
        issues
    }

    fn has_misunderstanding_mechanics(&self, system_body: &str) -> bool {
        let misunderstanding_indicators = [
            "misunderstand", "misinterpret", "failure_rate", "communication_error",
            "0.2", "0.3", "0.4", "20%", "30%", "40%"
        ];
        
        misunderstanding_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn has_perfect_communication(&self, system_body: &str) -> bool {
        // Look for patterns that suggest perfect information transfer
        let perfect_patterns = [
            r"\.clone\(\).*message", // Direct message cloning
            r"message.*=.*message", // Direct message assignment
            r"100%.*success", // Perfect success rates
        ];
        
        perfect_patterns.iter().any(|pattern| 
            Regex::new(pattern).unwrap().is_match(system_body)
        ) && !system_body.contains("error") && !system_body.contains("failure")
    }

    fn has_believable_contagion_speed(&self, system_context: &str) -> bool {
        // Check for time-based or gradual contagion
        let believable_indicators = [
            "delta_time", "time_factor", "gradual", "slow", "rate",
            "personality", "susceptibility", "resistance"
        ];
        
        believable_indicators.iter().any(|indicator| 
            system_context.to_lowercase().contains(indicator)
        )
    }

    fn has_information_loss(&self, system_context: &str) -> bool {
        let information_loss_indicators = [
            "filter", "bias", "distort", "lose", "degrade", "noise",
            "personality", "emotional_state", "perception"
        ];
        
        information_loss_indicators.iter().any(|indicator| 
            system_context.to_lowercase().contains(indicator)
        )
    }

    fn has_confirmation_bias(&self, system_body: &str) -> bool {
        let bias_indicators = [
            "confirmation", "bias", "belief", "existing", "prior",
            "interpret", "filter", "selective"
        ];
        
        bias_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn has_subjective_perception(&self, system_body: &str) -> bool {
        let subjective_indicators = [
            "personality", "emotional_state", "perspective", "subjective",
            "individual", "different", "vary", "personal"
        ];
        
        subjective_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn find_system_body(&self, content: &str, system_name: &str) -> Option<String> {
        let system_start_pattern = format!(r"fn {}\s*\([^{{]*\{{", regex::escape(system_name));
        let system_start_regex = Regex::new(&system_start_pattern).unwrap();
        
        if let Some(start_match) = system_start_regex.find(content) {
            let start_pos = start_match.end() - 1;
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

    fn get_system_context(&self, content: &str, line_number: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let start = line_number.saturating_sub(10);
        let end = (line_number + 10).min(lines.len());
        lines[start..end].join("\n")
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
                IssueType::MissingMisunderstandingMechanics => {
                    suggestions.push("Implement 20-40% misunderstanding rate in social interactions for believable conflicts".to_string());
                }
                IssueType::PerfectCommunication => {
                    suggestions.push("Add communication failures and misinterpretation to prevent perfect information transfer".to_string());
                }
                IssueType::UnrealisticContagionSpeed => {
                    suggestions.push("Implement gradual emotional contagion with personality-based susceptibility".to_string());
                }
                IssueType::MissingInformationLoss => {
                    suggestions.push("Add 'Plato's Cave' information loss through expression, perception, and interpretation layers".to_string());
                }
                IssueType::MissingConfirmationBias => {
                    suggestions.push("Implement confirmation bias where agents interpret information through existing beliefs".to_string());
                }
                IssueType::MissingSubjectivePerception => {
                    suggestions.push("Add personality and emotional filtering to create subjective perception of events".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for SocialDynamicsValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates decision systems for emotionally logical but not optimal choices.
#[derive(Debug)]
pub struct DecisionSystemValidator {
    pub feel_over_science_patterns: Vec<Regex>,
    pub optimal_decision_patterns: Vec<Regex>,
    pub emotional_logic_patterns: Vec<Regex>,
}

impl DecisionSystemValidator {
    pub fn new() -> Self {
        let feel_over_science_patterns = vec![
            Regex::new(r"emotion.*decision|decision.*emotion").unwrap(),
            Regex::new(r"feeling.*choice|choice.*feeling").unwrap(),
            Regex::new(r"intuition|gut_feeling|instinct").unwrap(),
        ];

        let optimal_decision_patterns = vec![
            Regex::new(r"optimal|maximize|minimize|best_choice").unwrap(),
            Regex::new(r"utility.*max|max.*utility").unwrap(),
            Regex::new(r"perfect.*decision|ideal.*choice").unwrap(),
        ];

        let emotional_logic_patterns = vec![
            Regex::new(r"emotional_state.*influence|influence.*emotional").unwrap(),
            Regex::new(r"mood.*affect|affect.*mood").unwrap(),
            Regex::new(r"stress.*decision|decision.*stress").unwrap(),
        ];

        Self {
            feel_over_science_patterns,
            optimal_decision_patterns,
            emotional_logic_patterns,
        }
    }

    pub fn validate_decision_system(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        
        // Find decision-making systems
        let decision_system_regex = Regex::new(r"fn (\w*(?:decision|choice|select)\w*_system)\s*\(").unwrap();
        
        for captures in decision_system_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check for "Feel Over Science" philosophy compliance
                let philosophy_issues = self.validate_feel_over_science(&system_body, system_name, file_path);
                issues.extend(philosophy_issues);
                
                // Check for emotional logic implementation
                let emotional_logic_issues = self.validate_emotional_logic(&system_body, system_name, file_path);
                issues.extend(emotional_logic_issues);
                
                // Check for believable decision patterns
                let believability_issues = self.validate_decision_believability(&system_body, system_name, file_path);
                issues.extend(believability_issues);
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn validate_feel_over_science(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for optimal decision-making patterns (should be avoided)
        for pattern in &self.optimal_decision_patterns {
            for mat in pattern.find_iter(system_body) {
                issues.push(create_validation_issue(
                    issue_type: IssueType::OptimalDecisionMaking,
                    message: format!(
                        "Decision system '{}' uses optimal decision-making pattern '{}'. This violates 'Feel Over Science' philosophy.",
                        system_name, mat.as_str()
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number: self.find_line_number(system_body, mat.as_str()),
                    suggestion: Some(
                        "Replace optimal decision-making with emotionally logical but suboptimal choices for believable behavior".to_string()
                    ),
                });
            }
        }
        
        // Check for emotional decision influence
        if !self.has_emotional_decision_influence(system_body) {
            issues.push(create_validation_issue(
                issue_type: IssueType::MissingEmotionalDecisionInfluence,
                message: format!(
                    "Decision system '{}' lacks emotional influence. Decisions should be emotionally logical, not purely rational.",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                suggestion: Some(
                    "Add emotional state and personality influence to decision-making for 'Feel Over Science' compliance".to_string()
                ),
            });
        }
        
        issues
    }

    fn validate_emotional_logic(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for emotional logic patterns
        if !self.has_emotional_logic_patterns(system_body) {
            issues.push(create_validation_issue(
                issue_type: IssueType::MissingEmotionalLogic,
                message: format!(
                    "Decision system '{}' lacks emotional logic patterns. Decisions should make emotional sense even if not optimal.",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                suggestion: Some(
                    "Implement emotional logic where decisions feel right to the agent based on their emotional state and personality".to_string()
                ),
            });
        }
        
        // Check for stress and mood impact on decisions
        if !self.has_stress_mood_impact(system_body) {
            issues.push(create_validation_issue(
                issue_type: IssueType::MissingStressMoodImpact,
                message: format!(
                    "Decision system '{}' doesn't account for stress and mood impact on decision quality.",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                suggestion: Some(
                    "Add stress and mood factors that degrade decision quality for realistic human-like behavior".to_string()
                ),
            });
        }
        
        issues
    }

    fn validate_decision_believability(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for behavioral feedback system
        if !self.has_behavioral_feedback(system_body) {
            issues.push(create_validation_issue(
                issue_type: IssueType::MissingBehavioralFeedback,
                message: format!(
                    "Decision system '{}' lacks behavioral feedback mechanisms to detect believability compromises.",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                suggestion: Some(
                    "Add feedback system to detect when decisions become too optimal or lose emotional believability".to_string()
                ),
            });
        }
        
        // Check for decision consistency with personality
        if !self.has_personality_consistency(system_body) {
            issues.push(create_validation_issue(
                issue_type: IssueType::MissingDecisionPersonalityConsistency,
                message: format!(
                    "Decision system '{}' doesn't ensure decisions are consistent with agent personality over time.",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                suggestion: Some(
                    "Add personality consistency checks to ensure decisions align with established character traits".to_string()
                ),
            });
        }
        
        issues
    }

    fn has_emotional_decision_influence(&self, system_body: &str) -> bool {
        self.feel_over_science_patterns.iter().any(|pattern| 
            pattern.is_match(system_body)
        )
    }

    fn has_emotional_logic_patterns(&self, system_body: &str) -> bool {
        self.emotional_logic_patterns.iter().any(|pattern| 
            pattern.is_match(system_body)
        )
    }

    fn has_stress_mood_impact(&self, system_body: &str) -> bool {
        let stress_mood_indicators = [
            "stress", "mood", "anxiety", "fatigue", "overwhelm",
            "emotional_state", "mental_state", "cognitive_load"
        ];
        
        stress_mood_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn has_behavioral_feedback(&self, system_body: &str) -> bool {
        let feedback_indicators = [
            "feedback", "monitor", "detect", "assess", "evaluate",
            "believability", "consistency", "quality"
        ];
        
        feedback_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn has_personality_consistency(&self, system_body: &str) -> bool {
        let consistency_indicators = [
            "personality", "consistent", "character", "trait",
            "align", "match", "coherent"
        ];
        
        consistency_indicators.iter().any(|indicator| 
            system_body.to_lowercase().contains(indicator)
        )
    }

    fn find_system_body(&self, content: &str, system_name: &str) -> Option<String> {
        let system_start_pattern = format!(r"fn {}\s*\([^{{]*\{{", regex::escape(system_name));
        let system_start_regex = Regex::new(&system_start_pattern).unwrap();
        
        if let Some(start_match) = system_start_regex.find(content) {
            let start_pos = start_match.end() - 1;
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

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::OptimalDecisionMaking => {
                    suggestions.push("Replace optimal decision algorithms with emotionally logical but suboptimal choices".to_string());
                }
                IssueType::MissingEmotionalDecisionInfluence => {
                    suggestions.push("Add emotional state and personality influence to all decision-making systems".to_string());
                }
                IssueType::MissingEmotionalLogic => {
                    suggestions.push("Implement emotional logic where decisions feel right to the agent even if not optimal".to_string());
                }
                IssueType::MissingStressMoodImpact => {
                    suggestions.push("Add stress and mood factors that realistically degrade decision quality".to_string());
                }
                IssueType::MissingBehavioralFeedback => {
                    suggestions.push("Implement feedback systems to detect and prevent believability compromises".to_string());
                }
                IssueType::MissingDecisionPersonalityConsistency => {
                    suggestions.push("Ensure decision patterns remain consistent with established personality traits".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for DecisionSystemValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_personality_consistency_validation() {
        let validator = PersonalityConsistencyValidator::new();
        
        // Test code with personality-driven behavior
        let good_code = r#"
        fn decision_making_system(
            mut agents: Query<(&mut DecisionState, &Personality, &EmotionalState)>,
        ) {
            for (mut decision, personality, emotion) in agents.iter_mut() {
                let openness_factor = personality.openness.value();
                let risk_tolerance = openness_factor * emotion.valence;
                decision.choice = make_decision_with_risk(risk_tolerance);
            }
        }
        "#;
        
        let result = validator.validate_personality_consistency(good_code, Path::new("test.rs"));
        assert!(result.passed, "Personality-driven behavior should pass validation");
        
        // Test code without personality influence
        let bad_code = r#"
        fn decision_making_system(
            mut agents: Query<&mut DecisionState>,
        ) {
            for mut decision in agents.iter_mut() {
                decision.choice = make_optimal_decision();
            }
        }
        "#;
        
        let result = validator.validate_personality_consistency(bad_code, Path::new("test.rs"));
        assert!(!result.passed, "Non-personality-driven behavior should fail validation");
    }

    #[test]
    fn test_social_dynamics_validation() {
        let validator = SocialDynamicsValidator::new();
        
        // Test code with misunderstanding mechanics
        let good_code = r#"
        fn social_interaction_system(
            mut interactions: EventReader<SocialInteraction>,
        ) {
            for interaction in interactions.read() {
                let misunderstanding_rate = 0.3; // 30% failure rate
                if random() < misunderstanding_rate {
                    create_misunderstanding(interaction);
                }
            }
        }
        "#;
        
        let result = validator.validate_social_dynamics(good_code, Path::new("test.rs"));
        assert!(result.passed, "Social dynamics with misunderstanding should pass validation");
        
        // Test code with perfect communication
        let bad_code = r#"
        fn social_interaction_system(
            mut interactions: EventReader<SocialInteraction>,
        ) {
            for interaction in interactions.read() {
                let message = interaction.message.clone();
                send_perfect_message(message);
            }
        }
        "#;
        
        let result = validator.validate_social_dynamics(bad_code, Path::new("test.rs"));
        assert!(!result.passed, "Perfect communication should fail validation");
    }

    #[test]
    fn test_decision_system_validation() {
        let validator = DecisionSystemValidator::new();
        
        // Test code with emotional decision influence
        let good_code = r#"
        fn decision_system(
            mut agents: Query<(&mut Decision, &EmotionalState, &Personality)>,
        ) {
            for (mut decision, emotion, personality) in agents.iter_mut() {
                let emotional_weight = emotion.valence * personality.neuroticism.value();
                decision.choice = make_emotionally_logical_choice(emotional_weight);
            }
        }
        "#;
        
        let result = validator.validate_decision_system(good_code, Path::new("test.rs"));
        assert!(result.passed, "Emotionally influenced decisions should pass validation");
        
        // Test code with optimal decision-making
        let bad_code = r#"
        fn decision_system(
            mut agents: Query<&mut Decision>,
        ) {
            for mut decision in agents.iter_mut() {
                decision.choice = find_optimal_solution();
            }
        }
        "#;
        
        let result = validator.validate_decision_system(bad_code, Path::new("test.rs"));
        assert!(!result.passed, "Optimal decision-making should fail validation");
    }
}
