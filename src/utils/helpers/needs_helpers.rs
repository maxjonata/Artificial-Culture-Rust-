use crate::components::components_constants::GameConstants;
use crate::components::components_needs::{BasicNeeds, Desire, DesireThresholds};

/// Helper function to apply decay to needs based on homeostatic drive theory
/// System based on Homeostatic Drive Theory - organisms maintain internal balance
/// All values are normalized between 0.0-1.0
/// Now frame-rate independent using delta_time scaling
pub fn apply_needs_decay(needs: &mut BasicNeeds, game_constants: &GameConstants, delta_time: f32) -> (f32, f32, f32, f32, f32) {
    // Scale decay rates by delta_time for frame-rate independence
    // This ensures consistent behavior regardless of FPS
    let hunger_change = -game_constants.hunger_decay * delta_time;
    let thirst_change = -game_constants.thirst_decay * delta_time;
    let fatigue_change = -game_constants.fatigue_regen * delta_time; // Fatigue decreases over time (rest regenerates)
    let safety_change = -game_constants.safety_decay * delta_time;
    let social_change = -game_constants.social_decay * delta_time;

    needs.hunger = (needs.hunger + hunger_change).clamp(0.0, 1.0);
    needs.thirst = (needs.thirst + thirst_change).clamp(0.0, 1.0);
    needs.fatigue = (needs.fatigue + fatigue_change).clamp(0.0, 1.0);
    needs.safety = (needs.safety + safety_change).clamp(0.0, 1.0);
    needs.social = (needs.social + social_change).clamp(0.0, 1.0);

    (hunger_change, thirst_change, fatigue_change, safety_change, social_change)
}

/// Helper function to increase social need based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// All values are normalized between 0.0-1.0
pub fn increase_social_need(needs: &mut BasicNeeds, amount: f32) -> f32 {
    let old_social = needs.social;
    needs.social = (needs.social + amount).clamp(0.0, 1.0);
    needs.social - old_social // Return actual change for ML tracking
}

/// Helper function implementing Maslow's Hierarchy of Needs for desire evaluation
/// System based on Maslow's Hierarchy of Needs and Threshold Psychology
/// All values are normalized between 0.0-1.0
pub fn evaluate_most_urgent_desire(needs: &BasicNeeds, thresholds: &DesireThresholds) -> (Desire, f32) {
    let mut urgent_desires = Vec::new();

    // ML-HOOK: Each urgency calculation provides quantifiable state for observation space
    if needs.hunger < thresholds.hunger_threshold {
        let urgency = (thresholds.hunger_threshold - needs.hunger) * thresholds.priority_weights.hunger;
        urgent_desires.push((Desire::FindFood, urgency));
    }

    if needs.thirst < thresholds.thirst_threshold {
        let urgency = (thresholds.thirst_threshold - needs.thirst) * thresholds.priority_weights.thirst;
        urgent_desires.push((Desire::FindWater, urgency));
    }

    if needs.fatigue < thresholds.fatigue_threshold {
        let urgency = (thresholds.fatigue_threshold - needs.fatigue) * thresholds.priority_weights.fatigue;
        urgent_desires.push((Desire::Rest, urgency));
    }

    if needs.safety < thresholds.safety_threshold {
        let urgency = (thresholds.safety_threshold - needs.safety) * thresholds.priority_weights.safety;
        urgent_desires.push((Desire::FindSafety, urgency));
    }

    if needs.social < thresholds.social_threshold {
        let urgency = (thresholds.social_threshold - needs.social) * thresholds.priority_weights.social;
        urgent_desires.push((Desire::Socialize, urgency));
    }

    // Return the desire with highest urgency score and the score itself for ML tracking
    urgent_desires
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(desire, urgency)| (desire, urgency))
        .unwrap_or((Desire::Wander, 0.0))
}

/// Helper function to get quantifiable satisfaction levels for ML observation space
/// ML-HOOK: Provides normalized satisfaction metrics for reward calculation
/// All values are already normalized between 0.0-1.0
pub fn get_satisfaction_level(needs: &BasicNeeds, desire: &Desire) -> f32 {
    match desire {
        Desire::FindFood => needs.hunger,
        Desire::FindWater => needs.thirst,
        Desire::Rest => needs.fatigue,
        Desire::FindSafety => needs.safety,
        Desire::Socialize => needs.social,
        Desire::Wander => 0.5, // Neutral satisfaction for wandering
    }
}

/// Creates randomized initial needs based on realistic biological variation
/// Based on Individual Differences Theory - people have varying baseline needs
/// References: Maslow (1943) - individual variation in need satisfaction patterns
pub fn create_random_basic_needs() -> BasicNeeds {
    use rand::prelude::*;
    let mut rng = rand::rng();

    BasicNeeds {
        // Hunger: Most people start moderately satisfied (0.6-0.9 range)
        // Biological variation in metabolic rates and meal timing
        hunger: rng.random_range(0.6..0.9),

        // Thirst: Critical for survival, people maintain higher hydration (0.7-0.95 range)
        // Dehydration symptoms appear quickly, so baseline is higher
        thirst: rng.random_range(0.7..0.95),

        // Fatigue: Wide variation based on sleep cycles and individual chronotypes (0.4-0.8 range)
        // Some people are naturally more energetic, others more tired
        fatigue: rng.random_range(0.4..0.8),

        // Safety: Most people feel relatively safe in normal environments (0.7-0.95 range)
        // Anxiety disorders and personality traits cause individual variation
        safety: rng.random_range(0.7..0.95),

        // Social: Highest variation due to personality differences (0.3-0.8 range)
        // Introverts vs extroverts have vastly different social need baselines
        social: rng.random_range(0.3..0.8),
    }
}