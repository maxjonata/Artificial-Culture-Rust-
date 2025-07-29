# Artificial Society - Development Roadmap

This document describes the development plan for "Artificial Society", an AI simulation system for an MMORPG, built with
the Bevy Engine in Rust. The goal is to create a dynamic and emergent world, driven by agents with complex behaviors and
social interactions. **where every agent possesses the potential for significance.**

**📅 Last Update:** July 29, 2025
**🚀 Current Status:** Phase 1.6 - Learning and Adaptation Systems

## 🎯 **Core Architectural Philosophy**

**IMPORTANT:** The project is founded on two core principles: **Neuro-inspired Cognition** and **Equality of Potential
**.

1. **Neuro-inspired Cognition:** Agent behavior is not scripted but **emerges** from the interaction of simulated
   neural, physiological, and social systems. Knowledge is not stored as data but as the **plasticity of synaptic
   connections** between concepts.
2. **Equality of Potential:** Every agent, regardless of their initial role, operates on the same fundamental cognitive
   architecture. There are no simplified "LOD brains". A farmer has the same potential for complex thought as a king;
   their differences arise from experience, not from design limitations.

This architecture's ultimate goal is to serve as a "digital twin" environment for **training reinforcement learning
models**, where the simulation provides a rich, quantifiable observation space for ML agents to learn and optimize
complex behaviors.

## 🎯 **Machine Learning Architecture Objective**

**IMPORTANT:** The main architectural objective is to create a system where agent behaviors and parameters can be
iteratively optimized via Machine Learning. The simulation should serve as a "digital twin" environment for training
reinforcement learning models. Therefore:

- 🔢 **Quantifiable Components:** All attributes must be numeric (f32, bool, Vec2) to create "observation space"
- 🎮 **Action Space Hooks:** Decision systems should facilitate future hooks for ML agent actions
- 🏆 **Reward Function Integration:** Component changes should be structured for reward calculation
- 🔄 **Modular Architecture:** Orthogonal and parallelizable systems via Bevy ECS
- 📊 **Data-Driven Design:** Components and systems should be easily extensible for new behaviors
- 🧠 **Neuro-Inspired Cogition:** Agent behavior emerges from the interaction of simulated neural, physiological, and
  social
  systems. Knowledge is not stored as data but as the plasticity of synaptic connections between concepts.
- 🧬 **Equality of Potential:** Every agent, regardless of their initial role, operates on the same fundamental cognitive
  architecture. There are no simplified "LOD brains". A farmer has the same potential for complex thought as a king;
  their differences arise from experience, not from design limitations.

## ⚠️ **Academic References Status - Critical Assessment (2025)**

### **Neuroscience and Psychophysiology:**

✅ **Highly Accepted:**

- McEwen, B.S. (2007) - Still fundamental reference for allostasis
- Doya, K. (2008) - Mathematical models of neurotransmitters remain valid
- LeDoux, J.E. (1996) - Solid foundation on emotion and brain
- Damasio, A. (1994) - Emotion-reason integration still accepted

⚠️ **Partially Contested - REPLACEMENT ARTICLES:**

- **PROBLEM:** Baumeister et al. (1998) on "ego depletion" - **CONTROVERSIAL**: Recent meta-analyses (Hagger &
  Chatzisarantis, 2016) question the replicability of ego depletion effects

**🔄 ROBUST SUBSTITUTE ARTICLES:**

- **Inzlicht, M., & Schmeichel, B.J. (2012).** "What is ego depletion? Toward a mechanistic revision of the resource
  model of self-control." *Perspectives on Psychological Science*, 7(5), 450-463. - **Critical revision that
  reformulates the concept**
- **Lurquin, J.H., & Miyake, A. (2017).** "Challenges to ego-depletion research go beyond the replication crisis: A need
  for tackling the conceptual crisis." *Frontiers in Psychology*, 8, 568. - **Analysis of conceptual limitations**
- **Dang, J. (2018).** "An updated meta-analysis of the ego depletion effect." *Psychological Research*, 82(4),
  645-651. - **More robust and recent meta-analysis**
- **Friese, M., et al. (2019).** "A multilab replication of the ego depletion effect." *Social Psychology*, 50(4),
  175-188. - **Systematic multi-laboratory replication**

**🆕 MODERN THEORETICAL ALTERNATIVES:**

- **Kool, W., & Botvinick, M. (2014).** "A labor/leisure tradeoff in cognitive control." *Journal of Experimental
  Psychology*, 143(1), 131-141. - **Economic model of cognitive control**
- **Shenhav, A., et al. (2017).** "Toward a rational and mechanistic account of mental effort." *Annual Review of
  Neuroscience*, 40, 99-124. - **Adaptive control theory based on cost-benefit**
- **Musslick, S., & Cohen, J.D. (2021).** "Rationalizing constraints on the capacity for cognitive control." *Trends in
  Cognitive Sciences*, 25(9), 757-775. - **Capacity limitations as adaptive architecture**

### **Cognitive Psychology:**

✅ **Highly Accepted:**

- Kahneman, D. (2003) - System 1/System 2 still dominant
- Festinger, L. (1957) - Cognitive dissonance remains valid
- Tversky & Kahneman (1974) - Fundamental heuristics and biases

⚠️ **Needs Update - COMPLEMENTARY ARTICLES:**

- **PROBLEM:** Thagard, P. (2000) - Coherence models have been refined by more recent Bayesian theories

**🔄 BAYESIAN COMPLEMENTARY ARTICLES:**

- **Tenenbaum, J.B., et al. (2011).** "How to grow a mind: Statistics, structure, and abstraction." *Science*, 331(
  6022), 1279-1285. - **Hierarchical Bayesian cognition**
- **Griffiths, T.L., et al. (2010).** "Probabilistic models of cognition: Exploring representations and inductive
  biases." *Trends in Cognitive Sciences*, 14(8), 357-364. - **Probabilistic models of cognition**
- **Perfors, A., et al. (2011).** "A tutorial introduction to Bayesian models of cognitive development." *Cognition*,
  120(3), 302-321. - **Bayesian cognitive development**
- **Lake, B.M., et al. (2017).** "Building machines that learn and think like people." *Behavioral and Brain Sciences*,
  40, e253. - **Human-like learning with Bayesian inference**

