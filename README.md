# <p align="center"><img src="./thumb-horizontal.jpg" alt="Artificial Society"></p>

<h3 align="center">Artificial Society (Foundations Phase)</h3>
<p align="center">
Believable AI-driven social simulation in Rust / Bevy – emergent minds through imperfect communication.
</p>
<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version 0.1.0" />
  <img src="https://img.shields.io/badge/bevy-0.16.1-brightgreen" alt="Bevy 0.16.1" />
  <img src="https://img.shields.io/badge/rust-Edition%202024-orange" alt="Rust Edition 2024" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-CC--BY--NC--SA--4.0-lightgrey" alt="License" /></a>
</p>

---

## TL;DR Architecture Summary

- Goal: Emergent, human-believable social dynamics via lossy information flow, not scripted branching logic.
- Core Loop: Physiology → Modulated Expression → Distorted Perception → Inference → Updated Beliefs → Next Action.
- Data Model: Continuous numeric component fields (no discrete mood enums); subjective state stored per agent.
- Interaction Principle: Agents never read each other's internal truth—only perceived approximations + biased beliefs.
- ECS Strategy: Domain-isolated plugins (physiology, perception, cognition, social) communicating through events.
- Performance Assumption: High-frequency social simulation must be co-located (single process per zone, later multi-zone
  scale-out).
- Scaling Plan: Many headless Bevy workers (zones) + SpaceTimeDB for persistence & cross-zone synchronization.
- ML Readiness: Observation = flattened component vectors; intervention hooks at desire selection, action arbitration,
  and belief updates.

---

## Table of Contents

