# Artificial Society - Development Roadmap

This document describes the development plan for "Artificial Society", an AI simulation system for an MMORPG, built with the Bevy Engine in Rust. The goal is to create a dynamic and emergent world, driven by agents with complex behaviors and social interactions.

**📅 Last Update:** July 28, 2025
**🚀 Current Status:** Phase 1.6 - Optimization and ML Preparation

## 🎯 **Machine Learning Architecture Objective**

**IMPORTANT:** The main architectural objective is to create a system where agent behaviors and parameters can be iteratively optimized via Machine Learning. The simulation should serve as a "digital twin" environment for training reinforcement learning models. Therefore:

- 🔢 **Quantifiable Components:** All attributes must be numeric (f32, bool, Vec2) to create "observation space"
- 🎮 **Action Space Hooks:** Decision systems should facilitate future hooks for ML agent actions
- 🏆 **Reward Function Integration:** Component changes should be structured for reward calculation
- 🔄 **Modular Architecture:** Orthogonal and parallelizable systems via Bevy ECS

## ⚠️ **Academic References Status - Critical Assessment (2025)**

### **Neuroscience and Psychophysiology:**
✅ **Highly Accepted:**
- McEwen, B.S. (2007) - Still fundamental reference for allostasis
- Doya, K. (2008) - Mathematical models of neurotransmitters remain valid
- LeDoux, J.E. (1996) - Solid foundation on emotion and brain
- Damasio, A. (1994) - Emotion-reason integration still accepted

⚠️ **Partially Contested - REPLACEMENT ARTICLES:**
- **PROBLEM:** Baumeister et al. (1998) on "ego depletion" - **CONTROVERSIAL**: Recent meta-analyses (Hagger & Chatzisarantis, 2016) question the replicability of ego depletion effects

**🔄 ROBUST SUBSTITUTE ARTICLES:**
- **Inzlicht, M., & Schmeichel, B.J. (2012).** "What is ego depletion? Toward a mechanistic revision of the resource model of self-control." *Perspectives on Psychological Science*, 7(5), 450-463. - **Critical revision that reformulates the concept**
- **Lurquin, J.H., & Miyake, A. (2017).** "Challenges to ego-depletion research go beyond the replication crisis: A need for tackling the conceptual crisis." *Frontiers in Psychology*, 8, 568. - **Analysis of conceptual limitations**
- **Dang, J. (2018).** "An updated meta-analysis of the ego depletion effect." *Psychological Research*, 82(4), 645-651. - **More robust and recent meta-analysis**
- **Friese, M., et al. (2019).** "A multilab replication of the ego depletion effect." *Social Psychology*, 50(4), 175-188. - **Systematic multi-laboratory replication**

**🆕 MODERN THEORETICAL ALTERNATIVES:**
- **Kool, W., & Botvinick, M. (2014).** "A labor/leisure tradeoff in cognitive control." *Journal of Experimental Psychology*, 143(1), 131-141. - **Economic model of cognitive control**
- **Shenhav, A., et al. (2017).** "Toward a rational and mechanistic account of mental effort." *Annual Review of Neuroscience*, 40, 99-124. - **Adaptive control theory based on cost-benefit**
- **Musslick, S., & Cohen, J.D. (2021).** "Rationalizing constraints on the capacity for cognitive control." *Trends in Cognitive Sciences*, 25(9), 757-775. - **Capacity limitations as adaptive architecture**

### **Cognitive Psychology:**
✅ **Highly Accepted:**
- Kahneman, D. (2003) - System 1/System 2 still dominant
- Festinger, L. (1957) - Cognitive dissonance remains valid
- Tversky & Kahneman (1974) - Fundamental heuristics and biases

⚠️ **Needs Update - COMPLEMENTARY ARTICLES:**
- **PROBLEM:** Thagard, P. (2000) - Coherence models have been refined by more recent Bayesian theories