**🆕 HYBRID MODELS (COHERENCE + BAYES):**

- **Holyoak, K.J., & Thagard, P. (1989).** "Analogical mapping by constraint satisfaction." *Cognitive Science*, 13(3),
  295-355. - **Constraint satisfaction + coherence**
- **Eliasmith, C. (2013).** "How to build a brain: A neural architecture for biological cognition." Oxford University
  Press. - **Unified neural architecture**
- **O'Reilly, R.C., et al. (2016).** "Recurrent processing during object recognition." *Frontiers in Psychology*, 3,
    124.
        - **Recurrent processing and coherence**

### **Social Psychology:**

✅ **Validated Classics:**

- Granovetter, M. (1973) - "Weak ties" remains fundamental
- Asch, S.E. (1956) - Social conformity consistently replicated
- Tajfel & Turner (1979) - Social identity theory robust

⚠️ **Historical Context - MODERNIZATION ARTICLES:**

- **PROBLEM:** Sherif, M. (1936) - Valid but methods outdated by current ethical standards

**🔄 MODERN ETHICAL REPLICATIONS:**

- **Haslam, S.A., & Reicher, S.D. (2012).** "Contesting the 'nature' of conformity: What Milgram and Zimbardo's studies
  really show." *PLoS Biology*, 10(11), e1001426. - **Ethical reanalysis of classic experiments**
- **McDonald, M.M., et al. (2012).** "Evolution and the psychology of intergroup conflict: The male warrior hypothesis."
  *Philosophical Transactions of the Royal Society B*, 367(1589), 670-679. - **Evolutionary group dynamics**
- **Efferson, C., et al. (2008).** "Conformists and mavericks: The empirics of frequency-dependent cultural
  transmission." *Evolution and Human Behavior*, 29(1), 56-64. - **Evolutionary-based conformity**

**🆕 CONFORMITY NEUROIMAGING:**

- **Berns, G.S., et al. (2005).** "Neurobiological correlates of social conformity and independence during mental
  rotation." *Biological Psychiatry*, 58(3), 245-253. - **Neural basis of conformity**
- **Klucharev, V., et al. (2009).** "Reinforcement learning signals predict social conformity." *Neuron*, 61(1),
  140-151. - **Reinforcement learning and social conformity**

### **Complex Systems:**

✅ **Fundamental and Current:**

- Strogatz, S.H. (2014) - Contemporary reference
- Watts & Strogatz (1998) - Small-world networks still accepted
- Holland, J.H. (1995) - Complex adaptive systems validated

**🆕 RECENT UPDATES IN COMPLEX SYSTEMS:**

- **Barabási, A.L. (2016).** "Network science." Cambridge University Press. - **Updated network theory**
- **Fortunato, S., & Hric, D. (2016).** "Community detection in networks: A user guide." *Physics Reports*, 659, 1-44. -
  **Community detection in networks**
- **Holme, P., & Saramäki, J. (2012).** "Temporal networks." *Physics Reports*, 519(3), 97-125. - **Dynamic temporal
  networks**

### **Sociology:**

✅ **Robust Classics:**

- Coleman, J.S. (1988) - Social capital still relevant
- Ostrom, E. (2000) - Collective action and norms validated

**🆕 SOCIAL CAPITAL UPDATES:**

- **Lin, N. (2017).** "Building a network theory of social capital." *Social Capital*, 3-28. - **Network theory of
  social capital**
- **Burt, R.S. (2005).** "Brokerage and closure: An introduction to social capital." Oxford University Press. - **Social
  brokerage and closure**

### **📊 CRITICAL SUBSTITUTIONS SUMMARY:**

**🔴 HIGH PRIORITY (Implement Immediately):**

1. **Ego Depletion:** Replace Baumeister (1998) with adaptive control models (Shenhav et al., 2017)
2. **Coherence Models:** Complement Thagard (2000) with Bayesian inference (Tenenbaum et al., 2011)

**🟡 MEDIUM PRIORITY (Implement in Phase 2):**

1. **Social Conformity:** Add modern neuroimaging (Berns et al., 2005)
2. **Complex Networks:** Integrate temporal networks (Holme & Saramäki, 2012)

**🟢 LOW PRIORITY (Future Reference):**

1. **Social Capital:** Update with network theory (Lin, 2017)
2. **Hybrid Cognition:** Explore unified neural architectures (Eliasmith, 2013)

## Phase 1: Foundation and Prototype (✅ Complete)

The objective of this phase was to establish the technical foundation of the project, ensure the development environment
was functional, and create a minimal prototype to validate the main architecture.

-   [x] **Project Setup:** Initialize Rust project with Bevy Engine.
-   [x] **Dependency Management:** Configure and resolve compatible versions for `bevy`, `rand`, and debugging
    libraries.
-   [x] **Basic ECS Architecture:** Define the first Components (`Npc`, `Personality`, `KnowledgeBase`) and Systems.
-   [x] **Propagation Prototype:** Implement a minimal simulation system where information (a "rumor") spreads through a
    population of NPCs.
-   [x] **Debug Tools Integration:** Successfully integrate `bevy_inspector_egui` to allow real-time inspection of
    entities and components.
-   [x] **Basic Needs System:** Implement needs decay (hunger, thirst, fatigue) and social interaction via collisions.

## Phase 1.2: Base Component Refinement (✅ Complete)

**Implementation Period:** June 2025
**Objective:** Refine and stabilize the fundamental components created in Phase 1, ensuring the ECS architecture is
solid for future expansions.

### 1.2.1 Needs System Enhancement

-   [x] **`Needs` Refactoring:** Complete restructuring of the needs component
    -   [ ] Value normalization (0.0-100.0 for all needs)
    -   [ ] Differentiated decay rates by need type
    -   [ ] Threshold system for triggering behaviors

-   [ ] **Basic Resources System:** Initial implementation of `Resources` component
    -   [ ] Data structures for different resource types
    -   [ ] Ownership and resource transfer system
    -   [ ] Integration with basic needs

### 1.2.2 Component Architecture Improvement

-   [ ] **Component Standardization:** Consistent application of standard derives
    -   [ ] `#[derive(Component, Debug, Reflect, Default)]` on all components
    -   [ ] Automatic registration in `CustomComponentsPlugin`
    -   [ ] Inline documentation with scientific comments

