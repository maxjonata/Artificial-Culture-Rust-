use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};

use crate::cicd::configuration::CiCdConfigurationManager;
use crate::cicd::git_hooks::{GitHookManager, HookType};

/// Automated setup and installation system for CI/CD infrastructure.
#[derive(Debug)]
pub struct CiCdSetupManager {
    pub repository_path: PathBuf,
    pub configuration_manager: CiCdConfigurationManager,
    pub dependency_checker: DependencyChecker,
    pub setup_progress: SetupProgress,
}

/// Tracks the progress of the setup process.
#[derive(Debug, Default)]
pub struct SetupProgress {
    pub steps_completed: Vec<SetupStep>,
    pub current_step: Option<SetupStep>,
    pub total_steps: usize,
    pub errors: Vec<SetupError>,
}

/// Individual setup steps.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetupStep {
    ValidateRepository,
    CheckDependencies,
    InstallMissingTools,
    CreateConfigurationFiles,
    InstallGitHooks,
    ValidateSetup,
    GenerateDocumentation,
    RunInitialValidation,
}

/// Dependency checker for required tools and components.
#[derive(Debug)]
pub struct DependencyChecker {
    pub required_tools: Vec<RequiredTool>,
    pub optional_tools: Vec<OptionalTool>,
    pub rust_components: Vec<RustComponent>,
}

/// Required tool definition.
#[derive(Debug, Clone)]
pub struct RequiredTool {
    pub name: String,
    pub command: String,
    pub version_check: Option<String>,
    pub installation_guide: String,
    pub is_available: bool,
}

/// Optional tool definition.
#[derive(Debug, Clone)]
pub struct OptionalTool {
    pub name: String,
    pub command: String,
    pub purpose: String,
    pub installation_guide: String,
    pub is_available: bool,
}

/// Rust component definition.
#[derive(Debug, Clone)]
pub struct RustComponent {
    pub name: String,
    pub component_name: String,
    pub purpose: String,
    pub is_installed: bool,
}

/// Setup configuration for customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupConfiguration {
    pub auto_install_hooks: bool,
    pub auto_install_tools: bool,
    pub create_default_config: bool,
    pub run_initial_validation: bool,
    pub generate_documentation: bool,
    pub setup_level: SetupLevel,
    pub custom_config_path: Option<PathBuf>,
}

/// Level of setup to perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetupLevel {
    Minimal,      // Just hooks and basic config
    Standard,     // Full setup with all validations
    Development,  // Development setup with debug tools
    Production,   // Production-ready setup with all optimizations
}

impl CiCdSetupManager {
    /// Creates a new setup manager for the given repository.
    pub fn new(repository_path: PathBuf) -> Result<Self, SetupError> {
        // Validate that this is a git repository
        let git_dir = repository_path.join(".git");
        if !git_dir.exists() {
            return Err(SetupError::NotAGitRepository(repository_path));
        }
        
        let config_path = repository_path.join(".cicd-config.toml");
        let configuration_manager = CiCdConfigurationManager::new(config_path)
            .map_err(|e| SetupError::ConfigurationError(e.to_string()))?;
        
        let dependency_checker = DependencyChecker::new();
        
        Ok(Self {
            repository_path,
            configuration_manager,
            dependency_checker,
            setup_progress: SetupProgress::default(),
        })
    }
    
    /// Runs the complete automated setup process.
    pub async fn run_automated_setup(&mut self, config: SetupConfiguration) -> Result<SetupReport, SetupError> {
        let mut report = SetupReport::new();
        
        // Define setup steps based on configuration
        let steps = self.determine_setup_steps(&config);
        self.setup_progress.total_steps = steps.len();
        
        for step in steps {
            self.setup_progress.current_step = Some(step.clone());
            
            match self.execute_setup_step(&step, &config).await {
                Ok(step_result) => {
                    self.setup_progress.steps_completed.push(step.clone());
                    report.step_results.insert(step, step_result);
                }
                Err(error) => {
                    self.setup_progress.errors.push(error.clone());
                    report.errors.push(error.clone());
                    
                    // Decide whether to continue or abort based on error severity
                    if error.is_critical() {
                        return Err(error);
                    }
                }
            }
        }
        
        self.setup_progress.current_step = None;
        report.success = self.setup_progress.errors.is_empty();
        
        Ok(report)
    }
    