**🔄 BAYESIAN COMPLEMENTARY ARTICLES:**
- **Tenenbaum, J.B., et al. (2011).** "How to grow a mind: Statistics, structure, and abstraction." *Science*, 331(6022), 1279-1285. - **Hierarchical Bayesian cognition**
- **Griffiths, T.L., et al. (2010).** "Probabilistic models of cognition: Exploring representations and inductive biases." *Trends in Cognitive Sciences*, 14(8), 357-364. - **Probabilistic models of cognition**
- **Perfors, A., et al. (2011).** "A tutorial introduction to Bayesian models of cognitive development." *Cognition*, 120(3), 302-321. - **Bayesian cognitive development**
- **Lake, B.M., et al. (2017).** "Building machines that learn and think like people." *Behavioral and Brain Sciences*, 40, e253. - **Human-like learning with Bayesian inference**

**🆕 HYBRID MODELS (COHERENCE + BAYES):**
- **Holyoak, K.J., & Thagard, P. (1989).** "Analogical mapping by constraint satisfaction." *Cognitive Science*, 13(3), 295-355. - **Constraint satisfaction + coherence**
- **Eliasmith, C. (2013).** "How to build a brain: A neural architecture for biological cognition." Oxford University Press. - **Unified neural architecture**
- **O'Reilly, R.C., et al. (2016).** "Recurrent processing during object recognition." *Frontiers in Psychology*, 3, 124. - **Recurrent processing and coherence**

### **Social Psychology:**
✅ **Validated Classics:**
- Granovetter, M. (1973) - "Weak ties" remains fundamental
- Asch, S.E. (1956) - Social conformity consistently replicated
- Tajfel & Turner (1979) - Social identity theory robust

⚠️ **Historical Context - MODERNIZATION ARTICLES:**
- **PROBLEM:** Sherif, M. (1936) - Valid but methods outdated by current ethical standards

**🔄 MODERN ETHICAL REPLICATIONS:**
- **Haslam, S.A., & Reicher, S.D. (2012).** "Contesting the 'nature' of conformity: What Milgram and Zimbardo's studies really show." *PLoS Biology*, 10(11), e1001426. - **Ethical reanalysis of classic experiments**
- **McDonald, M.M., et al. (2012).** "Evolution and the psychology of intergroup conflict: The male warrior hypothesis." *Philosophical Transactions of the Royal Society B*, 367(1589), 670-679. - **Evolutionary group dynamics**
- **Efferson, C., et al. (2008).** "Conformists and mavericks: The empirics of frequency-dependent cultural transmission." *Evolution and Human Behavior*, 29(1), 56-64. - **Evolutionary-based conformity**

**🆕 CONFORMITY NEUROIMAGING:**
- **Berns, G.S., et al. (2005).** "Neurobiological correlates of social conformity and independence during mental rotation." *Biological Psychiatry*, 58(3), 245-253. - **Neural basis of conformity**
- **Klucharev, V., et al. (2009).** "Reinforcement learning signals predict social conformity." *Neuron*, 61(1), 140-151. - **Reinforcement learning and social conformity**

### **Complex Systems:**
✅ **Fundamental and Current:**
- Strogatz, S.H. (2014) - Contemporary reference
- Watts & Strogatz (1998) - Small-world networks still accepted
- Holland, J.H. (1995) - Complex adaptive systems validated

**🆕 RECENT UPDATES IN COMPLEX SYSTEMS:**
- **Barabási, A.L. (2016).** "Network science." Cambridge University Press. - **Updated network theory**
- **Fortunato, S., & Hric, D. (2016).** "Community detection in networks: A user guide." *Physics Reports*, 659, 1-44. - **Community detection in networks**
- **Holme, P., & Saramäki, J. (2012).** "Temporal networks." *Physics Reports*, 519(3), 97-125. - **Dynamic temporal networks**

### **Sociology:**
✅ **Robust Classics:**
- Coleman, J.S. (1988) - Social capital still relevant
- Ostrom, E. (2000) - Collective action and norms validated

