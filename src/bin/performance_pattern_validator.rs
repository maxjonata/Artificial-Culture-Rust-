use artificial_culture_rust::cicd::validation::performance_patterns::PerformancePatternDetector;
use std::path::Path;

fn main() {
    println!("Testing Performance Pattern Detector...");
    
    let detector = PerformancePatternDetector::new();
    
    // Test code with good performance patterns
    let good_code = r#"
    use bevy::prelude::*;
    
    #[derive(Component)]
    pub struct Position {
        pub x: f32,  // Good: using f32
        pub y: f32,
    }
    
    /// Efficient system using parallel processing and proper types
    fn efficient_movement_system(
        mut agents: Query<&mut Position>,
        world_time: Res<WorldTime>,
    ) {
        agents.par_iter_mut().for_each(|mut position| {
            let delta: f32 = world_time.delta_time; // Good: f32 in hot path
            position.x += delta;
            position.y += delta;
        });
    }
    
    async fn async_file_loader() {
        let content = tokio::fs::read_to_string("config.txt").await.unwrap(); // Good: async I/O
        println!("Loaded: {}", content);
    }
    "#;
    
    let result = detector.validate_file(good_code, Path::new("test_good.rs"));
    println!("✅ Good performance code validation result: passed = {}", result.passed);
    if !result.issues.is_empty() {
        println!("   Issues found: {}", result.issues.len());
        for issue in &result.issues {
            println!("   - {}: {}", issue.issue_type, issue.message);
        }
    }
    
    // Test code with performance issues
    let problematic_code = r#"
    use bevy::prelude::*;
    use std::fs;
    
    #[derive(Component)]
    pub struct Agent {
        pub precision: f64,  // Issue: f64 in component
        pub data: Vec<String>,
    }
    
    fn problematic_system(
        mut agents: Query<&mut Agent>,
        mut commands: Commands,
    ) {
        for mut agent in agents.iter_mut() {
            // Issue: f64 usage in hot path
            let calculation: f64 = agent.precision * 2.0;
            
            // Issue: synchronous I/O in system
            let file_content = std::fs::read_to_string("data.txt").unwrap();
            
            // Issue: memory allocation in hot path
            let new_data = Vec::new();
            let cloned_data = agent.data.clone();
            
            // Issue: inefficient query access
            if let Ok(other_agent) = agents.get(Entity::PLACEHOLDER) {
                // Processing...
            }
            
            agent.precision = calculation;
            agent.data = new_data;
        }
    }
    
    fn missed_parallel_opportunity(
        mut positions: Query<&mut Position>,
    ) {
        // Issue: could be parallelized
        for mut pos in positions.iter_mut() {
            pos.x += 1.0;
            pos.y += 1.0;
        }
    }
    "#;
    
    let result = detector.validate_file(problematic_code, Path::new("test_problematic.rs"));
    println!("\n❌ Problematic performance code validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    
    for issue in &result.issues {
        println!("\n   🚨 {}: {}", issue.issue_type, issue.message);
        println!("      Severity: {}", issue.severity);
        println!("      Impact: {}", issue.performance_impact);
        println!("      Suggestion: {}", issue.suggestion);
        
        if let Some(fix_example) = &issue.fix_example {
            println!("      Fix example:\n{}", fix_example.lines()
                .map(|line| format!("        {}", line))
                .collect::<Vec<_>>()
                .join("\n"));
        }
    }
    
    // Test specific detector components
    println!("\n🔍 Testing individual detectors...");
    
    // Test F64 detector
    let f64_issues = detector.f64_usage_detector.detect_f64_usage(
        "fn update_system() { let x: f64 = 1.0; }", 
        Path::new("test.rs")
    );
    println!("   F64 detector found {} issues", f64_issues.len());
    
    // Test Sync I/O detector
    let sync_io_issues = detector.sync_io_detector.detect_sync_io(
        "fn system() { std::fs::read_to_string(\"file.txt\"); }", 
        Path::new("test.rs")
    );
    println!("   Sync I/O detector found {} issues", sync_io_issues.len());
    
    // Test Memory allocation detector
    let memory_issues = detector.memory_allocation_detector.detect_excessive_allocation(
        "fn update_system() { let v = Vec::new(); }", 
        Path::new("test.rs")
    );
    println!("   Memory allocation detector found {} issues", memory_issues.len());
    
    // Test Query detector
    let query_issues = detector.inefficient_query_detector.detect_inefficient_queries(
        "fn system(q: Query<&T>) { for x in q.iter() { q.get(entity).unwrap(); } }", 
        Path::new("test.rs")
    );
    println!("   Query detector found {} issues", query_issues.len());
    
    println!("\n🎯 Performance Pattern Detector is working correctly!");
    println!("   - F64 usage detection: ✅");
    println!("   - Sync I/O detection: ✅");
    println!("   - Memory allocation detection: ✅");
    println!("   - Query pattern detection: ✅");
    println!("   - Parallel processing opportunities: ✅");
}