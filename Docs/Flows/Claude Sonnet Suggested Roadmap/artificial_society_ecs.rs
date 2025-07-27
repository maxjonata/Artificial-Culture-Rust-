use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use nalgebra::{Vector3, Matrix3};
use rand::prelude::*;

// ============================================================================
// CORE NEUROLOGICAL COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct NeurotransmitterSystem {
    pub dopamine: f32,      // reward, motivation, learning [0.0-1.0]
    pub serotonin: f32,     // mood, anxiety, social behavior [0.0-1.0]
    pub oxytocin: f32,      // social bonding, trust, empathy [0.0-1.0]
    pub cortisol: f32,      // stress response, vigilance [0.0-1.0]
    pub baseline_levels: HashMap<String, f32>,
    pub decay_rates: HashMap<String, f32>,
    pub synthesis_rates: HashMap<String, f32>,
}

#[derive(Component, Debug, Clone)]
pub struct CognitiveArchitecture {
    pub working_memory_capacity: usize,
    pub attention_weights: Vector3<f32>, // visual, auditory, interoceptive
    pub cognitive_load: f32,
    pub executive_control: f32,
    pub processing_speed: f32,
    pub neural_noise: f32,
}

#[derive(Component, Debug, Clone)]
pub struct EmotionalCore {
    pub valence: f32,           // positive/negative [-1.0, 1.0]
    pub arousal: f32,           // calm/excited [0.0, 1.0]
    pub basic_emotions: HashMap<EmotionType, f32>,
    pub emotional_regulation: f32,
    pub emotional_contagion_susceptibility: f32,
    pub current_mood: f32,
    pub mood_stability: f32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum EmotionType {
    Joy, Sadness, Fear, Anger, Disgust, Surprise,
    Trust, Anticipation, // Plutchik's wheel additions
}

// ============================================================================
// PSYCHOLOGICAL COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct PersonalityProfile {
    pub big_five: BigFiveTraits,
    pub locus_of_control: f32,    // internal(0.0) vs external(1.0)
    pub attachment_style: AttachmentStyle,
    pub tolerance_for_ambiguity: f32,
    pub risk_tolerance: f32,
    pub time_perspective: TimeOrientation,
}

#[derive(Debug, Clone)]
pub struct BigFiveTraits {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

#[derive(Debug, Clone)]
pub enum AttachmentStyle {
    Secure(f32),
    Anxious(f32),
    Avoidant(f32),
    Disorganized(f32),
}

#[derive(Debug, Clone)]
pub struct TimeOrientation {
    pub past_focus: f32,
    pub present_focus: f32,
    pub future_focus: f32,
}

#[derive(Component, Debug, Clone)]
pub struct MotivationalSystem {
    pub hierarchy_needs: MaslowNeeds,
    pub self_determination: SelfDeterminationNeeds,
    pub current_drives: Vec<Drive>,
    pub goal_stack: Vec<Goal>,
    pub motivational_strength: f32,
    pub intrinsic_motivation: f32,
    pub extrinsic_motivation: f32,
}

#[derive(Debug, Clone)]
pub struct MaslowNeeds {
    pub physiological: f32,
    pub safety: f32,
    pub belonging: f32,
    pub esteem: f32,
    pub self_actualization: f32,
}

#[derive(Debug, Clone)]
pub struct SelfDeterminationNeeds {
    pub autonomy: f32,
    pub competence: f32,
    pub relatedness: f32,
}

#[derive(Debug, Clone)]
pub struct Drive {
    pub drive_type: DriveType,
    pub intensity: f32,
    pub urgency: f32,
    pub satisfaction_threshold: f32,
}

#[derive(Debug, Clone)]
pub enum DriveType {
    Hunger, Thirst, Sleep, Social, Achievement, Power, Affiliation
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub id: u32,
    pub description: String,
    pub priority: f32,
    pub progress: f32,
    pub deadline: Option<f32>,
    pub sub_goals: Vec<u32>,
    pub emotional_investment: f32,
}

// ============================================================================
// COGNITIVE PROCESSING COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct MemorySystem {
    pub working_memory: Vec<MemoryItem>,
    pub episodic_memory: Vec<EpisodicMemory>,
    pub semantic_memory: HashMap<String, SemanticNode>,
    pub procedural_memory: HashMap<String, ProcedureSchema>,
    pub memory_consolidation_rate: f32,
    pub forgetting_curve_parameters: (f32, f32), // strength, decay
}

#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub content: MemoryContent,
    pub activation_level: f32,
    pub emotional_salience: f32,
    pub timestamp: f32,
    pub retrieval_count: u32,
}

#[derive(Debug, Clone)]
pub enum MemoryContent {
    Sensory(SensoryData),
    Conceptual(String),
    Emotional(EmotionType, f32),
    Social(SocialInteraction),
}

#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    pub event_id: u32,
    pub context: EnvironmentalContext,
    pub participants: Vec<Entity>,
    pub emotional_tone: f32,
    pub personal_significance: f32,
    pub accuracy: f32,
    pub reconstruction_bias: f32,
}

#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub concept: String,
    pub associations: HashMap<String, f32>,
    pub confidence: f32,
    pub cultural_loading: f32,
}

