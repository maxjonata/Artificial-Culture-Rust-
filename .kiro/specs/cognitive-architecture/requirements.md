# Requirements Document

## Introduction

The Cognitive Architecture feature implements decision-making systems that feel human through their flaws, biases, and emotional logic. This system creates NPCs that make decisions based on personality traits, emotional states, memories, and learned behaviors rather than optimal calculations. The architecture supports both fast emotional reactions (System 1) and slower deliberative thinking (System 2), creating believable cognitive diversity among agents.

## Requirements

### Requirement 1: Personality-Based Decision Modulation

**User Story:** As a player, I want NPCs to have distinct personalities that consistently influence their decision-making, so that each character feels unique and predictable in their own way.

#### Acceptance Criteria

1. WHEN an agent has high openness THEN the system SHALL increase exploration_drive by 0.7x to 1.3x multiplier
2. WHEN an agent has high extraversion THEN the system SHALL increase social_seeking by 0.5x to 1.5x multiplier
3. WHEN an agent has high conscientiousness THEN the system SHALL increase resource_focus by 0.8x to 1.2x multiplier
4. WHEN an agent has high neuroticism THEN the system SHALL increase safety_priority by 1.0x to 1.8x multiplier
5. WHEN dangerous environments are present THEN the system SHALL increase safety_priority by additional 0.3
6. WHEN personality expression occurs THEN the system SHALL emit PersonalityExpressed event with dominant tendency

### Requirement 2: Dual-Process Decision Making

**User Story:** As a player, I want NPCs to sometimes make quick emotional decisions and sometimes think things through, so that their behavior feels naturally human with both impulsive and calculated responses.

#### Acceptance Criteria

1. WHEN cognitive clarity is high THEN the system SHALL increase deliberative decision weight
2. WHEN emotional reactivity is high THEN the system SHALL increase intuitive decision weight
3. WHEN stress level exceeds 0.7 THEN the system SHALL reduce deliberative weight by stress_value * DeliberativeReductionFactor
4. WHEN energy level is low (<0.4) THEN the system SHALL favor habitual responses over complex planning
5. WHEN decision weights are calculated THEN the system SHALL ensure Habitual + Deliberative + Emotional weights sum to 1.0
6. WHEN decision conflicts arise THEN the system SHALL emit DecisionConflict event when top options have similar scores

### Requirement 3: Emotional Override System

**User Story:** As a player, I want NPCs to sometimes act irrationally when overwhelmed by emotions, so that they feel authentically human rather than perfectly logical.

#### Acceptance Criteria

1. WHEN fear exceeds 0.8 THEN the system SHALL force flee actions regardless of other considerations
2. WHEN anger exceeds 0.7 THEN the system SHALL force aggressive actions regardless of other considerations
3. WHEN stress exceeds 0.9 THEN the system SHALL force immediate need satisfaction actions
4. WHEN high conscientiousness is present THEN the system SHALL reduce override probability by 0.3
5. WHEN high neuroticism is present THEN the system SHALL increase override probability by 0.2
6. WHEN emotional override occurs THEN the system SHALL emit EmotionalOverride event

### Requirement 4: Mood Influence on Decision Making

**User Story:** As a player, I want NPCs' current moods to affect their decision preferences, so that the same character can act differently based on their emotional state.

#### Acceptance Criteria

1. WHEN positive valence (>0.3) is present THEN the system SHALL increase social action weights by 0.2
2. WHEN negative valence (<-0.3) is present THEN the system SHALL increase safety action weights by 0.3
3. WHEN high arousal (>0.5) is present THEN the system SHALL increase immediate action weights by 0.4 and reduce planning weights by 0.3
4. WHEN low dominance (<-0.3) is present THEN the system SHALL reduce leadership action weights by 0.5
5. WHEN mood changes significantly (>0.3 absolute change) THEN the system SHALL update DecisionWeightingProfile accordingly
6. WHEN mood influences decisions THEN the system SHALL emit MoodDecisionInfluence event

### Requirement 5: Perception Bias and Categorization

**User Story:** As a player, I want NPCs to perceive the world through their own biases and mental categories, so that they react differently to the same situations based on their worldview.

#### Acceptance Criteria

1. WHEN processing sensory input THEN the system SHALL apply familiarity_bias (0.0-1.0) to known vs unknown elements
2. WHEN threat_salience is high THEN the system SHALL amplify perceived threat levels in the environment
3. WHEN goal_relevance_filter is active THEN the system SHALL filter out information not relevant to current goals
4. WHEN high neuroticism is present THEN the system SHALL multiply threat perception by 1.5x
5. WHEN low agreeableness is present THEN the system SHALL add 0.2 to hostility perception
6. WHEN novel entity types are encountered THEN the system SHALL emit NewSchemaFormed event

### Requirement 6: Social Memory and Relationship Tracking

**User Story:** As a player, I want NPCs to remember our past interactions and treat me accordingly, so that my actions have lasting consequences in the social world.

#### Acceptance Criteria

