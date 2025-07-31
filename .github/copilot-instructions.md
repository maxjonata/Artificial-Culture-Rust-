# GitHub Copilot Custom Instructions for "Artificial Society"

## 1. Core Architectural Philosophy & Goal

**This is the most important section. All generated code must align with this philosophy.**

This project, "Artificial Society," is an agent-based AI simulation in Rust using the Bevy Engine. The goal is to model
complex, emergent social dynamics. The architecture is a synthesis of two key paradigms:

1. **The Agent's Internal World (Micro-level): A "Brain as a Society of Systems"**
    - **Inspiration:** Inspired by large-scale, neuro-computational models of the brain.
    - **Principle:** An agent's mind is **NOT** a single, monolithic decision-maker. It is a **society of competing and
      cooperating sub-systems** (e.g., physiological needs, emotional responses, rational planning).
    - **Implementation:** Behavior emerges from the interaction of these internal systems.
    - **Key Takeaway:** Think of each agent as a self-contained, complex adaptive system.

2. **The External World (Macro-level): A "Society of Agents"**
    - **Inspiration:** Inspired by academic Agent-Based Modeling (ABM).
    - **Principle:** Complex, large-scale social phenomena **emerge** from simple, local interactions between many
      individual agents. The global outcome is **not** scripted.
    - **Key Takeaway:** The world is a bottom-up simulation. We build the ants, not the anthill.

**The Grand Challenge:** The core engineering task of this project is to **bridge these two paradigms**. We must create
the "API" that translates the macro-level social and environmental pressures into micro-level neuro-cognitive inputs for
each agent, and vice-versa.

---

**Five Core Technical Principles (Non-negotiable):**

1. **The Mantle of Ignorance (Information Scarcity):** **Agents are NOT omniscient.** An agent's decision-making systems
   **MUST NOT** directly query the internal state of other entities. All information about the external world must be
   filtered through **Perception Systems** (e.g., `vision_system`, `hearing_system`) which write to the agent's own
   internal **Knowledge Components** (`MemoryStore`, `BeliefSystem`, `PerceivedEntities`). Decisions are based on this
   internal, subjective, and potentially flawed model of reality, not on the ground truth.

2. **Neuro-inspired Cognition (The "Memory Address" Model):** Knowledge and memory are **NOT** stored as explicit state.
   They are represented as the **plasticity of synaptic connections** between abstract concepts (represented by `Entity`
   IDs or indices). Learning is the process of changing the `f32` weight of these connections.

3. **Event-Driven Architecture (The "Neuron" Model):** **Systems MUST be event-driven wherever possible.** Instead of
   polling, systems should be triggered by specific Bevy `Events`. This mimics how neurons work and is critical for
   performance.

4. **Equality of Potential:** Every agent operates on the same fundamental cognitive architecture. There are no
   simplified "LOD brains". Differences arise from experience, not design.

5. **Data-Oriented Design (DOD):** The design prioritizes data layout and transformation.
    - **Compact Data Types:** Aggressively prefer `u16`/`u32` for indices and quantized weights (`u8`, `f16`) over `f32`
      where possible.
    - **Flat Architecture:** Avoid deep abstractions. Systems are simple functions that transform data.

**Ultimate Goal:** The simulation must serve as a "digital twin" environment for training Reinforcement Learning models.

## 2. Terminology & Disambiguation (CRITICAL)

**The word "Resource" has two distinct meanings in this project. You MUST distinguish between them.**

1. **In-Game Resource (The Simulation Concept):** Refers to items within the game world that can be collected, consumed,
   or owned by NPCs (e.g., food, wood, gold).
    - **Implementation:** Typically implemented as a **`Component`** on an entity (e.g., `ResourceSource`, `Inventory`).
    - **Example:** "A system for NPCs to find a food **Resource**."

2. **Bevy Resource (The Engine Concept):** Refers to a global, unique data structure accessed via `Res<T>` or
   `ResMut<T>` (e.g., `Res<Time>`).
    - **Implementation:** A `struct` or `enum` inserted into the Bevy `App` with `.insert_resource()`.
    - **Example:** "A system that needs access to the `Time` **Bevy Resource**."

**When generating code or comments, be explicit if there is any ambiguity.**

## 3. High-Level Design Principles (MANDATORY)