-   [ ] **Initial Entity Builders:** Creation of first entity constructors
    -   [ ] Basic `NPCBuilder` for standardized NPC creation
    -   [ ] Controlled randomization of initial attributes
    -   [ ] Validation of mandatory components

## Phase 1.3: Knowledge and Memory System (📋 Planned)

**Implementation Period:** June 2025
**Objective:** Implement more sophisticated knowledge, memory, and information propagation systems than simple rumor
spreading.

### 1.3.1 Knowledge System Evolution

-   [ ] **`KnowledgeBase` Enhancement:** Significant expansion of knowledge system
    -   [ ] Different information types (facts, beliefs, rumors, observations)
    -   [ ] Credibility and information source system
    -   [ ] Temporal decay of unreinforced information

-   [ ] **Basic Episodic Memory:** Implementation of event memory
    -   [ ] Storage of significant social interactions
    -   [ ] Temporal association of events
    -   [ ] Memory influence on future decisions

### 1.3.2 Advanced Information Propagation

-   [ ] **Rumor System Refactoring:** Evolution of basic propagation system
    -   [ ] Information metadata (origin, timestamp, modifications)
    -   [ ] Transmission probability based on relationships
    -   [ ] Information mutation and distortion during transmission

-   [ ] **Verification System:** Basic fact-checking mechanism
    -   [ ] Comparison of conflicting information
    -   [ ] Contradiction resolution via source credibility
    -   [ ] Dynamic belief updates based on new evidence

## Phase 1.4: Movement and Basic Physics (📋 Planned)

**Implementation Period:** July 2025
**Objective:** Implement physical movement systems, collisions, and basic spatial interactions using bevy_rapier2d.

### 1.4.1 Physics Integration

-   [ ] **bevy_rapier2d Setup:** Complete physics system configuration
    -   [ ] RapierPhysicsPlugin initialization
    -   [ ] Gravity and world settings configuration
    -   [ ] Debug rendering setup for development

-   [ ] **Movement Components:** Creation of components for physical movement
    -   [ ] Integration with Rapier's `RigidBody` and `Collider`
    -   [ ] Controlled velocity and acceleration
    -   [ ] Movement limits based on NPC attributes

### 1.4.2 Collision and Interaction System

-   [ ] **Collision Detection:** System for physical interactions between NPCs
    -   [ ] Event-driven collision detection
    -   [ ] Differentiation between collision types (social, physical, resource)
    -   [ ] Triggers to initiate social interactions

-   [ ] **Goal-Directed Movement:** Basic navigation system
    -   [ ] NPCs can move toward specific points
    -   [ ] Basic collision avoidance during movement
    -   [ ] Foundation for emergent navigation behaviors

### 1.4.3 Visual and Debug Systems

-   [ ] **Basic Visual System:** Visual representation of NPCs and environment
    -   [ ] Simple sprites for NPCs with state indicators
    -   [ ] Needs visualization via colors or icons
    -   [ ] Debug overlays for development

-   [ ] **Debug Tools:** Enhancement of inspection tools
    -   [ ] Entity inspector with bevy_inspector_egui
    -   [ ] Knowledge network visualization
    -   [ ] Real-time performance metrics

## Phase 1.5: Spatial Awareness and Cognitive Mapping Foundation (📋 Planned)

**Implementation Period:** July 2025
**Objective:** Establish fundamental spatial navigation and environment management systems that will serve as the
foundation for more complex emergent behaviors.

**Key Academic References:**

- **Tolman, E. C. (1948).** "Cognitive maps in rats and men." *Psychological Review*, 55(4), 189-208. - **Foundational
  work on internal spatial representation**
- **O'Keefe, J., & Nadel, L. (1978).** "The Hippocampus as a Cognitive Map." Oxford University Press. - **Fundamental
  reference on place cells and spatial cognition**

### 1.5.1 Cognitive Mapping Components

-   [ ] **`Landmark` Component:** System for identifying and categorizing notable spatial locations
    -   [ ] Landmark types (visual, resource, social, danger)
    -   [ ] Salience weighting based on agent experiences
    -   [ ] Temporal decay of landmark significance

-   [ ] **`NeuralState` Component:** Agent's current perceptual and spatial awareness state
    -   [ ] Active place cell representation (current spatial context)
    -   [ ] Head direction encoding (orientation awareness)
    -   [ ] Grid cell activation patterns (spatial coordinate system)
    -   [ ] Boundary vector cells (environmental edge detection)

-   [ ] **`NavigationNetwork` Component:** Synaptic connection weights between spatial concepts
    -   [ ] `HashMap<Entity, HashMap<Entity, f32>>` structure for landmark-to-landmark connections
    -   [ ] Dynamic weight adjustment based on traversal frequency
    -   [ ] Decay mechanism for unused connections
    -   [ ] Integration with episodic memory for route learning

### 1.5.2 Environmental Integration Systems

-   [ ] **`Environment` Component:** Enhanced spatial representation compatible with cognitive mapping
    -   [ ] Spatial region and zone management
    -   [ ] Integration with physics systems (bevy_rapier2d)
    -   [ ] Dynamic landmark registration and updates
    -   [ ] Environmental boundary detection for navigation

-   [ ] **`EnvironmentSystem`:** Dynamic environment management with cognitive mapping support
    -   [ ] Real-time environmental data updates
    -   [ ] Spatial change detection and landmark adjustment
    -   [ ] Synchronization with physics systems
    -   [ ] Support for emergent landmark discovery

### 1.5.3 Foundation for Neural Navigation

-   [ ] **Spatial Query System:** Basic infrastructure for place cell activation
    -   [ ] Distance-based landmark activation
    -   [ ] Multi-landmark triangulation support
    -   [ ] Preparation for Hebbian learning integration

-   [ ] **Entity Builders Enhancement:** Updated constructors for cognitive mapping
    -   [ ] `NPCBuilder` integration with `NeuralState` initialization
    -   [ ] `NavigationNetwork` pre-population with basic spatial knowledge
    -   [ ] Landmark entity creation utilities

