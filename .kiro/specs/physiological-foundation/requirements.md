# Requirements Document

## Introduction

The Physiological Foundation feature implements the base biological simulation layer for AI agents in the Artificial Society project. This system provides the fundamental needs-driven behaviors that create believable, human-like NPCs through physiological states including hunger, energy, safety, and social needs. The system serves as the foundation for all higher-level cognitive and social behaviors.

## Requirements

### Requirement 1: Core Needs System

**User Story:** As a player, I want NPCs to have basic physiological needs that drive their behavior, so that their actions feel motivated and relatable rather than random.

#### Acceptance Criteria

1. WHEN an agent's hunger level increases over time THEN the system SHALL update the hunger component from 0.0 (satisfied) to 1.0 (starving)
2. WHEN an agent's energy decreases through activity THEN the system SHALL reduce energy from 1.0 (fully rested) to 0.0 (exhausted)
3. WHEN an agent is in a threatening environment THEN the system SHALL increase safety need from 0.0 (secure) to 1.0 (terrified)
4. WHEN an agent is isolated from others THEN the system SHALL increase social need from 0.0 (fulfilled) to 1.0 (lonely)
5. WHEN any need exceeds 0.8 threshold THEN the system SHALL emit a NeedCritical event
6. WHEN any need drops below 0.3 threshold THEN the system SHALL emit a NeedSatisfied event

### Requirement 2: Stress Response System

**User Story:** As a player, I want NPCs to show realistic stress responses to unmet needs and threats, so that I can understand their emotional state and predict their behavior.

#### Acceptance Criteria

1. WHEN multiple needs are unmet simultaneously THEN the system SHALL calculate acute stress as the weighted sum of need values
2. WHEN an agent experiences prolonged stress THEN the system SHALL accumulate chronic load over time
3. WHEN acute stress exceeds 0.7 THEN the system SHALL transition agent to Allostasis state
4. WHEN chronic load exceeds 0.9 THEN the system SHALL transition agent to PostTraumatic state
5. WHEN stress levels change significantly THEN the system SHALL emit StressThresholdCrossed events
6. WHEN an agent is in PostTraumatic state THEN the system SHALL increase baseline reactivity to threats

### Requirement 3: Energy Management System

**User Story:** As a player, I want NPCs to manage their energy realistically, becoming less effective when tired and seeking rest when exhausted, so that their behavior patterns feel natural.

#### Acceptance Criteria

1. WHEN an agent performs high-activity actions THEN the system SHALL decrease energy by activity_level * 0.02 per minute
2. WHEN an agent is resting (activity < 0.2) THEN the system SHALL increase energy by 0.05 per minute
3. WHEN energy drops below 0.2 THEN the system SHALL emit EnergyDepleted event
4. WHEN energy recovers above 0.8 THEN the system SHALL emit EnergyRestored event
5. WHEN an agent is tired (energy < 0.4) THEN the system SHALL reduce social perception accuracy by 40%
6. WHEN high stress (arousal > 0.7) is present THEN the system SHALL increase energy drain by 1.5x

### Requirement 4: Environmental Threat Detection

**User Story:** As a player, I want NPCs to detect and respond to environmental threats based on their personality and past experiences, so that their survival behaviors feel intelligent and varied.

#### Acceptance Criteria

1. WHEN scanning environment every 2 seconds THEN the system SHALL detect threat markers within 5-unit radius
2. WHEN calculating threat perception THEN the system SHALL apply personality modifiers (high neuroticism = 1.5x threat perception)
3. WHEN an agent has negative memories of similar locations THEN the system SHALL add 0.3 to threat assessment
4. WHEN threat level exceeds threshold THEN the system SHALL emit ThreatDetected event with location and intensity
5. WHEN no immediate threats are present THEN the system SHALL gradually reduce safety need over time

### Requirement 5: Social Isolation Detection

**User Story:** As a player, I want NPCs to recognize when they're socially isolated and seek interaction accordingly, so that social dynamics emerge naturally from individual needs.

