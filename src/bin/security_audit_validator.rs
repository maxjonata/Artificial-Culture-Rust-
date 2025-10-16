use artificial_culture_rust::cicd::validation::security_audit::{
    SecurityAuditSystem, SecurityAuditConfig, SecurityAuditError,
};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <project_path> [--strict]", args[0]);
        eprintln!("  --strict: Run in strict mode (failures block build)");
        process::exit(1);
    }
    
    let project_path = Path::new(&args[1]);
    let strict_mode = args.contains(&"--strict".to_string());
    
    // Configure security audit
    let mut config = SecurityAuditConfig::default();
    config.warning_only_mode = !strict_mode;
    
    let audit_system = SecurityAuditSystem::new(config);
    
    println!("🔒 Running security audit on: {}", project_path.display());
    if strict_mode {
        println!("⚠️  Running in STRICT mode - failures will block build");
    } else {
        println!("ℹ️  Running in WARNING-ONLY mode - failures will not block build");
    }
    println!();
    
    match audit_system.run_security_audit(project_path) {
        Ok(result) => {
            println!("{}", result.format_report());
            
            if !result.passed && strict_mode {
                eprintln!("❌ Security audit failed in strict mode");
                process::exit(1);
            } else if !result.passed {
                eprintln!("⚠️  Security audit found issues (warning-only mode)");
                process::exit(0);
            } else {
                println!("✅ Security audit passed");
                process::exit(0);
            }
        }
        Err(e) => {
            eprintln!("❌ Security audit error: {}", e);
            
            match e {
                SecurityAuditError::ToolNotInstalled(tool) => {
                    eprintln!();
                    eprintln!("💡 To install {}:", tool);
                    match tool.as_str() {
                        "cargo-audit" => {
                            eprintln!("   cargo install cargo-audit");
                        }
                        "cargo-outdated" => {
                            eprintln!("   cargo install cargo-outdated");
                        }
                        _ => {
                            eprintln!("   Please install {} manually", tool);
                        }
                    }
                }
                _ => {}
            }
            
            if strict_mode {
                process::exit(1);
            } else {
                eprintln!("⚠️  Continuing in warning-only mode");
                process::exit(0);
            }
        }
    }
}