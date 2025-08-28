# Artificial Society Implementation Roadmap - Complete SMART Checklist

## Overview
Agent-based AI simulation creating believable NPCs through emergent social dynamics. All states use continuous mathematical normalizations (0.0-1.0) instead of discrete categories.

---

## Phase 1: Foundation Layer (Weeks 1-3) - 15%

### Week 1: Core Type System & Architecture
- [x] **1.1.1** Implement `Normalized<f32>` wrapper type with automatic 0.0-1.0 clamping
- [x] **1.1.2** Create type aliases in `src/core/types.rs` for `EntityId` (u16), `Weight` (u8)
- [x] **1.1.3** Implement conversion utilities between u8 (0-255) and f32 (0.0-1.0)
- [x] **1.1.4** Add runtime validation for normalized ranges in debug and release builds
- [x] **1.2.1** Create `assert_range` utility for value validation
- [x] **1.3.1** Create master `AiPlugin` coordinator in `src/ai/mod.rs`
- [x] **1.3.2** Set up domain plugin structure: Physiology, Cognition, Social, Perception, Environment
- [x] **1.3.3** Implement plugin dependency management and loading order **(factories pattern)**
- [x] **1.3.4** Create centralized event registration system **(helpers pattern)**

**Success Criteria:** All types compile, plugins load without errors, debug validation works

### Week 2: Agent Identity & Development Tools
- [x] **1.4.1** Implement `PersonalityVector` component with Big Five dimensions (0.0-1.0 each)
  - `openness: f32`, `conscientiousness: f32`, `extraversion: f32`, `agreeableness: f32`, `neuroticism: f32`
- [x] **1.4.2** Create `RoleAffinities` component with continuous role preferences (0.0-1.0 each)
  - `leadership_tendency: f32`, `cooperation_drive: f32`, `exploration_urge: f32`, `protection_instinct: f32`
- [x] **1.4.3** Implement `PersonalityShiftSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `StressThresholdCrossed`, `TraumaticEvent`, `SuccessfulLeadership` events
  - **Inputs:** `PersonalityVector`, `StressLevel`, `RecentEvents`, `RoleAffinities`
  - **Processing:** Apply stress-based temporary shifts to personality dimensions using sigmoid curves. High stress increases neuroticism (+0.1-0.3), decreases openness (-0.1-0.2). Traumatic events create permanent small shifts (-0.05 to +0.05). Calculate role affinity changes based on successful/failed leadership attempts.
  - **Outputs:** Modified `PersonalityVector`, updated `RoleAffinities`, `PersonalityShiftEvent`
- [ ] **1.5.1** Integrate `bevy_inspector_egui` for real-time component inspection
- [ ] **1.5.2** Create performance profiler in `src/presentation/profiler.rs`
- [ ] **1.5.3** Set up multi-level logging with domain-specific targets
- [ ] **1.5.4** Configure frame pacing for consistent 60fps

**Success Criteria:** Agents have measurable personality dimensions, debug UI shows real-time values

### Week 3: Basic Expression System
- [ ] **1.6.1** Implement `Needs` component with continuous values (0.0-1.0)
  - `hunger: f32`, `safety: f32`, `social_connection: f32`, `autonomy: f32`
- [ ] **1.6.2** Create `EmotionalState` component with dimensional model
  - `valence: f32` (pleasure/displeasure), `arousal: f32` (activation/calm), `dominance: f32` (control/submission)
- [ ] **1.6.3** Implement `ApparentStateVector` for external expression
  - `tension_level: f32`, `approachability: f32`, `confidence: f32`, `focus_intensity: f32`
- [ ] **1.6.4** Create `SocialExpressionSystem` (**Update Schedule, Polling**)
  - **Frequency:** Every frame (high priority for player observation)
  - **Inputs:** `EmotionalState`, `Needs`, `PersonalityVector`, `StressLevel`
  - **Processing:** Transform internal emotional state through personality filters. Introverted agents (low extraversion) reduce approachability by 0.2-0.4. High stress increases tension_level by stress_value * 0.8. Unmet safety needs decrease confidence by (1.0 - safety) * 0.6. Apply noise (±0.1) to simulate imperfect expression.
  - **Outputs:** Updated `ApparentStateVector`, `ExpressionChangeEvent`
- [ ] **1.7.1** Extend entity builder with agent-specific validation
- [ ] **1.7.2** Add serialization support for builder configurations

**Success Criteria:** Single NPC's ApparentStateVector changes dynamically based on internal state

---

## Phase 2: Physiological Foundation (Weeks 4-6) - 30%

### Week 4: Core Needs & Stress
- [ ] **2.1.1** Create `needs_decay_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 5 seconds (performance optimization)
  - **Inputs:** `Needs`, `EnergyLevel`, `ActivityLevel`, `EnvironmentFactors`, `Time`
  - **Processing:** Apply time-based decay rates: hunger increases by 0.05/hour, social_connection decreases by 0.03/hour when isolated, safety decreases by 0.1/hour in dangerous areas. Multiply decay rates by (2.0 - energy_level) to simulate fatigue effects. Apply environmental modifiers (cold increases hunger decay by 1.5x).
  - **Outputs:** Updated `Needs`, `NeedCritical` events when values exceed 0.8, `NeedSatisfied` events when values drop below 0.3