**🧠 Neurobiological Foundation Notes:**
The cognitive mapping system establishes the neural infrastructure necessary for emergent navigation behaviors. Unlike
traditional pathfinding algorithms, this approach allows agents to develop internal spatial representations through
exploration and experience, mimicking the way mammalian brains construct cognitive maps of their environment.

## 🆕 Phase 1.6: Learning and Adaptation Systems (📋 Planned)

**Implementation Period:** July 2025 (Current)
**Objective:** Implement the core neural learning mechanisms that enable emergent navigation behaviors, including
synaptic plasticity, sensory processing, and decision-making systems based on neural activation propagation.

**Key Academic References:**

- **Hebb, D. O. (1949).** "The Organization of Behavior." Wiley. - **Foundation of synaptic plasticity: "neurons that
  fire together, wire together"**
- **Bliss, T.V., & Lømo, T. (1973).** "Long‐lasting potentiation of synaptic transmission in the dentate area of the
  anaesthetized rabbit following stimulation of the perforant path." *Journal of Physiology*, 232(2), 331-356. - *
  *Experimental basis of long-term potentiation**

### 1.6.1 Synaptic Plasticity Implementation

-   [ ] **`SensoryInputSystem`:** Neural activation of place cells based on environmental input
    -   [ ] Distance-based activation functions for landmark detection
    -   [ ] Multi-modal sensory integration (visual, olfactory, auditory cues)
    -   [ ] Gaussian activation patterns around landmarks
    -   [ ] **ML-HOOK:** Sensory patterns as observation space for RL agents

-   [ ] **`SynapticPlasticitySystem`:** Hebbian learning for connection strengthening
    -   [ ] Co-activation detection between place cells
    -   [ ] Weight strengthening based on simultaneous firing
    -   [ ] Temporal decay for unused synaptic connections
    -   [ ] Spike-timing dependent plasticity (STDP) approximation

### 1.6.2 Decision Making Through Neural Propagation

-   [ ] **`DecisionMakingSystem`:** Movement decisions from neural activation propagation
    -   [ ] Activation spreading through NavigationNetwork
    -   [ ] Action potential thresholds for movement initiation
    -   [ ] Competition between alternative routes through lateral inhibition
    -   [ ] **Algorithm:** Winner-take-all networks for action selection

-   [ ] **Neural State Management:** Dynamic updating of agent neural states
    -   [ ] Place cell activation updates based on movement
    -   [ ] Head direction cell integration with movement vectors
    -   [ ] Grid cell pattern maintenance during navigation

### 1.6.3 Performance and Scalability Preparation

-   [ ] **Neural Computation Optimization:**
    -   [ ] Sparse matrix operations for NavigationNetwork
    -   [ ] Batched neural updates for multiple agents
    -   [ ] Memory-efficient activation pattern storage

-   [ ] **Multi-Agent Neural Coordination:**
    -   [ ] Stress testing with 100+ neural navigation agents
    -   [ ] Performance profiling of synaptic plasticity systems
    -   [ ] Optimization of place cell activation calculations

**🎯 Immediate Next Steps:**

1. [ ] Complete SensoryInputSystem with Gaussian place cell activation
2. [ ] Implement basic Hebbian learning in SynapticPlasticitySystem
3. [ ] Design winner-take-all decision making mechanism
4. [ ] Setup automated testing for emergent navigation behaviors

**🧠 Neural Navigation Philosophy:**
This phase represents a fundamental shift from algorithmic pathfinding to emergent navigation behaviors. Rather than
computing optimal paths through explicit algorithms, agents develop internal spatial knowledge through exploration and
synaptic plasticity, allowing for more organic and believable movement patterns that can adapt to environmental changes
without reprogramming.

## 🆕 Phase 1.7: Scalability and Simulation Management (📋 Planned)

**Implementation Period:** August 2025
**Objective:** Implement the "Waves of Reality" system for dynamic simulation scaling, allowing all NPCs to maintain
their full cognitive complexity while managing computational load through adaptive temporal resolution.

**Key Concept Reference:**

- **Simulation Level of Detail (LOD):** Industry-standard technique for managing computational complexity in large-scale
  simulations, adapted here for AI agent processing rather than visual rendering.

### 1.7.1 Waves of Reality Architecture

-   [ ] **`SimulationGrid` Resource:** Spatial partitioning system with dynamic tick rates
    -   [ ] Grid-based world subdivision for computational management
    -   [ ] `TickRate` per cell (f32: updates per second, range 0.1-60.0)
    -   [ ] Dynamic adjustment based on proximity to interest points
    -   [ ] Smooth transitions between adjacent cells to avoid discontinuities

-   [ ] **`WavePropagationSystem`:** Dynamic interest point influence calculation
    -   [ ] Distance-based tick rate decay from players and significant events
    -   [ ] Multiple simultaneous influence sources with additive effects
    -   [ ] Exponential decay functions: `tick_rate = base_rate * exp(-distance/decay_constant)`
    -   [ ] System event registration (combat, social gatherings, emergencies)

### 1.7.2 Adaptive AI Processing

-   [ ] **Temporal Batch Processing:** AI systems adapted for variable update frequencies
    -   [ ] Timer-driven system execution based on grid cell tick rates
    -   [ ] Batch processing of agents within same temporal resolution
    -   [ ] State interpolation for smooth visual representation during low-frequency updates

-   [ ] **AI System Refactoring:** Existing cognitive systems adapted for variable timing
    -   [ ] `SynapticPlasticitySystem` with accumulated time deltas
    -   [ ] `DecisionMakingSystem` with temporal scaling for neural propagation
    -   [ ] `SocialInteractionSystem` with frequency-adjusted relationship updates

### 1.7.3 Interest Point Management

-   [ ] **`InterestPoint` Component:** Dynamic points that influence simulation density
    -   [ ] Player position tracking with configurable influence radius
    -   [ ] Event-based interest points (ongoing conflicts, social gatherings)
    -   [ ] Temporal decay for transient events
    -   [ ] Hierarchical importance levels affecting influence strength

-   [ ] **Dynamic Interest Detection:** Automatic identification of emerging interest points
    -   [ ] Social clustering detection (multiple NPCs gathering)
    -   [ ] Conflict escalation monitoring
    -   [ ] Resource scarcity hotspots
    -   [ ] Emergent cultural phenomena (information cascades, norm formation)

