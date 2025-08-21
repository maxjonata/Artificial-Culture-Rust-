# ⚠️ CURRENT IMPLEMENTATION - ALIGNED WITH MAIN SPECIFICATION

**Status**: This document provides current test organization guidance and is aligned with the main specification.

**For Current Development**: This guide supports the **Primary Documentation Sources**:
- `Docs/Fundaments/Artificial Society_ Complete Technical and Philosophical Specification.md`
- `Docs/Fundaments/DETAILED_ROADMAP.md`

---

# Test Organization

This directory contains all tests for the Artificial Society simulation, organized by domain to match the main codebase structure.

## Structure

```
src/tests/
├── mod.rs                    # Test module declarations
├── README.md                 # This file
└── core/
    ├── mod.rs               # Core module test declarations
    └── types.rs             # Tests for core type definitions
```

## Test Categories

### Core Types Tests (`src/tests/core/types.rs`)

#### `normalized_tests` - Type-safe Normalized wrapper
- ✅ `test_normalized_construction` - Safe construction with clamping
- ✅ `test_normalized_arithmetic` - Arithmetic operations with bounds
- ✅ `test_normalized_constants` - Predefined constants (ZERO, ONE, HALF, etc.)
- ✅ `test_quantized_conversion` - Conversion to/from u8 quantized values
- ✅ `test_normalized_assignment_operations` - Assignment operators (+=, -=, etc.)
- ✅ `test_normalized_try_new` - Checked construction with Result
- ✅ `test_normalized_lerp` - Linear interpolation
- ✅ `test_normalized_clamp` - Value clamping between bounds
- ✅ `test_normalized_division_by_zero` - Safe division by zero handling
- ✅ `test_normalized_from_conversions` - Type conversions (u8 ↔ f32)

#### `validation_tests` - Legacy validation utilities
- ✅ `test_assert_range_function` - Generic range validation
- ✅ `test_assert_range_panic` - Range validation panic behavior

#### `utility_tests` - Utility functions
- ✅ `test_big_five_to_normalized` - Big Five personality scale conversion

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test test_normalized        # All Normalized type tests
cargo test validation_tests       # Validation utility tests
cargo test utility_tests         # Utility function tests

# Run individual tests
cargo test test_normalized_construction
cargo test test_big_five_to_normalized
```

## Test Results Summary

- **Total Tests**: 12 tests
- **Normalized Type Tests**: 9 tests ✅
- **Validation Tests**: 2 tests ✅
- **Utility Tests**: 1 test ✅
- **All Passing**: ✅

## Adding New Tests

When adding new functionality, create tests in the appropriate domain directory:

1. **Core functionality** → `src/tests/core/`
2. **AI systems** → `src/tests/ai/` (create when needed)
3. **World systems** → `src/tests/world/` (create when needed)
4. **Presentation** → `src/tests/presentation/` (create when needed)

Follow the existing pattern:
- Group related tests in `mod` blocks
- Use descriptive test names
- Include both positive and negative test cases
- Test edge cases (division by zero, out-of-bounds values, etc.)

## Test Philosophy

Tests should verify:
1. **Type Safety** - Invalid states are impossible
2. **Bounds Enforcement** - Values stay within valid ranges
3. **Performance** - Zero-cost abstractions work as expected
4. **Scientific Accuracy** - Calculations match expected behavior
5. **Error Handling** - Graceful handling of edge cases