**🆕 SOCIAL CAPITAL UPDATES:**
- **Lin, N. (2017).** "Building a network theory of social capital." *Social Capital*, 3-28. - **Network theory of social capital**
- **Burt, R.S. (2005).** "Brokerage and closure: An introduction to social capital." Oxford University Press. - **Social brokerage and closure**

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

The objective of this phase was to establish the technical foundation of the project, ensure the development environment was functional, and create a minimal prototype to validate the main architecture.

-   [x] **Project Setup:** Initialize Rust project with Bevy Engine.
-   [x] **Dependency Management:** Configure and resolve compatible versions for `bevy`, `rand`, and debugging libraries.
-   [x] **Basic ECS Architecture:** Define the first Components (`Npc`, `Personality`, `KnowledgeBase`) and Systems.
-   [x] **Propagation Prototype:** Implement a minimal simulation system where information (a "rumor") spreads through a population of NPCs.
-   [x] **Debug Tools Integration:** Successfully integrate `bevy_inspector_egui` to allow real-time inspection of entities and components.
-   [x] **Basic Needs System:** Implement needs decay (hunger, thirst, fatigue) and social interaction via collisions.

## Phase 1.2: Base Component Refinement (✅ Complete)

**Implementation Period:** June 2025
**Objective:** Refine and stabilize the fundamental components created in Phase 1, ensuring the ECS architecture is solid for future expansions.

### 1.2.1 Needs System Enhancement
-   [x] **`Needs` Refactoring:** Complete restructuring of the needs component
    -   [x] Value normalization (0.0-100.0 for all needs)
    -   [x] Differentiated decay rates by need type
    -   [x] Threshold system for triggering behaviors

-   [x] **Basic Resources System:** Initial implementation of `Resources` component
    -   [x] Data structures for different resource types
    -   [x] Ownership and resource transfer system
    -   [x] Integration with basic needs

### 1.2.2 Component Architecture Improvement
-   [x] **Component Standardization:** Consistent application of standard derives
    -   [x] `#[derive(Component, Debug, Reflect, Default)]` on all components
    -   [x] Automatic registration in `CustomComponentsPlugin`
    -   [x] Inline documentation with scientific comments

-   [x] **Initial Entity Builders:** Creation of first entity constructors
    -   [x] Basic `NPCBuilder` for standardized NPC creation
    -   [x] Controlled randomization of initial attributes
    -   [x] Validation of mandatory components

## Phase 1.3: Knowledge and Memory System (✅ Complete)

**Implementation Period:** June 2025
**Objective:** Implement more sophisticated knowledge, memory, and information propagation systems than simple rumor spreading.

### 1.3.1 Knowledge System Evolution
-   [x] **`KnowledgeBase` Enhancement:** Significant expansion of knowledge system
    -   [x] Different information types (facts, beliefs, rumors, observations)
    -   [x] Credibility and information source system
    -   [x] Temporal decay of unreinforced information

-   [x] **Basic Episodic Memory:** Implementation of event memory
    -   [x] Storage of significant social interactions
    -   [x] Temporal association of events
    -   [x] Memory influence on future decisions

### 1.3.2 Advanced Information Propagation
-   [x] **Rumor System Refactoring:** Evolution of basic propagation system
    -   [x] Information metadata (origin, timestamp, modifications)
    -   [x] Transmission probability based on relationships
    -   [x] Information mutation and distortion during transmission

-   [x] **Verification System:** Basic fact-checking mechanism
    -   [x] Comparison of conflicting information
    -   [x] Contradiction resolution via source credibility
    -   [x] Dynamic belief updates based on new evidence

## Phase 1.4: Movement and Basic Physics (✅ Complete)

**Implementation Period:** July 2025
**Objective:** Implement physical movement systems, collisions, and basic spatial interactions using bevy_rapier2d.

### 1.4.1 Physics Integration
-   [x] **bevy_rapier2d Setup:** Complete physics system configuration
    -   [x] RapierPhysicsPlugin initialization
    -   [x] Gravity and world settings configuration
    -   [x] Debug rendering setup for development

