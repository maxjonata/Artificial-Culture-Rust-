use std::env;
use std::path::Path;
use std::process;

use artificial_society::cicd::validation::{
    ValidationOrchestrator, CheckType,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <check-type> [files...] [--auto-fix] [--bypass <reason>]", args[0]);
        eprintln!("Check types: pre-commit, pre-push, performance-regression");
        eprintln!("Examples:");
        eprintln!("  {} pre-commit src/ai/cognition/personality.rs", args[0]);
        eprintln!("  {} pre-push src/ai/ --auto-fix", args[0]);
        eprintln!("  {} pre-commit src/ --bypass \"EMERGENCY_COMMIT: Critical production fix\"", args[0]);
        process::exit(1);
    }

    let check_type_str = &args[1];
    let mut files = Vec::new();
    let mut auto_fix = false;
    let mut bypass_reason = None;
    
    // Parse arguments
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--auto-fix" => auto_fix = true,
            "--bypass" => {
                if i + 1 < args.len() {
                    bypass_reason = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --bypass requires a reason");
                    process::exit(1);
                }
            }
            arg if !arg.starts_with("--") => {
                files.push(Path::new(arg));
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }

    // Default to current directory if no files specified
    if files.is_empty() {
        files.push(Path::new("."));
    }

    let mut orchestrator = ValidationOrchestrator::new();

    // Check for bypass
    let check_type = match check_type_str {
        "pre-commit" => CheckType::PreCommit,
        "pre-push" => CheckType::PrePush,
        "performance-regression" => CheckType::ContinuousIntegration,
        _ => {
            eprintln!("Invalid check type: {}. Use pre-commit, pre-push, or performance-regression", check_type_str);
            process::exit(1);
        }
    };

    if let Some(reason) = &bypass_reason {
        if orchestrator.should_bypass_validation(check_type.clone(), reason) {
            println!("🚨 Validation bypassed: {}", reason);
            println!("⚠️  Remember to address issues after the emergency situation is resolved.");
            process::exit(0);
        } else {
            eprintln!("❌ Bypass not allowed for reason: {}", reason);
            process::exit(1);
        }
    }

    // Collect all Rust files if directories are specified
    let mut rust_files = Vec::new();
    for file_path in &files {
        if file_path.is_dir() {
            collect_rust_files(file_path, &mut rust_files);
        } else if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            rust_files.push(file_path);
        }
    }

    // Run validation based on check type
    let result = match check_type {
        CheckType::PreCommit => {
            println!("🚀 Running pre-commit validation (FAST - Essential Only)...");
            orchestrator.run_pre_commit_validation(&rust_files)
        }
        CheckType::PrePush => {
            println!("🔒 Running pre-push validation (COMPREHENSIVE - Everything)...");
            orchestrator.run_pre_push_validation(&rust_files)
        }
        CheckType::ContinuousIntegration => {
            println!("📊 Running performance regression detection...");
            orchestrator.run_performance_regression_check()
        }
        _ => {
            eprintln!("Unsupported check type for orchestrator");
            process::exit(1);
        }
    };

    // Print the complete report
    result.print_report();

    // Execute automatic fixes if requested and available
    if auto_fix && result.can_auto_fix {
        println!("\n🤖 Executing automatic fixes...");
        
        let all_issues: Vec<_> = result.results.iter()
            .flat_map(|r| r.issues.iter())
            .cloned()
            .collect();
        
        let auto_fix_result = orchestrator.execute_automatic_fixes(&all_issues);
        
        println!("   Attempted: {}", auto_fix_result.total_attempted);
        println!("   Successful: {}", auto_fix_result.successful_fixes);
        println!("   Failed: {}", auto_fix_result.failed_fixes);
        
        if auto_fix_result.successful_fixes > 0 {
            println!("\n✅ Automatic fixes applied. Please review changes and re-run validation.");
        }
        
        if !auto_fix_result.failed_issues.is_empty() {
            println!("\n❌ Some automatic fixes failed:");
            for (issue, error) in &auto_fix_result.failed_issues {
                println!("   {}:{} - {}", issue.file_path.display(), issue.line_number, error);
            }
        }
    }

    // Exit with appropriate code
    if result.passed() {
        println!("\n🎉 All validation checks passed!");
        process::exit(0);
    } else {
        println!("\n❌ Validation failed with {} issues.", result.total_issues());
        
        if result.has_blocking_issues {
            println!("🚨 Critical or high priority issues must be fixed before proceeding.");
        } else {
            println!("⚠️  Medium/low priority issues found. Consider fixing before push.");
        }
        
        process::exit(1);
    }
}

fn collect_rust_files(dir: &Path, files: &mut Vec<&Path>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip target and .git directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name != "target" && name != ".git" {
                        collect_rust_files(&path, files);
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // This is a bit of a hack to convert PathBuf to &Path
                // In a real implementation, we'd handle this differently
                files.push(Box::leak(path.into_boxed_path()));
            }
        }
    }
}