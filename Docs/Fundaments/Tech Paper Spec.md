# Believable AI Agents in MMORPG: Design Specifications

## Chapter 1: Executive Summary

This document outlines a design philosophy for Non-Player Characters (NPCs) in an MMORPG, aiming to create **believable
AI agents** through **emergent social dynamics**. The goal is not to perfectly simulate human neurobiology, but to draw
inspiration from scientific research to craft agents that *feel* like other players, enriching the game world with
organic, unpredictable, and compelling social interactions. Instead of rigid blueprints, agents will operate on a "
behavior recipe" – a flexible, interconnected web of internal states and learned responses.

The core principle is **selective fidelity**: prioritize elements that enhance player-facing believability and emergent
drama, while abstracting or simplifying complex scientific details that offer little direct value to player experience.
Agents will possess a "virtual self" that can shift and adapt, perceive the world through subjective biases, and make
decisions influenced by emotions, needs, and a persistent social memory. This approach will foster an illusion of
conscious will and complex inner lives, creating a dynamic world where NPC relationships, motivations, and
misunderstandings generate authentic, unscripted narratives for players to discover and influence.

## Chapter 2: Key Scientific Insights Guiding Design

The design of believable AI agents is founded on several key insights drawn from cognitive science and psychology:

* ### The Illusion of Conscious Will & Constructed Self

  * **Insight:** Human conscious will is often an experience or a feeling, rather than a direct causal force for
    action[^1][^2][^3]. Similarly, the "self" is not a fixed, singular entity but a flexible construction, akin to an "
    operating
    system" or "software shell" that can vary and even switch within an individual[^4][^5][^6][^7][^8][^9][^10]. This means agents can appear to
    possess conscious will and distinct personalities without requiring complex, literal consciousness.
  * **Translation to Mechanics:** AI agents will operate with a "virtual agent" or "persona" that defines their
    current behavioral tendencies, goals, and emotional responses [^6][^7]. This persona can dynamically change based on
    environmental, social, or internal triggers, making agents adaptable and less predictable than typical rule-based
    AI[^4][^11]. The experience of will can be mimicked by linking an agent's actions to its conscious thoughts or expressed
    desires, even if the true causal chain is a complex interplay of internal systems[^3].
* ### Perception as Hypothesis-Testing & Selective Reality

  * **Insight:** Human perception is not a passive reception of objective reality, but an active, "expectation-driven"
    process of hypothesis testing[^12][^13]. We constantly use prior schemas, interests, and current concerns to interpret
    ambiguous sensory input, often constructing "fictions" or biased understandings[^14][^15][^16]. This leads to cognitive
    illusions, framing effects, and overconfidence in our interpretations, even with limited evidence[^17].
  * **Translation to Mechanics:** Agents will possess "Perception Bias Profiles" and "Schema Memory" that filter and
    interpret sensory input. Their interpretation of events, other agents' actions, or the environment will be
    influenced by their current emotional state, active goals, and learned biases, rather than purely objective data.
    This allows for emergent misunderstandings and varied reactions to the same event, mirroring human subjective
    experience.
* ### Behavior as a Seamless Web of Influences

  * **Insight:** Human behavior is a complex product of intertwined biological and environmental factors, ranging from
    genetics and early development to current sensory input and hormone levels[^18][^19]. There is "no space" for a free will
    concept that is "in your brain but not of your brain"[^19]. This perspective emphasizes a deterministic, yet incredibly
    intricate, "behavior recipe."
  * **Translation to Mechanics:** Agent actions will emerge from a sophisticated interplay of internal systems,
    including needs, emotions, memory, and personality traits. Instead of a linear decision tree, actions are selected
    through a dynamic arbitration process that weights various influences[^20]. This "seamless" integration will make agent
    behavior feel holistic and deeply rooted in their simulated being, even if the underlying mechanisms are
    simplified.
* ### The Social & Emotional Drivers of Decision-Making

  * **Insight:** Emotions significantly influence human decision-making, often described as the "emotional tail wags
    the rational dog"[^21]. Social interactions can lead to emotional contagion[^22], and the prefrontal cortex plays a crucial
    role in regulating these emotional responses in a social context[^23][^24]. Basic needs (physiological, safety, belonging)
    also drive fundamental motivations[^22].
  * **Translation to Mechanics:** Agents will have dynamic "Emotional States" and "Needs" components that directly
    influence their goals and action priorities[^20][^22]. Emotional contagion will allow moods to spread through groups,
    fostering emergent collective behaviors like panic or celebration[^22]. The "affect heuristic" will be simulated, where
    an agent's emotional appraisal of a situation can alter its perception of risks and benefits[^21].
* ### Reputation and Social Memory as Behavioral Guides

  * **Insight:** In human societies, reputation and past interactions are critical for guiding current and future
    decisions[^25][^26][^27][^28]. Entities must be long-lived, ratings captured and distributed, and past interactions must inform
    present choices[^26][^28].
  * **Translation to Mechanics:** Agents will maintain explicit "Social Memory" and "Relationship Components" for
    other entities, including players[^29]. These memories will store interaction history and derive aggregate "Trust"
    and "Affiliation" scores, which in turn will modulate future interactions, cooperation, and conflict[^25][^26]. This creates
    persistent, evolving social relationships and emergent factions.

## Chapter 3: Core Systems Implementation Guidance (Conceptual)

This chapter outlines the conceptual design of core systems using the Bevy Entity Component System (ECS) paradigm.

### 3.1 Agent Core (Virtual Agent Operating System)

* **Explanation:** This system defines an agent's overarching "personality" and high-level behavioral drivers, embodying
  the concept of the flexible, constructed self. It acts as a central hub, influencing all other agent systems.
* **Bevy ECS Representation:**
  * **Entity:** `Agent` (unique identifier for each NPC).
  * **Components:**
    * `CurrentPersona: PersonaType` (e.g., `Leader`, `Follower`, `LoneWolf`, `Trader`, `Defender`) [^6][^7][^8].
    * `ActiveGoals: Vec<Goal>` (e.g., `ForageFood`, `PatrolArea`, `Socialize`, `FleeThreat`).
    * `MemoryAccessLayer: Vec<MemoryFlag>` (influences how agents retrieve and use memories, e.g.,
      `BiasTowardsThreatMemories`).
    * `PerceptionBiasProfile: PerceptionBiasSettings` (detailed in 3.2).
  * **Systems:**
    * `AgentControllerSystem`: Orchestrates other systems by reading `CurrentPersona` and `ActiveGoals`. It's a
      meta-system, not directly changing state but signaling other systems how to behave.
    * `PersonaSwitchSystem`: Evaluates internal and external triggers to change an agent's `CurrentPersona`.
* **Values/Ranges/Formulas:** `CurrentPersona` is an enum, `ActiveGoals` is a prioritized list.
* **Player-Facing Impact:** Players will observe distinct behavioral patterns and shifts in an agent's demeanor. An
  agent might be friendly one day and guarded the next, or a "follower" might unexpectedly rise to "leader" status.
* **Simplifications:** Start with a few distinct `PersonaType` enums and clear, simple triggers for switching between
  them.
* **Mermaid Diagram:**
  ```mermaid
  graph TD
      A[Agent Entity] --> C(CurrentPersona: PersonaType)
      A --> B(ActiveGoals: Vec<Goal>)
      C --> D{AgentControllerSystem}
      B --> D
      D -- Orchestrates --> E[Perception & Bias System]
      D -- Orchestrates --> F[Emotional & Motivation System]
      D -- Orchestrates --> G[Decision-Making & Action System]
      H[Environmental/Social Triggers] --> I{PersonaSwitchSystem}
      I --> C
  ```
* **Quick-win Prototype:** Implement 2-3 basic personas (e.g., "Aggressive," "Helpful," "Neutral"). Create a simple
  trigger (e.g., player performs a hostile action) that switches a "Neutral" agent to "Aggressive."
* **Drama/Emergence Focus:** Creates individual character arcs and allows for dynamic shifts in an agent's role within
  social structures. A passive agent might become a staunch defender under threat, or a leader might become reclusive
  after a failure.
* **Essential vs Optional Tags:** **Essential** for the core "believable agent" feel.
* **Complexity Warnings:** Balancing dynamic persona shifts with maintaining a sense of persistent identity for the
  agent. Too frequent changes can make agents feel random rather than complex.
* **Inter-system Interactions:** Interacts heavily with all other systems, as `CurrentPersona` and `ActiveGoals`
  influence perception, emotion generation, and action selection.
* **Priority/Dependency Guidance:** `Agent Core` is foundational. Implement basic persona types and goal management
  first.
* **Validation for Emergent Behavior:** Observe if agents exhibit diverse and context-appropriate high-level behaviors
  over extended play, including unexpected role adoption or shifts.

### 3.2 Perception & Bias System

* **Explanation:** This system models how agents receive and interpret information from the game world, acknowledging
  that perception is subjective and prone to biases[^12][^13][^14][^15].
* **Bevy ECS Representation:**
  * **Entity:** `Agent`, `ObservableObject`, `OtherAgent`.
  * **Components:**
    * `RawSensoryInput: Vec<RawSignal>` (e.g., `VisualSignal { object_id, type, distance }`,
      `AuditorySignal { source_id, sound_type }`).
    * `PerceptionBuffer: Vec<PerceivedFact>` (interpreted, potentially biased facts, e.g.,
      `PerceivedThreat { source_id, level }`, `PerceivedResource { type, value }`).
    * `PerceptionBiasProfile: PerceptionBiasSettings` (part of Agent or a dedicated component).
    * `SchemaMemory: HashMap<StimulusType, LearnedInterpretation>` (stores learned patterns and their associated
      interpretations[^16]).
  * **Systems:**
    * `SensoryProcessingSystem`: Converts `RawSensoryInput` into initial `PerceivedFact`s.
    * `BiasApplicationSystem`: Modifies `PerceptionBuffer` entries based on `PerceptionBiasProfile` (e.g., amplifies
      perceived threats if `ThreatSalience` is high).
    * `CategorizationSystem`: Interprets biased `PerceptionBuffer` data using `SchemaMemory` and `ActiveGoals` to
      form a coherent (though potentially flawed) `WorldModel` for the agent[^16].
* **Values/Ranges/Formulas:**
  * `PerceptionBiasSettings`:
    * `FamiliarityBias: f32` (0.0-1.0, e.g., `0.0` for novelty-seeking, `1.0` for preferring known elements).
    * `ThreatSalience: f32` (0.0-1.0, amplifies perceived threat levels).
    * `GoalRelevanceFilter: f32` (0.0-1.0, filters out information not relevant to current goals).
  * **Formula Example (Simplified Threat Perception):**
    `perceived_threat_level = raw_threat_signal * agent.ThreatSalience + (1.0 - agent.FamiliarityBias) * raw_novelty_score`
* **Player-Facing Impact:** Agents might misinterpret player actions (e.g., a friendly gesture seen as hostile due to
  high `ThreatSalience`), react unexpectedly to environmental cues, or be overconfident in limited observations[^17]. This
  aligns with "Plato's Cave" idea of not directly accessing reality.
* **Simplifications:** Start with 2-3 simple biases affecting only a few `PerceivedFact` types. Limit `SchemaMemory` to
  basic, learned interpretations.
* **Mermaid Diagram:**
  ```mermaid
  graph TD
      A[RawSensoryInput] --> B{SensoryProcessingSystem}
      B --> C[PerceptionBuffer]
      C --> D{BiasApplicationSystem}
      D -- PerceptionBiasProfile --> E[CategorizationSystem]
      E -- ActiveGoals --> E
      E -- SchemaMemory --> F[Agent's WorldModel]
      F --> G[Decision-Making]
  ```
* **Quick-win Prototype:** Give some agents a very high `ThreatSalience`. Observe them frequently reacting with fear or
  aggression to otherwise neutral stimuli, particularly other players.
* **Drama/Emergence Focus:** Enables deep emergent narratives where agents' actions stem from genuine (simulated)
  misunderstandings, rather than scripted errors. Leads to emergent conflicts, cautious interactions, or unwarranted
  trust.
* **Essential vs Optional Tags:** **Essential** for believable, non-omniscient agents.
* **Complexity Warnings:** Can make debugging challenging if biases are too complex. Start simple and iterate.
* **Inter-system Interactions:** `PerceptionBuffer` directly feeds `Emotional & Motivation System` and
  `Decision-Making & Action System`. `ActiveGoals` and `CurrentPersona` (from Agent Core) influence which biases are
  active.
* **Priority/Dependency Guidance:** Implement after basic `Agent Core` and basic `Emotional State` to allow biases to
  interact with core motivations.
* **Validation for Emergent Behavior:** Test if agents in similar situations react differently based on their
  `PerceptionBiasProfile` and if these differences lead to divergent behavioral paths (e.g., fight vs. flee, trust vs.
  distrust).

### 3.3 Emotional & Motivation System

* **Explanation:** Manages an agent's basic needs and dynamic emotional states, which are fundamental drivers of
  behavior and critical for human-like social dynamics.