- [ ] **2.1.2** Implement `calculate_overall_distress()` utility function
- [ ] **2.1.3** Add `NeedCritical` and `NeedSatisfied` events with entity references
- [ ] **2.1.4** Create `StressLevel` component (0.0-1.0) with threshold-based state changes
- [ ] **2.2.1** Implement `EnergyLevel` component affecting all other systems
- [ ] **2.2.2** Create `energy_management_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 10 seconds (lower priority than needs)
  - **Inputs:** `EnergyLevel`, `ActivityLevel`, `Needs`, `EmotionalState`, `Time`
  - **Processing:** Decrease energy by activity_level * 0.02/minute. High stress (arousal > 0.7) increases energy drain by 1.5x. Resting (activity_level < 0.2) recovers energy by 0.05/minute. Hunger above 0.6 reduces energy recovery by 50%. Apply circadian rhythm modifier (0.8x recovery during "day", 1.2x during "night").
  - **Outputs:** Updated `EnergyLevel`, `EnergyDepleted` event when < 0.2, `EnergyRestored` event when > 0.8
- [ ] **2.2.3** Link energy level to social perception accuracy and decision speed

**Success Criteria:** Needs decay at observable rates, stress affects behavior, energy impacts performance

### Week 5: System Integration & Emotional Contagion
- [ ] **2.3.1** Implement `environmental_threat_detection_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 2 seconds (safety-critical)
  - **Inputs:** `Transform`, `EnvironmentFeatures`, `PersonalityVector`, `RecentMemory`
  - **Processing:** Scan 5-unit radius for threat markers. Calculate threat_score based on proximity (1/distance²) and threat_intensity. Apply personality modifiers: high neuroticism multiplies by 1.5x, low agreeableness adds 0.2 to threat perception. Check recent memory for negative experiences in similar locations (adds 0.3 to threat_score).
  - **Outputs:** Updated `Needs.safety`, `ThreatDetected` event with threat_level and location
- [ ] **2.3.2** Create `isolation_detection_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 30 seconds (social needs change slowly)
  - **Inputs:** `Transform`, `SocialMemory`, `PersonalityVector`, `Time`
  - **Processing:** Count nearby agents within 3-unit radius. Calculate time_since_last_interaction from SocialMemory. If no agents nearby for > 30 minutes, increase social_need by 0.1/hour. High extraversion multiplies isolation effect by 1.8x. Low extraversion reduces effect by 0.6x.
  - **Outputs:** Updated `Needs.social_connection`, `IsolationDetected` event when social_need > 0.7
- [ ] **2.3.3** Add `stress_response_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `NeedCritical`, `ThreatDetected`, `SocialConflict`, `TraumaticEvent` events
  - **Inputs:** `Needs`, `EmotionalState`, `PersonalityVector`, `RecentEvents`
  - **Processing:** Calculate stress from unmet needs: sum of (need_value * need_weight) where critical needs have weight 2.0. Add emotional arousal * 0.4. High neuroticism adds 0.2 baseline stress. Recent negative events add 0.1-0.3 stress with exponential decay over 24 hours.
  - **Outputs:** Updated `StressLevel`, `CognitiveClarity` (1.0 - stress_level * 0.6), `StressThresholdCrossed` event