- [Overview](#overview)
- [Core Philosophical & Technical Principles](#core-philosophical--technical-principles)
- [Current Focus (Foundations Phase)](#current-focus-foundations-phase)
- [Layered Communication Pipeline (Target Architecture)](#layered-communication-pipeline-target-architecture)
- [Repository Structure (Linked Technical Overview)](#repository-structure-linked-technical-overview)
- [Milestone Roadmap (Condensed)](#milestone-roadmap-condensed)
- [Features (Prototype Stage)](#features-prototype-stage)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation Map](#documentation-map)
- [Design Guarantees (Target State)](#design-guarantees-target-state)
- [Machine Learning Readiness (Forward-Looking)](#machine-learning-readiness-forward-looking)
- [Contributing](#contributing)
- [License](#license)
- [Attribution & AI Assistance Disclaimer](#attribution--ai-assistance-disclaimer)
- [Status Badge (Human Summary)](#status-badge-human-summary)
- [Next Steps](#next-steps)

---

## Overview

Artificial Society is an experimental, research-oriented simulation (future MMORPG target) exploring whether **human
players can use real-world social intuition when interacting with NPCs** – and sometimes be unsure if they are facing a
human or an AI. The project focuses on **social believability**, not raw intelligence. The core design principle: *the
interesting drama emerges from the distortions, biases, and failures in communication between imperfect agents*.

This repository currently contains the early Bevy prototype (Milestone 1 → 2 transition). Earlier code that only modeled
simple rumor diffusion is being refactored toward the full four‑layer communication pipeline described in the
specification.

---

## Core Philosophical & Technical Principles

1. Mantle of Ignorance – Agents are never omniscient; all knowledge is mediated through perception buffers and
   subjective belief formation.
2. Continuous State Spaces – Emotions, expression, intent, and social stance are continuous vectors, not enums.
3. Simulation Locality – High‑frequency social + physiological simulation co-locates in a single process (later:
   multiple zone workers) for emergent coupling.
4. Event-Driven ECS – Systems react to events (neuron metaphor) instead of polling where feasible.
5. Selective Fidelity – Science informs design, but experiential believability beats exhaustive biological accuracy.
6. Data-Oriented Design – Flat, cache-friendly components; small numeric fields for future ML observation spaces.
7. Equality of Potential – All agents share the same architectural substrate; differences emerge from learning &
   history.

---

## Current Focus (Foundations Phase)

Implementing the **Expressive Agent** and **Subjective Observer** milestones:

- Physiological + cognitive modulators feeding into an `ApparentStateVector` (expression layer).
- Perception layer introducing distortion (attention, emotional reactivity, confirmation bias).
- Belief formation groundwork for future inference and persistent relationships.

---

## Layered Communication Pipeline (Target Architecture)

Internal Truth → Expression → Perception → Inference

- Layer 1 (Internal State): Needs, stress, emotions, beliefs (private).
- Layer 2 (Expression): Transforms internal gradients → externally observable continuous social vector.
- Layer 3 (Perception): Subjective, biased reconstruction (noise, modulators, prior beliefs).
- Layer 4 (Inference): Pattern-matching & intent attribution (often wrong in consistent, human ways).

Resulting emergent phenomena: miscommunication, self‑reinforcing bias, emotional contagion, reputational drift,
proto‑culture formation.

---

## Repository Structure (Linked Technical Overview)

The codebase is organized **by domain (feature)**. Each domain exposes (or will expose) a `Plugin` in its `mod.rs` to
register components, events, and systems. Below: direct links to real files with concise technical roles.

```
(root)
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── ai/
│   │   ├── mod.rs
│   │   ├── cognition/
│   │   ├── navigation/
│   │   ├── perception/
│   │   ├── physiology/
│   │   └── social/
│   ├── core/
│   ├── presentation/
│   ├── utils/
│   └── world/
└── Docs/
```

### Entry Point

- [`src/main.rs`](src/main.rs) – Builds the Bevy `App`; aggregates domain plugins and presentation/debug layers.

### AI Root Module

- [`src/ai/mod.rs`](src/ai/mod.rs) – Re-exports domain modules; central place to add future plugin wiring.

### Cognition Domain

- [`src/ai/cognition/mod.rs`](src/ai/cognition/mod.rs) – (Planned) plugin definition & module exports.
- [`src/ai/cognition/decision.rs`](src/ai/cognition/decision.rs) – Action / intent arbitration scaffolding (future:
  utility integration).
- [`src/ai/cognition/desires.rs`](src/ai/cognition/desires.rs) – Desire generation primitives (seed for motivational
  layer).
- [`src/ai/cognition/inference.rs`](src/ai/cognition/inference.rs) – Placeholder for social/intent inference logic (
  Layer 4 target).
- [`src/ai/cognition/knowledge.rs`](src/ai/cognition/knowledge.rs) – Structures for subjective world model (belief
  graph / indices planned).
- [`src/ai/cognition/memory.rs`](src/ai/cognition/memory.rs) – Episodic & semantic memory abstractions (will back
  learning & bias).
- [`src/ai/cognition/personality.rs`](src/ai/cognition/personality.rs) – Trait vectors & modulation factors (continuous,
  parametric).

### Navigation Domain

- [`src/ai/navigation/mod.rs`](src/ai/navigation/mod.rs) – (Planned) plugin; groups path & spatial learning systems.
- [`src/ai/navigation/pathfinding.rs`](src/ai/navigation/pathfinding.rs) – Path resolution utilities (heuristic / future
  nav-mesh integration).
- [`src/ai/navigation/mapping.rs`](src/ai/navigation/mapping.rs) – Spatial knowledge representation (occupancy /
  familiarity metrics planned).
- [`src/ai/navigation/learning.rs`](src/ai/navigation/learning.rs) – Reinforcement of route selection (future ML hooks).

### Perception Domain

- [`src/ai/perception/mod.rs`](src/ai/perception/mod.rs) – (Planned) plugin for registering sensory systems.
- [`src/ai/perception/vision.rs`](src/ai/perception/vision.rs) – Visual acquisition (Rapier queries & LOS mechanics
  roadmap).
- [`src/ai/perception/attention.rs`](src/ai/perception/attention.rs) – Attention weighting & noise injection (distortion
  for Layer 3).

### Physiology Domain

- [`src/ai/physiology/mod.rs`](src/ai/physiology/mod.rs) – (Planned) plugin; binds needs & state update systems.
- [`src/ai/physiology/needs.rs`](src/ai/physiology/needs.rs) – Core homeostatic drives (hunger, energy, safety,
  social—continuous floats).
- [`src/ai/physiology/states.rs`](src/ai/physiology/states.rs) – Stress state machine enums / classification
  scaffolding.
- [`src/ai/physiology/systems.rs`](src/ai/physiology/systems.rs) – Decay, feedback modulation, and threshold event
  emitters (future eventization).

### Social Domain

- [`src/ai/social/mod.rs`](src/ai/social/mod.rs) – (Planned) plugin for interaction & expression pipeline systems.
- [`src/ai/social/interaction.rs`](src/ai/social/interaction.rs) – Interaction triggers (legacy rumor logic anchor for
  refactor).
- [`src/ai/social/reputation.rs`](src/ai/social/reputation.rs) – Reputation / belief aggregation framework (mapping to
  continuous trust axes).
- [`src/ai/social/systems.rs`](src/ai/social/systems.rs) – Execution of social updates (will emit expression &
  perception events).

### Core (Cross-Domain Infrastructure)

- [`src/core/mod.rs`](src/core/mod.rs) – Core exports.
- [`src/core/constants.rs`](src/core/constants.rs) – Tunable numeric constants (move toward scientifically justified
  sets).
- [`src/core/entities.rs`](src/core/entities.rs) – Shared entity classification / marker components.
- [`src/core/spawning.rs`](src/core/spawning.rs) – Spawn routines (will migrate to builder + type-state pattern).
- [`src/core/utils.rs`](src/core/utils.rs) – Generic helpers not tied to a single domain.

### Presentation / Debug

- [`src/presentation/mod.rs`](src/presentation/mod.rs) – (Planned) presentation plugin.
- [`src/presentation/fps_display.rs`](src/presentation/fps_display.rs) – Frame timing overlay.
- [`src/presentation/vision_debug.rs`](src/presentation/vision_debug.rs) – Visualizes perception rays / LOS (diagnostic
  only).

### Utilities

- [`src/utils/mod.rs`](src/utils/mod.rs) – Misc macro & helper re-exports.
- [`src/utils/helpers`](src/utils/helpers) – (Directory) Generic helper modules.
- [`src/utils/macros`](src/utils/macros) – (Directory) Code generation / ergonomics macros.

### World Assembly

- [`src/world/mod.rs`](src/world/mod.rs) – Environment / map scaffolding (will host resource node spawning & zone
  partitioning).

### Documentation

- [
  `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md`](Docs/Fundaments/Artificial%20Society_%20Complete%20Technical%20and%20Philosophical%20Specification.md) –
  Authoritative design & theory.
- [`Docs/Structures/components.md`](Docs/Structures/components.md) – Component catalog.
- [`Docs/Structures/systems.md`](Docs/Structures/systems.md) – System purposes & schedules.
- [`Docs/Flows/interaction_flows.md`](Docs/Flows/interaction_flows.md) – Planned interaction sequences.
- [`Docs/Flows/roadmap.md`](Docs/Flows/roadmap.md) – Broader development roadmap.

> NOTE: Several `mod.rs` files do not yet implement their plugin struct; converting each domain into a fully isolated
> plugin is an active refactor task (Milestone 1 tail / Milestone 2 start).

---

## Milestone Roadmap (Condensed)

| Milestone                  | Goal                                   | Key Proof                           | Status      |
|----------------------------|----------------------------------------|-------------------------------------|-------------|
| 1 Expressive Agent         | Internal → external expression mapping | Hunger/stress affect tension vector | In Progress |
| 2 Subjective Observer      | Distorted perception + biased belief   | A perceives B incorrectly & reacts  | Pending     |
| 3 Persistent Social Memory | Relationship drift & confirmation bias | Early misunderstanding persists     | Planned     |
| 4 Goal-Oriented Behavior   | Utility / strategy overlay             | Multi-step social strategies        | Future      |
| 5 Group Dynamics           | Emergent factions & norms              | Cultural clustering                 | Future      |
| 6 Scale & Optimization     | 100+ agents @60 FPS                    | Profiling & LOD                     | Future      |
| 7 Network Integration      | SpaceTimeDB + zone workers             | Persisted social state              | Future      |

Detailed criteria live in: `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md` (
Sections 6 & 10).

---

## Features (Prototype Stage)

Implemented / Partial:

- Bevy ECS foundation (Bevy 0.16.1, dynamic linking).
- Basic agent spawning & movement.
- Early perception & social interaction scaffolds (legacy rumor system).
- Inspector-based live tuning (`bevy_inspector_egui`).

Planned (Near Term):

- Modular domain plugins (physiology, cognition, perception, social) with events.
- Expression → perception distortion pipeline.
- Belief system with confirmation bias.
- Stress & modulators affecting cognitive clarity and emotional reactivity.

Longer Term:

- Reputation, intent inference, cultural drift, zone-based scaling, ML observation/export hooks, SpaceTimeDB
  integration.

---

## Installation

Prerequisites: Recent Rust toolchain (matching Bevy 0.16.x requirements).

```bash
git clone https://github.com/maxjonata/Artificial-Culture-Rust-.git
cd Artificial-Culture-Rust-
cargo run --release
```

(Using `--release` recommended for smoother frame pacing; Bevy dynamic linking reduces initial compile time.)

If dynamic linking on your platform causes issues, remove the `"dynamic_linking"` feature from `Cargo.toml` and rebuild.

---

## Usage

Run the binary to open the simulation window. Current prototype shows mobile agents (legacy rumor coloration may still
appear). Upcoming refactors will replace color swaps with vector-based expression diagnostics.

Quick actions:

- Escape (or inspector binding): Toggle the EGUI inspector (inspect & tweak component fields live).
- Adjust component values to observe downstream changes (e.g., hunger → tension once Milestone 1 is complete).

---

## Documentation Map

Primary Spec (read first):

- `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md`

Supporting:

- `Docs/Structures/` – ECS architecture, components, systems
- `Docs/Flows/` – Interaction flows & roadmap (some UNFINISHED markers)
- `Docs/Fundaments/neurological.md`, `psychological.md`, `sociological.md` – Theoretical grounding
- `Docs/Papers/` – Cited academic sources

Some documents intentionally contain placeholders for iterative refinement; unfinished sections are marked accordingly.

---

## Design Guarantees (Target State)

- No system reads another agent's ground‑truth internal state directly.
- Communication pathways are numerical, lossy, and biasable.
- All social/emotional constructs are continuous and composable.
- Events carry `Entity` IDs and minimal payload; receivers fetch fresh state.
- Components are pure data carriers (no inherent logic).
- Parameters justified by referenced literature (see Papers folder) where stabilized.

---

## Machine Learning Readiness (Forward-Looking)

- Components kept numerically dense for observation vectors.
- Future hooks: action selection interception, reward shaping from physiological deltas, offline dataset generation for
  RL / imitation learning.
- Planned export: periodic snapshot serialization for trajectory replay & training.

---

## Contributing

Contributions, issue reports, and research discussions are welcome.
Guidelines (informal during early phase):

- Favor general abstractions over ad hoc features.
- Keep systems single-responsibility & event-driven.
- Include rationale comments referencing theory when adding constants.
- Avoid adding new discrete enums for social states—prefer parametric / continuous representations.

Feel free to open a draft PR to discuss architectural alignment early.

---

## License

Creative Commons **CC BY-NC-SA 4.0**. Non-commercial use permitted with attribution & share-alike. For commercial
licensing inquiries, open an issue or reach out directly.

---

## Attribution & AI Assistance Disclaimer

Development is heavily accelerated using AI pair tools (GitHub Copilot, GPT variants, Claude, Manus, etc.). All
generated content is reviewed and adapted to conform to project philosophy and technical constraints.

---

## Status Badge (Human Summary)

"Early foundation prototype – refactoring toward expressive + subjective perception layers. Expect breaking changes,
experimental APIs, and incomplete docs."

---

## Next Steps

1. Finalize `Needs` + modulators → expression mapping (Milestone 1 completion).
2. Implement perception distortion & belief accumulation (Milestone 2 start).
3. Replace legacy rumor-only logic with generalized communication events.
4. Implement per-domain plugins (cognition, perception, physiology, social) with event registration.
5. Introduce expression → perception → belief debug overlay for live tracing.

---

Questions or research collaboration ideas? Open an issue – focused, theory-backed proposals appreciated.
