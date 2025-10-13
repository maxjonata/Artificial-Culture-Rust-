# Implementation Plan

- [ ] 1. Set up core physiological components and needs system
  - Create Needs component with hunger, energy, safety, social fields using Normalized<f32>
  - Implement need value validation and clamping (0.0-1.0 range)
  - Add need accessor methods and most urgent need calculation
  - Create NeedType enum for different need categories
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [ ] 2. Implement needs decay and satisfaction systems
  - [ ] 2.1 Create needs decay system with virtual time
    - Implement hunger increase over time using WorldTime.delta_time
    - Add energy decrease through activity with activity_level multiplier
    - Create social need increase during isolation periods
    - _Requirements: 1.1, 1.2, 1.4_
  
  - [ ] 2.2 Add need threshold event system
    - Implement NeedCritical event emission when needs exceed 0.8
    - Add NeedSatisfied event emission when needs drop below 0.3
    - Create event data structures with entity, need type, and severity
    - _Requirements: 1.5, 1.6_

- [ ] 3. Build stress response system with state transitions
  - [ ] 3.1 Create StressSystem component with three states
    - Implement acute_stress and chronic_load using Normalized<f32>
    - Add StressState enum (Homeostasis, Allostasis, PostTraumatic)
    - Create stress calculation from weighted sum of unmet needs
    - _Requirements: 2.1, 2.2_
  
  - [ ] 3.2 Implement stress state transitions
    - Add Allostasis transition when acute stress exceeds 0.7
    - Implement PostTraumatic transition when chronic load exceeds 0.9
    - Create baseline reactivity modifier for PostTraumatic state
    - _Requirements: 2.3, 2.4, 2.6_
  
  - [ ]* 3.3 Add stress threshold event system
    - Implement StressThresholdCrossed event emission
    - Create stress level change detection and reporting
    - _Requirements: 2.5_

- [ ] 4. Create energy management with activity-based drain
  - [ ] 4.1 Implement energy drain and restoration
    - Add high-activity energy decrease (activity_level * 0.02 per minute)
    - Implement rest-based energy increase (0.05 per minute when activity < 0.2)
    - Create stress-based energy drain multiplier (1.5x when arousal > 0.7)
    - _Requirements: 3.1, 3.2, 3.6_
  
  - [ ] 4.2 Add energy threshold events and effects
    - Implement EnergyDepleted event when energy drops below 0.2
    - Add EnergyRestored event when energy recovers above 0.8
    - Create tired state effects (40% social perception reduction when energy < 0.4)
    - _Requirements: 3.3, 3.4, 3.5_

- [ ] 5. Implement environmental threat detection system
  - [ ] 5.1 Create threat scanning and detection
    - Implement environmental threat marker detection within 5-unit radius
    - Add threat scanning system running every 2 seconds
    - Create threat level calculation with personality modifiers
    - _Requirements: 4.1, 4.2_
  
  - [ ] 5.2 Add personality and memory-based threat modulation
    - Implement neuroticism-based threat perception amplification (1.5x)
    - Add negative location memory influence (+0.3 to threat assessment)
    - Create gradual safety need reduction when no threats present
    - _Requirements: 4.2, 4.3, 4.5_
  
  - [ ]* 5.3 Create ThreatDetected event system
    - Implement threat detection event emission with location and intensity
    - Add threat level threshold detection and reporting
    - _Requirements: 4.4_

- [ ] 6. Build social isolation detection and response
  - [ ] 6.1 Create social proximity scanning
    - Implement nearby agent counting within 3-unit radius every 30 seconds
    - Add isolation duration tracking and social need increase
    - Create social need increase rate (0.1 per hour when isolated > 30 minutes)
    - _Requirements: 5.1, 5.2_
  
  - [ ] 6.2 Add personality-based isolation effects
    - Implement extraversion-based isolation multipliers (1.8x high, 0.6x low)
    - Create personality-driven social need sensitivity
    - _Requirements: 5.3, 5.4_
  
  - [ ]* 6.3 Add IsolationDetected event system
    - Implement isolation event emission when social need exceeds 0.7
    - Create isolation severity tracking and reporting
    - _Requirements: 5.5_

- [ ] 7. Create mood integration from physiological state
  - [ ] 7.1 Implement mood calculation from needs
    - Create valence calculation: (2.0 * (1.0 - average_need_value)) - 1.0
    - Add recent interaction influence on valence (+0.1 to +0.3 positive, -0.1 to -0.4 negative)
    - Implement valence clamping to -1.0 to 1.0 range
    - _Requirements: 6.1, 6.2, 6.3_
  
  - [ ] 7.2 Add personality-based mood modulation
    - Implement neuroticism-based baseline valence reduction (-0.2 when > 0.6)
    - Create personality-driven mood stability and reactivity
    - _Requirements: 6.4_
  
  - [ ] 7.3 Create mood change detection and decision influence
    - Implement mood change detection (>0.3 absolute value change)
    - Add action weight modification based on valence polarity and magnitude
    - _Requirements: 6.5, 6.6_
  
  - [ ]* 7.4 Add MoodChange event system
    - Implement mood change event emission with old/new values
    - Create mood influence tracking for decision systems
    - _Requirements: 6.5_

- [ ] 8. Integrate WorldTime for temporal consistency
  - [ ] 8.1 Replace all time calculations with WorldTime
    - Convert all decay rates to use WorldTime.delta_time
    - Implement virtual time-based need changes and energy management
    - Add time scaling support (1x to 1000x speed) for all physiological processes
    - _Requirements: 7.1, 7.2_
  
  - [ ] 8.2 Add time jump handling and state recalculation
    - Implement state recalculation after time jumps based on elapsed virtual time
    - Create ScheduledUpdate components with virtual timestamps
    - Add exponential decay calculations using virtual world time
    - _Requirements: 7.3, 7.4, 7.5_
  
  - [ ]* 8.3 Add temporal validation and debugging
    - Implement timestamp validation to prevent future timestamps
    - Create temporal consistency checking and debugging tools
    - _Requirements: 7.6_

- [ ] 9. Implement drift prevention and stability systems
  - [ ] 9.1 Add bounded value updates and decay mechanisms
    - Implement natural decay and regression to baseline for all values
    - Add bounded influence with diminishing returns near extremes
    - Create feedback loop stability through decay mechanisms
    - _Requirements: 8.1, 8.3, 8.4_
  
  - [ ] 9.2 Create long-term stability validation
    - Implement drift detection for values growing beyond expected ranges
    - Add stability monitoring for virtual months of simulation time
    - Create high time scaling stability validation (>100x speed)
    - _Requirements: 8.2, 8.6_
  
  - [ ]* 9.3 Add stability warning and correction systems
    - Implement drift detection alerts and corrective measures
    - Create stability warning emission when values approach extremes
    - _Requirements: 8.5_

- [ ] 10. Optimize performance and create system integration
  - [ ] 10.1 Implement adaptive scheduling for 100+ agents
    - Create importance-based update frequency for different agents
    - Add virtual time interval-based updates instead of frame-based
    - Implement efficient agent batching and parallel processing
    - _Requirements: 9.1, 9.2, 9.3_
  
  - [ ] 10.2 Create event interfaces for system integration
    - Implement clear event interfaces for cognitive and social systems
    - Add event data structures with all necessary information for other modules
    - Create personality-based behavioral pattern differentiation
    - _Requirements: 9.4, 9.6_
  
  - [ ]* 10.3 Add debugging and inspection tools
    - Implement bevy_inspector_egui integration for all components
    - Create physiological state visualization and debugging tools
    - _Requirements: 9.5_