* **Bevy ECS Representation:**
  * **Entity:** `Agent`.
  * **Components:**
    * `Needs: HashMap<NeedType, f32>` (e.g., `Hunger: 0.8`, `Safety: 0.3`, `Belonging: 0.5`)[^22].
    * `EmotionalState: EmotionalVector` (e.g., `Fear: 0.7`, `Anger: 0.2`, `Joy: 0.1` representing valence/arousal)[^22].
    * `SocialInfluenceBuffer: Vec<EmotionalSignal>` (receives emotional inputs from nearby agents for contagion).
    * `StressLevel: f32` (0.0-1.0, derived from needs, threats, emotions).
  * **Systems:**
    * `NeedSatisfactionSystem`: Increases/decreases `Needs` over time and triggers actions to satisfy critical
      needs.
    * `EmotionGenerationSystem`: Updates `EmotionalState` based on `Needs`, `WorldModel` (from Perception), and
      `StressLevel`.
    * `EmotionalContagionSystem`: Spreads `EmotionalState` between proximate agents, modulated by personality
      traits[^22].
    * `StressResponseSystem`: Calculates `StressLevel` and updates `DecisionWeightingProfile`[^20][^30].
* **Values/Ranges/Formulas:**
  * `Needs`: 0.0-1.0 (0.0 = completely satisfied, 1.0 = critical).
  * `EmotionalVector`: Each emotion 0.0-1.0.
  * **Formula Example (Emotional Contagion):**
    `recipient.EmotionalState.fear += sender.EmotionalState.fear * ContagionFactor * recipient.SocialityTrait * DistanceDecay`
    `ContagionFactor`: 0.05-0.2. `SocialityTrait`: from Personality (0.0-1.0). `DistanceDecay`:
    `1 - (distance / max_range)`.
  * **Formula Example (Stress Level Calculation):**
    `stress_level = (critical_needs_sum * 0.4) + (perceived_threat_sum * 0.3) + (emotional_state.anxiety * 0.3)`
* **Player-Facing Impact:** Agents visibly react with fear, joy, or anger. Groups of agents might collectively panic or
  become agitated. Agents might prioritize urgent needs over long-term goals.
* **Simplifications:** Focus on 3-5 primary emotions. Use simple accumulation and decay for needs.
* **Mermaid Diagram:**
  ```mermaid
  graph TD
      A[Agent] --> B(Needs: HashMap)
      A --> C(EmotionalState: EmotionalVector)
      A --> D(StressLevel: f32)
      E[WorldModel (from Perception)] --> F{EmotionGenerationSystem}
      B --> F
      C --> F
      D --> F
      F --> C
      G[Nearby Agents' EmotionalState] --> H{EmotionalContagionSystem}
      H --> C
      C --> I{StressResponseSystem}
      B --> I
      E --> I
      I --> D
      D --> J[Decision-Making]
  ```
* **Quick-win Prototype:** Introduce a "threat" object (e.g., a monster). Observe agents with high `StressLevel` and
  `Fear` component prioritize `FleeThreat` actions. Then, observe how neighboring agents gain `Fear` based on their
  `SocialityTrait`.
* **Drama/Emergence Focus:** Creates group dynamics like mob behavior, shared celebrations, or collective despair.
  Emotions drive spontaneous, less predictable actions.
* **Essential vs Optional Tags:** **Essential**.
* **Complexity Warnings:** Emotion states can rapidly escalate or dampen; requires careful tuning of thresholds and
  decay rates.
* **Inter-system Interactions:** `EmotionalState` and `Needs` are core inputs to `Decision-Making & Action System`.
  `StressLevel` influences `DecisionWeightingProfile`. `WorldModel` (from `Perception`) drives `EmotionGeneration`.
* **Priority/Dependency Guidance:** Implement basic needs and 2-3 core emotions first, then add contagion and stress.
* **Validation for Emergent Behavior:** Observe if groups of agents show synchronized emotional responses to events and
  if individual agent actions are clearly influenced by their current emotional and stress levels.

### 3.4 Social Memory & Reputation System

* **Explanation:** Enables agents to remember past interactions and form dynamic opinions about other entities, shaping
  their social behavior and relationships [^25][^26][^27][^28].
* **Bevy ECS Representation:**
  * **Entity:** `Agent`.
  * **Components:**
    * `SocialMemory: HashMap<EntityId, Vec<InteractionEvent>>` (stores a log of recent interactions with specific
      entities).
    * `RelationshipComponent: HashMap<EntityId, (Trust: f32, Affiliation: f32)>` (aggregate scores representing
      current relationship status with other entities).
  * **Systems:**
    * `InteractionLoggingSystem`: Records `InteractionEvent`s (e.g., `(player_id, NPCHelpedPlayer, success)`,
      `(player_id, NPCAttackedPlayer, critical)`).
    * `ReputationUpdateSystem`: Periodically processes `SocialMemory` to update `Trust` and `Affiliation` scores in
      `RelationshipComponent` for each known entity.
    * `MemoryDecaySystem`: Gradually reduces the influence of older `InteractionEvent`s or reduces
      `Trust/Affiliation` over time for less frequent interactions.
* **Values/Ranges/Formulas:**
  * `Trust`: -1.0 (complete distrust) to 1.0 (complete trust).
  * `Affiliation`: -1.0 (hostile) to 1.0 (friendly).
  * **Formula Example (Trust Update):**
    `target.RelationshipComponent[source].Trust += GetTrustDelta(action_type, outcome) * (1.0 + source.Personality.Agreeableness * 0.2)`
    `TrustDelta` values: e.g., `NPCHelpedPlayer: +0.1`, `NPCAttackedPlayer: -0.2`.
  * **Parameter Ranges:**
    * `MemoryDecayRate`: 0.005-0.02 per game hour for `Trust/Affiliation` (small, continuous decay).
    * `InteractionEventRetentionTime`: 1-5 game days (how long detailed events are stored).
* **Player-Facing Impact:** Agents will treat players and other NPCs differently based on past interactions. A helpful
  player might be greeted warmly, while a hostile one is avoided or attacked. Factions and persistent grudges can
  emerge.
* **Simplifications:** Initially, store only aggregate `Trust` and `Affiliation` scores per entity, rather than detailed
  event logs.
* **Mermaid Diagram:**
  ```mermaid
  graph TD
      A[Agent] --> B(SocialMemory: HashMap)
      A --> C(RelationshipComponent: HashMap)
      D[Observed Interaction] --> E{InteractionLoggingSystem}
      E --> B
      B --> F{ReputationUpdateSystem}
      F --> C
      G{MemoryDecaySystem} --> B
      G --> C
      C --> H[Decision-Making]
  ```
* **Quick-win Prototype:** If a player helps an agent, that agent's `RelationshipComponent` for the player increases in
  `Trust` and `Affiliation`. Subsequent interactions show the agent being more cooperative towards the player.
* **Drama/Emergence Focus:** Creates rich, evolving social networks among NPCs and between NPCs and players, leading to
  emergent alliances, rivalries, and long-term consequences for actions.
* **Essential vs Optional Tags:** **Essential** for persistent social dynamics.
* **Complexity Warnings:** Memory management for `SocialMemory` can be a performance bottleneck with many agents.
  Consider summarization or purging old events.
* **Inter-system Interactions:** `RelationshipComponent` strongly influences `Decision-Making & Action System` (e.g.,
  probability of cooperation/attack). `Perception & Bias System` might bias how new interactions are interpreted based
  on existing `Trust` levels.
* **Priority/Dependency Guidance:** Implement after `Agent Core` and basic `Perception` for agents to act on their
  perceived social world.
* **Validation for Emergent Behavior:** Observe the formation and dissolution of alliances/rivalries, and if agents'
  long-term behavior towards specific entities aligns with their recorded social memory.

### 3.5 Decision-Making & Action System

* **Explanation:** This system determines an agent's specific actions based on its current internal state, perceived
  environment, and personality. It arbitrates between different behavioral influences (e.g., habitual vs. deliberative)
  to choose the "behavior recipe"[^20].
* **Bevy ECS Representation:**
  * **Entity:** `Agent`.
  * **Components:**
    * `ActionQueue: Vec<Action>` (ordered list of actions to perform).
    * `DecisionWeightingProfile: DecisionWeights` (e.g., `HabitualWeight: f32`, `DeliberativeWeight: f32`,
      `EmotionalIntuitiveWeight: f32`)[^20].
    * `LearnedBehaviors: HashMap<Situation, ActionPattern>` (stores successful, habitual action patterns)[^20].
    * `SkillSet: HashMap<SkillType, SkillLevel>` (e.g., `Combat: 0.7`, `Foraging: 0.5`).
  * **Systems:**
    * `BehaviorRecipeSystem`: Generates a set of possible actions given the current `WorldModel`, `ActiveGoals`,
      `Needs`, `EmotionalState`.
    * `DecisionArbitrationSystem`: Chooses the optimal action(s) from the possible set by applying weights from
      `DecisionWeightingProfile` (influenced by `StressLevel`, `CurrentPersona`, `Personality`)[^20]. It may prioritize
      quick, habitual responses under stress or carefully calculated, deliberative actions when time allows.
    * `ActionExecutionSystem`: Executes actions from `ActionQueue`, potentially consuming resources or altering the
      environment.
    * `LearningSystem`: Updates `LearnedBehaviors` and `DecisionWeightingProfile` based on action outcomes and
      rewards (Reinforcement Learning)[^31][^32][^33][^34][^35].
* **Values/Ranges/Formulas:**
  * `DecisionWeights`: Sum to 1.0 (e.g., `Habitual: 0.4`, `Deliberative: 0.3`, `Emotional: 0.3`).
  * **Formula Example (Action Score for Decision):**
    `action_score = (habitual_likelihood * DecisionWeightingProfile.HabitualWeight) + (deliberative_value * DecisionWeightingProfile.DeliberativeWeight) + (emotional_impulse * DecisionWeightingProfile.EmotionalIntuitiveWeight)`
    `emotional_impulse` could be `EmotionalState.fear * FleeImpulseFactor`.
  * **Parameter Ranges:** `DecisionWeightingProfile` can shift based on `StressLevel` (e.g.,
    `deliberative_weight *= (1.0 - StressLevel) * DeliberativeReductionFactor`)[^20].
* **Player-Facing Impact:** Agents display a variety of behaviors, from spontaneous emotional reactions to careful
  planning. An agent might impulsively attack a perceived threat (high emotional weight) or meticulously gather
  resources (high deliberative weight).
* **Simplifications:** Start with simpler action generation (e.g., predefined action lists for specific states) before
  implementing complex `BehaviorRecipeSystem`.
* **Mermaid Diagram:**
  ```mermaid
  graph TD
      A[Agent's Internal State] --> B{BehaviorRecipeSystem: Generate Actions}
      B --> C[Possible Actions]
      C --> D{DecisionArbitrationSystem}
      D -- DecisionWeightingProfile --> D
      D -- LearnedBehaviors --> D
      D -- SkillSet --> D
      D --> E[Chosen Action(s)]
      E --> F{ActionExecutionSystem}
      F --> G[World State Change]
      G --> H{LearningSystem}
      H --> LearnedBehaviors
      H --> DecisionWeightingProfile
  ```
* **Quick-win Prototype:** Agents with high `StressLevel` will have their `DeliberativeWeight` reduced, making them more
  likely to choose simple, immediate actions (e.g., `Flee`, `Attack`) over complex tasks.
* **Drama/Emergence Focus:** Creates realistic and sometimes frustrating or surprising agent choices. Allows for agents
  to learn and adapt, developing unique "styles" of play over time, like human players do.
* **Essential vs Optional Tags:** **Essential** for agent behavior.
* **Complexity Warnings:** Balancing the influence of multiple decision factors is crucial. Too many rules can lead to
  agents getting "stuck" or acting illogically. Reinforcement learning needs careful reward design[^34][^35].
* **Inter-system Interactions:** This system is the output hub, taking input from `Agent Core`, `Perception & Bias`,
  `Emotional & Motivation`, and `Social Memory`. Its output (actions) feeds back into the `World State Change` and
  `LearningSystem`.
* **Priority/Dependency Guidance:** Implement once `Agent Core`, `Perception`, and `Emotional & Motivation` can provide
  sufficient input.
* **Validation for Emergent Behavior:** Assess if agents perform actions that appear purposeful, varied, and responsive
  to their changing internal and external conditions. Look for evidence of learning over time.

---

## Chapter 4: Behavior Parameters and Personalities

This chapter details the parameters and frameworks that define individual agent behavior, ensuring diversity and
believability.

### 4.1 Personality Framework

* **Dimensions:** While not explicitly listed in the sources, the concept of individual differences in behavior and
  social complexity is highlighted[^4][^24][^36]. We will adapt a widely recognized framework like the Big Five to provide broad,
  player-observable personality traits.
  * **Openness (to experience):** E.g., willingness to explore new areas, try new tactics, or interact with unknown
    entities.
  * **Conscientiousness:** E.g., adherence to goals, reliability in tasks, planning horizon.
  * **Extraversion:** E.g., propensity for social interaction, group leadership, assertiveness.
  * **Agreeableness:** E.g., cooperativeness, empathy, forgiveness, likelihood of peaceful resolution.
  * **Neuroticism:** E.g., emotional stability, anxiety levels, stress reactivity, likelihood of panic.
