use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::cicd::pipeline::{
    CiCdConfiguration as PipelineCiCdConfiguration, 
    ParallelExecutionConfig,
    TimeoutConfiguration,
    NotificationConfiguration,
    QualityGateConfiguration,
    ReportingConfiguration,
    ValidationType,
};
use crate::cicd::git_hooks::{HookConfiguration, BypassConfiguration, BypassCondition};
// Remove unused import - we'll define our own ConfigurationError

/// Comprehensive CI/CD configuration system with hot-reload capabilities.
#[derive(Debug)]
pub struct CiCdConfigurationManager {
    pub configuration: Arc<RwLock<ComprehensiveCiCdConfiguration>>,
    pub config_file_path: PathBuf,
    pub file_watcher: Option<ConfigFileWatcher>,
    pub validation_rules: ValidationRulesConfiguration,
    pub hot_reload_enabled: bool,
}

/// Complete CI/CD configuration encompassing all aspects of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveCiCdConfiguration {
    pub pre_commit_config: PreCommitConfiguration,
    pub pre_push_config: PrePushConfiguration,
    pub ci_pipeline_config: PipelineCiCdConfiguration,
    pub validation_rules: ValidationRulesConfiguration,
    pub git_hooks_config: GitHooksConfiguration,
    pub performance_targets: PerformanceTargetsConfiguration,
    pub behavioral_validation_config: BehavioralValidationConfiguration,
    pub security_config: SecurityConfiguration,
    pub documentation_config: DocumentationConfiguration,
    pub cross_platform_config: CrossPlatformConfiguration,
}

/// Configuration for pre-commit hooks and validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCommitConfiguration {
    pub enabled: bool,
    pub timeout_seconds: u64,
    pub enabled_checks: Vec<PreCommitCheckType>,
    pub bypass_conditions: Vec<BypassCondition>,
    pub parallel_execution: bool,
    pub progress_reporting: bool,
    pub fail_fast: bool,
    pub formatting_config: FormattingConfiguration,
    pub linting_config: LintingConfiguration,
    pub testing_config: TestingConfiguration,
}

/// Types of pre-commit checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreCommitCheckType {
    CodeFormatting,
    EssentialLinting,
    UnitTestsChangedModules,
    DocumentationValidationChanged,
    BasicSyntaxCheck,
}

/// Configuration for pre-push hooks and comprehensive validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePushConfiguration {
    pub enabled: bool,
    pub timeout_seconds: u64,
    pub comprehensive_validation: bool,
    pub cross_platform_builds: bool,
    pub security_audit_enabled: bool,
    pub performance_regression_check: bool,
    pub behavioral_consistency_check: bool,
    pub bypass_conditions: Vec<BypassCondition>,
    pub validation_order: Vec<ValidationType>,
    pub parallel_validation: bool,
    pub quality_gates: QualityGateConfiguration,
}

/// Configuration for Git hooks management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHooksConfiguration {
    pub auto_install: bool,
    pub hooks_directory: PathBuf,
    pub pre_commit_hook: HookConfiguration,
    pub pre_push_hook: HookConfiguration,
    pub bypass_config: BypassConfiguration,
    pub hook_templates: HashMap<String, String>,
}

/// Validation rules configuration for different pattern types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRulesConfiguration {
    pub ai_patterns: AiPatternValidationRules,
    pub ecs_architecture: EcsArchitectureValidationRules,
    pub performance_patterns: PerformancePatternValidationRules,
    pub code_quality: CodeQualityValidationRules,
    pub behavioral_consistency: BehavioralConsistencyValidationRules,
}

/// AI pattern validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPatternValidationRules {
    pub personality_trait_validation: PersonalityTraitValidationConfig,
    pub emotional_state_validation: EmotionalStateValidationConfig,
    pub temporal_consistency_validation: TemporalConsistencyValidationConfig,
    pub system_modulation_validation: SystemModulationValidationConfig,
    pub strict_mode: bool,
    pub custom_patterns: Vec<CustomValidationPattern>,
}