-   [x] **Movement Components:** Creation of components for physical movement
    -   [x] Integration with Rapier's `RigidBody` and `Collider`
    -   [x] Controlled velocity and acceleration
    -   [x] Movement limits based on NPC attributes

### 1.4.2 Collision and Interaction System
-   [x] **Collision Detection:** System for physical interactions between NPCs
    -   [x] Event-driven collision detection
    -   [x] Differentiation between collision types (social, physical, resource)
    -   [x] Triggers to initiate social interactions

-   [x] **Goal-Directed Movement:** Basic navigation system
    -   [x] NPCs can move toward specific points
    -   [x] Basic collision avoidance during movement
    -   [x] Preparation for more complex pathfinding

### 1.4.3 Visual and Debug Systems
-   [x] **Basic Visual System:** Visual representation of NPCs and environment
    -   [x] Simple sprites for NPCs with state indicators
    -   [x] Needs visualization via colors or icons
    -   [x] Debug overlays for development

-   [x] **Debug Tools:** Enhancement of inspection tools
    -   [x] Entity inspector with bevy_inspector_egui
    -   [x] Knowledge network visualization
    -   [x] Real-time performance metrics

## Phase 1.5: Spatial Systems and Navigation (✅ Complete)

**Implementation Period:** July 2025
**Objective:** Establish fundamental spatial navigation and environment management systems that will serve as the foundation for more complex emergent behaviors.

### 1.5.1 Environment and Spatial Navigation System
-   [x] **`Environment` Component:** System for managing spatial and environmental data
    -   [x] Data structures for world representation
    -   [x] Spatial region and zone management
    -   [x] Integration with physics systems (bevy_rapier2d)
    
-   [x] **`PathfindingComponent`:** Navigation system for agents
    -   [x] Data structures for routes and destinations
    -   [x] Integration with environment spatial representation
    -   [x] Preparation for A* algorithms and emergent navigation

-   [x] **`EnvironmentSystem`:** Dynamic environment management
    -   [x] Real-time environmental data updates
    -   [x] Spatial change detection
    -   [x] Synchronization with physics systems

-   [x] **`PathfindingSystem`:** Navigation processing
    -   [x] Route calculation for agents
    -   [x] Dynamic path updates
    -   [x] Performance optimization for multiple agents

### 1.5.2 Existing Component Improvements
-   [x] **`Needs` Refactoring:** Better structuring of needs system
    -   [x] More realistic temporal decay
    -   [x] Interactions between different needs
    -   [x] Preparation for homeostasis system

-   [x] **`NPC` Improvements:** More robust data structures
    -   [x] Integration with new spatial systems
    -   [x] Preparation for emergent behaviors
    -   [x] Better attribute organization

-   [x] **`Resources` Enhancement:** More flexible resource system
    -   [x] Dynamic resource management
    -   [x] Integration with spatial environment
    -   [x] Preparation for emergent economy

### 1.5.3 Entity Builders and Integration
-   [x] **Environment Entity Builder:** Automatic construction of environmental entities
    -   [x] Creation of environment entities with appropriate components
    -   [x] Integration with physics systems
    -   [x] Zone and region configuration

-   [x] **NPC Builder Improvements:** Enhancements to NPC constructor
    -   [x] Integration with new spatial components
    -   [x] Automatic pathfinding initialization
    -   [x] Base behavior configuration

### 📊 Phase 1.5 Status:
- **Components Implemented:** ✅ 2/2 (Environment, Pathfinding)
- **Systems Implemented:** ✅ 2/2 (EnvironmentSystem, PathfindingSystem)  
- **Entity Builders:** ✅ 2/2 (Environment, NPC Enhanced)
- **Main Integration:** ✅ Complete
- **Documentation:** ✅ APIs documented

## 🆕 Phase 1.6: Optimization and ML Preparation (🔄 In Progress)

**Implementation Period:** July 2025 (Current)
**Objective:** Optimize existing systems, implement A* algorithms, and prepare architecture for future ML integration.