#### Acceptance Criteria

1. WHEN scanning for nearby agents every 30 seconds THEN the system SHALL count agents within 3-unit radius
2. WHEN no agents are nearby for more than 30 minutes THEN the system SHALL increase social need by 0.1 per hour
3. WHEN an agent has high extraversion THEN the system SHALL multiply isolation effects by 1.8x
4. WHEN an agent has low extraversion THEN the system SHALL reduce isolation effects by 0.6x
5. WHEN social need exceeds 0.7 THEN the system SHALL emit IsolationDetected event

### Requirement 6: Mood Integration System

**User Story:** As a player, I want NPC moods to reflect their physiological state and recent experiences, so that I can read their emotional state through their behavior and expressions.

#### Acceptance Criteria

1. WHEN calculating mood THEN the system SHALL derive valence using formula: (2.0 * (1.0 - average_need_value)) - 1.0 to map to -1.0 to 1.0 range
2. WHEN recent positive social interactions occur THEN the system SHALL add 0.1 to 0.3 to valence (clamped to 1.0)
3. WHEN recent negative events occur THEN the system SHALL subtract 0.1 to 0.4 from valence (clamped to -1.0)
4. WHEN personality has high neuroticism (>0.6) THEN the system SHALL reduce baseline valence by 0.2
5. WHEN mood changes by more than 0.3 absolute value THEN the system SHALL emit MoodChange event
6. WHEN mood affects decision-making THEN the system SHALL modify action weights based on valence polarity and magnitude

### Requirement 7: Temporal Coordination and Time Scaling

**User Story:** As a developer, I want all physiological calculations to be based on virtual world time, so that the simulation remains consistent regardless of time scaling or time jumps.

#### Acceptance Criteria

1. WHEN calculating decay rates THEN the system SHALL use WorldTime.delta_time for all time-based calculations
2. WHEN time scale changes (1x to 1000x speed) THEN the system SHALL maintain proportional behavior changes
3. WHEN time jumps occur THEN the system SHALL recalculate states based on elapsed virtual time
4. WHEN systems update THEN the system SHALL use ScheduledUpdate components with virtual timestamps
5. WHEN memory decay occurs THEN the system SHALL use exponential decay based on virtual world time elapsed
6. WHEN debugging time issues THEN the system SHALL validate that no components have future timestamps

### Requirement 8: Exponential Drift Prevention

**User Story:** As a developer, I want all physiological systems to remain stable over long simulation periods and high time scaling, so that agent behavior doesn't drift into unrealistic extremes.

#### Acceptance Criteria

1. WHEN systems run for virtual months THEN the system SHALL prevent exponential growth in any physiological values
2. WHEN time scaling is high (>100x) THEN the system SHALL maintain behavioral stability without drift
3. WHEN feedback loops exist between systems THEN the system SHALL implement natural decay and regression to baseline
4. WHEN values approach extremes THEN the system SHALL apply bounded influence with diminishing returns
5. WHEN drift is detected THEN the system SHALL emit stability warnings and apply corrective measures
6. WHEN testing long-term stability THEN the system SHALL validate that no values grow beyond expected ranges

### Requirement 9: System Performance and Integration

**User Story:** As a developer, I want the physiological systems to perform efficiently and integrate seamlessly with other AI systems, so that the simulation can support 100+ agents at 60fps.

#### Acceptance Criteria

1. WHEN running with 100+ agents THEN the system SHALL maintain 60fps performance
2. WHEN needs decay over time THEN the system SHALL update based on virtual time intervals, not frame counts
3. WHEN energy management runs THEN the system SHALL use adaptive scheduling based on agent importance
4. WHEN systems integrate THEN the system SHALL provide clear event interfaces for other AI modules
5. WHEN debugging THEN the system SHALL expose all component values through bevy_inspector_egui
6. WHEN personality differences exist THEN the system SHALL create visibly distinct behavioral patterns