    /// Determines which setup steps to execute based on configuration.
    fn determine_setup_steps(&self, config: &SetupConfiguration) -> Vec<SetupStep> {
        let mut steps = vec![
            SetupStep::ValidateRepository,
            SetupStep::CheckDependencies,
        ];
        
        match config.setup_level {
            SetupLevel::Minimal => {
                if config.auto_install_hooks {
                    steps.push(SetupStep::InstallGitHooks);
                }
                if config.create_default_config {
                    steps.push(SetupStep::CreateConfigurationFiles);
                }
            }
            SetupLevel::Standard | SetupLevel::Development | SetupLevel::Production => {
                if config.auto_install_tools {
                    steps.push(SetupStep::InstallMissingTools);
                }
                if config.create_default_config {
                    steps.push(SetupStep::CreateConfigurationFiles);
                }
                if config.auto_install_hooks {
                    steps.push(SetupStep::InstallGitHooks);
                }
                steps.push(SetupStep::ValidateSetup);
                
                if config.generate_documentation {
                    steps.push(SetupStep::GenerateDocumentation);
                }
                
                if config.run_initial_validation {
                    steps.push(SetupStep::RunInitialValidation);
                }
            }
        }
        
        steps
    }
    
    /// Executes a single setup step.
    async fn execute_setup_step(&mut self, step: &SetupStep, config: &SetupConfiguration) -> Result<StepResult, SetupError> {
        match step {
            SetupStep::ValidateRepository => self.validate_repository(),
            SetupStep::CheckDependencies => self.check_dependencies().await,
            SetupStep::InstallMissingTools => self.install_missing_tools().await,
            SetupStep::CreateConfigurationFiles => self.create_configuration_files(config).await,
            SetupStep::InstallGitHooks => self.install_git_hooks().await,
            SetupStep::ValidateSetup => self.validate_setup().await,
            SetupStep::GenerateDocumentation => self.generate_documentation().await,
            SetupStep::RunInitialValidation => self.run_initial_validation().await,
        }
    }
    
    /// Validates that the repository is properly set up for CI/CD.
    fn validate_repository(&self) -> Result<StepResult, SetupError> {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check for .git directory
        if !self.repository_path.join(".git").exists() {
            return Err(SetupError::NotAGitRepository(self.repository_path.clone()));
        }
        
        // Check for Cargo.toml (Rust project)
        if !self.repository_path.join("Cargo.toml").exists() {
            issues.push("No Cargo.toml found - this doesn't appear to be a Rust project".to_string());
            suggestions.push("Initialize a Rust project with 'cargo init' or ensure you're in the correct directory".to_string());
        }
        
        // Check for src directory
        if !self.repository_path.join("src").exists() {
            issues.push("No src directory found".to_string());
            suggestions.push("Create a src directory with your Rust source code".to_string());
        }
        
        // Check for existing CI/CD configuration
        let config_files = [
            ".github/workflows",
            ".gitlab-ci.yml",
            "Jenkinsfile",
            ".cicd-config.toml",
        ];
        
        let existing_ci = config_files.iter()
            .filter(|&file| self.repository_path.join(file).exists())
            .collect::<Vec<_>>();
        
        if !existing_ci.is_empty() {
            let existing_list: Vec<String> = existing_ci.iter().map(|s| s.to_string()).collect();
            suggestions.push(format!(
                "Existing CI/CD configuration found: {}. Consider backing up before proceeding.",
                existing_list.join(", ")
            ));
        }
        
        Ok(StepResult {
            success: issues.is_empty(),
            message: if issues.is_empty() {
                "Repository validation passed".to_string()
            } else {
                format!("Repository validation found {} issues", issues.len())
            },
            details: issues,
            suggestions,
            duration: std::time::Duration::from_millis(10),
        })
    }
    
