use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug)]
pub struct GitHookManager {
    pub hook_configurations: HashMap<HookType, HookConfiguration>,
    pub bypass_mechanisms: BypassConfiguration,
    pub progress_reporters: Vec<Box<dyn ProgressReporter>>,
    pub error_formatters: Vec<Box<dyn ErrorFormatter>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HookType {
    PreCommit,
    PrePush,
    CommitMsg,
    PostCheckout,
}

impl HookType {
    pub fn filename(&self) -> &'static str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PrePush => "pre-push",
            HookType::CommitMsg => "commit-msg",
            HookType::PostCheckout => "post-checkout",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HookConfiguration {
    pub script_path: PathBuf,
    pub timeout_seconds: u64,
    pub required_tools: Vec<String>,
    pub environment_variables: HashMap<String, String>,
    pub bypass_conditions: Vec<BypassCondition>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BypassConfiguration {
    pub emergency_bypass_enabled: bool,
    pub bypass_keywords: Vec<String>,
    pub bypass_environment_variable: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BypassCondition {
    CommitMessageContains(String),
    EnvironmentVariable(String, String),
    FileExists(PathBuf),
    BranchName(String),
}

pub trait ProgressReporter: std::fmt::Debug {
    fn report_progress(&self, stage: &str, progress: f32, message: &str);
    fn report_completion(&self, stage: &str, duration: Duration, success: bool);
}

pub trait ErrorFormatter: std::fmt::Debug {
    fn format_error(&self, error: &HookError) -> String;
    fn suggest_fix(&self, error: &HookError) -> Option<String>;
}

#[derive(Debug, thiserror::Error)]
pub enum HookInstallationError {
    #[error("Failed to create hooks directory: {0}")]
    DirectoryCreation(std::io::Error),

    #[error("Failed to write hook script: {0}")]
    ScriptWrite(std::io::Error),

    #[error("Failed to set hook permissions: {0}")]
    PermissionSet(std::io::Error),

    #[error("Repository not found at path: {0}")]
    RepositoryNotFound(PathBuf),

    #[error("Hook script generation failed: {0}")]
    ScriptGeneration(ScriptGenerationError),
}

#[derive(Debug, thiserror::Error)]
pub enum ScriptGenerationError {
    #[error("Unsupported hook type")]
    UnsupportedHookType,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Template rendering failed: {0}")]
    TemplateRenderingFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum HookError {
    #[error("Hook execution timeout after {timeout}s")]
    Timeout { timeout: u64 },

    #[error("Hook execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Required tool missing: {tool}")]
    MissingTool { tool: String },

    #[error("Validation failed: {details}")]
    ValidationFailed { details: String },
}

impl Default for GitHookManager {
    fn default() -> Self {
        let mut hook_configurations = HashMap::new();

        // Pre-commit configuration (fast essential checks)
        hook_configurations.insert(
            HookType::PreCommit,
            HookConfiguration {
                script_path: PathBuf::from("pre-commit"),
                timeout_seconds: 45, // Total target: 45 seconds
                required_tools: vec![
                    "cargo".to_string(),
                    "rustfmt".to_string(),
                    "clippy".to_string(),
                ],
                environment_variables: HashMap::new(),
                bypass_conditions: vec![
                    BypassCondition::CommitMessageContains("BYPASS_PRECOMMIT".to_string()),
                    BypassCondition::EnvironmentVariable("SKIP_HOOKS".to_string(), "1".to_string()),
                ],
            },
        );

        // Pre-push configuration (comprehensive validation)
        hook_configurations.insert(
            HookType::PrePush,
            HookConfiguration {
                script_path: PathBuf::from("pre-push"),
                timeout_seconds: 600, // Total target: 10 minutes
                required_tools: vec![
                    "cargo".to_string(),
                    "rustfmt".to_string(),
                    "clippy".to_string(),
                ],
                environment_variables: HashMap::new(),
                bypass_conditions: vec![
                    BypassCondition::CommitMessageContains("BYPASS_PREPUSH".to_string()),
                    BypassCondition::EnvironmentVariable("SKIP_HOOKS".to_string(), "1".to_string()),
                ],
            },
        );

        Self {
            hook_configurations,
            bypass_mechanisms: BypassConfiguration {
                emergency_bypass_enabled: true,
                bypass_keywords: vec![
                    "EMERGENCY".to_string(),
                    "HOTFIX".to_string(),
                    "BYPASS_HOOKS".to_string(),
                ],
                bypass_environment_variable: Some("EMERGENCY_BYPASS".to_string()),
            },
            progress_reporters: Vec::new(),
            error_formatters: Vec::new(),
        }
    }
}

impl GitHookManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn install_hooks(&mut self, repository_path: &Path) -> Result<(), HookInstallationError> {
        // Verify this is a git repository
        let git_dir = repository_path.join(".git");
        if !git_dir.exists() {
            return Err(HookInstallationError::RepositoryNotFound(
                repository_path.to_path_buf(),
            ));
        }

        let hooks_dir = git_dir.join("hooks");

        // Create hooks directory if it doesn't exist
        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir).map_err(HookInstallationError::DirectoryCreation)?;
        }

        // Install each configured hook
        for (hook_type, config) in &self.hook_configurations {
            let hook_file = hooks_dir.join(hook_type.filename());

            // Generate hook script
            let hook_script = self
                .generate_hook_script(hook_type, config)
                .map_err(HookInstallationError::ScriptGeneration)?;

            // Write hook script
            fs::write(&hook_file, hook_script).map_err(HookInstallationError::ScriptWrite)?;

            // Set executable permissions (Unix-like systems)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&hook_file)
                    .map_err(HookInstallationError::PermissionSet)?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&hook_file, perms)
                    .map_err(HookInstallationError::PermissionSet)?;
            }

            println!("✅ Installed {} hook", hook_type.filename());
        }

