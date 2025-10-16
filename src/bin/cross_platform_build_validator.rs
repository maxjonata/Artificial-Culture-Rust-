use std::env;
use std::path::PathBuf;
use std::process;

use artificial_society::cicd::validation::cross_platform_builds::{
    CrossPlatformBuildValidator, BuildResult, ConditionalCompilationResult,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        eprintln!("Commands:");
        eprintln!("  validate-all [project-path]      - Validate builds for all platforms");
        eprintln!("  validate-platform <platform>     - Validate build for specific platform");
        eprintln!("  validate-conditional [path]      - Validate conditional compilation");
        eprintln!("  check-setup                      - Check cross-compilation setup");
        eprintln!("  generate-artifacts [project-path] - Generate distribution artifacts");
        eprintln!("  generate-report [project-path]   - Generate comprehensive build report");
        eprintln!("  troubleshoot <platform> [path]  - Generate troubleshooting guide");
        process::exit(1);
    }

    let command = &args[1];
    let validator = CrossPlatformBuildValidator::new();

    match command.as_str() {
        "validate-all" => {
            let project_path = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            println!("🚀 Starting cross-platform build validation...");
            println!("📁 Project path: {}", project_path.display());
            
            let results = validator.validate_all_platforms(&project_path);
            let success = handle_build_results(results);
            
            if success {
                println!("✅ All cross-platform builds passed!");
                process::exit(0);
            } else {
                println!("❌ Some cross-platform builds failed!");
                process::exit(1);
            }
        }
        
        "validate-platform" => {
            if args.len() < 3 {
                eprintln!("Error: Platform name required");
                eprintln!("Available platforms: windows, linux, macos");
                process::exit(1);
            }

            let platform_name = &args[2];
            let project_path = if args.len() > 3 {
                PathBuf::from(&args[3])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            println!("🏗️  Validating build for platform: {}", platform_name);
            
            // Find the specified platform
            let platform = validator.target_platforms.iter()
                .find(|p| p.name.to_lowercase().contains(&platform_name.to_lowercase()))
                .cloned();

            if let Some(platform) = platform {
                let mut all_success = true;
                
                for config in &validator.build_configurations {
                    println!("Building {} configuration for {}...", config.name, platform.name);
                    let result = validator.build_for_platform(&project_path, &platform, config);
                    
                    if !result.success {
                        all_success = false;
                    }
                    
                    print_build_result(&result);
                }

                if all_success {
                    println!("✅ Platform {} validation passed!", platform_name);
                    process::exit(0);
                } else {
                    println!("❌ Platform {} validation failed!", platform_name);
                    process::exit(1);
                }
            } else {
                eprintln!("Error: Unknown platform '{}'", platform_name);
                eprintln!("Available platforms:");
                for p in &validator.target_platforms {
                    eprintln!("  - {}", p.name);
                }
                process::exit(1);
            }
        }
        
        "validate-conditional" => {
            let project_path = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            println!("🔍 Validating conditional compilation...");
            
            let results = validator.validate_conditional_compilation(&project_path);
            let success = handle_conditional_results(results);
            
            if success {
                println!("✅ Conditional compilation validation passed!");
                process::exit(0);
            } else {
                println!("❌ Conditional compilation validation failed!");
                process::exit(1);
            }
        }
        
        "check-setup" => {
            println!("🔧 Checking cross-compilation setup...");
            
            let mut all_good = true;
            
            for platform in &validator.target_platforms {
                println!("\n📋 Platform: {}", platform.name);
                println!("   Target: {}", platform.target_triple);
                println!("   Host platform: {}", platform.is_host_platform);
                
                if let Some(cross_setup) = &platform.cross_compile_setup {
                    println!("   Cross-compilation required:");
                    
                    if let Some(linker) = &cross_setup.linker {
                        println!("     Linker: {}", linker);
                    }
                    
                    for tool in &cross_setup.required_tools {
                        print!("     Tool '{}': ", tool);
                        
                        let check_command = if cfg!(target_os = "windows") {
                            std::process::Command::new("where").arg(tool).output()
                        } else {
                            std::process::Command::new("which").arg(tool).output()
                        };

                        match check_command {
                            Ok(output) if output.status.success() => {
                                println!("✅ Available");
                            }
                            _ => {
                                println!("❌ Not found");
                                all_good = false;
                            }
                        }
                    }
                } else {
                    println!("   ✅ No cross-compilation setup required");
                }
            }
            
            if all_good {
                println!("\n✅ Cross-compilation setup is complete!");
                process::exit(0);
            } else {
                println!("\n❌ Cross-compilation setup is incomplete!");
                println!("\n🔧 Setup instructions:");
                println!("   Windows: Install Visual Studio Build Tools or MinGW-w64");
                println!("   Linux: Install build-essential and cross-compilation tools");
                println!("   macOS: Install Xcode Command Line Tools and osxcross");
                process::exit(1);
            }
        }

        "generate-artifacts" => {
            let project_path = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            println!("📦 Generating distribution artifacts...");
            
            let mut all_success = true;
            
            for platform in &validator.target_platforms {
                for config in &validator.build_configurations {
                    if config.name == "Release" { // Only generate artifacts for release builds
                        println!("Generating artifacts for {} - {}...", platform.name, config.name);
                        
                        match validator.artifact_manager.generate_distribution_artifacts(&project_path, platform, config) {
                            Ok(artifacts) => {
                                println!("✅ Generated {} artifacts for {}", artifacts.len(), platform.name);
                                for artifact in &artifacts {
                                    let size_kb = artifact.size_bytes / 1024;
                                    println!("   - {} ({} KB)", artifact.path.display(), size_kb);
                                }
                            }
                            Err(e) => {
                                println!("❌ Failed to generate artifacts for {}: {}", platform.name, e);
                                all_success = false;
                            }
                        }
                    }
                }
            }
            
            if all_success {
                println!("\n✅ All distribution artifacts generated successfully!");
                process::exit(0);
            } else {
                println!("\n❌ Some artifact generation failed!");
                process::exit(1);
            }
        }

        "generate-report" => {
            let project_path = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            println!("📊 Generating build report...");
            
            // Run validation to get results
            let results = validator.validate_all_platforms(&project_path);
            
            // Generate report
            let report_path = project_path.join("build-report.md");
            match validator.artifact_manager.create_build_report(&results, &report_path) {
                Ok(()) => {
                    println!("✅ Build report generated: {}", report_path.display());
                    process::exit(0);
                }
                Err(e) => {
                    println!("❌ Failed to generate build report: {}", e);
                    process::exit(1);
                }
            }
        }

        "troubleshoot" => {
            if args.len() < 3 {
                eprintln!("Error: Platform name required for troubleshooting");
                eprintln!("Available platforms: windows, linux, macos");
                process::exit(1);
            }

            let platform_name = &args[2];
            let project_path = if args.len() > 3 {
                PathBuf::from(&args[3])
            } else {
                env::current_dir().expect("Failed to get current directory")
            };

            // Find the specified platform
            let platform = validator.target_platforms.iter()
                .find(|p| p.name.to_lowercase().contains(&platform_name.to_lowercase()))
                .cloned();

            if let Some(platform) = platform {
                println!("🔍 Generating troubleshooting guide for {}...", platform.name);
                
                // Try to build and collect errors
                let mut all_errors = Vec::new();
                
                for config in &validator.build_configurations {
                    let result = validator.build_for_platform(&project_path, &platform, config);
                    all_errors.extend(result.errors);
                }

                // Generate troubleshooting guide
                let guide = validator.error_analyzer.generate_troubleshooting_guide(&all_errors, &platform);
                
                // Save guide to file
                let guide_path = project_path.join(format!("troubleshooting-{}.md", platform.target_triple));
                match std::fs::write(&guide_path, &guide) {
                    Ok(()) => {
                        println!("✅ Troubleshooting guide generated: {}", guide_path.display());
                        println!("\n{}", guide);
                        process::exit(0);
                    }
                    Err(e) => {
                        println!("❌ Failed to save troubleshooting guide: {}", e);
                        println!("\n{}", guide);
                        process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: Unknown platform '{}'", platform_name);
                eprintln!("Available platforms:");
                for p in &validator.target_platforms {
                    eprintln!("  - {}", p.name);
                }
                process::exit(1);
            }
        }
        
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            eprintln!("Run with no arguments to see available commands");
            process::exit(1);
        }
    }
}

fn handle_build_results(results: Vec<BuildResult>) -> bool {
    let mut all_success = true;
    
    println!("\n📊 Build Results Summary:");
    println!("========================");
    
    for result in &results {
        print_build_result(result);
        if !result.success {
            all_success = false;
        }
    }
    
    // Print overall statistics
    let total_builds = results.len();
    let successful_builds = results.iter().filter(|r| r.success).count();
    let failed_builds = total_builds - successful_builds;
    
    println!("\n📈 Statistics:");
    println!("   Total builds: {}", total_builds);
    println!("   Successful: {} ✅", successful_builds);
    println!("   Failed: {} ❌", failed_builds);
    
    if failed_builds > 0 {
        println!("\n🔧 Failed Build Details:");
        for result in results.iter().filter(|r| !r.success) {
            println!("\n❌ {} - {}:", result.platform.name, result.configuration.name);
            for error in &result.errors {
                println!("   Error: {}", error.message);
                if !error.fix_suggestions.is_empty() {
                    println!("   Suggestions:");
                    for suggestion in &error.fix_suggestions {
                        println!("     - {}", suggestion);
                    }
                }
            }
        }
    }
    
    all_success
}

fn print_build_result(result: &BuildResult) {
    let status_icon = if result.success { "✅" } else { "❌" };
    let build_time_ms = result.build_time.as_millis();
    
    println!("\n{} {} - {}", status_icon, result.platform.name, result.configuration.name);
    println!("   Build time: {}ms", build_time_ms);
    
    if result.success {
        println!("   Artifacts: {}", result.artifacts.len());
        for artifact in &result.artifacts {
            let size_kb = artifact.size_bytes / 1024;
            println!("     - {} ({} KB)", artifact.path.display(), size_kb);
        }
        
        if !result.warnings.is_empty() {
            println!("   Warnings: {}", result.warnings.len());
            for warning in &result.warnings {
                println!("     ⚠️  {}", warning.message);
            }
        }
    } else {
        println!("   Errors: {}", result.errors.len());
        for error in &result.errors {
            println!("     ❌ {}", error.message);
        }
    }
}

fn handle_conditional_results(results: Vec<ConditionalCompilationResult>) -> bool {
    let mut all_valid = true;
    
    println!("\n📊 Conditional Compilation Results:");
    println!("===================================");
    
    for result in &results {
        let status_icon = if result.valid { "✅" } else { "❌" };
        println!("{} {}", status_icon, result.file_path.display());
        
        if !result.valid {
            all_valid = false;
            for issue in &result.issues {
                println!("   ❌ Line {}: {}", issue.line_number, issue.message);
                println!("      Suggestion: {}", issue.suggestion);
            }
        }
    }
    
    let total_files = results.len();
    let valid_files = results.iter().filter(|r| r.valid).count();
    let invalid_files = total_files - valid_files;
    
    println!("\n📈 Statistics:");
    println!("   Total files: {}", total_files);
    println!("   Valid: {} ✅", valid_files);
    println!("   Invalid: {} ❌", invalid_files);
    
    all_valid
}