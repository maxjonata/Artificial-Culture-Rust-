# Artificial Society - Development Roadmap (High Granularity)

This document provides a detailed, tactical plan for the development of "Artificial Society". Each sub-phase represents
a distinct functional milestone.

**📅 Last Update:** July 30, 2025
**🚀 Current Focus:** Phase 1.3 - Foundational AI Systems

---

## 🏛️ Core Architectural Philosophy

- **Mantle of Ignorance:** Agent decisions are based on internal, subjective models, not on omniscient ground truth.
- **Neuro-inspired Cognition:** Knowledge is the plasticity of connections between concepts.
- **Event-Driven & Data-Oriented:** The architecture prioritizes performance through event-driven systems and DOD.
- **Equality of Potential:** All agents share the same cognitive architecture.

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

## Phase 1: Local Simulation Foundation

**Objective:** Establish a robust, single-player simulation environment running entirely within a local Bevy
application.

### **Sub-Phase 1.1: Technical Foundation (✅ Complete)**

-   [x] **1.1.1 Project Initialization:** Initialize Rust project with Bevy Engine.
-   [x] **1.1.2 Core Dependency Configuration:** Configure and resolve compatible versions for `bevy` and `rand`.
-   [x] **1.1.3 Physics & Debugging Setup:** Integrate and configure `bevy_rapier2d` and `bevy_inspector_egui`.
-   [x] **1.1.4 Basic ECS Architecture:** Define initial `Npc`, `Personality`, `KnowledgeBase` components and a simple
    propagation system.
-   [x] **1.1.5 Debug Tool Validation:** Confirm real-time inspection of entities and components is functional.

### **Sub-Phase 1.2: Core Agent Physiology (✅ Complete)**

-   [x] **1.2.1 Needs Component Implementation:** Create `Needs` component with quantifiable values (e.g., hunger,
    energy).
-   [x] **1.2.2 Needs Decay System:** Implement a system for time-based decay of need values.
-   [x] **1.2.3 Dual-Threshold System:** Refactor `Needs` logic to use a high-threshold for action initiation and a
    low-threshold for action termination to prevent "ping-pong" behavior.
-   [x] **1.2.4 Physics-Based Interaction:** Configure `bevy_rapier2d` colliders and sensors to trigger `CollisionEvent`
    s for social interaction opportunities.
-   [x] **1.2.5 Generic Builder Implementation:** Create the generic, type-safe builder foundation in
    `generic_type_safe_builder.rs`.
-   [x] **1.2.6 NPC Builder Specialization:** Implement the specific `NpcBuilder` using the generic foundation.

### **Sub-Phase 1.3: Foundational AI Systems (🔄 In Progress)**

**Objective:** Implement the core cognitive loop of Perception -> Cognition -> Action, respecting the "Mantle of
Ignorance".

