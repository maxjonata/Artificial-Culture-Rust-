# Artificial Society Implementation Roadmap

**ADHD-Friendly Step-by-Step Development Plan**

## Overview

This roadmap breaks down the Artificial Society project into 6 clear phases, each with specific deliverables,
percentages, and implementation details. All floating-point values will be normalized to [0.0, 1.0] range using the
smallest possible types for performance.

---

## Phase 1: Foundation Layer (Weeks 1-3) - 15% Complete

**Goal**: Establish basic ECS architecture and core utility systems

### 1.1 Core Type System Setup (Week 1)

- [x] **Implement performance-optimized types** (25% of Phase 1)
    - Use `f32` for primary values (0.0-1.0 normalized)
    - Use `u16` for entity IDs and indices where possible
    - Use `u8` for quantized weights (0-255 mapped to 0.0-1.0)
    - Create type aliases in `src/core/types.rs`:
      ```rust
      pub type NormalizedFloat = f32;  // Always 0.0-1.0
      pub type AgentId = u16;
      pub type QuantizedWeight = u8;   // 0-255 → 0.0-1.0
      ```

- [x] **Complete macro utilities** (25% of Phase 1)
    - ~~✅ `clamped_setters!` macro (already implemented)~~
        - **Changed for the preferred typesafe "Normalized"**
    - ✅ `register_types!` macro (already implemented)
    - [x] **Add validation macros for 0.0-1.0 range enforcement**``
        - [x] ~~Create `validate_normalized` check util in `src/utils/macros/`~~
            - [x] **Instead it was created a normalized typesafe**
        - [x] Create `assert_range` check util for debug builds
        - [x] Add runtime validation for release builds
        - [x] ~~Implement `NormalizedValue<T>` wrapper type~~
            - [x] **Improved with safe type "Normalized"**
        - [x] Add conversion utilities between `u8` and `f32`

- [x] **Basic ECS plugin structure** (50% of Phase 1)
    - [x] **Create foundation plugin architecture**
        - [x] Create `src/ai/mod.rs` with base plugin traits
        - [x] Implement `AiPlugin` as master coordinator
        - [x] Create plugin loading order system
        - [x] Add plugin dependency management
    - [x] **Create domain plugins**
        - [x] `PhysiologyPlugin` in `src/ai/physiology/mod.rs`
        - [x] `CognitionPlugin` in `src/ai/cognition/mod.rs`
        - [x] `SocialPlugin` in `src/ai/social/mod.rs`
        - [x] `PerceptionPlugin` in `src/ai/perception/mod.rs`
        - [x] `EnvironmentPlugin` in `src/world/environment/mod.rs`
    - [x] **Plugin registration system**
        - [x] Each plugin registers its components with `app.register_types::<(...)>()`
        - [x] Implement centralized event registration
        - [ ] ~~Add system scheduling and priorities~~
        
          **Premature tasks, there's no system to schedule**
        - [x] Create plugin configuration resources

### 1.2 Development Tools Setup (Week 2)

- [x] **Debug UI with `bevy_inspector_egui`**
    - [x] Create `src/presentation/debug_ui.rs`
    - [x] Implement component inspector panels
    - [x] Add real-time value monitoring
    - [x] Create performance metrics display
    - [x] Add entity search and filtering
    - [x] Implement custom component editors

- [x] **Performance profiling setup**
    - [x] Integrate `bevy_framepace` for consistent timing
    - [x] Create `src/presentation/profiler.rs`
    - [x] Add system execution time tracking
    - [x] Implement memory usage monitoring
    - [ ] Create performance bottleneck alerts
    - [x] Add frame time graphs and metrics

- [x] **Basic logging system**
    - [x] Configure `tracing` with multiple log levels
    - [x] Create domain-specific log targets
    - [x] Implement file-based logging rotation
    - [x] Add development vs production log configs
    - [x] Create error reporting system

### 1.3 Entity Builder System (Week 3)

- [x] **Enhance generic type-safe builder**
    - [x] Extend `src/core/builders.rs` with agent-specific states
    - [x] Add validation for required components
    - [x] Implement builder trait constraints
    - [x] Create default configuration presets
    - [x] Add serialization support for builder configs