- [ ] **2.4.1** Create `mood_calculation_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `NeedSatisfied`, `SocialInteractionComplete`, `PositiveEvent`, `NegativeEvent` events
  - **Inputs:** `Needs`, `EmotionalState`, `SocialMemory`, `RecentEvents`, `PersonalityVector`
  - **Processing:** Calculate valence from needs satisfaction: average of (1.0 - need_value) for all needs. Add recent positive social interactions (+0.1 to +0.3 valence). Subtract recent negative events (-0.1 to -0.4 valence). Apply personality baseline: high neuroticism reduces valence by 0.2, high extraversion adds 0.1.
  - **Outputs:** Updated `EmotionalState.valence`, `MoodChange` event when change > 0.2
- [ ] **2.4.2** Implement `EmotionalContagionSystem` (**Update Schedule, Polling**)
  - **Frequency:** Every 3 seconds (emotions spread relatively quickly)
  - **Inputs:** `Transform`, `EmotionalState`, `PersonalityVector`, `RelationshipComponent`
  - **Processing:** For each agent within 2-unit radius, calculate emotional influence based on distance (1/distance), relationship trust (0.5-1.5 multiplier), and target's openness (0.7-1.3 multiplier). Transfer emotion_delta = source_emotion * influence_strength * contagion_rate (0.1). Apply stronger contagion for negative emotions (fear, anger) with 1.5x multiplier.
  - **Outputs:** Updated `EmotionalState` for nearby agents, `EmotionalContagion` event with source and affected entities
- [ ] **2.4.3** Tune contagion parameters: factor (0.1), sociality multiplier (1.0), distance decay (1.5)
- [ ] **2.4.4** Test panic propagation through agent groups

**Success Criteria:** Mood changes affect expressions, emotional states spread through groups

### Week 6: Validation & Optimization
- [ ] **2.5.1** Connect all physiological systems for coherent behavior
- [ ] **2.5.2** Conduct player intuition testing for emotional state readability
- [ ] **2.5.3** Validate personality differences create distinct character feels
- [ ] **2.6.1** Profile system performance with 100+ agents
- [ ] **2.6.2** Optimize for consistent 60fps target
- [ ] **2.6.3** Remove systems that don't affect player experience

**Success Criteria:** 60fps with 100+ agents, players can read emotional states intuitively

---

## Phase 3: Cognitive Architecture (Weeks 7-10) - 50%

### Week 7: Perception & Bias
- [ ] **3.1.1** Create `PerceptionBiasProfile` with continuous bias strengths
  - `familiarity_bias: f32`, `threat_salience: f32`, `goal_relevance_filter: f32`
- [ ] **3.1.2** Implement `SensoryProcessingSystem` (**Update Schedule, Polling**)
  - **Frequency:** Every frame (critical for player interaction)
  - **Inputs:** `Transform`, `VisionRange`, `EnvironmentFeatures`, `OtherAgents`, `EnergyLevel`
  - **Processing:** Detect all entities within vision_range (modified by energy: tired agents have 0.7x range). Create `RawSensoryInput` with entity positions, apparent states, and environment features. Apply attention limits: can only track 3-5 entities simultaneously, prioritizing by proximity and movement.
  - **Outputs:** `RawSensoryInput` component, `EntityDetected` events for new entities in range
- [ ] **3.1.3** Create `BiasApplicationSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `EntityDetected`, `SensoryInputChanged` events
  - **Inputs:** `RawSensoryInput`, `PerceptionBiasProfile`, `PersonalityVector`, `EmotionalState`, `SocialMemory`
  - **Processing:** Apply familiarity_bias: known entities get 0.8-1.2x attention multiplier. Apply threat_salience: multiply perceived threat by (1.0 + threat_salience). Filter by goal_relevance: entities matching current goals get 1.5x attention. High neuroticism increases threat perception by 0.3, low agreeableness adds 0.2 to hostility perception.
  - **Outputs:** `FilteredPerception` component with biased entity assessments
- [ ] **3.1.4** Add `CategorizationSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `FilteredPerception` updates
  - **Inputs:** `FilteredPerception`, `SchemaMemory`, `PersonalityVector`
  - **Processing:** Match perceived entities against stored schemas (friend/neutral/threat categories). Use similarity scoring based on appearance, behavior patterns, and past interactions. Apply personality modifiers: high openness creates more nuanced categories, low openness forces binary friend/threat classification.
  - **Outputs:** `CategorizedEntities` with assigned categories and confidence scores, `NewSchemaFormed` event for novel entity types
- [ ] **3.1.5** Test misinterpretation of player actions based on bias profiles

**Success Criteria:** Same object perceived differently by agents with different bias profiles

### Week 8: Decision Making & Personality
- [ ] **3.2.1** Implement `DecisionWeightingProfile` with continuous preference weights
- [ ] **3.2.2** Create `UtilityCalculationSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `NewGoalSet`, `EnvironmentChanged`, `NeedCritical` events
  - **Inputs:** `AvailableActions`, `Needs`, `EmotionalState`, `PersonalityVector`, `DecisionWeightingProfile`, `PerceivedEnvironment`
  - **Processing:** For each available action, calculate utility score: base_utility + need_satisfaction_bonus + personality_modifier + emotional_modifier. Need satisfaction: action that reduces hunger gets utility = hunger_level * 0.8. Personality modifiers: high conscientiousness adds 0.2 to planning actions, high extraversion adds 0.3 to social actions. Emotional modifiers: high arousal reduces utility of slow actions by 0.4.
  - **Outputs:** `ActionUtilities` ranked list, `DecisionConflict` event when top options have similar scores
- [ ] **3.2.3** Add personality influence on decision weights
- [ ] **3.3.1** Create `ActionTendencies` component with continuous drives
  - `exploration_drive: f32`, `social_seeking: f32`, `resource_focus: f32`, `safety_priority: f32`
- [ ] **3.3.2** Implement `PersonalityExpressionSystem` (**Update Schedule, Polling**)
  - **Frequency:** Every 15 seconds (personality changes slowly)
  - **Inputs:** `PersonalityVector`, `CurrentContext`, `RecentActions`, `SocialFeedback`
  - **Processing:** Map Big Five traits to action tendencies: openness → exploration_drive (0.7x to 1.3x multiplier), extraversion → social_seeking (0.5x to 1.5x), conscientiousness → resource_focus (0.8x to 1.2x), neuroticism → safety_priority (1.0x to 1.8x). Apply context modifiers: dangerous environments increase safety_priority by 0.3, social environments increase social_seeking by 0.2.
  - **Outputs:** Updated `ActionTendencies`, `PersonalityExpressed` event with dominant tendency
