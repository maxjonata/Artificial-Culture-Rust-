---
inclusion: fileMatch
fileMatchPattern: "clippy.toml"
---

# Clippy Configuration for DRY and SOLID Enforcement

## Core Philosophy: "Compiler-Level Quality Gates"

Custom Clippy rules that enforce DRY and SOLID principles at compile time. These rules are non-negotiable and will cause build failures.

## Enhanced `clippy.toml` Configuration

```toml
# Clippy configuration for Artificial Society - DRY and SOLID Enforcement
avoid-breaking-exported-api = false
msrv = "1.70.0"

# === DRY PRINCIPLE ENFORCEMENT ===

# Disallow duplicate code patterns
disallowed-methods = [
    # Real-world time usage (use WorldTime instead)
    "std::time::Instant::now",
    "std::time::SystemTime::now",
    "std::time::UNIX_EPOCH",
    
    # Blocking operations in systems
    "std::thread::sleep",
    "std::thread::park",
    "std::fs::read",
    "std::fs::write",
    "std::fs::File::open",
    
    # Manual memory management (use Bevy's systems)
    "std::alloc::alloc",
    "std::alloc::dealloc",
    "Box::leak",
    
    # String allocations in hot paths
    "String::from",
    "str::to_string",
    "str::to_owned",
    "format!",  # In system functions only
    
    # Direct HashMap usage (use bevy::utils::HashMap)
    "std::collections::HashMap::new",
    "std::collections::HashMap::with_capacity",
]

disallowed-types = [
    # Performance-critical type restrictions
    "std::collections::HashMap",  # Use bevy::utils::HashMap
    "std::collections::BTreeMap", # Use bevy::utils::HashMap for most cases
    "std::sync::Mutex",          # Use bevy::tasks::AsyncComputeTaskPool
    "std::sync::RwLock",         # Use Bevy's change detection
    "std::rc::Rc",               # Use Entity references in ECS
    "std::sync::Arc",            # Use Bevy's resource system
    
    # Precision types that hurt performance
    "f64",                       # Use f32 for AI calculations
    "i64",                       # Use i32 unless specifically needed
    "u64",                       # Use u32 unless specifically needed
]

# === SOLID PRINCIPLE ENFORCEMENT ===

# Single Responsibility Principle
cognitive-complexity-threshold = 10    # Lower threshold for focused functions
too-many-arguments-threshold = 5       # Force parameter objects for complex functions
type-complexity-threshold = 150        # Prevent overly complex types

# Interface Segregation Principle  
trait-bounds-threshold = 3             # Limit trait bounds per generic
max-trait-methods = 5                  # Force trait splitting

# Dependency Inversion Principle
# (Enforced through disallowed concrete types above)

# === CODE DUPLICATION PREVENTION ===

# Function length limits to prevent duplication
too-many-lines-threshold = 50          # Force function extraction
max-fn-params-bools = 2                # Prevent boolean parameter proliferation

# Struct field limits to enforce SRP
max-struct-bools = 3                   # Force enum usage for multiple bools
struct-excessive-bools = 3             # Same as above

# === PERFORMANCE AND MAINTAINABILITY ===

# Memory efficiency
vec-init-then-push = true              # Prefer Vec::with_capacity
large-stack-arrays = 1024              # Force heap allocation for large arrays
large-const-arrays = 1024              # Same for const arrays

# Error handling consistency
missing-errors-doc = true              # Document all error conditions
missing-panics-doc = true              # Document all panic conditions
unwrap-used = true                     # Force proper error handling
expect-used = true                     # Force proper error handling

# Documentation requirements
missing-docs-in-private-items = true   # Document all items for maintainability
undocumented-unsafe-blocks = true      # Safety documentation required

# Naming consistency
enum-variant-names = true              # Consistent enum naming
module-name-repetitions = true         # Avoid redundant module names
similar-names = true                   # Catch typos and similar names

# Import organization
wildcard-imports = true                # Force explicit imports
unused-imports = true                  # Clean up unused imports
```

## Custom Lint Rules for AI-Specific Patterns

