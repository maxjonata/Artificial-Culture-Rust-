use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identificador único para todas entidades.
pub type EntityId = Uuid;

/// Evento básico para persistência futura.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    BeliefChanged {
        entity: EntityId,
        belief: String,
        certainty: f32,
    },
    MemoryAdded {
        entity: EntityId,
        memory: String,
    },
    ActionPerformed {
        entity: EntityId,
        action: String,
    },
    EmotionalStateChanged {
        entity: EntityId,
        emotion: String,
        value: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Physiological {
    pub hunger: f32,
    pub safety: f32,
    pub rest: f32,
    pub health: f32,
    pub comfort: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emotion {
    pub fear: f32,
    pub anger: f32,
    pub happiness: f32,
    pub sadness: f32,
    pub surprise: f32,
    pub disgust: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeliefSystem {
    pub beliefs: std::collections::HashMap<String, f32>,
    pub certainty: std::collections::HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialNetwork {
    pub friends: Vec<EntityId>,
    pub enemies: Vec<EntityId>,
    pub reputation: std::collections::HashMap<EntityId, f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub experiences: Vec<String>, // Aqui pode ser embedding no futuro.
    pub strength: std::collections::HashMap<String, f32>,
    pub decay_rate: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Goals {
    pub current_goals: Vec<String>,
    pub priorities: std::collections::HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CulturalTraits {
    pub rituals: Vec<String>,
    pub norms: std::collections::HashMap<String, f32>,
    pub artifacts: Vec<String>,
}
