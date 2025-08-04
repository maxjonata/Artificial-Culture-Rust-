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

1. **Any_Thing:** Refers to items within the game world that can be collected, consumed,
   or owned by NPCs (e.g., food, wood, gold).
    - **Implementation:** Typically implemented as a **`Component`** on an _entity (e.g., `ResourceSource`,
      `Inventory`).
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
    - *Example:* The project uses a generic Type-State Builder in `generic_type_safe_builder.rs`. All specific _entity
      builders **MUST** be created using this generic foundation.
    - *Rule of Thumb:* If you find yourself copying and pasting code and only changing a type or a value, that logic *
      *MUST** be generalized.

2. **Orthogonality and Single Responsibility Principle (SRP):** Systems must be small, focused, and independent. A
   system should do one thing well and not have side effects on unrelated parts of the simulation. This is crucial for
   parallelism and debugging.

3. **Event-Driven Communication:** Systems should be decoupled. The primary method of communication is through Bevy
   `Events`. A system should not know who sent an event, only that an event it cares about has occurred.

## 4. Project Structure & File Organization (Strict)

**This is a critical instruction. The project is organized by functional domain (feature), not by code type.**

- **Core Principle:** All code related to a single feature (e.g., "cognition", "navigation", "physiology") is grouped
  into its own module directory under `src/ai/`.
- **Module Structure:** Each domain module (e.g., `src/ai/cognition/`) contains all its related code: components,
  systems, events, and its own `mod.rs` file.
- **File Naming:** Use `snake_case`. Files inside a domain module should be named logically (e.g., `decision.rs`,
  `memory.rs`).

- **`src/ai/`:** The root for all agent intelligence logic.
    - **`src/ai/{domain_name}/` (e.g., `src/ai/physiology/`):**
        - This is a self-contained module for a specific feature.
        - It contains all related components, systems, and events.
        - **Example:** `src/ai/physiology/needs.rs` would contain the `Needs` component, the `needs_decay_system`, and
          any related events like `NeedsThresholdCrossed`.
    - **`src/ai/{domain_name}/mod.rs`:**
        - Declares the sub-modules (like `pub mod needs;`).
        - Defines the domain's `Plugin` (e.g., `pub struct PhysiologyPlugin;`).

- **`src/core/`:** For truly generic, cross-domain code.
    - **`src/core/builders.rs`:** Contains the generic Type-State Builder and all specific builder definitions (like
      `NpcBuilder`).
    - **`src/core/utils.rs`:** Contains utility functions that are not tied to any specific domain.

- **`src/presentation/`:** For code related to visualization and debugging.
    - **`src/presentation/debug_ui.rs`:** Contains the `debug_ui_system` for rendering UI overlays. This module is for
      **human observation only** and must not interact with the AI's decision-making loop.

- **`src/main.rs`:**
    - The entry point of the application.
    - Its primary role is to build the Bevy `App` and **add the plugins** from each domain (e.g.,
      `.add_plugins(PhysiologyPlugin)`).

## 5. Bevy Plugin Architecture (MANDATORY)

**The project's architecture is built on Bevy Plugins. Each functional domain MUST be encapsulated in its own plugin.**

- **Plugin Definition:** Each domain module (e.g., `src/ai/physiology/`) must define a public `struct` that implements
  the `Plugin` trait (e.g., `pub struct PhysiologyPlugin;`).
- **Responsibilities of a Plugin:** The `build` method of a domain's plugin is responsible for:
    1. **Registering all components and events** defined within that domain using `.register_type::<T>()`.
    2. **Adding all systems** defined within that domain to the appropriate `Schedule` (e.g., `Update`, `Startup`).
- **Central Registration:** The `main.rs` file simply adds these domain plugins to the `App`. This keeps `main.rs`
  clean and makes it easy to enable or disable entire features of the simulation.

## 6. Technical Stack & Dynamic Versioning

- **Engine:** Bevy Engine. Infer the version from `Cargo.toml`.
- **Physics & Collision:** **Use `bevy_rapier2d` for ALL physics interactions.** Infer the version from `Cargo.toml`.
- **Debugging UI:** Use `bevy_inspector_egui`. Infer the version from `Cargo.toml`.

## 7. Coding Style & Syntax Rules (Strict)

- **Language:** **All generated code, comments, and documentation MUST be in English.**
- **The `query.get()` Pattern for Events:**
    - **Prefer reacting to `EventReader<T>` over iterating a `Query<T>` in the `Update` schedule.**
    - Events must be lightweight and carry the `Entity` ID, not component data.
    - Receiving systems use **`query.get(_entity)`** or **`query.get_mut(_entity)`** for a direct, performant lookup of
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