**Before writing any code, consider these architectural principles.**

1. **Generalization over Specialization:** **ALWAYS prefer generic solutions over specific ones.** Before implementing a
   new piece of logic, first consider if it's a specific instance of a more general problem. If so, implement the
   general solution.
    - *Example:* The project uses a generic Type-State Builder in `generic_type_safe_builder.rs`. All specific entity
      builders **MUST** be created using this generic foundation.
    - *Rule of Thumb:* If you find yourself copying and pasting code and only changing a type or a value, that logic *
      *MUST** be generalized.

2. **Orthogonality and Single Responsibility Principle (SRP):** Systems must be small, focused, and independent. A
   system should do one thing well and not have side effects on unrelated parts of the simulation. This is crucial for
   parallelism and debugging.

3. **Event-Driven Communication:** Systems should be decoupled. The primary method of communication is through Bevy
   `Events`. A system should not know who sent an event, only that an event it cares about has occurred.

## 4. Project Structure & File Organization (Strict)

Adhere strictly to the provided project structure. **This is a critical instruction.**

- **File Naming:** Use `snake_case` for all file and directory names. The prefix should match the module name (e.g.,
  `systems_needs.rs`).

- **`src/components/`:**
    - Component `struct`s and `enum`s related to a specific domain are grouped into a single file (e.g.,
      `components_needs.rs`).
    - The `impl Default for YourComponent` block **MUST** be in the same file as the component's definition.

- **`src/systems/`:**
    - Systems related to a specific domain are grouped into a single file (e.g., `systems_needs.rs`).

- **`src/systems/events/`:**
    - Bevy `Event` `struct`s are defined here, grouped by domain (e.g., `events_needs.rs`).

- **`src/entity_builders/`:**
    - **`generic_type_safe_builder.rs`:** Contains the generic, reusable Type-State Builder implementation. **This file
      should rarely be modified.**
    - **`entity_builder_default.rs`:** **This is where all specific entity builders (like `NpcBuilder`) are defined.**
      They **MUST** be implemented as type aliases or thin wrappers around the generic builder.

- **`src/components/components_default.rs`:**
    - This file contains the **`CustomComponentsPlugin`**.
    - **ALL** custom components **MUST** be registered here.

## 5. Technical Stack & Dynamic Versioning

- **Engine:** Bevy Engine. Infer the version from `Cargo.toml`.
- **Physics & Collision:** **Use `bevy_rapier2d` for ALL physics interactions.** Infer the version from `Cargo.toml`.
- **Debugging UI:** Use `bevy_inspector_egui`. Infer the version from `Cargo.toml`.

## 6. Coding Style & Syntax Rules (Strict)

- **Language:** **All generated code, comments, and documentation MUST be in English.**
- **The `query.get()` Pattern for Events:**
    - **Prefer reacting to `EventReader<T>` over iterating a `Query<T>` in the `Update` schedule.**
    - Events must be lightweight and carry the `Entity` ID, not component data.
    - Receiving systems use **`query.get(entity)`** or **`query.get_mut(entity)`** for a direct, performant lookup of
      the most up-to-date component data.
- **Component Design:**
    - Components are simple data containers. **NO LOGIC IN COMPONENTS.**
    - When creating a new Component, **ALWAYS** add these derives:
      ```rust
      #[derive(Component, Debug, Reflect, Default)]
      #[reflect(Component)] 
      ```
- **System Naming:** Name systems clearly, ending with `_system`. Example: `pub fn desire_generation_system(...)`.
- **No Magic Numbers:** Avoid hardcoded values. Constants should be defined in a central configuration component or
  resource, and their values must be justified by scientific references where applicable (see Section 8).

## 7. Type-Safe Entity Creation (MANDATORY)

**All complex entities MUST be created using the generic Type-State Builder located
at `src/entity_builders/generic_type_safe_builder.rs`.**

- **Generic Foundation:** The generic builder handles the state machine logic using `PhantomData` and generic type
  parameters.
- **Specific Builder Definition:** Specific builders (e.g., `NpcBuilder`) are defined in `entity_builder_default.rs` as
  **type aliases** of the generic builder, specifying the concrete state marker types.
- **Implementation via Extension Traits:** Methods specific to a builder (like `.with_personality()`) are implemented on
  the generic builder struct using extension traits, constrained by the generic state types.

