use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Security audit integration for vulnerability detection and dependency management.
#[derive(Debug)]
pub struct SecurityAuditSystem {
    pub vulnerability_scanner: VulnerabilityScanner,
    pub license_checker: LicenseChecker,
    pub dependency_analyzer: DependencyAnalyzer,
    pub audit_config: SecurityAuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditConfig {
    pub cargo_audit_timeout: Duration,
    pub warning_only_mode: bool,
    pub allowed_licenses: HashSet<String>,
    pub blocked_licenses: HashSet<String>,
    pub dependency_update_validation: bool,
    pub critical_vulnerability_threshold: VulnerabilitySeverity,
}

impl Default for SecurityAuditConfig {
    fn default() -> Self {
        let mut allowed_licenses = HashSet::new();
        allowed_licenses.insert("MIT".to_string());
        allowed_licenses.insert("Apache-2.0".to_string());
        allowed_licenses.insert("BSD-3-Clause".to_string());
        allowed_licenses.insert("ISC".to_string());
        allowed_licenses.insert("Unlicense".to_string());
        
        let mut blocked_licenses = HashSet::new();
        blocked_licenses.insert("GPL-3.0".to_string());
        blocked_licenses.insert("AGPL-3.0".to_string());
        blocked_licenses.insert("LGPL-3.0".to_string());
        
        Self {
            cargo_audit_timeout: Duration::from_secs(60),
            warning_only_mode: true,
            allowed_licenses,
            blocked_licenses,
            dependency_update_validation: true,
            critical_vulnerability_threshold: VulnerabilitySeverity::High,
        }
    }
}

/// Scans for security vulnerabilities using cargo audit.
#[derive(Debug)]
pub struct VulnerabilityScanner {
    pub config: SecurityAuditConfig,
    pub audit_database_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditResult {
    pub vulnerabilities: Vec<Vulnerability>,
    pub license_issues: Vec<LicenseIssue>,
    pub dependency_issues: Vec<DependencyIssue>,
    pub passed: bool,
    pub warnings_only: bool,
    pub execution_time: Duration,
    pub recommendations: Vec<SecurityRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub package: String,
    pub version: String,
    pub severity: VulnerabilitySeverity,
    pub title: String,
    pub description: String,
    pub patched_versions: Vec<String>,
    pub unaffected_versions: Vec<String>,
    pub url: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseIssue {
    pub package: String,
    pub version: String,
    pub license: String,
    pub issue_type: LicenseIssueType,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseIssueType {
    Blocked,
    Unknown,
    Incompatible,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyIssue {
    pub package: String,
    pub version: String,
    pub issue_type: DependencyIssueType,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyIssueType {
    Outdated,
    Yanked,
    Unmaintained,
    Bloated,
    Duplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub recommendation_type: RecommendationType,
    pub package: Option<String>,
    pub current_version: Option<String>,
    pub recommended_version: Option<String>,
    pub description: String,
    pub urgency: RecommendationUrgency,
    pub fix_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    UpdateDependency,
    RemoveDependency,
    ChangeLicense,
    AddSecurityPatch,
    ReviewUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationUrgency {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatThresholds {
    pub compile_time_seconds: f32,
    pub binary_size_mb: f32,
    pub dependency_count: usize,
    pub transitive_dependency_count: usize,
}

impl Default for BloatThresholds {
    fn default() -> Self {
        Self {
            compile_time_seconds: 30.0,
            binary_size_mb: 10.0,
            dependency_count: 20,
            transitive_dependency_count: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeSuggestion {
    pub name: String,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub migration_effort: MigrationEffort,
    pub performance_impact: PerformanceImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationEffort {
    Trivial,    // Drop-in replacement
    Easy,       // Minor API changes
    Moderate,   // Some refactoring needed
    Difficult,  // Significant changes required
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceImpact {
    Improvement(String),
    Neutral,
    Degradation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureOptimizationRule {
    pub crate_name: String,
    pub default_features_bloat: bool,
    pub recommended_features: Vec<String>,
    pub features_to_avoid: Vec<String>,
    pub size_reduction_estimate: String,
}

impl SecurityAuditSystem {
    pub fn new(config: SecurityAuditConfig) -> Self {
        Self {
            vulnerability_scanner: VulnerabilityScanner::new(config.clone()),
            license_checker: LicenseChecker::new(config.clone()),
            dependency_analyzer: DependencyAnalyzer::new(config.clone()),
            audit_config: config,
        }
    }
    
    /// Runs complete security audit including vulnerabilities, licenses, and dependencies.
    pub fn run_security_audit(&self, project_path: &Path) -> Result<SecurityAuditResult, SecurityAuditError> {
        let start_time = std::time::Instant::now();
        
        // Run vulnerability scan
        let vulnerabilities = self.vulnerability_scanner.scan_vulnerabilities(project_path)?;
        
        // Check license compatibility
        let license_issues = self.license_checker.check_licenses(project_path)?;
        
        // Analyze dependencies
        let dependency_issues = self.dependency_analyzer.analyze_dependencies(project_path)?;
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&vulnerabilities, &license_issues, &dependency_issues);
        
        // Determine if audit passed (in warning-only mode, always passes)
        let passed = self.audit_config.warning_only_mode || 
                    self.evaluate_audit_success(&vulnerabilities, &license_issues, &dependency_issues);
        
        Ok(SecurityAuditResult {
            vulnerabilities,
            license_issues,
            dependency_issues,
            passed,
            warnings_only: self.audit_config.warning_only_mode,
            execution_time: start_time.elapsed(),
            recommendations,
        })
    }
    
    /// Validates dependency updates by running full test suite.
    pub fn validate_dependency_update(&self, project_path: &Path) -> Result<DependencyUpdateResult, SecurityAuditError> {
        if !self.audit_config.dependency_update_validation {
            return Ok(DependencyUpdateResult {
                passed: true,
                test_results: Vec::new(),
                build_success: true,
                warnings: Vec::new(),
            });
        }
        
        let start_time = std::time::Instant::now();
        
        // Run cargo build to ensure compilation
        let build_result = Command::new("cargo")
            .args(&["build", "--workspace"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo build failed: {}", e)))?;
        
        let build_success = build_result.status.success();
        
        // Run full test suite
        let test_result = Command::new("cargo")
            .args(&["test", "--workspace"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo test failed: {}", e)))?;
        
        let test_success = test_result.status.success();
        
        let mut warnings = Vec::new();
        if !build_success {
            warnings.push("Build failed after dependency update".to_string());
        }
        if !test_success {
            warnings.push("Tests failed after dependency update".to_string());
        }
        
        Ok(DependencyUpdateResult {
            passed: build_success && test_success,
            test_results: vec![TestResult {
                name: "full_test_suite".to_string(),
                passed: test_success,
                duration: start_time.elapsed(),
                output: String::from_utf8_lossy(&test_result.stdout).to_string(),
            }],
            build_success,
            warnings,
        })
    }
    
    fn generate_recommendations(
        &self,
        vulnerabilities: &[Vulnerability],
        license_issues: &[LicenseIssue],
        dependency_issues: &[DependencyIssue],
    ) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();
        
        // Generate vulnerability recommendations
        for vuln in vulnerabilities {
            if !vuln.patched_versions.is_empty() {
                recommendations.push(SecurityRecommendation {
                    recommendation_type: RecommendationType::UpdateDependency,
                    package: Some(vuln.package.clone()),
                    current_version: Some(vuln.version.clone()),
                    recommended_version: vuln.patched_versions.first().cloned(),
                    description: format!(
                        "Update {} to fix vulnerability: {}",
                        vuln.package, vuln.title
                    ),
                    urgency: match vuln.severity {
                        VulnerabilitySeverity::Critical => RecommendationUrgency::Critical,
                        VulnerabilitySeverity::High => RecommendationUrgency::High,
                        VulnerabilitySeverity::Medium => RecommendationUrgency::Medium,
                        VulnerabilitySeverity::Low => RecommendationUrgency::Low,
                    },
                    fix_command: Some(format!("cargo update -p {}", vuln.package)),
                });
            }
        }
        
        // Generate license recommendations
        for license_issue in license_issues {
            recommendations.push(SecurityRecommendation {
                recommendation_type: RecommendationType::ChangeLicense,
                package: Some(license_issue.package.clone()),
                current_version: Some(license_issue.version.clone()),
                recommended_version: None,
                description: license_issue.recommendation.clone(),
                urgency: match license_issue.issue_type {
                    LicenseIssueType::Blocked => RecommendationUrgency::High,
                    LicenseIssueType::Incompatible => RecommendationUrgency::Medium,
                    LicenseIssueType::Unknown => RecommendationUrgency::Low,
                    LicenseIssueType::Missing => RecommendationUrgency::Medium,
                },
                fix_command: None,
            });
        }
        
        // Generate dependency recommendations
        for dep_issue in dependency_issues {
            let (rec_type, urgency) = match dep_issue.issue_type {
                DependencyIssueType::Outdated => (RecommendationType::UpdateDependency, RecommendationUrgency::Medium),
                DependencyIssueType::Yanked => (RecommendationType::UpdateDependency, RecommendationUrgency::High),
                DependencyIssueType::Unmaintained => (RecommendationType::ReviewUsage, RecommendationUrgency::Medium),
                DependencyIssueType::Bloated => (RecommendationType::RemoveDependency, RecommendationUrgency::Low),
                DependencyIssueType::Duplicate => (RecommendationType::RemoveDependency, RecommendationUrgency::Medium),
            };
            
            recommendations.push(SecurityRecommendation {
                recommendation_type: rec_type,
                package: Some(dep_issue.package.clone()),
                current_version: Some(dep_issue.version.clone()),
                recommended_version: None,
                description: dep_issue.recommendation.clone(),
                urgency,
                fix_command: match dep_issue.issue_type {
                    DependencyIssueType::Outdated => Some(format!("cargo update -p {}", dep_issue.package)),
                    _ => None,
                },
            });
        }
        
        recommendations
    }
    
    fn evaluate_audit_success(
        &self,
        vulnerabilities: &[Vulnerability],
        license_issues: &[LicenseIssue],
        dependency_issues: &[DependencyIssue],
    ) -> bool {
        // Check for critical vulnerabilities
        let has_critical_vulns = vulnerabilities.iter()
            .any(|v| v.severity >= self.audit_config.critical_vulnerability_threshold);
        
        // Check for blocked licenses
        let has_blocked_licenses = license_issues.iter()
            .any(|l| matches!(l.issue_type, LicenseIssueType::Blocked));
        
        // Check for yanked dependencies
        let has_yanked_deps = dependency_issues.iter()
            .any(|d| matches!(d.issue_type, DependencyIssueType::Yanked));
        
        !has_critical_vulns && !has_blocked_licenses && !has_yanked_deps
    }
}

impl VulnerabilityScanner {
    pub fn new(config: SecurityAuditConfig) -> Self {
        Self {
            config,
            audit_database_path: None,
        }
    }
    
    pub fn scan_vulnerabilities(&self, project_path: &Path) -> Result<Vec<Vulnerability>, SecurityAuditError> {
        // Ensure cargo-audit is installed
        self.ensure_cargo_audit_installed()?;
        
        // Run cargo audit
        let output = Command::new("cargo")
            .args(&["audit", "--format", "json"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo audit failed: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // In warning-only mode, don't fail on audit errors
            if self.config.warning_only_mode {
                eprintln!("Warning: cargo audit failed: {}", stderr);
                return Ok(Vec::new());
            } else {
                return Err(SecurityAuditError::AuditFailed(stderr.to_string()));
            }
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_audit_output(&stdout)
    }
    
    fn ensure_cargo_audit_installed(&self) -> Result<(), SecurityAuditError> {
        let output = Command::new("cargo")
            .args(&["audit", "--version"])
            .output()
            .map_err(|_| SecurityAuditError::ToolNotInstalled("cargo-audit".to_string()))?;
        
        if !output.status.success() {
            return Err(SecurityAuditError::ToolNotInstalled("cargo-audit".to_string()));
        }
        
        Ok(())
    }
    
    fn parse_audit_output(&self, output: &str) -> Result<Vec<Vulnerability>, SecurityAuditError> {
        let mut vulnerabilities = Vec::new();
        
        // Parse JSON output from cargo audit
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(vuln) = self.parse_vulnerability_json(&json) {
                        vulnerabilities.push(vuln);
                    }
                }
                Err(_) => {
                    // Skip non-JSON lines (warnings, etc.)
                    continue;
                }
            }
        }
        
        Ok(vulnerabilities)
    }
    
    fn parse_vulnerability_json(&self, json: &serde_json::Value) -> Option<Vulnerability> {
        let advisory = json.get("advisory")?;
        
        Some(Vulnerability {
            id: advisory.get("id")?.as_str()?.to_string(),
            package: json.get("package")?.as_str()?.to_string(),
            version: json.get("version")?.as_str()?.to_string(),
            severity: self.parse_severity(advisory.get("severity")?.as_str()?),
            title: advisory.get("title")?.as_str()?.to_string(),
            description: advisory.get("description")?.as_str()?.to_string(),
            patched_versions: advisory.get("patched_versions")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            unaffected_versions: advisory.get("unaffected_versions")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            url: advisory.get("url").and_then(|v| v.as_str()).map(String::from),
            date: advisory.get("date").and_then(|v| v.as_str()).map(String::from),
        })
    }
    
    fn parse_severity(&self, severity: &str) -> VulnerabilitySeverity {
        match severity.to_lowercase().as_str() {
            "critical" => VulnerabilitySeverity::Critical,
            "high" => VulnerabilitySeverity::High,
            "medium" => VulnerabilitySeverity::Medium,
            "low" => VulnerabilitySeverity::Low,
            _ => VulnerabilitySeverity::Medium,
        }
    }
}

/// Checks license compatibility for all dependencies.
#[derive(Debug)]
pub struct LicenseChecker {
    pub config: SecurityAuditConfig,
}

impl LicenseChecker {
    pub fn new(config: SecurityAuditConfig) -> Self {
        Self { config }
    }
    
    pub fn check_licenses(&self, project_path: &Path) -> Result<Vec<LicenseIssue>, SecurityAuditError> {
        // Get dependency tree with license information
        let output = Command::new("cargo")
            .args(&["tree", "--format", "{p} {l}"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo tree failed: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SecurityAuditError::CommandFailed(format!("cargo tree failed: {}", stderr)));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_license_output(&stdout)
    }
    
    fn parse_license_output(&self, output: &str) -> Result<Vec<LicenseIssue>, SecurityAuditError> {
        let mut license_issues = Vec::new();
        
        for line in output.lines() {
            if let Some((package_version, license)) = self.parse_license_line(line) {
                if let Some((package, version)) = package_version.split_once(' ') {
                    if let Some(issue) = self.check_license_compatibility(package, version, &license) {
                        license_issues.push(issue);
                    }
                }
            }
        }
        
        Ok(license_issues)
    }
    
    fn parse_license_line(&self, line: &str) -> Option<(String, String)> {
        // Parse format: "package version license"
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 3 {
            let package_version = format!("{} {}", parts[0], parts[1]);
            let license = parts[2..].join(" ");
            Some((package_version, license))
        } else {
            None
        }
    }
    
    fn check_license_compatibility(&self, package: &str, version: &str, license: &str) -> Option<LicenseIssue> {
        if license.is_empty() || license == "N/A" {
            return Some(LicenseIssue {
                package: package.to_string(),
                version: version.to_string(),
                license: license.to_string(),
                issue_type: LicenseIssueType::Missing,
                recommendation: format!("Package {} has no license information. Consider finding an alternative or contacting the maintainer.", package),
            });
        }
        
        // Check if license is blocked
        if self.config.blocked_licenses.contains(license) {
            return Some(LicenseIssue {
                package: package.to_string(),
                version: version.to_string(),
                license: license.to_string(),
                issue_type: LicenseIssueType::Blocked,
                recommendation: format!("License {} is blocked. Remove dependency {} or find an alternative with a compatible license.", license, package),
            });
        }
        
        // Check if license is allowed
        if !self.config.allowed_licenses.contains(license) {
            return Some(LicenseIssue {
                package: package.to_string(),
                version: version.to_string(),
                license: license.to_string(),
                issue_type: LicenseIssueType::Unknown,
                recommendation: format!("License {} is not in the allowed list. Review compatibility with project licensing.", license),
            });
        }
        
        None
    }
}

/// Analyzes dependencies for bloat, outdated versions, and other issues.
#[derive(Debug)]
pub struct DependencyAnalyzer {
    pub config: SecurityAuditConfig,
    pub bloat_detector: DependencyBloatDetector,
}

/// Specialized detector for dependency bloat and alternatives.
#[derive(Debug)]
pub struct DependencyBloatDetector {
    pub size_thresholds: BloatThresholds,
    pub alternative_suggestions: HashMap<String, Vec<AlternativeSuggestion>>,
    pub feature_optimization_rules: Vec<FeatureOptimizationRule>,
}

impl DependencyAnalyzer {
    pub fn new(config: SecurityAuditConfig) -> Self {
        Self { 
            config,
            bloat_detector: DependencyBloatDetector::new(),
        }
    }
    
    pub fn analyze_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        // Check for outdated dependencies
        issues.extend(self.check_outdated_dependencies(project_path)?);
        
        // Check for yanked crates
        issues.extend(self.check_yanked_crates(project_path)?);
        
        // Check for duplicate dependencies
        issues.extend(self.check_duplicate_dependencies(project_path)?);
        
        // Check for bloated dependencies
        issues.extend(self.check_bloated_dependencies(project_path)?);
        
        // Check for unnecessary dependencies
        issues.extend(self.check_unnecessary_dependencies(project_path)?);
        
        Ok(issues)
    }
    
    fn check_outdated_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let output = Command::new("cargo")
            .args(&["outdated", "--format", "json"])
            .current_dir(project_path)
            .output();
        
        // cargo-outdated might not be installed, so handle gracefully
        let output = match output {
            Ok(output) => output,
            Err(_) => {
                eprintln!("Warning: cargo-outdated not installed. Skipping outdated dependency check.");
                return Ok(Vec::new());
            }
        };
        
        if !output.status.success() {
            eprintln!("Warning: cargo outdated failed. Skipping outdated dependency check.");
            return Ok(Vec::new());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_outdated_output(&stdout)
    }
    
    fn parse_outdated_output(&self, output: &str) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        match serde_json::from_str::<serde_json::Value>(output) {
            Ok(json) => {
                if let Some(dependencies) = json.get("dependencies").and_then(|d| d.as_array()) {
                    for dep in dependencies {
                        if let Some(issue) = self.parse_outdated_dependency(dep) {
                            issues.push(issue);
                        }
                    }
                }
            }
            Err(_) => {
                // Fallback to text parsing if JSON parsing fails
                eprintln!("Warning: Failed to parse cargo outdated JSON output");
            }
        }
        
        Ok(issues)
    }
    
    fn parse_outdated_dependency(&self, dep: &serde_json::Value) -> Option<DependencyIssue> {
        let name = dep.get("name")?.as_str()?;
        let current = dep.get("project")?.as_str()?;
        let latest = dep.get("latest")?.as_str()?;
        
        if current != latest {
            Some(DependencyIssue {
                package: name.to_string(),
                version: current.to_string(),
                issue_type: DependencyIssueType::Outdated,
                description: format!("Dependency {} is outdated. Current: {}, Latest: {}", name, current, latest),
                recommendation: format!("Update {} to version {} using 'cargo update -p {}'", name, latest, name),
            })
        } else {
            None
        }
    }
    
    fn check_yanked_crates(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        // This would require checking crates.io API or using cargo metadata
        // For now, return empty as this is complex to implement
        Ok(Vec::new())
    }
    
    fn check_duplicate_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let output = Command::new("cargo")
            .args(&["tree", "--duplicates"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo tree --duplicates failed: {}", e)))?;
        
        if !output.status.success() {
            return Ok(Vec::new());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_duplicate_output(&stdout)
    }
    
    fn parse_duplicate_output(&self, output: &str) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        for line in output.lines() {
            if let Some((package, version)) = self.parse_duplicate_line(line) {
                issues.push(DependencyIssue {
                    package: package.clone(),
                    version: version.clone(),
                    issue_type: DependencyIssueType::Duplicate,
                    description: format!("Multiple versions of {} found in dependency tree", package),
                    recommendation: format!("Consider consolidating {} versions to reduce binary size", package),
                });
            }
        }
        
        Ok(issues)
    }
    
    fn parse_duplicate_line(&self, line: &str) -> Option<(String, String)> {
        // Parse cargo tree --duplicates output
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
    
    fn check_bloated_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        // Get dependency metadata
        let output = Command::new("cargo")
            .args(&["metadata", "--format-version", "1"])
            .current_dir(project_path)
            .output()
            .map_err(|e| SecurityAuditError::CommandFailed(format!("cargo metadata failed: {}", e)))?;
        
        if !output.status.success() {
            return Ok(Vec::new());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let metadata: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| SecurityAuditError::ParseError(format!("Failed to parse metadata: {}", e)))?;
        
        if let Some(packages) = metadata.get("packages").and_then(|p| p.as_array()) {
            for package in packages {
                if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
                    issues.extend(self.bloat_detector.analyze_dependency_bloat(name, package));
                }
                if let Some(issue) = self.analyze_package_bloat(package) {
                    issues.push(issue);
                }
            }
        }
        
        Ok(issues)
    }
    
    fn analyze_package_bloat(&self, package: &serde_json::Value) -> Option<DependencyIssue> {
        let name = package.get("name")?.as_str()?;
        let version = package.get("version")?.as_str()?;
        
        // Skip workspace packages
        if let Some(source) = package.get("source") {
            if source.is_null() {
                return None;
            }
        } else {
            return None;
        }
        
        // Check for known bloated dependencies
        let bloated_crates = self.get_known_bloated_crates();
        if let Some(alternatives) = bloated_crates.get(name) {
            return Some(DependencyIssue {
                package: name.to_string(),
                version: version.to_string(),
                issue_type: DependencyIssueType::Bloated,
                description: format!("Dependency {} is known to be bloated and may increase compile times and binary size", name),
                recommendation: format!("Consider alternatives: {}", alternatives.join(", ")),
            });
        }
        
        // Check for excessive dependencies
        if let Some(deps) = package.get("dependencies").and_then(|d| d.as_array()) {
            if deps.len() > 20 {
                return Some(DependencyIssue {
                    package: name.to_string(),
                    version: version.to_string(),
                    issue_type: DependencyIssueType::Bloated,
                    description: format!("Dependency {} has {} dependencies, which may indicate bloat", name, deps.len()),
                    recommendation: format!("Review if all features of {} are needed, consider feature flags to reduce bloat", name),
                });
            }
        }
        
        None
    }
    
    fn get_known_bloated_crates(&self) -> HashMap<&'static str, Vec<&'static str>> {
        let mut bloated = HashMap::new();
        
        // Common bloated crates and their alternatives
        bloated.insert("reqwest", vec!["ureq", "surf", "isahc"]);
        bloated.insert("tokio", vec!["async-std", "smol"]);
        bloated.insert("clap", vec!["structopt", "argh", "pico-args"]);
        bloated.insert("serde_json", vec!["sonic-rs", "simd-json"]);
        bloated.insert("chrono", vec!["time", "jiff"]);
        bloated.insert("uuid", vec!["uuid with minimal features"]);
        bloated.insert("image", vec!["image with specific format features only"]);
        
        bloated
    }
    
    fn check_unnecessary_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        // Use cargo-machete if available to find unused dependencies
        let output = Command::new("cargo")
            .args(&["machete"])
            .current_dir(project_path)
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                issues.extend(self.parse_machete_output(&stdout)?);
            }
            _ => {
                // cargo-machete not available, use basic heuristics
                issues.extend(self.check_unused_dependencies_heuristic(project_path)?);
            }
        }
        
        Ok(issues)
    }
    
    fn parse_machete_output(&self, output: &str) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        let mut issues = Vec::new();
        
        for line in output.lines() {
            if line.contains("unused") {
                if let Some(package) = self.extract_package_from_machete_line(line) {
                    issues.push(DependencyIssue {
                        package: package.clone(),
                        version: "unknown".to_string(),
                        issue_type: DependencyIssueType::Bloated,
                        description: format!("Dependency {} appears to be unused", package),
                        recommendation: format!("Remove unused dependency {} from Cargo.toml", package),
                    });
                }
            }
        }
        
        Ok(issues)
    }
    
    fn extract_package_from_machete_line(&self, line: &str) -> Option<String> {
        // Parse cargo-machete output format
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 0 {
            Some(parts[0].to_string())
        } else {
            None
        }
    }
    
    fn check_unused_dependencies_heuristic(&self, project_path: &Path) -> Result<Vec<DependencyIssue>, SecurityAuditError> {
        // Basic heuristic: check if dependencies are imported in source files
        let mut issues = Vec::new();
        
        // Get list of dependencies from Cargo.toml
        let cargo_toml_path = project_path.join("Cargo.toml");
        let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| SecurityAuditError::IoError(e))?;
        
        let dependencies = self.extract_dependencies_from_toml(&cargo_toml_content);
        
        // Check if each dependency is used in source files
        for dep in dependencies {
            if !self.is_dependency_used(project_path, &dep)? {
                issues.push(DependencyIssue {
                    package: dep.clone(),
                    version: "unknown".to_string(),
                    issue_type: DependencyIssueType::Bloated,
                    description: format!("Dependency {} may be unused (heuristic check)", dep),
                    recommendation: format!("Verify if {} is actually used, consider removing if unused", dep),
                });
            }
        }
        
        Ok(issues)
    }
    
    fn extract_dependencies_from_toml(&self, content: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        let mut in_dependencies_section = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line == "[dependencies]" {
                in_dependencies_section = true;
                continue;
            }
            
            if line.starts_with('[') && line != "[dependencies]" {
                in_dependencies_section = false;
                continue;
            }
            
            if in_dependencies_section && !line.is_empty() && !line.starts_with('#') {
                if let Some(dep_name) = line.split('=').next() {
                    let dep_name = dep_name.trim().trim_matches('"');
                    dependencies.push(dep_name.to_string());
                }
            }
        }
        
        dependencies
    }
    
    fn is_dependency_used(&self, project_path: &Path, dependency: &str) -> Result<bool, SecurityAuditError> {
        // Simple heuristic: search for "use dependency" or "extern crate dependency" in source files
        let src_path = project_path.join("src");
        
        if !src_path.exists() {
            return Ok(true); // Assume used if we can't check
        }
        
        let output = Command::new("grep")
            .args(&["-r", &format!("use {}", dependency), src_path.to_str().unwrap()])
            .output();
        
        match output {
            Ok(output) => Ok(output.status.success()),
            Err(_) => {
                // grep not available, assume dependency is used
                Ok(true)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyUpdateResult {
    pub passed: bool,
    pub test_results: Vec<TestResult>,
    pub build_success: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration: Duration,
    pub output: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityAuditError {
    #[error("Command failed: {0}")]
    CommandFailed(String),
    
    #[error("Tool not installed: {0}")]
    ToolNotInstalled(String),
    
    #[error("Audit failed: {0}")]
    AuditFailed(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl SecurityAuditResult {
    /// Formats the audit result for display.
    pub fn format_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("🔒 Security Audit Report\n");
        report.push_str("========================\n\n");
        
        if self.warnings_only {
            report.push_str("⚠️  Running in WARNING-ONLY mode (not blocking build)\n\n");
        }
        
        // Vulnerabilities section
        if !self.vulnerabilities.is_empty() {
            report.push_str(&format!("🚨 Vulnerabilities Found: {}\n", self.vulnerabilities.len()));
            for vuln in &self.vulnerabilities {
                report.push_str(&format!(
                    "  • {} v{}: {} ({:?})\n    {}\n",
                    vuln.package, vuln.version, vuln.title, vuln.severity, vuln.description
                ));
                if !vuln.patched_versions.is_empty() {
                    report.push_str(&format!("    Patched in: {}\n", vuln.patched_versions.join(", ")));
                }
                report.push('\n');
            }
        } else {
            report.push_str("✅ No vulnerabilities found\n\n");
        }
        
        // License issues section
        if !self.license_issues.is_empty() {
            report.push_str(&format!("📄 License Issues Found: {}\n", self.license_issues.len()));
            for issue in &self.license_issues {
                report.push_str(&format!(
                    "  • {} v{}: {} license ({:?})\n    {}\n\n",
                    issue.package, issue.version, issue.license, issue.issue_type, issue.recommendation
                ));
            }
        } else {
            report.push_str("✅ No license issues found\n\n");
        }
        
        // Dependency issues section
        if !self.dependency_issues.is_empty() {
            report.push_str(&format!("📦 Dependency Issues Found: {}\n", self.dependency_issues.len()));
            for issue in &self.dependency_issues {
                report.push_str(&format!(
                    "  • {} v{}: {:?}\n    {}\n\n",
                    issue.package, issue.version, issue.issue_type, issue.description
                ));
            }
        } else {
            report.push_str("✅ No dependency issues found\n\n");
        }
        
        // Recommendations section
        if !self.recommendations.is_empty() {
            report.push_str("💡 Recommendations:\n");
            for rec in &self.recommendations {
                let urgency_icon = match rec.urgency {
                    RecommendationUrgency::Critical => "🔴",
                    RecommendationUrgency::High => "🟠",
                    RecommendationUrgency::Medium => "🟡",
                    RecommendationUrgency::Low => "🟢",
                };
                
                report.push_str(&format!("  {} {}\n", urgency_icon, rec.description));
                if let Some(cmd) = &rec.fix_command {
                    report.push_str(&format!("    Command: {}\n", cmd));
                }
                report.push('\n');
            }
        }
        
        report.push_str(&format!("Execution time: {:.2}s\n", self.execution_time.as_secs_f32()));
        report.push_str(&format!("Status: {}\n", if self.passed { "✅ PASSED" } else { "❌ FAILED" }));
        
        report
    }
}

impl DependencyBloatDetector {
    pub fn new() -> Self {
        Self {
            size_thresholds: BloatThresholds::default(),
            alternative_suggestions: Self::build_alternative_suggestions(),
            feature_optimization_rules: Self::build_feature_optimization_rules(),
        }
    }
    
    fn build_alternative_suggestions() -> HashMap<String, Vec<AlternativeSuggestion>> {
        let mut suggestions = HashMap::new();
        
        // HTTP client alternatives
        suggestions.insert("reqwest".to_string(), vec![
            AlternativeSuggestion {
                name: "ureq".to_string(),
                description: "Minimal HTTP client with no async runtime dependency".to_string(),
                pros: vec![
                    "Much smaller binary size".to_string(),
                    "Faster compile times".to_string(),
                    "No tokio dependency".to_string(),
                ],
                cons: vec![
                    "Synchronous only".to_string(),
                    "Fewer features".to_string(),
                ],
                migration_effort: MigrationEffort::Easy,
                performance_impact: PerformanceImpact::Improvement("Reduced binary size by ~2MB".to_string()),
            },
            AlternativeSuggestion {
                name: "surf".to_string(),
                description: "Async HTTP client with modular backends".to_string(),
                pros: vec![
                    "Smaller than reqwest".to_string(),
                    "Modular design".to_string(),
                ],
                cons: vec![
                    "Less mature ecosystem".to_string(),
                ],
                migration_effort: MigrationEffort::Moderate,
                performance_impact: PerformanceImpact::Improvement("Reduced compile time by 20%".to_string()),
            },
        ]);
        
        // Async runtime alternatives
        suggestions.insert("tokio".to_string(), vec![
            AlternativeSuggestion {
                name: "async-std".to_string(),
                description: "Async runtime with std-like API".to_string(),
                pros: vec![
                    "Familiar std-like API".to_string(),
                    "Smaller than full tokio".to_string(),
                ],
                cons: vec![
                    "Less ecosystem support".to_string(),
                ],
                migration_effort: MigrationEffort::Moderate,
                performance_impact: PerformanceImpact::Neutral,
            },
            AlternativeSuggestion {
                name: "smol".to_string(),
                description: "Minimal async runtime".to_string(),
                pros: vec![
                    "Very small footprint".to_string(),
                    "Fast compile times".to_string(),
                ],
                cons: vec![
                    "Fewer built-in utilities".to_string(),
                ],
                migration_effort: MigrationEffort::Difficult,
                performance_impact: PerformanceImpact::Improvement("Reduced binary size by ~1.5MB".to_string()),
            },
        ]);
        
        // CLI parsing alternatives
        suggestions.insert("clap".to_string(), vec![
            AlternativeSuggestion {
                name: "argh".to_string(),
                description: "Derive-based argument parsing".to_string(),
                pros: vec![
                    "Much faster compile times".to_string(),
                    "Smaller binary size".to_string(),
                    "Derive-based API".to_string(),
                ],
                cons: vec![
                    "Fewer features".to_string(),
                    "No shell completion".to_string(),
                ],
                migration_effort: MigrationEffort::Easy,
                performance_impact: PerformanceImpact::Improvement("Reduced compile time by 50%".to_string()),
            },
            AlternativeSuggestion {
                name: "pico-args".to_string(),
                description: "Minimal argument parsing".to_string(),
                pros: vec![
                    "Extremely fast compilation".to_string(),
                    "Tiny binary footprint".to_string(),
                ],
                cons: vec![
                    "Manual parsing required".to_string(),
                    "No help generation".to_string(),
                ],
                migration_effort: MigrationEffort::Moderate,
                performance_impact: PerformanceImpact::Improvement("Reduced compile time by 80%".to_string()),
            },
        ]);
        
        // JSON parsing alternatives
        suggestions.insert("serde_json".to_string(), vec![
            AlternativeSuggestion {
                name: "sonic-rs".to_string(),
                description: "Fast JSON parser with SIMD optimization".to_string(),
                pros: vec![
                    "Much faster parsing".to_string(),
                    "SIMD optimized".to_string(),
                ],
                cons: vec![
                    "Less mature".to_string(),
                    "API differences".to_string(),
                ],
                migration_effort: MigrationEffort::Moderate,
                performance_impact: PerformanceImpact::Improvement("2-3x faster JSON parsing".to_string()),
            },
        ]);
        
        // Date/time alternatives
        suggestions.insert("chrono".to_string(), vec![
            AlternativeSuggestion {
                name: "time".to_string(),
                description: "Modern date/time library with better API".to_string(),
                pros: vec![
                    "Better type safety".to_string(),
                    "Smaller dependency tree".to_string(),
                    "More secure".to_string(),
                ],
                cons: vec![
                    "Different API".to_string(),
                ],
                migration_effort: MigrationEffort::Moderate,
                performance_impact: PerformanceImpact::Improvement("Reduced compile time by 15%".to_string()),
            },
        ]);
        
        suggestions
    }
    
    fn build_feature_optimization_rules() -> Vec<FeatureOptimizationRule> {
        vec![
            FeatureOptimizationRule {
                crate_name: "serde".to_string(),
                default_features_bloat: true,
                recommended_features: vec!["derive".to_string()],
                features_to_avoid: vec!["rc".to_string(), "unstable".to_string()],
                size_reduction_estimate: "~200KB binary size reduction".to_string(),
            },
            FeatureOptimizationRule {
                crate_name: "tokio".to_string(),
                default_features_bloat: true,
                recommended_features: vec!["rt".to_string(), "macros".to_string()],
                features_to_avoid: vec!["full".to_string(), "test-util".to_string()],
                size_reduction_estimate: "~1MB binary size reduction".to_string(),
            },
            FeatureOptimizationRule {
                crate_name: "reqwest".to_string(),
                default_features_bloat: true,
                recommended_features: vec!["json".to_string()],
                features_to_avoid: vec!["blocking".to_string(), "cookies".to_string(), "gzip".to_string()],
                size_reduction_estimate: "~500KB binary size reduction".to_string(),
            },
            FeatureOptimizationRule {
                crate_name: "image".to_string(),
                default_features_bloat: true,
                recommended_features: vec!["png".to_string()], // Only include needed formats
                features_to_avoid: vec!["default".to_string()],
                size_reduction_estimate: "~2MB binary size reduction".to_string(),
            },
            FeatureOptimizationRule {
                crate_name: "bevy".to_string(),
                default_features_bloat: true,
                recommended_features: vec!["dynamic_linking".to_string()], // For development
                features_to_avoid: vec!["default".to_string()],
                size_reduction_estimate: "Faster compile times in development".to_string(),
            },
        ]
    }
    
    /// Analyzes a dependency for bloat and suggests alternatives.
    pub fn analyze_dependency_bloat(&self, package_name: &str, metadata: &serde_json::Value) -> Vec<DependencyIssue> {
        let mut issues = Vec::new();
        
        // Check if this is a known bloated dependency
        if let Some(alternatives) = self.alternative_suggestions.get(package_name) {
            let version = metadata.get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            
            issues.push(DependencyIssue {
                package: package_name.to_string(),
                version: version.to_string(),
                issue_type: DependencyIssueType::Bloated,
                description: format!(
                    "Dependency '{}' is known to increase compile times and binary size significantly",
                    package_name
                ),
                recommendation: self.format_alternative_recommendations(alternatives),
            });
        }
        
        // Check for feature optimization opportunities
        if let Some(rule) = self.feature_optimization_rules.iter().find(|r| r.crate_name == package_name) {
            if rule.default_features_bloat {
                let version = metadata.get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                
                issues.push(DependencyIssue {
                    package: package_name.to_string(),
                    version: version.to_string(),
                    issue_type: DependencyIssueType::Bloated,
                    description: format!(
                        "Dependency '{}' uses default features which may include unnecessary bloat",
                        package_name
                    ),
                    recommendation: format!(
                        "Disable default features and only enable needed ones: {}. Estimated savings: {}",
                        rule.recommended_features.join(", "),
                        rule.size_reduction_estimate
                    ),
                });
            }
        }
        
        issues
    }
    
    fn format_alternative_recommendations(&self, alternatives: &[AlternativeSuggestion]) -> String {
        let mut recommendation = "Consider these alternatives:\n".to_string();
        
        for alt in alternatives {
            recommendation.push_str(&format!(
                "  • {}: {} (Migration: {:?}, Impact: {})\n",
                alt.name,
                alt.description,
                alt.migration_effort,
                match &alt.performance_impact {
                    PerformanceImpact::Improvement(desc) => desc,
                    PerformanceImpact::Neutral => "No significant impact",
                    PerformanceImpact::Degradation(desc) => desc,
                }
            ));
            
            if !alt.pros.is_empty() {
                recommendation.push_str(&format!("    Pros: {}\n", alt.pros.join(", ")));
            }
            
            if !alt.cons.is_empty() {
                recommendation.push_str(&format!("    Cons: {}\n", alt.cons.join(", ")));
            }
        }
        
        recommendation
    }
    
    /// Provides upgrade guidance for security issues.
    pub fn generate_upgrade_guidance(&self, vulnerability: &Vulnerability) -> String {
        let mut guidance = format!(
            "🔒 Security Vulnerability: {} in {} v{}\n",
            vulnerability.title, vulnerability.package, vulnerability.version
        );
        
        guidance.push_str(&format!("Severity: {:?}\n", vulnerability.severity));
        guidance.push_str(&format!("Description: {}\n\n", vulnerability.description));
        
        if !vulnerability.patched_versions.is_empty() {
            guidance.push_str("✅ Remediation Steps:\n");
            guidance.push_str(&format!(
                "1. Update to a patched version: {}\n",
                vulnerability.patched_versions.join(", ")
            ));
            guidance.push_str(&format!(
                "2. Run: cargo update -p {}\n",
                vulnerability.package
            ));
            guidance.push_str("3. Test your application to ensure compatibility\n");
            guidance.push_str("4. Run security audit again to verify fix\n\n");
        } else if !vulnerability.unaffected_versions.is_empty() {
            guidance.push_str("⚠️  No patched version available. Consider:\n");
            guidance.push_str(&format!(
                "1. Downgrade to unaffected version: {}\n",
                vulnerability.unaffected_versions.join(", ")
            ));
            guidance.push_str("2. Find alternative dependencies\n");
            guidance.push_str("3. Implement additional security measures\n\n");
        } else {
            guidance.push_str("❌ No fix available. Consider:\n");
            guidance.push_str("1. Finding alternative dependencies\n");
            guidance.push_str("2. Implementing workarounds\n");
            guidance.push_str("3. Accepting the risk with proper documentation\n\n");
        }
        
        if let Some(url) = &vulnerability.url {
            guidance.push_str(&format!("📖 More information: {}\n", url));
        }
        
        guidance
    }
}

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