- [ ] **3.4.1** Create `MoodInfluenceSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `MoodChange`, `EmotionalStateChanged` events
  - **Inputs:** `EmotionalState`, `DecisionWeightingProfile`, `AvailableActions`
  - **Processing:** Apply mood-based decision modifiers: positive valence increases social action weights by 0.2, negative valence increases safety action weights by 0.3. High arousal increases immediate action weights by 0.4, reduces planning action weights by 0.3. Low dominance reduces leadership action weights by 0.5.
  - **Outputs:** Modified `DecisionWeightingProfile`, `MoodDecisionInfluence` event
- [ ] **3.4.2** Implement `emotional_override_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `HighEmotionalArousal`, `PanicState`, `RageState` events
  - **Inputs:** `EmotionalState`, `SelectedAction`, `PersonalityVector`, `StressLevel`
  - **Processing:** Check for emotional override conditions: fear > 0.8 forces flee actions, anger > 0.7 forces aggressive actions, stress > 0.9 forces immediate need satisfaction. Apply personality resistance: high conscientiousness reduces override probability by 0.3, high neuroticism increases by 0.2. Calculate override_probability = emotion_intensity * stress_multiplier * personality_modifier.
  - **Outputs:** Potentially modified `SelectedAction`, `EmotionalOverride` event when override occurs

**Success Criteria:** Personality creates consistent character feel, emotions influence decisions

### Week 9: Memory & Learning
- [ ] **3.6.1** Implement `SocialMemory` component storing interaction history
- [ ] **3.6.2** Create `RelationshipComponent` with continuous relationship dimensions
  - `trust_level: f32`, `familiarity: f32`, `emotional_attachment: f32`, `perceived_status: f32`
- [ ] **3.6.3** Add `InteractionLoggingSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `SocialInteractionComplete`, `ConflictResolved`, `SharedExperience` events
  - **Inputs:** `SocialInteraction` events, `SocialMemory`, `RelationshipComponent`, `EmotionalState`
  - **Processing:** Record interaction details: type, outcome, emotional context, witnesses. Update relationship dimensions: positive interactions increase trust by 0.05-0.15, increase familiarity by 0.02-0.08. Negative interactions decrease trust by 0.1-0.3. Apply emotional weighting: interactions during high arousal have 1.5x impact on emotional_attachment.
  - **Outputs:** Updated `SocialMemory` with new interaction record, modified `RelationshipComponent`, `RelationshipChanged` event
- [ ] **3.6.4** Implement `MemoryDecaySystem` (**Update Schedule, Polling**)
  - **Frequency:** Every 1 hour (memory changes slowly)
  - **Inputs:** `SocialMemory`, `RelationshipComponent`, `Time`, `PersonalityVector`
  - **Processing:** Apply time-based decay to memory vividness and relationship strength. Recent memories (< 24 hours) retain full strength, older memories decay exponentially with half-life of 7 days. Strong emotional memories decay 50% slower. High openness increases memory retention by 20%, high neuroticism increases negative memory retention by 40%.
  - **Outputs:** Updated memory vividness scores, adjusted relationship strengths, `MemoryFaded` event for forgotten interactions
- [ ] **3.7.1** Create `LearningSystem` (**Update Schedule, Event-Driven**)
  - **Triggers:** `ActionOutcome`, `SocialFeedback`, `SuccessfulAction`, `FailedAction` events
  - **Inputs:** `ActionOutcome` events, `DecisionWeightingProfile`, `PersonalityVector`, `EmotionalState`
  - **Processing:** Update action preferences based on outcomes: successful actions increase weight by 0.05-0.2, failed actions decrease by 0.1-0.3. Apply personality learning modifiers: high openness learns faster (+50% weight changes), low openness learns slower (-30%). Emotional context affects learning: positive emotions during success increase learning by 30%, negative emotions during failure increase avoidance learning by 50%.
  - **Outputs:** Modified `DecisionWeightingProfile`, `LearningOccurred` event with learned behavior
- [ ] **3.7.2** Add personality influence on learning speed and generalization

**Success Criteria:** Agents remember interactions, relationships evolve over time

### Week 10: Integration & Validation
- [ ] **3.8.1** Connect personality, mood, decisions, and memory systems
- [ ] **3.8.2** Test behavioral consistency over time
- [ ] **3.8.3** Validate emotional logic in decision-making
- [ ] **3.8.4** Conduct player prediction testing for agent behavior

**Success Criteria:** Agents feel like consistent characters, decisions are emotionally logical

---

## Phase 4: Social Communication Pipeline (Weeks 11-15) - 70%

### Week 11: Expression Refinement
- [ ] **4.1.1** Add personality-based filtering to `SocialExpressionSystem`
- [ ] **4.1.2** Implement `stress_expression_modulation_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `StressThresholdCrossed`, `SocialContextChanged` events
  - **Inputs:** `StressLevel`, `PersonalityVector`, `ApparentStateVector`, `SocialContext`
  - **Processing:** Apply stress-based expression distortion: high stress (>0.7) can either amplify expression (1.5x tension_level) or suppress it (0.3x approachability). Personality determines response: high neuroticism amplifies stress expression, high conscientiousness suppresses it. Social context modifiers: public settings reduce stress expression by 0.4, private settings allow full expression.
  - **Outputs:** Modified `ApparentStateVector`, `ExpressionDistortion` event with distortion type and intensity