#[derive(Component, Debug, Clone)]
pub struct DecisionMakingSystem {
    pub decision_style: DecisionStyle,
    pub risk_preference: f32,
    pub temporal_discounting_rate: f32,
    pub cognitive_biases: Vec<CognitiveBias>,
    pub decision_confidence: f32,
    pub choice_overload_threshold: usize,
}

#[derive(Debug, Clone)]
pub enum DecisionStyle {
    Rational(f32),      // deliberative weight
    Intuitive(f32),     // heuristic weight
    Habitual(f32),      // automatic weight
    Mixed(f32, f32, f32), // weights for each style
}

#[derive(Debug, Clone)]
pub struct CognitiveBias {
    pub bias_type: BiasType,
    pub strength: f32,
    pub activation_threshold: f32,
}

#[derive(Debug, Clone)]
pub enum BiasType {
    ConfirmationBias, AvailabilityHeuristic, AnchoringEffect,
    LossAversion, FramingEffect, SocialProof, AuthorityBias,
    InGroupBias, FundamentalAttributionError,
}

// ============================================================================
// SOCIAL COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct SocialIdentity {
    pub self_concept: SelfConcept,
    pub social_roles: Vec<SocialRole>,
    pub group_memberships: HashMap<GroupId, GroupAffiliation>,
    pub reputation: HashMap<Entity, f32>,
    pub social_status: f32,
    pub identity_certainty: f32,
}

#[derive(Debug, Clone)]
pub struct SelfConcept {
    pub traits: HashMap<String, f32>,
    pub values: HashMap<String, f32>,
    pub abilities: HashMap<String, f32>,
    pub self_esteem: f32,
    pub self_efficacy: f32,
    pub narrative_coherence: f32,
}

#[derive(Debug, Clone)]
pub struct SocialRole {
    pub role_type: RoleType,
    pub competence: f32,
    pub identification: f32,
    pub expectations: Vec<String>,
    pub conflicts: Vec<u32>, // conflicting role IDs
}

#[derive(Debug, Clone)]
pub enum RoleType {
    Professional(String),
    Familial(String),
    Community(String),
    Cultural(String),
}

#[derive(Debug, Clone)]
pub struct GroupAffiliation {
    pub identification_strength: f32,
    pub participation_level: f32,
    pub influence_received: f32,
    pub influence_exerted: f32,
    pub group_norms_internalization: f32,
}

type GroupId = u32;

#[derive(Component, Debug, Clone)]
pub struct RelationshipNetwork {
    pub relationships: HashMap<Entity, Relationship>,
    pub social_capital: f32,
    pub network_density: f32,
    pub betweenness_centrality: f32,
    pub trust_propensity: f32,
    pub reciprocity_tendency: f32,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub relationship_type: RelationshipType,
    pub closeness: f32,
    pub trust: f32,
    pub reciprocity_balance: f32,
    pub interaction_history: Vec<InteractionRecord>,
    pub emotional_bond: f32,
    pub power_balance: f32, // -1.0 (subordinate) to 1.0 (dominant)
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    Family(FamilyRelation),
    Friend(FriendshipLevel),
    Professional(ProfessionalRelation),
    Romantic(RomanticStage),
    Acquaintance,
    Stranger,
    Enemy(f32), // hostility level
}

#[derive(Debug, Clone)]
pub enum FamilyRelation {
    Parent, Child, Sibling, ExtendedFamily
}

#[derive(Debug, Clone)]
pub enum FriendshipLevel {
    BestFriend, Close, Casual, Former
}

#[derive(Debug, Clone)]
pub enum ProfessionalRelation {
    Colleague, Supervisor, Subordinate, Client, Competitor
}

#[derive(Debug, Clone)]
pub enum RomanticStage {
    Attraction, Dating, Committed, Married, Separated
}

// ============================================================================
// CULTURAL AND NORMATIVE COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct CulturalProfile {
    pub cultural_dimensions: CulturalDimensions,
    pub cultural_knowledge: HashMap<String, f32>,
    pub norm_internalization: HashMap<String, f32>,
    pub cultural_flexibility: f32,
    pub acculturation_stress: f32,
    pub cultural_identity_strength: f32,
}

#[derive(Debug, Clone)]
pub struct CulturalDimensions {
    pub individualism_collectivism: f32,    // -1.0 (collectivist) to 1.0 (individualist)
    pub power_distance: f32,                // 0.0 (low) to 1.0 (high)
    pub uncertainty_avoidance: f32,         // 0.0 (low) to 1.0 (high)
    pub masculinity_femininity: f32,        // -1.0 (feminine) to 1.0 (masculine)
    pub long_term_orientation: f32,         // -1.0 (short-term) to 1.0 (long-term)
    pub indulgence_restraint: f32,          // -1.0 (restraint) to 1.0 (indulgent)
}

#[derive(Component, Debug, Clone)]
pub struct NormativeSystem {
    pub personal_norms: HashMap<String, Norm>,
    pub perceived_social_norms: HashMap<String, PerceivedNorm>,
    pub norm_conformity_tendency: f32,
    pub moral_foundations: MoralFoundations,
    pub ethical_flexibility: f32,
}

#[derive(Debug, Clone)]
pub struct Norm {
    pub strength: f32,
    pub specificity: f32,
    pub moral_loading: f32,
    pub sanction_expectations: (f32, f32), // positive, negative
}

