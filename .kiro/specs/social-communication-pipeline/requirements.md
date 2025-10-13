# Requirements Document

## Introduction

The Social Communication Pipeline implements the core innovation of the Artificial Society project - the "Plato's Cave" system where agents communicate through imperfect expression and perception layers. This creates the misunderstandings and subjective interpretations that drive emergent social dynamics and believable NPC interactions. The system models the gap between internal state, external expression, subjective perception, and interpretation.

## Requirements

### Requirement 1: Social Expression System

**User Story:** As a player, I want NPCs to express their internal states through observable behaviors that can be misinterpreted, so that social interactions feel complex and human-like rather than perfectly transparent.

#### Acceptance Criteria

1. WHEN an agent has internal emotional state THEN the system SHALL translate it to ApparentStateVector with values ranging from -1.0 to 1.0 with personality-based filtering
2. WHEN an agent is stressed (>0.7) THEN the system SHALL either amplify expression (shift toward +1.0 tension) or suppress it (shift toward neutral 0.0) based on personality
3. WHEN an agent has high conscientiousness THEN the system SHALL suppress stress expression in public settings by shifting values 0.4 toward neutral
4. WHEN an agent has high neuroticism THEN the system SHALL amplify stress expression by increasing absolute values by up to 0.3
5. WHEN expression changes significantly (>0.3 absolute change) THEN the system SHALL emit ExpressionChangeEvent
6. WHEN internal state differs from expression THEN the system SHALL create observable gap between true feelings (-1.0 to 1.0) and apparent state (-1.0 to 1.0)

### Requirement 2: Perception and Attention System

**User Story:** As a player, I want NPCs to have limited and biased perception of others, so that they make realistic mistakes in reading social situations and create believable conflicts.

#### Acceptance Criteria

1. WHEN observing other agents THEN the system SHALL allocate attention points (3-7 based on energy) among perceived entities
2. WHEN an agent is tired (energy < 0.4) THEN the system SHALL reduce attention capacity by 40%
3. WHEN goal-relevant entities are present THEN the system SHALL give them 2x attention priority
4. WHEN threatening entities are detected THEN the system SHALL give them 3x attention priority
5. WHEN too many high-priority targets exist THEN the system SHALL emit AttentionOverload event
6. WHEN attention is limited THEN the system SHALL miss or misinterpret some social cues

### Requirement 3: Mood-Based Perception Filtering

**User Story:** As a player, I want NPCs' emotional states to color how they perceive others, so that the same behavior can be interpreted differently based on the observer's mood and personality.

#### Acceptance Criteria

1. WHEN an agent has negative valence (<-0.3) THEN the system SHALL shift perceived expressions toward negative by 0.2-0.5
2. WHEN an agent has positive valence (>0.3) THEN the system SHALL shift perceived expressions toward positive by 0.1-0.3
3. WHEN an agent has high arousal (>0.5) THEN the system SHALL amplify all perceived expression magnitudes by 1.3x
4. WHEN an agent has low dominance (<-0.3) THEN the system SHALL reduce confidence in perceptions by shifting toward neutral by 0.3
5. WHEN an agent has high agreeableness (>0.6) THEN the system SHALL bias perceived expressions toward positive by 0.2
6. WHEN emotional state changes significantly (>0.3) THEN the system SHALL update perception filters accordingly

### Requirement 4: Belief Formation and Confirmation Bias

**User Story:** As a player, I want NPCs to form persistent beliefs about others that resist change, so that first impressions matter and social relationships develop realistic complexity over time.

#### Acceptance Criteria

1. WHEN observing new behavior THEN the system SHALL form beliefs using memory-weighted averaging with recent observations
2. WHEN new evidence matches existing beliefs THEN the system SHALL give it 1.5x weight in belief updates
3. WHEN new evidence contradicts existing beliefs THEN the system SHALL give it 0.7x weight in belief updates
4. WHEN beliefs have high confidence (>0.8) THEN the system SHALL resist change with 0.5x update rate
5. WHEN personality has high openness THEN the system SHALL reduce confirmation bias by 30%
6. WHEN significant belief changes occur THEN the system SHALL emit BeliefChanged event

### Requirement 5: Intention Inference System

**User Story:** As a player, I want NPCs to guess at others' intentions based on limited information and their own biases, so that they sometimes misunderstand motivations and create interesting social dynamics.

#### Acceptance Criteria

1. WHEN observing actions THEN the system SHALL infer intentions using belief priors and bias filters
2. WHEN positive relationships exist THEN the system SHALL bias toward benevolent interpretations (+0.3 to intention assessment)
3. WHEN negative relationships exist THEN the system SHALL bias toward malevolent interpretations (-0.4 to intention assessment)
4. WHEN social context changes THEN the system SHALL interpret same actions differently
5. WHEN inference differs significantly from actual intent THEN the system SHALL emit IntentionMisinterpreted event
6. WHEN intentions are inferred THEN the system SHALL assign confidence scores based on available evidence

### Requirement 6: Social Approach and Communication Style

**User Story:** As a player, I want NPCs to decide when and how to approach others based on their needs, relationships, and personality, so that social interactions feel motivated and varied.

#### Acceptance Criteria