- [ ] **4.1.3** Create gap between internal state and external expression
- [ ] **4.1.4** Test "tense stillness" vs "obvious stress" expression patterns

**Success Criteria:** Observable differences between internal state and apparent state

### Weeks 12-13: Advanced Perception & Belief Formation
- [ ] **4.2.1** Enhance `VisionSystem` with observer state filtering
- [ ] **4.2.2** Add `attention_management_system` (**Update Schedule, Polling**)
  - **Frequency:** Every frame (attention is critical for social interaction)
  - **Inputs:** `PerceivedEntities`, `CurrentGoals`, `EmotionalState`, `EnergyLevel`, `PersonalityVector`
  - **Processing:** Allocate attention points (3-7 based on energy) among perceived entities. Priority scoring: goal-relevant entities get 2x points, threatening entities get 3x points, familiar entities get 1.5x points. High arousal reduces total attention by 20% but increases threat focus by 50%. Tired agents (energy < 0.4) have 40% reduced attention capacity.
  - **Outputs:** `AttentionAllocation` with focus levels per entity, `AttentionOverload` event when too many high-priority targets
- [ ] **4.2.3** Implement `mood_perception_filter_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `MoodChange`, `EmotionalStateChanged` events
  - **Inputs:** `EmotionalState`, `PerceivedEntities`, `PersonalityVector`
  - **Processing:** Apply mood-based perception distortion: negative valence increases perceived threat by 0.2-0.5, positive valence increases perceived friendliness by 0.1-0.3. High arousal amplifies all perceptions by 1.3x. Low dominance reduces confidence in perceptions by 0.3. Apply personality filters: high agreeableness adds 0.2 to perceived friendliness, low agreeableness subtracts 0.3.
  - **Outputs:** `DistortedPerceptions` with mood-filtered entity assessments
- [ ] **4.3.1** Create `BeliefSystem` for opinion formation about others
- [ ] **4.3.2** Implement `belief_formation_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `NewObservation`, `SocialInteractionComplete`, `WitnessedEvent` events
  - **Inputs:** `DistortedPerceptions`, `SocialMemory`, `PersonalityVector`, `CurrentBeliefs`
  - **Processing:** Form beliefs about entity intentions and character: aggregate recent observations with memory-weighted averaging. Apply confirmation bias: new evidence matching existing beliefs gets 1.5x weight, contradicting evidence gets 0.7x weight. Strong beliefs (confidence > 0.8) resist change with 0.5x update rate. Personality modifiers: high openness reduces confirmation bias by 30%, low agreeableness increases negative belief formation by 40%.
  - **Outputs:** Updated `BeliefSystem` with entity assessments and confidence scores, `BeliefChanged` event for significant updates
- [ ] **4.3.3** Add first impression effects with persistence
- [ ] **4.3.4** Create `InterpretationSystem` for intention inference (**Update Schedule, Event-Driven**)
  - **Triggers:** `ActionObserved`, `BehaviorWitnessed` events
  - **Inputs:** `ObservedActions`, `BeliefSystem`, `PerceptionBiasProfile`, `SocialContext`
  - **Processing:** Infer intentions from observed actions using belief priors and bias filters. Positive relationships bias toward benevolent interpretations (+0.3 to intention assessment), negative relationships bias toward malevolent (-0.4). Apply context: same action interpreted differently in different social settings.
  - **Outputs:** `InferredIntentions` with confidence scores, `IntentionMisinterpreted` event when inference differs significantly from actual intent
- [ ] **4.3.5** Tune inference accuracy and bias influence parameters

**Success Criteria:** Same action interpreted differently by different agents

### Weeks 14-15: Interaction & Misunderstanding Generation
- [ ] **4.4.1** Implement `social_approach_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `SocialNeedHigh`, `PositiveBeliefFormed`, `GroupActivityDetected` events
  - **Inputs:** `ActionTendencies`, `PerceivedEntities`, `RelationshipComponent`, `CurrentNeeds`, `PersonalityVector`
  - **Processing:** Calculate approach motivation for each perceived entity: base_motivation = social_seeking * (1.0 - social_connection_need). Apply relationship modifiers: high trust adds 0.3, low trust subtracts 0.5. Apply personality modifiers: high extraversion adds 0.4, high neuroticism subtracts 0.2 in unfamiliar situations. Consider entity status: high-status entities get approach bonus for low-dominance agents.
  - **Outputs:** `SocialApproachIntent` with target entity and motivation strength, `ApproachDecision` event
- [ ] **4.4.2** Add `communication_style_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `SocialInteractionInitiated`, `RelationshipChanged` events
  - **Inputs:** `PersonalityVector`, `EmotionalState`, `RelationshipComponent`, `SocialContext`
  - **Processing:** Determine communication style based on personality: high agreeableness increases cooperative language by 0.3, low agreeableness increases direct/blunt language by 0.4. High extraversion increases expressiveness by 0.5, low extraversion reduces by 0.3. Apply emotional modifiers: high arousal increases intensity by 0.4, negative valence increases cautious language by 0.3.
  - **Outputs:** `CommunicationStyle` component with style parameters, `StyleAdopted` event
