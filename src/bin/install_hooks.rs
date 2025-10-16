use artificial_culture_rust::cicd::{
    ConsoleProgressReporter, DefaultErrorFormatter, GitHookManager,
};
use std::env;
use std::path::Path;

fn main() {
    println!("🔧 Installing Git hooks for Artificial Society CI/CD...");

    // Get repository path from command line or use current directory
    let repo_path = env::args()
        .nth(1)
        .map(|p| Path::new(&p).to_path_buf())
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    println!("📁 Repository path: {}", repo_path.display());

    // Create hook manager with default configuration
    let mut hook_manager = GitHookManager::new();

    // Add progress reporting and error formatting
    hook_manager.add_progress_reporter(Box::new(ConsoleProgressReporter));
    hook_manager.add_error_formatter(Box::new(DefaultErrorFormatter));

    // Install hooks
    match hook_manager.install_hooks(&repo_path) {
        Ok(()) => {
            println!("✅ Git hooks installed successfully!");
            println!();
            println!("📋 Installed hooks:");
            println!("  • pre-commit: Fast essential checks (formatting, clippy, tests)");
            println!("  • pre-push: Comprehensive validation (build, patterns, full tests)");
            println!();
            println!("🚨 Emergency bypass options:");
            println!("  • Environment variable: EMERGENCY_BYPASS=1");
            println!("  • Commit message keywords: BYPASS_PRECOMMIT, BYPASS_PREPUSH");
            println!("  • Environment variable: SKIP_HOOKS=1");
            println!();
            println!("🎯 Performance targets:");
            println!("  • Pre-commit: <45 seconds total");
            println!("  • Pre-push: <10 minutes total");
        }
        Err(e) => {
            eprintln!("❌ Failed to install Git hooks: {}", e);
            std::process::exit(1);
        }
    }
}