### 1.6.1 Advanced Navigation Algorithms
-   [ ] **A* Pathfinding Implementation:** 
    -   [ ] Complete A* algorithm in PathfindingSystem
    -   [ ] Optimized heuristics for different terrain types
    -   [ ] Route caching to improve performance
    -   [ ] **ML-HOOK:** Pathfinding decisions as actions for RL agent

-   [ ] **Dynamic Obstacle Detection:**
    -   [ ] Detection of dynamic obstacles (other NPCs, moving objects)
    -   [ ] Automatic re-routing when paths are blocked
    -   [ ] Movement prediction to avoid collisions

### 1.6.2 Performance and Scalability
-   [ ] **Multi-Agent Stress Testing:**
    -   [ ] Tests with 100+ simultaneous agents
    -   [ ] Performance profiling and bottleneck identification
    -   [ ] Critical system optimizations

-   [ ] **Memory Optimization:**
    -   [ ] More efficient data structures
    -   [ ] Entity pool for recycling
    -   [ ] Optimized garbage collection

### 1.6.3 ML Integration Preparation
-   [ ] **Observation Space Architecture:**
    -   [ ] `MLObservationSpace` component to export quantifiable state
    -   [ ] Value normalization for neural networks
    -   [ ] Snapshot system for temporal states

-   [ ] **Action Interface Design:**
    -   [ ] Abstract interface for agent actions
    -   [ ] Mapping ML actions to Bevy systems
    -   [ ] Validation of valid actions by context

### 📊 Phase 1.6 Status:
- **A* Implementation:** 🔄 In development
- **Stress Testing:** 🔄 Setup in progress  
- **ML Preparation:** 📋 Planning
- **Performance Profiling:** ⏳ Pending

**🎯 Immediate Next Steps:**
1. [ ] Finalize A* implementation with heuristics
2. [ ] Setup automated performance tests
3. [ ] Create MLObservationSpace component
4. [ ] Document APIs for ML integration

## Phase 2: Neurological Foundation - The Physiological Basis of Action

**Main Academic References:**
- McEwen, B.S. (2007). "Physiology and neurobiology of stress and adaptation" - ✅ **Fundamental and current for homeostasis and allostasis**
- Sterling, P. (2012). "Allostasis: a model of predictive regulation" - ✅ **Validated computational model**
- Kahneman, D. (2003). "A perspective on judgment and choice: mapping bounded rationality" - ✅ **Solid foundation for dual processing**

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
    -   [ ] **Reference:** Aston-Jones & Cohen (2005). "Integrative theory of locus coeruleus-norepinephrine function" - ✅ **Accepted neurological theory**

## Phase 3: Cognitive Architecture - Construction of the "Self"

**Main Academic References:**
- Festinger, L. (1957). "A Theory of Cognitive Dissonance" - ✅ **Classic and validated foundation for belief systems**
- Tversky & Kahneman (1974). "Judgment under uncertainty: Heuristics and biases" - ✅ **Solid foundations of cognitive biases**
- Thagard, P. (2000). "Coherence in Thought and Action" - ⚠️ **Valid but complement with more recent Bayesian models**

### 3.1 Dual Processing System (System 1 vs System 2)
-   [ ] **`DualProcessCognition` Component:**
    -   [ ] System 1: Fast, automatic, emotional processing
    -   [ ] System 2: Slow, deliberative, rational processing
    -   [ ] ⚠️ **ATTENTION:** Willpower as limited resource (ego depletion) - **CONTROVERSIAL**
    -   [ ] **Updated Reference Suggested:** Inzlicht, M. & Schmeichel, B.J. (2012). "What is ego depletion?" - More current critical review
    -   [ ] **Original Reference:** Baumeister et al. (1998). "Ego depletion: Is the active self a limited resource?" - ⚠️ **Questioned by recent studies**

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
    -   [ ] **Reference:** Klayman & Ha (1987). "Confirmation, disconfirmation, and information in hypothesis testing" - ✅ **Validated methodology**

## Phase 4: Social Dynamics - The Web of Relationships