    /// Checks for required and optional dependencies.
    async fn check_dependencies(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check required tools
        let mut required_tools_to_update = Vec::new();
        for (i, tool) in self.dependency_checker.required_tools.iter().enumerate() {
            let is_available = self.check_tool_availability(&tool.command, tool.version_check.as_deref()).await;
            required_tools_to_update.push((i, is_available));
            if !is_available {
                missing_required.push(tool.name.clone());
                suggestions.push(format!("{}: {}", tool.name, tool.installation_guide));
            }
        }
        
        // Update availability status
        for (i, is_available) in required_tools_to_update {
            self.dependency_checker.required_tools[i].is_available = is_available;
        }
        
        // Check optional tools
        let mut optional_tools_to_update = Vec::new();
        for (i, tool) in self.dependency_checker.optional_tools.iter().enumerate() {
            let is_available = self.check_tool_availability(&tool.command, None).await;
            optional_tools_to_update.push((i, is_available));
            if !is_available {
                missing_optional.push(tool.name.clone());
                suggestions.push(format!("{} (optional): {}", tool.name, tool.installation_guide));
            }
        }
        
        // Update availability status
        for (i, is_available) in optional_tools_to_update {
            self.dependency_checker.optional_tools[i].is_available = is_available;
        }
        
        // Check Rust components
        let mut rust_components_to_update = Vec::new();
        for (i, component) in self.dependency_checker.rust_components.iter().enumerate() {
            let is_installed = self.check_rust_component(&component.component_name).await;
            rust_components_to_update.push((i, is_installed));
            if !is_installed {
                missing_required.push(component.name.clone());
                suggestions.push(format!("Install {} with: rustup component add {}", component.name, component.component_name));
            }
        }
        
        // Update installation status
        for (i, is_installed) in rust_components_to_update {
            self.dependency_checker.rust_components[i].is_installed = is_installed;
        }
        
        let success = missing_required.is_empty();
        let mut details = Vec::new();
        
        if !missing_required.is_empty() {
            details.push(format!("Missing required tools: {}", missing_required.join(", ")));
        }
        
        if !missing_optional.is_empty() {
            details.push(format!("Missing optional tools: {}", missing_optional.join(", ")));
        }
        
        Ok(StepResult {
            success,
            message: if success {
                "All required dependencies are available".to_string()
            } else {
                format!("Missing {} required dependencies", missing_required.len())
            },
            details,
            suggestions,
            duration: start.elapsed(),
        })
    }
    
    /// Attempts to install missing tools automatically.
    async fn install_missing_tools(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut installed = Vec::new();
        let mut failed = Vec::new();
        let mut suggestions = Vec::new();
        
        // Install missing Rust components
        for component in &self.dependency_checker.rust_components {
            if !component.is_installed {
                match self.install_rust_component(&component.component_name).await {
                    Ok(_) => installed.push(component.name.clone()),
                    Err(e) => {
                        failed.push(component.name.clone());
                        suggestions.push(format!("Failed to install {}: {}", component.name, e));
                    }
                }
            }
        }
        
        // For other tools, provide installation guidance rather than attempting automatic installation
        for tool in &self.dependency_checker.required_tools {
            if !tool.is_available {
                suggestions.push(format!("Please install {}: {}", tool.name, tool.installation_guide));
            }
        }
        
        Ok(StepResult {
            success: failed.is_empty(),
            message: if installed.is_empty() && failed.is_empty() {
                "No tools needed installation".to_string()
            } else {
                format!("Installed {} tools, {} failed", installed.len(), failed.len())
            },
            details: if !installed.is_empty() {
                vec![format!("Successfully installed: {}", installed.join(", "))]
            } else {
                vec![]
            },
            suggestions,
            duration: start.elapsed(),
        })
    }
    
    /// Creates default configuration files.
    async fn create_configuration_files(&mut self, config: &SetupConfiguration) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut created_files = Vec::new();
        let mut suggestions = Vec::new();
        