### 1.7.4 Performance Metrics and Adaptation

-   [ ] **Computational Load Balancing:**
    -   [ ] Real-time performance monitoring per grid cell
    -   [ ] Automatic tick rate adjustment based on CPU utilization
    -   [ ] Dynamic grid subdivision for hotspot management
    -   [ ] Load distribution across multiple processing threads

-   [ ] **Quality Assurance Systems:**
    -   [ ] Behavioral continuity validation across tick rate transitions
    -   [ ] Social network integrity maintenance during variable updates
    -   [ ] Memory consistency checks for knowledge propagation

### 📊 Phase 1.7 Benefits:

**🌊 "Waves of Reality" Philosophy:**
This system ensures that every NPC retains their full cognitive potential regardless of their current computational
priority. Unlike traditional LOD systems that simplify distant objects, the Waves of Reality approach maintains the
intrinsic complexity of each agent while dynamically adjusting their temporal resolution. This creates a world where any
NPC can become the protagonist of their own emergent story, given sufficient player attention or systemic importance.

**⚖️ Computational Democracy:**
The system implements "computational democracy" where processing resources are allocated based on narrative significance
rather than arbitrary limitations. NPCs near players or involved in significant events receive high-frequency updates,
while distant agents continue their cognitive processes at lower frequencies, ready to spring into full complexity when
circumstances warrant attention.

**📊 Expected Performance Gains:**

- Support for 1000+ simultaneous NPCs with full cognitive architecture
- 60-90% reduction in AI processing overhead for distant agents
- Seamless scaling based on dynamic interest distribution
- Maintained behavioral complexity without simplified agent states

## Phase 2: Neurological Foundation - The Physiological Basis of Action

**Main Academic References:**

- McEwen, B.S. (2007). "Physiology and neurobiology of stress and adaptation" - ✅ **Fundamental and current for
  homeostasis and allostasis**
- Sterling, P. (2012). "Allostasis: a model of predictive regulation" - ✅ **Validated computational model**
- Kahneman, D. (2003). "A perspective on judgment and choice: mapping bounded rationality" - ✅ **Solid foundation for
  dual processing**

### 2.1 Neurotransmitter System

-   [ ] **`NeurotransmitterSystem` Component:**
    -   [ ] Implement dopamine levels (reward, motivation)
    -   [ ] Implement serotonin (mood, anxiety, social behavior)
    -   [ ] Implement oxytocin (social bonding, trust)
    -   [ ] Implement cortisol (stress response)
    -   [ ] Natural decay system and event-based synthesis
    -   [ ] **Reference:** Doya, K. (2008). "Modulators of decision making" - ✅ **Validated mathematical models**

### 2.2 Allostatic Load System

-   [ ] **`AllostaticLoadSystem` Component:**
    -   [ ] Stress accumulation when above baseline threshold
    -   [ ] Recovery during low-stress periods
    -   [ ] Physiological costs (immune, cardiovascular, cognitive)
    -   [ ] **Algorithm:** FitzHugh-Nagumo model for neural dynamics

### 2.3 Base Cognitive Architecture

-   [ ] **`CognitiveArchitecture` Component:**
    -   [ ] Working memory capacity
    -   [ ] Attention weights (visual, auditory, interoceptive)
    -   [ ] Cognitive load and executive control
    -   [ ] **Reference:** Aston-Jones & Cohen (2005). "Integrative theory of locus coeruleus-norepinephrine function" -
        ✅ **Accepted neurological theory**

## Phase 3: Cognitive Architecture - Construction of the "Self"

**Main Academic References:**

- Festinger, L. (1957). "A Theory of Cognitive Dissonance" - ✅ **Classic and validated foundation for belief systems**
- Tversky & Kahneman (1974). "Judgment under uncertainty: Heuristics and biases" - ✅ **Solid foundations of cognitive
  biases**
- Thagard, P. (2000). "Coherence in Thought and Action" - ⚠️ **Valid but complement with more recent Bayesian models**

### 3.1 Dual Processing System (System 1 vs System 2)

-   [ ] **`DualProcessCognition` Component:**
    -   [ ] System 1: Fast, automatic, emotional processing
    -   [ ] System 2: Slow, deliberative, rational processing
    -   [ ] ⚠️ **ATTENTION:** Willpower as limited resource (ego depletion) - **CONTROVERSIAL**
    -   [ ] **Updated Reference Suggested:** Inzlicht, M. & Schmeichel, B.J. (2012). "What is ego depletion?" - More
        current critical review
    -   [ ] **Original Reference:** Baumeister et al. (1998). "Ego depletion: Is the active self a limited resource?" -
        ⚠️ **Questioned by recent studies**

### 3.2 Belief System and Cognitive Dissonance

-   [ ] **`BeliefSystem` Component:**
    -   [ ] Belief network with confidence weights
    -   [ ] Automatic cognitive dissonance detection
    -   [ ] Resolution strategies: belief change, rationalization, denial
    -   [ ] **Algorithm:** Thagard's coherence model for dissonance resolution
    -   [ ] **Complement with:** Bayesian belief updating models (Tenenbaum et al., 2011)

### 3.3 Cognitive Bias System

-   [ ] **`CognitiveBias` Component:**
    -   [ ] Confirmation bias (selective information seeking)
    -   [ ] Availability bias (judgments based on recent memories)
    -   [ ] Anchoring effect in numerical decisions

    ✅ **Reference:** Klayman & Ha (1987). "Confirmation, disconfirmation, and information in hypothesis testing" -

## Phase 4: Social Dynamics - The Web of Relationships

**Main Academic References:**

- Granovetter, M. (1973). "The strength of weak ties" - ✅ **Fundamental classic and replicated**
- Coleman, J.S. (1988). "Social capital in the creation of human capital" - ✅ **Robust social capital theory**
- Ostrom, E. (2000). "Collective action and the evolution of social norms" - ✅ **Nobel Prize 2009, validated enforcement
  **

### 4.1 Social Trust Network

-   [ ] **`SocialTrustNetwork` Component:**
    -   [ ] Trust matrix between NPCs
    -   [ ] Trust propagation through indirect paths
    -   [ ] Temporal decay of trust
    -   [ ] **Algorithm:** Trust Propagation based on graph paths