- [ ] **4.4.3** Create `interaction_outcome_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `SocialInteractionAttempt` events
  - **Inputs:** `SocialInteraction` events, `CommunicationStyle`, `RelationshipComponent`, `PersonalityCompatibility`, `EmotionalState`
  - **Processing:** Calculate interaction success probability: base_success = (trust_level + personality_compatibility) / 2. Apply communication style matching: similar styles get +0.2 bonus, conflicting styles get -0.3 penalty. Emotional state effects: positive emotions add 0.1-0.3, negative emotions subtract 0.2-0.4. Generate outcome based on probability with random variance (±0.2).
  - **Outputs:** `InteractionOutcome` with success/failure and impact scores, relationship updates
- [ ] **4.5.1** Implement `communication_pipeline_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `CommunicationAttempt` events
  - **Inputs:** `InternalState`, `ApparentStateVector`, `DistortedPerceptions`, `BeliefSystem`
  - **Processing:** Execute full pipeline: Agent A's internal state → expression filtering → Agent B's perception filtering → Agent B's interpretation through beliefs. Track information loss at each stage: expression accuracy (0.6-0.9), perception accuracy (0.5-0.8), interpretation accuracy (0.4-0.7). Calculate final understanding accuracy as product of all stages.
  - **Outputs:** `CommunicationResult` with intended vs. understood message, `MisunderstandingDetected` event when accuracy < 0.3
- [ ] **4.5.2** Create `misunderstanding_cascade_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `MisunderstandingDetected`, `RumorSpread` events
  - **Inputs:** `MisunderstandingDetected` events, `SocialNetwork`, `EmotionalContagion`, `GroupMembership`
  - **Processing:** Propagate misunderstandings through social networks: each misunderstood agent spreads their interpretation to connected agents with 0.6-0.8 fidelity. Apply emotional amplification: negative emotions increase spread rate by 1.5x. Group dynamics: in-group members trust interpretations more (+0.3 credibility), out-group interpretations get -0.4 credibility.
  - **Outputs:** `CascadingMisunderstanding` events, updated group beliefs, `SocialTension` increases
- [ ] **4.5.3** Test persistent negative relationships from early misunderstandings
- [ ] **4.5.4** Validate believable social conflict generation

**Success Criteria:** Misunderstandings create believable conflicts, players understand why they happen

---

## Phase 5: Group Dynamics (Weeks 16-18) - 85%

### Week 16: Enhanced Social Influence
- [ ] **5.1.1** Refine `emotional_contagion_system` with relationship modifiers
- [ ] **5.1.2** Add `personality_influence_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 10 seconds (social influence is gradual)
  - **Inputs:** `PersonalityVector`, `SocialNetwork`, `GroupMembership`, `SocialStatus`
  - **Processing:** Calculate influence strength based on personality: high extraversion + high dominance = strong influencer (1.5x influence), high agreeableness = susceptible to influence (1.3x reception). Apply network position: central agents have 1.4x influence, peripheral agents have 0.7x. Status modifiers: high-status agents get 1.6x influence on low-status agents.
  - **Outputs:** `InfluenceStrength` ratings between agents, `InfluenceAttempt` events with success probability
- [ ] **5.1.3** Implement `group_mood_emergence_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 5 seconds (group mood changes moderately fast)
  - **Inputs:** `GroupMembership`, `EmotionalState`, `InfluenceStrength`, `SocialNetwork`
  - **Processing:** Calculate group emotional center as weighted average of member emotions, weighted by influence strength. Apply convergence pressure: each member's emotion moves toward group center by 0.1-0.3 per update. Resistant personalities (low agreeableness) converge 50% slower. Emotional leaders (high influence + extreme emotions) pull group mood more strongly.
  - **Outputs:** `GroupMood` component, `MoodConvergence` events, individual `EmotionalState` updates
- [ ] **5.1.4** Test collective behaviors (panic, celebration)

**Success Criteria:** Group mood emerges naturally, collective behaviors observable

### Week 17: Group Formation
- [ ] **5.2.1** Create `group_formation_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 60 seconds (group formation is slow)
  - **Inputs:** `PersonalityVector`, `SocialMemory`, `ProximityHistory`, `SharedExperiences`, `ValueAlignment`
  - **Processing:** Calculate compatibility scores between agents: personality similarity (cosine similarity of Big Five vectors), shared positive experiences (+0.2 per experience), value alignment on goals (+0.3 for matching priorities). Form groups when compatibility > 0.6 and proximity frequency > 0.4. Apply group size limits: optimal size 3-7 agents, larger groups split, smaller groups recruit.
  - **Outputs:** `GroupMembership` assignments, `GroupFormed` events, `GroupDissolved` events for incompatible groups