/// Personality trait validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraitValidationConfig {
    pub enforce_normalized_type: bool,
    pub require_range_documentation: bool,
    pub valid_trait_names: Vec<String>,
    pub range_min: f32,
    pub range_max: f32,
}

/// Emotional state validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalStateValidationConfig {
    pub enforce_bipolar_range: bool,
    pub require_validation_methods: bool,
    pub valid_emotional_dimensions: Vec<String>,
    pub bipolar_range_min: f32,
    pub bipolar_range_max: f32,
}

/// Temporal consistency validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConsistencyValidationConfig {
    pub enforce_world_time_usage: bool,
    pub detect_real_time_usage: bool,
    pub allowed_real_time_contexts: Vec<String>,
}

/// System modulation validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemModulationValidationConfig {
    pub require_personality_modulation: bool,
    pub detect_unmodulated_systems: bool,
    pub allowed_unmodulated_systems: Vec<String>,
}

/// ECS architecture validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsArchitectureValidationRules {
    pub component_validation: ComponentValidationConfig,
    pub system_validation: SystemValidationConfig,
    pub domain_separation_validation: DomainSeparationValidationConfig,
    pub plugin_validation: PluginValidationConfig,
    pub strict_mode: bool,
}

/// Component validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentValidationConfig {
    pub enforce_pure_data: bool,
    pub detect_behavior_methods: bool,
    pub allowed_method_patterns: Vec<String>,
    pub validate_field_types: bool,
}

/// System validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemValidationConfig {
    pub enforce_query_usage: bool,
    pub detect_direct_component_access: bool,
    pub require_event_communication: bool,
    pub validate_system_signatures: bool,
}

/// Domain separation validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSeparationValidationConfig {
    pub enforce_domain_boundaries: bool,
    pub allowed_domains: Vec<String>,
    pub cross_domain_communication_rules: Vec<CommunicationRule>,
    pub validate_file_organization: bool,
}

/// Plugin validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginValidationConfig {
    pub enforce_single_plugin_per_domain: bool,
    pub validate_plugin_registration: bool,
    pub required_plugin_methods: Vec<String>,
}

/// Performance pattern validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePatternValidationRules {
    pub f64_usage_detection: F64UsageDetectionConfig,
    pub sync_io_detection: SyncIoDetectionConfig,
    pub memory_allocation_detection: MemoryAllocationDetectionConfig,
    pub query_pattern_validation: QueryPatternValidationConfig,
    pub performance_thresholds: PerformanceThresholds,
}

/// F64 usage detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct F64UsageDetectionConfig {
    pub enabled: bool,
    pub detect_in_hot_paths: bool,
    pub allowed_contexts: Vec<String>,
    pub hot_path_patterns: Vec<String>,
}

/// Synchronous I/O detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncIoDetectionConfig {
    pub enabled: bool,
    pub detect_in_systems: bool,
    pub allowed_sync_contexts: Vec<String>,
    pub sync_io_patterns: Vec<String>,
}

/// Memory allocation detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocationDetectionConfig {
    pub enabled: bool,
    pub detect_excessive_allocations: bool,
    pub allocation_threshold: usize,
    pub hot_path_allocation_detection: bool,
}

/// Query pattern validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPatternValidationConfig {
    pub enabled: bool,
    pub detect_inefficient_queries: bool,
    pub validate_query_filters: bool,
    pub max_query_complexity: usize,
}

/// Code quality validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityValidationRules {
    pub documentation_requirements: DocumentationRequirements,
    pub complexity_thresholds: ComplexityThresholds,
    pub duplication_detection: DuplicationDetectionConfig,
    pub naming_conventions: NamingConventionsConfig,
}

