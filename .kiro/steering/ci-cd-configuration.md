---
inclusion: fileMatch
fileMatchPattern: ".github/**/*"
---

# CI/CD Configuration for Artificial Society

## Core Philosophy: "Quality Gates Prevent Regressions"

Every code change must pass through multiple quality gates that validate both technical correctness and behavioral believability. The CI/CD system enforces the "Feel Over Science" philosophy automatically.

## Pre-Commit Hook Configuration

### `.pre-commit-config.yaml`
```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml

  - repo: https://github.com/domlysz/BlenderGIS
    rev: master
    hooks:
      - id: cargo-fmt
        args: ['--all', '--', '--check']
      - id: cargo-clippy
        args: ['--all-targets', '--all-features', '--', '-D', 'warnings']

  - repo: local
    hooks:
      - id: dry-principle-enforcement
        name: DRY Principle Hard Enforcement
        entry: python scripts/dry_violation_detector.py
        language: python
        files: \.rs$
        pass_filenames: true
        
      - id: solid-principle-enforcement
        name: SOLID Principle Hard Enforcement
        entry: python scripts/solid_principle_checker.py
        language: python
        files: \.rs$
        pass_filenames: true
        
      - id: ai-behavior-validation
        name: AI Behavior Pattern Validation
        entry: scripts/validate_ai_patterns.py
        language: python
        files: ^src/ai/.*\.rs$
        
      - id: performance-pattern-check
        name: Performance Pattern Validation
        entry: scripts/check_performance_patterns.py
        language: python
        files: ^src/.*\.rs$
        
      - id: architecture-validation
        name: Architecture Pattern Enforcement
        entry: scripts/architecture_validator.py
        language: python
        files: ^src/.*\.rs$
        
      - id: magic-number-elimination
        name: Magic Number Hard Enforcement
        entry: python scripts/magic_number_detector.py
        language: python
        files: \.rs$
        pass_filenames: true
```

### Custom Validation Scripts

#### `scripts/validate_ai_patterns.py`
```python
#!/usr/bin/env python3
"""
Validates AI code follows project patterns and philosophy.
"""
import re
import sys
from pathlib import Path

class AIPatternValidator:
    def __init__(self):
        self.errors = []
        
    def validate_value_ranges(self, content, filename):
        """Ensure proper value ranges for AI components."""
        # Check for personality traits (should be 0.0-1.0)
        personality_pattern = r'pub\s+(\w+):\s*f32.*//.*personality'
        if re.search(personality_pattern, content, re.IGNORECASE):
            if not re.search(r'0\.0.*1\.0|Normalized<f32>', content):
                self.errors.append(f"{filename}: Personality traits must use 0.0-1.0 range or Normalized<f32>")
        
        # Check for emotional states (should be -1.0 to 1.0)
        emotion_pattern = r'pub\s+(valence|arousal|dominance):\s*f32'
        if re.search(emotion_pattern, content):
            if not re.search(r'-1\.0.*1\.0', content):
                self.errors.append(f"{filename}: Emotional states must use -1.0 to 1.0 range")
    
    def validate_personality_modulation(self, content, filename):
        """Ensure all AI systems are modulated by personality."""
        if 'System' in content and 'fn ' in content:
            if 'Personality' not in content and 'ai/' in filename:
                self.errors.append(f"{filename}: AI systems should be modulated by personality traits")
    
    def validate_temporal_usage(self, content, filename):
        """Ensure proper use of WorldTime instead of real time."""
        if re.search(r'Instant::now\(\)|SystemTime::now\(\)', content):
            if 'WorldTime' not in content:
                self.errors.append(f"{filename}: Use WorldTime instead of real-world time for AI calculations")
    
    def validate_event_driven_architecture(self, content, filename):
        """Ensure systems use events for communication."""
        if re.search(r'Query.*&mut.*Query.*&mut', content):
            self.errors.append(f"{filename}: Avoid multiple mutable queries; use events for system communication")

def main():
    validator = AIPatternValidator()
    
    for file_path in sys.argv[1:]:
        if file_path.endswith('.rs'):
            with open(file_path, 'r') as f:
                content = f.read()
                validator.validate_value_ranges(content, file_path)
                validator.validate_personality_modulation(content, file_path)
                validator.validate_temporal_usage(content, file_path)
                validator.validate_event_driven_architecture(content, file_path)
    
    if validator.errors:
        for error in validator.errors:
            print(f"ERROR: {error}")
        sys.exit(1)
    
    print("✓ All AI patterns validated successfully")

if __name__ == "__main__":
    main()
```

