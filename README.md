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
- Data Shape: Type-safe `Normalized` values for all [0.0, 1.0] ranges; continuous numeric fields for ML integration.
- ECS Style: Domain-based plugin architecture + event-driven systems (`Entity` IDs in events; fresh lookup via
  `query.get`).
- Type Safety: Zero-cost `Normalized` wrapper prevents invalid states while maintaining f32 performance.
- Test Organization: Comprehensive test suite in `src/tests/` organized by domain.
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

This repository implements the foundational architecture with a type-safe `Normalized` wrapper system, comprehensive
domain-based plugin architecture, and organized test structure. The four-layer communication pipeline is being
built on this solid foundation with strict separation between current and historical approaches.

---

## Foundational Principles

1. **Mantle of Ignorance**: No omniscience; perception components are the only ingress of external world data.
2. **Type Safety**: `Normalized` wrapper enforces [0.0, 1.0] bounds at compile-time, preventing invalid states.
3. **Event-Driven Reactivity**: Prefer events over polling (neuron metaphor; cache-friendly targeted lookups).
4. **Domain Plugin Architecture**: Each AI domain (cognition, perception, physiology, social) is a self-contained plugin.
5. **Equality of Potential**: Identical cognitive substrate across all agents; divergence via learned/experienced weights.
6. **Data-Oriented Design**: Type-safe numeric fields; minimized branching; zero-cost abstractions.
7. **Continuous Social State**: Emotions/attitudes/intent as continuous `Normalized` vectors, not enums.
8. **Test-Driven Development**: Comprehensive test suite in `src/tests/` organized by domain.
9. **Documentation Hierarchy**: Clear separation between current specification and historical approaches.

---

## Current Milestone Focus

**Foundation Phase Complete**: Type-safe architecture with `Normalized` wrapper system and comprehensive plugin structure.

**Current Status**: Foundation phase with critical gaps identified:

- ✅ **Type-Safe Foundation**: `Normalized` wrapper prevents invalid states, comprehensive arithmetic operations
- ✅ **Plugin Architecture**: Domain-based plugins (AI, Core, World, Presentation) with proper separation
- ✅ **Debug Infrastructure**: bevy_inspector_egui integration and performance monitoring systems
- ⚠️ **Missing Core Systems**: WorldTime, functional entity spawning, AI system implementations
- ⚠️ **Unused Components**: PersonalityVector, social components exist but aren't integrated
- 🔄 **Next Priority**: Implement WorldTime resource and fix entity spawning system

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
  main.rs                    # Plugin aggregator
  ai/
    mod.rs                   # AiPlugin - orchestrates all AI systems
    cognition/mod.rs         # AiCognitionPlugin - decision making, memory, beliefs
    perception/mod.rs        # AiPerceptionPlugin - sensory processing, world model
    physiology/mod.rs        # AiPhysiologyPlugin - needs, stress, homeostasis
    social/mod.rs            # AiSocialPlugin - communication, relationships
  core/
    mod.rs                   # CorePlugin - foundational systems
    constants.rs             # GameConstants with Normalized values
    entities.rs              # Entity markers and classifications
    types.rs                 # Normalized type and utilities
    spawning.rs              # Entity creation systems
  presentation/
    mod.rs                   # PresentationPlugin (planned)
    fps_display.rs           # Debug overlays
  utils/
    mod.rs                   # Utility aggregation
    helpers/                 # Generic helper modules
    macros/                  # Type registration macros (clamped_setters removed)
  world/
    mod.rs                   # WorldPlugin (planned)
  tests/                     # ✅ NEW: Organized test structure
    mod.rs                   # Test module declarations
    core/
      mod.rs                 # Core module tests
      types.rs               # Normalized type tests (12 tests passing)
Docs/
  Fundaments/              # ✅ PRIMARY SOURCES (authoritative)
    Tech Phylosophy Spec.md # Main technical and philosophical specification
    Tech Paper Spec.md     # Technical paper specification
    Roadmap from Spec.md   # Implementation roadmap
  Backstory/               # ⚠️ HISTORICAL (reference only)
    DETAILED_ROADMAP.md    # Legacy detailed roadmap
    components.md          # Historical ECS design
    systems.md             # Legacy system architecture
    neurological.md        # Historical research notes
    psychological.md       # Historical research notes
    sociological.md        # Historical research notes
  Structures/              # ⚠️ HISTORICAL (reference only)
    normalized_type_guide.md # Type system implementation guide
  Flows/                   # ⚠️ HISTORICAL (reference only - empty)
  Papers/                  # ✅ RESEARCH SOURCES (parameter values)
