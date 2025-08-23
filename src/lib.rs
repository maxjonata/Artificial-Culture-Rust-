//! Artificial Society - Agent-Based AI Simulation
//!
//! This crate implements a comprehensive agent-based simulation system using the Bevy Engine.
//! The simulation models complex, emergent social dynamics through the interaction of intelligent
//! agents with sophisticated internal cognitive architectures.
//!
//! # Architecture Overview
//!
//! The system is built on two key paradigms:
//!
//! ## 1. The Agent's Internal World (Micro-level): "Brain as a Society of Systems"
//! Each agent's mind is a society of competing and cooperating sub-systems (physiological needs,
//! emotional responses, rational planning). Behavior emerges from the interaction of these
//! internal systems.
//!
//! ## 2. The External World (Macro-level): "Society of Agents"
//! Complex, large-scale social phenomena emerge from simple, local interactions between many
//! individual agents. The global outcome is not scripted but emerges from bottom-up interactions.
//!
//! # Core Modules
//!
//! - [`ai`] - Agent intelligence systems (cognition, physiology, perception, social)
//! - [`core`] - Core game systems and utilities
//! - [`presentation`] - Visualization, debugging, and performance monitoring
//! - [`utils`] - Shared utilities and helper functions
//!
//! # Performance Monitoring
//!
//! The crate includes a comprehensive performance monitoring and alerting system that tracks:
//! - System resource usage (CPU, Memory, GPU)
//! - Frame performance metrics (FPS, frame time, jitter)
//! - AI system performance budgets
//! - Entity scaling performance correlation
//!
//! See [`presentation::performance_alerts`] for details.
//!
//! # Testing
//!
//! The crate follows a comprehensive testing strategy with:
//! - Unit tests for individual components
//! - Integration tests for system interactions
//! - Mock systems for controlled testing environments
//! - Performance benchmarks and regression tests
//!
//! Tests are organized in the [`tests`] module and can be run with `cargo test`.

pub mod ai;
pub mod core;
pub mod presentation;
pub mod utils;

#[cfg(test)]
pub mod tests;