### 4.2 Advanced Information Propagation System

-   [ ] **Rumor System Refactoring:**
    -   [ ] `InformationUnit` entity with metadata (source, credibility, veracity)
    -   [ ] Probability of accepting information based on source trust
    -   [ ] Fact-checking system through direct observation
    -   [ ] **Reference:** Easley & Kleinberg (2010). "Networks, Crowds, and Markets" - ✅ **Validated information
        cascades**

### 4.3 Group Dynamics and Faction Formation

-   [ ] **`GroupDynamics` Component:**
    -   [ ] Automatic group formation based on mutual trust
    -   [ ] Emergent group norms
    -   [ ] In-group/out-group detection
    -   [ ] **Reference:** Tajfel & Turner (1979). "An integrative theory of intergroup conflict" - ✅ **Robust and
        widely accepted theory**

## Phase 5: Cultural Dynamics and Scapegoating

**Main Academic References:**

- Girard, R. (1986). "The Scapegoat" - ✅ **Fundamental anthropological theory (though qualitative)**
- Allport, G.W. (1954). "The Nature of Prejudice" - ✅ **Classic of social psychology of prejudice**
- Axelrod, R. (1997). "The dissemination of culture" - ✅ **Validated computational model of cultural diffusion**

### 5.1 Scapegoating System

-   [ ] **`ScapegoatMechanism` Component:**
    -   [ ] Scapegoat selection algorithm based on:
        - Social distance from group
        - Lack of power/influence
        - Social visibility
        - Prior antagonism
        - Symbolic threat to group
    -   [ ] **Implementation:** Multifactorial score weighted by crisis level

### 5.2 Cultural Norms Emergence

-   [ ] **`CulturalNorms` Component:**
    -   [ ] Norm evolution through repeated interactions
    -   [ ] Social enforcement of norms (punishment/reward)
    -   [ ] Cultural transmission between generations
    -   [ ] **Reference:** Nowak, M.A. (2006). "Five rules for the evolution of cooperation" - ✅ **Validated mathematics
        of cooperation**

## Phase 6: Chaotic Hypergraph System - Intersection Point

**Main Academic References:**

- Strogatz, S.H. (2014). "Nonlinear Dynamics and Chaos" - Mathematical foundation for dynamical systems
- Watts & Strogatz (1998). "Collective dynamics of 'small-world' networks" - Complex network models

### 6.1 Hypergraph Architecture

-   [ ] **`HypergraphNode` Component:**
    -   [ ] Nodes representing states of any component
    -   [ ] Dynamic edges connecting related states
    -   [ ] Non-linear activation propagation
    -   [ ] Detection of emergent patterns and phase transitions

### 6.2 Chaotic Feedback System

-   [ ] **`ChaosEngine` System:**
    -   [ ] Feedback between neurological, cognitive and social layers
    -   [ ] Amplification of small individual differences
    -   [ ] Bifurcation points in collective behaviors
    -   [ ] **Algorithm:** Non-linear dynamical systems with strange attractors

## Phase 7: Hidden Natures and Emergent Mysteries

### 7.1 Supernatural Natures

-   [ ] **`HiddenNature` Component:**
    -   [ ] Vampires: feeding impulse vs. human mask
    -   [ ] Werewolves: bestial impulses vs. rational control
    -   [ ] Conflict with willpower system

### 7.2 Mystery Generation

-   [ ] **`MysteryGenerator` System:**
    -   [ ] Events with non-obvious causes
    -   [ ] Investigation system for curious NPCs
    -   [ ] Emergent narratives based on discoveries

## 🆕 Phase 8: Technical Evolution - Transition to Three-Dimensional Reality

**Implementation Period:** 2026 (Long-term)
**Objective:** Migrate the entire simulation architecture from 2D to 3D representation while maintaining all cognitive
and social systems, expanding the possibilities for emergent behaviors in three-dimensional space.

### 8.1 3D Rendering Migration

-   [ ] **Visual System Overhaul:** Complete transition from 2D sprites to 3D models
    -   [ ] Replace `SpriteBundle` with `PbrBundle` for physically-based rendering
    -   [ ] GLTF asset pipeline integration for Blender-created models
    -   [ ] 3D NPC model library with facial expressions and body language
    -   [ ] Environmental asset integration (buildings, terrain, vegetation)
    -   [ ] Dynamic lighting system for atmospheric realism

-   [ ] **3D User Interface Adaptation:**
    -   [ ] Camera system refactoring for 3D navigation
    -   [ ] Debug visualization adaptation to 3D space
    -   [ ] Inspector tools enhancement for 3D entity manipulation
    -   [ ] Performance profiling tools for 3D rendering pipeline

### 8.2 Physics System Migration

-   [ ] **bevy_rapier3d Integration:** Complete physics engine transition
    -   [ ] Migration from `bevy_rapier2d` to `bevy_rapier3d`
    -   [ ] 3D collider adaptation for NPC interactions
    -   [ ] Gravity system configuration for realistic 3D movement
    -   [ ] 3D collision detection for complex environmental geometry

-   [ ] **3D Spatial Queries:** Enhanced spatial reasoning for 3D environments
    -   [ ] 3D ray casting for line-of-sight calculations
    -   [ ] Volumetric spatial partitioning for efficient neighbor detection
    -   [ ] Height-aware pathfinding considerations
    -   [ ] Multi-level environment support (buildings, terrain elevation)

### 8.3 Navigation System 3D Adaptation

-   [ ] **Neural Navigation Enhancement:** Cognitive mapping system adapted for 3D space
    -   [ ] 3D place cells with volumetric activation patterns
    -   [ ] Enhanced grid cells for 3D coordinate representation
    -   [ ] Boundary vector cells adapted for 3D environmental edges
    -   [ ] **Maintained Principle:** Connectionistic navigation approach preserved in 3D

-   [ ] **3D Landmark System:** Spatial awareness enhanced for three-dimensional environments
    -   [ ] Vertical landmark detection (elevated positions, multi-story structures)
    -   [ ] 3D distance calculations with height considerations
    -   [ ] Line-of-sight occlusion effects on landmark visibility
    -   [ ] **Reference:** Jeffery, K.J. (2010). "Theoretical accounts of spatial learning: a neurobiological view."
        *Behavioural Processes*, 83(3), 303-314. - **3D spatial cognition in mammals**