        println!("🎉 All Git hooks installed successfully!");
        Ok(())
    }

    fn generate_hook_script(
        &self,
        hook_type: &HookType,
        config: &HookConfiguration,
    ) -> Result<String, ScriptGenerationError> {
        match hook_type {
            HookType::PreCommit => self.generate_pre_commit_script(config),
            HookType::PrePush => self.generate_pre_push_script(config),
            _ => Err(ScriptGenerationError::UnsupportedHookType),
        }
    }

    fn generate_pre_commit_script(
        &self,
        config: &HookConfiguration,
    ) -> Result<String, ScriptGenerationError> {
        let bypass_checks = self.generate_bypass_checks(&config.bypass_conditions);
        let timeout = config.timeout_seconds;

        Ok(format!(
            r#"#!/bin/bash
set -e

echo "🚀 Running pre-commit quality gates (FAST - Essential Only)..."

{bypass_checks}

# Tool availability check
command -v cargo >/dev/null 2>&1 || {{ echo "❌ cargo not found. Please install Rust toolchain."; exit 1; }}
command -v rustfmt >/dev/null 2>&1 || {{ echo "❌ rustfmt not found. Run 'rustup component add rustfmt'"; exit 1; }}
command -v clippy-driver >/dev/null 2>&1 || {{ echo "❌ clippy not found. Run 'rustup component add clippy'"; exit 1; }}

# 1. Code formatting check (target: 5 seconds)
echo "📝 Checking code formatting..."
timeout {timeout} cargo fmt --check || {{
    echo "❌ Code formatting failed. Run 'cargo fmt' to fix."
    echo "💡 Fix command: cargo fmt"
    exit 1
}}

# 2. Essential clippy lints (target: 10 seconds)  
echo "🔍 Running essential clippy checks..."
timeout {timeout} cargo clippy --workspace -- -D warnings || {{
    echo "❌ Clippy found issues. Fix warnings before committing."
    echo "💡 Fix command: cargo clippy --fix --workspace"
    exit 1
}}

# 3. Unit tests for changed modules only (target: 25 seconds)
echo "🧪 Running tests for changed modules..."
CHANGED_FILES=$(git diff --cached --name-only --diff-filter=AM | grep '\.rs$' || true)
if [ ! -z "$CHANGED_FILES" ]; then
    # Extract module paths and run targeted tests
    timeout {timeout} cargo test --lib || {{
        echo "❌ Unit tests failed for changed modules."
        echo "💡 Fix command: cargo test --lib"
        exit 1
    }}
fi

# 4. Documentation validation for changed files only (target: 5 seconds)
echo "📚 Validating documentation for changed files..."
DOC_FILES=$(git diff --cached --name-only --diff-filter=AM | grep -E '\.(md|rs)$' || true)
if [ ! -z "$DOC_FILES" ]; then
    timeout {timeout} cargo doc --no-deps --document-private-items --quiet || {{
        echo "❌ Documentation build failed for changed files."
        echo "💡 Fix command: cargo doc --no-deps --document-private-items"
        exit 1
    }}
fi

echo "✅ Pre-commit checks passed! Commit allowed."
"#
        ))
    }