**Main Academic References:**
- Granovetter, M. (1973). "The strength of weak ties" - ✅ **Fundamental classic and replicated**
- Coleman, J.S. (1988). "Social capital in the creation of human capital" - ✅ **Robust social capital theory**
- Ostrom, E. (2000). "Collective action and the evolution of social norms" - ✅ **Nobel Prize 2009, validated enforcement**

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
    -   [ ] **Reference:** Easley & Kleinberg (2010). "Networks, Crowds, and Markets" - ✅ **Validated information cascades**

### 4.3 Group Dynamics and Faction Formation
-   [ ] **`GroupDynamics` Component:**
    -   [ ] Automatic group formation based on mutual trust
    -   [ ] Emergent group norms
    -   [ ] In-group/out-group detection
    -   [ ] **Reference:** Tajfel & Turner (1979). "An integrative theory of intergroup conflict" - ✅ **Robust and widely accepted theory**

## Phase 5: Cultural Dynamics and Scapegoating

**Main Academic References:**
- Girard, R. (1986). "The Scapegoat" - ✅ **Fundamental anthropological theory (though qualitative)**
- Allport, G.W. (1954). "The Nature of Prejudice" - ✅ **Classic of social psychology of prejudice**
- Axelrod, R. (1997). "The dissemination of culture" - ✅ **Validated computational model of cultural diffusion**

### 5.1 Scapegoating System
-   [ ] **`ScapegoatMechanism` Component:**
    -   [ ] Scapegoat selection algorithm based on:
        -   Social distance from group
        -   Lack of power/influence
        -   Social visibility
        -   Prior antagonism
        -   Symbolic threat to group
    -   [ ] **Implementation:** Multifactorial score weighted by crisis level

### 5.2 Cultural Norms Emergence
-   [ ] **`CulturalNorms` Component:**
    -   [ ] Norm evolution through repeated interactions
    -   [ ] Social enforcement of norms (punishment/reward)
    -   [ ] Cultural transmission between generations
    -   [ ] **Reference:** Nowak, M.A. (2006). "Five rules for the evolution of cooperation" - ✅ **Validated mathematics of cooperation**

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

## Technical Implementation - ECS Architecture

The architecture follows a hierarchical structure with 5 main layers:

### 1. Neurological Layer
**Neurotransmitters, allostatic load, willpower**

**Academic References:**
- McEwen, B.S. (2007). "Physiology and neurobiology of stress and adaptation" - Foundation for allostatic load
- Doya, K. (2008). "Modulators of decision making" - Mathematical models of neurotransmitters
- Baumeister, R.F. et al. (1998). "Ego depletion: Is the active self a limited resource?" - Willpower system

This fundamental layer represents the biological substrate that supports all cognitive and behavioral processes of NPCs. Here reside the neurotransmitter systems (dopamine, serotonin, oxytocin, cortisol) that modulate emotional states, motivation and stress responses. The allostatic load system monitors the physiological cost of continuous adaptation to the environment, accumulating "wear" when NPCs operate outside their homeostatic thresholds. Willpower acts as a limited resource that depletes with difficult decisions and regenerates with rest, directly influencing self-regulation capacity. This layer establishes the chemical and energetic foundations for all higher processes, creating individual variability in emotional response patterns and adaptation capacity.

### 2. Cognitive Layer
**Belief system, dual processing, decision making**

**Academic References:**
- Kahneman, D. (2003). "A perspective on judgment and choice: mapping bounded rationality" - Dual processing
- Festinger, L. (1957). "A Theory of Cognitive Dissonance" - Belief system and dissonance
- Thagard, P. (2000). "Coherence in Thought and Action" - Coherence theory for belief networks
- Tversky & Kahneman (1974). "Judgment under uncertainty: Heuristics and biases" - Cognitive biases