### 8.4 Enhanced Environmental Complexity

-   [ ] **Multi-Level Environments:** Support for complex 3D architectural spaces
    -   [ ] Interior/exterior environment transitions
    -   [ ] Multi-story building navigation
    -   [ ] Vertical movement patterns (stairs, elevators, climbing)
    -   [ ] Dynamic environmental elements (doors, moving platforms)

-   [ ] **3D Social Dynamics:** Enhanced social behaviors in three-dimensional space
    -   [ ] Height-based social dynamics (looking up/down implications)
    -   [ ] 3D personal space and proxemics
    -   [ ] Architectural influence on social interactions
    -   [ ] Privacy and visibility considerations in 3D environments

### 📊 Phase 8 Technical Considerations:

**🏗️ Architecture Preservation:**
The transition to 3D maintains all existing cognitive, social, and neural systems without modification to their core
logic. The ECS architecture ensures that spatial representation changes do not affect the underlying behavioral
complexity, preserving years of development in agent intelligence systems.

**⚖️ Performance Scaling:**
The Waves of Reality system becomes even more crucial in 3D environments due to increased rendering and physics
overhead. The dynamic LOD system will adapt to manage both AI processing and visual rendering based on player proximity
and system performance.

**🧠 Cognitive Mapping Evolution:**
The neural navigation system's transition to 3D represents a natural evolution rather than a fundamental redesign.
Mammalian spatial cognition already operates in three dimensions, making this adaptation biologically plausible and
computationally elegant.

## Technical Implementation - ECS Architecture

The architecture follows a hierarchical structure with 5 main layers, now enhanced with considerations for neural
navigation, dynamic simulation scaling, and future 3D evolution:

### 1. Neurological Layer

**Neurotransmitters, allostatic load, willpower, neural navigation**

**Academic References:**

- McEwen, B.S. (2007). "Physiology and neurobiology of stress and adaptation" - Foundation for allostatic load
- Doya, K. (2008). "Modulators of decision making" - Mathematical models of neurotransmitters
- Inzlicht, M., & Schmeichel, B.J. (2012). "What is ego depletion? Toward a mechanistic revision of the resource model
  of self-control." - **Updated willpower theory**
- **O'Keefe, J., & Nadel, L. (1978). "The Hippocampus as a Cognitive Map."** - **Neural basis of spatial navigation**

This fundamental layer represents the biological substrate that supports all cognitive and behavioral processes of NPCs.
Here reside the neurotransmitter systems (dopamine, serotonin, oxytocin, cortisol) that modulate emotional states,
motivation and stress responses. The allostatic load system monitors the physiological cost of continuous adaptation to
the environment, accumulating "wear" when NPCs operate outside their homeostatic thresholds. The updated willpower
system follows adaptive control theory rather than simple resource depletion, making it more robust and scientifically
grounded.

**🧠 Neural Navigation Integration:** This layer now includes the foundational neural circuits for spatial navigation,
including place cells, grid cells, and boundary vector cells. These neurobiological structures provide the substrate for
emergent navigation behaviors, allowing agents to develop internal cognitive maps through exploration and synaptic
plasticity rather than relying on algorithmic pathfinding.

### 2. Cognitive Layer

**Belief system, dual processing, decision making, synaptic plasticity**

**Academic References:**

- Kahneman, D. (2003). "A perspective on judgment and choice: mapping bounded rationality" - Dual processing
- Festinger, L. (1957). "A Theory of Cognitive Dissonance" - Belief system and dissonance
- Thagard, P. (2000). "Coherence in Thought and Action" - Coherence theory for belief networks
- Tversky & Kahneman (1974). "Judgment under uncertainty: Heuristics and biases" - Cognitive biases
- **Hebb, D. O. (1949). "The Organization of Behavior."** - **Synaptic plasticity foundation**

Built upon the neurological foundation, this layer implements the higher mental processes that define the "cognitive
personality" of each NPC. The dual processing system manages the constant tension between fast and intuitive responses (
System 1) versus rational and costly deliberation (System 2), modulated by neurotransmitter states and willpower levels.
The belief system maintains a complex network of knowledge, values and expectations that can conflict, generating
cognitive dissonance that needs resolution through strategies like rationalization, belief change or denial.

**🔗 Synaptic Learning Integration:** The cognitive layer now incorporates Hebbian learning mechanisms that allow NPCs to
form and strengthen associations between concepts, places, and experiences. This creates a dynamic knowledge network
where frequently accessed information becomes more readily available, and unused connections decay over time, mimicking
biological neural plasticity.

### 3. Social-Cultural Layer

**Trust networks, information propagation, group dynamics**

**Academic References:**

- Granovetter, M. (1973). "The strength of weak ties" - Foundation for trust networks
- Coleman, J.S. (1988). "Social capital in the creation of human capital" - Social capital theory
- Tajfel & Turner (1979). "An integrative theory of intergroup conflict" - Group dynamics
- Girard, R. (1986). "The Scapegoat" - Scapegoating mechanisms
- Easley & Kleinberg (2010). "Networks, Crowds, and Markets" - Information propagation

This layer manages complex interactions between NPCs, creating a dynamic web of relationships that evolves continuously.
Trust networks establish preferential channels for information exchange, where source credibility dramatically
influences acceptance of new ideas. The information propagation system simulates how rumors, beliefs and knowledge
spread through the population, suffering mutations, amplifications and distortions based on individual cognitive
characteristics and social relations. Group dynamics emerge naturally from mutual trust aggregation, creating factions,
hierarchies and implicit social norms that pressure individuals toward conformity.

**🌊 Waves of Reality Consideration:** Social interactions are now subject to the dynamic simulation scaling system,
where the frequency of social updates depends on proximity to interest points. However, the social network maintains its
integrity across all tick rates, ensuring relationship continuity regardless of computational priority.

### 4. Hypergraph Layer

**Dynamic nodes, chaotic feedback, emergent patterns**

**Academic References:**

