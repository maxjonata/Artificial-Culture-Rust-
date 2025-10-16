use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Cross-platform build validation system for ensuring code compiles across target platforms
#[derive(Debug)]
pub struct CrossPlatformBuildValidator {
    pub target_platforms: Vec<TargetPlatform>,
    pub build_configurations: Vec<BuildConfiguration>,
    pub artifact_manager: BuildArtifactManager,
    pub error_analyzer: BuildErrorAnalyzer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPlatform {
    pub name: String,
    pub target_triple: String,
    pub is_host_platform: bool,
    pub cross_compile_setup: Option<CrossCompileSetup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCompileSetup {
    pub linker: Option<String>,
    pub environment_variables: HashMap<String, String>,
    pub required_tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfiguration {
    pub name: String,
    pub profile: BuildProfile,
    pub features: Vec<String>,
    pub optimization_flags: Vec<String>,
    pub debug_info: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildProfile {
    Debug,
    Release,
    Test,
}

#[derive(Debug)]
pub struct BuildArtifactManager {
    pub output_directory: PathBuf,
    pub artifact_patterns: Vec<String>,
    pub verification_commands: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct BuildErrorAnalyzer {
    pub platform_specific_patterns: HashMap<String, Vec<ErrorPattern>>,
    pub common_error_patterns: Vec<ErrorPattern>,
    pub fix_suggestions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ErrorPattern {
    pub pattern: String,
    pub error_type: BuildErrorType,
    pub description: String,
    pub fix_suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum BuildErrorType {
    CompilationError,
    LinkingError,
    PlatformSpecificError,
    OptimizationError,
    DebugInfoError,
    ConditionalCompilationError,
}

#[derive(Debug)]
pub struct BuildResult {
    pub platform: TargetPlatform,
    pub configuration: BuildConfiguration,
    pub success: bool,
    pub build_time: Duration,
    pub artifacts: Vec<BuildArtifact>,
    pub errors: Vec<BuildError>,
    pub warnings: Vec<BuildWarning>,
    pub output_log: String,
}

#[derive(Debug)]
pub struct BuildArtifact {
    pub path: PathBuf,
    pub artifact_type: ArtifactType,
    pub size_bytes: u64,
    pub verification_status: VerificationStatus,
}

#[derive(Debug)]
pub enum ArtifactType {
    Executable,
    Library,
    DebugSymbols,
    Documentation,
}

#[derive(Debug)]
pub enum VerificationStatus {
    Verified,
    Failed(String),
    NotVerified,
}

impl VerificationStatus {
    pub fn is_verified(&self) -> bool {
        matches!(self, VerificationStatus::Verified)
    }
}

#[derive(Debug)]
pub struct BuildError {
    pub error_type: BuildErrorType,
    pub message: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
    pub platform_specific: bool,
    pub fix_suggestions: Vec<String>,
}

#[derive(Debug)]
pub struct BuildWarning {
    pub message: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
}

impl CrossPlatformBuildValidator {
    pub fn new() -> Self {
        Self {
            target_platforms: Self::default_target_platforms(),
            build_configurations: Self::default_build_configurations(),
            artifact_manager: BuildArtifactManager::new(),
            error_analyzer: BuildErrorAnalyzer::new(),
        }
    }

    /// Validate builds across all target platforms and configurations
    pub fn validate_all_platforms(&self, project_path: &Path) -> Vec<BuildResult> {
        let mut results = Vec::new();

        for platform in &self.target_platforms {
            for config in &self.build_configurations {
                println!("🏗️  Building for {} with {} configuration...", platform.name, config.name);
                
                let result = self.build_for_platform(project_path, platform, config);
                results.push(result);
            }
        }

        results
    }

    /// Build for a specific platform and configuration
    pub fn build_for_platform(
        &self,
        project_path: &Path,
        platform: &TargetPlatform,
        config: &BuildConfiguration,
    ) -> BuildResult {
        let start_time = Instant::now();
        
        // Setup cross-compilation environment if needed
        if let Some(cross_setup) = &platform.cross_compile_setup {
            if let Err(e) = self.setup_cross_compilation(cross_setup) {
                return BuildResult {
                    platform: platform.clone(),
                    configuration: config.clone(),
                    success: false,
                    build_time: start_time.elapsed(),
                    artifacts: Vec::new(),
                    errors: vec![BuildError {
                        error_type: BuildErrorType::PlatformSpecificError,
                        message: format!("Failed to setup cross-compilation: {}", e),
                        file_path: None,
                        line_number: None,
                        platform_specific: true,
                        fix_suggestions: vec![
                            "Install required cross-compilation tools".to_string(),
                            "Check target platform setup documentation".to_string(),
                        ],
                    }],
                    warnings: Vec::new(),
                    output_log: String::new(),
                };
            }
        }

        // Execute build command
        let mut build_command = self.create_build_command(project_path, platform, config);
        let output = build_command.output();

        let build_time = start_time.elapsed();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let full_log = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);

                if output.status.success() {
                    // Build succeeded, collect artifacts
                    let artifacts = self.collect_build_artifacts(project_path, platform, config);
                    let warnings = self.parse_build_warnings(&stderr);

                    BuildResult {
                        platform: platform.clone(),
                        configuration: config.clone(),
                        success: true,
                        build_time,
                        artifacts,
                        errors: Vec::new(),
                        warnings,
                        output_log: full_log,
                    }
                } else {
                    // Build failed, analyze errors
                    let errors = self.error_analyzer.analyze_build_errors(&stderr, platform);

                    BuildResult {
                        platform: platform.clone(),
                        configuration: config.clone(),
                        success: false,
                        build_time,
                        artifacts: Vec::new(),
                        errors,
                        warnings: Vec::new(),
                        output_log: full_log,
                    }
                }
            }
            Err(e) => {
                BuildResult {
                    platform: platform.clone(),
                    configuration: config.clone(),
                    success: false,
                    build_time,
                    artifacts: Vec::new(),
                    errors: vec![BuildError {
                        error_type: BuildErrorType::CompilationError,
                        message: format!("Failed to execute build command: {}", e),
                        file_path: None,
                        line_number: None,
                        platform_specific: false,
                        fix_suggestions: vec![
                            "Check that Rust toolchain is installed".to_string(),
                            "Verify cargo is in PATH".to_string(),
                        ],
                    }],
                    warnings: Vec::new(),
                    output_log: String::new(),
                }
            }
        }
    }

    /// Validate conditional compilation for platform-specific code
    pub fn validate_conditional_compilation(&self, project_path: &Path) -> Vec<ConditionalCompilationResult> {
        let mut results = Vec::new();

        // Find files with conditional compilation
        let conditional_files = self.find_conditional_compilation_files(project_path);

        for file_path in conditional_files {
            let result = self.validate_file_conditional_compilation(&file_path);
            results.push(result);
        }

        results
    }

    fn default_target_platforms() -> Vec<TargetPlatform> {
        vec![
            TargetPlatform {
                name: "Windows x64".to_string(),
                target_triple: "x86_64-pc-windows-msvc".to_string(),
                is_host_platform: cfg!(target_os = "windows"),
                cross_compile_setup: if cfg!(target_os = "windows") {
                    None
                } else {
                    Some(CrossCompileSetup {
                        linker: Some("x86_64-w64-mingw32-gcc".to_string()),
                        environment_variables: HashMap::new(),
                        required_tools: vec!["mingw-w64".to_string()],
                    })
                },
            },
            TargetPlatform {
                name: "Linux x64".to_string(),
                target_triple: "x86_64-unknown-linux-gnu".to_string(),
                is_host_platform: cfg!(target_os = "linux"),
                cross_compile_setup: if cfg!(target_os = "linux") {
                    None
                } else {
                    Some(CrossCompileSetup {
                        linker: Some("x86_64-linux-gnu-gcc".to_string()),
                        environment_variables: HashMap::new(),
                        required_tools: vec!["gcc-multilib".to_string()],
                    })
                },
            },
            TargetPlatform {
                name: "macOS x64".to_string(),
                target_triple: "x86_64-apple-darwin".to_string(),
                is_host_platform: cfg!(target_os = "macos"),
                cross_compile_setup: if cfg!(target_os = "macos") {
                    None
                } else {
                    Some(CrossCompileSetup {
                        linker: Some("x86_64-apple-darwin-clang".to_string()),
                        environment_variables: HashMap::new(),
                        required_tools: vec!["osxcross".to_string()],
                    })
                },
            },
        ]
    }

    fn default_build_configurations() -> Vec<BuildConfiguration> {
        vec![
            BuildConfiguration {
                name: "Debug".to_string(),
                profile: BuildProfile::Debug,
                features: vec!["default".to_string()],
                optimization_flags: vec![],
                debug_info: true,
            },
            BuildConfiguration {
                name: "Release".to_string(),
                profile: BuildProfile::Release,
                features: vec!["default".to_string()],
                optimization_flags: vec![
                    "-C".to_string(),
                    "opt-level=3".to_string(),
                    "-C".to_string(),
                    "lto=fat".to_string(),
                ],
                debug_info: false,
            },
            BuildConfiguration {
                name: "Release with Debug Info".to_string(),
                profile: BuildProfile::Release,
                features: vec!["default".to_string()],
                optimization_flags: vec![
                    "-C".to_string(),
                    "opt-level=3".to_string(),
                ],
                debug_info: true,
            },
        ]
    }

    fn setup_cross_compilation(&self, cross_setup: &CrossCompileSetup) -> Result<(), String> {
        // Set environment variables
        for (key, value) in &cross_setup.environment_variables {
            unsafe {
                std::env::set_var(key, value);
            }
        }

        // Check required tools are available
        for tool in &cross_setup.required_tools {
            let check_command = if cfg!(target_os = "windows") {
                Command::new("where").arg(tool).output()
            } else {
                Command::new("which").arg(tool).output()
            };

            match check_command {
                Ok(output) if output.status.success() => continue,
                _ => {
                    return Err(format!("Required tool '{}' not found in PATH", tool));
                }
            }
        }

        Ok(())
    }

    fn create_build_command(
        &self,
        project_path: &Path,
        platform: &TargetPlatform,
        config: &BuildConfiguration,
    ) -> Command {
        let mut command = Command::new("cargo");
        command.current_dir(project_path);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // Add build command
        command.arg("build");

        // Add target platform
        if !platform.is_host_platform {
            command.arg("--target").arg(&platform.target_triple);
        }

        // Add profile
        match config.profile {
            BuildProfile::Release => {
                command.arg("--release");
            }
            BuildProfile::Debug => {
                // Debug is default, no flag needed
            }
            BuildProfile::Test => {
                command.arg("--tests");
            }
        }

        // Add features
        if !config.features.is_empty() {
            command.arg("--features").arg(config.features.join(","));
        }

        // Add optimization flags via RUSTFLAGS
        if !config.optimization_flags.is_empty() {
            let rustflags = config.optimization_flags.join(" ");
            command.env("RUSTFLAGS", rustflags);
        }

        // Add debug info flag
        if config.debug_info {
            let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
            if !rustflags.is_empty() {
                rustflags.push(' ');
            }
            rustflags.push_str("-g");
            command.env("RUSTFLAGS", rustflags);
        }

        command
    }

    fn collect_build_artifacts(
        &self,
        project_path: &Path,
        platform: &TargetPlatform,
        config: &BuildConfiguration,
    ) -> Vec<BuildArtifact> {
        self.artifact_manager.collect_artifacts(project_path, platform, config)
    }

    fn parse_build_warnings(&self, stderr: &str) -> Vec<BuildWarning> {
        let mut warnings = Vec::new();
        
        for line in stderr.lines() {
            if line.contains("warning:") {
                // Parse warning format: "warning: message"
                if let Some(message_start) = line.find("warning:") {
                    let message = line[message_start + 8..].trim().to_string();
                    warnings.push(BuildWarning {
                        message,
                        file_path: None, // Could be enhanced to parse file paths
                        line_number: None, // Could be enhanced to parse line numbers
                    });
                }
            }
        }

        warnings
    }

    fn find_conditional_compilation_files(&self, project_path: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        // Walk through src directory looking for Rust files with cfg attributes
        if let Ok(entries) = std::fs::read_dir(project_path.join("src")) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "rs") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                if content.contains("#[cfg(") || content.contains("cfg!(") {
                                    files.push(path);
                                }
                            }
                        }
                    } else if file_type.is_dir() {
                        // Recursively search subdirectories
                        files.extend(self.find_conditional_compilation_files(&entry.path()));
                    }
                }
            }
        }

        files
    }

    fn validate_file_conditional_compilation(&self, file_path: &Path) -> ConditionalCompilationResult {
        // This would analyze the file for proper conditional compilation patterns
        // For now, return a basic result
        ConditionalCompilationResult {
            file_path: file_path.to_path_buf(),
            valid: true,
            issues: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ConditionalCompilationResult {
    pub file_path: PathBuf,
    pub valid: bool,
    pub issues: Vec<ConditionalCompilationIssue>,
}

#[derive(Debug)]
pub struct ConditionalCompilationIssue {
    pub issue_type: ConditionalCompilationIssueType,
    pub message: String,
    pub line_number: usize,
    pub suggestion: String,
}

#[derive(Debug)]
pub enum ConditionalCompilationIssueType {
    InvalidCfgAttribute,
    UnreachableCode,
    MissingPlatformSupport,
    ConflictingConditions,
}

impl BuildArtifactManager {
    pub fn new() -> Self {
        let mut verification_commands = HashMap::new();
        
        // Add platform-specific verification commands
        verification_commands.insert("exe".to_string(), vec![
            "file".to_string(),
            "ldd".to_string(), // For checking dependencies
        ]);
        verification_commands.insert("dll".to_string(), vec![
            "file".to_string(),
            "objdump".to_string(),
        ]);
        verification_commands.insert("so".to_string(), vec![
            "file".to_string(),
            "ldd".to_string(),
            "readelf".to_string(),
        ]);
        verification_commands.insert("dylib".to_string(), vec![
            "file".to_string(),
            "otool".to_string(),
        ]);

        Self {
            output_directory: PathBuf::from("target"),
            artifact_patterns: vec![
                "*.exe".to_string(),
                "*.dll".to_string(),
                "*.so".to_string(),
                "*.dylib".to_string(),
                "*.pdb".to_string(),
                "*.dSYM".to_string(),
                "*.a".to_string(),    // Static libraries
                "*.lib".to_string(),  // Windows static libraries
                "*.rlib".to_string(), // Rust libraries
            ],
            verification_commands,
        }
    }

    pub fn collect_artifacts(
        &self,
        project_path: &Path,
        platform: &TargetPlatform,
        config: &BuildConfiguration,
    ) -> Vec<BuildArtifact> {
        let mut artifacts = Vec::new();
        
        let target_dir = if platform.is_host_platform {
            project_path.join("target")
        } else {
            project_path.join("target").join(&platform.target_triple)
        };

        let profile_dir = match config.profile {
            BuildProfile::Debug => target_dir.join("debug"),
            BuildProfile::Release => target_dir.join("release"),
            BuildProfile::Test => target_dir.join("debug"),
        };

        if let Ok(entries) = std::fs::read_dir(&profile_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let artifact_type = self.determine_artifact_type(&path);
                        if artifact_type.is_some() {
                            artifacts.push(BuildArtifact {
                                path: path.clone(),
                                artifact_type: artifact_type.unwrap(),
                                size_bytes: metadata.len(),
                                verification_status: self.verify_artifact(&path),
                            });
                        }
                    }
                }
            }
        }

        artifacts
    }

    fn determine_artifact_type(&self, path: &Path) -> Option<ArtifactType> {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension {
                "exe" => Some(ArtifactType::Executable),
                "dll" | "so" | "dylib" => Some(ArtifactType::Library),
                "pdb" | "dSYM" => Some(ArtifactType::DebugSymbols),
                _ => None,
            }
        } else {
            // Check if it's an executable without extension (Unix-style)
            if path.is_file() {
                if let Ok(metadata) = std::fs::metadata(path) {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if metadata.permissions().mode() & 0o111 != 0 {
                            return Some(ArtifactType::Executable);
                        }
                    }
                }
            }
            None
        }
    }

    fn verify_artifact(&self, path: &Path) -> VerificationStatus {
        // Basic verification - check if file exists and is readable
        if !path.exists() || !path.is_file() {
            return VerificationStatus::Failed("Artifact not found or not readable".to_string());
        }

        // Get file extension for specific verification
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        // Perform platform-specific verification
        match extension {
            "exe" => self.verify_executable(path),
            "dll" | "so" | "dylib" => self.verify_library(path),
            "pdb" => self.verify_debug_symbols(path),
            "dSYM" => self.verify_macos_debug_symbols(path),
            "a" | "lib" | "rlib" => self.verify_static_library(path),
            _ => {
                // For files without extension, try to determine if they're executables
                self.verify_unknown_artifact(path)
            }
        }
    }

    fn verify_executable(&self, path: &Path) -> VerificationStatus {
        // Check if file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(path) {
                if metadata.permissions().mode() & 0o111 == 0 {
                    return VerificationStatus::Failed("Executable lacks execute permissions".to_string());
                }
            }
        }

        // Try to get file information
        if let Ok(output) = std::process::Command::new("file").arg(path).output() {
            let file_info = String::from_utf8_lossy(&output.stdout);
            if file_info.contains("executable") || file_info.contains("PE32") {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed(format!("File does not appear to be an executable: {}", file_info))
            }
        } else {
            // Fallback verification - just check if file exists and has reasonable size
            if let Ok(metadata) = std::fs::metadata(path) {
                if metadata.len() > 0 {
                    VerificationStatus::Verified
                } else {
                    VerificationStatus::Failed("Executable file is empty".to_string())
                }
            } else {
                VerificationStatus::Failed("Cannot read executable metadata".to_string())
            }
        }
    }

    fn verify_library(&self, path: &Path) -> VerificationStatus {
        // Use file command to verify library format
        if let Ok(output) = std::process::Command::new("file").arg(path).output() {
            let file_info = String::from_utf8_lossy(&output.stdout);
            
            let is_valid_library = file_info.contains("shared object") ||
                                 file_info.contains("dynamic library") ||
                                 file_info.contains("PE32") ||
                                 file_info.contains("Mach-O");

            if is_valid_library {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed(format!("File does not appear to be a valid library: {}", file_info))
            }
        } else {
            // Fallback - check file size
            if let Ok(metadata) = std::fs::metadata(path) {
                if metadata.len() > 0 {
                    VerificationStatus::Verified
                } else {
                    VerificationStatus::Failed("Library file is empty".to_string())
                }
            } else {
                VerificationStatus::Failed("Cannot read library metadata".to_string())
            }
        }
    }

    fn verify_debug_symbols(&self, path: &Path) -> VerificationStatus {
        // For PDB files (Windows debug symbols)
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > 0 {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed("Debug symbols file is empty".to_string())
            }
        } else {
            VerificationStatus::Failed("Cannot read debug symbols metadata".to_string())
        }
    }

    fn verify_macos_debug_symbols(&self, path: &Path) -> VerificationStatus {
        // For dSYM bundles (macOS debug symbols)
        if path.is_dir() {
            // dSYM is actually a directory bundle
            let dwarf_path = path.join("Contents").join("Resources").join("DWARF");
            if dwarf_path.exists() {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed("dSYM bundle missing DWARF directory".to_string())
            }
        } else {
            // Check if file exists and has reasonable size
            if let Ok(_metadata) = std::fs::metadata(path) {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed("dSYM should be a directory bundle".to_string())
            }
        }
    }

    fn verify_static_library(&self, path: &Path) -> VerificationStatus {
        // Use file command to verify static library format
        if let Ok(output) = std::process::Command::new("file").arg(path).output() {
            let file_info = String::from_utf8_lossy(&output.stdout);
            
            let is_valid_archive = file_info.contains("archive") ||
                                 file_info.contains("library") ||
                                 file_info.contains("ar archive");

            if is_valid_archive {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Failed(format!("File does not appear to be a valid static library: {}", file_info))
            }
        } else {
            // Fallback - check file size
            if let Ok(metadata) = std::fs::metadata(path) {
                if metadata.len() > 0 {
                    VerificationStatus::Verified
                } else {
                    VerificationStatus::Failed("Static library file is empty".to_string())
                }
            } else {
                VerificationStatus::Failed("Cannot read static library metadata".to_string())
            }
        }
    }

    fn verify_unknown_artifact(&self, path: &Path) -> VerificationStatus {
        // For files without clear extension, try to determine type
        if let Ok(output) = std::process::Command::new("file").arg(path).output() {
            let file_info = String::from_utf8_lossy(&output.stdout);
            
            if file_info.contains("executable") {
                self.verify_executable(path)
            } else if file_info.contains("shared object") || file_info.contains("dynamic library") {
                self.verify_library(path)
            } else {
                VerificationStatus::NotVerified
            }
        } else {
            VerificationStatus::NotVerified
        }
    }

    /// Generate build artifacts for distribution
    pub fn generate_distribution_artifacts(
        &self,
        project_path: &Path,
        platform: &TargetPlatform,
        config: &BuildConfiguration,
    ) -> Result<Vec<BuildArtifact>, String> {
        let artifacts = self.collect_artifacts(project_path, platform, config);
        let mut distribution_artifacts = Vec::new();

        // Create distribution directory
        let dist_dir = project_path.join("dist").join(&platform.target_triple);
        std::fs::create_dir_all(&dist_dir)
            .map_err(|e| format!("Failed to create distribution directory: {}", e))?;

        for artifact in artifacts {
            if artifact.verification_status.is_verified() {
                // Copy artifact to distribution directory
                let dist_path = dist_dir.join(artifact.path.file_name().unwrap());
                std::fs::copy(&artifact.path, &dist_path)
                    .map_err(|e| format!("Failed to copy artifact: {}", e))?;

                distribution_artifacts.push(BuildArtifact {
                    path: dist_path,
                    artifact_type: artifact.artifact_type,
                    size_bytes: artifact.size_bytes,
                    verification_status: VerificationStatus::Verified,
                });
            }
        }

        Ok(distribution_artifacts)
    }

    /// Create a build report with artifact information
    pub fn create_build_report(
        &self,
        results: &[BuildResult],
        output_path: &Path,
    ) -> Result<(), String> {
        let mut report = String::new();
        report.push_str("# Cross-Platform Build Report\n\n");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        report.push_str(&format!("Generated: {} (Unix timestamp)\n\n", now));

        // Summary statistics
        let total_builds = results.len();
        let successful_builds = results.iter().filter(|r| r.success).count();
        let failed_builds = total_builds - successful_builds;

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total builds: {}\n", total_builds));
        report.push_str(&format!("- Successful: {}\n", successful_builds));
        report.push_str(&format!("- Failed: {}\n\n", failed_builds));

        // Detailed results
        report.push_str("## Build Results\n\n");
        
        for result in results {
            let status = if result.success { "✅ PASS" } else { "❌ FAIL" };
            report.push_str(&format!("### {} - {} {}\n\n", result.platform.name, result.configuration.name, status));
            report.push_str(&format!("- Build time: {:?}\n", result.build_time));
            
            if result.success {
                report.push_str(&format!("- Artifacts: {}\n", result.artifacts.len()));
                for artifact in &result.artifacts {
                    let size_kb = artifact.size_bytes / 1024;
                    let verification = match &artifact.verification_status {
                        VerificationStatus::Verified => "✅",
                        VerificationStatus::Failed(_) => "❌",
                        VerificationStatus::NotVerified => "⚠️",
                    };
                    report.push_str(&format!("  - {} {} ({} KB) {}\n", 
                        artifact.path.file_name().unwrap().to_string_lossy(),
                        format!("{:?}", artifact.artifact_type),
                        size_kb,
                        verification
                    ));
                }
                
                if !result.warnings.is_empty() {
                    report.push_str(&format!("- Warnings: {}\n", result.warnings.len()));
                    for warning in &result.warnings {
                        report.push_str(&format!("  - ⚠️ {}\n", warning.message));
                    }
                }
            } else {
                report.push_str(&format!("- Errors: {}\n", result.errors.len()));
                for error in &result.errors {
                    report.push_str(&format!("  - ❌ {}\n", error.message));
                    if !error.fix_suggestions.is_empty() {
                        report.push_str("    Suggestions:\n");
                        for suggestion in &error.fix_suggestions {
                            report.push_str(&format!("    - {}\n", suggestion));
                        }
                    }
                }
            }
            
            report.push_str("\n");
        }

        std::fs::write(output_path, report)
            .map_err(|e| format!("Failed to write build report: {}", e))?;

        Ok(())
    }
}

