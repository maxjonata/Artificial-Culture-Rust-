#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_project() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        
        // Create a basic Cargo.toml
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"
chrono = "0.4"
"#;
        
        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
        
        // Create src directory and main.rs
        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::write(
            temp_dir.path().join("src").join("main.rs"),
            "fn main() { println!(\"Hello, world!\"); }"
        ).unwrap();
        
        temp_dir
    }

    #[test]
    fn test_security_audit_system_creation() {
        let config = SecurityAuditConfig::default();
        let audit_system = SecurityAuditSystem::new(config);
        
        assert!(audit_system.audit_config.warning_only_mode);
        assert!(audit_system.audit_config.allowed_licenses.contains("MIT"));
        assert!(audit_system.audit_config.blocked_licenses.contains("GPL-3.0"));
    }

    #[test]
    fn test_dependency_bloat_detector() {
        let detector = DependencyBloatDetector::new();
        
        // Test known bloated dependency detection
        assert!(detector.alternative_suggestions.contains_key("reqwest"));
        assert!(detector.alternative_suggestions.contains_key("tokio"));
        assert!(detector.alternative_suggestions.contains_key("clap"));
        
        // Test feature optimization rules
        assert!(!detector.feature_optimization_rules.is_empty());
        let serde_rule = detector.feature_optimization_rules.iter()
            .find(|r| r.crate_name == "serde");
        assert!(serde_rule.is_some());
        assert!(serde_rule.unwrap().default_features_bloat);
    }

    #[test]
    fn test_bloat_analysis() {
        let detector = DependencyBloatDetector::new();
        
        // Create mock package metadata for reqwest
        let package_json = serde_json::json!({
            "name": "reqwest",
            "version": "0.11.0",
            "source": "registry+https://github.com/rust-lang/crates.io-index"
        });
        
        let issues = detector.analyze_dependency_bloat("reqwest", &package_json);
        
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| matches!(i.issue_type, DependencyIssueType::Bloated)));
        
        // Check that alternatives are suggested
        let bloat_issue = issues.iter().find(|i| matches!(i.issue_type, DependencyIssueType::Bloated)).unwrap();
        assert!(bloat_issue.recommendation.contains("ureq"));
        assert!(bloat_issue.recommendation.contains("surf"));
    }

    #[test]
    fn test_license_checker() {
        let config = SecurityAuditConfig::default();
        let checker = LicenseChecker::new(config);
        
        // Test blocked license detection
        let issue = checker.check_license_compatibility("test-package", "1.0.0", "GPL-3.0");
        assert!(issue.is_some());
        assert!(matches!(issue.unwrap().issue_type, LicenseIssueType::Blocked));
        
        // Test allowed license
        let issue = checker.check_license_compatibility("test-package", "1.0.0", "MIT");
        assert!(issue.is_none());
        
        // Test unknown license
        let issue = checker.check_license_compatibility("test-package", "1.0.0", "UNKNOWN-LICENSE");
        assert!(issue.is_some());
        assert!(matches!(issue.unwrap().issue_type, LicenseIssueType::Unknown));
    }

    #[test]
    fn test_vulnerability_severity_parsing() {
        let scanner = VulnerabilityScanner::new(SecurityAuditConfig::default());
        
        assert!(matches!(scanner.parse_severity("critical"), VulnerabilitySeverity::Critical));
        assert!(matches!(scanner.parse_severity("high"), VulnerabilitySeverity::High));
        assert!(matches!(scanner.parse_severity("medium"), VulnerabilitySeverity::Medium));
        assert!(matches!(scanner.parse_severity("low"), VulnerabilitySeverity::Low));
        assert!(matches!(scanner.parse_severity("unknown"), VulnerabilitySeverity::Medium));
    }

    #[test]
    fn test_upgrade_guidance_generation() {
        let detector = DependencyBloatDetector::new();
        
        let vulnerability = Vulnerability {
            id: "RUSTSEC-2023-0001".to_string(),
            package: "test-crate".to_string(),
            version: "1.0.0".to_string(),
            severity: VulnerabilitySeverity::High,
            title: "Test Vulnerability".to_string(),
            description: "A test vulnerability for demonstration".to_string(),
            patched_versions: vec!["1.0.1".to_string(), "1.1.0".to_string()],
            unaffected_versions: vec![],
            url: Some("https://rustsec.org/advisories/RUSTSEC-2023-0001".to_string()),
            date: Some("2023-01-01".to_string()),
        };
        
        let guidance = detector.generate_upgrade_guidance(&vulnerability);
        
        assert!(guidance.contains("Security Vulnerability"));
        assert!(guidance.contains("test-crate"));
        assert!(guidance.contains("High"));
        assert!(guidance.contains("cargo update -p test-crate"));
        assert!(guidance.contains("1.0.1"));
        assert!(guidance.contains("rustsec.org"));
    }

    #[test]
    fn test_security_audit_result_formatting() {
        let result = SecurityAuditResult {
            vulnerabilities: vec![
                Vulnerability {
                    id: "RUSTSEC-2023-0001".to_string(),
                    package: "test-crate".to_string(),
                    version: "1.0.0".to_string(),
                    severity: VulnerabilitySeverity::High,
                    title: "Test Vulnerability".to_string(),
                    description: "A test vulnerability".to_string(),
                    patched_versions: vec!["1.0.1".to_string()],
                    unaffected_versions: vec![],
                    url: None,
                    date: None,
                }
            ],
            license_issues: vec![],
            dependency_issues: vec![],
            passed: false,
            warnings_only: true,
            execution_time: Duration::from_secs(5),
            recommendations: vec![
                SecurityRecommendation {
                    recommendation_type: RecommendationType::UpdateDependency,
                    package: Some("test-crate".to_string()),
                    current_version: Some("1.0.0".to_string()),
                    recommended_version: Some("1.0.1".to_string()),
                    description: "Update test-crate to fix vulnerability".to_string(),
                    urgency: RecommendationUrgency::High,
                    fix_command: Some("cargo update -p test-crate".to_string()),
                }
            ],
        };
        
        let report = result.format_report();
        
        assert!(report.contains("Security Audit Report"));
        assert!(report.contains("WARNING-ONLY mode"));
        assert!(report.contains("Vulnerabilities Found: 1"));
        assert!(report.contains("test-crate"));
        assert!(report.contains("High"));
        assert!(report.contains("Recommendations:"));
        assert!(report.contains("🟠")); // High urgency icon
        assert!(report.contains("cargo update -p test-crate"));
        assert!(report.contains("Execution time: 5.00s"));
        assert!(report.contains("❌ FAILED"));
    }

    #[test]
    fn test_dependency_extraction_from_toml() {
        let analyzer = DependencyAnalyzer::new(SecurityAuditConfig::default());
        
        let toml_content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"

[dev-dependencies]
tempfile = "3.0"
"#;
        
        let dependencies = analyzer.extract_dependencies_from_toml(toml_content);
        
        assert!(dependencies.contains(&"serde".to_string()));
        assert!(dependencies.contains(&"tokio".to_string()));
        assert!(dependencies.contains(&"reqwest".to_string()));
        assert!(!dependencies.contains(&"tempfile".to_string())); // dev-dependency should not be included
    }

    #[test]
    fn test_alternative_suggestion_formatting() {
        let detector = DependencyBloatDetector::new();
        
        let alternatives = vec![
            AlternativeSuggestion {
                name: "ureq".to_string(),
                description: "Minimal HTTP client".to_string(),
                pros: vec!["Small size".to_string(), "Fast compile".to_string()],
                cons: vec!["Sync only".to_string()],
                migration_effort: MigrationEffort::Easy,
                performance_impact: PerformanceImpact::Improvement("2MB smaller".to_string()),
            }
        ];
        
        let formatted = detector.format_alternative_recommendations(&alternatives);
        
        assert!(formatted.contains("Consider these alternatives:"));
        assert!(formatted.contains("ureq"));
        assert!(formatted.contains("Minimal HTTP client"));
        assert!(formatted.contains("Easy"));
        assert!(formatted.contains("2MB smaller"));
        assert!(formatted.contains("Pros: Small size, Fast compile"));
        assert!(formatted.contains("Cons: Sync only"));
    }
}