- [ ] **Create specialized builders**
    - [ ] ~~`NpcBuilder` with personality presets~~
      **with no personality presets yet, cause theres not even personality**
    - [ ] `EnvironmentBuilder` for world objects
    - [ ] `LocationBuilder` for places and zones
    - [ ] Add builder extension traits for domain-specific features

**Phase 1 Success Criteria**:

- All plugins load without errors ✓
- Type system enforces 0.0-1.0 ranges ✓
- Debug UI shows component values in real-time ✓
- Performance profiler shows stable 60fps ✓
- Entity builder creates valid agents ✓

---

## Phase 2: Physiological Foundation (Weeks 4-6) - 30% Complete

**Goal**: Implement the base biological simulation layer

### 2.1 Core Physiological Components (Week 4)

- [ ] **Needs System Implementation** (50% of Phase 2)
    - [ ] **Create `src/ai/physiology/needs.rs`**
        ```rust
        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct Needs {
            pub hunger: f32,    // 0.0 = satisfied, 1.0 = starving
            pub energy: f32,    // 0.0 = exhausted, 1.0 = fully rested  
            pub safety: f32,    // 0.0 = secure, 1.0 = terrified
            pub social: f32,    // 0.0 = fulfilled, 1.0 = lonely
        }
        ```
    - [ ] **Add needs calculation utilities**
        - [ ] Implement `calculate_overall_distress()` function
        - [ ] Add `get_most_urgent_need()` method
        - [ ] Create needs interpolation functions
    - [ ] **Create needs-related events**
        - [ ] `NeedCritical` event for emergency thresholds
        - [ ] `NeedSatisfied` event for positive changes
    - [ ] **Keep it simple and intuitive**
        - [ ] Focus on what players can observe and understand
        - [ ] Avoid complex scientific models that don't affect behavior

- [ ] **Stress System Implementation** (30% of Phase 2)
    - [ ] **Create `src/ai/physiology/stress.rs`**
        ```rust
        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct StressSystem {
            pub acute_stress: f32,    // 0.0 = calm, 1.0 = panic
            pub chronic_load: f32,    // 0.0 = fresh, 1.0 = burned out
            pub state: StressState,
        }
        ```
    - [ ] **Implement stress state transitions**
        - [ ] Create simple state machine logic for stress phases
        - [ ] Add intuitive thresholds (stressed people act differently)
        - [ ] Implement stress recovery during calm periods
    - [ ] **Focus on observable behavioral changes**
        - [ ] Stressed agents make worse decisions
        - [ ] High stress affects social interactions negatively
        - [ ] Chronic stress changes personality expression

- [ ] **Basic Energy/Fatigue System** (20% of Phase 2)
    - [ ] **Create `src/ai/physiology/energy.rs`**
        - [ ] Simple energy level that affects all other systems
        - [ ] Tired agents are less socially perceptive
        - [ ] Fatigue affects decision-making speed
        - [ ] Energy recovery through rest (no complex sleep cycles needed)

### 2.2 Physiological Systems (Week 5)

- [ ] **Needs Decay System** (40% of Phase 2)
    - [ ] **Create `needs_decay_system` in `src/ai/physiology/systems.rs`**
        - [ ] Hunger increases slowly over time (player-noticeable rate)
        - [ ] Energy decreases with activity, recovers with rest
        - [ ] Social need increases when isolated from others
        - [ ] Safety decreases in threatening environments
    - [ ] **Focus on player-observable patterns**
        - [ ] Hungry agents seek food and become irritable
        - [ ] Tired agents rest and avoid complex social situations
        - [ ] Lonely agents actively seek social interaction
        - [ ] Fearful agents avoid perceived threats

- [ ] **Stress Response System** (30% of Phase 2)
    - [ ] **Create `stress_response_system`**
        - [ ] Monitor needs for stress triggers (simple thresholds)
        - [ ] Stressed agents express tension visibly
        - [ ] High stress reduces cognitive clarity
        - [ ] Chronic stress makes agents more reactive
    - [ ] **Keep effects intuitive and relatable**
        - [ ] Stressed people are less friendly
        - [ ] They make impulsive decisions
        - [ ] They're harder to calm down
        - [ ] They avoid new social situations

