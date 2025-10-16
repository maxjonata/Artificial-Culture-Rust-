use std::path::PathBuf;
use std::env;
use clap::{Parser, Subcommand};

use artificial_culture_rust::cicd::setup::{
    CiCdSetupManager, 
    SetupConfiguration, 
    SetupLevel,
};

/// CI/CD Setup and Installation Tool
#[derive(Parser)]
#[command(name = "cicd-setup")]
#[command(about = "Automated CI/CD setup and installation system")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Repository path (defaults to current directory)
    #[arg(short, long)]
    path: Option<PathBuf>,
    
    /// Setup level
    #[arg(short, long, default_value = "standard")]
    level: SetupLevel,
    
    /// Skip automatic tool installation
    #[arg(long)]
    no_auto_install: bool,
    
    /// Skip Git hooks installation
    #[arg(long)]
    no_hooks: bool,
    
    /// Skip configuration file creation
    #[arg(long)]
    no_config: bool,
    
    /// Skip initial validation
    #[arg(long)]
    no_validation: bool,
    
    /// Skip documentation generation
    #[arg(long)]
    no_docs: bool,
    
    /// Custom configuration file path
    #[arg(long)]
    config_path: Option<PathBuf>,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run complete automated setup
    Setup,
    /// Check dependencies only
    CheckDeps,
    /// Install Git hooks only
    InstallHooks,
    /// Create configuration files only
    CreateConfig,
    /// Validate existing setup
    Validate,
    /// Generate documentation only
    GenerateDocs,
    /// Show setup status
    Status,
}

