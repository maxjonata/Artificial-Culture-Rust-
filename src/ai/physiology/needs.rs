use bevy::prelude::*;

/// Component representing an NPC's basic physiological and psychological needs
/// All values are normalized between 0.0-1.0 for ML compatibility
/// Based on Homeostatic Drive Theory - organisms maintain internal balance
/// IMPORTANT: All needs follow "higher value = MORE URGENT" semantics (inverted for action)
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct BasicNeeds {
    /// Thirst urgency (0.0 = hydrated, 1.0 = critically dehydrated)
    pub thirst: f32,
    /// Hunger urgency (0.0 = well fed, 1.0 = starving)
    pub hunger: f32,
    /// Rest level (0.0 = well rested, 1.0 = exhausted)
    pub rest: f32,
    /// Safety need (0.0 = safe, 1.0 = in danger)
    pub safety: f32,
    /// Social level (0.0 = socially satisfied, 1.0 = very lonely)
    pub social: f32,
}

/// Component that defines thresholds for when desires should be activated
/// Follows Single Responsibility Principle - manages only desire thresholds
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DesireThresholds {
    /// Hunger threshold above which FindFood desire is triggered
    pub hunger_threshold: DualThreshold,
    /// Thirst threshold above which FindWater desire is triggered
    pub thirst_threshold: DualThreshold,
    /// Rest threshold above which Rest desire is triggered
    pub rest_threshold: DualThreshold,
    /// Safety threshold above which FindSafety desire is triggered
    pub safety_threshold: DualThreshold,
    /// Social threshold above which Socialize desire is triggered
    pub social_threshold: DualThreshold,
    /// Priority weights for competing desires (higher = more important)
    pub priority_weights: DesirePriorities,
}

#[derive(Reflect, Debug)]
pub struct DualThreshold {
    pub moderate_threshold: f32, // Threshold above which desire is activated (start considering)
    pub urgent_threshold: f32,   // Threshold above which pathfinding starts (urgent action)
}

/// Priority system for resolving competing desires
/// Based on Maslow's hierarchy with physiological needs having highest priority
#[derive(Reflect, Debug)]
pub struct DesirePriorities {
    /// Thirst has highest priority - survival critical
    pub thirst_weight: f32,
    /// Hunger is second priority - survival critical
    pub hunger_weight: f32,
    /// Rest management - necessary for function
    pub rest_weight: f32,
    /// Safety needs - security and protection
    pub safety_weight: f32,
    /// Social needs - psychological wellbeing
    pub social_weight: f32,
}