/// Behavioral consistency validation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralConsistencyValidationRules {
    pub personality_consistency: PersonalityConsistencyConfig,
    pub social_dynamics_validation: SocialDynamicsValidationConfig,
    pub decision_system_validation: DecisionSystemValidationConfig,
    pub believability_thresholds: BelievabilityThresholds,
}

/// Performance targets configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTargetsConfiguration {
    pub target_fps: f32,
    pub max_agent_count: usize,
    pub memory_usage_limit_mb: usize,
    pub frame_time_budget_ms: f32,
    pub ai_systems_budget_percentage: f32,
    pub regression_tolerance_percentage: f32,
}

/// Behavioral validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralValidationConfiguration {
    pub personality_difference_threshold: f32,
    pub misunderstanding_rate_min: f32,
    pub misunderstanding_rate_max: f32,
    pub emotional_contagion_speed_threshold: f32,
    pub decision_optimality_threshold: f32,
}

/// Security configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    pub cargo_audit_enabled: bool,
    pub warning_only_mode: bool,
    pub license_compatibility_check: bool,
    pub dependency_bloat_detection: bool,
    pub vulnerability_severity_threshold: String,
}

/// Documentation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfiguration {
    pub require_public_api_docs: bool,
    pub require_behavioral_purpose_docs: bool,
    pub validate_doc_examples: bool,
    pub generate_documentation_build: bool,
}

/// Cross-platform configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformConfiguration {
    pub target_platforms: Vec<String>,
    pub test_release_builds: bool,
    pub test_debug_builds: bool,
    pub validate_conditional_compilation: bool,
}

/// Custom validation pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidationPattern {
    pub name: String,
    pub pattern: String,
    pub description: String,
    pub severity: String,
    pub suggestion: String,
}

/// Communication rule for domain separation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRule {
    pub from_domain: String,
    pub to_domain: String,
    pub allowed_methods: Vec<String>,
}

/// Performance thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_frame_time_ms: f32,
    pub max_memory_usage_mb: usize,
    pub max_cpu_usage_percentage: f32,
}

/// Documentation requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationRequirements {
    pub min_doc_coverage_percentage: f32,
    pub require_examples: bool,
    pub require_behavioral_descriptions: bool,
}

/// Complexity thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityThresholds {
    pub max_cyclomatic_complexity: usize,
    pub max_function_length: usize,
    pub max_parameter_count: usize,
}

/// Duplication detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationDetectionConfig {
    pub enabled: bool,
    pub min_duplicate_lines: usize,
    pub ignore_patterns: Vec<String>,
}

/// Naming conventions configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConventionsConfig {
    pub enforce_rust_conventions: bool,
    pub ai_specific_patterns: Vec<String>,
    pub behavioral_naming_required: bool,
}

/// Personality consistency configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityConsistencyConfig {
    pub min_observable_difference: f32,
    pub consistency_check_scenarios: Vec<String>,
}

/// Social dynamics validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDynamicsValidationConfig {
    pub misunderstanding_rate_validation: bool,
    pub emotional_contagion_validation: bool,
    pub communication_loss_validation: bool,
}

/// Decision system validation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionSystemValidationConfig {
    pub emotional_logic_validation: bool,
    pub optimality_avoidance_validation: bool,
    pub feel_over_science_compliance: bool,
}

/// Believability thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BelievabilityThresholds {
    pub min_personality_impact: f32,
    pub max_decision_optimality: f32,
    pub min_emotional_influence: f32,
}

/// Formatting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfiguration {
    pub rust_fmt_enabled: bool,
    pub custom_fmt_rules: HashMap<String, String>,
}

/// Linting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintingConfiguration {
    pub clippy_enabled: bool,
    pub clippy_lints: Vec<String>,
    pub custom_lints: Vec<String>,
}

/// Testing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfiguration {
    pub unit_tests_enabled: bool,
    pub integration_tests_enabled: bool,
    pub behavioral_tests_enabled: bool,
    pub test_timeout_seconds: u64,
}