1. WHEN social interactions occur THEN the system SHALL record InteractionEvent with entity_id, action_type, and outcome
2. WHEN positive interactions occur THEN the system SHALL increase trust_level by 0.05-0.15 and familiarity by 0.02-0.08
3. WHEN negative interactions occur THEN the system SHALL decrease trust_level by 0.1-0.3
4. WHEN interactions happen during high arousal THEN the system SHALL apply 1.5x impact to emotional_attachment
5. WHEN memory decay occurs THEN the system SHALL reduce influence of interactions older than 24 hours with exponential decay
6. WHEN relationships change significantly THEN the system SHALL emit RelationshipChanged event

### Requirement 7: Learning and Adaptation System

**User Story:** As a player, I want NPCs to learn from their experiences and adapt their behavior over time, so that they feel like they're growing and changing rather than static.

#### Acceptance Criteria

1. WHEN actions succeed THEN the system SHALL increase action preference weights by 0.05-0.2
2. WHEN actions fail THEN the system SHALL decrease action preference weights by 0.1-0.3
3. WHEN high openness is present THEN the system SHALL increase learning speed by +50% weight changes
4. WHEN low openness is present THEN the system SHALL decrease learning speed by -30% weight changes
5. WHEN positive emotions during success THEN the system SHALL increase learning by 30%
6. WHEN learning occurs THEN the system SHALL emit LearningOccurred event with learned behavior

### Requirement 8: Belief System and Confirmation Bias

**User Story:** As a player, I want NPCs to form beliefs about the world and other characters that influence their future perceptions, so that they develop consistent worldviews and biases.

#### Acceptance Criteria

1. WHEN forming beliefs about entities THEN the system SHALL aggregate observations with memory-weighted averaging
2. WHEN new evidence matches existing beliefs THEN the system SHALL give it 1.5x weight in belief updates
3. WHEN new evidence contradicts existing beliefs THEN the system SHALL give it 0.7x weight in belief updates
4. WHEN beliefs have high confidence (>0.8) THEN the system SHALL resist change with 0.5x update rate
5. WHEN high openness is present THEN the system SHALL reduce confirmation bias by 30%
6. WHEN beliefs change significantly THEN the system SHALL emit BeliefChanged event

### Requirement 9: Attention and Cognitive Load Management

**User Story:** As a player, I want NPCs to have realistic limitations in what they can pay attention to and process, so that they miss things and make mistakes like real people do.

#### Acceptance Criteria

1. WHEN allocating attention THEN the system SHALL distribute 3-7 attention points based on energy level among perceived entities
2. WHEN high-priority targets exceed attention capacity THEN the system SHALL emit AttentionOverload event
3. WHEN tired (energy < 0.4) THEN the system SHALL reduce total attention capacity by 40%
4. WHEN high arousal is present THEN the system SHALL reduce total attention by 20% but increase threat focus by 50%
5. WHEN cognitive load is high THEN the system SHALL reduce accuracy of social perception and decision-making
6. WHEN attention is divided THEN the system SHALL miss subtle social cues and environmental details

### Requirement 10: Temporal Consistency in Learning and Memory

**User Story:** As a developer, I want all learning and memory systems to be synchronized to virtual world time, so that agents develop consistently regardless of simulation speed.

#### Acceptance Criteria

1. WHEN memories are stored THEN the system SHALL timestamp them with WorldTime.current_time
2. WHEN memory decay occurs THEN the system SHALL use exponential decay based on virtual hours elapsed
3. WHEN learning rates are applied THEN the system SHALL scale them proportionally to virtual time passage
4. WHEN belief formation occurs THEN the system SHALL weight recent vs old evidence based on virtual time differences
5. WHEN time jumps occur THEN the system SHALL recalculate memory strengths and learning progress appropriately
6. WHEN debugging temporal issues THEN the system SHALL validate memory timestamps against current world time

### Requirement 11: Cognitive Stability and Drift Prevention

**User Story:** As a developer, I want cognitive systems to remain stable over long periods, preventing belief extremism and personality drift that would make agents unrealistic.

#### Acceptance Criteria

1. WHEN beliefs are updated repeatedly THEN the system SHALL prevent confirmation bias from creating extreme beliefs through bounded updates
2. WHEN stress affects personality THEN the system SHALL apply temporary shifts rather than permanent personality changes
3. WHEN learning occurs over time THEN the system SHALL implement diminishing returns to prevent skill values from growing unbounded
4. WHEN memory systems interact THEN the system SHALL apply natural forgetting to prevent memory accumulation drift
5. WHEN emotional states cascade THEN the system SHALL implement decay mechanisms to prevent runaway emotional amplification
6. WHEN testing long-term stability THEN the system SHALL validate that personality traits remain within realistic ranges

### Requirement 12: Integration and Behavioral Consistency

**User Story:** As a developer, I want all cognitive systems to work together to create agents that feel like consistent characters with believable internal logic.

#### Acceptance Criteria

1. WHEN personality, mood, memory, and beliefs interact THEN the system SHALL produce coherent behavioral patterns
2. WHEN agents make decisions THEN the system SHALL ensure choices are emotionally logical even if not optimal
3. WHEN behavioral consistency is tested THEN the system SHALL maintain character feel across different situations
4. WHEN cognitive systems integrate THEN the system SHALL create emergent complexity from simple component interactions
5. WHEN performance is measured THEN the system SHALL maintain 60fps with 100+ agents running full cognitive architecture
6. WHEN debugging THEN the system SHALL provide clear visibility into decision-making processes through inspector UI