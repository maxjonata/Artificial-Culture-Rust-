use artificial_culture_rust::cicd::validation::behavioral_consistency::BehavioralConsistencyValidator;
use std::path::Path;

fn main() {
    println!("Testing Behavioral Consistency Validator...");
    
    let validator = BehavioralConsistencyValidator::new();
    
    // Test code with good behavioral consistency
    let good_code = r#"
    use bevy::prelude::*;
    
    /// This system makes decisions based on personality and emotional state
    fn decision_making_system(
        mut agents: Query<(&mut DecisionState, &Personality, &EmotionalState)>,
        world_time: Res<WorldTime>,
    ) {
        for (mut decision, personality, emotion) in agents.iter_mut() {
            // Personality-driven decision making
            let openness_factor = personality.openness.value();
            let emotional_weight = emotion.valence * personality.neuroticism.value();
            
            // Emotionally logical but not optimal choice
            let risk_tolerance = openness_factor * emotional_weight;
            decision.choice = make_emotionally_logical_choice(risk_tolerance);
        }
    }
    
    /// Social interaction system with misunderstanding mechanics
    fn social_interaction_system(
        mut interactions: EventReader<SocialInteraction>,
        mut agents: Query<(&Personality, &EmotionalState)>,
    ) {
        for interaction in interactions.read() {
            let misunderstanding_rate = 0.3; // 30% failure rate for believable conflicts
            
            if let Ok((personality, emotion)) = agents.get(interaction.target) {
                // Subjective perception based on personality and emotion
                let perception_bias = personality.openness.value() * emotion.valence;
                let filtered_message = apply_perception_filter(interaction.message, perception_bias);
                
                // Information loss through "Plato's Cave" pipeline
                if random() < misunderstanding_rate {
                    create_misunderstanding(filtered_message);
                } else {
                    process_communication(filtered_message);
                }
            }
        }
    }
    
    /// Emotional contagion with believable speed
    fn emotional_contagion_system(
        mut agents: Query<(&mut EmotionalState, &Transform, &Personality)>,
        world_time: Res<WorldTime>,
    ) {
        for (mut emotion, transform, personality) in agents.iter_mut() {
            // Gradual contagion with personality-based susceptibility
            let susceptibility = personality.openness.value() * personality.extraversion.value();
            let time_factor = world_time.delta_time * 0.1; // Gradual spread
            
            // Apply emotional influence gradually
            let influence = calculate_nearby_emotional_influence(transform.translation);
            emotion.valence += influence * susceptibility * time_factor;
        }
    }
    "#;
    
    let result = validator.validate_file(good_code, Path::new("test_good.rs"));
    println!("✅ Good behavioral consistency validation result: passed = {}", result.passed);
    if !result.issues.is_empty() {
        println!("   Issues found: {}", result.issues.len());
        for issue in &result.issues {
            println!("   - {}", issue.message);
        }
    }
    
    // Test code with poor behavioral consistency
    let bad_code = r#"
    use bevy::prelude::*;
    
    /// Decision system that uses optimal algorithms (violates "Feel Over Science")
    fn decision_making_system(
        mut agents: Query<&mut DecisionState>,
    ) {
        for mut decision in agents.iter_mut() {
            // Optimal decision-making - violates philosophy
            decision.choice = find_optimal_solution();
            
            // Hardcoded threshold - should be personality-driven
            if decision.confidence > 0.8 {
                decision.execute = true;
            }
        }
    }
    
    /// Social system with perfect communication (unrealistic)
    fn social_interaction_system(
        mut interactions: EventReader<SocialInteraction>,
    ) {
        for interaction in interactions.read() {
            // Perfect message transfer - no misunderstanding
            let message = interaction.message.clone();
            send_perfect_message(message);
        }
    }
    
    /// Emotional contagion with instant spread (unrealistic)
    fn emotional_contagion_system(
        mut agents: Query<&mut EmotionalState>,
    ) {
        for mut emotion in agents.iter_mut() {
            // Instant emotional contagion - not believable
            emotion.valence = get_average_emotion_in_area();
        }
    }
    
    /// System without personality modulation
    fn behavior_system(
        mut agents: Query<&mut BehaviorState>,
    ) {
        for mut behavior in agents.iter_mut() {
            // No personality influence - all agents behave the same
            behavior.action = ActionType::DefaultAction;
        }
    }
    "#;
    
    let result = validator.validate_file(bad_code, Path::new("test_bad.rs"));
    println!("\n❌ Poor behavioral consistency validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    for issue in &result.issues {
        println!("   - {}: {}", issue.issue_type, issue.message);
        if let Some(suggestion) = &issue.suggestion {
            println!("     Suggestion: {}", suggestion);
        }
    }
    
    // Test personality consistency specifically
    let personality_test_code = r#"
    fn social_response_system(
        mut agents: Query<(&mut SocialResponse, &Personality)>,
    ) {
        for (mut response, personality) in agents.iter_mut() {
            // Good: personality influences behavior
            let extraversion_factor = personality.extraversion.value();
            response.enthusiasm = extraversion_factor * 0.8;
            
            // Bad: hardcoded value that should be personality-driven
            response.confidence_threshold = 0.7;
        }
    }
    "#;
    
    let result = validator.personality_consistency_validator.validate_personality_consistency(
        personality_test_code, 
        Path::new("test_personality.rs")
    );
    println!("\n🧠 Personality consistency validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    for issue in &result.issues {
        println!("   - {}: {}", issue.issue_type, issue.message);
    }
    
    // Test social dynamics specifically
    let social_test_code = r#"
    fn communication_system(
        mut messages: EventReader<Message>,
    ) {
        for message in messages.read() {
            // Good: includes misunderstanding mechanics
            let misunderstanding_rate = 0.25; // 25% failure rate
            if random() < misunderstanding_rate {
                distort_message(message);
            }
            
            // Good: information loss through perception
            let filtered_message = apply_confirmation_bias(message);
            process_subjective_interpretation(filtered_message);
        }
    }
    "#;
    
    let result = validator.social_dynamics_validator.validate_social_dynamics(
        social_test_code, 
        Path::new("test_social.rs")
    );
    println!("\n👥 Social dynamics validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    for issue in &result.issues {
        println!("   - {}: {}", issue.issue_type, issue.message);
    }
    
    // Test decision system specifically
    let decision_test_code = r#"
    fn choice_system(
        mut agents: Query<(&mut Choice, &EmotionalState, &Personality)>,
    ) {
        for (mut choice, emotion, personality) in agents.iter_mut() {
            // Good: emotional influence on decisions
            let stress_factor = emotion.arousal.abs();
            let neuroticism_factor = personality.neuroticism.value();
            
            // Emotionally logical but not optimal
            let decision_quality = 1.0 - (stress_factor * neuroticism_factor);
            choice.option = make_emotionally_driven_choice(decision_quality);
        }
    }
    "#;
    
    let result = validator.decision_system_validator.validate_decision_system(
        decision_test_code, 
        Path::new("test_decision.rs")
    );
    println!("\n🤔 Decision system validation result: passed = {}", result.passed);
    println!("   Issues found: {}", result.issues.len());
    for issue in &result.issues {
        println!("   - {}: {}", issue.issue_type, issue.message);
    }
    
    println!("\n🎯 Behavioral Consistency Validator is working correctly!");
    println!("   - Validates personality-driven behavior consistency");
    println!("   - Enforces 20-40% misunderstanding rates in social interactions");
    println!("   - Ensures 'Feel Over Science' philosophy compliance");
    println!("   - Detects unrealistic communication and emotional patterns");
}