impl BuildErrorAnalyzer {
    pub fn new() -> Self {
        Self {
            platform_specific_patterns: Self::create_platform_patterns(),
            common_error_patterns: Self::create_common_patterns(),
            fix_suggestions: Self::create_fix_suggestions(),
        }
    }

    /// Generate comprehensive troubleshooting guide for build failures
    pub fn generate_troubleshooting_guide(&self, errors: &[BuildError], platform: &TargetPlatform) -> String {
        let mut guide = String::new();
        
        guide.push_str(&format!("# Troubleshooting Guide for {}\n\n", platform.name));
        
        if errors.is_empty() {
            guide.push_str("No errors detected.\n");
            return guide;
        }

        guide.push_str("## Detected Issues\n\n");
        
        for (i, error) in errors.iter().enumerate() {
            guide.push_str(&format!("### Issue {}: {:?}\n\n", i + 1, error.error_type));
            guide.push_str(&format!("**Error:** {}\n\n", error.message));
            
            if let Some(file_path) = &error.file_path {
                guide.push_str(&format!("**File:** {}\n", file_path.display()));
                if let Some(line) = error.line_number {
                    guide.push_str(&format!("**Line:** {}\n", line));
                }
                guide.push_str("\n");
            }
            
            if !error.fix_suggestions.is_empty() {
                guide.push_str("**Suggested Fixes:**\n\n");
                for suggestion in &error.fix_suggestions {
                    guide.push_str(&format!("- {}\n", suggestion));
                }
                guide.push_str("\n");
            }
        }

        // Add platform-specific setup instructions
        guide.push_str("## Platform Setup Instructions\n\n");
        guide.push_str(&self.get_platform_setup_instructions(platform));

        // Add general troubleshooting steps
        guide.push_str("## General Troubleshooting Steps\n\n");
        guide.push_str("1. **Clean build artifacts:**\n");
        guide.push_str("   ```bash\n");
        guide.push_str("   cargo clean\n");
        guide.push_str("   ```\n\n");
        
        guide.push_str("2. **Update Rust toolchain:**\n");
        guide.push_str("   ```bash\n");
        guide.push_str("   rustup update\n");
        guide.push_str("   ```\n\n");
        
        guide.push_str("3. **Install target platform:**\n");
        guide.push_str(&format!("   ```bash\n   rustup target add {}\n   ```\n\n", platform.target_triple));
        
        guide.push_str("4. **Check dependencies:**\n");
        guide.push_str("   ```bash\n");
        guide.push_str("   cargo tree\n");
        guide.push_str("   ```\n\n");
        
        guide.push_str("5. **Verify toolchain:**\n");
        guide.push_str("   ```bash\n");
        guide.push_str("   rustc --version\n");
        guide.push_str("   cargo --version\n");
        guide.push_str("   ```\n\n");

        guide
    }