## 8. Scientific Grounding & Validation (Crucial Directive)

- **Grounding in Theory:** Add a doc comment (`///`) referencing the primary scientific concept.
- **Validation Against Papers:** **When implementing a system, refer to the papers in the project's `/docs` folder.**
  Logic, constants, and thresholds should be directly informed by these models.
- **Justify Your Parameters:** Add a comment explaining *why* a specific value was chosen, referencing the source
  material.
    - **Good Example:**
      ```rust
      /// System that decays needs over time, based on homeostatic principles.
      pub fn needs_decay_system(mut query: Query<&mut BasicNeeds>, time: Res<Time>) {
          // This decay rate is a starting point based on the metabolic models
          // discussed in Sterling (2012), representing a simplified energy expenditure.
          // See: /docs/sterling_2012_allostasis.pdf
          const HUNGER_DECAY_RATE: f32 = 0.5; 
          // ...
      }
      ```

## 9. Machine Learning Integration Directives

- **Quantifiable & Observable State:** All component fields must be numerical (`f32`, `bool`, `Vec2`, etc.) to create
  an "observation space" for a future ML agent.
- **Action & Reward Hooks:** Structure systems to facilitate future ML hooks.
    - *Example:*
      ```rust
      // ML-HOOK: The final desire selection is a candidate for an RL agent's "action space".
      // The resulting change in 'Needs' can be used to calculate a "reward".
      *desire = Desire::FindFood;
      ```

## 10. Code Snippet Examples (The "Artificial Society" Way)

- **Example of the "Mantle of Ignorance" Architecture:**

  **Anti-Pattern (DO NOT DO THIS):**
  ```rust
  // WRONG: This system gives the agent omniscient knowledge of another's needs.
  fn social_system(
      agent_q: Query<Entity, With<WantsToTalk>>,
      target_q: Query<&Needs>, // Direct access to internal state of others.
  ) { /* ... */ }
  ```

  **Correct Pattern (Perception -> Cognition):**
  ```rust
  // In `src/components/components_npc.rs`
  #[derive(Component, Default)]
  pub struct PerceivedEntities {
      // Stores what the agent currently sees. The data is what is externally apparent.
      pub in_sight: Vec<(Entity, ApparentState)>,
  }
  #[derive(Component, Clone, Copy)]
  pub struct ApparentState { // Publicly observable state
      pub posture: Posture,
      pub is_moving: bool,
  }

  // In `src/systems/systems_perception.rs`
  /// PERCEPTION SYSTEM: The ONLY system that can query other entities' state broadly.
  /// Its only job is to update the agent's internal knowledge components.
  pub fn vision_system(
      mut agent_query: Query<(&Transform, &mut PerceivedEntities)>,
      world_query: Query<(Entity, &Transform, &ApparentState)>,
  ) {
      for (agent_transform, mut perception) in agent_query.iter_mut() {
          perception.in_sight.clear();
          for (other_entity, other_transform, apparent_state) in world_query.iter() {
              // ... line-of-sight and distance check ...
              perception.in_sight.push((other_entity, *apparent_state));
          }
      }
  }

  // In `src/systems/systems_decision.rs`
  /// COGNITIVE SYSTEM: This system is "blind". It ONLY queries its own agent's components.
  pub fn decide_interaction_system(
      agent_query: Query<(Entity, &PerceivedEntities, &BeliefSystem)>,
  ) {
      for (agent, perception, beliefs) in agent_query.iter() {
          for (seen_entity, apparent_state) in &perception.in_sight {
              // Decision is based on PERCEIVED data, not ground truth.
              if apparent_state.posture == Posture::Friendly && beliefs.is_trustworthy(seen_entity) {
                  // Fire event to initiate interaction.
              }
          }
      }
  }
  ```

- **Example of the Generic Builder (`src/entity_builders/generic_type_safe_builder.rs`):**
  ```rust
  use bevy::prelude::*;
  use std::marker::PhantomData;

  // A generic builder that tracks the presence of a component of type `T`.
  pub struct GenericBuilder<'w, 's, T> {
      pub commands: &'w mut Commands<'s, 's>,
      pub component: Option<T>,
      pub _state: PhantomData<T>, // Tracks the component type for state
  }

  // This is a simplified example. The actual implementation would have more states.
  // For a full Type-State implementation, refer to the project's existing code.
  ```