- [ ] **5.2.2** Implement `proximity_tracking_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 30 seconds (proximity tracking doesn't need high frequency)
  - **Inputs:** `Transform`, `SocialNetwork`, `Time`
  - **Processing:** Track time spent within 3-unit radius of other agents over rolling 24-hour window. Calculate proximity_frequency = time_together / total_time. Weight recent proximity higher (exponential decay with 6-hour half-life). Apply activity context: shared activities (eating, working) get 2x proximity weight.
  - **Outputs:** `ProximityHistory` with frequency scores, `ProximityBond` events for high-frequency pairs
- [ ] **5.2.3** Add `shared_experience_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `GroupActivity`, `WitnessedEvent`, `CollectiveAction` events
  - **Inputs:** `GroupMembership`, `ActivityParticipation`, `EmotionalState`, `EventWitness`
  - **Processing:** Detect shared experiences: same location + same time + similar emotional response. Positive shared experiences (average valence > 0.6) create bonding (+0.1 to +0.3 relationship strength). Negative shared experiences create trauma bonds or mutual avoidance based on personality: high neuroticism creates avoidance, high agreeableness creates support bonds.
  - **Outputs:** `SharedExperience` records, updated `RelationshipComponent`, `BondingEvent` notifications
- [ ] **5.2.4** Test natural group formation without explicit programming

**Success Criteria:** Groups form based on compatibility, similar personalities cluster

### Week 18: Conflict & Resolution
- [ ] **5.3.1** Implement `conflict_emergence_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `MisunderstandingDetected`, `ResourceCompetition`, `PersonalityClash`, `TerritoryDispute` events
  - **Inputs:** `MisunderstandingDetected` events, `ResourceCompetition`, `PersonalityClashes`, `GroupTensions`
  - **Processing:** Detect conflict triggers: misunderstandings with trust_loss > 0.3, resource competition when multiple agents target same resource, personality clashes (low agreeableness + high neuroticism combinations). Calculate conflict escalation probability based on: emotional state (high arousal increases by 0.4), personality (low agreeableness + high neuroticism = 0.6 base probability), group support (in-group backing increases by 0.3).
  - **Outputs:** `ConflictInitiated` events with participants and intensity, `ConflictEscalation` tracking
- [ ] **5.3.2** Create `resource_competition_system` (**Update Schedule, Event-Driven**)
  - **Triggers:** `ResourceTargeted`, `ResourceScarcity` events
  - **Inputs:** `ResourceTargets`, `AgentGoals`, `ProximityToResources`, `GroupMembership`
  - **Processing:** Detect when multiple agents target same resource within time window. Calculate competition intensity based on: resource scarcity (fewer resources = higher intensity), agent need urgency (critical needs add 0.5 intensity), group dynamics (inter-group competition gets 1.5x intensity). Apply personality modifiers: high conscientiousness reduces aggressive competition by 0.3.
  - **Outputs:** `ResourceConflict` events, `CompetitionIntensity` scores, potential `ConflictEscalation`
- [ ] **5.3.3** Add `personality_clash_detection_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 20 seconds (personality clashes develop over time)
  - **Inputs:** `PersonalityVector`, `CommunicationStyle`, `InteractionHistory`, `StressLevel`
  - **Processing:** Identify problematic personality combinations: low agreeableness + low agreeableness = mutual hostility (0.7 clash probability), high dominance + high dominance = power struggle (0.6 clash probability), high neuroticism + low conscientiousness = frustration (0.5 clash probability). Apply stress amplification: high stress increases clash probability by 0.3.
  - **Outputs:** `PersonalityClash` events, `ClashProbability` scores between agent pairs
