use artificial_society::cicd::validation::DocumentationQualityValidator;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file_or_directory> [--all-files]", args[0]);
        eprintln!("  --all-files: Validate all files in the project");
        process::exit(1);
    }

    let validator = DocumentationQualityValidator::new();
    let mut total_issues = 0;
    let mut files_processed = 0;

    if args.contains(&"--all-files".to_string()) {
        // Validate all Rust files in the project
        println!("🔍 Running documentation and code quality validation on ALL files...");
        
        if let Err(e) = validate_directory(&validator, Path::new("src"), &mut total_issues, &mut files_processed) {
            eprintln!("Error validating directory: {}", e);
            process::exit(1);
        }
        
        // Also validate project-level documentation
        let project_result = validator.validate_project_documentation(Path::new("."));
        if !project_result.passed {
            total_issues += project_result.issues.len();
            println!("\n📋 Project Documentation Issues:");
            for issue in &project_result.issues {
                println!("  ❌ {}: {}", issue.file_path.display(), issue.message);
                if let Some(suggestion) = &issue.suggestion {
                    println!("     💡 Suggestion: {}", suggestion);
                }
            }
        }
        
    } else {
        // Validate specific file or directory
        let target_path = Path::new(&args[1]);
        
        if target_path.is_file() {
            if let Err(e) = validate_file(&validator, target_path, &mut total_issues) {
                eprintln!("Error validating file: {}", e);
                process::exit(1);
            }
            files_processed = 1;
        } else if target_path.is_dir() {
            if let Err(e) = validate_directory(&validator, target_path, &mut total_issues, &mut files_processed) {
                eprintln!("Error validating directory: {}", e);
                process::exit(1);
            }
        } else {
            eprintln!("Error: {} is not a valid file or directory", target_path.display());
            process::exit(1);
        }
    }

    // Print summary
    println!("\n📊 Documentation and Code Quality Validation Summary:");
    println!("  Files processed: {}", files_processed);
    println!("  Total issues found: {}", total_issues);
    
    if total_issues > 0 {
        println!("\n❌ Documentation and code quality validation failed!");
        println!("   Please address the issues above to improve code maintainability.");
        process::exit(1);
    } else {
        println!("\n✅ Documentation and code quality validation passed!");
        println!("   All files meet documentation and quality standards.");
    }
}

fn validate_file(validator: &DocumentationQualityValidator, file_path: &Path, total_issues: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
    // Only validate Rust files
    if !file_path.extension().map_or(false, |ext| ext == "rs") {
        return Ok(());
    }

    let content = fs::read_to_string(file_path)?;
    let result = validator.validate_file(&content, file_path);
    
    if !result.passed {
        *total_issues += result.issues.len();
        println!("\n📄 Issues in {}:", file_path.display());
        
        for issue in &result.issues {
            let severity_icon = match issue.severity {
                artificial_society::cicd::validation::IssueSeverity::Critical => "🚨",
                artificial_society::cicd::validation::IssueSeverity::High => "❌",
                artificial_society::cicd::validation::IssueSeverity::Medium => "⚠️",
                artificial_society::cicd::validation::IssueSeverity::Low => "ℹ️",
                artificial_society::cicd::validation::IssueSeverity::Info => "💡",
            };
            
            println!("  {} Line {}: {}", severity_icon, issue.line_number, issue.message);
            
            if let Some(suggestion) = &issue.suggestion {
                println!("     💡 Suggestion: {}", suggestion);
            }
            
            if let Some(example) = &issue.code_example {
                println!("     📝 Example:");
                for line in example.lines() {
                    println!("        {}", line);
                }
            }
        }
        
        if !result.suggestions.is_empty() {
            println!("  📋 General Suggestions:");
            for suggestion in &result.suggestions {
                println!("    • {}", suggestion);
            }
        }
    }
    
    Ok(())
}

fn validate_directory(validator: &DocumentationQualityValidator, dir_path: &Path, total_issues: &mut usize, files_processed: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            validate_file(validator, &path, total_issues)?;
            if path.extension().map_or(false, |ext| ext == "rs") {
                *files_processed += 1;
            }
        } else if path.is_dir() {
            // Skip target directory and hidden directories
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if !dir_name.starts_with('.') && dir_name != "target" {
                    validate_directory(validator, &path, total_issues, files_processed)?;
                }
            }
        }
    }
    
    Ok(())
}