        // Create main CI/CD configuration
        let config_path = config.custom_config_path.clone()
            .unwrap_or_else(|| self.repository_path.join(".cicd-config.toml"));
        
        if !config_path.exists() {
            self.configuration_manager.save_to_file().await
                .map_err(|e| SetupError::ConfigurationError(e.to_string()))?;
            created_files.push(config_path.file_name().unwrap().to_string_lossy().to_string());
        }
        
        // Create .gitignore entries for CI/CD artifacts
        let gitignore_path = self.repository_path.join(".gitignore");
        let gitignore_entries = vec![
            "# CI/CD artifacts",
            "target/ci-reports/",
            ".cicd-cache/",
            "*.cicd.log",
        ];
        
        if gitignore_path.exists() {
            let existing_content = fs::read_to_string(&gitignore_path)
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
            
            let mut new_entries = Vec::new();
            for entry in &gitignore_entries {
                if !existing_content.contains(entry) {
                    new_entries.push(*entry);
                }
            }
            
            if !new_entries.is_empty() {
                let mut updated_content = existing_content;
                updated_content.push_str("\n");
                updated_content.push_str(&new_entries.join("\n"));
                updated_content.push_str("\n");
                
                fs::write(&gitignore_path, updated_content)
                    .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
                
                suggestions.push("Updated .gitignore with CI/CD artifact patterns".to_string());
            }
        } else {
            fs::write(&gitignore_path, gitignore_entries.join("\n") + "\n")
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
            created_files.push(".gitignore".to_string());
        }
        
