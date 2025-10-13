---
inclusion: always
---

# Artificial Society Project Philosophy

## Core Design Principles

### 1. "Feel Over Science" - Selective Fidelity
- Use scientific research as inspiration and validation, not as strict implementation requirements
- Prioritize emergent behavior and player experience over scientific accuracy
- The goal is to replicate the *experience* of interacting with a mind, not to replicate the brain itself
- If players can't understand or relate to a behavior, it's too complex

### 2. "The Bugs Are Features" - Embrace Imperfection
- Miscommunication, misunderstanding, and subjective interpretation are core mechanics, not problems to solve
- Perfect communication eliminates drama and conflict - the interesting dynamics come from failures
- Agents should be fundamentally limited in understanding each other, just like humans
- Confirmation bias, emotional reactions, and flawed perception create believable social dynamics

### 3. "Plato's Cave" Architecture - Layered Reality
The entire social system is built around four layers:
- **Layer 1: Internal State** (The Truth) - Never directly accessible to other agents
- **Layer 2: Expression** (The Mask) - Imperfect translation filtered through personality
- **Layer 3: Perception** (The Distorted Eye) - Subjective observation colored by observer's state
- **Layer 4: Inference** (The Interpretation) - Pattern matching that's often wrong but consistently human-like

### 4. Continuous State Spaces
- Physiological needs use continuous values (0.0-1.0) representing absence to critical levels
- Social and emotional expressions use bipolar values (-1.0 to 1.0) representing opposing states with meaningful neutral (0.0)
- Avoid discrete enums and boolean flags for social/emotional systems
- Human social interaction is fundamentally sub-symbolic and gradient-based
- Use `Normalized<f32>` for unipolar values (needs, stress) and `f32` with validation for bipolar values (emotions, expressions)

### 5. Personality-Driven Everything
- Every system should be modulated by personality traits (Big Five model)
- Personality creates consistent character "feel" and predictable individual differences
- Same situation should produce different responses based on personality
- Personality affects not just decisions but perception, memory, and expression

## Implementation Guidelines

### Performance Requirements
- Maintain 60fps with 100+ agents running full AI architecture
- Use Data-Oriented Design principles with Bevy ECS
- Optimize for cache-friendly component layouts
- Profile regularly and remove systems that don't affect player experience

### Code Organization
- Domain-based architecture: organize by functional area, not code type
- Each domain exposes single Plugin that registers everything with Bevy
- Communicate between domains only through events
- Maintain clear boundaries and minimal coupling

### Temporal Coordination
- All time-based calculations must use virtual world time, not real-world time
- Support variable time scaling (1x to 1000x speed) and time jumps
- Use centralized WorldTime resource for all temporal calculations
- Ensure system synchronization regardless of time manipulation
- All decay rates, learning speeds, and memory retention use time deltas

### Testing and Validation
- Focus on emergent behavior validation over unit testing
- Can players intuitively understand agent behavior?
- Do agents feel like consistent characters over time?
- Are personality differences clearly observable?
- Do social conflicts arise naturally from system interactions?

### Scientific Grounding Sources
- **Dual-Process Theory**: Kahneman's "Thinking, Fast and Slow"
- **Emotion and Decision Making**: Damásio's "Descartes' Error"
- **Trauma and Plasticity**: van der Kolk's "The Body Keeps the Score"
- **Social Psychology**: Cialdini's "Influence", Brooks' "The Social Animal"
- **Evolutionary Psychology**: Dawkins' "The Selfish Gene"

## Development Priorities

### Phase 1: Foundation (Weeks 1-3) - COMPLETE
- Core type system with Normalized<f32> values
- Plugin architecture and development tools
- Entity builder system with validation

### Phase 2: Physiological Foundation (Weeks 4-6) - CURRENT
- Needs system (hunger, energy, safety, social)
- Stress response with state transitions
- Mood integration and emotional contagion

### Phase 3: Cognitive Architecture (Weeks 7-10)
- Personality-based decision making
- Dual-process cognition (System 1 vs System 2)
- Memory, learning, and belief formation

### Phase 4: Social Communication Pipeline (Weeks 11-15)
- Expression system with personality filtering
- Perception system with bias and attention limits
- Misunderstanding generation and cascade effects

### Phase 5: Group Dynamics (Weeks 16-18)
- Emotional contagion and social influence
- Group formation and cultural emergence
- Conflict and resolution patterns

## Key Success Metrics

### Player Experience
- Can players predict agent behavior after observing them?
- Do agents feel like distinct, consistent characters?
- Are social conflicts believable and emotionally logical?
- Do players occasionally wonder if they're interacting with another human?

### Technical Performance
- Stable 60fps with target agent count
- Clean separation of concerns between systems
- Debuggable and tunable AI parameters
- Emergent complexity from simple component interactions

### Social Dynamics
- Misunderstandings create persistent relationships
- Personality differences drive varied social outcomes
- Groups form naturally without explicit programming
- Cultural patterns emerge from individual interactions