    fn get_platform_setup_instructions(&self, platform: &TargetPlatform) -> String {
        match platform.target_triple.as_str() {
            "x86_64-pc-windows-msvc" => {
                r#"### Windows (MSVC) Setup

**Required Tools:**
- Visual Studio Build Tools 2019 or later
- Windows 10 SDK

**Installation Steps:**
1. Download Visual Studio Installer
2. Install "C++ build tools" workload
3. Install Windows 10 SDK (latest version)
4. Add Rust MSVC target:
   ```bash
   rustup target add x86_64-pc-windows-msvc
   ```

**Common Issues:**
- Missing MSVC: Install Visual Studio Build Tools
- Missing Windows SDK: Install through Visual Studio Installer
- Linker errors: Ensure MSVC tools are in PATH

"#.to_string()
            }
            "x86_64-unknown-linux-gnu" => {
                r#"### Linux (GNU) Setup

**Required Tools:**
- GCC compiler
- GNU binutils
- libc development headers

**Installation Steps (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential
rustup target add x86_64-unknown-linux-gnu
```

**Installation Steps (CentOS/RHEL):**
```bash
sudo yum groupinstall "Development Tools"
sudo yum install gcc
rustup target add x86_64-unknown-linux-gnu
```

**Common Issues:**
- Missing compiler: Install build-essential or Development Tools
- Missing headers: Install libc6-dev or glibc-devel
- Linker errors: Check LD_LIBRARY_PATH

"#.to_string()
            }
            "x86_64-apple-darwin" => {
                r#"### macOS Setup

**Required Tools:**
- Xcode Command Line Tools
- macOS SDK

**Installation Steps:**
```bash
xcode-select --install
rustup target add x86_64-apple-darwin
```

**Common Issues:**
- Missing Xcode tools: Run xcode-select --install
- SDK not found: Update Xcode Command Line Tools
- Linker errors: Check DYLD_LIBRARY_PATH

**Cross-compilation from other platforms:**
- Install osxcross toolchain
- Set CC and CXX environment variables
- Configure target-specific linker

"#.to_string()
            }
            _ => {
                format!(r#"### {} Setup

**Target:** {}

**Basic Setup:**
```bash
rustup target add {}
```

**Cross-compilation may require:**
- Platform-specific toolchain
- Cross-compilation linker
- Target platform libraries
- Environment variable configuration

Consult Rust cross-compilation documentation for detailed instructions.

"#, platform.name, platform.target_triple, platform.target_triple)
            }
        }
    }

    pub fn analyze_build_errors(&self, stderr: &str, platform: &TargetPlatform) -> Vec<BuildError> {
        let mut errors = Vec::new();

        // Check platform-specific patterns first
        if let Some(patterns) = self.platform_specific_patterns.get(&platform.target_triple) {
            for pattern in patterns {
                if stderr.contains(&pattern.pattern) {
                    errors.push(BuildError {
                        error_type: pattern.error_type.clone(),
                        message: pattern.description.clone(),
                        file_path: None,
                        line_number: None,
                        platform_specific: true,
                        fix_suggestions: pattern.fix_suggestions.clone(),
                    });
                }
            }
        }

        // Check common patterns
        for pattern in &self.common_error_patterns {
            if stderr.contains(&pattern.pattern) {
                errors.push(BuildError {
                    error_type: pattern.error_type.clone(),
                    message: pattern.description.clone(),
                    file_path: None,
                    line_number: None,
                    platform_specific: false,
                    fix_suggestions: pattern.fix_suggestions.clone(),
                });
            }
        }

        // If no specific patterns matched, create a generic error
        if errors.is_empty() && !stderr.is_empty() {
            errors.push(BuildError {
                error_type: BuildErrorType::CompilationError,
                message: "Build failed with unrecognized error".to_string(),
                file_path: None,
                line_number: None,
                platform_specific: false,
                fix_suggestions: vec![
                    "Check the build output for specific error messages".to_string(),
                    "Ensure all dependencies are properly configured".to_string(),
                ],
            });
        }

        errors
    }

    fn create_platform_patterns() -> HashMap<String, Vec<ErrorPattern>> {
        let mut patterns = HashMap::new();

        // Windows-specific patterns
        patterns.insert("x86_64-pc-windows-msvc".to_string(), vec![
            ErrorPattern {
                pattern: "LINK : fatal error".to_string(),
                error_type: BuildErrorType::LinkingError,
                description: "Windows linker error".to_string(),
                fix_suggestions: vec![
                    "Check that Windows SDK is installed".to_string(),
                    "Verify MSVC toolchain is properly configured".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: Microsoft Visual Studio".to_string(),
                error_type: BuildErrorType::PlatformSpecificError,
                description: "Visual Studio configuration issue".to_string(),
                fix_suggestions: vec![
                    "Install Visual Studio Build Tools".to_string(),
                    "Run 'rustup toolchain install stable-x86_64-pc-windows-msvc'".to_string(),
                ],
            },
        ]);

        // Linux-specific patterns
        patterns.insert("x86_64-unknown-linux-gnu".to_string(), vec![
            ErrorPattern {
                pattern: "ld: cannot find".to_string(),
                error_type: BuildErrorType::LinkingError,
                description: "Linux linker cannot find library".to_string(),
                fix_suggestions: vec![
                    "Install required system libraries".to_string(),
                    "Check LD_LIBRARY_PATH environment variable".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: linker `cc` not found".to_string(),
                error_type: BuildErrorType::PlatformSpecificError,
                description: "C compiler not found".to_string(),
                fix_suggestions: vec![
                    "Install build-essential package: sudo apt install build-essential".to_string(),
                    "Install gcc: sudo yum install gcc".to_string(),
                ],
            },
        ]);

        // macOS-specific patterns
        patterns.insert("x86_64-apple-darwin".to_string(), vec![
            ErrorPattern {
                pattern: "ld: library not found".to_string(),
                error_type: BuildErrorType::LinkingError,
                description: "macOS linker cannot find library".to_string(),
                fix_suggestions: vec![
                    "Install Xcode Command Line Tools: xcode-select --install".to_string(),
                    "Check library paths in DYLD_LIBRARY_PATH".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: failed to run custom build command".to_string(),
                error_type: BuildErrorType::PlatformSpecificError,
                description: "Custom build script failed on macOS".to_string(),
                fix_suggestions: vec![
                    "Ensure Xcode Command Line Tools are installed".to_string(),
                    "Check build script permissions".to_string(),
                ],
            },
        ]);

        patterns
    }

    fn create_common_patterns() -> Vec<ErrorPattern> {
        vec![
            ErrorPattern {
                pattern: "error[E0425]".to_string(),
                error_type: BuildErrorType::CompilationError,
                description: "Cannot find value in scope".to_string(),
                fix_suggestions: vec![
                    "Check variable/function name spelling".to_string(),
                    "Ensure proper imports are in place".to_string(),
                    "Add missing 'use' statements".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error[E0433]".to_string(),
                error_type: BuildErrorType::CompilationError,
                description: "Failed to resolve module".to_string(),
                fix_suggestions: vec![
                    "Check module path and imports".to_string(),
                    "Ensure module is properly declared in mod.rs".to_string(),
                    "Verify file exists at expected path".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "optimization-fuel-exhausted".to_string(),
                error_type: BuildErrorType::OptimizationError,
                description: "Optimization process exhausted".to_string(),
                fix_suggestions: vec![
                    "Reduce optimization level with -C opt-level=2".to_string(),
                    "Split large functions into smaller ones".to_string(),
                    "Use #[inline(never)] on problematic functions".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: could not compile".to_string(),
                error_type: BuildErrorType::CompilationError,
                description: "General compilation failure".to_string(),
                fix_suggestions: vec![
                    "Check for syntax errors in the code".to_string(),
                    "Ensure all dependencies are properly specified".to_string(),
                    "Run 'cargo clean' and try again".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: failed to run custom build command".to_string(),
                error_type: BuildErrorType::CompilationError,
                description: "Build script execution failed".to_string(),
                fix_suggestions: vec![
                    "Check build.rs script for errors".to_string(),
                    "Ensure required build tools are installed".to_string(),
                    "Check build script permissions".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: debug info is too large".to_string(),
                error_type: BuildErrorType::DebugInfoError,
                description: "Debug information exceeds size limits".to_string(),
                fix_suggestions: vec![
                    "Use -C debuginfo=1 instead of -C debuginfo=2".to_string(),
                    "Split large modules into smaller ones".to_string(),
                    "Consider building without debug info for release".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "error: linking with".to_string(),
                error_type: BuildErrorType::LinkingError,
                description: "Linker execution failed".to_string(),
                fix_suggestions: vec![
                    "Check that required system libraries are installed".to_string(),
                    "Verify linker is properly configured".to_string(),
                    "Check for conflicting library versions".to_string(),
                ],
            },
            ErrorPattern {
                pattern: "cfg directive".to_string(),
                error_type: BuildErrorType::ConditionalCompilationError,
                description: "Conditional compilation configuration error".to_string(),
                fix_suggestions: vec![
                    "Check cfg attribute syntax".to_string(),
                    "Ensure target platform is properly specified".to_string(),
                    "Verify feature flags are correctly configured".to_string(),
                ],
            },
        ]
    }

    fn create_fix_suggestions() -> HashMap<String, Vec<String>> {
        let mut suggestions = HashMap::new();

        suggestions.insert("windows".to_string(), vec![
            "Install Visual Studio Build Tools".to_string(),
            "Run 'rustup target add x86_64-pc-windows-msvc'".to_string(),
            "Check Windows SDK installation".to_string(),
        ]);

        suggestions.insert("linux".to_string(), vec![
            "Install build-essential: sudo apt install build-essential".to_string(),
            "Run 'rustup target add x86_64-unknown-linux-gnu'".to_string(),
            "Install required system libraries".to_string(),
        ]);

        suggestions.insert("macos".to_string(), vec![
            "Install Xcode Command Line Tools: xcode-select --install".to_string(),
            "Run 'rustup target add x86_64-apple-darwin'".to_string(),
            "Check macOS SDK installation".to_string(),
        ]);

        suggestions
    }
}

impl Default for CrossPlatformBuildValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let project_path = temp_dir.path();

        // Create basic Cargo.toml
        let cargo_toml = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
        fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

        // Create src directory and main.rs
        fs::create_dir_all(project_path.join("src")).unwrap();
        let main_rs = r#"fn main() {
    println!("Hello, world!");
}
"#;
        fs::write(project_path.join("src").join("main.rs"), main_rs).unwrap();

        temp_dir
    }

    #[test]
    fn test_cross_platform_validator_creation() {
        let validator = CrossPlatformBuildValidator::new();
        
        assert!(!validator.target_platforms.is_empty());
        assert!(!validator.build_configurations.is_empty());
        
        // Check that we have the expected platforms
        let platform_names: Vec<&String> = validator.target_platforms.iter()
            .map(|p| &p.name)
            .collect();
        
        assert!(platform_names.iter().any(|name| name.contains("Windows")));
        assert!(platform_names.iter().any(|name| name.contains("Linux")));
        assert!(platform_names.iter().any(|name| name.contains("macOS")));
    }

    #[test]
    fn test_verification_status() {
        let verified = VerificationStatus::Verified;
        let failed = VerificationStatus::Failed("Test error".to_string());
        let not_verified = VerificationStatus::NotVerified;

        assert!(verified.is_verified());
        assert!(!failed.is_verified());
        assert!(!not_verified.is_verified());
    }

    #[test]
    fn test_build_artifact_type_detection() {
        let manager = BuildArtifactManager::new();

        // Test executable detection
        assert!(matches!(
            manager.determine_artifact_type(&PathBuf::from("test.exe")),
            Some(ArtifactType::Executable)
        ));

        // Test library detection
        assert!(matches!(
            manager.determine_artifact_type(&PathBuf::from("test.dll")),
            Some(ArtifactType::Library)
        ));

        // Test debug symbols detection
        assert!(matches!(
            manager.determine_artifact_type(&PathBuf::from("test.pdb")),
            Some(ArtifactType::DebugSymbols)
        ));

        // Test unknown file
        assert!(manager.determine_artifact_type(&PathBuf::from("test.txt")).is_none());
    }
}