    fn generate_pre_push_script(
        &self,
        config: &HookConfiguration,
    ) -> Result<String, ScriptGenerationError> {
        let bypass_checks = self.generate_bypass_checks(&config.bypass_conditions);

        Ok(format!(
            r#"#!/bin/bash
set -e

echo "🔒 Running pre-push validation (COMPREHENSIVE - Everything)..."

{bypass_checks}

# Tool availability check
command -v cargo >/dev/null 2>&1 || {{ echo "❌ cargo not found. Please install Rust toolchain."; exit 1; }}

# 1. Complete build check (target: 2 minutes)
echo "🏗️  Running complete build check..."
timeout 120 cargo build --workspace || {{
    echo "❌ Build failed. Fix compilation errors before pushing."
    echo "💡 Fix command: cargo build --workspace"
    exit 1
}}

# 2. AI pattern validation on ALL files (target: 1 minute)
echo "🧠 Validating AI patterns on ALL files..."
if [ -f "target/debug/ai-pattern-validator" ] || cargo build --bin ai-pattern-validator 2>/dev/null; then
    timeout 60 cargo run --bin ai-pattern-validator -- --all-files || {{
        echo "❌ AI pattern validation failed. Check personality traits, emotional ranges, and temporal consistency."
        echo "💡 Review AI coding patterns in the documentation"
        exit 1
    }}
else
    echo "⚠️  AI pattern validator not available, skipping..."
fi

# 3. Bevy ECS validation on ALL Rust files (target: 1 minute)
echo "⚙️  Validating Bevy ECS patterns on ALL Rust files..."
if [ -f "target/debug/ecs-pattern-validator" ] || cargo build --bin ecs-pattern-validator 2>/dev/null; then
    timeout 60 cargo run --bin ecs-pattern-validator -- --all-rust-files || {{
        echo "❌ ECS pattern validation failed. Check component design and system architecture."
        echo "💡 Review ECS architecture patterns in the documentation"
        exit 1
    }}
else
    echo "⚠️  ECS pattern validator not available, skipping..."
fi

# 4. Performance pattern validation on ALL Rust files (target: 1 minute)
echo "⚡ Validating performance patterns on ALL Rust files..."
if [ -f "target/debug/performance-pattern-validator" ] || cargo build --bin performance-pattern-validator 2>/dev/null; then
    timeout 60 cargo run --bin performance-pattern-validator -- --all-rust-files || {{
        echo "❌ Performance pattern validation failed. Check for f64 usage and sync I/O in hot paths."
        echo "💡 Review performance optimization guidelines"
        exit 1
    }}
else
    echo "⚠️  Performance pattern validator not available, skipping..."
fi

# 5. Complete test suite (target: 4 minutes)
echo "🧪 Running complete test suite..."
timeout 240 cargo test --workspace || {{
    echo "❌ Test suite failed. Fix failing tests before pushing."
    echo "💡 Fix command: cargo test --workspace"
    exit 1
}}

# 6. Integration tests if they exist
if [ -d "tests" ]; then
    echo "🔗 Running integration tests..."
    timeout 120 cargo test --tests || {{
        echo "❌ Integration tests failed."
        echo "💡 Fix command: cargo test --tests"
        exit 1
    }}
fi

# 7. Strict clippy with all lints enabled (target: 30 seconds)
echo "🔍 Running strict clippy validation..."
timeout 30 cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic || {{
    echo "❌ Strict clippy validation failed. Fix all lints before pushing."
    echo "💡 Fix command: cargo clippy --fix --workspace --all-targets --all-features"
    exit 1
}}

# 8. Release build validation (target: 2 minutes)
echo "🚀 Validating release build..."
timeout 120 cargo build --workspace --release || {{
    echo "❌ Release build failed. Fix release-specific issues before pushing."
    echo "💡 Fix command: cargo build --workspace --release"
    exit 1
}}

# 9. Documentation build validation (target: 1 minute)
echo "📚 Building complete documentation..."
timeout 60 cargo doc --workspace --no-deps --document-private-items || {{
    echo "❌ Documentation build failed. Fix doc comments and examples."
    echo "💡 Fix command: cargo doc --workspace --no-deps --document-private-items"
    exit 1
}}

# 10. Security audit (warning only)
echo "🔒 Running security audit..."
if command -v cargo-audit >/dev/null 2>&1; then
    cargo audit || {{
        echo "⚠️  Security audit found issues (warning only - not blocking push)"
        echo "💡 Review security issues and consider updating dependencies"
    }}
else
    echo "⚠️  cargo-audit not installed, skipping security audit"
    echo "💡 Install with: cargo install cargo-audit"
fi

echo "✅ Pre-push validation passed! Push allowed."
"#
        ))
    }