        // Create CI/CD documentation
        let docs_dir = self.repository_path.join("docs").join("cicd");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
        }
        
        let readme_path = docs_dir.join("README.md");
        if !readme_path.exists() {
            let readme_content = self.generate_cicd_documentation();
            fs::write(&readme_path, readme_content)
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
            created_files.push("docs/cicd/README.md".to_string());
        }
        
        Ok(StepResult {
            success: true,
            message: format!("Created {} configuration files", created_files.len()),
            details: created_files.iter().map(|f| format!("Created: {}", f)).collect(),
            suggestions,
            duration: start.elapsed(),
        })
    }
    
    /// Installs Git hooks.
    async fn install_git_hooks(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        
        let mut hook_manager = GitHookManager::new();
        
        match hook_manager.install_hooks(&self.repository_path) {
            Ok(_) => {
                let installed_hooks = hook_manager.hook_configurations.keys()
                    .map(|hook_type| format!("{:?}", hook_type))
                    .collect::<Vec<_>>();
                
                Ok(StepResult {
                    success: true,
                    message: format!("Successfully installed {} Git hooks", installed_hooks.len()),
                    details: installed_hooks.iter().map(|h| format!("Installed: {} hook", h)).collect(),
                    suggestions: vec![
                        "Git hooks are now active and will run automatically".to_string(),
                        "Use 'git commit --no-verify' to bypass hooks in emergencies".to_string(),
                    ],
                    duration: start.elapsed(),
                })
            }
            Err(e) => Err(SetupError::HookInstallationError(e.to_string())),
        }
    }
    
    /// Validates the complete setup.
    async fn validate_setup(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut validation_results = Vec::new();
        let mut suggestions = Vec::new();
        
        // Validate configuration
        let _config = self.configuration_manager.get_configuration().await;
        validation_results.push("Configuration loaded successfully".to_string());
        
        // Validate Git hooks
        let hooks_dir = self.repository_path.join(".git").join("hooks");
        let expected_hooks = [HookType::PreCommit, HookType::PrePush];
        
        for hook_type in &expected_hooks {
            let hook_file = hooks_dir.join(hook_type.filename());
            if hook_file.exists() && hook_file.metadata().unwrap().len() > 0 {
                validation_results.push(format!("{:?} hook installed and configured", hook_type));
            } else {
                suggestions.push(format!("{:?} hook missing or empty", hook_type));
            }
        }
        
        // Validate tool availability
        let available_tools = self.dependency_checker.required_tools.iter()
            .filter(|tool| tool.is_available)
            .count();
        let total_tools = self.dependency_checker.required_tools.len();
        
        validation_results.push(format!("{}/{} required tools available", available_tools, total_tools));
        
        if available_tools < total_tools {
            suggestions.push("Some required tools are missing - CI/CD functionality may be limited".to_string());
        }
        
        let success = suggestions.is_empty();
        
        Ok(StepResult {
            success,
            message: if success {
                "Setup validation passed completely".to_string()
            } else {
                format!("Setup validation completed with {} issues", suggestions.len())
            },
            details: validation_results,
            suggestions,
            duration: start.elapsed(),
        })
    }
    
    /// Generates setup documentation.
    async fn generate_documentation(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut generated_files = Vec::new();
        
        // Generate developer onboarding guide
        let onboarding_path = self.repository_path.join("docs").join("DEVELOPER_ONBOARDING.md");
        if !onboarding_path.exists() {
            let onboarding_content = self.generate_onboarding_documentation();
            fs::write(&onboarding_path, onboarding_content)
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
            generated_files.push("DEVELOPER_ONBOARDING.md".to_string());
        }
        
        // Generate CI/CD troubleshooting guide
        let troubleshooting_path = self.repository_path.join("docs").join("cicd").join("TROUBLESHOOTING.md");
        if !troubleshooting_path.exists() {
            let troubleshooting_content = self.generate_troubleshooting_documentation();
            fs::write(&troubleshooting_path, troubleshooting_content)
                .map_err(|e| SetupError::FileOperationError(e.to_string()))?;
            generated_files.push("cicd/TROUBLESHOOTING.md".to_string());
        }
        
        Ok(StepResult {
            success: true,
            message: format!("Generated {} documentation files", generated_files.len()),
            details: generated_files.iter().map(|f| format!("Generated: docs/{}", f)).collect(),
            suggestions: vec![
                "Review the generated documentation for setup and troubleshooting guidance".to_string(),
            ],
            duration: start.elapsed(),
        })
    }
    
    /// Runs initial validation to ensure everything works.
    async fn run_initial_validation(&mut self) -> Result<StepResult, SetupError> {
        let start = std::time::Instant::now();
        let mut validation_results = Vec::new();
        let mut suggestions = Vec::new();
        
        // Test basic Rust compilation
        let output = Command::new("cargo")
            .args(&["check", "--workspace"])
            .current_dir(&self.repository_path)
            .output()
            .map_err(|e| SetupError::CommandExecutionError(e.to_string()))?;
        
        if output.status.success() {
            validation_results.push("Rust compilation check passed".to_string());
        } else {
            let error_output = String::from_utf8_lossy(&output.stderr);
            suggestions.push(format!("Compilation errors found: {}", error_output));
        }
        
        // Test formatting
        let output = Command::new("cargo")
            .args(&["fmt", "--check"])
            .current_dir(&self.repository_path)
            .output()
            .map_err(|e| SetupError::CommandExecutionError(e.to_string()))?;
        
        if output.status.success() {
            validation_results.push("Code formatting check passed".to_string());
        } else {
            suggestions.push("Code formatting issues found - run 'cargo fmt' to fix".to_string());
        }
        
        // Test clippy
        let output = Command::new("cargo")
            .args(&["clippy", "--workspace", "--", "-D", "warnings"])
            .current_dir(&self.repository_path)
            .output()
            .map_err(|e| SetupError::CommandExecutionError(e.to_string()))?;
        
        if output.status.success() {
            validation_results.push("Clippy linting check passed".to_string());
        } else {
            suggestions.push("Clippy warnings found - review and fix before committing".to_string());
        }
        
        let success = suggestions.is_empty();
        
        Ok(StepResult {
            success,
            message: if success {
                "Initial validation passed - CI/CD setup is working correctly".to_string()
            } else {
                format!("Initial validation found {} issues to address", suggestions.len())
            },
            details: validation_results,
            suggestions,
            duration: start.elapsed(),
        })
    }
    
    /// Checks if a tool is available on the system.
    async fn check_tool_availability(&self, command: &str, version_check: Option<&str>) -> bool {
        let mut cmd = Command::new(command);
        
        if let Some(version_arg) = version_check {
            cmd.arg(version_arg);
        } else {
            cmd.arg("--version");
        }
        
        cmd.output().map(|output| output.status.success()).unwrap_or(false)
    }
    
    /// Checks if a Rust component is installed.
    async fn check_rust_component(&self, component: &str) -> bool {
        Command::new("rustup")
            .args(&["component", "list", "--installed"])
            .output()
            .map(|output| {
                if output.status.success() {
                    let installed = String::from_utf8_lossy(&output.stdout);
                    installed.lines().any(|line| line.trim() == component)
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
    
    /// Installs a Rust component.
    async fn install_rust_component(&self, component: &str) -> Result<(), String> {
        let output = Command::new("rustup")
            .args(&["component", "add", component])
            .output()
            .map_err(|e| e.to_string())?;
        
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    
    /// Generates CI/CD documentation content.
    fn generate_cicd_documentation(&self) -> String {
        r#"# CI/CD Configuration

This document describes the CI/CD setup for this project.

## Overview

The CI/CD system provides automated quality gates and validation to ensure code quality, performance, and behavioral consistency.

## Components

### Pre-Commit Hooks
- **Purpose**: Fast essential checks before commit
- **Target Time**: <45 seconds
- **Checks**: Formatting, essential linting, unit tests for changed modules, documentation validation

### Pre-Push Hooks
- **Purpose**: Comprehensive validation before push
- **Target Time**: <10 minutes
- **Checks**: Complete build, AI pattern validation, ECS architecture validation, performance patterns, full test suite, security audit

### Configuration
- **File**: `.cicd-config.toml`
- **Hot Reload**: Supported for development
- **Customization**: All validation rules and thresholds are configurable

## Usage

### Normal Development
The hooks run automatically. No manual intervention required.

### Emergency Bypass
Use commit message keywords:
- `BYPASS_PRECOMMIT` - Skip pre-commit checks
- `BYPASS_PREPUSH` - Skip pre-push checks
- `EMERGENCY` - Skip all checks

### Manual Validation
Run individual validators:
```bash
cargo run --bin ai-pattern-validator
cargo run --bin ecs-pattern-validator
cargo run --bin performance-pattern-validator
```

## Troubleshooting

See `TROUBLESHOOTING.md` for common issues and solutions.
"#.to_string()
    }
    
    /// Generates developer onboarding documentation.
    fn generate_onboarding_documentation(&self) -> String {
        r#"# Developer Onboarding Guide

Welcome to the project! This guide will help you set up your development environment.

## Prerequisites

### Required Tools
- **Rust**: Install from https://rustup.rs/
- **Git**: Version control system
- **Cargo**: Rust package manager (included with Rust)

### Rust Components
The following components will be installed automatically:
- `rustfmt` - Code formatting
- `clippy` - Linting and code analysis

### Optional Tools
- `cargo-audit` - Security vulnerability scanning
- `cargo-watch` - Automatic rebuilding during development

## Setup Process

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

2. **Run automated setup**
   ```bash
   cargo run --bin cicd-setup
   ```

3. **Verify setup**
   ```bash
   cargo check
   cargo test
   ```

## Development Workflow

### Making Changes
1. Create a feature branch
2. Make your changes
3. Commit (pre-commit hooks will run automatically)
4. Push (pre-push hooks will run automatically)
5. Create a pull request

### Code Quality Standards
- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Fix all `cargo clippy` warnings
- **Testing**: Ensure all tests pass with `cargo test`
- **Documentation**: Document public APIs and behavioral purposes

### AI-Specific Guidelines
- Use `Normalized<f32>` for personality traits (0.0-1.0 range)
- Use `f32` for emotional states (-1.0 to 1.0 range)
- All AI systems must be modulated by personality traits
- Use `WorldTime` resource for temporal calculations

### Performance Guidelines
- Avoid `f64` in hot paths (use `f32` instead)
- No synchronous I/O in systems
- Use efficient Bevy query patterns
- Target 60fps with 100+ agents

## Getting Help

- Check `docs/cicd/TROUBLESHOOTING.md` for common issues
- Review the project's coding standards and conventions
- Ask questions in team channels or create issues

## Project Philosophy

This project follows the "Feel Over Science" philosophy:
- Prioritize believable behavior over scientific accuracy
- Embrace imperfection and miscommunication as features
- Use continuous state spaces for social/emotional systems
- Personality drives everything
"#.to_string()
    }
    
    /// Generates troubleshooting documentation.
    fn generate_troubleshooting_documentation(&self) -> String {
        r#"# CI/CD Troubleshooting Guide

Common issues and solutions for the CI/CD system.

## Hook Issues

### Pre-commit hook fails with timeout
**Problem**: Pre-commit checks take longer than 45 seconds
**Solutions**:
- Run `cargo clean` to clear build cache
- Check for large files in commit
- Temporarily bypass with `git commit --no-verify`
- Increase timeout in `.cicd-config.toml`

### Pre-push hook fails with compilation errors
**Problem**: Code doesn't compile
**Solutions**:
- Run `cargo build --workspace` locally
- Fix compilation errors before pushing
- Check for missing dependencies in `Cargo.toml`

### Hook script not executable
**Problem**: Permission denied when running hooks
**Solutions**:
- Run `chmod +x .git/hooks/pre-commit .git/hooks/pre-push`
- Reinstall hooks with `cargo run --bin cicd-setup`

## Tool Issues

### rustfmt not found
**Problem**: `rustfmt` component missing
**Solution**: `rustup component add rustfmt`

### clippy not found
**Problem**: `clippy` component missing
**Solution**: `rustup component add clippy`

### cargo-audit not found
**Problem**: Security audit tool missing
**Solution**: `cargo install cargo-audit`

## Validation Issues

### AI pattern validation fails
**Problem**: Personality traits not using correct types
**Solutions**:
- Use `Normalized<f32>` for personality traits
- Add range documentation: `/// openness: 0.0 (low) to 1.0 (high)`
- Ensure emotional states use `f32` with -1.0 to 1.0 range

### ECS architecture validation fails
**Problem**: Components contain behavior methods
**Solutions**:
- Move behavior methods to systems
- Keep components as pure data structures
- Use queries instead of direct component access

### Performance pattern validation fails
**Problem**: f64 usage in hot paths detected
**Solutions**:
- Replace `f64` with `f32` in performance-critical code
- Use `f64` only for non-performance-critical calculations
- Add exceptions to configuration if necessary

## Configuration Issues

### Configuration file not found
**Problem**: `.cicd-config.toml` missing
**Solution**: Run `cargo run --bin cicd-setup` to regenerate

### Invalid configuration format
**Problem**: TOML parsing errors
**Solutions**:
- Validate TOML syntax with online validator
- Check for missing quotes around strings
- Ensure proper indentation and structure

### Hot reload not working
**Problem**: Configuration changes not detected
**Solutions**:
- Restart the CI/CD process
- Check file permissions on configuration file
- Verify hot reload is enabled in configuration

## Performance Issues

### Slow validation times
**Problem**: CI/CD checks take too long
**Solutions**:
- Enable parallel execution in configuration
- Reduce validation scope for development
- Use incremental builds with `cargo check`

### Memory usage too high
**Problem**: Validation processes consume too much memory
**Solutions**:
- Reduce concurrent validation processes
- Clear build cache regularly
- Check for memory leaks in custom validators

## Emergency Procedures

### Complete bypass
**Environment variable**: `EMERGENCY_BYPASS=1`
**Commit message**: Include `EMERGENCY` keyword

### Partial bypass
**Pre-commit only**: Include `BYPASS_PRECOMMIT` in commit message
**Pre-push only**: Include `BYPASS_PREPUSH` in commit message

### Restore from backup
If hooks are corrupted:
1. `rm .git/hooks/pre-commit .git/hooks/pre-push`
2. `cargo run --bin cicd-setup`
3. Verify with test commit

## Getting Additional Help

1. Check the main CI/CD documentation in `docs/cicd/README.md`
2. Review project coding standards and conventions
3. Create an issue with detailed error messages and steps to reproduce
4. Include output from `cargo --version` and `rustc --version`
"#.to_string()
    }
}

impl DependencyChecker {
    /// Creates a new dependency checker with default tool requirements.
    pub fn new() -> Self {
        let required_tools = vec![
            RequiredTool {
                name: "Rust Compiler".to_string(),
                command: "rustc".to_string(),
                version_check: Some("--version".to_string()),
                installation_guide: "Install from https://rustup.rs/".to_string(),
                is_available: false,
            },
            RequiredTool {
                name: "Cargo".to_string(),
                command: "cargo".to_string(),
                version_check: Some("--version".to_string()),
                installation_guide: "Included with Rust installation".to_string(),
                is_available: false,
            },
            RequiredTool {
                name: "Git".to_string(),
                command: "git".to_string(),
                version_check: Some("--version".to_string()),
                installation_guide: "Install from https://git-scm.com/".to_string(),
                is_available: false,
            },
        ];
        
        let optional_tools = vec![
            OptionalTool {
                name: "Cargo Audit".to_string(),
                command: "cargo-audit".to_string(),
                purpose: "Security vulnerability scanning".to_string(),
                installation_guide: "cargo install cargo-audit".to_string(),
                is_available: false,
            },
            OptionalTool {
                name: "Cargo Watch".to_string(),
                command: "cargo-watch".to_string(),
                purpose: "Automatic rebuilding during development".to_string(),
                installation_guide: "cargo install cargo-watch".to_string(),
                is_available: false,
            },
        ];
        
        let rust_components = vec![
            RustComponent {
                name: "Rustfmt".to_string(),
                component_name: "rustfmt".to_string(),
                purpose: "Code formatting".to_string(),
                is_installed: false,
            },
            RustComponent {
                name: "Clippy".to_string(),
                component_name: "clippy".to_string(),
                purpose: "Linting and code analysis".to_string(),
                is_installed: false,
            },
        ];
        
        Self {
            required_tools,
            optional_tools,
            rust_components,
        }
    }
}

impl Default for SetupConfiguration {
    fn default() -> Self {
        Self {
            auto_install_hooks: true,
            auto_install_tools: true,
            create_default_config: true,
            run_initial_validation: true,
            generate_documentation: true,
            setup_level: SetupLevel::Standard,
            custom_config_path: None,
        }
    }
}

/// Result of a single setup step.
#[derive(Debug, Clone)]
pub struct StepResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
    pub suggestions: Vec<String>,
    pub duration: std::time::Duration,
}

/// Complete setup report.
#[derive(Debug)]
pub struct SetupReport {
    pub success: bool,
    pub step_results: HashMap<SetupStep, StepResult>,
    pub errors: Vec<SetupError>,
    pub total_duration: std::time::Duration,
    pub summary: String,
}

impl SetupReport {
    fn new() -> Self {
        Self {
            success: false,
            step_results: HashMap::new(),
            errors: Vec::new(),
            total_duration: std::time::Duration::default(),
            summary: String::new(),
        }
    }
}

/// Errors that can occur during setup.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SetupError {
    #[error("Not a Git repository: {0}")]
    NotAGitRepository(PathBuf),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("File operation error: {0}")]
    FileOperationError(String),
    
    #[error("Command execution error: {0}")]
    CommandExecutionError(String),
    
    #[error("Hook installation error: {0}")]
    HookInstallationError(String),
    
    #[error("Dependency check error: {0}")]
    DependencyCheckError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl SetupError {
    /// Determines if this error should abort the entire setup process.
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            SetupError::NotAGitRepository(_) | SetupError::ConfigurationError(_)
        )
    }
}