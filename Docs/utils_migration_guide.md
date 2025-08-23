# Utils Folder Migration Guide

## Overview

This document outlines the migration of utilities from the generic `src/utils/` folder to their specific usage locations, following the **component-private functionality principle** established by the generic type-safe builder system.

## Migration Rationale

The migration follows these principles:
1. **Component-Private Functionality**: Utilities should be embedded directly where they're used
2. **Domain-Driven Architecture**: Code should be organized by functional domain, not by code type
3. **Reduced Cross-Module Dependencies**: Minimize imports and coupling between modules
4. **Type-Safe Integration**: Utilities should be methods on the types they operate on

## Migration Summary

### ✅ **Migrated Utilities**

#### 1. **Type Registration → Core Module**
- **From**: `src/utils/helpers/overrides.rs` - `AppRegisterTypesExt`
- **To**: `src/core/mod.rs` - `CoreTypeRegistration` trait
- **Rationale**: Only used in core module for registering core types

**Before:**
```rust
use crate::utils::helpers::overrides::AppRegisterTypesExt;

app.register_types::<(GameConstants, Npc, Normalized)>();
```

**After:**
```rust
// Embedded directly in core module
app.register_core_types();
```

#### 2. **Big Five Conversion → Normalized Type**
- **From**: `src/utils/helpers/types.rs` - `big_five_to_normalized()`
- **To**: `src/core/types.rs` - `Normalized::from_big_five()`
- **Rationale**: Domain-specific utility for Normalized type

**Before:**
```rust
use crate::utils::helpers::types::big_five_to_normalized;

let openness = big_five_to_normalized(4.2);
```

**After:**
```rust
let openness = Normalized::from_big_five(4.2);
```

#### 3. **Validation Utilities → Normalized Type**
- **From**: `src/utils/helpers/types.rs` - `validate_normalized()`, `assert_range()`
- **To**: `src/core/types.rs` - `Normalized::validate_raw()`, `Normalized::assert_range()`
- **Rationale**: Primarily used for validating values before normalization

**Before:**
```rust
use crate::utils::helpers::types::{validate_normalized, assert_range};

validate_normalized(value, "mood");
assert_range(temp, "temperature", 0.0..=100.0);
```

**After:**
```rust
Normalized::validate_raw(value, "mood");
Normalized::assert_range(temp, "temperature", 0.0..=100.0);
```

#### 4. **Component Registration → Builder System**
- **From**: `src/utils/macros/mod.rs` - Registration macros
- **To**: `src/core/builders.rs` - `ComponentTupleRegistration` trait
- **Rationale**: Specific to enhanced components with behaviors

**Before:**
```rust
// Complex macro system in utils
impl_register_types_tuple!(A, B, C);
```

**After:**
```rust
// Embedded in builder system
app.register_components_with_behaviors::<(ComponentA, ComponentB)>();
```

### 🗑️ **Removed Utilities**

#### 1. **Generic Registration Macros**
- **Files**: `src/utils/macros/mod.rs`, `src/utils/helpers/mod.rs`
- **Reason**: Replaced by domain-specific registration methods
- **Impact**: No breaking changes - functionality preserved in new locations

#### 2. **Unused Event Registration**
- **Code**: `AppRegisterEventsExt` trait
- **Reason**: Never used in production code (marked with `#[allow(unused)]`)
- **Impact**: No impact - was unused

### 📁 **Updated File Structure**

**Before Migration:**
```
src/utils/
├── mod.rs
├── helpers/
│   ├── mod.rs (macro instantiations)
│   ├── overrides.rs (registration traits)
│   └── types.rs (conversion utilities)
└── macros/
    └── mod.rs (registration macros)
```

**After Migration:**
```
src/utils/
└── (empty - ready for removal)

src/core/
├── mod.rs (+ CoreTypeRegistration)
├── types.rs (+ conversion methods)
└── builders.rs (+ ComponentTupleRegistration)
```

## Migration Benefits

### 1. **Improved Type Safety**
- Utilities are now methods on the types they operate on
- Compile-time guarantees for correct usage
- Better IDE support and discoverability

### 2. **Reduced Coupling**
- No more cross-module imports for simple utilities
- Domain-specific functionality stays in its domain
- Cleaner dependency graph

### 3. **Better Performance**
- Utilities are now `#[inline]` methods on types
- Zero-cost abstractions with optimal compilation
- No function call overhead for simple operations

### 4. **Enhanced Maintainability**
- Related functionality is co-located
- Easier to find and modify domain-specific code
- Follows established Rust patterns (methods on types)

## Testing Updates

All tests have been updated to use the new locations:

```rust
// Updated test imports
use crate::core::types::Normalized;

// Updated test calls
let score = Normalized::from_big_five(4.0);
assert!(Normalized::validate_raw(0.5, "test"));
```

**Test Results**: All existing tests pass with the new implementations.

## Breaking Changes

### ⚠️ **For External Users**

If you were importing utilities from `src/utils/`, update your imports:

```rust
// OLD - will no longer work
use artificial_culture_rust::utils::helpers::types::big_five_to_normalized;

// NEW - use type methods
use artificial_culture_rust::core::types::Normalized;
let value = Normalized::from_big_five(score);
```

### ✅ **For Internal Code**

All internal code has been updated automatically. No manual changes required.

## Future Utils Policy

Going forward, utilities should be:

1. **Type Methods**: If the utility operates on a specific type, make it a method on that type
2. **Domain-Specific**: If the utility is used in only one domain, embed it in that domain
3. **Trait Methods**: If the utility extends existing functionality, use extension traits
4. **Generic Only**: Only keep utilities in `src/utils/` if they are truly generic and used across multiple domains

## Verification

To verify the migration was successful:

```bash
# All tests should pass
cargo test

# No unused imports warnings
cargo check

# Utils folder should be empty (ready for removal)
ls src/utils/
```

## Next Steps

1. **Remove Empty Utils Folder**: After verification, the `src/utils/` folder can be removed
2. **Update Documentation**: Update any documentation that references the old utility locations
3. **Monitor Usage**: Watch for any new utilities being added to ensure they follow the new patterns

This migration successfully implements the component-private functionality principle, reducing boilerplate and improving the overall architecture of the Artificial Society simulation.