#### `scripts/check_performance_patterns.py`
```python
#!/usr/bin/env python3
"""
Validates performance-critical patterns in Rust code.
"""
import re
import sys

class PerformancePatternChecker:
    def __init__(self):
        self.errors = []
        self.warnings = []
    
    def check_prohibited_patterns(self, content, filename):
        """Check for patterns that hurt performance."""
        # Prohibit f64 in hot paths
        if re.search(r'f64', content) and 'ai/' in filename:
            self.warnings.append(f"{filename}: Consider using f32 instead of f64 for AI calculations")
        
        # Prohibit synchronous I/O in systems
        if re.search(r'std::fs::|File::open|\.read\(|\.write\(', content) and 'fn.*system' in content:
            self.errors.append(f"{filename}: Avoid synchronous I/O in systems; use async or background tasks")
        
        # Check for expensive operations in hot loops
        if re.search(r'\.sin\(\)|\.cos\(\)|\.exp\(\)|\.log\(\)', content) and 'iter\(\)' in content:
            self.warnings.append(f"{filename}: Consider using lookup tables for expensive math in loops")
    
    def check_memory_patterns(self, content, filename):
        """Check for memory-inefficient patterns."""
        # Check for excessive allocations
        if re.search(r'Vec::new\(\).*loop|HashMap::new\(\).*loop', content):
            self.warnings.append(f"{filename}: Consider pre-allocating collections outside loops")
        
        # Check for string allocations in hot paths
        if re.search(r'String::from|\.to_string\(\)|format!', content) and 'system' in content:
            self.warnings.append(f"{filename}: Avoid string allocations in systems; use &str when possible")
    
    def check_parallel_patterns(self, content, filename):
        """Check for proper parallel processing patterns."""
        if re.search(r'\.iter\(\)\.for_each', content) and 'agents' in content:
            if not re.search(r'\.par_iter\(\)', content):
                self.warnings.append(f"{filename}: Consider using par_iter() for agent processing")

def main():
    checker = PerformancePatternChecker()
    
    for file_path in sys.argv[1:]:
        if file_path.endswith('.rs'):
            with open(file_path, 'r') as f:
                content = f.read()
                checker.check_prohibited_patterns(content, file_path)
                checker.check_memory_patterns(content, file_path)
                checker.check_parallel_patterns(content, file_path)
    
    # Print warnings
    for warning in checker.warnings:
        print(f"WARNING: {warning}")
    
    # Fail on errors
    if checker.errors:
        for error in checker.errors:
            print(f"ERROR: {error}")
        sys.exit(1)
    
    print("✓ Performance patterns validated successfully")

if __name__ == "__main__":
    main()
```

## GitHub Actions Workflow

### `.github/workflows/ci.yml`
```yaml
name: Continuous Integration

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  code-quality:
    name: Code Quality Checks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Validate AI patterns
        run: python scripts/validate_ai_patterns.py src/ai/**/*.rs
      
      - name: Check performance patterns
        run: python scripts/check_performance_patterns.py src/**/*.rs

  unit-tests:
    name: Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run unit tests
        run: cargo test --lib --bins
      
      - name: Run doc tests
        run: cargo test --doc

  behavioral-validation:
    name: AI Behavioral Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build test binary
        run: cargo build --release --bin behavioral_tests
      
      - name: Run personality consistency tests
        run: ./target/release/behavioral_tests --test personality_consistency
      
      - name: Run social dynamics tests
        run: ./target/release/behavioral_tests --test social_dynamics
      
      - name: Run emotional contagion tests
        run: ./target/release/behavioral_tests --test emotional_contagion
      
      - name: Validate communication pipeline
        run: ./target/release/behavioral_tests --test communication_pipeline

  performance-tests:
    name: Performance Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build performance test binary
        run: cargo build --release --bin performance_tests
      
      - name: Set CPU governor to performance
        run: |
          echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
      
      - name: Run 60fps validation with 100 agents
        run: ./target/release/performance_tests --agents 100 --target-fps 60 --duration 30
      
      - name: Run memory usage validation
        run: ./target/release/performance_tests --memory-test --agents 1000 --max-memory 100MB
      
      - name: Run parallel processing validation
        run: ./target/release/performance_tests --parallel-test --agents 500
      
      - name: Run LOD system validation
        run: ./target/release/performance_tests --lod-test --players 10 --agents 1000
      
      - name: Upload performance reports
        uses: actions/upload-artifact@v3
        with:
          name: performance-reports
          path: performance_reports/

  cross-platform:
    name: Cross-Platform Build
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build
        run: cargo build --release
      
      - name: Run basic tests
        run: cargo test --release

  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
      
      - name: Run security audit
        run: cargo audit
      
      - name: Check for known vulnerabilities
        run: cargo audit --deny warnings
```