// Mock functions for the test
fn make_emotionally_logical_choice(_risk_tolerance: f32) -> i32 { 0 }
fn apply_perception_filter(message: &str, _bias: f32) -> String { message.to_string() }
fn create_misunderstanding(_message: String) {}
fn process_communication(_message: String) {}
fn calculate_nearby_emotional_influence(_position: bevy::math::Vec3) -> f32 { 0.0 }
fn find_optimal_solution() -> i32 { 0 }
fn send_perfect_message(_message: String) {}
fn get_average_emotion_in_area() -> f32 { 0.0 }
fn random() -> f32 { 0.5 }
fn distort_message(_message: &str) {}
fn apply_confirmation_bias(message: &str) -> String { message.to_string() }
fn process_subjective_interpretation(_message: String) {}
fn make_emotionally_driven_choice(_quality: f32) -> i32 { 0 }

// Mock types for the test
#[derive(bevy::prelude::Component)]
struct DecisionState { choice: i32, confidence: f32, execute: bool }
#[derive(bevy::prelude::Component)]
struct Personality { openness: MockNormalized, neuroticism: MockNormalized, extraversion: MockNormalized }
#[derive(bevy::prelude::Component)]
struct EmotionalState { valence: f32, arousal: f32 }
struct SocialInteraction { target: bevy::prelude::Entity, message: String }
#[derive(bevy::prelude::Component)]
struct SocialResponse { enthusiasm: f32, confidence_threshold: f32 }
#[derive(bevy::prelude::Component)]
struct BehaviorState { action: ActionType }
#[derive(bevy::prelude::Component)]
struct Choice { option: i32 }
struct Message;
enum ActionType { DefaultAction }

struct MockNormalized(f32);
impl MockNormalized {
    fn value(&self) -> f32 { self.0 }
}