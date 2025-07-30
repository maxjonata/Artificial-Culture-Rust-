use bevy::prelude::*;

/// Marker component for NPCs
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Npc;

/// Component representing NPC personality traits based on Big Five model
/// Based on "The Big Five Personality Dimensions and Job Performance" (Barrick & Mount, 1991)
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Personality {
    /// Openness to new ideas and experiences (affects rumor spread and curiosity)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub openness: f32,
    /// Extraversion - sociability and assertiveness (affects social interactions)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub extraversion: f32,
    /// Agreeableness - cooperation and trust (affects rumor content and social harmony)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub agreeableness: f32,
    /// Conscientiousness - organization and responsibility (affects goal pursuit)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub conscientiousness: f32,
    /// Neuroticism - emotional stability (affects stress response and decision-making)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub neuroticism: f32,
}