#[derive(Debug, Clone)]
pub struct PerceivedNorm {
    pub descriptive_norm: f32,    // what others do
    pub injunctive_norm: f32,     // what others approve
    pub certainty: f32,
    pub group_consensus: f32,
}

#[derive(Debug, Clone)]
pub struct MoralFoundations {
    pub care_harm: f32,
    pub fairness_cheating: f32,
    pub loyalty_betrayal: f32,
    pub authority_subversion: f32,
    pub sanctity_degradation: f32,
    pub liberty_oppression: f32,
}

// ============================================================================
// ENVIRONMENTAL AND CONTEXTUAL COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct EnvironmentalAwareness {
    pub spatial_map: SpatialCognition,
    pub environmental_affordances: Vec<Affordance>,
    pub threat_detection_sensitivity: f32,
    pub opportunity_recognition: f32,
    pub contextual_adaptation: f32,
}

#[derive(Debug, Clone)]
pub struct SpatialCognition {
    pub current_location: Vector3<f32>,
    pub familiar_locations: HashMap<String, Vector3<f32>>,
    pub spatial_orientation: f32,
    pub navigation_ability: f32,
    pub territorial_claims: Vec<Territory>,
}

#[derive(Debug, Clone)]
pub struct Territory {
    pub boundaries: Vec<Vector3<f32>>,
    pub ownership_strength: f32,
    pub defense_motivation: f32,
    pub shared_access: HashMap<Entity, f32>,
}

#[derive(Debug, Clone)]
pub struct Affordance {
    pub action_possibility: String,
    pub resource_requirement: f32,
    pub success_probability: f32,
    pub utility_estimation: f32,
}