- [ ] **Mood Integration System** (30% of Phase 2)
    - [ ] **Create simple mood calculations**
        - [ ] Mood affected by needs satisfaction
        - [ ] Recent social interactions influence mood
        - [ ] Mood affects how agents express themselves
        - [ ] Personality modulates baseline mood tendencies

### 2.3 Integration and Testing (Week 6)

- [ ] **System Integration**
    - [ ] Connect needs, stress, and energy systems
    - [ ] Ensure changes create observable behavioral differences
    - [ ] Test that players can intuitively understand agent states
    - [ ] Validate that personality differences are noticeable

- [ ] **Player Experience Testing**
    - [ ] Can players tell when an agent is hungry, tired, or stressed?
    - [ ] Do agent behaviors feel natural and relatable?
    - [ ] Are personality differences clear without being cartoonish?
    - [ ] Do agents react to social situations in believable ways?

- [ ] **Performance Optimization**
    - [ ] Profile system execution times
    - [ ] Optimize for 60fps with 100+ agents
    - [ ] Remove any systems that don't affect player experience

**Scientific Grounding**: Based on intuitive understanding of human needs and stress responses that players can
recognize from real life, not complex physiological models.

**Phase 2 Success Criteria**:

- Agents show relatable need-driven behaviors ✓
- Stress visibly affects social interactions ✓
- Players can intuitively read agent emotional states ✓
- Performance maintains 60fps with 100 agents ✓
- Personality differences create distinct "characters" ✓

---

## Phase 3: Cognitive Architecture (Weeks 7-10) - 50% Complete

**Goal**: Implement decision-making that feels human - flawed, biased, and relatable

### 3.1 Core Cognitive Components (Week 7)

- [ ] **Personality Traits** (25% of Phase 3)
    - [ ] **Create `src/ai/cognition/personality.rs`**
        ```rust
        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]  
        pub struct Personality {
            pub neuroticism: f32,        // 0.0 = stable, 1.0 = volatile
            pub agreeableness: f32,      // 0.0 = competitive, 1.0 = cooperative
            pub conscientiousness: f32,  // 0.0 = impulsive, 1.0 = disciplined
            pub openness: f32,          // 0.0 = conventional, 1.0 = creative
            pub extraversion: f32,      // 0.0 = introverted, 1.0 = extraverted
        }
        ```
    - [ ] **Add personality-based behavior modifiers**
        - [ ] Neurotic agents stress easier and recover slower
        - [ ] Agreeable agents avoid conflict, seek harmony
        - [ ] Conscientious agents stick to routines, plan ahead
        - [ ] Open agents try new things, get bored easily
        - [ ] Extraverted agents seek social interaction, talk more

- [ ] **Simple Mood System** (25% of Phase 3)
    - [ ] **Create `src/ai/cognition/mood.rs`**
        ```rust
        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct Mood {
            pub current_mood: f32,      // -1.0 = very negative, 1.0 = very positive
            pub mood_stability: f32,    // How quickly mood changes (personality-based)
        }
        ```
    - [ ] **Mood affects everything players can see**
        - [ ] Happy agents are more social, helpful, optimistic
        - [ ] Sad agents withdraw, less responsive, pessimistic
        - [ ] Mood colors all social interactions and expressions

- [ ] **Basic Decision Making** (25% of Phase 3)
    - [ ] **Create `src/ai/cognition/decisions.rs`**
        - [ ] Simple priority system: address most urgent need first
        - [ ] Personality affects HOW they address needs (agreeable agents ask for help)
        - [ ] Mood affects decision speed and optimism
        - [ ] Stress makes decisions more impulsive and short-sighted

- [ ] **Working Memory** (25% of Phase 3)
    - [ ] **Create `src/ai/cognition/memory.rs`**
        - [ ] Remember recent interactions (last 5-10 social events)
        - [ ] Remember recent locations and what happened there
        - [ ] Stressed agents have worse memory
        - [ ] Emotional events are remembered longer

### 3.2 Decision Systems (Week 8)

- [ ] **Goal Management** (50% of Phase 3)
    - [ ] **Create simple goal prioritization**
        - [ ] Survival goals (eat, sleep, safety) come first
        - [ ] Social goals when basic needs are met
        - [ ] Personality affects goal preferences
        - [ ] Current mood affects goal pursuit style
    - [ ] **Make goals visible through behavior**
        - [ ] Hungry agents look for food sources
        - [ ] Lonely agents approach others
        - [ ] Tired agents seek rest spots
        - [ ] Goals change based on success/failure

