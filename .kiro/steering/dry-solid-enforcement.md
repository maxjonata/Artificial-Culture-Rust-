---
inclusion: always
---

# DRY and SOLID Principles Enforcement for Artificial Society

## Core Philosophy: "Principled Code Prevents Technical Debt"

Every line of code must adhere to DRY (Don't Repeat Yourself) and SOLID principles. These are not suggestions - they are hard requirements enforced by CI/CD with zero tolerance for violations.

## DRY (Don't Repeat Yourself) Enforcement

### Code Duplication Detection Rules

#### **Zero Tolerance Thresholds**
- **Functions**: No more than 3 lines of identical code across functions
- **Components**: No duplicate field definitions across structs
- **Systems**: No duplicate query patterns or logic blocks
- **Constants**: All magic numbers must be named constants
- **Type Definitions**: No duplicate type aliases or similar structs

#### **Automated Detection Patterns**
```rust
// ❌ VIOLATION: Duplicate validation logic
impl PersonalityVector {
    pub fn validate_openness(&self) -> Result<(), String> {
        if self.openness.value() < 0.0 || self.openness.value() > 1.0 {
            return Err("Invalid openness value".to_string());
        }
        Ok(())
    }
    
    pub fn validate_conscientiousness(&self) -> Result<(), String> {
        if self.conscientiousness.value() < 0.0 || self.conscientiousness.value() > 1.0 {
            return Err("Invalid conscientiousness value".to_string());
        }
        Ok(())
    }
}

// ✅ CORRECT: DRY validation with generic function
impl PersonalityVector {
    fn validate_trait_range(value: f32, trait_name: &str) -> Result<(), String> {
        if value < 0.0 || value > 1.0 {
            return Err(format!("Invalid {} value: {}", trait_name, value));
        }
        Ok(())
    }
    
    pub fn validate(&self) -> Result<(), String> {
        Self::validate_trait_range(self.openness.value(), "openness")?;
        Self::validate_trait_range(self.conscientiousness.value(), "conscientiousness")?;
        // ... other traits
        Ok(())
    }
}
```

#### **Component Duplication Prevention**
```rust
// ❌ VIOLATION: Duplicate component patterns
#[derive(Component)]
pub struct HungerNeed {
    pub current_level: Normalized<f32>,
    pub decay_rate: f32,
    pub critical_threshold: f32,
}

#[derive(Component)]
pub struct EnergyNeed {
    pub current_level: Normalized<f32>,
    pub decay_rate: f32,
    pub critical_threshold: f32,
}

// ✅ CORRECT: Generic need component
#[derive(Component)]
pub struct Need<T: NeedType> {
    pub current_level: Normalized<f32>,
    pub decay_rate: f32,
    pub critical_threshold: f32,
    pub need_type: PhantomData<T>,
}

pub type HungerNeed = Need<Hunger>;
pub type EnergyNeed = Need<Energy>;
```

#### **System Logic Deduplication**
```rust
// ❌ VIOLATION: Duplicate system patterns
fn hunger_decay_system(
    mut agents: Query<&mut HungerNeed>,
    world_time: Res<WorldTime>,
) {
    for mut hunger in agents.iter_mut() {
        let decay_amount = hunger.decay_rate * world_time.delta_time;
        hunger.current_level = (hunger.current_level.value() + decay_amount).clamp(0.0, 1.0).into();
    }
}

fn energy_decay_system(
    mut agents: Query<&mut EnergyNeed>,
    world_time: Res<WorldTime>,
) {
    for mut energy in agents.iter_mut() {
        let decay_amount = energy.decay_rate * world_time.delta_time;
        energy.current_level = (energy.current_level.value() + decay_amount).clamp(0.0, 1.0).into();
    }
}

// ✅ CORRECT: Generic decay system
fn need_decay_system<T: NeedType + Component>(
    mut agents: Query<&mut Need<T>>,
    world_time: Res<WorldTime>,
) {
    for mut need in agents.iter_mut() {
        let decay_amount = need.decay_rate * world_time.delta_time;
        need.current_level = (need.current_level.value() + decay_amount).clamp(0.0, 1.0).into();
    }
}
```

### Magic Number Elimination
```rust
// ❌ VIOLATION: Magic numbers scattered throughout code
if stress_level > 0.8 {
    // trigger stress response
}
if personality.openness.value() > 0.7 {
    // high openness behavior
}

// ✅ CORRECT: Named constants
pub mod thresholds {
    pub const STRESS_CRITICAL_THRESHOLD: f32 = 0.8;
    pub const HIGH_OPENNESS_THRESHOLD: f32 = 0.7;
    pub const EMOTIONAL_CONTAGION_RADIUS: f32 = 10.0;
    pub const MEMORY_DECAY_RATE: f32 = 0.001;
}
```

## SOLID Principles Enforcement

### Single Responsibility Principle (SRP)

#### **Component Responsibility Rules**
```rust
// ❌ VIOLATION: Component doing too many things
#[derive(Component)]
pub struct Agent {
    pub personality: PersonalityVector,
    pub needs: Vec<f32>,
    pub relationships: HashMap<Entity, f32>,
    pub memory: Vec<MemoryRecord>,
    pub current_action: Option<Action>,
    pub stress_level: f32,
    pub energy: f32,
}

// ✅ CORRECT: Single responsibility components
#[derive(Component)]
pub struct PersonalityVector { /* personality only */ }

#[derive(Component)]
pub struct PhysiologicalNeeds { /* needs only */ }

#[derive(Component)]
pub struct SocialMemory { /* memory only */ }

#[derive(Component)]
pub struct CurrentAction { /* action only */ }
```

#### **System Responsibility Rules**
```rust
// ❌ VIOLATION: System doing multiple unrelated things
fn agent_update_system(
    mut agents: Query<(&mut PersonalityVector, &mut PhysiologicalNeeds, &mut SocialMemory)>,
    world_time: Res<WorldTime>,
) {
    for (mut personality, mut needs, mut memory) in agents.iter_mut() {
        // Updates personality - WRONG RESPONSIBILITY
        // Updates needs - WRONG RESPONSIBILITY  
        // Updates memory - WRONG RESPONSIBILITY
    }
}

// ✅ CORRECT: Separate systems for separate responsibilities
fn personality_development_system() { /* personality only */ }
fn needs_decay_system() { /* needs only */ }
fn memory_decay_system() { /* memory only */ }
```

### Open/Closed Principle (OCP)

#### **Extensible Behavior Systems**
```rust
// ✅ CORRECT: Open for extension, closed for modification
pub trait DecisionMaker {
    fn make_decision(&self, context: &DecisionContext) -> Decision;
}

#[derive(Component)]
pub struct PersonalityBasedDecisionMaker {
    pub personality: PersonalityVector,
}

impl DecisionMaker for PersonalityBasedDecisionMaker {
    fn make_decision(&self, context: &DecisionContext) -> Decision {
        // Personality-based logic
    }
}

#[derive(Component)]
pub struct EmotionBasedDecisionMaker {
    pub emotional_state: EmotionalState,
}

impl DecisionMaker for EmotionBasedDecisionMaker {
    fn make_decision(&self, context: &DecisionContext) -> Decision {
        // Emotion-based logic
    }
}

// System works with any DecisionMaker implementation
fn decision_making_system<T: DecisionMaker + Component>(
    agents: Query<&T>,
    context: Res<DecisionContext>,
) {
    for decision_maker in agents.iter() {
        let decision = decision_maker.make_decision(&context);
        // Process decision
    }
}
```

### Liskov Substitution Principle (LSP)

#### **Behavioral Substitutability**
```rust
// ✅ CORRECT: All need types behave consistently
pub trait NeedBehavior {
    fn decay_rate(&self) -> f32;
    fn critical_threshold(&self) -> f32;
    fn satisfaction_rate(&self) -> f32;
    
    // Contract: decay_rate must be positive
    // Contract: thresholds must be in 0.0-1.0 range
    // Contract: satisfaction_rate must be positive
}

pub struct HungerNeed;
impl NeedBehavior for HungerNeed {
    fn decay_rate(&self) -> f32 { 0.1 } // Positive ✓
    fn critical_threshold(&self) -> f32 { 0.8 } // In range ✓
    fn satisfaction_rate(&self) -> f32 { 0.2 } // Positive ✓
}

pub struct EnergyNeed;
impl NeedBehavior for EnergyNeed {
    fn decay_rate(&self) -> f32 { 0.05 } // Positive ✓
    fn critical_threshold(&self) -> f32 { 0.9 } // In range ✓
    fn satisfaction_rate(&self) -> f32 { 0.15 } // Positive ✓
}
```

### Interface Segregation Principle (ISP)

#### **Focused Trait Interfaces**
```rust
// ❌ VIOLATION: Fat interface forcing unnecessary dependencies
pub trait AgentBehavior {
    fn update_personality(&mut self);
    fn process_social_interaction(&mut self, other: Entity);
    fn update_physiological_needs(&mut self);
    fn make_decision(&self) -> Decision;
    fn update_memory(&mut self);
}

// ✅ CORRECT: Segregated interfaces
pub trait PersonalityUpdater {
    fn update_personality(&mut self);
}

pub trait SocialInteractor {
    fn process_social_interaction(&mut self, other: Entity);
}

pub trait PhysiologicalProcessor {
    fn update_physiological_needs(&mut self);
}

pub trait DecisionMaker {
    fn make_decision(&self) -> Decision;
}

pub trait MemoryManager {
    fn update_memory(&mut self);
}
```

### Dependency Inversion Principle (DIP)

#### **Abstraction-Based Dependencies**
```rust
// ❌ VIOLATION: High-level module depending on low-level concrete implementation
pub struct SocialInteractionSystem {
    pub memory_storage: FileBasedMemoryStorage, // Concrete dependency
}

impl SocialInteractionSystem {
    pub fn process_interaction(&mut self, interaction: SocialInteraction) {
        // Directly uses FileBasedMemoryStorage methods
        self.memory_storage.store_interaction(interaction);
    }
}

// ✅ CORRECT: Depend on abstractions
pub trait MemoryStorage {
    fn store_interaction(&mut self, interaction: SocialInteraction);
    fn retrieve_interactions(&self, entity: Entity) -> Vec<SocialInteraction>;
}

pub struct SocialInteractionSystem<T: MemoryStorage> {
    pub memory_storage: T, // Abstract dependency
}

impl<T: MemoryStorage> SocialInteractionSystem<T> {
    pub fn process_interaction(&mut self, interaction: SocialInteraction) {
        // Uses abstract interface
        self.memory_storage.store_interaction(interaction);
    }
}
```

## CI/CD Enforcement Rules

### Pre-Commit Validation
- **Code Duplication Scanner**: Fails build if >3 lines duplicated
- **SOLID Principle Checker**: Validates component/system responsibilities
- **Magic Number Detector**: Fails if unnamed constants found
- **Interface Bloat Detector**: Fails if traits have >5 methods
- **Dependency Direction Validator**: Ensures abstractions don't depend on concretions

### Build-Time Enforcement
- **Clippy Rules**: Custom lints for DRY/SOLID violations
- **Macro Validation**: Ensures generated code follows principles
- **Documentation Requirements**: All public APIs must document their single responsibility
- **Test Coverage**: Each responsibility must have dedicated tests

### Runtime Validation
- **Performance Impact**: DRY violations that hurt performance fail benchmarks
- **Behavioral Consistency**: SOLID violations that break AI behavior fail behavioral tests
- **Memory Efficiency**: Code duplication that wastes memory fails memory tests

## Violation Consequences

### Immediate Build Failure
- Any DRY violation detected by static analysis
- Any SOLID principle violation in component design
- Any magic number without named constant
- Any interface with >5 methods
- Any concrete dependency in high-level modules

### Performance Regression
- Code duplication that increases binary size >1%
- SOLID violations that hurt cache performance
- Interface bloat that increases compilation time >5%

### Behavioral Impact
- DRY violations that create inconsistent AI behavior
- SOLID violations that make personality traits unpredictable
- Interface violations that break system modularity

## Refactoring Guidelines

### DRY Refactoring Process
1. **Identify Duplication**: Use AST analysis to find similar code blocks
2. **Extract Common Logic**: Create generic functions/traits for shared behavior
3. **Parameterize Differences**: Use generics/traits to handle variations
4. **Validate Behavior**: Ensure refactoring doesn't change AI behavior
5. **Performance Test**: Confirm refactoring doesn't hurt performance

### SOLID Refactoring Process
1. **Analyze Responsibilities**: Map each component/system to its single purpose
2. **Split Violations**: Break multi-responsibility components into focused ones
3. **Create Abstractions**: Define traits for extensibility points
4. **Invert Dependencies**: Make high-level modules depend on abstractions
5. **Validate Integration**: Ensure refactored code maintains system behavior

## Success Metrics

### Code Quality Metrics
- **Duplication Ratio**: <2% of codebase (measured by lines)
- **Component Cohesion**: Each component has single, clear responsibility
- **System Coupling**: Systems communicate only through events/abstractions
- **Interface Segregation**: No interface has >5 methods
- **Dependency Direction**: 100% of dependencies point toward abstractions

### Performance Metrics
- **Binary Size**: DRY enforcement reduces size by >15%
- **Compilation Time**: SOLID design reduces incremental compilation by >20%
- **Runtime Performance**: Principled code maintains 60fps target
- **Memory Usage**: Efficient abstractions don't increase memory >5%

### Maintainability Metrics
- **Change Impact**: Single responsibility changes affect <3 files
- **Extension Ease**: New behaviors added without modifying existing code
- **Test Isolation**: Each responsibility tested independently
- **Documentation Clarity**: Each component's purpose clear from docs

This enforcement ensures that the Artificial Society codebase remains maintainable, performant, and aligned with software engineering best practices while supporting the complex AI behaviors required for believable social simulation.