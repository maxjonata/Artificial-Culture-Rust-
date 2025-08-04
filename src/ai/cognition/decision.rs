use crate::ai::cognition::memory::CognitiveWorkingMemory;
use crate::ai::physiology::needs::BasicNeeds;
use bevy::prelude::*;

/// Component representing the agent's current dominant desire/goal
/// Based on Goal-Oriented Action Planning (GOAP) and Drive Theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DesireDecision {
    /// Current primary goal the agent is pursuing
    pub current_goal: GoalType,
    /// Strength/urgency of the current desire (0.0-1.0)
    pub intensity: f32,
    /// Time when this desire was last updated
    pub last_update: f32,
    /// Goal persistence - how long to pursue before reconsidering
    pub persistence_duration: f32,
}

/// Types of goals an agent can pursue
/// Based on Maslow's Hierarchy of Needs and Behavioral Economics
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Default)]
pub enum GoalType {
    /// No active goal - agent will wander or explore
    #[default]
    Idle,
    /// Seeking water to satisfy thirst
    FindWater,
    /// Seeking food to satisfy hunger
    FindFood,
    /// Seeking rest/sleep to recover energy
    FindRest,
    /// Seeking safety from threats
    FindSafety,
    /// Seeking social interaction to reduce social need
    Socialize,
    /// Exploring to discover new Things
    Explore,
}

/// Event triggered when an agent makes a decision
/// Used for ML observation and inter-system communication
#[derive(Event, Debug)]
pub struct DecisionMade {
    pub agent: Entity,
    pub previous_goal: GoalType,
    pub new_goal: GoalType,
    pub decision_factors: DecisionFactors,
}

/// Factors that influenced a decision - useful for ML training
#[derive(Debug, Clone)]
pub struct DecisionFactors {
    /// Current need levels that influenced the decision
    pub need_urgency: f32,
    /// Memory of relevant Things
    pub memory_influence: f32,
    /// Social pressure influence
    pub social_influence: f32,
    /// Exploration drive
    pub curiosity: f32,
}

impl Default for DesireDecision {
    fn default() -> Self {
        Self {
            current_goal: GoalType::Idle,
            intensity: 0.0,
            last_update: 0.0,
            persistence_duration: 5.0, // Pursue goals for 5 seconds before reconsidering
        }
    }
}

/// Core decision-making system using utility-based AI
/// Based on Multi-Criteria Decision Analysis and Behavioral Economics
pub fn decision_making_system(
    mut query: Query<(Entity, &mut DesireDecision, &BasicNeeds, &CognitiveWorkingMemory)>,
    mut decision_events: EventWriter<DecisionMade>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut desire, needs, working_memory) in query.iter_mut() {
        // Check if it's time to reconsider current goal
        let time_since_update = current_time - desire.last_update;
        if time_since_update < desire.persistence_duration && desire.current_goal != GoalType::Idle {
            continue; // Stick with current goal for now
        }

        let previous_goal = desire.current_goal;

        // Calculate utility scores for each possible goal
        let goal_utilities = calculate_goal_utilities(needs, working_memory);

        // Find the goal with highest utility
        let (best_goal, best_utility) = goal_utilities
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(goal, utility)| (*goal, *utility))
            .unwrap_or((GoalType::Idle, 0.0f32));

        // Only change goals if the new utility is significantly better
        let utility_threshold = 0.1;
        if best_utility > desire.intensity + utility_threshold || desire.current_goal == GoalType::Idle {
            let decision_factors = DecisionFactors {
                need_urgency: calculate_max_need_urgency(needs),
                memory_influence: calculate_memory_influence(working_memory),
                social_influence: 0.0, // Will be enhanced when social systems are restored
                curiosity: calculate_exploration_drive(working_memory),
            };

            // Send decision event for ML training and other systems
            decision_events.write(DecisionMade {
                agent: entity,
                previous_goal,
                new_goal: best_goal,
                decision_factors,
            });

            // Update desire
            desire.current_goal = best_goal;
            desire.intensity = best_utility;
            desire.last_update = current_time;
        }
    }
}

/// Calculate utility scores for all possible goals
/// Based on Multi-Attribute Utility Theory (MAUT)
fn calculate_goal_utilities(needs: &BasicNeeds, _working_memory: &CognitiveWorkingMemory) -> Vec<(GoalType, f32)> {
    vec![
        // Water has highest urgency multiplier due to survival priority
        (GoalType::FindWater, calculate_water_utility(needs.thirst)),
        (GoalType::FindFood, calculate_food_utility(needs.hunger)),
        (GoalType::FindRest, calculate_rest_utility(needs.rest)),
        (GoalType::FindSafety, calculate_safety_utility(needs.safety)),
        (GoalType::Socialize, calculate_social_utility(needs.social)),
        (GoalType::Explore, calculate_exploration_utility()),
        (GoalType::Idle, 0.1), // Small baseline utility for doing nothing
    ]
}

/// Water utility based on physiological urgency
/// Water needs are most critical - exponential urgency curve
fn calculate_water_utility(thirst: f32) -> f32 {
    if thirst > 0.8 {
        // Critical thirst - exponential urgency
        1.0 - (-10.0 * (1.0 - thirst)).exp()
    } else {
        // Normal thirst response
        thirst
    }
}

/// Food utility with medium urgency curve
fn calculate_food_utility(hunger: f32) -> f32 {
    if hunger > 0.7 {
        // High hunger - quadratic increase
        hunger * hunger
    } else {
        // Normal hunger response
        hunger * 0.8
    }
}

/// Rest utility for fatigue management
fn calculate_rest_utility(fatigue: f32) -> f32 {
    if fatigue > 0.6 {
        fatigue * 1.2 // Slight urgency boost when very tired
    } else {
        fatigue * 0.7
    }
}

/// Safety utility - baseline need
fn calculate_safety_utility(safety_need: f32) -> f32 {
    safety_need * 0.6
}

/// Social utility for social need
fn calculate_social_utility(social: f32) -> f32 {
    social * 0.5
}

/// Exploration utility - curiosity drive
fn calculate_exploration_utility() -> f32 {
    0.2 // Baseline curiosity
}

/// Calculate the maximum need urgency across all needs
fn calculate_max_need_urgency(needs: &BasicNeeds) -> f32 {
    [needs.thirst, needs.hunger, needs.rest, needs.safety, needs.social]
        .iter()
        .fold(0.0, |acc, &x| acc.max(x))
}

/// Calculate how much working memory influences decisions
fn calculate_memory_influence(working_memory: &CognitiveWorkingMemory) -> f32 {
    working_memory.active_concepts.len() as f32 / working_memory.capacity as f32
}

/// Calculate exploration drive based on working memory and curiosity
fn calculate_exploration_drive(working_memory: &CognitiveWorkingMemory) -> f32 {
    // Base curiosity modified by working memory activity
    let base_curiosity = 0.2;
    let memory_factor = 1.0 - calculate_memory_influence(working_memory);
    base_curiosity * (1.0 + memory_factor * 0.5)
}