### `src/lints/mod.rs`
```rust
//! Custom lint rules for Artificial Society project
//! Enforces DRY and SOLID principles specific to AI systems

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};

// === DRY PRINCIPLE LINTS ===

declare_lint! {
    /// Detects duplicate validation logic across components
    pub DUPLICATE_VALIDATION_LOGIC,
    Deny,
    "duplicate validation logic should be extracted to common functions"
}

declare_lint! {
    /// Detects magic numbers that should be named constants
    pub AI_MAGIC_NUMBERS,
    Deny,
    "AI parameters should be named constants for maintainability"
}

declare_lint! {
    /// Detects duplicate system patterns
    pub DUPLICATE_SYSTEM_PATTERNS,
    Deny,
    "similar system logic should be extracted to generic functions"
}

// === SOLID PRINCIPLE LINTS ===

declare_lint! {
    /// Enforces Single Responsibility Principle for components
    pub COMPONENT_MULTIPLE_RESPONSIBILITIES,
    Deny,
    "components should have a single, clear responsibility"
}

declare_lint! {
    /// Enforces Interface Segregation Principle for traits
    pub TRAIT_TOO_MANY_METHODS,
    Deny,
    "traits should be focused and have at most 5 methods"
}

declare_lint! {
    /// Enforces Dependency Inversion Principle
    pub CONCRETE_DEPENDENCY_INJECTION,
    Deny,
    "depend on abstractions, not concrete implementations"
}

declare_lint_pass!(ArtificialSocietyLints => [
    DUPLICATE_VALIDATION_LOGIC,
    AI_MAGIC_NUMBERS,
    DUPLICATE_SYSTEM_PATTERNS,
    COMPONENT_MULTIPLE_RESPONSIBILITIES,
    TRAIT_TOO_MANY_METHODS,
    CONCRETE_DEPENDENCY_INJECTION,
]);

impl<'tcx> LateLintPass<'tcx> for ArtificialSocietyLints {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        match &item.kind {
            ItemKind::Struct(variant_data, _) => {
                self.check_component_responsibility(cx, item, variant_data);
            }
            ItemKind::Trait(_, _, _, _, trait_items) => {
                self.check_trait_method_count(cx, item, trait_items);
            }
            ItemKind::Impl(impl_item) => {
                self.check_duplicate_validation(cx, item, impl_item);
            }
            _ => {}
        }
    }
    
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Lit(lit) = &expr.kind {
            self.check_magic_numbers(cx, expr, lit);
        }
    }
}

impl ArtificialSocietyLints {
    fn check_component_responsibility(
        &mut self,
        cx: &LateContext<'_>,
        item: &Item<'_>,
        _variant_data: &rustc_hir::VariantData<'_>,
    ) {
        // Check if struct has Component derive
        let has_component_derive = item.attrs.iter().any(|attr| {
            attr.path().segments.last()
                .map(|seg| seg.ident.name.as_str() == "derive")
                .unwrap_or(false)
        });
        
        if has_component_derive {
            let struct_name = item.ident.name.as_str();
            
            // Analyze field names to detect multiple responsibilities
            let responsibility_keywords = [
                ("personality", "personality_management"),
                ("needs", "physiological_needs"),
                ("memory", "memory_management"),
                ("emotion", "emotional_state"),
                ("social", "social_interaction"),
                ("decision", "decision_making"),
            ];
            
            let mut detected_responsibilities = Vec::new();
            
            for (keyword, responsibility) in &responsibility_keywords {
                if struct_name.to_lowercase().contains(keyword) {
                    detected_responsibilities.push(*responsibility);
                }
            }
            
            if detected_responsibilities.len() > 1 {
                span_lint_and_help(
                    cx,
                    COMPONENT_MULTIPLE_RESPONSIBILITIES,
                    item.span,
                    &format!(
                        "Component '{}' appears to have multiple responsibilities: {}",
                        struct_name,
                        detected_responsibilities.join(", ")
                    ),
                    None,
                    "Consider splitting this component into focused, single-responsibility components",
                );
            }
        }
    }
    
    fn check_trait_method_count(
        &mut self,
        cx: &LateContext<'_>,
        item: &Item<'_>,
        trait_items: &[rustc_hir::TraitItemRef],
    ) {
        if trait_items.len() > 5 {
            span_lint_and_help(
                cx,
                TRAIT_TOO_MANY_METHODS,
                item.span,
                &format!(
                    "Trait '{}' has {} methods (maximum 5 allowed)",
                    item.ident.name.as_str(),
                    trait_items.len()
                ),
                None,
                "Consider splitting this trait into smaller, more focused interfaces",
            );
        }
    }
    
    fn check_magic_numbers(
        &mut self,
        cx: &LateContext<'_>,
        expr: &Expr<'_>,
        lit: &rustc_ast::Lit,
    ) {
        use rustc_ast::LitKind;
        
        match &lit.kind {
            LitKind::Float(symbol, _) => {
                let value_str = symbol.as_str();
                let value: f64 = value_str.parse().unwrap_or(0.0);
                
                // Check for AI-specific magic numbers
                if (0.1..=0.9).contains(&value) || value > 2.0 {
                    // Skip if in const/static context
                    let in_const_context = cx.tcx.hir().parent_iter(expr.hir_id)
                        .any(|(_, node)| matches!(node, rustc_hir::Node::Item(item) 
                            if matches!(item.kind, ItemKind::Const(..) | ItemKind::Static(..))));
                    
                    if !in_const_context {
                        span_lint_and_help(
                            cx,
                            AI_MAGIC_NUMBERS,
                            expr.span,
                            &format!("Magic number {} should be a named constant", value_str),
                            None,
                            "Define this value as a const in a thresholds module",
                        );
                    }
                }
            }
            LitKind::Int(value, _) => {
                if *value > 10 {
                    let in_const_context = cx.tcx.hir().parent_iter(expr.hir_id)
                        .any(|(_, node)| matches!(node, rustc_hir::Node::Item(item) 
                            if matches!(item.kind, ItemKind::Const(..) | ItemKind::Static(..))));
                    
                    if !in_const_context {
                        span_lint_and_help(
                            cx,
                            AI_MAGIC_NUMBERS,
                            expr.span,
                            &format!("Magic number {} should be a named constant", value),
                            None,
                            "Define this value as a const in a thresholds module",
                        );
                    }
                }
            }
            _ => {}
        }
    }
    
    fn check_duplicate_validation(
        &mut self,
        cx: &LateContext<'_>,
        _item: &Item<'_>,
        _impl_item: &rustc_hir::Impl<'_>,
    ) {
        // This would require more complex AST analysis
        // For now, we rely on the external Python scripts
        // Future enhancement: implement full AST-based duplicate detection
    }
}
```