-   [ ] **1.3.1 Perception System - Vision:**
    -   [ ] Create a `vision_system` that uses spatial queries (e.g., Rapier's `query_pipeline`) to find nearby
        entities.
    -   [ ] Define an `ApparentState` component that contains only externally visible information (e.g., `is_running`,
        `is_carrying_item`).
    -   [ ] The system populates an agent's `PerceivedEntities: Vec<(Entity, ApparentState)>` component. **Crucially, it
        must not grant access to internal components of other entities.**
-   [ ] **1.3.2 Decision-Making - Utility Calculation:**
    -   [ ] Create a `decision_making_system` triggered by an `EvaluateDecision` event.
    -   [ ] Implement a `NeedsConfig` component to store priorities and thresholds for each need.
    -   [ ] The system calculates a utility score for each potential action using the formula:
        `Score = (CurrentNeedValue / HighThreshold) * Priority`.
    -   [ ] It selects the desire with the highest score and adds a `CurrentDesire` component to the agent.
-   [ ] **1.3.3 Action Management - Execution & Interruption:**
    -   [ ] Create systems that act upon `CurrentDesire` components (e.g., `movement_action_system`).
    -   [ ] Create an `action_continuation_system` that runs on agents with an active action. It checks if the relevant
        need has fallen below its `low_threshold`.
    -   [ ] If the low threshold is met, it removes the `CurrentDesire` and sends an `ActionCompleted` event, which can
        trigger a new decision evaluation.
-   [ ] **1.3.4 Cognitive Realism - Delay & Imprecision:**
    -   [ ] Implement an `ActionBuffer` component. Decisions do not take effect immediately but are pushed into this
        buffer with a timestamp.
    -   [ ] A separate system processes the buffer, executing actions only after a small, variable "cognitive delay".
    -   [ ] Implement "motor noise" by adding a small, random vector to movement targets or action executions to prevent
        perfect precision.

### **Sub-Phase 1.4: Emergent Navigation & Memory (📋 Planned)**

**Objective:** Replace simple "go-to-point" movement with a neuro-inspired cognitive mapping system.

-   [ ] **1.4.1 Cognitive Map Representation:**
    -   [ ] Create a `Landmark` component to tag significant entities (e.g., resource nodes, home locations).
    -   [ ] Create a `NavigationNetwork` component, likely a `HashMap<Entity, f32>`, to store synaptic weights from the
        agent's current location to known landmarks.
-   [ ] **1.4.2 Hebbian Learning System:**
    -   [ ] Create a `synaptic_plasticity_system` that listens for `MovedBetweenLandmarks` events.
    -   [ ] The event carries `(agent, from_landmark, to_landmark)`.
    -   [ ] The system strengthens the `from -> to` connection in the agent's `NavigationNetwork`. Implement a decay
        mechanism for unused connections.
-   [ ] **1.4.3 Navigation by Activation:**
    -   [ ] When a `Desire` requires movement, a `pathfinding_initiation_system` identifies relevant landmarks (e.g., "
        food sources").
    -   [ ] It "activates" these target nodes in the agent's `NavigationNetwork`.
-   [ ] **1.4.4 Pathfinding by Spreading Activation:**
    -   [ ] Create a `path_propagation_system` that simulates activation spreading backward from the target nodes
        through the network.
    -   [ ] The agent then chooses to move towards the adjacent landmark with the highest activation level.
    -   [ ] The final path is an emergent property of the learned synaptic weights.

## Phase 2: Social & Cultural Dynamics

**Objective:** Build upon the autonomous agents to create complex social interactions.

### **Sub-Phase 2.1: Advanced Knowledge & Beliefs (📋 Planned)**

-   [ ] **2.1.1 Knowledge Representation:** Refactor `KnowledgeBase` into a more complex structure, possibly a
    `HashMap<Entity, Belief>`, where `Belief` contains the information and a `credibility` score.
-   [ ] **2.1.2 Information Propagation:** Create a `rumor_propagation_system` where the probability of an agent
    adopting a belief is a function of the source's `trust_level` and the agent's own `Personality` (e.g., openness).
-   [ ] **2.1.3 Fact-Checking & Dissonance:** Implement a system where agents can verify information through direct
    perception. If a belief contradicts perception, it triggers a `CognitiveDissonance` event, which a separate system
    must resolve (e.g., by lowering trust in the source or discarding the belief).

### **Sub-Phase 2.2: Social Relationships (📋 Planned)**

-   [ ] **2.2.1 Social Network Component:** Create a `SocialNetwork` component to store relationships (e.g.,
    `HashMap<Entity, RelationshipData>`), where `RelationshipData` includes `trust` and `affinity`.
-   [ ] **2.2.2 Trust Modification System:** Create systems that listen for positive (`HelpedInNeed`) and negative (
    `Theft`, `Attack`) interaction events and modify the `trust` values in the `SocialNetwork`.
-   [ ] **2.2.3 Group Formation:** Implement a periodic `group_cohesion_system` that analyzes the `SocialNetwork`.
    Agents with high mutual trust can form a `Group` entity, adding each other to a `Members` component.

## Phase 3: Optimization & Scaling (Local Simulation)

**Objective:** Ensure the local simulation can handle a large number of complex agents efficiently.

### **Sub-Phase 3.1: Performance Analysis (📋 Planned)**

-   [ ] **3.1.1 Profiling Setup:** Integrate a Bevy profiling tool (e.g., `bevy_framepace`) to get detailed performance
    data.
-   [ ] **3.1.2 Bottleneck Identification:** Run stress tests with 500+ agents to identify the most computationally
    expensive systems.

### **Sub-Phase 3.2: Simulation LOD Implementation (📋 Planned)**

-   [ ] **3.2.1 Grid & Tick Rate:** Implement the `SimulationGrid` resource and the logic to assign a variable
    `TickRate` to each cell.
-   [ ] **3.2.2 Interest Points:** Create an `InterestPoint` component (initially for the player/camera) that influences
    the `TickRate` of nearby grid cells.
-   [ ] **3.2.3 System Refactoring for Variable Time:** Modify core AI systems (`needs_decay`, `synaptic_plasticity`) to
    correctly handle variable time deltas, ensuring simulation integrity at lower tick rates.

## Phase 4: Network Integration & Multiplayer

**Objective:** Transition the local simulation into a client-server architecture using Space and Time DB.

### **Sub-Phase 4.1: Initial SDB Setup (📋 Planned)**

-   [ ] **4.1.1 SDB Schema Definition:** Define the core SDB tables for persistent state (e.g., `CharacterSheet`,
    `WorldMap`, `Inventory`).
-   [ ] **4.1.2 Core Reducers:** Implement the most basic SDB reducers for account creation and login.

### **Sub-Phase 4.2: Architecture Bifurcation (📋 Planned)**

-   [ ] **4.2.1 Headless Worker Creation:** Refactor the existing Bevy application into a "headless" worker that can run
    on a server.
-   [ ] **4.2.2 Client Creation:** Create a new Bevy application for the player client, stripping out all server-side
    logic (AI decision-making, authoritative physics).
-   [ ] **4.2.3 `bevy_spacetimedb` Integration:** Add the `bevy_spacetimedb` crate to the worker and establish a basic
    connection to the SDB instance.

### **Sub-Phase 4.3: State Synchronization (📋 Planned)**

-   [ ] **4.3.1 Worker State Loading:** Implement the logic for the worker to load its zone's state from SDB tables on
    startup.
-   [ ] **4.3.2 SDB-to-Bevy Replication:** Use `bevy_spacetimedb` to automatically sync SDB table updates to Bevy
    components in the worker.
- [- ] **4.3.3 Bevy-to-SDB Synchronization:** Implement systems in the worker that send transactions to SDB reducers to
  update the authoritative state (e.g., after an NPC's health changes).

### **Sub-Phase 4.4: Player Integration (📋 Planned)**

-   [ ] **4.4.1 Player Control:** Implement input handling on the client that sends transactions to SDB reducers (e.g.,
    `MovePlayerReducer`).
-   [ ] **4.4.2 Client-Side Prediction:** Implement movement prediction for the player character on the client to mask
    network latency.
-   [ ] **4.4.3 Server Reconciliation:** Implement the logic for the client to correct its predicted state based on the
    authoritative state it receives from the SDB.

## Phase 5: Advanced Systems & Long-Term Goals

**Objective:** Implement the final layers of cultural dynamics and prepare for future evolution.

### **Sub-Phase 5.1: Cultural Dynamics (📋 Planned)**

-   [ ] **5.1.1 Norm Emergence:** Implement systems where repeated behaviors within a group can crystallize into a
    `CulturalNorm` component.
-   [ ] **5.1.2 Social Enforcement:** Create systems where agents can "punish" or "reward" others for deviating from or
    adhering to established norms, modifying `trust`.
-   [ ] **5.1.3 Scapegoating Mechanism:** During high-stress events, implement a system that identifies a vulnerable
    out-group member and focuses negative interactions on them.

### **Sub-Phase 5.2: Future Evolution (📋 Planned)**

-   [ ] **5.2.1 3D Migration Path:** Create a technical design document outlining the steps to migrate from 2D to 3D,
    including changes to physics, rendering, and the navigation system.
-   [ ] **5.2.2 Machine Learning Hooks:** With the networked architecture stable, design a clean API for an external ML
    training process to observe state from SDB and send actions via reducers.
-   [ ] **5.2.3 Emergent Mysteries:** Implement a system that can generate `InformationUnit`s with no obvious cause,
    prompting investigation behaviors from curious NPCs.

## Phase 6: Advanced Social & Political Dynamics

**Objective:** Simulate complex societal structures, including power dynamics, economics, and political maneuvering.

### **Sub-Phase 6.1: Economic Simulation (📋 Planned)**

-   [ ] **6.1.1 Resource Scarcity & Value:** Implement a system where the value of in-game resources fluctuates based on
    supply and demand.
-   [ ] **6.1.2 NPC Professions & Production:** Assign professions to NPCs (e.g., farmer, blacksmith). Create systems
    for them to produce specific resources and consume others.
-   [ ] **6.1.3 Barter & Trade System:** Implement a `trading_system` where NPCs can initiate trade proposals with each
    other to satisfy their needs based on perceived resource value.
-   [ ] **6.1.4 Emergent Markets:** Allow for the creation of central "market" locations where NPCs gather to trade,
    creating hotspots of economic activity.

### **Sub-Phase 6.2: Power & Influence (📋 Planned)**

-   [ ] **6.2.1 Influence Component:** Create an `Influence` component that quantifies an agent's social, economic, and
    political power.
-   [ ] **6.2.2 Power Dynamics System:** Influence is gained through wealth accumulation, leadership of a large group,
    or control of critical resources.
-   [ ] **6.2.3 Social Hierarchy Emergence:** Implement systems where agents with high influence are more likely to have
    their trade proposals accepted, their beliefs adopted, and can enforce norms more effectively.

## Phase 7: The Supernatural & Hidden World

**Objective:** Weave the game's unique supernatural elements into the core simulation, creating internal conflicts for a
subset of agents.

### **Sub-Phase 7.1: Hidden Natures (📋 Planned)**

-   [ ] **7.1.1 `HiddenNature` Component:** Create a component (e.g., `Vampire`, `Werewolf`) that is **not** externally
    visible.
-   [ ] **7.1.2 Supernatural Needs:** Add new, powerful needs associated with these natures (e.g., `Bloodthirst` for
    vampires).
-   [ ] **7.1.3 Internal Conflict System:** Create a system where these new needs generate extremely high-priority
    `Desire`s that may conflict with an agent's established social norms and relationships.
-   [ ] **7.1.4 Masking & Secrecy:** Implement behaviors where agents with a hidden nature actively try to satisfy their
    supernatural needs in secret, avoiding perception by other agents.

### **Sub-Phase 7.2: Discovery & Revelation (📋 Planned)**

-   [ ] **7.2.1 Evidence System:** Actions related to hidden natures (e.g., a vampire feeding) can leave behind
    `Evidence` as entities in the world.
-   [ ] **7.2.2 Investigation Behavior:** NPCs with high `openness` or those who are negatively affected by these
    actions may be prompted to investigate the evidence.
-   [ ] **7.2.3 Revelation & Accusation:** If an NPC gathers enough evidence, it can form a `Belief` about another
    agent's hidden nature, potentially leading to social ostracism, accusations, or conflict.

## Phase 8: Long-Term Technical Evolution

**Objective:** Plan for the long-term technical health and evolution of the project, primarily the migration to a 3D
environment.

### **Sub-Phase 8.1: 3D Asset & Pipeline Preparation (📋 Planned)**

-   [ ] **8.1.1 3D Modeling:** Create or acquire a base set of 3D models for NPCs and environmental assets (e.g., in
    Blender).
-   [ ] **8.1.2 GLTF Pipeline:** Establish a reliable pipeline for exporting models from Blender and importing them into
    Bevy using the GLTF format.

### **Sub-Phase 8.2: Phased 3D Migration (📋 Planned)**

-   [ ] **8.2.1 Physics Migration:** Create a separate project branch to migrate from `bevy_rapier2d` to
    `bevy_rapier3d`. Adapt all physics-related code, including colliders and spatial queries.
-   [ ] **8.2.2 Rendering Migration:** Replace `SpriteBundle`s with `PbrBundle`s. Implement a basic 3D camera and
    lighting setup.
-   [ ] **8.2.3 Navigation System Adaptation:** This is the most complex step. The neuro-inspired navigation system must
    be adapted for 3D space.
    -   [ ] Research and implement 3D equivalents for place cells and grid cells, possibly using volumetric activation
        fields.
    -   [ ] Adapt the activation spreading algorithm to work with 3D spatial relationships (e.g., considering
        verticality).
-   [ ] **8.2.4 Final Integration:** Merge the 3D branch into the main project once all core systems are functional in
    the new dimension.

## Phase 9: Machine Learning Integration

**Objective:** With a stable, networked, and feature-rich simulation, begin the process of using machine learning to
enhance and evolve agent behavior.

### **Sub-Phase 9.1: Data Collection & Observation (📋 Planned)**

-   [ ] **9.1.1 `ObservationBuilder` System:** Create a system that runs on the Bevy Worker and collects the state of a
    specific agent into a flattened vector (`Vec<f32>`). This vector is the "observation space".
-   [ ] **9.1.2 Data Logging:** Send this observation data to a dedicated data warehouse (e.g., a separate database or
    logging service) for offline analysis and training.
-   [ ] **9.1.3 Reward Function Definition:** Implement systems to calculate reward signals based on the agent's state
    changes (e.g., positive reward for satisfying a need, negative reward for taking damage).

### **Sub-Phase 9.2: Offline Training (📋 Planned)**

-   [ ] **9.2.1 Model Training:** Use the collected data to train an initial reinforcement learning model (e.g., using a
    Python framework like PyTorch or TensorFlow).
-   [ ] **9.2.2 Model Export:** Export the trained model weights in a format that can be loaded by the Bevy Worker (
    e.g., ONNX).

### **Sub-Phase 9.3: Online Inference (📋 Planned)**

-   [ ] **9.3.1 ML Inference in Bevy:** Integrate a Rust-based ML inference library (e.g., `tract` or `tch-rs`) into the
    Bevy Worker.
-   [ ] **9.3.2 Hybrid Decision-Making:** Create a new `ml_decision_making_system`. An agent can now be controlled
    either by the original utility-based system or by the ML model.
-   [ ] **9.3.3 A/B Testing:** Implement functionality to run both AI systems side-by-side to compare their performance
    and believability.

## Phase 10: Alpha, Beta, and Live Operations

**Objective:** Transition the project from a development build to a live, playable experience.

### **Sub-Phase 10.1: Pre-Alpha & Alpha (📋 Planned)**

-   [ ] **10.1.1 Feature Freeze:** Define a set of core features for the first playable version and freeze further
    development on new systems.
- [--] **10.1.2 Internal Testing:** Conduct rigorous internal testing focused on stability, performance, and major bugs.
-   [ ] **10.1.3 Closed Alpha:** Invite a small, trusted group of players to test the game, focusing on gameplay
    feedback and server load.

### **Sub-Phase 10.2: Beta (📋 Planned)**

-   [ ] **10.2.1 Open Beta:** Open the game to a wider audience. Focus on scalability testing, bug hunting, and
    balancing.
-   [ ] **10.2.2 Community Management:** Establish channels for player feedback and communication.
-   [ ] **10.2.3 Final Polish:** Address feedback from the beta period, focusing on user experience, UI, and game
    balance.

### **Sub-Phase 10.3: Launch & Live-Ops (📋 Planned)**

-   [ ] **10.3.1 Official Launch:** Release the game to the public.
-   [ ] **10.3.2 Monitoring & Maintenance:** Implement robust server monitoring, logging, and on-call procedures for
    handling live issues.
-   [ ] **10.3.3 Content Cadence:** Establish a roadmap for post-launch content, including new systems, story events,
    and bug fixes based on live player data.

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