* **Research Basis:** The need for individuality is supported by discussions on individual differences in causal
  influence processing[^4][^36], social complexity[^24], and the intertwined biological/environmental factors shaping who we are[^18][^19].
  Emotional responses[^23][^37] and self-control[^38][^39] also have individual variability.
* **Bevy ECS Component Mapping:**
  * `PersonalityComponent`: A component attached to each `Agent` entity, containing normalized `f32` values (0.0-1.0)
    for each dimension.
    * `openness: f32`
    * `conscientiousness: f32`
    * `extraversion: f32`
    * `agreeableness: f32`
    * `neuroticism: f32`
* **Modulation Rules (Examples):**
  * `Openness`: High values increase `ExploreNewArea` action probability, decrease `FamiliarityBias` in
    `PerceptionBiasProfile`.
  * `Agreeableness`: High values increase `EmotionalContagion` susceptibility, decrease `ThreatSalience`, and increase
    `TrustDelta` when observing prosocial actions. Also influences `RelationshipComponent` decay.
  * `Neuroticism`: High values amplify `EmotionalState.fear` and `StressLevel` increases from perceived threats.
    Reduces `DeliberativeWeight` under stress.
* **Realistic Population Distributions:**
  * Each trait will be assigned a mean and standard deviation, allowing for a realistic distribution across the NPC
    population (e.g., a normal distribution).
  * `PopulationPersonalityDistributionResource`: A global resource storing these statistical parameters.
  * Example: `extraversion: {mean: 0.5, std_dev: 0.2}`.
* **Mermaid Diagram (Personality Influence Example):**
  ```mermaid
  graph TD
      A[Agent] --> B(PersonalityComponent)
      B -- Neuroticism (High) --> C[EmotionalState: Increased Fear/Anxiety]
      C --> D[StressLevel: Higher Response]
      D --> E[DecisionWeightingProfile: Reduced DeliberativeWeight]
      B -- Agreeableness (High) --> F[RelationshipComponent: Faster Trust Gain]
      F --> G[DecisionArbitrationSystem: Increased Cooperation Probability]
  ```
* **Quick-win Prototype:** Generate a batch of agents with varied `PersonalityComponent` values. Observe how agents with
  high `Agreeableness` are more likely to `Cooperate` when offered a fair trade, while those with low `Agreeableness`
  are more prone to `RejectOffer` or `Attack`.
* **Drama/Emergence Focus:** Ensures a diverse and interesting cast of characters. Leads to emergent social structures,
  power dynamics, and individual reputation arcs.
* **Essential vs Optional Tags:** **Essential**.
* **Simplifications:** Start with 2-3 prominent traits (e.g., Aggressiveness/Agreeableness, Bravery/Neuroticism,
  Sociality/Extraversion) before implementing the full Big Five.
* **Complexity Warnings:** Interactions between multiple personality traits can create complex emergent behaviors. Keep
  modulation rules clear and testable.

### 4.2 Core Behavior Parameters (with Research-Based Values)

This section maps prioritized scientific concepts to specific parameters and provides ranges inspired by the sources,
emphasizing human-like perception and social interaction.