impl std::str::FromStr for SetupLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(SetupLevel::Minimal),
            "standard" => Ok(SetupLevel::Standard),
            "development" | "dev" => Ok(SetupLevel::Development),
            "production" | "prod" => Ok(SetupLevel::Production),
            _ => Err(format!("Invalid setup level: {}. Valid options: minimal, standard, development, production", s)),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Determine repository path
    let repo_path = cli.path.unwrap_or_else(|| env::current_dir().unwrap());
    
    // Create setup manager
    let mut setup_manager = CiCdSetupManager::new(repo_path.clone())?;
    
    // Create setup configuration
    let setup_config = SetupConfiguration {
        auto_install_hooks: !cli.no_hooks,
        auto_install_tools: !cli.no_auto_install,
        create_default_config: !cli.no_config,
        run_initial_validation: !cli.no_validation,
        generate_documentation: !cli.no_docs,
        setup_level: cli.level,
        custom_config_path: cli.config_path,
    };
    
    match cli.command.unwrap_or(Commands::Setup) {
        Commands::Setup => {
            println!("🚀 Starting CI/CD setup for repository: {}", repo_path.display());
            println!("📋 Setup level: {:?}", setup_config.setup_level);
            
            let report = setup_manager.run_automated_setup(setup_config).await?;
            
            print_setup_report(&report, cli.verbose);
            
            if report.success {
                println!("\n✅ CI/CD setup completed successfully!");
                println!("🎉 Your repository is now ready for development with automated quality gates.");
                println!("\n📚 Next steps:");
                println!("   1. Review the generated documentation in docs/");
                println!("   2. Make a test commit to verify hooks are working");
                println!("   3. Customize .cicd-config.toml if needed");
            } else {
                println!("\n⚠️  CI/CD setup completed with issues.");
                println!("📋 Review the errors above and run setup again if needed.");
                std::process::exit(1);
            }
        }
        
        Commands::CheckDeps => {
            println!("🔍 Checking dependencies...");
            
            let step_result = setup_manager.check_dependencies().await?;
            print_step_result("Dependency Check", &step_result, cli.verbose);
            
            if !step_result.success {
                std::process::exit(1);
            }
        }
        
        Commands::InstallHooks => {
            println!("🪝 Installing Git hooks...");
            
            let step_result = setup_manager.install_git_hooks().await?;
            print_step_result("Git Hooks Installation", &step_result, cli.verbose);
            
            if !step_result.success {
                std::process::exit(1);
            }
        }
        
        Commands::CreateConfig => {
            println!("⚙️  Creating configuration files...");
            
            let step_result = setup_manager.create_configuration_files(&setup_config).await?;
            print_step_result("Configuration Creation", &step_result, cli.verbose);
            
            if !step_result.success {
                std::process::exit(1);
            }
        }
        
        Commands::Validate => {
            println!("✅ Validating setup...");
            
            let step_result = setup_manager.validate_setup().await?;
            print_step_result("Setup Validation", &step_result, cli.verbose);
            
            if !step_result.success {
                std::process::exit(1);
            }
        }
        
        Commands::GenerateDocs => {
            println!("📚 Generating documentation...");
            
            let step_result = setup_manager.generate_documentation().await?;
            print_step_result("Documentation Generation", &step_result, cli.verbose);
            
            if !step_result.success {
                std::process::exit(1);
            }
        }
        
        Commands::Status => {
            println!("📊 CI/CD Setup Status");
            println!("Repository: {}", repo_path.display());
            
            // Check basic status
            let git_dir = repo_path.join(".git");
            let config_file = repo_path.join(".cicd-config.toml");
            let hooks_dir = git_dir.join("hooks");
            
            println!("\n🔍 Basic Checks:");
            println!("   Git repository: {}", if git_dir.exists() { "✅" } else { "❌" });
            println!("   Configuration file: {}", if config_file.exists() { "✅" } else { "❌" });
            println!("   Hooks directory: {}", if hooks_dir.exists() { "✅" } else { "❌" });
            
            // Check hooks
            println!("\n🪝 Git Hooks:");
            let pre_commit = hooks_dir.join("pre-commit");
            let pre_push = hooks_dir.join("pre-push");
            println!("   Pre-commit hook: {}", if pre_commit.exists() { "✅" } else { "❌" });
            println!("   Pre-push hook: {}", if pre_push.exists() { "✅" } else { "❌" });
            
            // Check dependencies
            println!("\n🔧 Dependencies:");
            let mut setup_manager = CiCdSetupManager::new(repo_path)?;
            let dep_result = setup_manager.check_dependencies().await?;
            
            for tool in &setup_manager.dependency_checker.required_tools {
                println!("   {}: {}", tool.name, if tool.is_available { "✅" } else { "❌" });
            }
            
            for component in &setup_manager.dependency_checker.rust_components {
                println!("   {}: {}", component.name, if component.is_installed { "✅" } else { "❌" });
            }
            
            if !dep_result.success {
                println!("\n💡 Run 'cicd-setup check-deps' for detailed dependency information");
            }
        }
    }
    
    Ok(())
}

fn print_setup_report(report: &artificial_culture_rust::cicd::setup::SetupReport, verbose: bool) {
    println!("\n📊 Setup Report");
    println!("Overall success: {}", if report.success { "✅" } else { "❌" });
    
    if !report.errors.is_empty() {
        println!("\n❌ Errors:");
        for error in &report.errors {
            println!("   • {}", error);
        }
    }
    
    println!("\n📋 Step Results:");
    for (step, result) in &report.step_results {
        let status = if result.success { "✅" } else { "❌" };
        println!("   {} {:?}: {} ({:.2}s)", status, step, result.message, result.duration.as_secs_f32());
        
        if verbose || !result.success {
            for detail in &result.details {
                println!("      • {}", detail);
            }
            
            if !result.suggestions.is_empty() {
                println!("      💡 Suggestions:");
                for suggestion in &result.suggestions {
                    println!("         - {}", suggestion);
                }
            }
        }
    }
}

fn print_step_result(step_name: &str, result: &artificial_culture_rust::cicd::setup::StepResult, verbose: bool) {
    let status = if result.success { "✅" } else { "❌" };
    println!("{} {}: {} ({:.2}s)", status, step_name, result.message, result.duration.as_secs_f32());
    
    if verbose || !result.success {
        for detail in &result.details {
            println!("   • {}", detail);
        }
        
        if !result.suggestions.is_empty() {
            println!("   💡 Suggestions:");
            for suggestion in &result.suggestions {
                println!("      - {}", suggestion);
            }
        }
    }
}