- Strogatz, S.H. (2014). "Nonlinear Dynamics and Chaos" - Mathematical foundation for dynamical systems
- Watts & Strogatz (1998). "Collective dynamics of 'small-world' networks" - Complex network models
- Prigogine, I. (1984). "Order out of Chaos" - Dissipative structures and self-organization
- Holland, J.H. (1995). "Hidden Order: How Adaptation Builds Complexity" - Complex adaptive systems

Representing the chaotic intersection point between all other layers, the hypergraph creates non-linear and dynamic
connections between any states of any system components. Unlike traditional networks, hypergraphs allow multiple
elements to connect simultaneously through "hyperedges", creating complex activation patterns that can propagate
cascading changes through apparently unrelated domains. This layer detects and amplifies small fluctuations,
transforming minimal differences into divergent behaviors through positive feedback loops.

**⚡ Neural Activation Propagation:** The hypergraph layer now serves as the medium for neural activation propagation in
the navigation system, allowing activation patterns to flow between place cells, landmarks, and decision-making centers
in ways that create emergent routing behaviors.

### 5. Emergent Layer

**Emergent behavior detection, phase transitions, 3D evolution readiness**

**Academic References:**

- Anderson, P.W. (1972). "More is Different" - Emergence in complex systems
- Bar-Yam, Y. (1997). "Dynamics of Complex Systems" - Phase transition analysis
- Johnson, S. (2001). "Emergence: The Connected Lives of Ants, Brains, Cities" - Emergent patterns
- Mitchell, M. (2009). "Complexity: A Guided Tour" - Complex systems and self-organization

The most abstract layer monitors and quantifies phenomena that emerge from complex interaction between all lower layers,
but cannot be predicted or explained by them in isolation. Here resides the meta-systemic intelligence that detects
collective patterns like social movement formation, sudden cultural changes, moral panics or behavioral revolutions. The
system analyzes aggregate metrics like belief distribution, social connection density, average stress levels and
cultural coherence to identify "social temperatures" that precede phase transitions.

**🏗️ 3D Evolution Monitoring:** This layer will eventually monitor the transition from 2D to 3D behaviors, detecting
when agents successfully adapt their cognitive mapping and social dynamics to three-dimensional environments, ensuring
seamless evolution of emergent behaviors across dimensional transitions.

### Data Flows:

#### **Upward Flow (Bottom-Up):**

**Neural Circuits → Spatial Cognition → Navigational Policy → Emergent Movement Patterns**

> This flow now begins with fundamental neural circuits (`Place Cells`) creating spatial awareness. These neural
> activations influence cognitive processes like attention and decision-making (via `ActivationPropagation`), which
> manifest as a **navigational policy** (always move towards the highest activation). When multiple NPCs follow their
> unique learned policies, collective phenomena like crowd formation, territorial disputes, or coordinated migrations
> emerge.


**Academic References:**

- LeDoux, J.E. (1996). "The Emotional Brain" - Emotional influence on cognition
- Damasio, A. (1994). "Descartes' Error: Emotion, Reason and the Human Brain" - Emotion-reason integration
- Cacioppo, J.T. & Berntson, G.G. (1992). "Social psychological contributions to the decade of the brain" - Biological
  basis of social behavior
- Freeman, W.J. (2000). "How Brains Make Up Their Minds" - Neural emergence of cognitive patterns

**🧭 Navigation Enhancement:** The upward flow now includes the progression from basic place cell activation to complex
route planning, allowing simple neural firing patterns to generate sophisticated spatial behaviors that influence social
dynamics and ultimately create emergent group movement patterns.

#### **Downward Flow (Top-Down):**

**Environmental Pressure → Cultural Spatial Norms → Cognitive Map Biasing → Synaptic Adaptation**

> Social pressures now directly influence the **learning process** itself. Cultural norms (e.g., "avoid the dark
> forest") can act as a modulating factor on the `SynapticPlasticitySystem`, causing connections leading into "taboo"
> areas to be learned more slowly or forgotten more quickly. This demonstrates how culture can physically shape an
> individual's neural representation of the world.


**Academic References:**

- Asch, S.E. (1956). "Studies of independence and conformity" - Social pressure and conformity
- Sherif, M. (1936). "The Psychology of Social Norms" - Norm formation and enforcement
- Sapolsky, R.M. (2004). "Why Zebras Don't Get Ulcers" - Impact of social stress on biology
- Dickerson, S.S. & Kemeny, M.E. (2004). "Acute stressors and cortisol responses" - Social stress and neurochemistry

> **🗺️ Cognitive Mapping Influence:** Social pressures now directly influence the **learning process** itself. Cultural
> norms (e.g., "avoid the dark forest") can act as a modulating factor on the `SynapticPlasticitySystem`, causing
> connections leading into "taboo" areas to be learned more slowly or forgotten more quickly. This demonstrates how
> culture can physically shape an individual's neural representation of the world.

#### **Lateral Flow (Hypergraph):**

**Any Component ↔ Hypergraph ↔ Any Component**

**Academic References:**

- Kauffman, S.A. (1993). "The Origins of Order: Self-Organization and Selection in Evolution" - Spontaneous
  self-organization
- Varela, F.J. et al. (1991). "The Embodied Mind: Cognitive Science and Human Experience" - Structural coupling
- Gell-Mann, M. (1994). "The Quark and the Jaguar: Adventures in the Simple and the Complex" - Complex adaptive systems
- Wolfram, S. (2002). "A New Kind of Science" - Computational emergence in simple systems

This flow represents the most innovative and chaotic aspect of the architecture, allowing direct and non-linear
connections between any states of any components, regardless of their hierarchical position. Unlike vertical flows that
follow predictable paths, the hypergraph creates dynamic "shortcuts" based on activation patterns, temporal correlations
or unexpected resonances. For example, a specific neurotransmitter pattern in one NPC can resonate directly with group
dynamics in another social cluster, or an individual belief can instantly activate an emergent cultural norm without
passing through intermediate stages. This flow is responsible for "quantum behavioral leaps" - sudden and apparently
inexplicable changes that characterize real complex systems. It allows small fluctuations to amplify exponentially
through non-obvious paths, creating bifurcations, phase transitions and truly emergent behaviors that cannot be
predicted by linear flows. The hypergraph acts as a "complexity accelerator", ensuring the system remains dynamic,
unpredictable and capable of spontaneous self-organization.