1. WHEN social needs are high THEN the system SHALL calculate approach motivation for each perceived entity
2. WHEN high trust relationships exist THEN the system SHALL add 0.3 to approach motivation
3. WHEN low trust relationships exist THEN the system SHALL subtract 0.5 from approach motivation
4. WHEN an agent has high extraversion THEN the system SHALL add 0.4 to social approach motivation
5. WHEN an agent has high neuroticism THEN the system SHALL subtract 0.2 from approach in unfamiliar situations
6. WHEN approach decisions are made THEN the system SHALL emit ApproachDecision event

### Requirement 7: Communication Style Adaptation

**User Story:** As a player, I want NPCs to communicate in ways that reflect their personality and emotional state, so that conversations feel authentic and character-appropriate.

#### Acceptance Criteria

1. WHEN high agreeableness is present THEN the system SHALL increase cooperative language style by 0.3
2. WHEN low agreeableness is present THEN the system SHALL increase direct/blunt language style by 0.4
3. WHEN high extraversion is present THEN the system SHALL increase expressiveness by 0.5
4. WHEN high arousal is present THEN the system SHALL increase communication intensity by 0.4
5. WHEN negative valence is present THEN the system SHALL increase cautious language by 0.3
6. WHEN communication style is adopted THEN the system SHALL emit StyleAdopted event

### Requirement 8: Interaction Outcome Calculation

**User Story:** As a player, I want social interactions between NPCs to succeed or fail based on realistic factors like personality compatibility and communication styles, so that relationships develop organically.

#### Acceptance Criteria

1. WHEN social interactions occur THEN the system SHALL calculate success probability from (trust_level + personality_compatibility) / 2
2. WHEN communication styles match THEN the system SHALL add +0.2 bonus to interaction success
3. WHEN communication styles conflict THEN the system SHALL add -0.3 penalty to interaction success
4. WHEN positive emotions are present THEN the system SHALL add 0.1-0.3 to interaction success
5. WHEN negative emotions are present THEN the system SHALL subtract 0.2-0.4 from interaction success
6. WHEN interactions complete THEN the system SHALL update relationship components based on outcomes

### Requirement 9: Misunderstanding Cascade System

**User Story:** As a player, I want misunderstandings between NPCs to spread through social networks and create persistent conflicts, so that social drama emerges naturally from communication failures.

#### Acceptance Criteria

1. WHEN misunderstandings are detected THEN the system SHALL propagate interpretations through social networks with 0.6-0.8 fidelity
2. WHEN negative emotions are present THEN the system SHALL increase misunderstanding spread rate by 1.5x
3. WHEN in-group members share interpretations THEN the system SHALL add +0.3 credibility to their versions
4. WHEN out-group members share interpretations THEN the system SHALL subtract -0.4 credibility from their versions
5. WHEN cascading misunderstandings occur THEN the system SHALL emit CascadingMisunderstanding events
6. WHEN social tensions increase THEN the system SHALL update group belief systems accordingly

### Requirement 10: Temporal Dynamics in Social Memory

**User Story:** As a developer, I want social relationships and beliefs to evolve over virtual time, so that long-term social dynamics develop naturally regardless of simulation speed.

#### Acceptance Criteria

1. WHEN social interactions are recorded THEN the system SHALL timestamp them with WorldTime.current_time
2. WHEN relationship strength decays THEN the system SHALL use virtual time-based exponential decay
3. WHEN belief confidence changes THEN the system SHALL weight recent vs old evidence based on virtual time elapsed
4. WHEN first impressions fade THEN the system SHALL reduce their influence based on virtual days/weeks passed
5. WHEN time scaling changes THEN the system SHALL maintain proportional relationship development rates
6. WHEN debugging social memory THEN the system SHALL show interaction timestamps relative to current world time

### Requirement 11: Social Stability and Cascade Control

**User Story:** As a developer, I want social dynamics to remain believable over long periods, preventing relationship extremism and social cascade runaway effects.

#### Acceptance Criteria

1. WHEN emotional contagion spreads THEN the system SHALL implement natural emotional decay to prevent runaway amplification
2. WHEN relationships evolve THEN the system SHALL apply bounded influence to prevent extreme trust/distrust values
3. WHEN misunderstandings cascade THEN the system SHALL implement information decay to prevent permanent social damage
4. WHEN social influence occurs THEN the system SHALL use diminishing returns to prevent complete personality homogenization
5. WHEN reputation spreads THEN the system SHALL implement forgetting mechanisms to allow reputation recovery over time
6. WHEN testing social stability THEN the system SHALL validate that relationships remain within believable ranges

### Requirement 12: Communication Pipeline Integration

**User Story:** As a developer, I want the complete communication pipeline to demonstrate information loss and distortion at each stage, so that the "Plato's Cave" concept creates believable social complexity.

#### Acceptance Criteria

1. WHEN communication attempts occur THEN the system SHALL execute full pipeline: internal state → expression → perception → interpretation
2. WHEN expression filtering occurs THEN the system SHALL achieve 0.6-0.9 accuracy in state translation
3. WHEN perception filtering occurs THEN the system SHALL achieve 0.5-0.8 accuracy in observation
4. WHEN interpretation occurs THEN the system SHALL achieve 0.4-0.7 accuracy in understanding
5. WHEN final understanding accuracy < 0.3 THEN the system SHALL emit MisunderstandingDetected event
6. WHEN pipeline completes THEN the system SHALL track intended vs. understood message differences