    fn generate_bypass_checks(&self, conditions: &[BypassCondition]) -> String {
        let mut checks = Vec::new();

        for condition in conditions {
            match condition {
                BypassCondition::CommitMessageContains(keyword) => {
                    checks.push(format!(
                        r#"if git log --oneline -1 | grep -q "{}"; then
    echo "⚠️  Hook checks bypassed via commit message keyword: {}"
    exit 0
fi"#,
                        keyword, keyword
                    ));
                }
                BypassCondition::EnvironmentVariable(var, value) => {
                    checks.push(format!(
                        r#"if [ "${}" = "{}" ]; then
    echo "⚠️  Hook checks bypassed via environment variable: {}={}"
    exit 0
fi"#,
                        var, value, var, value
                    ));
                }
                BypassCondition::FileExists(path) => {
                    checks.push(format!(
                        r#"if [ -f "{}" ]; then
    echo "⚠️  Hook checks bypassed via bypass file: {}"
    exit 0
fi"#,
                        path.display(),
                        path.display()
                    ));
                }
                BypassCondition::BranchName(branch) => {
                    checks.push(format!(
                        r#"if [ "$(git branch --show-current)" = "{}" ]; then
    echo "⚠️  Hook checks bypassed for branch: {}"
    exit 0
fi"#,
                        branch, branch
                    ));
                }
            }
        }

        // Add emergency bypass check
        if self.bypass_mechanisms.emergency_bypass_enabled {
            if let Some(env_var) = &self.bypass_mechanisms.bypass_environment_variable {
                checks.push(format!(
                    r#"if [ "${}" = "1" ]; then
    echo "🚨 EMERGENCY BYPASS ACTIVATED - All checks skipped!"
    exit 0
fi"#,
                    env_var
                ));
            }
        }

        checks.join("\n\n")
    }