- [ ] **Emotional Decision Making** (50% of Phase 3)
    - [ ] **Implement "System 1" fast reactions**
        - [ ] Immediate responses to threats, opportunities
        - [ ] Based on personality and current emotional state
        - [ ] Often not optimal but feel human
    - [ ] **Simple deliberation for complex decisions**
        - [ ] Only when agent has time and energy
        - [ ] Can be interrupted by urgent needs or emotions
        - [ ] Better decisions but takes time

### 3.3 Memory and Learning (Week 9)

- [ ] **Social Memory** (50% of Phase 3)
    - [ ] **Remember people and relationships**
        - [ ] Track who was helpful/harmful
        - [ ] Remember recent conversations
        - [ ] Personality affects memory bias (neurotic = remember negatives)
        - [ ] Emotional events create stronger memories

- [ ] **Simple Learning** (50% of Phase 3)
    - [ ] **Learn from social outcomes**
        - [ ] If approach X worked, try it again
        - [ ] If someone was helpful before, trust them more
        - [ ] If a place was dangerous, avoid it
        - [ ] Personality affects learning speed and generalization

### 3.4 Integration and Testing (Week 10)

- [ ] **System Integration**
    - [ ] Connect personality, mood, decisions, and memory
    - [ ] Ensure personality creates consistent character "feel"
    - [ ] Test that players can predict agent behavior after observation

- [ ] **Player Intuition Testing**
    - [ ] Can players identify agent personalities through behavior?
    - [ ] Do agents react to social situations in relatable ways?
    - [ ] Are decisions emotionally logical even if not optimal?
    - [ ] Do agents feel like consistent characters over time?

**Scientific Foundation**: Based on Big Five personality research and Kahneman's dual-process theory, but simplified to
what players can observe and relate to.

**Phase 3 Success Criteria**:

- Agents have distinct, recognizable personalities ✓
- Decisions feel emotionally logical to players ✓
- Agents remember and learn from social interactions ✓
- Personality differences create interesting social dynamics ✓
- Players can predict behavior without it being boring ✓

---

## Phase 4: Social Communication Pipeline (Weeks 11-15) - 70% Complete

**Goal**: Implement the "Plato's Cave" system - imperfect communication that creates drama

### 4.1 Expression System (Week 11)

- [ ] **What Agents Show** (50% of Phase 4)
    - [ ] **Create `src/ai/social/expression.rs`**
        ```rust
        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct SocialExpression {
            pub apparent_mood: f32,        // What others see of their mood
            pub apparent_stress: f32,      // How stressed they appear
            pub social_openness: f32,      // How approachable they seem
            pub apparent_interest: f32,    // How interested they seem in others
        }
        ```
    - [ ] **Expression is NOT the same as internal state**
        - [ ] Introverted agents hide their true feelings more
        - [ ] Stressed agents may appear fine or very obviously stressed
        - [ ] Personality affects how much "leaks through"
        - [ ] Cultural/social context affects expression

### 4.2 Perception System (Week 12-13)

- [ ] **What Agents See** (25% of Phase 4)
    - [ ] **Create `src/ai/perception/social_perception.rs`**
        - [ ] Agents can only see other agents' expressions, not internal states
        - [ ] Perception is filtered by their own mood and personality
        - [ ] Attention limits: can't track everyone at once
        - [ ] Distance and facing direction affect perception quality

- [ ] **Belief Formation** (25% of Phase 4)
    - [ ] **Create `src/ai/social/beliefs.rs`**
        - [ ] Form beliefs about others based on observed behavior
        - [ ] Confirmation bias: see what you expect to see
        - [ ] First impressions matter and are hard to change
        - [ ] Personality affects interpretation (neurotic = assume hostility)

### 4.3 Social Interactions (Week 14-15)

- [ ] **Communication Attempts** (15% of Phase 4)
    - [ ] Simple interaction system based on goals and beliefs
    - [ ] Agents approach others based on their needs and perceptions
    - [ ] Personality affects communication style
    - [ ] Success depends on both agents' states and perceptions