```

---

## Domain Modules (Technical Detail)

### Entry Layer

- [src/main.rs](src/main.rs): Application bootstrap; composes domain plugins with clean plugin architecture.
- [src/ai/mod.rs](src/ai/mod.rs): `AiPlugin` - orchestrates all AI subsystems (cognition, perception, physiology, social).

### Cognition

- [cognition/mod.rs](src/ai/cognition/mod.rs): `AiCognitionPlugin` - executive functions, decision-making, memory systems.
  - **Architecture**: Dual-process theory, working memory model, Bayesian belief updating
  - **Components**: WorkingMemory, LongTermMemory, BeliefSystem, DecisionMaker, GoalPlanner
  - **Integration**: Receives perception input, considers physiological drives, outputs behavioral decisions

### Perception

- [perception/mod.rs](src/ai/perception/mod.rs): `AiPerceptionPlugin` - sensory processing and world model construction.
  - **Architecture**: Hierarchical predictive processing, Bayesian perception, attention filtering
  - **Components**: VisionSystem, HearingSystem, WorldModel, AttentionFilter, PerceptualMemory
  - **Mantle of Ignorance**: Enforces information scarcity - agents only access world through sensory channels

### Physiology

- [physiology/mod.rs](src/ai/physiology/mod.rs): `AiPhysiologyPlugin` - biological drives and homeostatic regulation.
  - **Architecture**: Allostatic load model, circadian rhythms, stress response systems
  - **Components**: BasicNeeds, StressSystem, HealthStatus, CircadianClock, Metabolism
  - **Integration**: Provides motivational drives, affects perception through stress, generates emotional responses

### Social

- [social/mod.rs](src/ai/social/mod.rs): `AiSocialPlugin` - interpersonal relationships and communication.
  - **Architecture**: Social Identity Theory, Theory of Mind, Dunbar's number constraints
  - **Components**: SocialCognition, RelationshipTracker, CommunicationSystem, GroupMembership
  - **Emergent Phenomena**: Social networks, cultural evolution, collective intelligence, status hierarchies



### Core (Cross-Domain Infrastructure)

- [core/mod.rs](src/core/mod.rs): `CorePlugin` - foundational systems and type registration.
- [types.rs](src/core/types.rs): **NEW** - `Normalized` type-safe wrapper with comprehensive arithmetic operations.
- [constants.rs](src/core/constants.rs): `GameConstants` with type-safe `Normalized` values.
- [entities.rs](src/core/entities.rs): Shared marker & classification components (type tags for queries).
- [spawning.rs](src/core/spawning.rs): Runtime entity spawning systems.

### Presentation / Debug (Non-AI Logic)

- [presentation/mod.rs](src/presentation/mod.rs): (Planned `PresentationPlugin`).
- [fps_display.rs](src/presentation/fps_display.rs): FPS overlay.

### Utilities

- [utils/mod.rs](src/utils/mod.rs): Aggregated helper exports.
- [utils/helpers](src/utils/helpers): Directory containing generic helper modules.
- [utils/macros](src/utils/macros): Type registration macros (deprecated `clamped_setters!` removed).

### Tests

- [tests/mod.rs](src/tests/mod.rs): Test module organization by domain.
- [tests/core/types.rs](src/tests/core/types.rs): Comprehensive `Normalized` type tests (12 tests passing).

### World Assembly

- [world/mod.rs](src/world/mod.rs): Spatial/world scaffolding (future resource nodes & zone partition registration).

### Documentation (Updated Hierarchy)

**✅ CURRENT (Authoritative):**
- [Main Specification](Docs/Fundaments/Artificial%20Society_%20Complete%20Technical%20and%20Philosophical%20Specification.md)
- [Implementation Roadmap](Docs/Fundaments/Roadmap%20from%20Spec.md)
- [Normalized Type Guide](docs/normalized_type_guide.md)

**⚠️ HISTORICAL (Reference Only):**
- [Components Catalog](Docs/Structures/components.md) - Historical ECS design
- [Systems Overview](Docs/Structures/systems.md) - Legacy system architecture
- [Interaction Flows](Docs/Flows/interaction_flows.md) - Early flow planning

---

## Milestone Roadmap

| Phase                      | Focus                                  | Key Deliverables                                     | Status      |
|----------------------------|----------------------------------------|------------------------------------------------------|-------------|
| **Phase 1: Foundation**    | **Core infrastructure & WorldTime**   | **WorldTime resource, entity spawning, type system** | **⚠️ Partial** |
| **Phase 2: Physiological** | **Needs, stress, energy systems**     | **Basic biological drives and mood integration**     | **❌ Not Started** |
| **Phase 3: Performance**   | **Memory optimization, parallel processing** | **60fps with 100+ agents, LOD system**        | **❌ Not Started** |
| **Phase 4: Cognitive**     | **Decision-making, memory, learning** | **Personality-driven behavior, dual-process cognition** | **❌ Not Started** |
| **Phase 5: Social**        | **"Plato's Cave" communication**      | **Four-layer pipeline, believable misunderstandings** | **❌ Not Started** |
| **Phase 6: Social Dynamics** | **Relationships, group formation**   | **Emergent social behaviors, reputation systems**   | **❌ Not Started** |
| **Phase 7: Testing**       | **Behavioral validation, drift detection** | **"Social Turing Test" achievement**          | **❌ Not Started** |
| **Phase 8: Configuration** | **Runtime tuning, A/B testing**       | **Parameter optimization, scenario templates**      | **❌ Not Started** |
| **Phase 9: Persistence**   | **Cross-server sync, data integrity** | **World persistence, agent migration**              | **❌ Not Started** |
| **Phase 10: Production**   | **Master integration, deployment**    | **Production-ready AI society simulation**          | **❌ Not Started** |

Detailed acceptance criteria: see spec sections on milestones.

---

## Prototype Features

**✅ Currently Working:**

- **Type-Safe Foundation**: `Normalized` wrapper with comprehensive arithmetic operations and validation
- **Plugin Architecture**: Domain-based structure (AI, Core, World, Presentation) with proper separation
- **Debug Infrastructure**: bevy_inspector_egui integration and performance monitoring systems
- **Physics Integration**: Rapier2D physics system with environment management
- **Project Structure**: Proper Rust project setup with comprehensive type safety

**⚠️ Critical Issues:**

- **Non-functional Entity Spawning**: Current spawning system is completely commented out
- **Empty AI Systems**: All AI domain plugins are skeleton implementations with no functionality
- **Missing WorldTime**: No temporal coordination system for time scaling and consistency
- **Unused Components**: PersonalityVector and social components exist but aren't integrated

**🔄 Immediate Development Priorities:**

1. **Implement WorldTime resource** with time scaling support (1x to 1000x speed)
2. **Fix entity spawning system** to create functional agents with proper components
3. **Create basic Needs component** with hunger, energy, safety, social needs
4. **Connect personality system** to actual behavioral modulation
5. **Follow Master Implementation Plan** for systematic development

**🔮 Future Development:**
- Complete physiological foundation (needs, stress, energy, mood)
- Implement cognitive architecture (decision-making, memory, learning)
- Build "Plato's Cave" social communication pipeline
- Add performance optimization for 100+ agents at 60fps
- Create behavioral validation and "Social Turing Test" achievement

---

## Type-Safe Architecture Highlights

### `Normalized` Wrapper System

The project now uses a comprehensive type-safe `Normalized` wrapper for all [0.0, 1.0] values:

```rust
// OLD: Manual validation prone to errors
pub struct Needs {
    pub hunger: f32,  // Could be set to invalid values
}
needs.hunger = 2.0;  // Compiles but invalid!