- **Example of a Specific Builder Definition (`src/entity_builders/entity_builder_default.rs`):**
  ```rust
  use bevy::prelude::*;
  use crate::components::components_npc::Personality;
  use crate::entity_builders::generic_type_safe_builder::GenericBuilder; // Import the generic builder

  // --- State Marker Types for the NPC ---
  pub struct NoPersonality;
  pub struct HasPersonality;
  // ... other state markers

  // --- Type Alias for the specific NPC Builder ---
  // This defines NpcBuilder as a specific configuration of the generic builder.
  pub type NpcBuilder<'w, 's, P> = GenericBuilder<'w, 's, (PhantomData<P>, Option<Personality>)>;

  // --- Extension Trait to add methods to the builder ---
  // We implement methods on the generic builder, but constrain them to our specific states.
  pub trait NpcBuilderExt<'w, 's> {
      fn with_personality(self, personality: Personality) -> NpcBuilder<'w, 's, HasPersonality>;
      fn build(self) -> Entity;
  }

  // Implementation for the "NoPersonality" state
  impl<'w, 's> NpcBuilderExt<'w, 's> for NpcBuilder<'w, 's, NoPersonality> {
      fn with_personality(self, personality: Personality) -> NpcBuilder<'w, 's, HasPersonality> {
          // ... logic to transition state
          unimplemented!()
      }
      // .build() is not implemented here, so it can't be called.
      fn build(self) -> Entity { unimplemented!() }
  }

  // Implementation for the "HasPersonality" state
  impl<'w, 's> NpcBuilderExt<'w, 's> for NpcBuilder<'w, 's, HasPersonality> {
      // .with_personality() is not needed here.
      fn with_personality(self, _personality: Personality) -> NpcBuilder<'w, 's, HasPersonality> { self }
      
      fn build(self) -> Entity {
          // ... logic to spawn the entity
          unimplemented!()
      }
  }
  ```

- **Example of Using the Builder in a System:**
  ```rust
  use crate::entity_builders::entity_builder_default::{NpcBuilder, NpcBuilderExt};

  fn setup_world(mut commands: Commands) {
      // The specific builder is created, but it uses the generic foundation.
      let builder = NpcBuilder::new(&mut commands); 
      
      let npc_entity = builder
          .with_personality(Personality::default())
          // .with_needs(...) etc.
          .build(); // This is guaranteed by the compiler to be valid.
  }
  ```

## 11. High-Level Runtime Architecture Overview

**This section clarifies the separation of concerns between different parts of the live infrastructure.**

The final application will not be a single executable but a distributed system composed of three distinct layers. Code
should be written with this separation in mind.

1. **The SDB Core (The Central Nervous System):**
    - **Location:** Runs on the Space and Time DB cloud.
    - **Code:** Consists of table definitions and **Reducers**.
    - **Responsibility:** Manages persistent state, player authentication, and validates high-level, discrete actions (
      e.g., `PlayerTradeItem`, `NpcCompleteQuest`). It is the ultimate source of truth. **It does NOT run continuous
      simulations like physics or AI perception.**

2. **The Bevy Simulation Worker (The Local Brain / Reflexes):**
    - **Location:** Runs as a **native, headless Bevy application** on a separate server (or multiple servers for
      different zones).
    - **Code:** This is where the majority of our Bevy systems reside (`PhysicsSystem`, `AISystems`,
      `InteractionSystem`).
    - **Responsibility:** Executes the **high-frequency, continuous simulation** for a specific zone of the world. It
      operates on data loaded into local RAM for maximum performance. It uses the `bevy_spacetimedb` crate to
      communicate with the SDB Core.

3. **The Player Client (The Sensory Interface):**
    - **Location:** Runs on the player's machine.
    - **Code:** A Bevy (or other engine) application focused on rendering, sound, and input.
    - **Responsibility:** Handles input prediction and state interpolation to provide a smooth experience. It receives
      its state from the SDB Core, not directly from a Bevy Worker.

**Rule:** When writing a system, always consider: "Does this logic belong in a fast, local simulation (Bevy Worker) or
as a secure, global state change (SDB Reducer)?" The default for complex, continuous processes is **always** the Bevy
Worker.