- [ ] **5.3.4** Implement `conflict_resolution_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 2 minutes (conflict resolution is slow)
  - **Inputs:** `ActiveConflicts`, `PersonalityVector`, `SocialNetwork`, `GroupMediators`, `TimeElapsed`
  - **Processing:** Apply resolution mechanisms: natural cooling (conflicts decay by 0.05/day), personality-based forgiveness (high agreeableness forgives 2x faster), social mediation (mutual friends reduce conflict by 0.2), resource abundance (reduces resource conflicts by 0.4). Some conflicts become persistent grudges (low agreeableness + high neuroticism = 0.3 persistence probability).
  - **Outputs:** `ConflictResolved` events, `PersistentGrudge` markers, updated `RelationshipComponent`
- [ ] **5.4.1** Conduct emergent narrative analysis through interaction logging
- [ ] **5.4.2** Validate intuitive group dynamics understanding
- [ ] **5.4.3** Test stable belief formation and persistent relationships

**Success Criteria:** Believable conflicts arise and resolve, long-term grudges/alliances form

---

## Phase 6: Polish & Optimization (Weeks 19-21) - 100%

### Week 19: Performance Optimization
- [ ] **6.1.1** Achieve 60 FPS with 100+ agents on mid-range hardware
- [ ] **6.1.2** Implement `simulation_lod_system` (**Update Schedule, Polling**)
  - **Frequency:** Every 5 seconds (LOD changes don't need high frequency)
  - **Inputs:** `PlayerPosition`, `AgentPositions`, `DistanceToPlayer`, `AgentImportance`
  - **Processing:** Divide world into zones: active (0-50 units from player), adjacent (50-100 units), distant (100+ units). Active zone: full simulation at 60fps. Adjacent zone: reduced simulation at 20fps, simplified social interactions. Distant zone: statistical simulation at 5fps, major events only. Apply importance modifiers: quest-relevant agents maintain higher LOD regardless of distance.
  - **Outputs:** `SimulationLOD` assignments per agent, `LODTransition` events, performance metrics
- [ ] **6.1.3** Optimize memory usage for long-running simulations
- [ ] **6.1.4** Ensure social interaction latency under 16ms

**Success Criteria:** Consistent 60 FPS with target agent count, smooth interactions

### Week 20: Behavioral Tuning
- [ ] **6.2.1** Balance personality effect strengths for believability
- [ ] **6.2.2** Tune mood change rates for natural feel
- [ ] **6.2.3** Adjust perception and belief formation parameters
- [ ] **6.2.4** Balance conflict frequency for engaging but not chaotic gameplay
- [ ] **6.2.5** Conduct `behavioral_divergence_analysis_system` (**Startup Schedule, Event-Driven**)
  - **Triggers:** `AnalysisRequested`, `BehaviorLogFull` events
  - **Inputs:** `AgentBehaviorLogs`, `HumanBaselineBehaviors`, `PlayerFeedback`, `BehaviorMetrics`
  - **Processing:** Compare agent behavior patterns against human baseline data: decision consistency (should be 0.6-0.8), emotional responsiveness (should match human variance), social interaction patterns (should show realistic relationship formation rates). Identify deviations that enhance vs. harm believability. Flag behaviors that are too predictable (consistency > 0.9) or too random (consistency < 0.4).
  - **Outputs:** `BehaviorAnalysis` reports, `TuningRecommendations`, `BeliebabilityScore` metrics

**Success Criteria:** Agent behavior feels natural, players can predict behavior intuitively

### Week 21: Final Integration & Testing
- [ ] **6.3.1** Complete end-to-end system integration testing
- [ ] **6.3.2** Validate all SMART objectives met
- [ ] **6.3.3** Conduct final player experience validation
- [ ] **6.3.4** Document system parameters and tuning guidelines
- [ ] **6.3.5** Prepare for production deployment

**Success Criteria:** All systems integrated, player experience goals achieved, ready for deployment

---

## System Schedule Summary

### Update Schedule - Polling Systems
- **High Frequency (Every Frame):** `SocialExpressionSystem`, `SensoryProcessingSystem`, `attention_management_system`
- **Medium Frequency (2-10 seconds):** `needs_decay_system`, `energy_management_system`, `environmental_threat_detection_system`, `EmotionalContagionSystem`, `personality_influence_system`, `group_mood_emergence_system`, `simulation_lod_system`
- **Low Frequency (30+ seconds):** `isolation_detection_system`, `PersonalityExpressionSystem`, `MemoryDecaySystem`, `group_formation_system`, `proximity_tracking_system`, `personality_clash_detection_system`, `conflict_resolution_system`

### Update Schedule - Event-Driven Systems
- **Reactive Systems:** `PersonalityShiftSystem`, `stress_response_system`, `mood_calculation_system`, `BiasApplicationSystem`, `CategorizationSystem`, `UtilityCalculationSystem`, `MoodInfluenceSystem`, `emotional_override_system`, `InteractionLoggingSystem`, `LearningSystem`, `stress_expression_modulation_system`, `mood_perception_filter_system`, `belief_formation_system`, `InterpretationSystem`, `social_approach_system`, `communication_style_system`, `interaction_outcome_system`, `communication_pipeline_system`, `misunderstanding_cascade_system`, `shared_experience_system`, `conflict_emergence_system`, `resource_competition_system`

### Startup Schedule
- **Analysis Systems:** `behavioral_divergence_analysis_system`

---

## Success Metrics

### Technical Performance
- [ ] 60 FPS with 100+ agents
- [ ] Memory usage scales linearly
- [ ] Social interaction latency < 16ms

### Player Experience
- [ ] Players can read agent emotional states intuitively
- [ ] Agent behavior feels natural and engaging
- [ ] Misunderstandings create believable drama
- [ ] Groups form and conflict naturally

### Emergent Behavior
- [ ] Persistent relationships develop from interactions
- [ ] Collective behaviors emerge from individual rules
- [ ] Social conflicts arise from believable causes
- [ ] Agents develop unique "personalities" over time

