pub mod ai_patterns;
pub mod ecs_architecture;
pub mod performance_patterns;
pub mod behavioral_consistency;
pub mod performance_regression;
pub mod security_audit;
pub mod cross_platform_builds;
pub mod documentation_quality;
pub mod error_handling;
pub mod fix_commands;
pub mod orchestrator;

// Re-export main validators
pub use ai_patterns::AiPatternValidator;
pub use ecs_architecture::BevyEcsValidator;
pub use performance_patterns::PerformancePatternDetector;
pub use behavioral_consistency::BehavioralConsistencyValidator;
pub use performance_regression::PerformanceRegressionDetector;
pub use security_audit::SecurityAuditSystem;
pub use cross_platform_builds::CrossPlatformBuildValidator;
pub use documentation_quality::DocumentationQualityValidator;
pub use error_handling::ValidationErrorHandler;
pub use fix_commands::FixCommandGenerator;
pub use orchestrator::ValidationOrchestrator;

// Re-export common types from ai_patterns (first module)
pub use ai_patterns::{ValidationResult, ValidationIssue, IssueType};