/// Event fired when a need crosses a critical threshold
#[derive(Event, Debug)]
pub struct NeedThresholdCrossed {
    pub agent: Entity,
    pub need_type: NeedType,
    pub threshold_type: ThresholdType,
    pub current_value: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum NeedType {
    Thirst,
    Hunger,
    Rest,
    Safety,
    Social,
}

#[derive(Debug, Clone, Copy)]
pub enum ThresholdType {
    Moderate,
    Urgent,
}

impl Default for BasicNeeds {
    fn default() -> Self {
        Self {
            thirst: 0.2,      // Start slightly thirsty
            hunger: 0.1,      // Start with minimal hunger
            rest: 0.0,     // Start well-rested
            safety: 0.0,      // Start feeling safe
            social: 0.3,  // Start with some social need
        }
    }
}

impl Default for DualThreshold {
    fn default() -> Self {
        Self {
            moderate_threshold: 0.4,  // Start considering action at 40%
            urgent_threshold: 0.7,    // Take urgent action at 70%
        }
    }
}

impl Default for DesireThresholds {
    fn default() -> Self {
        Self {
            hunger_threshold: DualThreshold { moderate_threshold: 0.3, urgent_threshold: 0.6 },
            thirst_threshold: DualThreshold { moderate_threshold: 0.4, urgent_threshold: 0.8 }, // Water more urgent
            rest_threshold: DualThreshold { moderate_threshold: 0.5, urgent_threshold: 0.8 },
            safety_threshold: DualThreshold { moderate_threshold: 0.3, urgent_threshold: 0.7 },
            social_threshold: DualThreshold { moderate_threshold: 0.4, urgent_threshold: 0.7 },
            priority_weights: DesirePriorities::default(),
        }
    }
}

impl Default for DesirePriorities {
    fn default() -> Self {
        Self {
            // Based on Maslow's hierarchy - physiological needs dominate
            thirst_weight: 1.0,      // Highest priority - survival critical
            hunger_weight: 0.9,      // Second highest - survival critical
            rest_weight: 0.6,     // Important for function
            safety_weight: 0.7,      // Security needs
            social_weight: 0.4,  // Psychological needs
        }
    }
}

/// System that gradually increases needs over time
/// Based on Homeostatic Drive Theory and metabolic research
pub fn needs_decay_system(
    mut query: Query<(Entity, &mut BasicNeeds)>,
    mut threshold_events: EventWriter<NeedThresholdCrossed>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut needs) in query.iter_mut() {
        let old_needs = needs.clone();

        // Physiological needs increase over time
        // Rates based on simplified human metabolic models
        needs.thirst = (needs.thirst + 0.08 * delta_time).min(1.0);  // Thirst builds fastest
        needs.hunger = (needs.hunger + 0.04 * delta_time).min(1.0);  // Hunger builds slower
        needs.rest = (needs.rest + 0.02 * delta_time).min(1.0);     // Rest builds slowly

        // FIXED: Safety needs increase over time (feeling unsafe without security)
        // In real environments, safety needs would be context-dependent
        needs.safety = (needs.safety + 0.015 * delta_time).min(1.0);

        // Social needs increase without social interaction
        needs.social = (needs.social + 0.03 * delta_time).min(1.0);

        // Check for threshold crossings and send events
        check_threshold_crossings(entity, &old_needs, &needs, &mut threshold_events);
    }
}

/// Check if any need thresholds were crossed and send appropriate events
fn check_threshold_crossings(
    entity: Entity,
    old_needs: &BasicNeeds,
    new_needs: &BasicNeeds,
    threshold_events: &mut EventWriter<NeedThresholdCrossed>,
) {
    let thresholds = DesireThresholds::default();

    // Check thirst thresholds
    check_need_threshold(
        entity,
        old_needs.thirst,
        new_needs.thirst,
        &thresholds.thirst_threshold,
        NeedType::Thirst,
        threshold_events,
    );

    // Check hunger thresholds
    check_need_threshold(
        entity,
        old_needs.hunger,
        new_needs.hunger,
        &thresholds.hunger_threshold,
        NeedType::Hunger,
        threshold_events,
    );

    // Check rest thresholds
    check_need_threshold(
        entity,
        old_needs.rest,
        new_needs.rest,
        &thresholds.rest_threshold,
        NeedType::Rest,
        threshold_events,
    );

    // Check safety thresholds
    check_need_threshold(
        entity,
        old_needs.safety,
        new_needs.safety,
        &thresholds.safety_threshold,
        NeedType::Safety,
        threshold_events,
    );

    // Check social thresholds
    check_need_threshold(
        entity,
        old_needs.social,
        new_needs.social,
        &thresholds.social_threshold,
        NeedType::Social,
        threshold_events,
    );
}

/// Helper function to check if a specific need crossed a threshold
fn check_need_threshold(
    entity: Entity,
    old_value: f32,
    new_value: f32,
    threshold: &DualThreshold,
    need_type: NeedType,
    threshold_events: &mut EventWriter<NeedThresholdCrossed>,
) {
    // Check if moderate threshold was crossed (upward)
    if old_value < threshold.moderate_threshold && new_value >= threshold.moderate_threshold {
        threshold_events.write(NeedThresholdCrossed {
            agent: entity,
            need_type,
            threshold_type: ThresholdType::Moderate,
            current_value: new_value,
        });
    }

    // Check if urgent threshold was crossed (upward)
    if old_value < threshold.urgent_threshold && new_value >= threshold.urgent_threshold {
        threshold_events.write(NeedThresholdCrossed {
            agent: entity,
            need_type,
            threshold_type: ThresholdType::Urgent,
            current_value: new_value,
        });
    }
}
