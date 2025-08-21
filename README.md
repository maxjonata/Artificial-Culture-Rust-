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

## TL;DR (Architecture in One Screen)

- Goal: Emergent, socially believable NPC dynamics via lossy information + biased inference (not scripted trees).
- Core Cognitive Loop: Physiology ➜ Expression ➜ Distorted Perception ➜ Biased Belief Update ➜ Desire/Intent ➜ Action.
- Information Rule: Agents never read another agent's ground‑truth internals; only their own perceived/derived model.
- Data Shape: Continuous numeric component fields (dense observation space; minimal enums) for future ML integration.
- ECS Style: Feature (domain) modularization + event-driven systems (`Entity` IDs in events; fresh lookup via
  `query.get`).
- Simulation Scaling Vision: Many headless Bevy zone workers + SpaceTimeDB persistence layer (later stage).
- ML Readiness: Observation vectors = flattened component sets; hooks at desire arbitration, action selection, belief
  reinforcement.

---

## Table of Contents

- [Overview](#overview)
- [Foundational Principles](#foundational-principles)
- [Current Milestone Focus](#current-milestone-focus)
- [Layered Communication Model](#layered-communication-model)
- [Repository Structure (Linked)](#repository-structure-linked)
- [Domain Modules (Technical Detail)](#domain-modules-technical-detail)
- [Milestone Roadmap](#milestone-roadmap)
- [Prototype Features](#prototype-features)
- [Install & Run](#install--run)
- [Runtime Usage](#runtime-usage)
- [Documentation Map](#documentation-map)
- [Design Invariants](#design-invariants)
- [ML Integration Trajectory](#ml-integration-trajectory)
- [Contributing Guidelines](#contributing-guidelines)
- [License](#license)
- [AI Assistance Notice](#ai-assistance-notice)
- [Status Snapshot](#status-snapshot)
- [Immediate Next Steps](#immediate-next-steps)

---

## Overview

Artificial Society is a research-focused Rust/Bevy simulation probing whether players can apply authentic human social
intuition when facing AI agents—occasionally feeling uncertainty about whether an entity is human‑ or AI‑controlled.
Believability emerges not from perfect cognition but from *structured imperfection*: limited perception, biased
updating, noisy expression, and asymmetric information propagation.

This repository is the early foundational prototype in active refactor toward a four-layer communication & cognition
pipeline. Legacy/placeholder systems are being migrated to strict domain plugins and event pathways.

---

## Foundational Principles

1. Mantle of Ignorance: No omniscience; perception components are the only ingress of external world data.
2. Neuro-Inspired Data: Memory & knowledge modeled as weighted relations (planned) rather than opaque blobs.
3. Event-Driven Reactivity: Prefer events over polling (neuron metaphor; cache-friendly targeted lookups).
4. Equality of Potential: Identical cognitive substrate across all agents; divergence via learned/experienced weights &
   state histories.
5. Data-Oriented Design: Flat numeric fields; minimized branching; small types where feasible.
6. Continuous Social State: Emotions/attitudes/intent projected as continuous vectors, not enumerated categories.
7. Scientific Grounding (Iterative): Parameters documented with source rationale once stabilized.

---

## Current Milestone Focus

Active work spans Milestone 1 (Expressive Agent) closing and Milestone 2 (Subjective Observer) initiation:

- Map physiological + internal modulators ➜ outward expression vector.
- Implement attention-weighted, noisy perception producing biased salience & partial internal reconstruction.
- Establish belief accumulation scaffolding (confirmation bias & retention weighting forthcoming).

---

## Layered Communication Model

Internal Truth ➜ Expression ➜ Perception ➜ Inference ➜ (Feedback into desires & planned actions)

- Layer 1: Private internal state (needs, stress, proto-emotions, memory indices).
- Layer 2: Expression transform exposes only *projected* approximations (lossy, rate-limited).
- Layer 3: Perception reconstructs with attention constraints + noise + prior-driven bias.
- Layer 4: Inference attributes intent / reputation updates (with systematic error).

Emergent outcomes: miscommunication loops, reputation drift, emotional contagion, proto-cultural stratification.

---

## Repository Structure (Linked)

> Organised strictly by feature domain (NOT by type). Each `mod.rs` will (or soon will) host a `Plugin` implementing
> domain registration. Only real, existing files are linked below.

```
Cargo.toml
src/
  main.rs
  ai/
    mod.rs
    cognition/
      mod.rs
      decision.rs
      desires.rs
      inference.rs
      knowledge.rs
      memory.rs
      personality.rs
    navigation/
      mod.rs
      learning.rs
      mapping.rs
      pathfinding.rs
    perception/
      mod.rs
      attention.rs
      vision.rs
    physiology/
      mod.rs
      needs.rs
      states.rs
      systems.rs
    social/
      mod.rs
      interaction.rs
      reputation.rs
      systems.rs
  core/
    mod.rs
    constants.rs
    entities.rs
    spawning.rs
    utils.rs
  presentation/
    mod.rs
    fps_display.rs
    vision_debug.rs
  utils/
    mod.rs
    helpers/
    macros/
  world/
    mod.rs
Docs/
  Fundaments/... (primary specs & theory)
  Structures/... (components/systems overviews)
  Flows/... (interaction & roadmap drafts)
```

---

## Domain Modules (Technical Detail)

### Entry Layer

- [src/main.rs](src/main.rs): Application bootstrap; composes domain + presentation plugins (some still TODO).
- [src/ai/mod.rs](src/ai/mod.rs): Domain namespace root; central re-export point.

### Cognition

- [cognition/mod.rs](src/ai/cognition/mod.rs): (Will finalize `CognitionPlugin`) exports internal cognition subsystems.
- [decision.rs](src/ai/cognition/decision.rs): Intent/action arbitration scaffolding (future utility scoring & ML action
  hook site).
- [desires.rs](src/ai/cognition/desires.rs): Desire generation seeds (will emit desire events).
- [inference.rs](src/ai/cognition/inference.rs): Placeholder for social intent inference Layer 4.
- [knowledge.rs](src/ai/cognition/knowledge.rs): Subjective knowledge graph structures (agent-centric indices planned).
- [memory.rs](src/ai/cognition/memory.rs): Working / episodic memory abstractions (decay + consolidation to come).
- [personality.rs](src/ai/cognition/personality.rs): Continuous trait vectors modulating thresholds & rates.

### Navigation

- [navigation/mod.rs](src/ai/navigation/mod.rs): (Planned `NavigationPlugin`).
- [learning.rs](src/ai/navigation/learning.rs): Reinforcement placeholders for route selection.
- [mapping.rs](src/ai/navigation/mapping.rs): Spatial familiarity & occupancy representation (future heuristic
  weighting).
- [pathfinding.rs](src/ai/navigation/pathfinding.rs): Path acquisition utilities (future navmesh / Rapier queries).

### Perception

- [perception/mod.rs](src/ai/perception/mod.rs): (Upcoming `PerceptionPlugin`). Registers sensory + attention systems.
- [attention.rs](src/ai/perception/attention.rs): Selective attention component set; salience weighting & capacity
  management.
- [vision.rs](src/ai/perception/vision.rs): Visual acquisition stub (range tests + future LOS occlusion & noise
  injection).

### Physiology

- [physiology/mod.rs](src/ai/physiology/mod.rs): (Upcoming `PhysiologyPlugin`). Needs & internal drive systems.
- [needs.rs](src/ai/physiology/needs.rs): Core drive metrics (hunger/energy/social safety etc.).
- [states.rs](src/ai/physiology/states.rs): Transitional stress / arousal state markers (may become continuous
  classification).
- [systems.rs](src/ai/physiology/systems.rs): Decay & threshold event emission (refactor toward event-driven updates).

### Social

- [social/mod.rs](src/ai/social/mod.rs): (Upcoming `SocialPlugin`). Expression + interaction scheduling.
- [interaction.rs](src/ai/social/interaction.rs): Legacy interaction triggers (to generalize into event-based pipeline).
- [reputation.rs](src/ai/social/reputation.rs): Reputation / trust axis accumulation (continuous score basis).
- [systems.rs](src/ai/social/systems.rs): Social state update routines (target: expression ➜ perception events).

### Core (Cross-Domain Infrastructure)

- [core/mod.rs](src/core/mod.rs): Core exports.
- [constants.rs](src/core/constants.rs): Central tuning parameters (to receive documented justifications).
- [entities.rs](src/core/entities.rs): Shared marker & classification components (type tags for queries).
- [spawning.rs](src/core/spawning.rs): Runtime entity spawning (to migrate into type‑state builders later).

### Presentation / Debug (Non-AI Logic)

- [presentation/mod.rs](src/presentation/mod.rs): (Planned `PresentationPlugin`).
- [fps_display.rs](src/presentation/fps_display.rs): FPS overlay.

### Utilities

- [utils/mod.rs](src/utils/mod.rs): Aggregated helper exports.
- [utils/helpers](src/utils/helpers): Directory containing generic helper modules.
- [utils/macros](src/utils/macros): Macro definitions improving ergonomics.

### World Assembly

- [world/mod.rs](src/world/mod.rs): Spatial/world scaffolding (future resource nodes & zone partition registration).

### Documentation (Selected)

- [Complete Technical & Philosophical Spec](Docs/Fundaments/Artificial%20Society_%20Complete%20Technical%20and%20Philosophical%20Specification.md)
- [Components Catalog](Docs/Structures/components.md)
- [Systems Overview](Docs/Structures/systems.md)
- [Interaction Flows](Docs/Flows/interaction_flows.md)
- [Roadmap Narrative](Docs/Fundaments/DETAILED_ROADMAP.md)

---

## Milestone Roadmap

| Milestone                  | Focus                                  | Proof Artifact                                       | Status      |
|----------------------------|----------------------------------------|------------------------------------------------------|-------------|
| 1 Expressive Agent         | Internal ➜ outward expression mapping  | Physiological gradients modulate expression vector   | In Progress |
| 2 Subjective Observer      | Distorted perception + biased beliefs  | Agent misinterprets another, drives altered response | Pending     |
| 3 Persistent Social Memory | Relationship drift + confirmation bias | Early misread persists & shapes future               | Planned     |
| 4 Goal-Oriented Behavior   | Utility / multi-step strategy          | Sequenced intent generation & execution              | Future      |
| 5 Group Dynamics           | Factional / norm emergence             | Clustering of aligned reputational vectors           | Future      |
| 6 Scale & Optimization     | 100+ agents @ 60 FPS                   | Profiling & perf reports                             | Future      |
| 7 Network Integration      | SpaceTimeDB + zone workers             | Persisted cross-zone social state                    | Future      |

Detailed acceptance criteria: see spec sections on milestones.

---

## Prototype Features

Implemented / Partial:

- Bevy ECS foundation (0.16.1) + dynamic linking.
- Agent spawning & rudimentary movement.
- Early perception & social interaction scaffolds (legacy rumor structure).
- Inspector-driven live tuning (`bevy_inspector_egui`).

Near-Term (Active Refactor):

- Domain plugins with isolated registration & type reflection.
- Expression ➜ perception distortion path.
- Belief accumulation with confirmation bias weighting.
- Attention capacity limits & salience emission events.

Later: Reputation modeling, cultural drift, ML observation export, SpaceTimeDB integration.

---

## Install & Run

Prerequisites: Recent Rust toolchain (Rust 2024 edition compatible with Bevy 0.16.x).

```bash
git clone https://github.com/maxjonata/Artificial-Culture-Rust-.git
cd Artificial-Culture-Rust-
cargo run --release
```

Use `--release` for stable frame pacing. If dynamic linking issues arise on your platform, remove the `dynamic_linking`
feature in `Cargo.toml` and rebuild.

---

## Runtime Usage

Current prototype: spawns agents; early interaction visualization. Expect rapid iteration & breaking API changes.

Quick Notes:

- Toggle inspector (EGUI) to inspect and tweak numeric component fields.
- Observe downstream propagation (e.g., altering needs should eventually affect expression vector once pipeline
  solidifies).

---

## Documentation Map

Primary Spec:

- Full Design & Theory: `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md`

Supporting:

- ECS Structures: `Docs/Structures/`
- Flows & Roadmap: `Docs/Flows/`
- Theoretical Grounding: `Docs/Fundaments/neurological.md`, `psychological.md`, `sociological.md`
- Source Papers: `Docs/Papers/` (citations for parameter justification)

Some files are marked UNFINISHED intentionally; iterative hardening is expected.

---

## Design Invariants

- No direct access to another agent's internal ground-truth state.
- Events carry only identifiers / minimal scalar payload; receivers fetch authoritative state at handling time.
- Components are pure data containers (`#[derive(Component, Debug, Reflect, Default)]`).
- All socially meaningful constructs trend toward continuous, parametric representations.
- Constants must migrate toward documented, literature-backed justification.
- Systems remain single-responsibility and side-effect bounded to enable parallel scheduling.

---

## ML Integration Trajectory

- Observation Vector: Concatenate normalized component fields (per-agent) for RL environments.
- Action Hook Points: Desire selection, attention reallocation, social act choice.
- Reward Candidates: Need deltas, reputation gains, reduced uncertainty metrics.
- Future Export: Periodic snapshot serialization for offline trajectory datasets (imitation + RL training).

---

## Contributing Guidelines

Early-phase but aiming for architectural consistency:

- Prefer generalized abstractions (avoid one-off duplicated patterns).
- Use events for inter-domain communication instead of direct cross-query coupling.
- Document any new constant with rationale & (ideally) citation.
- Avoid adding categorical enums for social/emotional states—favor floats/vectors.
- Keep systems small (< ~50 LOC of logic) and clearly named with `_system` suffix when executed by scheduler.

Draft PRs encouraged for architectural discussion before large changes.

---

## License

Creative Commons CC BY-NC-SA 4.0. Non-commercial reuse with attribution & share-alike. For commercial licensing
inquiries, please open an issue.

---

## AI Assistance Notice

Development leverages AI pair tools (GitHub Copilot, GPT-family, Claude, others). All AI-generated material is reviewed
for alignment with project philosophy & data-oriented constraints.

---

## Status Snapshot

Early foundation refactor: migrating toward expression + subjective perception layers, domain plugin encapsulation, and
event-driven cognition pipeline. Expect experimental APIs & rapid change.

---

## Immediate Next Steps

1. Finalize `Needs` ➜ expression modulation mapping (Milestone 1 closure).
2. Implement attention-based perceptual filtering + distortion injection (kickoff Milestone 2).
3. Generalize legacy rumor logic into unified social communication events.
4. Introduce domain plugin structs with registration & reflection for all existing modules.
5. Add debug overlay tracing expression ➜ perception ➜ belief transitions.

---

Questions, research proposals, or collaboration ideas: open an issue (succinct, theory-aligned proposals appreciated).