// NEW: Type-safe with automatic bounds enforcement
pub struct Needs {
    pub hunger: Normalized,  // Cannot store invalid values
}
needs.hunger += 2.0;  // Automatically clamped to 1.0
```

**Benefits:**
- **Zero-cost abstraction**: Same memory layout as f32
- **Compile-time safety**: Invalid states are impossible
- **Automatic clamping**: All arithmetic operations maintain bounds
- **Bevy integration**: Full Component and Reflect support
- **Comprehensive tests**: 12 tests covering all operations

### Removed Deprecated Code

- ❌ **`clamped_setters!` macro**: Replaced by type-safe `Normalized` fields
- ❌ **Manual validation utilities**: `quantized_to_float`, `float_to_quantized` (now type methods)
- ❌ **Legacy utility functions**: Replaced by `Normalized` arithmetic operations
- ✅ **Kept validation macros**: For testing and external input validation

---

## Install & Run

Prerequisites: Recent Rust toolchain (Rust 2024 edition compatible with Bevy 0.16.x).

```bash
git clone https://github.com/maxjonata/Artificial-Culture-Rust-.git
cd Artificial-Culture-Rust-
cargo run --release
```

**⚠️ Current State**: The application will run but no agents will spawn (spawning system is commented out). You'll see:
- Empty world with physics environment
- Debug UI (bevy_inspector_egui) for component inspection
- Performance monitoring systems
- Plugin architecture loading correctly

**Next Steps**: Follow the [Master Implementation Plan](.kiro/specs/master-implementation-plan.md) to implement missing systems.

Use `--release` for stable frame pacing. If dynamic linking issues arise on your platform, remove the `dynamic_linking`
feature in `Cargo.toml` and rebuild.

---

## Runtime Usage

**Current State**: Foundation systems only - no agents spawn yet.

**What Works:**
- Debug UI (EGUI) for component inspection
- Performance monitoring and alerting systems
- Physics environment with Rapier2D integration
- Plugin architecture loading and coordination

**What's Missing:**
- Agent spawning (system is commented out)
- AI behaviors (all domain plugins are empty)
- WorldTime resource for temporal coordination
- Functional personality and needs systems

**Development Workflow:**
1. Use the debug UI to inspect existing components
2. Follow the [Master Implementation Plan](.kiro/specs/master-implementation-plan.md) for systematic development
3. Implement WorldTime resource first (enables all other systems)
4. Fix entity spawning to create functional agents
5. Add basic physiological systems (needs, stress, mood)

---

## Documentation Map

**⚠️ IMPORTANT: Documentation Hierarchy**

**PRIMARY SOURCES (Authoritative for Current Development):**
- **Main Specification**: `Docs/Fundaments/Tech Phylosophy Spec.md` - Core technical and philosophical specification
- **Technical Paper**: `Docs/Fundaments/Tech Paper Spec.md` - Technical implementation specification
- **Implementation Roadmap**: `Docs/Fundaments/Roadmap from Spec.md` - Current implementation roadmap
- **Master Implementation Plan**: `.kiro/specs/master-implementation-plan.md` - Unified task coordination

**RESEARCH SOURCES (For Parameter Values & Design Inspiration):**
- **Academic Papers**: `Docs/Papers/` - Use for realistic parameter values and behavioral patterns, NOT for achieving scientific perfection

**HISTORICAL/BACKSTORY DOCUMENTATION (Reference Only - Not for Current Development):**
- **Legacy Roadmap**: `Docs/Backstory/DETAILED_ROADMAP.md` - Historical detailed roadmap
- **ECS Structures**: `Docs/Backstory/components.md`, `systems.md` - Historical ECS design approaches
- **Research Notes**: `Docs/Backstory/neurological.md`, `psychological.md`, `sociological.md` - Historical research notes
- **Type Guide**: `Docs/Structures/normalized_type_guide.md` - Type system implementation guide
- **Flows**: `Docs/Flows/` - Legacy planning documents (empty)

**⚠️ AI Development Notice**: AI assistants should ONLY use the Primary Sources for current development guidance. All other documentation represents historical approaches and lessons learned, preserved for reference but not for active development.

---

## Design Invariants

- **Type Safety**: All [0.0, 1.0] values use `Normalized` wrapper to prevent invalid states at compile-time.
- **Mantle of Ignorance**: No direct access to another agent's internal ground-truth state.
- **Event-Driven**: Events carry only identifiers; receivers fetch authoritative state at handling time.
- **Pure Components**: Components are data containers (`#[derive(Component, Debug, Reflect, Default)]`).
- **Continuous Representation**: Social/emotional constructs as `Normalized` vectors, not enums.
- **Domain Plugins**: Each AI domain (cognition, perception, physiology, social) is self-contained.
- **Test Coverage**: All core functionality has comprehensive tests in `src/tests/`.
- **Documentation Hierarchy**: Clear separation between current specification and historical approaches.