## 8. Type-Safe Entity Creation (MANDATORY)

**All complex entities MUST be created using the generic Type-State Builder located
at `src/entity_builders/generic_type_safe_builder.rs`.**

- **Generic Foundation:** The generic builder handles the state machine logic using `PhantomData` and generic type
  parameters.
- **Specific Builder Definition:** Specific builders (e.g., `NpcBuilder`) are defined in `entity_builder_default.rs` as
  **type aliases** of the generic builder, specifying the concrete state marker types.
- **Implementation via Extension Traits:** Methods specific to a builder (like `.with_personality()`) are implemented on
  the generic builder struct using extension traits, constrained by the generic state types.

## 9. Scientific Grounding & Validation (Crucial Directive)

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

## 10. Machine Learning Integration Directives

- **Quantifiable & Observable State:** All component fields must be numerical (`f32`, `bool`, `Vec2`, etc.) to create
  an "observation space" for a future ML agent.
- **Action & Reward Hooks:** Structure systems to facilitate future ML hooks.
    - *Example:*
      ```rust
      // ML-HOOK: The final desire selection is a candidate for an RL agent's "action space".
      // The resulting change in 'Needs' can be used to calculate a "reward".
      *desire = Desire::FindFood;
      ```

## 11. Code Snippet Examples (The "Artificial Society" Way)

- **Example of a Domain Module (`src/ai/physiology/needs.rs`):**
  ```rust
  use bevy::prelude::*;

  // --- Component Definition ---
  /// Stores the agent's primary physiological needs.
  #[derive(Component, Debug, Reflect, Default)]
  #[reflect(Component)]
  pub struct Needs {
      pub hunger: f32,
  }

  // --- Event Definition ---
  #[derive(Event)]
  pub struct NeedsThresholdCrossed {
      pub agent: Entity,
      pub need: NeedType,
  }
  
  #[derive(Debug)]
  pub enum NeedType { Hunger }

  // --- System Definition ---
  /// System that decays needs over time.
  pub fn needs_decay_system(mut query: Query<&mut Needs>) {
      // ... logic ...
  }
  ```

- **Example of a Domain Plugin (`src/ai/physiology/mod.rs`):**
  ```rust
  use bevy::prelude::*;
  pub mod needs; // Make the contents of needs.rs public within the module

  use needs::{needs_decay_system, Needs, NeedsThresholdCrossed};

  pub struct PhysiologyPlugin;

  impl Plugin for PhysiologyPlugin {
      fn build(&self, app: &mut App) {
          app
              // Register components and events from this domain
              .register_type::<Needs>()
              .add_event::<NeedsThresholdCrossed>()
              // Add systems from this domain
              .add_systems(Update, needs_decay_system);
      }
  }
  ```

- **Example of `main.rs`:**
  ```rust
  use bevy::prelude::*;
  use artificial_society::ai::physiology::PhysiologyPlugin;
  use artificial_society::ai::cognition::CognitionPlugin;
  // ... other imports

  fn main() {
      App::new()
          .add_plugins(DefaultPlugins)
          // Add the domain plugins. This is the only place they are registered.
          .add_plugins(PhysiologyPlugin)
          .add_plugins(CognitionPlugin)
          // ... add other domain plugins
          .run();
  }
  ```

## 12. High-Level Runtime Architecture Overview

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

## 13. Terminal Command Generation (Safety Rule)

**This is a non-negotiable rule for generating any shell commands.**

**Principle: Clarity and Step-by-Step Execution.** All terminal commands must be presented as a sequence of individual,
atomic steps.

**Strict Rules:**

1. **One Command Per Line:** Each command must be on its own line.
2. **No Chaining:** **DO NOT** use command chaining operators like `&&`, `||`, or `;`.
3. **No Piping:** **DO NOT** use pipes (`|`). If a multi-step process is needed, explain it in text and provide the
   separate commands.
4. **No Complex Scripts:** Do not generate multi-line shell scripts. Provide the sequence of simple commands instead.

**Correct Pattern:**
> To generate a dependency graph, follow these steps:
>
> 1. First, generate the `.dot` file from cargo:
     >     ```bash
     > cargo depgraph --all-deps > dependency_graph.dot
     >     ```
> 2. Then, use the `dot` command to convert the file into an image:
     >     ```bash
     > dot -Tpng dependency_graph.dot -o dependency_graph.png
     >     ```