/// File watcher for configuration hot-reload.
#[derive(Debug)]
pub struct ConfigFileWatcher {
    pub file_path: PathBuf,
    pub last_modified: std::time::SystemTime,
}

impl CiCdConfigurationManager {
    /// Creates a new configuration manager with default settings.
    pub fn new(config_file_path: PathBuf) -> Result<Self, ConfigurationError> {
        let configuration = Arc::new(RwLock::new(Self::default_configuration()));
        let validation_rules = Self::default_validation_rules();
        
        Ok(Self {
            configuration,
            config_file_path,
            file_watcher: None,
            validation_rules,
            hot_reload_enabled: false,
        })
    }
    
    /// Loads configuration from file.
    pub async fn load_from_file(&mut self) -> Result<(), ConfigurationError> {
        if !self.config_file_path.exists() {
            // Create default configuration file
            self.create_default_config_file().await?;
        }
        
        let config_content = tokio::fs::read_to_string(&self.config_file_path).await
            .map_err(|e| ConfigurationError::FileReadError(e.to_string()))?;
        
        let config: ComprehensiveCiCdConfiguration = toml::from_str(&config_content)
            .map_err(|e| ConfigurationError::ParseError(e.to_string()))?;
        
        // Validate configuration
        self.validate_configuration(&config)?;
        
        // Update configuration
        let mut current_config = self.configuration.write().await;
        *current_config = config;
        
        Ok(())
    }
    