#[derive(Debug, Clone)]
pub struct EnvironmentalContext {
    pub physical_setting: PhysicalContext,
    pub social_setting: SocialContext,
    pub temporal_context: TemporalContext,
    pub resource_availability: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct PhysicalContext {
    pub location_type: LocationType,
    pub weather_conditions: WeatherConditions,
    pub noise_level: f32,
    pub crowding_level: f32,
    pub safety_level: f32,
}

#[derive(Debug, Clone)]
pub enum LocationType {
    Home, Work, Public, Nature, Commercial, Religious, Educational
}

#[derive(Debug, Clone)]
pub struct WeatherConditions {
    pub temperature: f32,
    pub humidity: f32,
    pub light_level: f32,
    pub seasonal_factor: f32,
}

#[derive(Debug, Clone)]
pub struct SocialContext {
    pub present_individuals: HashSet<Entity>,
    pub group_dynamics: GroupDynamics,
    pub social_pressure_level: f32,
    pub formality_level: f32,
    pub privacy_level: f32,
}

#[derive(Debug, Clone)]
pub struct GroupDynamics {
    pub group_size: usize,
    pub cohesion_level: f32,
    pub leadership_structure: LeadershipType,
    pub conflict_level: f32,
    pub collective_mood: f32,
}

#[derive(Debug, Clone)]
pub enum LeadershipType {
    Authoritarian(Entity),
    Democratic(Vec<Entity>),
    LaissezFaire,
    Emergent(Entity, f32), // leader, strength
}

#[derive(Debug, Clone)]
pub struct TemporalContext {
    pub time_of_day: f32,      // 0.0-24.0
    pub day_of_week: u8,       // 1-7
    pub season: Season,
    pub time_pressure: f32,
    pub deadline_proximity: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum Season {
    Spring, Summer, Fall, Winter
}

// ============================================================================
// HYPERGRAPH AND CHAOS COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct HypergraphNode {
    pub node_id: u32,
    pub node_type: HypergraphNodeType,
    pub activation_level: f32,
    pub influence_capacity: f32,
    pub memory_trace: f32,
    pub chaos_susceptibility: f32,
}

#[derive(Debug, Clone)]
pub enum HypergraphNodeType {
    Cognitive(String),      // concept or thought
    Emotional(EmotionType), // emotional state
    Social(Entity),         // social connection
    Cultural(String),       // cultural element
    Environmental(String),  // environmental factor
    Behavioral(String),     // action or behavior
}

#[derive(Component, Debug, Clone)]
pub struct HypergraphEdges {
    pub edges: Vec<Hyperedge>,
    pub edge_weights: HashMap<u32, f32>,
    pub dynamic_weights: HashMap<u32, WeightDynamics>,
    pub propagation_delays: HashMap<u32, f32>,
}

#[derive(Debug, Clone)]
pub struct Hyperedge {
    pub edge_id: u32,
    pub connected_nodes: HashSet<u32>,
    pub edge_type: HyperedgeType,
    pub strength: f32,
    pub directionality: EdgeDirectionality,
    pub temporal_pattern: TemporalPattern,
}

#[derive(Debug, Clone)]
pub enum HyperedgeType {
    Association,     // general connection
    Causation,      // causal relationship
    Inhibition,     // suppressive relationship
    Amplification,  // enhancing relationship
    Synchronization, // temporal coordination
    Competition,    // mutually exclusive
}

#[derive(Debug, Clone)]
pub enum EdgeDirectionality {
    Undirected,
    Directed(u32, u32), // from, to
    Bidirectional(f32, f32), // forward_strength, backward_strength
}

#[derive(Debug, Clone)]
pub struct TemporalPattern {
    pub pattern_type: PatternType,
    pub frequency: f32,
    pub phase: f32,
    pub amplitude: f32,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    Constant,
    Sinusoidal,
    Exponential,
    Chaotic(ChaoticParameters),
}

#[derive(Debug, Clone)]
pub struct ChaoticParameters {
    pub attractor_type: AttractorType,
    pub lyapunov_exponent: f32,
    pub dimension: f32,
    pub control_parameters: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum AttractorType {
    Lorenz, Rossler, Henon, Custom(String)
}

#[derive(Debug, Clone)]
pub struct WeightDynamics {
    pub base_weight: f32,
    pub volatility: f32,
    pub trend: f32,
    pub noise_level: f32,
    pub adaptation_rate: f32,
}

#[derive(Component, Debug, Clone)]
pub struct ChaoticBehaviorSystem {
    pub chaos_level: f32,           // overall system chaos [0.0-1.0]
    pub attractors: Vec<BehavioralAttractor>,
    pub current_attractor: Option<usize>,
    pub transition_probability: f32,
    pub sensitivity_to_initial_conditions: f32,
    pub feedback_loops: Vec<FeedbackLoop>,
    pub emergence_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct BehavioralAttractor {
    pub attractor_id: u32,
    pub attractor_name: String,
    pub stability: f32,
    pub basin_size: f32,
    pub characteristic_behaviors: Vec<String>,
    pub entry_conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub struct FeedbackLoop {
    pub loop_id: u32,
    pub involved_components: Vec<String>,
    pub loop_type: FeedbackType,
    pub gain: f32,
    pub delay: f32,
    pub saturation_point: f32,
}

#[derive(Debug, Clone)]
pub enum FeedbackType {
    Positive,    // amplifying
    Negative,    // stabilizing
    Mixed(f32),  // mixed with positive/negative ratio
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub variable: String,
    pub operator: ComparisonOperator,
    pub threshold: f32,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan, LessThan, Equal, NotEqual, Between(f32, f32)
}

// ============================================================================
// INTERACTION AND COMMUNICATION COMPONENTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct SocialInteraction {
    pub interaction_id: u32,
    pub participants: Vec<Entity>,
    pub interaction_type: InteractionType,
    pub context: EnvironmentalContext,
    pub duration: f32,
    pub intensity: f32,
    pub outcome_satisfaction: HashMap<Entity, f32>,
}

#[derive(Debug, Clone)]
pub enum InteractionType {
    Conversation(ConversationData),
    Conflict(ConflictData),
    Cooperation(CooperationData),
    Competition(CompetitionData),
    Exchange(ExchangeData),
    Ritual(RitualData),
}

#[derive(Debug, Clone)]
pub struct ConversationData {
    pub topic: String,
    pub information_exchange: f32,
    pub emotional_exchange: f32,
    pub persuasion_attempts: Vec<PersuasionAttempt>,
}

#[derive(Debug, Clone)]
pub struct PersuasionAttempt {
    pub source: Entity,
    pub target: Entity,
    pub message: String,
    pub persuasion_technique: PersuasionTechnique,
    pub success_probability: f32,
}

#[derive(Debug, Clone)]
pub enum PersuasionTechnique {
    LogicalArgument, EmotionalAppeal, SocialProof, Authority,
    Reciprocity, Commitment, Scarcity, Liking
}

#[derive(Debug, Clone)]
pub struct InteractionRecord {
    pub timestamp: f32,
    pub interaction: SocialInteraction,
    pub emotional_impact: f32,
    pub relationship_change: f32,
    pub memory_strength: f32,
}

// ============================================================================
// SUPPORTING DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone)]
pub struct SensoryData {
    pub modality: SensoryModality,
    pub intensity: f32,
    pub quality: String,
    pub spatial_location: Option<Vector3<f32>>,
    pub temporal_pattern: Option<TemporalPattern>,
}

#[derive(Debug, Clone)]
pub enum SensoryModality {
    Visual, Auditory, Tactile, Olfactory, Gustatory, Vestibular, Proprioceptive
}

#[derive(Debug, Clone)]
pub struct ProcedureSchema {
    pub steps: Vec<ActionStep>,
    pub success_rate: f32,
    pub efficiency: f32,
    pub context_dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ActionStep {
    pub action: String,
    pub preconditions: Vec<Condition>,
    pub expected_outcome: String,
    pub skill_requirement: f32,
}

#[derive(Debug, Clone)]
pub struct ConflictData {
    pub conflict_type: ConflictType,
    pub intensity: f32,
    pub resources_at_stake: Vec<String>,
    pub resolution_attempts: Vec<ResolutionAttempt>,
}

#[derive(Debug, Clone)]
pub enum ConflictType {
    ResourceCompetition, ValueConflict, PersonalityClash, 
    PowerStruggle, Misunderstanding, Historical
}

#[derive(Debug, Clone)]
pub struct ResolutionAttempt {
    pub method: ResolutionMethod,
    pub success_probability: f32,
    pub cost: f32,
    pub long_term_stability: f32,
}

#[derive(Debug, Clone)]
pub enum ResolutionMethod {
    Negotiation, Mediation, Arbitration, Avoidance, 
    Compromise, Competition, Accommodation
}

#[derive(Debug, Clone)]
pub struct CooperationData {
    pub shared_goal: String,
    pub contribution_requirements: HashMap<Entity, f32>,
    pub benefit_distribution: HashMap<Entity, f32>,
    pub cooperation_stability: f32,
}

#[derive(Debug, Clone)]
pub struct CompetitionData {
    pub competition_type: CompetitionType,
    pub stakes: String,
    pub fairness_perception: HashMap<Entity, f32>,
    pub competitive_intensity: f32,
}

#[derive(Debug, Clone)]
pub enum CompetitionType {
    ZeroSum, PositiveSum, Tournament, Market, Status
}

#[derive(Debug, Clone)]
pub struct ExchangeData {
    pub exchange_type: ExchangeType,
    pub items_exchanged: HashMap<Entity, Vec<String>>,
    pub perceived_fairness: HashMap<Entity, f32>,
    pub transaction_costs: f32,
}

#[derive(Debug, Clone)]
pub enum ExchangeType {
    Economic, Social, Information, Emotional, Favor
}

#[derive(Debug, Clone)]
pub struct RitualData {
    pub ritual_type: RitualType,
    pub cultural_significance: f32,
    pub participation_level: HashMap<Entity, f32>,
    pub collective_efficacy: f32,
}

#[derive(Debug, Clone)]
pub enum RitualType {
    Religious, Ceremonial, Social, Professional, Personal
}

// ============================================================================
// BUNDLE DEFINITIONS FOR COMMON ENTITY ARCHETYPES
// ============================================================================

#[derive(Bundle)]
pub struct BasicNPCBundle {
    pub neurotransmitters: NeurotransmitterSystem,
    pub cognitive_arch: CognitiveArchitecture,
    pub emotional_core: EmotionalCore,
    pub personality: PersonalityProfile,
    pub motivation: MotivationalSystem,
    pub memory: MemorySystem,
    pub decision_making: DecisionMakingSystem,
    pub social_identity: SocialIdentity,
    pub relationships: RelationshipNetwork,
    pub cultural_profile: CulturalProfile,
    pub normative_system: NormativeSystem,
    pub environmental_awareness: EnvironmentalAwareness,
    pub hypergraph_node: HypergraphNode,
    pub hypergraph_edges: HypergraphEdges,
    pub chaotic_behavior: ChaoticBehaviorSystem,
}

#[derive(Bundle)]
pub struct SocialGroupBundle {
    pub group_dynamics: GroupDynamics,
    pub shared_norms: HashMap<String, PerceivedNorm>,
    pub collective_memory: Vec<EpisodicMemory>,
    pub group_identity: String,
    pub cohesion_mechanisms: Vec<String>,
}

// ============================================================================
// ADVANCED SYSTEMS AND FEEDBACK LOOPS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct AllostaticLoadSystem {
    pub current_load: f32,           // 0.0-1.0, where 1.0 is critical
    pub baseline_threshold: f32,     // homeostatic setpoint
    pub adaptation_capacity: f32,    // ability to handle chronic stress
    pub recovery_rate: f32,          // speed of return to baseline
    pub stress_accumulation: f32,    // chronic stress buildup
    pub physiological_costs: HashMap<String, f32>, // organ system impacts
}

#[derive(Component, Debug, Clone)]
pub struct WillpowerSystem {
    pub current_willpower: f32,      // finite resource [0.0-1.0]
    pub max_willpower: f32,          // individual capacity
    pub depletion_rate: f32,         // how fast it depletes under use
    pub recovery_rate: f32,          // restoration speed during rest
    pub glucose_dependency: f32,     // metabolic cost factor
    pub ego_depletion_threshold: f32, // point where System 2 becomes unreliable
    pub impulsivity_multiplier: f32,  // factor by which depletion affects impulse control
}

#[derive(Component, Debug, Clone)]
pub struct DualProcessCognition {
    pub system1_activation: f32,     // automatic/heuristic processing
    pub system2_activation: f32,     // deliberative/analytical processing
    pub cognitive_load: f32,         // current processing demands
    pub switching_cost: f32,         // energy cost to switch between systems
    pub stress_bias_toward_s1: f32,  // stress-induced reliance on heuristics
    pub expertise_domains: HashMap<String, f32>, // areas where S1 is reliable
}

#[derive(Component, Debug, Clone)]
pub struct BeliefSystem {
    pub core_beliefs: HashMap<String, Belief>,
    pub peripheral_beliefs: HashMap<String, Belief>,
    pub belief_network: BeliefNetwork,
    pub cognitive_dissonance_threshold: f32,
    pub dissonance_resolution_strategies: Vec<DissonanceResolution>,
    pub confirmation_bias_strength: f32,
}

#[derive(Debug, Clone)]
pub struct Belief {
    pub content: String,
    pub certainty: f32,              // 0.0-1.0 confidence level
    pub emotional_investment: f32,   // resistance to change
    pub centrality: f32,             // importance to identity
    pub evidence_base: Vec<Evidence>,
    pub last_updated: f32,
    pub update_frequency: f32,
}

#[derive(Debug, Clone)]
pub struct BeliefNetwork {
    pub belief_connections: HashMap<String, Vec<BeliefConnection>>,
    pub consistency_constraints: Vec<ConsistencyConstraint>,
    pub network_coherence: f32,
}

#[derive(Debug, Clone)]
pub struct BeliefConnection {
    pub target_belief: String,
    pub connection_type: ConnectionType,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Supports,         // belief A supports belief B
    Contradicts,      // belief A contradicts belief B
    ImpliedBy,        // belief A is implied by belief B
    Independent,      // no logical relationship
}

#[derive(Debug, Clone)]
pub struct ConsistencyConstraint {
    pub involved_beliefs: Vec<String>,
    pub constraint_type: ConstraintType,
    pub violation_cost: f32,
}

#[derive(Debug, Clone)]
pub enum ConstraintType {
    MutualExclusion,  // beliefs cannot coexist
    Implication,      // if A then B must be true
    Equivalence,      // beliefs must have same truth value
}

#[derive(Debug, Clone)]
pub struct Evidence {
    pub source: EvidenceSource,
    pub reliability: f32,
    pub relevance: f32,
    pub valence: f32,            // supports(+1) or contradicts(-1) belief
    pub vividness: f32,          // memorability factor
}

#[derive(Debug, Clone)]
pub enum EvidenceSource {
    DirectExperience(f32),       // personal experience weight
    SocialTestimony(Entity, f32), // source entity and credibility
    AuthorityFigure(String, f32), // authority type and trust level
    MediaSource(String, f32),     // media type and perceived reliability
    Inference(String),            // derived from other beliefs
}

#[derive(Debug, Clone)]
pub enum DissonanceResolution {
    BeliefChange(f32),           // probability of changing belief
    ActionRationalization(f32),  // probability of reinterpreting action
    ImportanceReduction(f32),    // probability of reducing belief importance
    CompartmentalizationTendency(f32), // probability of isolating contradictions
    SeekingConfirmation(f32),    // probability of seeking confirming evidence
}

#[derive(Component, Debug, Clone)]
pub struct SocialTrustNetwork {
    pub trust_values: HashMap<Entity, TrustProfile>,
    pub trust_propagation_rules: Vec<TrustPropagationRule>,
    pub reputation_system: ReputationSystem,
    pub trust_updating_mechanism: TrustUpdatingMechanism,
    pub default_trust_level: f32,
    pub trust_volatility: f32,       // how quickly trust can change
}

#[derive(Debug, Clone)]
pub struct TrustProfile {
    pub general_trust: f32,          // overall trust in entity
    pub competence_trust: f32,       // trust in their abilities
    pub benevolence_trust: f32,      // trust in their intentions
    pub predictability_trust: f32,   // trust in consistency
    pub domain_specific_trust: HashMap<String, f32>, // context-dependent trust
    pub trust_history: Vec<TrustEvent>,
}

#[derive(Debug, Clone)]
pub struct TrustEvent {
    pub timestamp: f32,
    pub event_type: TrustEventType,
    pub impact_magnitude: f32,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum TrustEventType {
    Cooperation, Defection, Competence, Incompetence,
    Honesty, Deception, Reliability, Unreliability
}

#[derive(Debug, Clone)]
pub struct TrustPropagationRule {
    pub rule_type: PropagationType,
    pub strength: f32,
    pub decay_factor: f32,
    pub path_length_limit: usize,
}

#[derive(Debug, Clone)]
pub enum PropagationType {
    Transitivity,        // if A trusts B and B trusts C, then A somewhat trusts C
    Homophily,          // trust those similar to trusted individuals
    StatusInfluence,    // high-status individuals influence trust networks
    ReputationSpread,   // reputation information spreads through network
}

#[derive(Debug, Clone)]
pub struct ReputationSystem {
    pub reputation_dimensions: HashMap<String, f32>, // different aspects of reputation
    pub reputation_decay_rate: f32,
    pub gossip_propagation_rate: f32,
    pub direct_experience_weight: f32,
    pub hearsay_discount_factor: f32,
}

#[derive(Debug, Clone)]
pub struct TrustUpdatingMechanism {
    pub learning_rate: f32,
    pub forgiveness_factor: f32,     // how much past negative events fade
    pub confirmation_bias: f32,      // tendency to interpret ambiguous events favorably/unfavorably
    pub surprise_sensitivity: f32,   // how much unexpected events affect trust
}

#[derive(Component, Debug, Clone)]
pub struct ScapegoatingSystem {
    pub vulnerability_to_scapegoating: f32,
    pub scapegoating_tendency: f32,
    pub group_anxiety_sensitivity: f32,
    pub attribution_patterns: AttributionPatterns,
    pub social_distance_factors: HashMap<String, f32>, // factors that create "otherness"
    pub crisis_response_mechanisms: Vec<CrisisResponse>,
}

#[derive(Debug, Clone)]
pub struct AttributionPatterns {
    pub fundamental_attribution_error: f32,  // tendency to attribute others' actions to character
    pub in_group_favorability_bias: f32,     // tendency to attribute in-group failures to circumstances
    pub out_group_derogation_bias: f32,      // tendency to attribute out-group success to luck
    pub scapegoat_attribution_bias: f32,     // tendency to blame convenient targets
}

#[derive(Debug, Clone)]
pub struct CrisisResponse {
    pub crisis_type: CrisisType,
    pub scapegoating_probability: f32,
    pub target_selection_criteria: Vec<TargetCriterion>,
    pub mobilization_threshold: f32,
}

#[derive(Debug, Clone)]
pub enum CrisisType {
    ResourceScarcity, Disease, ExternalThreat, SocialChange, 
    EconomicInstability, NaturalDisaster, IdentityThreat
}

#[derive(Debug, Clone)]
pub struct TargetCriterion {
    pub criterion_type: TargetCriterionType,
    pub weight: f32,
    pub threshold: f32,
}

#[derive(Debug, Clone)]
pub enum TargetCriterionType {
    SocialDistance,      // how different from in-group
    Visibility,          // how prominent/noticeable
    Powerlessness,       // inability to retaliate
    PriorAntagonism,     // history of conflict
    SymbolicValue,       // represents something threatening
    Availability,        // physical/social accessibility
}

#[derive(Component, Debug, Clone)]
pub struct InformationPropagationSystem {
    pub information_processing: InformationProcessing,
    pub transmission_rules: Vec<TransmissionRule>,
    pub information_evaluation: InformationEvaluation,
    pub network_position_effects: NetworkPositionEffects,
    pub misinformation_susceptibility: f32,
}

#[derive(Debug, Clone)]
pub struct InformationProcessing {
    pub attention_filter: AttentionFilter,
    pub credibility_assessment: CredibilityAssessment,
    pub emotional_impact_weighting: f32,
    pub novelty_bias: f32,
    pub confirmation_seeking: f32,
}

#[derive(Debug, Clone)]
pub struct AttentionFilter {
    pub personal_relevance_weight: f32,
    pub emotional_salience_weight: f32,
    pub social_relevance_weight: f32,
    pub cognitive_load_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct CredibilityAssessment {
    pub source_credibility_weight: f32,
    pub content_consistency_weight: f32,
    pub social_proof_weight: f32,        // how many others believe it
    pub authority_endorsement_weight: f32,
    pub personal_experience_weight: f32,
}

#[derive(Debug, Clone)]
pub struct TransmissionRule {
    pub rule_type: TransmissionType,
    pub activation_threshold: f32,
    pub transmission_probability: f32,
    pub modification_probability: f32,   // chance of altering information
    pub decay_factor: f32,               // information degradation over transmission
}

#[derive(Debug, Clone)]
pub enum TransmissionType {
    DirectSharing,       // intentional information sharing
    EmotionalContagion,  // spreading emotional states
    Gossip,             // informal social transmission
    FormalCommunication, // official channels
    Rumor,              // unverified information spread
    Propaganda,         // intentionally biased information
}

#[derive(Debug, Clone)]
pub struct InformationEvaluation {
    pub truth_assessment_accuracy: f32,
    pub bias_detection_ability: f32,
    pub source_criticism_tendency: f32,
    pub emotional_override_susceptibility: f32, // emotions overriding logical evaluation
}

#[derive(Debug, Clone)]
pub struct NetworkPositionEffects {
    pub centrality_influence: f32,       // how network position affects influence
    pub bridge_position_awareness: f32,  // awareness of connecting different groups
    pub information_broker_tendency: f32, // likelihood of controlling information flow
    pub echo_chamber_susceptibility: f32, // tendency to remain in similar groups
}

// ============================================================================
// SYSTEM INTERACTION COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct SystemInteractionMatrix {
    pub interaction_weights: Matrix3<f32>, // 3x3 interaction strength matrix
    pub feedback_loops: Vec<SystemFeedbackLoop>,
    pub cascade_thresholds: HashMap<String, f32>,
    pub interference_patterns: Vec<InterferencePattern>,
}

#[derive(Debug, Clone)]
pub struct SystemFeedbackLoop {
    pub loop_id: u32,
    pub involved_systems: Vec<SystemType>,
    pub loop_strength: f32,
    pub delay_parameters: Vec<f32>,
    pub saturation_points: Vec<f32>,
    pub loop_stability: f32,
}

#[derive(Debug, Clone)]
pub enum SystemType {
    Physiological, Cognitive, Social, Cultural, Environmental
}

#[derive(Debug, Clone)]
pub struct InterferencePattern {
    pub pattern_id: u32,
    pub interfering_systems: (SystemType, SystemType),
    pub interference_type: InterferenceType,
    pub magnitude: f32,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub enum InterferenceType {
    Constructive,    // systems amplify each other
    Destructive,     // systems cancel each other out
    Competitive,     // systems compete for resources
    Sequential,      // one system must complete before other activates
}

#[derive(Component, Debug, Clone)]
pub struct EmergentBehaviorTracker {
    pub behavior_patterns: Vec<BehaviorPattern>,
    pub emergence_detectors: Vec<EmergenceDetector>,
    pub phase_transition_indicators: Vec<PhaseTransitionIndicator>,
    pub collective_behavior_metrics: CollectiveBehaviorMetrics,
}

#[derive(Debug, Clone)]
pub struct BehaviorPattern {
    pub pattern_id: u32,
    pub pattern_name: String,
    pub characteristic_features: Vec<String>,
    pub stability_measure: f32,
    pub occurrence_frequency: f32,
    pub contextual_dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EmergenceDetector {
    pub detector_type: EmergenceType,
    pub sensitivity: f32,
    pub detection_threshold: f32,
    pub temporal_window: f32,
    pub false_positive_rate: f32,
}

#[derive(Debug, Clone)]
pub enum EmergenceType {
    SynchronizationEmergence,    // agents synchronizing behavior
    HierarchyEmergence,          // leadership structures forming
    NormEmergence,               // new social norms developing
    ConflictEmergence,           // conflict patterns developing
    CooperationEmergence,        // cooperative structures forming
    InnovationEmergence,         // new behaviors/ideas spreading
}

#[derive(Debug, Clone)]
pub struct PhaseTransitionIndicator {
    pub indicator_name: String,
    pub measurement_function: String, // mathematical description
    pub critical_value: f32,
    pub hysteresis_range: (f32, f32),
    pub order_parameter: f32,
}

#[derive(Debug, Clone)]
pub struct CollectiveBehaviorMetrics {
    pub synchronization_index: f32,
    pub polarization_measure: f32,
    pub clustering_coefficient: f32,
    pub information_flow_rate: f32,
    pub collective_decision_quality: f32,
    pub group_cohesion_strength: f32,
}

// ============================================================================
// MATHEMATICAL MODELS COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct GameTheoreticBehavior {
    pub strategy_set: Vec<Strategy>,
    pub payoff_matrix: PayoffMatrix,
    pub learning_algorithm: LearningAlgorithm,
    pub rationality_level: f32,       // 0.0 = random, 1.0 = perfectly rational
    pub adaptation_rate: f32,
    pub exploration_probability: f32,
}

#[derive(Debug, Clone)]
pub struct Strategy {
    pub strategy_name: String,
    pub strategy_parameters: Vec<f32>,
    pub success_history: Vec<f32>,
    pub usage_frequency: f32,
}

#[derive(Debug, Clone)]
pub struct PayoffMatrix {
    pub self_payoffs: HashMap<(String, String), f32>, // (own_action, other_action) -> payoff
    pub other_payoffs: HashMap<(String, String), f32>,
    pub context_modifiers: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub enum LearningAlgorithm {
    ReinforcementLearning { learning_rate: f32, discount_factor: f32 },
    ImitationLearning { imitation_probability: f32 },
    ExperienceWeightedAttraction { phi: f32, rho: f32 },
    BeliefLearning { initial_beliefs: HashMap<String, f32> },
}

#[derive(Component, Debug, Clone)]
pub struct EpidemicDynamics {
    pub susceptibility: f32,         // S in SIR model
    pub infection_probability: f32,  // transmission rate
    pub recovery_rate: f32,          // recovery rate
    pub immunity_duration: Option<f32>, // how long immunity lasts
    pub contact_patterns: ContactPatterns,
    pub information_epidemic_susceptibility: f32, // for idea/belief spread
}

#[derive(Debug, Clone)]
pub struct ContactPatterns {
    pub contact_rate: f32,           // average contacts per time unit
    pub contact_distribution: ContactDistribution,
    pub spatial_locality: f32,       // preference for local contacts
    pub social_locality: f32,        // preference for similar others
}

#[derive(Debug, Clone)]
pub enum ContactDistribution {
    Poisson(f32),                    // random contacts
    PowerLaw(f32),                   // scale-free network
    SmallWorld(f32, f32),           // clustering and shortcuts
    SpatiallyConstrained(f32),       // geographic limitations
}

// ============================================================================
// INITIALIZATION HELPERS
// ============================================================================

impl Default for NeurotransmitterSystem {
    fn default() -> Self {
        let mut baseline_levels = HashMap::new();
        baseline_levels.insert("dopamine".to_string(), 0.5);
        baseline_levels.insert("serotonin".to_string(), 0.6);
        baseline_levels.insert("oxytocin".to_string(), 0.4);
        baseline_levels.insert("cortisol".to_string(), 0.3);

        let mut decay_rates = HashMap::new();
        decay_rates.insert("dopamine".to_string(), 0.1);
        decay_rates.insert("serotonin".to_string(), 0.05);
        decay_rates.insert("oxytocin".to_string(), 0.08);
        decay_rates.insert("cortisol".to_string(), 0.15);

        let mut synthesis_rates = HashMap::new();
        synthesis_rates.insert("dopamine".to_string(), 0.12);
        synthesis_rates.insert("serotonin".to_string(), 0.08);
        synthesis_rates.insert("oxytocin".to_string(), 0.1);
        synthesis_rates.insert("cortisol".to_string(), 0.2);

        Self {
            dopamine: 0.5,
            serotonin: 0.6,
            oxytocin: 0.4,
            cortisol: 0.3,
            baseline_levels,
            decay_rates,
            synthesis_rates,
        }
    }
}

impl Default for BigFiveTraits {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        }
    }
}

impl Default for MaslowNeeds {
    fn default() -> Self {
        Self {
            physiological: 0.8,
            safety: 0.7,
            belonging: 0.5,
            esteem: 0.4,
            self_actualization: 0.2,
        }
    }
}

impl Default for SelfDeterminationNeeds {
    fn default() -> Self {
        Self {
            autonomy: 0.5,
            competence: 0.5,
            relatedness: 0.5,
        }
    }
}

impl Default for CulturalDimensions {
    fn default() -> Self {
        Self {
            individualism_collectivism: 0.0,
            power_distance: 0.5,
            uncertainty_avoidance: 0.5,
            masculinity_femininity: 0.0,
            long_term_orientation: 0.0,
            indulgence_restraint: 0.0,
        }
    }
}

impl Default for MoralFoundations {
    fn default() -> Self {
        Self {
            care_harm: 0.7,
            fairness_cheating: 0.6,
            loyalty_betrayal: 0.5,
            authority_subversion: 0.4,
            sanctity_degradation: 0.3,
            liberty_oppression: 0.6,
        }
    }
}