Built upon the neurological foundation, this layer implements the higher mental processes that define the "cognitive personality" of each NPC. The dual processing system manages the constant tension between fast and intuitive responses (System 1) versus rational and costly deliberation (System 2), modulated by neurotransmitter states and willpower levels. The belief system maintains a complex network of knowledge, values and expectations that can conflict, generating cognitive dissonance that needs resolution through strategies like rationalization, belief change or denial. Cognitive biases systematically filter and distort reality perception, creating unique "subjective realities" for each NPC and making information propagation a complex and non-linear process.

### 3. Social-Cultural Layer
**Trust networks, information propagation, group dynamics**

**Academic References:**
- Granovetter, M. (1973). "The strength of weak ties" - Foundation for trust networks
- Coleman, J.S. (1988). "Social capital in the creation of human capital" - Social capital theory
- Tajfel & Turner (1979). "An integrative theory of intergroup conflict" - Group dynamics
- Girard, R. (1986). "The Scapegoat" - Scapegoating mechanisms
- Easley & Kleinberg (2010). "Networks, Crowds, and Markets" - Information propagation

This layer manages complex interactions between NPCs, creating a dynamic web of relationships that evolves continuously. Trust networks establish preferential channels for information exchange, where source credibility dramatically influences acceptance of new ideas. The information propagation system simulates how rumors, beliefs and knowledge spread through the population, suffering mutations, amplifications and distortions based on individual cognitive characteristics and social relations. Group dynamics emerge naturally from mutual trust aggregation, creating factions, hierarchies and implicit social norms that pressure individuals toward conformity. This layer also implements scapegoating mechanisms, where groups under stress channel frustration toward specific targets based on social vulnerability criteria.

### 4. Hypergraph Layer
**Dynamic nodes, chaotic feedback, emergent patterns**

**Academic References:**
- Strogatz, S.H. (2014). "Nonlinear Dynamics and Chaos" - Mathematical foundation for dynamical systems
- Watts & Strogatz (1998). "Collective dynamics of 'small-world' networks" - Complex network models
- Prigogine, I. (1984). "Order out of Chaos" - Dissipative structures and self-organization
- Holland, J.H. (1995). "Hidden Order: How Adaptation Builds Complexity" - Complex adaptive systems

Representing the chaotic intersection point between all other layers, the hypergraph creates non-linear and dynamic connections between any states of any system components. Unlike traditional networks, hypergraphs allow multiple elements to connect simultaneously through "hyperedges", creating complex activation patterns that can propagate cascading changes through apparently unrelated domains. This layer detects and amplifies small fluctuations, transforming minimal differences into divergent behaviors through positive feedback loops. The system continuously monitors temporal patterns, identifying bifurcation points where the collective system can abruptly transition between qualitatively different states, creating the unpredictability necessary for genuine emergence.

### 5. Emergent Layer
**Emergent behavior detection, phase transitions**

**Academic References:**
- Anderson, P.W. (1972). "More is Different" - Emergence in complex systems
- Bar-Yam, Y. (1997). "Dynamics of Complex Systems" - Phase transition analysis
- Johnson, S. (2001). "Emergence: The Connected Lives of Ants, Brains, Cities" - Emergent patterns
- Mitchell, M. (2009). "Complexity: A Guided Tour" - Complex systems and self-organization

The most abstract layer monitors and quantifies phenomena that emerge from complex interaction between all lower layers, but cannot be predicted or explained by them in isolation. Here resides the meta-systemic intelligence that detects collective patterns like social movement formation, sudden cultural changes, moral panics or behavioral revolutions. The system analyzes aggregate metrics like belief distribution, social connection density, average stress levels and cultural coherence to identify "social temperatures" that precede phase transitions. This layer also implements self-observation mechanisms that allow the system to recognize when operating in stable versus chaotic regimes, adjusting global parameters to maintain the balance between order and chaos necessary for creativity and continuous adaptation.

### Data Flows:

#### **Upward Flow (Bottom-Up):** 
**Neurotransmitters → Emotions → Cognition → Social Behavior → Emergence**