---

## ML Integration Trajectory

- Observation Vector: Concatenate normalized component fields (per-agent) for RL environments.
- Action Hook Points: Desire selection, attention reallocation, social act choice.
- Reward Candidates: Need deltas, reputation gains, reduced uncertainty metrics.
- Future Export: Periodic snapshot serialization for offline trajectory datasets (imitation + RL training).

---

## Contributing Guidelines

Foundation established with architectural consistency:

- **Type Safety First**: Use `Normalized` for all [0.0, 1.0] values instead of raw f32.
- **Plugin Architecture**: Each domain is a self-contained plugin with comprehensive documentation.
- **Test-Driven**: All new functionality requires tests in `src/tests/` organized by domain.
- **Event-Driven**: Use events for inter-domain communication instead of direct coupling.
- **Continuous Values**: Use `Normalized` vectors for social/emotional states, not enums.
- **Documentation**: Follow the established hierarchy (current vs. historical approaches).
- **Small Systems**: Keep systems focused (< ~50 LOC) with clear `_system` suffix naming.

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

## 🔍 Current Implementation Status

### ✅ **Foundation Complete**
- **Core Type System**: `Normalized<f32>` and `Severity` types with full arithmetic operations
- **Plugin Architecture**: Domain-separated structure (AI, Core, World, Presentation)
- **Debug Infrastructure**: bevy_inspector_egui integration and performance monitoring
- **Physics Integration**: Rapier2D physics system with environment management
- **Project Structure**: Proper Rust project with comprehensive type safety