    pub fn add_progress_reporter(&mut self, reporter: Box<dyn ProgressReporter>) {
        self.progress_reporters.push(reporter);
    }

    pub fn add_error_formatter(&mut self, formatter: Box<dyn ErrorFormatter>) {
        self.error_formatters.push(formatter);
    }

    pub fn configure_hook(&mut self, hook_type: HookType, config: HookConfiguration) {
        self.hook_configurations.insert(hook_type, config);
    }

    pub fn check_bypass_conditions(&self, hook_type: &HookType, repository_path: &Path) -> bool {
        if let Some(config) = self.hook_configurations.get(hook_type) {
            for condition in &config.bypass_conditions {
                if self.evaluate_bypass_condition(condition, repository_path) {
                    return true;
                }
            }
        }

        // Check emergency bypass
        if self.bypass_mechanisms.emergency_bypass_enabled {
            if let Some(env_var) = &self.bypass_mechanisms.bypass_environment_variable {
                if std::env::var(env_var).unwrap_or_default() == "1" {
                    return true;
                }
            }
        }

        false
    }

    fn evaluate_bypass_condition(
        &self,
        condition: &BypassCondition,
        repository_path: &Path,
    ) -> bool {
        match condition {
            BypassCondition::CommitMessageContains(_keyword) => {
                // This would need to be implemented with git command execution
                // For now, return false as it's checked in the script
                false
            }
            BypassCondition::EnvironmentVariable(var, value) => {
                std::env::var(var).unwrap_or_default() == *value
            }
            BypassCondition::FileExists(path) => repository_path.join(path).exists(),
            BypassCondition::BranchName(_branch) => {
                // This would need to be implemented with git command execution
                // For now, return false as it's checked in the script
                false
            }
        }
    }
}

// Default implementations for traits
#[derive(Debug)]
pub struct ConsoleProgressReporter;

impl ProgressReporter for ConsoleProgressReporter {
    fn report_progress(&self, stage: &str, progress: f32, message: &str) {
        println!("🔄 {}: {:.1}% - {}", stage, progress * 100.0, message);
    }

    fn report_completion(&self, stage: &str, duration: Duration, success: bool) {
        let icon = if success { "✅" } else { "❌" };
        println!(
            "{} {} completed in {:.2}s",
            icon,
            stage,
            duration.as_secs_f32()
        );
    }
}

#[derive(Debug)]
pub struct DefaultErrorFormatter;

impl ErrorFormatter for DefaultErrorFormatter {
    fn format_error(&self, error: &HookError) -> String {
        match error {
            HookError::Timeout { timeout } => {
                format!(
                    "❌ Hook execution timed out after {}s. Consider optimizing checks or increasing timeout.",
                    timeout
                )
            }
            HookError::ExecutionFailed { message } => {
                format!("❌ Hook execution failed: {}", message)
            }
            HookError::MissingTool { tool } => {
                format!(
                    "❌ Required tool '{}' not found. Please install it before running hooks.",
                    tool
                )
            }
            HookError::ValidationFailed { details } => {
                format!("❌ Validation failed: {}", details)
            }
        }
    }

    fn suggest_fix(&self, error: &HookError) -> Option<String> {
        match error {
            HookError::MissingTool { tool } => {
                match tool.as_str() {
                    "cargo" => Some("Install Rust toolchain from https://rustup.rs/".to_string()),
                    "rustfmt" => Some("Run: rustup component add rustfmt".to_string()),
                    "clippy" => Some("Run: rustup component add clippy".to_string()),
                    "cargo-audit" => Some("Run: cargo install cargo-audit".to_string()),
                    _ => None,
                }
            }
            HookError::Timeout { .. } => {
                Some("Try running the checks manually to identify slow operations, or increase timeout in hook configuration".to_string())
            }
            _ => None,
        }
    }
}