## Performance Testing Configuration

### Virtual Environment Specifications
```yaml
# .github/workflows/performance-baseline.yml
name: Performance Baseline Testing

on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM
  workflow_dispatch:

jobs:
  performance-baseline:
    name: Performance Baseline on Simulated Hardware
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup simulated medium-low hardware
        run: |
          # Limit CPU cores to simulate medium-low hardware
          echo "Setting CPU affinity to 2 cores"
          sudo cpulimit --limit=50 --pid=$$ &
          
          # Limit memory to 4GB
          echo "Setting memory limit to 4GB"
          sudo systemd-run --scope -p MemoryLimit=4G bash
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Build optimized binary
        run: cargo build --release --bin performance_benchmark
      
      - name: Run comprehensive performance suite
        run: |
          ./target/release/performance_benchmark \
            --test-suite comprehensive \
            --agents 100 \
            --duration 300 \
            --target-fps 60 \
            --memory-limit 4GB \
            --cpu-cores 2 \
            --output performance_baseline.json
      
      - name: Compare against baseline
        run: python scripts/compare_performance_baseline.py performance_baseline.json
      
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: performance-baseline
          path: performance_baseline.json
```

## Code Quality Configuration

### `clippy.toml`
```toml
# Clippy configuration for Artificial Society
avoid-breaking-exported-api = false
msrv = "1.70.0"

# AI-specific lints
disallowed-methods = [
    "std::time::Instant::now",  # Use WorldTime instead
    "std::time::SystemTime::now",  # Use WorldTime instead
    "std::thread::sleep",  # Avoid blocking in systems
]

disallowed-types = [
    "std::collections::HashMap",  # Prefer bevy::utils::HashMap for performance
]

# Performance-focused lints
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 8
type-complexity-threshold = 250
```

### `rustfmt.toml`
```toml
# Rust formatting configuration
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
merge_derives = true
use_try_shorthand = true
use_field_init_shorthand = true
force_explicit_abi = true
condense_wildcard_suffixes = true
color = "Auto"
required_version = "1.5.1"

# AI code specific formatting
imports_granularity = "Module"
group_imports = "StdExternalCrate"
```

## Development Environment Setup

### `devcontainer.json`
```json
{
  "name": "Artificial Society Development",
  "image": "mcr.microsoft.com/devcontainers/rust:1-1-bullseye",
  "features": {
    "ghcr.io/devcontainers/features/common-utils:2": {},
    "ghcr.io/devcontainers/features/git:1": {},
    "ghcr.io/devcontainers/features/github-cli:1": {}
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "serayuzgur.crates",
        "tamasfe.even-better-toml",
        "ms-vscode.test-adapter-converter"
      ],
      "settings": {
        "rust-analyzer.checkOnSave.command": "clippy",
        "rust-analyzer.cargo.features": "all",
        "editor.formatOnSave": true,
        "editor.codeActionsOnSave": {
          "source.fixAll": true
        }
      }
    }
  },
  "postCreateCommand": "cargo install cargo-watch cargo-audit cargo-tarpaulin",
  "remoteUser": "vscode"
}
```

This comprehensive CI/CD infrastructure ensures that:

1. **Code Quality** is maintained through automated formatting, linting, and pattern validation
2. **Performance Standards** are enforced through automated benchmarking on standardized hardware
3. **Behavioral Consistency** is validated through specialized AI behavior tests
4. **Architecture Patterns** are enforced to maintain the "Feel Over Science" philosophy
5. **Security** is maintained through dependency auditing and vulnerability scanning
6. **Cross-Platform Compatibility** is ensured through multi-OS testing
7. **Development Environment** is standardized through containerization and tooling