### ⚠️ **Critical Gaps Identified**
- **WorldTime Resource**: Missing - required for temporal consistency across all systems
- **Entity Spawning**: Current spawning system is commented out and non-functional
- **AI System Implementation**: All AI domains are skeleton plugins with no functionality
- **Component Integration**: PersonalityVector exists but isn't used in any systems

### 🚧 **Major Missing Systems**
- **No Physiological Systems**: Needs, stress, energy systems missing
- **No Cognitive Systems**: Decision-making, memory, learning missing  
- **No Social Systems**: Communication pipeline completely missing
- **No Performance Optimization**: LOD, parallel processing not implemented
- **No Testing Framework**: Behavioral validation systems missing

### 🎯 **Immediate Action Required**
1. **Implement WorldTime resource** (enables all temporal systems)
2. **Fix entity spawning system** (enables agent creation)
3. **Implement basic Needs component** (enables physiological foundation)
4. **Create functional personality system** (enables behavioral differences)

### 🔧 **Code Issues to Fix**
- **Unused Components**: PersonalityVector, RoleAffinities, AgentEvent not used anywhere
- **Spawning System**: All entity creation code is commented out
- **AI Plugins**: All AI domain plugins are empty skeletons
- **Missing Dependencies**: Need to add WorldTime, temporal systems
- **Type Usage**: Severity type implemented but never used

---

## 📋 **Master Implementation Plan**

A comprehensive [Master Implementation Plan](.kiro/specs/master-implementation-plan.md) has been created that:
- **Coordinates all 9 spec domains** into a unified roadmap
- **Shows current implementation status** with clear progress indicators
- **Provides dependency-ordered phases** for systematic development
- **Includes success metrics** for each development phase
- **Offers flexible timeline** focused on quality over speed

**Current Phase**: Phase 1 - Foundation Systems (Core Infrastructure)
**Next Phase**: Phase 2 - Physiological Foundation (Biological Simulation Layer)

---

## Immediate Next Steps

1. **🔄 PRIORITY**: Implement WorldTime resource with time scaling support
2. **🔄 PRIORITY**: Fix entity spawning system to create functional agents
3. **🔄 PRIORITY**: Create basic Needs component with decay systems
4. **🔄 PRIORITY**: Connect existing PersonalityVector to actual behavior systems
5. **📋 PLANNED**: Follow Master Implementation Plan phases for systematic development

---

Questions, research proposals, or collaboration ideas: open an issue (succinct, theory-aligned proposals appreciated).
