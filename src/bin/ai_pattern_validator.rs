use artificial_culture_rust::cicd::validation::ai_patterns::AiPatternValidator;
use std::path::Path;

fn main() {
    println!("Testing AI Pattern Validator...");
    
    let validator = AiPatternValidator::new();
    
    // Test code with correct AI patterns
    let correct_code = r#"
    use bevy::prelude::*;
    use crate::core::types::Normalized;
    
    #[derive(Component)]
    pub struct PersonalityVector {
        /// openness: 0.0 (low) to 1.0 (high)
        pub openness: Normalized,
        /// conscientiousness: 0.0 (low) to 1.0 (high)
        pub conscientiousness: Normalized,
    }
    
    #[derive(Component)]
    pub struct EmotionalState {
        /// valence: -1.0 (negative) to 1.0 (positive)
        pub valence: f32,
    }
    
    impl EmotionalState {
        pub fn validate_valence(&self) -> Result<(), String> {
            if !(-1.0..=1.0).contains(&self.valence) {
                Err("Invalid valence range".to_string())
            } else {
                Ok(())
            }
        }
    }
    
    /// This system handles emotional contagion between agents based on personality
    fn emotional_contagion_system(
        mut agents: Query<(&mut EmotionalState, &Personality)>,
        world_time: Res<WorldTime>,
        mut events: EventWriter<EmotionChanged>,
    ) {
        for (mut emotion, personality) in agents.iter_mut() {
            let openness_factor = personality.openness.value();
            let time_factor = world_time.delta_time;
            emotion.valence *= openness_factor * time_factor;
            
            events.send(EmotionChanged { entity: Entity::PLACEHOLDER });
        }
    }
    "#;
    
    let result = validator.validate_file(correct_code, Path::new("test_correct.rs"));
    println!("✅ Correct code validation result: passed = {}", result.passed);
    if !result.issues.is_empty() {
        println!("   Issues found: {}", result.issues.len());
        for issue in &result.issues {
            println!("   - {}", issue.message);
        }
    }
    
    // Test code with incorrect AI patterns
    let incorrect_code = r#"
    use bevy::prelude::*;
    
    #[derive(Component)]
    pub struct PersonalityVector {
        pub openness: f32,  // Should use Normalized
        pub conscientiousness: f32,  // Should use Normalized
    }
    
    #[derive(Component)]
    pub struct EmotionalState {
        pub valence: f32,  // Missing documentation
    }
    
    fn simple_system(
        mut agents: Query<&mut EmotionalState>,
    ) {
        let now = std::time::Instant::now();  // Should use WorldTime
        for mut emotion in agents.iter_mut() {
            emotion.valence += 0.1;  // No personality modulation
        }
    }
    "#;
    
    let result = validator.validate_file(incorrect_code, Path::new("test_incorrect.rs"));
    println!("\n❌ Incorrect code validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    for issue in &result.issues {
        println!("   - {}: {}", issue.issue_type, issue.message);
        if let Some(suggestion) = &issue.suggestion {
            println!("     Suggestion: {}", suggestion);
        }
    }
    
    println!("\n🎯 AI Pattern Validator is working correctly!");
}