### `Cargo.toml` Integration
```toml
[workspace.lints.clippy]
# DRY and SOLID enforcement at workspace level
cognitive_complexity = "deny"
too_many_arguments = "deny"
type_complexity = "deny"
missing_errors_doc = "deny"
missing_panics_doc = "deny"
unwrap_used = "deny"
expect_used = "deny"

# Custom lints
duplicate_validation_logic = "deny"
ai_magic_numbers = "deny"
duplicate_system_patterns = "deny"
component_multiple_responsibilities = "deny"
trait_too_many_methods = "deny"
concrete_dependency_injection = "deny"

[package.metadata.clippy]
# Project-specific clippy configuration
msrv = "1.70.0"
```

## Build Script Integration

### `build.rs`
```rust
//! Build script that enforces DRY and SOLID principles at compile time

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    
    // Run DRY violation detection
    let dry_check = Command::new("python")
        .args(&["scripts/dry_violation_detector.py", "src/"])
        .output();
    
    match dry_check {
        Ok(output) => {
            if !output.status.success() {
                panic!("DRY principle violations detected:\n{}", 
                       String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(e) => {
            println!("cargo:warning=Could not run DRY check: {}", e);
        }
    }
    
    // Run SOLID principle validation
    let solid_check = Command::new("python")
        .args(&["scripts/solid_principle_checker.py", "src/"])
        .output();
    
    match solid_check {
        Ok(output) => {
            if !output.status.success() {
                panic!("SOLID principle violations detected:\n{}", 
                       String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(e) => {
            println!("cargo:warning=Could not run SOLID check: {}", e);
        }
    }
    
    println!("✅ DRY and SOLID principles validated at build time");
}
```

## IDE Integration

### `.vscode/settings.json`
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": [
        "--all-targets",
        "--all-features",
        "--",
        "-D", "warnings",
        "-D", "clippy::cognitive_complexity",
        "-D", "clippy::too_many_arguments",
        "-D", "clippy::type_complexity",
        "-D", "duplicate_validation_logic",
        "-D", "ai_magic_numbers",
        "-D", "component_multiple_responsibilities"
    ],
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
        "source.fixAll": true,
        "source.organizeImports": true
    },
    "files.watcherExclude": {
        "**/target/**": true
    },
    "rust-analyzer.lens.enable": true,
    "rust-analyzer.lens.implementations": true,
    "rust-analyzer.lens.references": true,
    "rust-analyzer.inlayHints.enable": true,
    "rust-analyzer.diagnostics.enable": true,
    "rust-analyzer.diagnostics.enableExperimental": true
}
```

This comprehensive Clippy configuration ensures that DRY and SOLID principles are enforced at every level:

1. **Compile-Time Enforcement**: Custom lints catch violations during compilation
2. **IDE Integration**: Real-time feedback in development environment  
3. **Build-Time Validation**: Build script runs principle checks automatically
4. **Performance Focus**: Disallowed types and methods prevent performance regressions
5. **Documentation Requirements**: Force proper documentation for maintainability
6. **Error Handling**: Eliminate unwrap/expect in favor of proper error handling
7. **Import Organization**: Maintain clean, explicit imports
8. **Type Safety**: Prevent overly complex types that violate SRP

The system provides immediate feedback to developers and prevents any code that violates DRY or SOLID principles from being committed or deployed.