* #### Perception Biases with Misinterpretation

  * **Concept Name:** **Subjective Perception Filters**
  * **Key Insight:** Agents interpret sensory input through their internal states and learned schemas, leading to
    biased and often inaccurate "WorldModels"[^12][^13][^14][^15][^16][^17].
  * **Mechanic:** Sensory inputs are processed through an agent's `PerceptionBiasProfile`, dynamically altering the
    perceived reality.
  * **Implementation Strategy:**
    * **ECS Component:**
      `PerceptionBiasProfile { familiarity_bias: f32, threat_salience: f32, goal_relevance_filter: f32 }`.
    * **ECS System:** `BiasApplicationSystem` modifies `PerceptionBuffer` data.
    * **Tuning Parameters:**
      * **`familiarity_bias` (0.0 - 1.0):** How strongly an agent prefers/distorts towards familiar stimuli. `0.0`
        for novelty-seeking (explores), `1.0` for comfort in routine (avoids new). Default: `0.5`. Influences
        `CategorizationSystem`'s reliance on `SchemaMemory`.
      * **`threat_salience` (0.0 - 2.0):** Multiplier for perceived threat levels. `0.0` (ignores threats),
        `1.0` (objective), `2.0` (hyper-vigilant). Default: `0.7` (slight underestimation, perhaps to allow for
        exploration). High values lead to overestimation of danger[^17].
      * **`goal_relevance_filter` (0.0 - 1.0):** How much current `ActiveGoals` filter out irrelevant information.
        `0.0` (distracted by everything), `1.0` (laser-focused). Default: `0.6`. Agents focus on information
        directly supporting their goals, potentially neglecting other important cues[^17].
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Raw Visual Input] --> B(SensoryProcessingSystem)
        B --> C[PerceptionBuffer]
        C --> D{BiasApplicationSystem}
        D -- PerceptionBiasProfile --> E[Agent's WorldModel (Biased)]
        E --> F[Decision-Making]
    ```
  * **Quick-win Prototype:** Implement a simple object (e.g., "Shiny Rock"). An agent with low
    `goal_relevance_filter` (and no active mining goal) might still stop to "inspect" it, whereas a high
    `goal_relevance_filter` agent on a `FleeThreat` mission would ignore it. An agent with high `threat_salience` will
    always perceive a distant, neutral player as a potential attacker.
  * **Player Impact:** Players will find agents frustratingly oblivious or surprisingly cautious, leading to
    unscripted drama. Agents might miss obvious opportunities or become aggressive without player intent, creating a
    more dynamic and challenging world.
  * **Simplifications:** Apply biases as simple multipliers or thresholds. Do not simulate detailed neural pathways.
  * **Complexity Warnings:** Too many biases can make agent behavior appear arbitrary. Begin with a few high-impact
    biases.
* #### Emotional Contagion

  * **Concept Name:** **Affective Resonance**
  * **Key Insight:** Emotions can spread between individuals in proximity[^22]. The "emotional tail wags the rational dog"[^21].
  * **Mechanic:** Agents' `EmotionalState` can be influenced by nearby agents' `EmotionalState`, modulated by
    `SocialityTrait`.
  * **Implementation Strategy:**
    * **ECS Component:** `EmotionalState { fear: f32, anger: f32, joy: f32, ... }`, `SocialityTrait: f32` (from
      Personality).
    * **ECS System:** `EmotionalContagionSystem`.
    * **Tuning Parameters:**
      * **`contagion_factor` (0.0 - 0.2):** Strength of emotional transfer. `0.0` (no contagion), `0.2` (highly
        contagious). Default: `0.1`.
      * **`sociality_trait_multiplier` (0.5 - 1.5):** How much an agent's `SocialityTrait` (e.g., derived from
        `Extraversion` or `Agreeableness`) amplifies/dampens contagion. Default: `1.0`.
      * **`distance_decay_exponent` (0.5 - 2.0):** How quickly contagion effect diminishes with distance. Higher
        exponent means faster decay. Default: `1.5`.
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Agent_A (EmotionalState)] --> C{EmotionalContagionSystem}
        B[Agent_B (EmotionalState)] --> C
        C -- Proximity & Traits --> D[Agent_A (Updated EmotionalState)]
        C -- Proximity & Traits --> E[Agent_B (Updated EmotionalState)]
    ```
  * **Quick-win Prototype:** Create a "panic event" for one agent. Observe a nearby group's `EmotionalState.fear`
    increase, leading to a chain reaction of fleeing behavior.
  * **Player Impact:** Players will experience crowds reacting collectively to events (e.g., mass panic during a
    monster attack, shared celebration after a victory), making the world feel more alive and reactive.
  * **Simplifications:** Focus on direct, immediate contagion within a limited radius for basic emotions. Ignore
    complex social network analysis for spread.
  * **Complexity Warnings:** Can lead to runaway emotions or unrealistic synchronicity if not carefully balanced with
    individual resistance and decay.
* #### Social Memory (Reputation & Relationships)

  * **Concept Name:** **Reputational Schema**
  * **Key Insight:** Past interactions dictate future social behavior[^25][^26][^27][^28].
  * **Mechanic:** Agents maintain `RelationshipComponent`s with other entities, which update based on observed
    actions.
  * **Implementation Strategy:**
    * **ECS Component:** `RelationshipComponent { relationships: HashMap<EntityId, RelationshipStatus> }`, where
      `RelationshipStatus { trust: f32, affiliation: f32 }`.
    * **ECS System:** `ReputationUpdateSystem` and `MemoryDecaySystem`.
    * **Tuning Parameters:**
      * **`trust_delta_prosocial` (0.05 - 0.2):** Amount `trust` increases for positive actions. Default: `0.1`.
      * **`trust_delta_antisocial` (-0.1 - -0.3):** Amount `trust` decreases for negative actions. Default:
        `-0.2`.
      * **`affiliation_delta_cooperate` (0.05 - 0.15):** Amount `affiliation` increases for cooperation. Default:
        `0.08`.
      * **`affiliation_delta_compete` (-0.05 - -0.15):** Amount `affiliation` decreases for competition. Default:
        `-0.08`.
      * **`memory_decay_rate_daily` (0.001 - 0.01):** Daily decay rate for `trust` and `affiliation` if no
        interaction. Default: `0.005`.
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Observed Action (Agent_X to Agent_Y)] --> B{ReputationUpdateSystem}
        B --> C[Agent_Z's RelationshipComponent (for Agent_X)]
        C --> D[Decision-Making (Agent_Z towards Agent_X)]
    ```
  * **Quick-win Prototype:** Two agents. Agent A performs a `HelpOther` action to Agent B. Agent B's
    `RelationshipComponent` for A increases. Later, if A requests `Help` from B, B is more likely to accept.
  * **Player Impact:** Players build meaningful relationships (or rivalries) with NPCs. Reputation impacts trade,
    quests, and social standing. Emergent factions could form around high-reputation leaders.
  * **Simplifications:** Focus on direct observation. A "gossip" system (indirect reputation sharing) could be an
    optional future addition.
  * **Complexity Warnings:** Balancing the speed of reputation change with the stability of relationships. Rapid
    shifts can feel gamey.
* #### Stress-Driven Decisions

  * **Concept Name:** **Adaptive Decision Arbitration**
  * **Key Insight:** High stress levels can shift decision-making from deliberative to habitual or emotional
    responses[^20][^30].
  * **Mechanic:** `StressLevel` dynamically alters the weights used by the `DecisionArbitrationSystem` for `Habitual`,
    `Deliberative`, and `EmotionalIntuitive` action choices.
  * **Implementation Strategy:**
    * **ECS Component:** `StressLevel: f32` (0.0-1.0),
      `DecisionWeightingProfile { habitual: f32, deliberative: f32, emotional_intuitive: f32 }`.
    * **ECS System:** `StressResponseSystem` calculates and updates `StressLevel` and modifies
      `DecisionWeightingProfile`.
    * **Tuning Parameters:**
      * **`stress_threshold_impulse` (0.5 - 0.8):** `StressLevel` at which deliberative thinking significantly
        reduces. Default: `0.7`.
      * **`deliberative_reduction_factor` (0.2 - 0.8):** Multiplier for `deliberative_weight` when above
        threshold. Default: `0.5`.
      * **`habitual_boost_factor` (1.2 - 2.0):** Multiplier for `habitual_weight` when above threshold. Default:
        `1.5`.
      * **`emotional_boost_factor` (1.2 - 2.0):** Multiplier for `emotional_intuitive_weight` when above
        threshold. Default: `1.5`.
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Agent Internal State] --> B(StressLevel: f32)
        B --> C{StressResponseSystem}
        C --> D[DecisionWeightingProfile (Adjusted)]
        D --> E[DecisionArbitrationSystem]
    ```
  * **Quick-win Prototype:** An agent with high `StressLevel` (e.g., from a prolonged fight) will abandon a complex
    `GatherResources` plan and instead choose immediate `Flee` or `Heal` actions, even if it's not the most "rational"
    long-term choice.
  * **Player Impact:** Agents making desperate, impulsive, or unexpectedly skilled (due to practiced habits) decisions
    under pressure. This creates more dynamic and believable combat or crisis scenarios.
  * **Simplifications:** Linear interpolation for weight changes based on `StressLevel`.
  * **Complexity Warnings:** Agents might appear "dumb" if stress always disables higher-level thought. Incorporate
    `StressResistance` (from Personality) to mitigate.
* #### Individuality (Virtual Agent "Operating Systems")

  * **Concept Name:** **Dynamic Persona System**
  * **Key Insight:** The subjective self is a flexible construction; different "operating systems" or personas can be
    run.
  * **Mechanic:** Agents can adopt different high-level `CurrentPersona` types, which significantly modify their
    underlying behavior parameters[^6][^7][^8][^9][^10].
  * **Implementation Strategy:**
    * **ECS Component:** `CurrentPersona: PersonaType`.
    * **ECS Resource:** `PersonaDefinitions: HashMap<PersonaType, PersonaModifierSet>` (defines how each persona
      modifies base stats or weights).
    * **ECS System:** `PersonaSwitchSystem` (triggers persona changes based on social cues, environmental context,
      or internal states).
    * **Tuning Parameters:**
      * **`persona_switch_threshold` (0.5 - 0.9):** How much internal/external pressure is needed to trigger a
        persona switch. Default: `0.7`.
      * **`persona_cooldown_time` (GameHours):** Prevents rapid, unnatural persona switching. Default: `2-6` game
        hours.
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Agent] --> B(CurrentPersona: PersonaType)
        B --> C{PersonaDefinitions: (Modifies other components)}
        C --> D[PerceptionBiasProfile]
        C --> E[DecisionWeightingProfile]
        C --> F[ActionPriorities]
        G[Trigger Conditions] --> H{PersonaSwitchSystem}
        H --> B
    ```
  * **Quick-win Prototype:** An agent with a base `Follower` persona will defer to a `Leader` agent. If the `Leader`
    is removed (e.g., killed by player), the `Follower` might (with a certain probability and based on its
    `Neuroticism` and `Extraversion` traits) switch to `LoneWolf` or `AspiringLeader` persona.
  * **Player Impact:** Agents demonstrate deep character development or unexpected leadership/subservience. Players
    must learn to interact with the current "version" of an NPC, making relationships more nuanced.
  * **Simplifications:** Start with distinct, easily recognizable personas. The `PersonaModifierSet` can be simple
    additive/multiplicative changes to other parameters.
  * **Complexity Warnings:** Ensuring persona transitions are narratively consistent and not jarring for the player.
* #### Communication Failures (Plato's Cave)

  * **Concept Name:** **Inferred Intentions & States**
  * **Key Insight:** An agent's true internal state is not directly accessible to others; interpretation is based
    solely on observable behaviors, filtered through the observer's own biases[^40][^41][^42][^43][^44][^45].
  * **Mechanic:** Agents infer the intentions and emotional states of other entities based on observed actions and
    their own `PerceptionBiasProfile` and `SchemaMemory`.
  * **Implementation Strategy:**
    * **ECS Component (on observing agent):** `ObservedEntityStates: HashMap<EntityId, InferredState>`, where
      `InferredState { inferred_intent: IntentType, inferred_emotion: EmotionalVector }`.
    * **ECS Component (on observed agent):** `ObservableActions: Vec<ActionType>`.
    * **ECS System:** `InterpretationSystem`.
    * **Tuning Parameters:**
      * **`inference_accuracy_base` (0.5 - 0.9):** Base likelihood of correctly inferring an intention/emotion.
        Default: `0.7`.
      * **`bias_influence_on_inference` (0.1 - 0.5):** How much the observer's `PerceptionBiasProfile` skews their
        inference. Default: `0.3`.
      * **`relationship_trust_modifier` (-0.2 - 0.2):** `Trust` (from `RelationshipComponent`) impacts inference.
        High trust leans towards positive inferences, low trust towards negative. Default: `0.15`.
  * **Mermaid Diagram:**
    ```mermaid
    graph TD
        A[Observed Agent's ObservableActions] --> B{InterpretationSystem}
        B -- Observing Agent's PerceptionBiasProfile --> B
        B -- Observing Agent's SchemaMemory --> B
        B -- Observing Agent's RelationshipComponent --> B
        B --> C[Observing Agent's InferredState (of Observed Agent)]
        C --> D[Decision-Making (Observing Agent)]
    ```
  * **Quick-win Prototype:** Player performs a `CollectResource` action near an agent. An agent with
    `high_threat_salience` might `InferIntent: Poaching` (hostile), leading to aggression, while an agent with high
    `Agreeableness` might `InferIntent: Gathering` (neutral).
  * **Player Impact:** Players must consider how their actions *appear* to NPCs, not just their own intent. Leads to
    emergent social conflicts and diplomatic challenges based on genuine misunderstandings.
  * **Simplifications:** Limit inference to clear action types and obvious emotions. Body language and facial
    expressions (complex animation features) are outside the scope of initial implementation.
  * **Complexity Warnings:** Requires a comprehensive mapping of `ObservableActions` to `InferredState` probabilities,
    then applying biases. Can be hard to debug when agents "misunderstand" repeatedly.

---

## Chapter 5: Validation Framework for Believable Agents

The ultimate goal is not scientific exactitude, but that agents *feel* like other players. The validation framework will
focus on assessing this subjective "human-likeness" and the emergence of compelling social dynamics.

### Ranked System Implementation Order

Prioritizing implementation based on foundational importance and immediate impact on believability:

1. **Agent Core (Basic Persona & Goals):** Establish the basic identity and drives for agents (e.g., `CurrentPersona` as
   a simple enum, `ActiveGoals` as a list). This is the foundation for all other behaviors.
2. **Emotional & Motivation System (Basic Needs & Emotions):** Introduce dynamic `Needs` (e.g., Hunger, Safety) and
   primary `EmotionalState` (Fear, Joy, Anger). This makes agents feel immediately reactive and "alive."
3. **Perception & Bias System (Basic Filtering):** Implement initial `PerceptionBiasProfile` (e.g., `ThreatSalience`)
   and `SchemaMemory` for interpreting basic environmental cues. This introduces subjectivity.
4. **Decision-Making & Action System (Simple Arbitration):** Create a basic `BehaviorRecipeSystem` and
   `DecisionArbitrationSystem` that uses `Needs`, `EmotionalState`, and `ActiveGoals` to select actions. Focus on a
   simple `ActionQueue`.
5. **Social Memory & Reputation System (Basic Trust/Distrust):** Implement rudimentary `RelationshipComponent` for other
   entities, allowing agents to track simple `Trust` and `Affiliation` values based on direct interactions.
6. **Communication Failures (Inferred Intentions & States):** Develop the `InterpretationSystem` for agents to infer
   intentions/emotions from observed `ObservableActions`, applying their `PerceptionBiasProfile`. This is crucial for
   emergent social drama.
7. **Personality Framework (Expanding Traits):** Introduce a more detailed `PersonalityComponent` (e.g., 3-5 traits) and
   integrate its influence into existing systems (e.g., `EmotionalContagion`, `ReputationUpdate`).
8. **Stress-Driven Decisions (Adaptive Decision Arbitration):** Refine the `StressResponseSystem` to dynamically adjust
   `DecisionWeightingProfile` based on `StressLevel`.
9. **Advanced Persona System (Dynamic Persona Switching):** Implement `PersonaSwitchSystem` with defined triggers and
   `PersonaDefinitions` to allow agents to adopt different roles and modify their behaviors more dramatically over time.

### Human-Likeness Checklist

This checklist focuses on player perception and emergent behavior to evaluate agent believability:

* **1. Emotional Responsiveness:** Do agents visibly and audibly (e.g., animations, sound effects) react emotionally (
  fear, joy, anger, sadness) to significant events, and do these emotions clearly influence their subsequent actions (
  e.g., fleeing in fear, celebrating in joy)?[^22][^23]
* **2. Social Awareness:** Do agents demonstrate different behaviors towards players/NPCs they "know" (based on
  `RelationshipComponent`) versus strangers? Do they respond differently to perceived threats/help from these known
  entities?[^25][^26][^27][^28]
* **3. Contextual Adaptation:** Do agent behaviors change logically and dynamically in response to varying environmental
  conditions (e.g., resource scarcity leads to more aggressive foraging, presence of a monster leads to vigilance or
  evasion)?[^31][^32]
* **4. Goal-Directedness with Flexibility:** Do agents pursue discernible goals (e.g., `GatherFood`,
  `DefendSettlement`), but also exhibit occasional, understandable deviations (e.g., getting distracted by a rare
  resource, panicking and fleeing even if their goal was to defend)?[^20][^21][^46]
* **5. Perceptual Bias & Misinterpretation:** Do agents sometimes misinterpret player actions or environmental cues in
  ways that create emergent (and often dramatic) situations, which make sense from the *agent's subjective
  perspective* (e.g., an agent attacks a player who was just observing, due to high `ThreatSalience` or a negative
  `RelationshipComponent`)?[^14][^16][^17]
* **6. Learning & Adaptation (over time):** Is there observable evidence of agents' behaviors or internal states (e.g.,
  `LearnedBehaviors`, `RelationshipComponent` scores) evolving based on repeated interactions or long-term consequences
  of their actions? (e.g., an agent learns to avoid a dangerous area, or trusts a player more after repeated positive
  interactions).[^32][^33][^34][^35]
* **7. Individuality & Persona Shifts:** Do different agents exhibit distinct baseline behaviors and preferences (e.g.,
  some are always more cautious, others more social)? Do individual agents occasionally show noticeable shifts in their
  general "demeanor" or priorities (e.g., a "trader" becomes a "defender" when their village is attacked), making them
  feel like dynamic characters?[^6][^7][^8][^47]
* **8. Communication Fidelity (Asymmetry):** Is there a noticeable "gap" between an agent's internal state/intent and a
  player's interpretation, creating opportunities for emergent narrative, player choice, and potential
  misunderstandings? Does the player sometimes have to *infer* what the agent is "thinking" or "feeling" without
  explicit communication?[^42][^43][^45]

---

### Validation Framework

To ensure that the agents achieve the desired "human-like" feel and emergent social dynamics, a multi-faceted validation
approach is necessary.

* **Mermaid Diagram (Validation Loop):**

  ```mermaid
  graph TD
      A[AI Agent Systems] --> B{Gameplay Session}
      B --> C[Player Perception & Feedback]
      B --> D[Agent Behavior Logs]
      C --> E{Qualitative Analysis: Believability Surveys}
      D --> F{Quantitative Analysis: Emergent Patterns, Divergence Metrics}
      E & F --> G[Human-Likeness Checklist Evaluation]
      G --> H{Design Iteration & Tuning}
      H --> A
  ```
* **Validation Methods:**

  1. **Observational Playtesting (Blind Testing):**
     * **Method:** Conduct playtests where players interact with agents (and potentially a small number of actual
       human players disguised as NPCs, if ethical and feasible) without knowing which is which. Afterward, players
       complete surveys rating perceived "humanity," emotional depth, decision-making believability, and the overall
       coherence of agent behavior.
     * **Focus:** Subjective player experience and the "feeling" of interacting with another player, not a machine.
       Assess if emergent narratives are compelling.
  2. **Turing Test Variants (Targeted):**
     * **Method:** Design specific interaction scenarios (e.g., negotiation for resources, collaborative task, social
       conversation). Present players with identical scenarios, one involving an AI agent and one a human. Ask
       players to identify which is which and explain their reasoning.
     * **Focus:** Direct comparison of AI output with human expectations for specific social behaviors, particularly
       those involving communication and inference.
  3. **Emergent Narrative Analysis:**
     * **Method:** Implement robust logging of all agent interactions, decisions, emotional states, and persona
       shifts during long-duration play sessions. Use data analysis tools to identify emergent patterns in
       relationships (alliances, rivalries), group dynamics (factions, mob behavior), and unexpected outcomes that
       resemble human social drama.
     * **Focus:** Confirming that simple rules at the individual level lead to complex, meaningful social phenomena
       at the collective level[^50][^51].
  4. **Behavioral Divergence Metrics:**
     * **Method:** Establish a baseline of expected (even if biased or "irrational") human responses in various
       scenarios. Compare agent behavior against this baseline. Focus on identifying *interesting* deviations that
       enhance believability and emergent drama, rather than just errors (e.g., an agent's irrational panic leading
       to a new quest opportunity).
     * **Focus:** Validating the effectiveness of "selective fidelity" – ensuring the chosen simplifications and
       biases contribute positively to the "human-like" feel.
  5. **"What If" Scenario Testing (Simulated Stress Tests):**
     * **Method:** Introduce controlled extreme conditions into the simulation (e.g., sudden massive resource
       depletion, introduction of a powerful external threat, rapid shift in a key agent's persona) and observe how
       the collective agent society reorganizes, forms new relationships, or shifts priorities.
     * **Focus:** Testing the robustness and realism of emergent social dynamics under pressure, ensuring they don't
       break down into predictable or uninteresting patterns. This helps validate the `Stress-Driven Decisions` and
       `Dynamic Persona System`.

### FOOTNOTES

[^1]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf
    In these pages, this research is approached from several directions. We look at the conditions that influence illusions of the experience of will— the cases when people feel they are willing an act that they in fact are not doing, or when they feel they are not willing an act that they indeed are doing. We explore conscious will in settings such as hypnosis, Ouija board spelling, automatic writing, and facilitated communication. We examine, too, such unusual phenomena as spirit possession, dissociative identity disorder, and trance channeling, to grasp some of the extreme transforma-tions of the experience of will. Psychological disorders—some caused by detectable brain damage and others, such as schizophrenia, by more sub-tle processes are examined also, to understand how the experience of con-scious will is modified in these conditions. The goal of this book is to put conscious will into perspective as a topic of psychological study. To do this, we need to understand how conscious will might be an illusion, a feeling that comes and goes independent of any actual causal relationship between our thoughts and our actions.
[^2]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf
    2 Chapter 1 - 2. Calling this an illusion may be a bit strong, and it might be more appropriate to think of this as a construction or fabrication. But the term illusion does convey the possibility that we place an erroneously large emphasis on how will appears to us and assume that this appearance is a deep insight. just to say that you decided to pick up the book and begin reading. You consciously willed what you are doing. These two explanations are both appealing but in different ways. The scientific explanation accounts for behavior as a mechanism and appeals to that part of us that knows how useful science is for understanding the world. It would be wonderful if we could understand people in just the same way. The conscious will explanation, on the other hand, has a much deeper grip on our intuition. We each have a profound sense that we con-sciously will much of what we do, and we experience ourselves willing our actions many times a day. As William James put it, “The whole sting and excitement of our voluntary life . . . depends on our sense that in it things are really being decided from one moment to another, and that it is not the dull rattling off of a chain that was forged innumerable ages ago” (1890, 453). Quite apart from any resentment we might feel on being cast into the role of mechanisms or robots, we appreciate the notion of con-scious will because we experience it so very acutely. We do things, and when we do them, we experience the action in such a way that it seems to flow seamlessly from our consciousness. We feel that we cause ourselves to behave. 
[^3]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf
    The research to date on the anatomy of conscious will, taken as a whole, suggests that there are multiple sources of this feeling. It appears that a person can derive the feeling of doing from conscious thoughts about what will be done, from feedback from muscles that have carried out the action, and even from visual perception of the action in the ab-sence of such thoughts or feedback. The research on phantom limbs in particular suggests that experience of conscious will arises from a re-markably flexible array of such indicators and can be misled by one when others are unreliable. The brain, in turn, shows evidence that the motor structures underlying action are distinct from the structures that allow the experience of will. The experience of will may be manufactured by the interconnected operation of multiple brain systems, and these do not seem to be the same as the systems that yield action. 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
[^4]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    Virtual Agency 223
    psychology (Acocella 1999; Gleaves 1996; Putnam 1989; Ross 1988; Spanos 1994).
    Most people faced with accounts of these anomalies of conscious agency have great difficulty imagining something like this happening to them. What would it be like to have a spirit take over your body? Where would “you” go at that time? If you had multiple personalities, what would it be like to switch between them? And if you leave one “self” and become another, what was the new one doing for your whole life until it just now popped into being? There are skeptics who reject these phe-nomena as faked (e.g., Aldridge-Morris 1989; Spanos 1994), perhaps in part because it seems so difficult to understand what it would be like to have this happen. Indeed, Aristotle’s theory of spirit possession in ancient Greece involved suspicion that the possessed were merely pretending, play-acting the role of a spirit as in the theater (Stoller 1995). We can’t appreciate how a subjective self might go away or how another might be created because we are such selves and can’t imagine going out of exis-tence and coming back as something else. To confront the topic of virtual authorship, then, is to try to understand how the very seat of human agency can be transformed (Matthews 1998; Wilkes 1988). The standard one person/one agent equation is seriously challenged, and we are vexed by the question of what it is like to be the person who changes subjective senses of self in this way. How could Bergen play the dummy and yet think that the dummy was someone else?
[^5]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    Virtual Agency 255
    12. Dennett (1992) proposes a similar concept of the agent “self as the center of narrative gravity.”
    how they evaluate themselves, and how these judgments of self influence behavior. There are very few accounts, however, of how the subjective self—the knower—is constituted. The notion of virtual agency is all about this and so opens up a new way of theorizing about an aspect of self that has previously been something of a mystery.
    If it weren’t for strange things like spirit possession and channeling, we might never have noticed that the subjective self can fluctuate and change over time. It is only by virtue of the ways in which people step outside themselves—to see the world from new perspectives, as different agents, with radically different points of view—that it becomes possible to recognize what an exciting and odd thing it is to have a subjective self at all. The subjective self each of us inhabits, in this light, is just one virtual agent of many possible virtual agents. Admittedly, the normal person’s point of view is pretty stable; it takes a lot of drumming and be-lieving and channeling lessons and who knows what else to get people to switch identities through the methods we’ve investigated so far. Yet such switching makes it possible to imagine that the self is not too different from a spirit—an imagined agent that is treated as real and that has real consequences for the person’s behavior and experience.12

[^6]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    The Self as Operating System
    It is probably time now to trot out the inevitable computer metaphor. Virtual agents seem a lot like software shells or operating systems on computers—different versions of Windows or Linux or Mac OS. On their face, virtual agents bear an important likeness to the interface of the operating system, the look and feel of the computer. An operating system or shell creates a virtual world, an array of what is possible with the ma-chine and what is easily remembered that sits there on the screen and creates the way in which the world is experienced. The virtual agent we experience at any given time similarly produces our experience of the world for us.

[^7]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    Virtual Agency 269
    computer from day to day. Some machines can boot up in Windows and also boot up in Linux, for example. What seems to happen in people who experience possession, channeling, and DID is highly reminiscent of re-booting the machine and bringing up a changed operating system after the boot. All the different operating systems work the “body”—they each give access to disk drives and printer ports and the like—but they each provide different “front ends” for the machine.
    This way of viewing virtual agency is a nice metaphor without much scientific implication. It is not clear how we would go about testing this metaphor, for example, or what it predicts beyond some of the basic resemblances between people and computers. The operating system/ self link does highlight, however, how the subjective self might be a construction—a replaceable set of ways in which things seem to the per-son—rather than an etched-in-stone feature of the structure of the mind. The point of this chapter has been establishing whether and under what conditions people might change the sense of self they experience as they act. We’ve seen any number of such transformations and have reached the point now of being able to step back and view the system as a whole. And, yes, it does look remarkably like a computer with interchangeable operating systems.

[^8]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    The conclusion we reach is that virtual agents can vary within each person, and perhaps more broadly, that there is generally a virtual agent for each person. The sense of having a conscious mind that experiences and chooses and acts is a basic feature of being human. But the fact that this perspective can change with vagaries of memory and experience sug-gests that this is a basic component that must be added onto the hard-ware of our brains and mental mechanisms for us to exist in anything like the way we now know ourselves. The development of an agent self in human beings is a process that overlays the experience of being human on an undercarriage of brain and nerve connections. We achieve the fact of having a perspective and being a conscious agent by appreciating the general idea of agents overall and then by constructing a virtual agent in which we can reside (see Attneave 1959).
      
[^9]: Consciousness-Explained.pdf  
    14
    CONSCIOUSNESS
    IMAGINED
    1. IMAGINING A CONSCIOUS ROBOT
    The phenomena of human consciousness have been explained in the preceding chapters in terms of the operations of a virtual machine," a sort of evolved (and evolving) computer program that shapes the activities of the brain. There is no Cartesian Theater; there are just Multiple Drafts composed by processes of content fixation playing var-
    ious semi-independent roles in the brain's larger economy of controlling a human body's Journey through life. The astonishingly persistent con- viction that there is a Cartesian Theater is the result of a variety of

[^10]: Consciousness-Explained.pdf 
    cognitive illusions that have now been exposed and explained. "Qualia" have been replaced by complex dispositional states of the brain, and the self (otherwise known as the Audience in the Cartesian Theater, the Central Meaner, or the Witness) turns out to be a valuable abstraction, a theorist's fiction rather than an internal observer or boss.
    if the self is "just" the Center of Narrative Gravity, and if all the phenomena of human consciousness are explicable as "just" the activ- ities of a virtual machine realized in the astronomically adjustable con- nections of a human brain, then, in principle, a suitably "programmed" robot, with a silicon-based computer brain, would be conscious, would have a self. More aptly, there would be a conscious self whose body was the robot and whose brain was the computer. This implication of

[^11]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    Virtual Agency 241
    although play-acted, may reveal surprising levels of perceptual detail about the virtual agent (“I didn’t know I could do that accent”) and may have unexpected emotional impact (“These jokes are actually funny”). This could promote the inference that the virtual agent is real.
    With the development of the virtual agent comes a transition in the locus of the control experience, as the agent’s thoughts begin to predict the body’s action better than the self’s thoughts do. Successful channeling seems to follow from the continued observation that the body’s acts are previewed by thoughts that are being (actively) attributed to the virtual agent. The acts seem uncontrollable by self, not necessarily because the self is even trying to control them but because the virtual agent’s thoughts seem to have this control. Thoughts coming to mind in the virtual agent’s accent, for example, may preview the body’s actions again and again. The misattribution of actions to the virtual agent continually deflates the sense of the self’s conscious will for the body’s actions. When the person is “in” the agent, the body’s actions seem controllable by that agent, and the person’s normal self seems less in control because thoughts attributed to self are simply not occurring as often.
       
[^12]: Consciousness-Explained.pdf 
    The structure of this party game bears a striking resemblance to the structure of a family of well-regarded models of perceptual systems.
    5. Empirical testing suggests that the game is more likely to produce a good story if in fact you favor affirmative answers slightly, by making plq the alphabetic dividing line between yes and no.
    12 PRELUDE: HOW ARE HALLUCINATIONS POSSIBLE?
    It is widely held that human vision, for instance, cannot be explained as an entirely "data-driven" or "bottom-up" process, but needs, at the highest levels, to be supplemented by a few "expectation-driven" rounds of hypothesis testing (Or something analogous to hypothesis testing). Another member of the family is the "analysis-by-synthesis" model of perception that also supposes that perceptions are built up in a process that weaves back and forth between centrally generated expectations, on the one hand, and confirmations (and disconfirma- tions) arising from the periphery on the other hand (e.g., Neisser, 1967). The general idea of these theories is that after a certain amount of "preprocessing" has occurred in the early or peripheral layers of the perceptual system, the tasks of perception are completed — objects are identified, recognized, categorized — by generate-and-test cycles. In such a cycle, one's current expectations and interests shape hypotheses for one's perceptual systems to confirm or disconfirm, and a rapid sequence of such hypothesis generations and confirmations produces the ultimate product, the ongoing, updated "model" of the world of the perceiver. Such accounts of perception are motivated by a variety of considerations, both biological and epistemological, and while I

[^13]: Consciousness-Explained.pdf  
    The structure of this party game bears a striking resemblance to the structure of a family of well-regarded models of perceptual systems.
    5. Empirical testing suggests that the game is more likely to produce a good story if in fact you favor affirmative answers slightly, by making plq the alphabetic dividing line between yes and no.
    12 PRELUDE: HOW ARE HALLUCINATIONS POSSIBLE?
    It is widely held that human vision, for instance, cannot be explained as an entirely "data-driven" or "bottom-up" process, but needs, at the highest levels, to be supplemented by a few "expectation-driven" rounds of hypothesis testing (Or something analogous to hypothesis testing). Another member of the family is the "analysis-by-synthesis" model of perception that also supposes that perceptions are built up in a process that weaves back and forth between centrally generated expectations, on the one hand, and confirmations (and disconfirma- tions) arising from the periphery on the other hand (e.g., Neisser, 1967). The general idea of these theories is that after a certain amount of "preprocessing" has occurred in the early or peripheral layers of the perceptual system, the tasks of perception are completed — objects are identified, recognized, categorized — by generate-and-test cycles. In such a cycle, one's current expectations and interests shape hypotheses for one's perceptual systems to confirm or disconfirm, and a rapid sequence of such hypothesis generations and confirmations produces the ultimate product, the ongoing, updated "model" of the world of the perceiver. Such accounts of perception are motivated by a variety of considerations, both biological and epistemological, and while I
       
[^14]: Consciousness-Explained.pdf 
    wouldn't say that any such model has been proven, experiments in- spired by the approach have borne up well. Some theorists have been so bold as to claim that perception must have this fundamental struc- ture.
    Whatever the ultimate verdict turns out to be on generate-and-test theories of perception, we can see that they support a simple and pow- erful account of hallucination. All we need suppose must happen for an otherwise normal perceptual system to be thrown into a halluci- natory mode is for the hypothesis-generation side of the cycle (the expectation-driven side) to operate normally, while the data-driven side of the cycle (the confirmation side) goes into a disordered or random or arbitrary round of confirmation and disconfirmation, just as in the party game. In other words, if noise in the data channel is arbitrarily amplified into "confirmations" and "disconfirmations" (the arbitrary yes and no answers in the party game), the current expectations, con- cerns, obsessions, and worries of the victim will lead to framing ques- tions or hypotheses whose content is guaranteed to reflect those interests, and so a "story" will unfold in the perceptual system without an author. We don't have to suppose the story is written in advance; we don't have to suppose that information is stored or composed in the illusionist part of the brain. All we suppose is that the illusionist

[^15]: Consciousness-Explained.pdf 
    wouldn't say that any such model has been proven, experiments in- spired by the approach have borne up well. Some theorists have been so bold as to claim that perception must have this fundamental struc- ture.
    Whatever the ultimate verdict turns out to be on generate-and-test theories of perception, we can see that they support a simple and pow- erful account of hallucination. All we need suppose must happen for an otherwise normal perceptual system to be thrown into a halluci- natory mode is for the hypothesis-generation side of the cycle (the expectation-driven side) to operate normally, while the data-driven side of the cycle (the confirmation side) goes into a disordered or random or arbitrary round of confirmation and disconfirmation, just as in the party game. In other words, if noise in the data channel is arbitrarily amplified into "confirmations" and "disconfirmations" (the arbitrary yes and no answers in the party game), the current expectations, con- cerns, obsessions, and worries of the victim will lead to framing ques- tions or hypotheses whose content is guaranteed to reflect those interests, and so a "story" will unfold in the perceptual system without an author. We don't have to suppose the story is written in advance; we don't have to suppose that information is stored or composed in the illusionist part of the brain. All we suppose is that the illusionist
       
[^16]: fuzzy_weighting_flows(UNFINISHED).md 
    Micro-detailed Flows with Fuzzy Weighting and Process Competition
    1. Sensory-Cognitive Flow with Dynamic Weighting
    1.1 Sensory Input and Processing
    • 1.1.1 Multimodal Sensory Transduction
        ◦ Visual stimuli → Retinal processing → Visual pathways (weight: 0.2-0.6, context-dependent)
        ◦ Auditory stimuli → Cochlear processing → Auditory pathways (weight: 0.1-0.5, context-dependent)
        ◦ Interoceptive stimuli → Visceral signals → Homeostatic pathways (weight: 0.1-0.7, need-dependent)
        ◦ Integration: Weighted sum with competition for salience (non-linear)
    • 1.1.2 Adaptive Attentional Filtering
        ◦ Bottom-up salience (weight: 0.3-0.8, stimulus-dependent)
        ◦ Top-down relevance (weight: 0.2-0.7, goal-dependent)
        ◦ Contextual novelty (weight: 0.1-0.6, habituation-dependent)
        ◦ Mechanism: Neural competition with lateral inhibition and recurrent feedback
    • 1.1.3 Probabilistic Perceptual Categorization
        ◦ Pattern recognition (confidence: 60-95%, experience-dependent)
        ◦ Activation of prior schemas (strength: 0.4-0.9, familiarity-dependent)
        ◦ Predictive filling (rate: 0.2-0.6, ambiguity-dependent)
        ◦ Result: Probability distribution over possible interpretations
       
[^17]: Daniel Kahneman-Thinking, Fast and Slow .pdf 
    Overconfidence: As the WY SIATI rule implies, neither the quantity nor the quality of the evidence counts for much in subjective confidence. The confidence that individuals have in their beliefs depends mostly on the quality of the story they can tell about what they see, even if they see little. We often fail to allow for the possibility that evidence that should be critical to our judgment is missing—what we see is all there is. Furthermore, our associative system tends to settle on a coherent pattern of activation and suppresses doubt and ambiguity. Framing effects: Different ways of presenting the same information often evoke different emotions. The statement that “the odds of survival one month after surgery are 90%” is more reassuring than the equivalent statement that “mortality within one month of surgery is 10%.” Similarly, cold cuts described as “90% fat-free” are more attractive than when they are described as “10% fat.” The equivalence of the alternative formulations is transparent, but an individual normally sees only one formulation, and what she sees is all there is. Base-rate neglect: Recall Steve, the meek and tidy soul who is often believed to be a librarian. The personality description is salient and vivid, and although you surely know that there are more male farm mu Base-rers than male librarians, that statistical fact almost certainly did not come to your mind when you first considered the question. What you saw was all there was.
       
[^18]: A Science of life without free will.pdf 
    For various reasons, humans were sculpted by evolution over millions of years to be, on the average, more aggressive than bonobos but less so than chimps, more social than orangutans but less so than baboons, more monogamous than mouse lemurs but more polygamous than marmosets. ’Nuff said.[57]
    SEAMLESS
    Where does intent come from? What makes us who we are at any given minute? What came before.[*] This raises an immensely important point first brought up in chapter 1, which is that the biology/environment interactions of, say, a minute ago and a decade ago are not separate entities. Suppose we are considering the genes someone inherited, back when they were a fertilized egg, and what those genes have to do with that person’s behavior. Well then, we are being geneticists thinking about genetics. We could even make our club more exclusive and be “behavior geneticists,” publishing our research only in a journal called, well, Behavior Genetics. But if we are talking about the genes inherited that are relevant to the person’s behavior, we’re automatically also talking about how the person’s brain was constructed—because brain construction is primarily carried out by the proteins coded for by “genes implicated in neurodevelopment.” Similarly, if we are studying the effects of childhood adversity on adult
       
[^19]: No Free Will: Biology of Behavior with Robert Sapolsky (Deep Summary) | Sloww 
    The ‘no free will’ argument in a nutshell:
    “I don’t think we have a shred of free will—despite 95% of philosophers and probably the majority of neuroscientists … The reason for this is you do something—you behave, you make a choice, whatever—and to understand why you did that and where that intention came from: · Part of it was due to the sensory environment you were in the previous minute. · Some of it is from the hormone levels in your bloodstream that morning. · Some of it is from whether you had a wonderful or stressful last three months and what sort of neuroplasticity happened. · Part of it is what hormone levels you were exposed to as a fetus. · Part of it is what culture your ancestors came up with and thus how you were parented when you were a kid. All of those are in there, and you can’t understand where behavior is coming from without incorporating all of those. And, at that point, not only are all of these relevant factors, but they are ultimately all one factor: · If you’re talking about what evolution has to do with your behavior, by definition you’re also talking about genetics. · If you’re talking about what your genes have to do with behavior, by definition you’re talking about how your brain was constructed or what proteins are coded for. · If you’re talking about your mood disorder now, you’re talking about the sense of efficacy you were getting as a five-year-old. They’re all intertwined. And when you look at all those influences, basically, the challenge is: show me a neuron that just caused that behavior (or show me a network of neurons that just caused that behavior), and show me that nothing about what they just did was influenced by anything from the sensory environment one second ago to the evolution of your species. And, there’s no space in there to fit in a free will concept that winds up being in your brain but not of your brain. There’s simply no wiggle room for it there.”
  
[^20]: fuzzy_weighting_flows(UNFINISHED).md 
    2.2.2 Competition Between Parallel Decision Systems
    • 2.2.1 Habitual System (Model-Free)
       ◦ Learned stimulus-response associations (strength: 0.3-0.9, repetition-dependent)
       ◦ Contextual automaticity (weight: 0.2-0.8, familiarity-dependent)
       ◦ Computational efficiency (advantage: 0.1-0.5, complexity-dependent)
       ◦ Dynamics: Fast activation with low cognitive cost
    • 2.2.2 Deliberative System (Model-Based)
       ◦ Simulation of consequences (precision: 0.4-0.8, knowledge-dependent)
       ◦ Evaluation of multiple goals (weight: 0.3-0.7, conflict-dependent)
       ◦ Adaptive flexibility (advantage: 0.2-0.6, novelty-dependent)
       ◦ Dynamics: Slow activation with high cognitive cost
    • 2.2.3 Emotional-Intuitive System (Heuristic)
       ◦ Somatic markers (intensity: 0.3-0.8, prior experience-dependent)
       ◦ Moral/social intuitions (strength: 0.2-0.7, socialization-dependent)
       ◦ Fast pattern processing (efficacy: 0.3-0.7, expertise-dependent)
       ◦ Dynamics: Intermediate activation with moderate cognitive cost
    • 2.2.4 Arbitration Between Systems
       ◦ Availability of cognitive resources (weight: 0.2-0.7, cognitive load-dependent)
       ◦ Confidence in each system (weight: 0.3-0.8, prior success-dependent)
       ◦ Time pressure (modulator: 0.5-1.5, urgency-dependent)
       ◦ Mechanism: Dynamic competition with mutual inhibition and contextual dominance

[^21]: Daniel Kahneman-Thinking, Fast and Slow .pdf 
    of bad consequences is a disastrous flaw.
    In a compelling demonstration of the workings of the affect heuristic, Slovic’s research team surveyed opinions about various technologies, including water fluoridation, chemical plants, food preservatives, and cars, and asked their respondents to list both the benefits >
    The best part of the experiment came next. After completing the initial survey, the respondents read brief passages with arguments in favor of various technologies. Some were given arguments that focused on the numerous benefits of a technology; others, arguments that stressed the low risks. These messages were effective in changing the emotional appeal of the technologies. The striking finding was that people who had received a message extolling the benefits of a technology also changed their beliefs about its risks. Although they had received no relevant evidence, the technology they now liked more than before was also perceived as less risky. Similarly, respondents who were told only that the risks of a technology were mild developed a more favorable view of its benefits. The implication is clear: as the psychologist Jonathan Haidt said in another context, “The emotional tail wags the rational dog.” The affect heuristic simplifies our lives by creating a world that is much tidier than reality. Good technologies have few costs in the imaginary world we inhabit, bad technologies have no benefits, and all decisions are easy. In the real world, of course, we often face painful tradeoffs between benefits and costs.
       
[^22]: psychological.md 
    Psychological Foundations for Social Simulation
    Cognitive and Behavioral Processes
    Motivation and Needs
    • Hierarchy of Needs (Maslow): physiological, safety, belonging, esteem, self-actualization
    • Self-Determination Theory: needs for autonomy, competence, and relatedness
    • Intrinsic vs. Extrinsic Motivation: internal or external origin of the drive to act
    • Homeostasis: tendency to maintain internal balance
    Emotions and Affect
    • Basic Emotions: joy, sadness, fear, anger, disgust, surprise
    • Emotional Regulation: ability to modulate emotional responses
    • Emotional Contagion: transfer of emotional states between individuals
    • Valence and Arousal: fundamental dimensions of emotional experience
       
[^23]: A Science of life without free will.pdf 
    And then the PFC does the harder thing. In most of those subjects, a few seconds after the amygdala activates, the PFC kicks in, turning off the amygdala. It’s a delayed frontocortical voice—“Don’t think that way. That’s not who I am.” And who are the folks in which the PFC doesn’t muzzle the amygdala? People whose racism is avowedly, unapologetically explicit —“That is who I am.”[13]
    In another experimental paradigm, a subject in a brain scanner plays an online game with two other people—each is represented by a symbol on the screen, forming a triangle. They toss a virtual ball around—the subject presses one of two buttons, determining which of the two symbols the ball is tossed to; the other two toss it to each other, toss it back to the subject. This goes on for a while, everyone having a fine time, and then, oh no, the other two people stop tossing the ball to the subject. It’s the middle-school nightmare: “They know I’m a dork.” The amygdala rapidly activates, along with the insular cortex, a region associated with disgust and distress. And then, after a delay, the PFC inhibits these other regions—“Get this in perspective; this is just a stupid game.” In a subset of individuals, however, the PFC doesn’t activate as much, and the amygdala and insular cortex just keep going, as the subject feels more subjective distress. Who are these impaired individuals? Teenagers—the PFC isn’t up to the task yet of dismissing social ostracism as meaningless. There you have it.[*], [14]
       
[^24]: dokumen.pub_determined-a-science-of-life-without-free-will-9780525560975-9780525560982-9780593656723.pdf 
    *Here are some factoids that emphasize the extent to which social demands sculpt the evolution of the PFC. The PFC contains a neuron type not found elsewhere in the brain. To add to its coolness, for a while people thought that these “von Economo neurons,” introduced in the footnote on page 61, occurred only in humans. But as something even cooler, the neurons also occur in the most socially complex species out there—other apes, elephants, cetaceans. A neurological disease called behavioral frontotemporal dementia demonstrates that PFC damage causes inappropriate social behavior. What are the first neurons that die in that disease? The von Economo neurons. So whatever they do (which isn’t at all clear), it has “doing the harder thing” written all over it. (Brief screed of interest to only a few readers—despite quasi–New Age neuroscientific claims, von Economo neurons are not mirror neurons responsible for empathy. These aren’t mirror neurons. And mirror neurons don’t do empathy. Don’t get me started.)
       
[^25]: A Survey of Trust and Reputation Systems for.pdf 
    These simple principles invite rigorous research in order to answer some fundamen-tal questions: What information elements are most suitable for deriving measures of trust and reputation in a given application? How can theseinformation elements be captured and collected? What are the best principles for designing such systems from a theoretic and from a usability point of view? Can they bemade resistant to attacks of manipulation by strategic agents? How should users include the in-formation provided by such systems into their decision process? What role can these systems play in the business model of commercial companies? Do these sys-tems truly improve the quality of online trade and interactions? These are important questions that need good answers in order to determine the poential for trust and reputation systems in online environments.

[^26]: A Survey of Trust and Reputation Systems for.pdf 
    According to Resnicket al. [56], reputation systems must have the following three properties to operate at all:
    1. Entities must be long lived, so that with every interaction there is always an expectation of future interactions.
    2. Ratings about current interactions are captured and distributed.
    3. Ratings about past interactions must guide decisions about current interactions.
       The longevity of agents means, for example, that it should beimpossible or diffi-cult for an agent to change identity or pseudonym for the purpose of erasing the connection to its past behaviour. The second property depends o the protocol with which ratings are provided, and this is usually not a problemfor centralised sys-tems, but represents a major challenge for distributed systems. The second property also depends on the willingness of participants to provide ratings, for which there must be some form of incentive. The third property depends onthe usability of rep-utation system, and how people and systems respond to it, andhis is reflected in the commercial and live reputation systems described in Sec.9, but only to a small extent in the theoretic proposals described in Sec.8 and Sec.10.
       
[^27]: jib2007-dss.pdf
    These simple principles invite rigorous research in order to answer some fundamen-tal questions: What information elements are most suitable for deriving measures of trust and reputation in a given application? How can theseinformation elements be captured and collected? What are the best principles for designing such systems from a theoretic and from a usability point of view? Can they bemade resistant to attacks of manipulation by strategic agents? How should users include the in-formation provided by such systems into their decision process? What role can these systems play in the business model of commercial companies? Do these sys-tems truly improve the quality of online trade and interactions? These are important questions that need good answers in order to determine the poential for trust and reputation systems in online environments.
       
[^28]: jib2007-dss.pdf 
    According to Resnicket al. [56], reputation systems must have the following three
    6
    properties to operate at all:
    1. Entities must be long lived, so that with every interaction there is always an expectation of future interactions.
    2. Ratings about current interactions are captured and distributed. 
    3. Ratings about past interactions must guide decisions about current interactions.
         The longevity of agents means, for example, that it should beimpossible or diffi-cult for an agent to change identity or pseudonym for the purpose of erasing the connection to its past behaviour. The second property depends o the protocol with which ratings are provided, and this is usually not a problemfor centralised sys-tems, but represents a major challenge for distributed systems. The second property also depends on the willingness of participants to provide ratings, for which there must be some form of incentive. The third property depends onthe usability of rep-utation system, and how people and systems respond to it, andhis is reflected in the commercial and live reputation systems described in Sec.9, but only to a small extent in the theoretic proposals described in Sec.8 and Sec.10.
       
[^29]: fuzzy_weighting_flows(UNFINISHED).md 
    1.3 Memory Retrieval and Integration
       • 1.3.1 Associative Network Activation
           ◦ Perceptual similarity → Activation of episodic memories (strength: 0.3-0.8, specificity-dependent)
           ◦ Semantic relevance → Activation of declarative knowledge (strength: 0.2-0.7, expertise-dependent)
           ◦ Emotional congruence → Activation of affective memories (strength: 0.3-0.9, intensity-dependent)
           ◦ Propagation: Activation diffusion with exponential decay and resonance reinforcement
       • 1.3.2 Bayesian Predictive Integration
           ◦ Priors based on experience → Initial expectations (weight: 0.3-0.7, confidence-dependent)
           ◦ Current sensory evidence → Belief updating (weight: 0.2-0.8, precision-dependent)
           ◦ Contextual consistency → Narrative coherence (weight: 0.1-0.6, ambiguity-dependent)
           ◦ Calculation: Approximate Bayesian inference with stochastic sampling
       • 1.3.3 Probabilistic Prospective Simulation
           ◦ Projection of immediate consequences (precision: 0.5-0.8, familiarity-dependent)
           ◦ Projection of social consequences (precision: 0.4-0.7, theory of mind-dependent)
           ◦ Projection of long-term consequences (precision: 0.3-0.6, time horizon-dependent)
           ◦ Mechanism: Mental simulation with multiple parallel scenarios and plausibility weighting
       
[^30]: Daniel Kahneman-Thinking, Fast and Slow .pdf 
    Speaking of Priming
    “The sight of all these people in uniforms does not prime creativity.”
    “The world makes much less sense than you think. The coherence comes mostly from the way your mind works.”
    “They were primed to find flaws, and this is exactly what they found.”
    “His System 1 constructed a story, and his System 2 believed it. It happens to allel
    “I made myself smile and I’m actually feeling better!”
    Whenever you are conscious, and perhaps even when you are not, multiple computations are going on in your brain, which maintain and update current answers to some key questions: Is anything new going on? Is there a threat? Are things going well? Should my attention be redirected? Is more effort needed for this task? You can think of a cockpit, with a set of dials that indicate the current values of each of these essential variables. The assessments are carried out automatically by System 1, and one of their functions is to determine whether extra effort is required from System 2.
[^31]: MQL-NPC: A Modified Q-Learning-based Approach to Design Intelligent Non-Player Character in a Survival Game 
    learning-based scheme. Due to the dynamics of the computer game environment, reinforcement learning is employed
    to make this agent smart. This leads the agent to react appropriately based on the game’s scenario by choosing an
    action that provides a higher reward in the current situation. This is like a brain for the target NPC that processes
    different situations and reacts appropriately. Our intelligent agent is applied to a sample survival game with different
    complexity levels. In this game, multiple characters and objects alongside win-and-lose scenarios are considered.
[^32]: MQL-NPC: A Modified Q-Learning-based Approach to Design Intelligent Non-Player Character in a Survival Game 
    Our designed intelligent NPC is equipped with modified Q-learning to interact and try different actions on objects
    and learn about them. This learning process leads to an experience saved in the designed agent to react best to the
    environment. The efficiency of our proposed approach is evaluated through multiple scenarios and the appropriate
    reaction of the NPC is verified.
    keywords: Non-Player Character(NPC), Intelligent Agent, Survival Game, Reinforcement Learning, Dynamic Environment.
    I. INTRODUCTION
[^33]: MQL-NPC: A Modified Q-Learning-based Approach to Design Intelligent Non-Player Character in a Survival Game 
    and the environment based on the system’s state. So, an appropriate interaction is planned step by step by looking over
    the result through the effects of performed actions. In this context, related studies focus on applying intelligence to the
    NPCs, game levels, and player experiences [1], [4], [5].
    Our proposed method considers a reinforcement learning- based scheme to control the agent’s interaction. In
    reinforcement learning, one of the main goals of an agent is to learn how to interact with its environment. To this aim, it
[^34]: MQL-NPC: A Modified Q-Learning-based Approach to Design Intelligent Non-Player Character in a Survival Game 
    learning policy guided by the environment and its repetition teaches the agent to have appropriate reactions. We employ a
    quasi-Q-learning scheme for the designed intelligent agent of a survival game. The learning process starts with design of a
    state-oriented system to interact with different objects and save the actions’ results. This result is a dynamic reward system
    that is proposed to evaluate the appropriateness of the selected object considering the situation and time step. These steps
    are implemented through a modified Q-learning algorithm for each agent of the game. The main contributions of our
       
[^35]: fuzzy_weighting_flows(UNFINISHED).md
    3.2 Outcome Evaluation and Learning
       • 3.2.1 Multidimensional Success Evaluation
           ◦ Achievement of primary goal (weight: 0.4-0.8, importance-dependent)
           ◦ Resource efficiency (weight: 0.2-0.6, scarcity-dependent)
           ◦ Secondary consequences (weight: 0.1-0.5, scope-dependent)
           ◦ Calculation: Weighted average with dynamic weights based on values and context
       • 3.2.2 Feedback Processing
           ◦ Explicit external feedback (weight: 0.3-0.7, source credibility-dependent)
           ◦ Internal feedback based on expectations (weight: 0.2-0.6, confidence-dependent)
           ◦ Indirect social feedback (weight: 0.1-0.5, social relevance-dependent)
           ◦ Integration: Bayesian synthesis with weighting by reliability and relevance
       • 3.2.3 Internal Model Updating
           ◦ Adjustment of expectations (rate: 0.1-0.5, surprise-dependent)
           ◦ Strategy refinement (rate: 0.1-0.4, efficacy-dependent)
           ◦ Revision of values/priorities (rate: 0.05-0.3, emotional impact-dependent)
           ◦ Mechanism: Reinforcement learning with adaptive learning rates and multiple parallel systems
[^36]: 2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
    An Analysis of Automatism 133
    behaviors they perform that are consistent with those thoughts. Another set of people might specifically ignore or suppress certain areas of think-ing and thus fail to infer that any of their behaviors relevant to those areas might be willed. There may even be people who regularly fail to grasp the consistency between their thoughts and actions and thus expe-rience the actions as less than willed.
    A full analysis of the individual differences underlying automatism could yield a broad and complex range of deficits in the ability to process information about one’s own apparent causal influence. For this reason, it is important as a first step to establish the conditions under which vari-ous such deficits occur even in people with normal processing capacities. Suffice it to say at this point that the search for a general dissociation-prone personality is likely instead to unearth many individual differences that can contribute to such effects. There may not be such a thing as one type of odd person, in other words, so it may be easier to attack the problem of classifying the causes of dissociation by focusing on odd situations.
    2002 - The illusion of conscious will - Daniel M. Wegner.pdf 
[^37]: dokumen.pub_determined-a-science-of-life-without-free-will-9780525560975-9780525560982-9780593656723.pdf 
    Cognition and Fear in Humans,” Journal of Neuroscience 25 (2005): 11489; I. Labuschagne et al., “Oxytocin Attenuates Amygdala Reactivity to Fear in Generalized Social Anxiety Disorder,” Neuropsychopharmacology 35 (2010): 2403.
    Oxytocin blunting the stress-response: M. Heinrichs et al., “Social Support and Oxytocin Interact to Suppress Cortisol and Subjective Responses to Psychosocial Stress,” Biological Psychiatry 54 (2003): 1389.
    Oxytocin effects on empathy, trust, and cooperation: S. Rodrigues et al., “Oxytocin Receptor Genetic Variation Relates to Empathy and Stress Reactivity in Humans,” Proceedings of the National Academy of Sciences of the United States of America 106 (2009): 21437; M. Kosfeld et al., “Oxytocin Increases Trust in Humans,” Nature 435 (2005): 673; A. Damasio, “Brain Trust,” Nature 435 (2005): 571; S. Israel et al., “The Oxytocin Receptor (OXTR) Contributes to Prosocial Fund Allocations in the Dictator Game and the Social Value Orientations Task,” Public Library of Science One 4 (2009): e5535; P. Zak, R. Kurzban, and W. Matzner, “Oxytocin Is Associated with Human Trustworthiness,” Hormones and Behavior 48 (2005): 522; T. Baumgartner et al., “Oxytocin Shapes the Neural Circuitry of Trust and Trust Adaptation in Humans,” Neuron 58 (2008): 639; J. Filling et al., “Effects of Intranasal Oxytocin and Vasopressin on Cooperative Behavior and Associated Brain Activity in Men,” Psychoneuroendocrinology 37 (2012): 447; A. Theodoridou et al., “Oxytocin and Social Perception: Oxytocin Increases Perceived Facial Trustworthiness and Attractiveness,” Hormones and Behavior 56 (2009): 128. A failure of replication: C. Apicella et al., “No Association between Oxytocin Receptor (OXTR) Gene Polymorphisms and Experimentally Elicited Social Preferences,” Public Library of Science One 5 (2010): e11153.
       
[^38]: Consciousness-Explained.pdf 
    for some brute facts lacking all functional justification. Some features of consciousness may just be selfish memes.
    Looking on the bright side, however, what problems is this new machine apparently well designed to solve7 The psychologist Julian Jaynes (1976) has argued persuasively that its capacities for self-
    222 AN EMPIRICAL THEORY OF THE MIND
    exhortation and self-reminding are a prerequisite for the sorts of elab- orated and long-term bouts of self-control without which agriculture, building projects, and other civilized and civilizing activities could not be organized. It also seems to be gool for the sorts of self-monitoring that can protect a flawed system from being victimized by its own failures, a theme developed in Artificial Intelligence by Douglas Hof- stadter (1985). It has been seen by the psychologist Nicholas Humphrey (1976, 1983a, 1986) as providing a means for exploiting what might be called social simulations — using introspection to guide one's hunches about what others are thinking and

[^39]: Consciousness-Explained.pdf 
    them.
    my point: it is only by various combinations of physical replicas and schematization (a
    relatively coarse-grained representation) that robust Illusions can be sustained And even
    at their best, they are experiences of virtual surreality. not something that you might
    mistake for the real thing for more than a moment. If you really want to fool someone
    into thinking he is in a cage wfth a gorilla, enlisting the help of an actor in a gorilla suit
    is going to be your best bet for a long time.
    8 PRELUDE: HOW ARE HALLUCINATIONS POSSIBLE?
[^40]: Consciousness-Explained.pdf 
    4. FICTIONAL WORLDS AND HETEROPHENOMENOLOGICAL
    WORLDS
    In addition to the particular problems raised by strange cases, there rr seem to be a general problem. Doesn't the very practice of inter-
    verbal behavior in this way presuppose the consciousness of ne subject and hence beg the zombie question? Suppose you are con-
    fronted by a "speaking" computer, and suppose you succeed in inter- preting its output as speech acts expressing its beliefs and opinions, presumably "about" its conscious states. The fact that there is a single, coherent interpretation of a sequence of behavior doesn't establish that the interpretation is true; it might be only as if the "subject" were conscious; we risk being taken in by a zombie with no inner life at all. You could not confirm that the computer was conscious of anything by this method of interpretation. Fair enough. We can't be sure that the speech acts we observe express real beliefs about actual experiences; perhaps they express only apparent beliefs about nonexistent experi- ences. Still, the fact that we had found even one stable interpretation of some entity's behavior as speech acts would always be a fact worthy of attention. Anyone who found an intersubjectively uniform way of interpreting the waving of a tree's branches in the breeze as "commen- taries" by "the weather" on current political events would have found something wonderful demanding an explanation, even if it turned out to be effects of an ingenious device created by some prankish en- gineer.
       
[^41]: Consciousness-Explained.pdf 
    A METHOD FOR PHENOMENOLOGY 81
    Now let's apply the analogy to the problem facing the experi- menter who wants to interpret the texts produced by subjects, with- out begging any questions about whether his subjects are zombies, computers, lying, or confused. Consider the advantages of adopting the tactic of interpreting these texts as fictions of a sort, not as litera- ture of course, but as generators of a theorist's fiction (which might, of course, prove to be true after all). The reader of a novel lets the text constitute a (fictional) world, a world determined by fiat by the text, exhaustively extrapolated as far as extrapolation will go and indeter- minate beyond; our experimenter, the heterophenomenologist, lets the subject's text constitute that subject's heterophenomenological world, a world determined by fiat by the text (as interpreted) and indeter- minate beyond. This permits the heterophenomenologist to postpone the knotty problems about what the relation might be between that (fictional) world and the real world. This permits theorists to agree in detail about just what a subject's heterophenomenological world is, while offering entirely different accounts of how heterophenomenol- ogical worlds map onto events in the brain (or the soul, for that mat- ter). The subject's heterophenomenological world will be a stable, intersubjectively confirmable theoretical posit, having the same me- taphysical status as, say, Sherlock Holmes's London or the world ac- cording to Garp.
       
[^42]: Consciousness-Explained.pdf 
    To sum up, subjects are unwitting creators of fiction, but to say that they are unwitting is to grant that what they say is, or can be, an account of exactly how it seems to them. They tell us what it is like to them to solve the problem, make the decision, recognize the object. Because they are sincere (apparently), we grant that that must be what it is like to them, but then it follows that what it is like to them is at best an uncertain guide to what is going on in them. Sometimes, the unwitting fictions we subjects create can be shown to be true after all, if we allow for some metaphorical slack as we did with Shakey's answer in style (2). For instance, recent research on imagery by cognitive psy- chologists shows that our introspective claims about the mental images we enjoy (whether of purple cows or pyramids) are not utterly false (Shepard and Cooper, 1982; Kosslyn, 1980; Kosslyn, Holtzman, Gaz- zaniga, and Farah, 1985). This will be discussed in more detail in chapter 10, and we will see how our introspective reports of imagery can be interpreted so they come out true. Like the earthly Feenoman, however, who turns out not to be able to fly or be in two places at once, the actual things we find in the brain to identih' as the mental images will not have all the wonderful properties subjects have confidently endowed their images with. Shakey's "images" provide an example of how something that really wasn't an image at all could be the very thing someone was talking about under the guise of an image. While the processes in the brain underlying human imagery are probably not
[^43]: Consciousness-Explained.pdf 
    390 THE PHILOSOPHICAL PROBLEMS OF CONSCIOUSNESS
    public things "red" and 'green" even if our private experiences were "the opposite" (or just different).
    Is there any way to tell whether this is the case? Consider the hypothesis that red things look the same to you and me. Is this hy- pothesis both irrefutable and unconfirmable? Many have thought so, and some have concluded that for just that reason it is one sort of nonsense or another, in spite of its initial appeal to common sense. Others have wondered if technology might come to the rescue and confirm (or disconfirm) the interpersonal inverted spectrum hypothesis. The science-fiction movie Brainstorm (not, I hasten to say, a version of my book Brainstorms) featured just the right imaginary device: Some neuroscientific apparatus fits on your head and feeds your visual ex- perience into my brain via a cable. With eyes closed I accurately report everything you are looking at, except that I marvel at how the sky is yellow, the grass red, and so forth. If we had such a machine, couldn't such an experiment with it confirm, empirically, the hypothesis that our qualia were different? But suppose the technician pulls the plug on the connecting cable, inverts it 180 degrees, reinserts it in the socket, and I now report the sky is blue, the grass green, and so forth. Which would be the "right" orientation of the plug? Designing and building such a device — supposing for the moment that it would be possible — would require that its "fidelity" be tuned or calibrated by the normal- ization of the two subjects' reports, so we would be right back at our evidential starting point. Now one might try to avert this conclusion with further elaborations, but the consensus among the qualophiles is that this is a lost cause; there seems to be general agreement that the moral of this thought experiment is that no intersubjective comparison of qualia would be possible, even with perfect technology. This does provide support, however, for the shockingly 'verificationist" or "pos- itivistic" view that the very idea of inverted qualia is nonsense — and hence that the very idea of qualia is nonsense. As the philosopher Ludwig Wittgenstem put using his famous "beetle in the box" analogy,
[^44]: Consciousness-Explained.pdf 
    Question-begging or not, it may still seem just plain obvious that. "the subjective colors you would be seeing things to be" would have to be "one way or the other." This just shows the powerful gravitational force that the Cartesian Theater exerts on our imaginations. It may help to break down the residual attractiveness of this idea if we consider further the invited parallel with image-inverting goggles. When the adaptations of the subjects wearing these goggles have become so sec- ond nature that they can ride bicycles and ski, the natural (but mis- guided) question to ask is this: Have they adapted by turning their experiential world back right side up, or by getting used to their ex- periential world being upside down? And what do they say? They say different things, which correlate roughly with how complete their ad- aptation was. The more complete it was, the more the subjects dismiss the question as improper or unanswerable. This is just what the Mul- tiple Drafts theory demands: Since there are a host of discriminations and reactions that need to be adjusted, scattered around in the brain, some of them dealing with low-level "reflexes" (such as ducking the right way when something looms at you) and others dealing with focally attended deliberate actions, it is not suiprising that as the adaptations in this patchwork accumulate, subjects should lose all conviction of whether to say "things look the way they used to look" instead of "things still look different, but I'm getting used to it." In some ways things look the same to them (as judged by their reactions), in other ways things look different (as judged by other reactions). If there were a single representation of visuo-motor space through which all reactions to visual stimuli had to be channeled, it would have to be "one way or the other," perhaps, but there is no such single representation. The way things look to them is composed of many partly independent habits of reaction, not a single intrinsically right-side-up or upside-down pic- tuie in the head. All that matters is the fit between the input and the       
[^45]: Consciousness-Explained.pdf 
    For most computer-users, it is only in terms of these metaphors that they have any appreciation of what is happening inside. This is one of the facts that makes a virtual machine such a tempting analogy for consciousness, for it has always seemed that our access to what is happening inside our own brains is limited; we don't have to know how the backstage machinery of our brains does its magic; we are ac- quainted with its operations only as they come clothed for us in the interactive metaphors of phenomenology. But if, when we avail our- selves of this tempting analogy, we maintain the "obvious" separation between Presentation on the one hand and User Appreciation of the show on the other, we seem to end up right back in the Cartesian Theater. How could there be a User illusion without this separation?
[^46]: An integrated theory of the mind por John RAnderson et al2004.pdf 
    The Goal Module
    Although human cognition is certainly embodied, its embodi-ment is not what gives human cognition its advantage over that of other species. Its advantage depends on its ability to achieve abstraction in content and control. Consider a person presented with the numbers 64 and 36. As far as the external stimulation is concerned, this presentation affords the individual a variety of actions—adding the numbers, subtracting them, dialing them on a phone, and so forth. Human ability to respond differently to these items depends on knowledge of what the current goal is and ability to sustain cognition in service of that goal without any change in the external environment. Suppose the goal is to add the numbers. Assuming that one does not already have the sum stored, one will have to go through a series of steps in coming up with the answer, and to do this, one has to keep one’s place in performing these steps and keep track of various partial results such as the sum of the tens digits. The goal module has this responsibility of keeping track of what these intentions are so that behavior will serve that goal. It enables people to keep the thread of their thought in the absence of supporting external stimuli.
[^47]: Multi-level Simulator for Artificial Societies - ROMJIST 
    Fig. 3. The distribution of the resources in the landscape.
    4.2. The agents
        The agents are separate entities men or women, with one of specific age: child, mature or old, with a specific power to work (bad, good or excellent) generated by the health status (bad, good or excellent) and the wealth (poor, medium or rich). The simulator was designed with a modular architecture in order to allow to develop the rules applied to agents or to include new features and new types of social behaviours. For instance, the simulator takes into account an exchange rate for resources exchanges with other agents. The values of the characteristics of the agents are distributed randomly: the “strongest” agents have high vision level and low metabolic rate, while the ones that have low vision level and high metabolic rate have practically few chances to survive. The exchange rate is the same for all the agents, and depends of
[^48]: What is involved in this practice of talking to subjects? It is an ineliminable element in psychological experiments, but does it pre- suppose the consciousness of the subjects? Don't experimenters then end up back with the Introspectionists, having to take a subject's un- testable word for what he or she understands? Don't we run some risk of being taken in by zombies or robots or other impostors?
    We must look more closely at the details of a generic human subject experiment. Suppose, as is often the case, that multiple record- ings are made of the entire experiment: videotape and sound tape, and electroencephalograph, and so forth. Nothing that is not thus recorded will we count as data. Let's focus on the recording of sounds — vocal sounds mainly — made by the subjects and experimenters during the experiment. Since the sounds made by the subjects are made by phys- ical means, they are in principle explainable and predictable by phys- ics, using the same principles, laws, models that we use to explain and predict automobile engine noises or thunderclaps. Or, since the sounds are made by physiological means, we could add the principles of phys- iology and attempt to explain the sounds using the resources of that science, just as we explain belches, snores, growling stomachs, and creaking joints. But the sounds we are primarily interested in, of course, are the vocal sounds, and more particularly the subset of them (ignoring the occasional burps, sneezes, and yawns) that are apparently amenable to a linguistic or semantic analysis. It is not always obvious just which sounds to include in this subset, but there is a way of playing it safe: we give copies of the tape recordings to three trained stenographers and have them independently prepare transcripts of the raw data.
    Consciousness-Explained.pdf 
[^49]: Consciousness-Explained.pdf 
    This simple step is freighted with implications; we move by it from one world — the world of mere physical sounds — into another: the world of words and meanings, syntax and semantics. This step (A METHOD FOR PHENOMENOLOGY 75) yields a radical reconstrual of the data, an abstraction from its acoustic and other physical properties to strings of words (though still adorned with precise timing — see, e.g., Ericsson and Simon, 1984). What gov- erns this reconstrual? Although there presumably are regular and dis- coverable relationships between the physical properties of the acoustic wave recorded on the tape and the phonemes that the typists hear and then further transcribe into words, we don't yet know enough about the relationships to describe them in detail. (If we did, the problem of making a machine that could take dictation would be solved. Although great progress has been made on this, there are still some major per- plexities.) Pending the completion of that research in acoustics and phonology, we can still trust our transcripts as objective renditions of the data so long as we take a few elementary precautions. First, having stenographers prepare the transcripts (instead of entrusting that job to the experimenter, for instance) guards against both willful and unwit- ting bias or overinterpretation. (Court stenographers fulfill the same neutral role.) Having three independent transcripts prepared gives us a measure of how objective the process is. Presumably, if the recording is good, the transcripts will agree word-for-word on all but a tiny frac- tion of one percent of the words. Wherever the transcripts disagree, we can simply throw out the data if we wish, or use agreement of two out of three transcripts to fix the transcript of record.
[^50]: Multi-level Simulator for Artificial Societies - ROMJIST 
    3. Artificial Societies – The Basic Model
    The concept of artificial societies is nowadays considered by scientists as a funda-mental computation paradigm, ingeniously combining the parallelism of computing methods and techniques that are specific for artificial intelligence, especially the pro-gramming based upon intelligent agents. The artificial societies allow the realisation of fascinating experiments regarding the evolution of societies of agents under differ-ent complex and dynamic conditions for modelling the social environment. Thus, it is estimated that one can model most of the social mechanisms, starting from formation and evolution of social classes and categories up to prediction with high precision of the election results, from creation and dissemination of rumours to identification of efficient measures to stop them, from generation of protest social movements to triggering and controlling the evolution of military conflagrations. Among the results already reported by the scientific literature, the most interesting are: trade modelling, modelling the migration phenomena, the adaptation to environment or the degrading of environment, evolution of prices, etc. 
[^51]: Multi-level Simulator for Artificial Societies - ROMJIST 
    The new model of artificial societies, as introduced by Epstein and Axtell [1], aims to simulate human societies and to study collective (social) phenomena from bottom up. Although this is not the first attempt of this kind, it is certainly the most elaborated and successful. In this approach fundamental social structures and group behaviors emerge from individuals operating in artificial environments. Both agents (individuals) and the landscape (environment) have simple local evolution laws, thus requiring only bounded demands on each agent’s information and computational capacity.