**Academic References:**
- LeDoux, J.E. (1996). "The Emotional Brain" - Emotional influence on cognition
- Damasio, A. (1994). "Descartes' Error: Emotion, Reason and the Human Brain" - Emotion-reason integration
- Cacioppo, J.T. & Berntson, G.G. (1992). "Social psychological contributions to the decade of the brain" - Biological basis of social behavior
- Freeman, W.J. (2000). "How Brains Make Up Their Minds" - Neural emergence of cognitive patterns

This flow represents the influence of biological foundations on high-order phenomena, following a natural progression from micro to macro. It begins with neurotransmitter levels that directly modulate emotional states - for example, low serotonin can induce anxiety, while high dopamine increases reward-seeking. These emotional states then influence cognitive processes: an anxious NPC may exhibit greater confirmation bias or faster System 1 processing, while a motivated NPC may invest more resources in deliberative System 2. Altered cognition manifests in modified social behaviors - stressed NPCs may be less trustworthy or more prone to social avoidance behaviors. Finally, when multiple NPCs exhibit simultaneous behavioral changes due to similar neurochemical patterns, collective phenomena emerge like social panics, mass movements or cultural changes. This flow ensures that the "biological realities" of NPCs translate into observable social complexity.

#### **Downward Flow (Top-Down):** 
**Social Pressure → Cultural Norms → Cognitive Dissonance → Stress → Neurochemistry**

**Academic References:**
- Asch, S.E. (1956). "Studies of independence and conformity" - Social pressure and conformity
- Sherif, M. (1936). "The Psychology of Social Norms" - Norm formation and enforcement
- Sapolsky, R.M. (2004). "Why Zebras Don't Get Ulcers" - Impact of social stress on biology
- Dickerson, S.S. & Kemeny, M.E. (2004). "Acute stressors and cortisol responses" - Social stress and neurochemistry

This flow captures how macro-social forces penetrate and reshape individual NPC biology. It begins with external social pressures - like group expectations, emerging cultural norms or collective crises - that establish specific behavioral demands on individuals. These pressures crystallize into cultural norms that define what is "acceptable" or "expected" within certain social contexts. When cultural norms conflict with personal beliefs, values or natural inclinations of an NPC, cognitive dissonance arises - a psychological tension that demands resolution. This unresolved cognitive dissonance generates persistent psychological stress, which manifests physically through cortisol elevation and other stress markers. Chronic stress, in turn, alters neurotransmitter synthesis and decay patterns, creating new neurobiological "configurations". This flow demonstrates how culture and society literally "reshape" individual biology, creating feedback loops where social pressures become neurological realities.

#### **Lateral Flow (Hypergraph):** 
**Any Component ↔ Hypergraph ↔ Any Component**

**Academic References:**
- Kauffman, S.A. (1993). "The Origins of Order: Self-Organization and Selection in Evolution" - Spontaneous self-organization
- Varela, F.J. et al. (1991). "The Embodied Mind: Cognitive Science and Human Experience" - Structural coupling
- Gell-Mann, M. (1994). "The Quark and the Jaguar: Adventures in the Simple and the Complex" - Complex adaptive systems
- Wolfram, S. (2002). "A New Kind of Science" - Computational emergence in simple systems

This flow represents the most innovative and chaotic aspect of the architecture, allowing direct and non-linear connections between any states of any components, regardless of their hierarchical position. Unlike vertical flows that follow predictable paths, the hypergraph creates dynamic "shortcuts" based on activation patterns, temporal correlations or unexpected resonances. For example, a specific neurotransmitter pattern in one NPC can resonate directly with group dynamics in another social cluster, or an individual belief can instantly activate an emergent cultural norm without passing through intermediate stages. This flow is responsible for "quantum behavioral leaps" - sudden and apparently inexplicable changes that characterize real complex systems. It allows small fluctuations to amplify exponentially through non-obvious paths, creating bifurcations, phase transitions and truly emergent behaviors that cannot be predicted by linear flows. The hypergraph acts as a "complexity accelerator", ensuring the system remains dynamic, unpredictable and capable of spontaneous self-organization.

This architecture ensures complex emergence through interaction of simple systems, following the SOLID and DRY principles established in previous phases.