- [ ] **Misunderstanding Generation** (10% of Phase 4)
    - [ ] **The core feature that creates interesting dynamics**
        - [ ] Agent A tries to communicate intent X
        - [ ] Agent B perceives it as intent Y (based on their filters)
        - [ ] Agent B responds to Y, confusing Agent A
        - [ ] Spiral of misunderstanding creates emergent drama

**Phase 4 Success Criteria**:

- Agents form different opinions about the same person ✓
- Misunderstandings create believable social conflicts ✓
- Players can see why misunderstandings happen ✓
- Social relationships change based on interactions ✓
- Personality affects social dynamics visibly ✓

---

## Phase 5: Group Dynamics (Weeks 16-18) - 85% Complete

**Goal**: Emergent social patterns that feel natural

### 5.1 Social Influence (Week 16)

- [ ] **Emotional Contagion** (50% of Phase 5)
    - [ ] **Simple emotion spreading**
        - [ ] Happy/sad agents influence others nearby
        - [ ] Stronger personalities influence more
        - [ ] Close relationships spread emotions faster
        - [ ] Group mood emerges from individual emotions

### 5.2 Group Formation (Week 17)

- [ ] **Natural Grouping** (30% of Phase 5)
    - [ ] **Groups form based on compatibility**
        - [ ] Similar personalities gravitate together
        - [ ] Shared positive experiences create bonds
        - [ ] Proximity and repeated interaction build relationships
        - [ ] Groups develop their own "culture" over time

### 5.3 Conflict and Resolution (Week 18)

- [ ] **Social Conflicts** (20% of Phase 5)
    - [ ] **Conflicts arise from misunderstandings and incompatible needs**
        - [ ] Resource competition creates tension
        - [ ] Personality clashes (high neuroticism vs low agreeableness)
        - [ ] Misunderstood intentions escalate conflicts
        - [ ] Some conflicts resolve, others persist realistically

**Phase 5 Success Criteria**:

- Groups form naturally without explicit programming ✓
- Conflicts arise from believable social dynamics ✓
- Emotional states spread through groups realistically ✓
- Players can understand group dynamics intuitively ✓

---

## Phase 6: Polish and Optimization (Weeks 19-21) - 100% Complete

**Goal**: Make it feel polished and perform well

### 6.1 Performance (Week 19)

- [ ] **Optimize for player experience** (50% of Phase 6)
    - [ ] 60 FPS with 100+ agents
    - [ ] Smooth social interactions without lag
    - [ ] Memory optimization for long-running simulations

### 6.2 Balancing (Week 20)

- [ ] **Tune for believability** (30% of Phase 6)
    - [ ] Adjust personality effect strengths
    - [ ] Balance mood change rates
    - [ ] Fine-tune perception and belief formation
    - [ ] Ensure conflicts aren't too frequent or rare

### 6.3 Player Experience (Week 21)

- [ ] **Final polish** (20% of Phase 6)
    - [ ] Debug UI for understanding agent states
    - [ ] Clear visual indicators of agent emotions/intentions
    - [ ] Documentation for interpreting agent behavior

**Phase 6 Success Criteria**:

- Consistent 60 FPS with target agent count ✓
- Players can intuitively read and predict agent behavior ✓
- Social dynamics feel natural and engaging ✓
- Agents create emergent stories players want to share ✓

---

## Technical Requirements Summary

### Core Philosophy: "Feel over Science"

- **Science as foundation**: Use research to inform parameter values and system design
- **Player intuition as goal**: If players can't understand it, it's too complex
- **Emotional logic**: Decisions should make emotional sense even if not optimal
- **Consistent personalities**: Agents should feel like recognizable characters

### Performance Targets

- **60 FPS** with 100+ agents
- **Recognizable personalities** within 2-3 interactions
- **Emergent conflicts** that feel natural, not scripted
- **Social learning** that creates persistent relationships

### Development Principles

- **Observable behavior**: Only implement what affects how agents act socially
- **Personality-driven**: Every system should be modulated by personality traits
- **Imperfect communication**: Misunderstandings create interesting dynamics
- **Emotional consistency**: Agents should feel human-like in their irrationality

This streamlined roadmap focuses on creating agents that **feel human** through their social flaws and emotional
consistency, using science as a guide for realistic parameters rather than as a strict rulebook.
