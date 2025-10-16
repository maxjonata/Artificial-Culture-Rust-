use artificial_culture_rust::cicd::validation::PerformanceRegressionDetector;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let project_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().expect("Failed to get current directory")
    };
    
    let baseline_path = project_path.join("target").join("performance_baseline.json");
    
    println!("🔍 Performance Regression Detection");
    println!("Project Path: {}", project_path.display());
    println!("Baseline Path: {}", baseline_path.display());
    println!();
    
    // Create performance regression detector
    let mut detector = PerformanceRegressionDetector::new(baseline_path);
    
    // Run performance regression detection
    let result = detector.detect_regressions(&project_path);
    
    // Report results
    if result.passed {
        println!("✅ Performance regression detection passed!");
        println!("   No performance regressions detected.");
        
        if !result.suggestions.is_empty() {
            println!("\n💡 Optimization suggestions:");
            for suggestion in &result.suggestions {
                println!("   • {}", suggestion);
            }
        }
    } else {
        println!("❌ Performance regression detection failed!");
        println!("   {} issues found:", result.issues.len());
        
        for (i, issue) in result.issues.iter().enumerate() {
            println!("\n{}. {} ({})", i + 1, issue.issue_type, format_severity(&issue.severity));
            println!("   {}", issue.message);
            
            if let Some(suggestion) = &issue.suggestion {
                println!("   💡 Suggestion: {}", suggestion);
            }
        }
        
        if !result.suggestions.is_empty() {
            println!("\n💡 General optimization suggestions:");
            for suggestion in &result.suggestions {
                println!("   • {}", suggestion);
            }
        }
        
        std::process::exit(1);
    }
}

fn format_severity(severity: &artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity) -> &'static str {
    match severity {
        artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity::Critical => "CRITICAL",
        artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity::High => "HIGH",
        artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity::Medium => "MEDIUM",
        artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity::Low => "LOW",
        artificial_culture_rust::cicd::validation::ai_patterns::IssueSeverity::Info => "INFO",
    }
}