    /// Saves current configuration to file.
    pub async fn save_to_file(&self) -> Result<(), ConfigurationError> {
        let config = self.configuration.read().await;
        let config_content = toml::to_string_pretty(&*config)
            .map_err(|e| ConfigurationError::SerializationError(e.to_string()))?;
        
        tokio::fs::write(&self.config_file_path, config_content).await
            .map_err(|e| ConfigurationError::FileWriteError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Enables hot-reload functionality.
    pub fn enable_hot_reload(&mut self) -> Result<(), ConfigurationError> {
        if !self.config_file_path.exists() {
            return Err(ConfigurationError::FileNotFound(
                self.config_file_path.to_string_lossy().to_string()
            ));
        }
        
        let metadata = std::fs::metadata(&self.config_file_path)
            .map_err(|e| ConfigurationError::FileReadError(e.to_string()))?;
        
        self.file_watcher = Some(ConfigFileWatcher {
            file_path: self.config_file_path.clone(),
            last_modified: metadata.modified()
                .map_err(|e| ConfigurationError::FileReadError(e.to_string()))?,
        });
        
        self.hot_reload_enabled = true;
        Ok(())
    }
    
    /// Checks for configuration file changes and reloads if necessary.
    pub async fn check_for_updates(&mut self) -> Result<bool, ConfigurationError> {
        if !self.hot_reload_enabled {
            return Ok(false);
        }
        
        let Some(ref mut watcher) = self.file_watcher else {
            return Ok(false);
        };
        
        let metadata = std::fs::metadata(&watcher.file_path)
            .map_err(|e| ConfigurationError::FileReadError(e.to_string()))?;
        
        let current_modified = metadata.modified()
            .map_err(|e| ConfigurationError::FileReadError(e.to_string()))?;
        
        if current_modified > watcher.last_modified {
            watcher.last_modified = current_modified;
            self.load_from_file().await?;
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Updates a specific configuration section.
    pub async fn update_configuration<F>(&self, updater: F) -> Result<(), ConfigurationError>
    where
        F: FnOnce(&mut ComprehensiveCiCdConfiguration) -> Result<(), ConfigurationError>,
    {
        let mut config = self.configuration.write().await;
        updater(&mut *config)?;
        
        // Validate updated configuration
        self.validate_configuration(&*config)?;
        
        Ok(())
    }
    
    /// Gets a read-only reference to the current configuration.
    pub async fn get_configuration(&self) -> tokio::sync::RwLockReadGuard<ComprehensiveCiCdConfiguration> {
        self.configuration.read().await
    }
    
    /// Validates the configuration for consistency and correctness.
    fn validate_configuration(&self, config: &ComprehensiveCiCdConfiguration) -> Result<(), ConfigurationError> {
        // Validate timeout values
        if config.pre_commit_config.timeout_seconds == 0 {
            return Err(ConfigurationError::InvalidConfiguration(
                "Pre-commit timeout must be greater than 0".to_string()
            ));
        }
        
        if config.pre_push_config.timeout_seconds == 0 {
            return Err(ConfigurationError::InvalidConfiguration(
                "Pre-push timeout must be greater than 0".to_string()
            ));
        }
        
        // Validate performance targets
        if config.performance_targets.target_fps <= 0.0 {
            return Err(ConfigurationError::InvalidConfiguration(
                "Target FPS must be greater than 0".to_string()
            ));
        }
        
        if config.performance_targets.max_agent_count == 0 {
            return Err(ConfigurationError::InvalidConfiguration(
                "Max agent count must be greater than 0".to_string()
            ));
        }
        
        // Validate behavioral thresholds
        let behavioral_config = &config.behavioral_validation_config;
        if behavioral_config.misunderstanding_rate_min >= behavioral_config.misunderstanding_rate_max {
            return Err(ConfigurationError::InvalidConfiguration(
                "Misunderstanding rate min must be less than max".to_string()
            ));
        }
        
        // Validate AI pattern rules
        let ai_rules = &config.validation_rules.ai_patterns;
        let personality_config = &ai_rules.personality_trait_validation;
        if personality_config.range_min >= personality_config.range_max {
            return Err(ConfigurationError::InvalidConfiguration(
                "Personality trait range min must be less than max".to_string()
            ));
        }
        
        let emotional_config = &ai_rules.emotional_state_validation;
        if emotional_config.bipolar_range_min >= emotional_config.bipolar_range_max {
            return Err(ConfigurationError::InvalidConfiguration(
                "Emotional state range min must be less than max".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Creates a default configuration file.
    async fn create_default_config_file(&self) -> Result<(), ConfigurationError> {
        let default_config = Self::default_configuration();
        let config_content = toml::to_string_pretty(&default_config)
            .map_err(|e| ConfigurationError::SerializationError(e.to_string()))?;
        
        // Ensure parent directory exists
        if let Some(parent) = self.config_file_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| ConfigurationError::FileWriteError(e.to_string()))?;
        }
        
        tokio::fs::write(&self.config_file_path, config_content).await
            .map_err(|e| ConfigurationError::FileWriteError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Returns default configuration.
    fn default_configuration() -> ComprehensiveCiCdConfiguration {
        ComprehensiveCiCdConfiguration {
            pre_commit_config: PreCommitConfiguration {
                enabled: true,
                timeout_seconds: 45,
                enabled_checks: vec![
                    PreCommitCheckType::CodeFormatting,
                    PreCommitCheckType::EssentialLinting,
                    PreCommitCheckType::UnitTestsChangedModules,
                    PreCommitCheckType::DocumentationValidationChanged,
                ],
                bypass_conditions: vec![BypassCondition::CommitMessageContains("BYPASS_PRECOMMIT".to_string())],
                parallel_execution: true,
                progress_reporting: true,
                fail_fast: true,
                formatting_config: FormattingConfiguration {
                    rust_fmt_enabled: true,
                    custom_fmt_rules: HashMap::new(),
                },
                linting_config: LintingConfiguration {
                    clippy_enabled: true,
                    clippy_lints: vec!["warnings".to_string()],
                    custom_lints: vec![],
                },
                testing_config: TestingConfiguration {
                    unit_tests_enabled: true,
                    integration_tests_enabled: false,
                    behavioral_tests_enabled: false,
                    test_timeout_seconds: 30,
                },
            },
            pre_push_config: PrePushConfiguration {
                enabled: true,
                timeout_seconds: 600,
                comprehensive_validation: true,
                cross_platform_builds: true,
                security_audit_enabled: true,
                performance_regression_check: true,
                behavioral_consistency_check: true,
                bypass_conditions: vec![BypassCondition::CommitMessageContains("BYPASS_PREPUSH".to_string())],
                validation_order: vec![
                    ValidationType::CodeFormatting,
                    ValidationType::AiPatternValidation,
                    ValidationType::EcsArchitectureValidation,
                    ValidationType::PerformancePatternValidation,
                    ValidationType::UnitTesting,
                    ValidationType::IntegrationTesting,
                    ValidationType::BehavioralConsistencyValidation,
                    ValidationType::PerformanceRegression,
                    ValidationType::SecurityAudit,
                    ValidationType::CrossPlatformBuild,
                    ValidationType::DocumentationValidation,
                ],
                parallel_validation: true,
                quality_gates: QualityGateConfiguration {
                    max_critical_issues: 0,
                    max_high_issues: 5,
                    max_medium_issues: 20,
                    required_test_coverage: 0.8,
                    max_performance_regression: 0.05,
                    behavioral_consistency_threshold: 0.9,
                },
            },
            ci_pipeline_config: Self::default_pipeline_configuration(),
            validation_rules: Self::default_validation_rules(),
            git_hooks_config: Self::default_git_hooks_configuration(),
            performance_targets: PerformanceTargetsConfiguration {
                target_fps: 60.0,
                max_agent_count: 100,
                memory_usage_limit_mb: 100,
                frame_time_budget_ms: 16.67,
                ai_systems_budget_percentage: 50.0,
                regression_tolerance_percentage: 5.0,
            },
            behavioral_validation_config: BehavioralValidationConfiguration {
                personality_difference_threshold: 0.1,
                misunderstanding_rate_min: 0.2,
                misunderstanding_rate_max: 0.4,
                emotional_contagion_speed_threshold: 0.5,
                decision_optimality_threshold: 0.7,
            },
            security_config: SecurityConfiguration {
                cargo_audit_enabled: true,
                warning_only_mode: true,
                license_compatibility_check: true,
                dependency_bloat_detection: true,
                vulnerability_severity_threshold: "medium".to_string(),
            },
            documentation_config: DocumentationConfiguration {
                require_public_api_docs: true,
                require_behavioral_purpose_docs: true,
                validate_doc_examples: true,
                generate_documentation_build: true,
            },
            cross_platform_config: CrossPlatformConfiguration {
                target_platforms: vec![
                    "x86_64-pc-windows-msvc".to_string(),
                    "x86_64-unknown-linux-gnu".to_string(),
                    "x86_64-apple-darwin".to_string(),
                ],
                test_release_builds: true,
                test_debug_builds: true,
                validate_conditional_compilation: true,
            },
        }
    }
    
    /// Returns default pipeline configuration.
    fn default_pipeline_configuration() -> PipelineCiCdConfiguration {
        PipelineCiCdConfiguration {
            stages: vec![],
            parallel_execution: ParallelExecutionConfig {
                max_concurrent_stages: 4,
                max_concurrent_validations: 8,
                enable_stage_parallelism: true,
                enable_validation_parallelism: true,
            },
            timeout_config: TimeoutConfiguration {
                stage_timeout_seconds: HashMap::new(),
                validation_timeout_seconds: HashMap::new(),
                global_timeout_seconds: 3600,
            },
            notification_config: NotificationConfiguration {
                enable_progress_notifications: true,
                enable_completion_notifications: true,
                enable_failure_notifications: true,
                notification_channels: vec![],
            },
            quality_gates: QualityGateConfiguration {
                max_critical_issues: 0,
                max_high_issues: 5,
                max_medium_issues: 20,
                required_test_coverage: 0.8,
                max_performance_regression: 0.05,
                behavioral_consistency_threshold: 0.9,
            },
            reporting_config: ReportingConfiguration {
                generate_detailed_reports: true,
                generate_summary_dashboard: true,
                report_output_directory: PathBuf::from("target/ci-reports"),
                include_metrics_trends: true,
                include_fix_suggestions: true,
            },
        }
    }
    
    /// Returns default validation rules.
    fn default_validation_rules() -> ValidationRulesConfiguration {
        ValidationRulesConfiguration {
            ai_patterns: AiPatternValidationRules {
                personality_trait_validation: PersonalityTraitValidationConfig {
                    enforce_normalized_type: true,
                    require_range_documentation: true,
                    valid_trait_names: vec![
                        "openness".to_string(),
                        "conscientiousness".to_string(),
                        "extraversion".to_string(),
                        "agreeableness".to_string(),
                        "neuroticism".to_string(),
                    ],
                    range_min: 0.0,
                    range_max: 1.0,
                },
                emotional_state_validation: EmotionalStateValidationConfig {
                    enforce_bipolar_range: true,
                    require_validation_methods: true,
                    valid_emotional_dimensions: vec![
                        "valence".to_string(),
                        "arousal".to_string(),
                        "dominance".to_string(),
                    ],
                    bipolar_range_min: -1.0,
                    bipolar_range_max: 1.0,
                },
                temporal_consistency_validation: TemporalConsistencyValidationConfig {
                    enforce_world_time_usage: true,
                    detect_real_time_usage: true,
                    allowed_real_time_contexts: vec!["logging".to_string(), "profiling".to_string()],
                },
                system_modulation_validation: SystemModulationValidationConfig {
                    require_personality_modulation: true,
                    detect_unmodulated_systems: true,
                    allowed_unmodulated_systems: vec!["debug".to_string(), "profiling".to_string()],
                },
                strict_mode: true,
                custom_patterns: vec![],
            },
            ecs_architecture: EcsArchitectureValidationRules {
                component_validation: ComponentValidationConfig {
                    enforce_pure_data: true,
                    detect_behavior_methods: true,
                    allowed_method_patterns: vec![
                        "get_".to_string(),
                        "is_".to_string(),
                        "has_".to_string(),
                        "validate".to_string(),
                        "new".to_string(),
                        "default".to_string(),
                    ],
                    validate_field_types: true,
                },
                system_validation: SystemValidationConfig {
                    enforce_query_usage: true,
                    detect_direct_component_access: true,
                    require_event_communication: true,
                    validate_system_signatures: true,
                },
                domain_separation_validation: DomainSeparationValidationConfig {
                    enforce_domain_boundaries: true,
                    allowed_domains: vec![
                        "ai".to_string(),
                        "world".to_string(),
                        "presentation".to_string(),
                        "core".to_string(),
                    ],
                    cross_domain_communication_rules: vec![],
                    validate_file_organization: true,
                },
                plugin_validation: PluginValidationConfig {
                    enforce_single_plugin_per_domain: true,
                    validate_plugin_registration: true,
                    required_plugin_methods: vec!["build".to_string()],
                },
                strict_mode: true,
            },
            performance_patterns: PerformancePatternValidationRules {
                f64_usage_detection: F64UsageDetectionConfig {
                    enabled: true,
                    detect_in_hot_paths: true,
                    allowed_contexts: vec!["test".to_string(), "benchmark".to_string()],
                    hot_path_patterns: vec![
                        "fn update".to_string(),
                        "fn run".to_string(),
                        "Query<".to_string(),
                    ],
                },
                sync_io_detection: SyncIoDetectionConfig {
                    enabled: true,
                    detect_in_systems: true,
                    allowed_sync_contexts: vec!["initialization".to_string(), "shutdown".to_string()],
                    sync_io_patterns: vec![
                        "std::fs::".to_string(),
                        "File::".to_string(),
                        "std::net::".to_string(),
                    ],
                },
                memory_allocation_detection: MemoryAllocationDetectionConfig {
                    enabled: true,
                    detect_excessive_allocations: true,
                    allocation_threshold: 1000,
                    hot_path_allocation_detection: true,
                },
                query_pattern_validation: QueryPatternValidationConfig {
                    enabled: true,
                    detect_inefficient_queries: true,
                    validate_query_filters: true,
                    max_query_complexity: 10,
                },
                performance_thresholds: PerformanceThresholds {
                    max_frame_time_ms: 16.67,
                    max_memory_usage_mb: 100,
                    max_cpu_usage_percentage: 80.0,
                },
            },
            code_quality: CodeQualityValidationRules {
                documentation_requirements: DocumentationRequirements {
                    min_doc_coverage_percentage: 80.0,
                    require_examples: true,
                    require_behavioral_descriptions: true,
                },
                complexity_thresholds: ComplexityThresholds {
                    max_cyclomatic_complexity: 10,
                    max_function_length: 50,
                    max_parameter_count: 5,
                },
                duplication_detection: DuplicationDetectionConfig {
                    enabled: true,
                    min_duplicate_lines: 3,
                    ignore_patterns: vec!["test".to_string()],
                },
                naming_conventions: NamingConventionsConfig {
                    enforce_rust_conventions: true,
                    ai_specific_patterns: vec![
                        "personality_".to_string(),
                        "emotional_".to_string(),
                        "behavioral_".to_string(),
                    ],
                    behavioral_naming_required: true,
                },
            },
            behavioral_consistency: BehavioralConsistencyValidationRules {
                personality_consistency: PersonalityConsistencyConfig {
                    min_observable_difference: 0.1,
                    consistency_check_scenarios: vec![
                        "social_interaction".to_string(),
                        "decision_making".to_string(),
                        "stress_response".to_string(),
                    ],
                },
                social_dynamics_validation: SocialDynamicsValidationConfig {
                    misunderstanding_rate_validation: true,
                    emotional_contagion_validation: true,
                    communication_loss_validation: true,
                },
                decision_system_validation: DecisionSystemValidationConfig {
                    emotional_logic_validation: true,
                    optimality_avoidance_validation: true,
                    feel_over_science_compliance: true,
                },
                believability_thresholds: BelievabilityThresholds {
                    min_personality_impact: 0.1,
                    max_decision_optimality: 0.7,
                    min_emotional_influence: 0.05,
                },
            },
        }
    }
    
    /// Returns default Git hooks configuration.
    fn default_git_hooks_configuration() -> GitHooksConfiguration {
        GitHooksConfiguration {
            auto_install: true,
            hooks_directory: PathBuf::from(".git/hooks"),
            pre_commit_hook: HookConfiguration {
                script_path: PathBuf::from(".git/hooks/pre-commit"),
                timeout_seconds: 45,
                required_tools: vec!["cargo".to_string()],
                environment_variables: HashMap::new(),
                bypass_conditions: vec![BypassCondition::CommitMessageContains("BYPASS_PRECOMMIT".to_string())],
            },
            pre_push_hook: HookConfiguration {
                script_path: PathBuf::from(".git/hooks/pre-push"),
                timeout_seconds: 600,
                required_tools: vec!["cargo".to_string()],
                environment_variables: HashMap::new(),
                bypass_conditions: vec![BypassCondition::CommitMessageContains("BYPASS_PREPUSH".to_string())],
            },
            bypass_config: BypassConfiguration {
                emergency_bypass_enabled: true,
                bypass_keywords: vec![
                    "BYPASS_PRECOMMIT".to_string(),
                    "BYPASS_PREPUSH".to_string(),
                    "EMERGENCY".to_string(),
                ],
                bypass_environment_variable: Some("EMERGENCY_BYPASS".to_string()),
            },
            hook_templates: HashMap::new(),
        }
    }
}

/// Errors that can occur during configuration management.
#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("File read error: {0}")]
    FileReadError(String),
    
    #[error("File write error: {0}")]
    FileWriteError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// Removed ValidationError conversion as it's not needed