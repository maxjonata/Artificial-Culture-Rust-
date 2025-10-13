---
inclusion: fileMatch
fileMatchPattern: "scripts/**/*"
---

# CI Enforcement Scripts for DRY and SOLID Principles

## Core Philosophy: "Automated Quality Gates Prevent Technical Debt"

These scripts provide hard enforcement of DRY and SOLID principles through static analysis, runtime validation, and behavioral testing. Zero tolerance for violations.

## DRY Principle Enforcement Scripts

### `scripts/dry_violation_detector.py`
```python
#!/usr/bin/env python3
"""
Detects DRY (Don't Repeat Yourself) violations in Rust code.
Hard fails CI on any duplication above thresholds.
"""

import ast
import re
import sys
import hashlib
from pathlib import Path
from typing import Dict, List, Set, Tuple
from dataclasses import dataclass

@dataclass
class CodeBlock:
    content: str
    file_path: str
    start_line: int
    end_line: int
    hash: str

class DRYViolationDetector:
    def __init__(self):
        self.violations = []
        self.code_blocks = []
        self.function_signatures = {}
        self.struct_definitions = {}
        self.constants = set()
        
    def extract_code_blocks(self, content: str, file_path: str) -> List[CodeBlock]:
        """Extract meaningful code blocks for duplication analysis."""
        blocks = []
        lines = content.split('\n')
        
        # Extract function bodies
        in_function = False
        function_start = 0
        brace_count = 0
        
        for i, line in enumerate(lines):
            stripped = line.strip()
            
            # Function start detection
            if re.match(r'^\s*(pub\s+)?fn\s+\w+', stripped):
                in_function = True
                function_start = i
                brace_count = 0
            
            if in_function:
                brace_count += stripped.count('{') - stripped.count('}')
                
                # Function end detection
                if brace_count == 0 and '{' in stripped:
                    function_body = '\n'.join(lines[function_start:i+1])
                    # Normalize whitespace for comparison
                    normalized = re.sub(r'\s+', ' ', function_body).strip()
                    
                    if len(normalized) > 50:  # Only check substantial functions
                        block_hash = hashlib.md5(normalized.encode()).hexdigest()
                        blocks.append(CodeBlock(
                            content=normalized,
                            file_path=file_path,
                            start_line=function_start + 1,
                            end_line=i + 1,
                            hash=block_hash
                        ))
                    
                    in_function = False
        
        return blocks
    
    def detect_duplicate_functions(self, blocks: List[CodeBlock]) -> List[str]:
        """Detect duplicate function implementations."""
        violations = []
        hash_to_blocks = {}
        
        for block in blocks:
            if block.hash in hash_to_blocks:
                existing = hash_to_blocks[block.hash]
                violations.append(
                    f"DUPLICATE FUNCTION: {block.file_path}:{block.start_line}-{block.end_line} "
                    f"duplicates {existing.file_path}:{existing.start_line}-{existing.end_line}"
                )
            else:
                hash_to_blocks[block.hash] = block
        
        return violations
    
    def detect_duplicate_structs(self, content: str, file_path: str) -> List[str]:
        """Detect duplicate struct definitions."""
        violations = []
        
        # Extract struct definitions
        struct_pattern = r'#\[derive\([^\]]+\)\]\s*(?:pub\s+)?struct\s+(\w+)\s*\{([^}]+)\}'
        structs = re.findall(struct_pattern, content, re.MULTILINE | re.DOTALL)
        
        for struct_name, struct_body in structs:
            # Normalize field definitions
            fields = []
            for line in struct_body.split('\n'):
                line = line.strip()
                if line and not line.startswith('//'):
                    # Extract field type pattern
                    field_match = re.match(r'pub\s+(\w+):\s*([^,]+)', line)
                    if field_match:
                        field_type = field_match.group(2).strip().rstrip(',')
                        fields.append(field_type)
            
            fields_signature = '|'.join(sorted(fields))
            
            if fields_signature in self.struct_definitions:
                existing_struct, existing_file = self.struct_definitions[fields_signature]
                if existing_file != file_path:  # Different files
                    violations.append(
                        f"DUPLICATE STRUCT PATTERN: {struct_name} in {file_path} "
                        f"has same field types as {existing_struct} in {existing_file}"
                    )
            else:
                self.struct_definitions[fields_signature] = (struct_name, file_path)
        
        return violations
    
    def detect_magic_numbers(self, content: str, file_path: str) -> List[str]:
        """Detect magic numbers that should be named constants."""
        violations = []
        
        # Find numeric literals in code (excluding tests and obvious cases)
        magic_number_pattern = r'(?<![\w\.])\d*\.\d+(?![\w\.])|(?<![\w\.])\d+(?![\w\.])'
        
        lines = content.split('\n')
        for i, line in enumerate(lines):
            # Skip test files, comments, and obvious cases
            if any(skip in line.lower() for skip in ['test', '//', '/*', 'const', 'static']):
                continue
            
            matches = re.finditer(magic_number_pattern, line)
            for match in matches:
                number = match.group()
                # Skip obvious non-magic numbers
                if number in ['0', '1', '2', '0.0', '1.0', '2.0']:
                    continue
                
                # Check if it's a meaningful threshold or parameter
                if float(number) > 2.0 or (0.1 <= float(number) <= 0.9):
                    violations.append(
                        f"MAGIC NUMBER: {file_path}:{i+1} - '{number}' should be a named constant"
                    )
        
        return violations
    
    def detect_duplicate_system_patterns(self, content: str, file_path: str) -> List[str]:
        """Detect duplicate system implementation patterns."""
        violations = []
        
        # Extract system function patterns
        system_pattern = r'fn\s+(\w+_system)\s*\([^)]+\)\s*\{([^}]+(?:\{[^}]*\}[^}]*)*)\}'
        systems = re.findall(system_pattern, content, re.MULTILINE | re.DOTALL)
        
        for system_name, system_body in systems:
            # Normalize system body for pattern matching
            normalized_body = re.sub(r'\w+(?=\.)', 'VAR', system_body)  # Replace variable names
            normalized_body = re.sub(r'\s+', ' ', normalized_body).strip()
            
            # Create pattern signature
            pattern_hash = hashlib.md5(normalized_body.encode()).hexdigest()
            
            if pattern_hash in self.function_signatures:
                existing_system, existing_file = self.function_signatures[pattern_hash]
                if existing_file != file_path:
                    violations.append(
                        f"DUPLICATE SYSTEM PATTERN: {system_name} in {file_path} "
                        f"has same logic pattern as {existing_system} in {existing_file}"
                    )
            else:
                self.function_signatures[pattern_hash] = (system_name, file_path)
        
        return violations
    
    def analyze_file(self, file_path: str) -> List[str]:
        """Analyze a single file for DRY violations."""
        violations = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Extract and analyze code blocks
            blocks = self.extract_code_blocks(content, file_path)
            self.code_blocks.extend(blocks)
            
            # Detect various types of violations
            violations.extend(self.detect_duplicate_structs(content, file_path))
            violations.extend(self.detect_magic_numbers(content, file_path))
            violations.extend(self.detect_duplicate_system_patterns(content, file_path))
            
        except Exception as e:
            violations.append(f"ERROR analyzing {file_path}: {e}")
        
        return violations
    
    def finalize_analysis(self) -> List[str]:
        """Perform cross-file analysis after all files processed."""
        return self.detect_duplicate_functions(self.code_blocks)

def main():
    detector = DRYViolationDetector()
    all_violations = []
    
    # Analyze all Rust files
    rust_files = []
    for arg in sys.argv[1:]:
        if arg.endswith('.rs'):
            rust_files.append(arg)
        else:
            # Directory - find all .rs files
            for rs_file in Path(arg).rglob('*.rs'):
                rust_files.append(str(rs_file))
    
    # Analyze each file
    for file_path in rust_files:
        violations = detector.analyze_file(file_path)
        all_violations.extend(violations)
    
    # Perform cross-file analysis
    cross_file_violations = detector.finalize_analysis()
    all_violations.extend(cross_file_violations)
    
    # Report results
    if all_violations:
        print("❌ DRY PRINCIPLE VIOLATIONS DETECTED:")
        for violation in all_violations:
            print(f"  {violation}")
        print(f"\nTotal violations: {len(all_violations)}")
        sys.exit(1)
    else:
        print("✅ No DRY violations detected")

if __name__ == "__main__":
    main()
```

### `scripts/solid_principle_checker.py`
```python
#!/usr/bin/env python3
"""
Validates SOLID principles in Rust code.
Hard fails CI on any SOLID principle violations.
"""

import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
from dataclasses import dataclass

@dataclass
class Component:
    name: str
    fields: List[str]
    methods: List[str]
    file_path: str
    responsibilities: Set[str]

@dataclass
class Trait:
    name: str
    methods: List[str]
    file_path: str

class SOLIDPrincipleChecker:
    def __init__(self):
        self.violations = []
        self.components = []
        self.traits = []
        
    def extract_components(self, content: str, file_path: str) -> List[Component]:
        """Extract component definitions for SRP analysis."""
        components = []
        
        # Find struct definitions with Component derive
        component_pattern = r'#\[derive\([^)]*Component[^)]*\)\][^{]*struct\s+(\w+)\s*\{([^}]+)\}'
        matches = re.finditer(component_pattern, content, re.MULTILINE | re.DOTALL)
        
        for match in matches:
            struct_name = match.group(1)
            struct_body = match.group(2)
            
            # Extract fields
            fields = []
            field_pattern = r'pub\s+(\w+):\s*([^,\n]+)'
            field_matches = re.finditer(field_pattern, struct_body)
            
            for field_match in field_matches:
                field_name = field_match.group(1)
                field_type = field_match.group(2).strip().rstrip(',')
                fields.append(f"{field_name}: {field_type}")
            
            # Infer responsibilities from field names and types
            responsibilities = self.infer_responsibilities(fields)
            
            components.append(Component(
                name=struct_name,
                fields=fields,
                methods=[],  # Will be filled by impl analysis
                file_path=file_path,
                responsibilities=responsibilities
            ))
        
        return components
    
    def infer_responsibilities(self, fields: List[str]) -> Set[str]:
        """Infer component responsibilities from field names and types."""
        responsibilities = set()
        
        responsibility_keywords = {
            'personality': 'personality_management',
            'needs': 'physiological_needs',
            'hunger': 'physiological_needs',
            'energy': 'physiological_needs',
            'stress': 'stress_management',
            'emotion': 'emotional_state',
            'memory': 'memory_management',
            'relationship': 'social_relationships',
            'interaction': 'social_interaction',
            'decision': 'decision_making',
            'action': 'action_execution',
            'perception': 'environmental_perception',
            'communication': 'communication',
            'learning': 'learning_adaptation',
        }
        
        for field in fields:
            field_lower = field.lower()
            for keyword, responsibility in responsibility_keywords.items():
                if keyword in field_lower:
                    responsibilities.add(responsibility)
        
        return responsibilities
    
    def check_single_responsibility_principle(self) -> List[str]:
        """Check Single Responsibility Principle violations."""
        violations = []
        
        for component in self.components:
            if len(component.responsibilities) > 1:
                violations.append(
                    f"SRP VIOLATION: Component '{component.name}' in {component.file_path} "
                    f"has multiple responsibilities: {', '.join(component.responsibilities)}"
                )
        
        return violations
    
    def extract_traits(self, content: str, file_path: str) -> List[Trait]:
        """Extract trait definitions for ISP analysis."""
        traits = []
        
        # Find trait definitions
        trait_pattern = r'pub\s+trait\s+(\w+)\s*\{([^}]+)\}'
        matches = re.finditer(trait_pattern, content, re.MULTILINE | re.DOTALL)
        
        for match in matches:
            trait_name = match.group(1)
            trait_body = match.group(2)
            
            # Extract method signatures
            methods = []
            method_pattern = r'fn\s+(\w+)\s*\([^)]*\)(?:\s*->\s*[^;{]+)?[;{]'
            method_matches = re.finditer(method_pattern, trait_body)
            
            for method_match in method_matches:
                method_name = method_match.group(1)
                methods.append(method_name)
            
            traits.append(Trait(
                name=trait_name,
                methods=methods,
                file_path=file_path
            ))
        
        return traits
    
    def check_interface_segregation_principle(self) -> List[str]:
        """Check Interface Segregation Principle violations."""
        violations = []
        
        for trait in self.traits:
            if len(trait.methods) > 5:  # Threshold for "fat interface"
                violations.append(
                    f"ISP VIOLATION: Trait '{trait.name}' in {trait.file_path} "
                    f"has {len(trait.methods)} methods (max 5 allowed). "
                    f"Consider splitting into smaller, focused interfaces."
                )
        
        return violations
    
    def check_dependency_inversion_principle(self, content: str, file_path: str) -> List[str]:
        """Check Dependency Inversion Principle violations."""
        violations = []
        
        # Find struct fields that depend on concrete types instead of traits
        concrete_dependency_pattern = r'pub\s+\w+:\s*([A-Z]\w+(?:Storage|Manager|Handler|Service))'
        matches = re.finditer(concrete_dependency_pattern, content)
        
        for match in matches:
            concrete_type = match.group(1)
            # Check if there's a corresponding trait
            trait_pattern = f"trait.*{concrete_type.replace('Storage', '').replace('Manager', '').replace('Handler', '').replace('Service', '')}"
            
            if not re.search(trait_pattern, content, re.IGNORECASE):
                violations.append(
                    f"DIP VIOLATION: {file_path} depends on concrete type '{concrete_type}'. "
                    f"Consider depending on a trait abstraction instead."
                )
        
        return violations
    
    def check_open_closed_principle(self, content: str, file_path: str) -> List[str]:
        """Check Open/Closed Principle violations."""
        violations = []
        
        # Look for large match statements that might need extension
        match_pattern = r'match\s+[^{]+\{([^}]+)\}'
        matches = re.finditer(match_pattern, content, re.MULTILINE | re.DOTALL)
        
        for match in matches:
            match_body = match.group(1)
            # Count match arms
            arm_count = len(re.findall(r'=>', match_body))
            
            if arm_count > 10:  # Large match might indicate OCP violation
                violations.append(
                    f"OCP POTENTIAL VIOLATION: {file_path} has large match statement "
                    f"with {arm_count} arms. Consider using trait objects for extensibility."
                )
        
        return violations
    
    def analyze_file(self, file_path: str) -> List[str]:
        """Analyze a single file for SOLID principle violations."""
        violations = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Extract components and traits
            components = self.extract_components(content, file_path)
            traits = self.extract_traits(content, file_path)
            
            self.components.extend(components)
            self.traits.extend(traits)
            
            # Check individual principles
            violations.extend(self.check_dependency_inversion_principle(content, file_path))
            violations.extend(self.check_open_closed_principle(content, file_path))
            
        except Exception as e:
            violations.append(f"ERROR analyzing {file_path}: {e}")
        
        return violations
    
    def finalize_analysis(self) -> List[str]:
        """Perform cross-file analysis after all files processed."""
        violations = []
        violations.extend(self.check_single_responsibility_principle())
        violations.extend(self.check_interface_segregation_principle())
        return violations

def main():
    checker = SOLIDPrincipleChecker()
    all_violations = []
    
    # Analyze all Rust files
    rust_files = []
    for arg in sys.argv[1:]:
        if arg.endswith('.rs'):
            rust_files.append(arg)
        else:
            # Directory - find all .rs files
            for rs_file in Path(arg).rglob('*.rs'):
                rust_files.append(str(rs_file))
    
    # Analyze each file
    for file_path in rust_files:
        violations = checker.analyze_file(file_path)
        all_violations.extend(violations)
    
    # Perform cross-file analysis
    cross_file_violations = checker.finalize_analysis()
    all_violations.extend(cross_file_violations)
    
    # Report results
    if all_violations:
        print("❌ SOLID PRINCIPLE VIOLATIONS DETECTED:")
        for violation in all_violations:
            print(f"  {violation}")
        print(f"\nTotal violations: {len(all_violations)}")
        sys.exit(1)
    else:
        print("✅ No SOLID principle violations detected")

if __name__ == "__main__":
    main()
```

### `scripts/architecture_validator.py`
```python
#!/usr/bin/env python3
"""
Validates overall architecture patterns and dependencies.
Ensures proper domain separation and event-driven communication.
"""

import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
from dataclasses import dataclass

@dataclass
class Module:
    name: str
    path: str
    imports: Set[str]
    exports: Set[str]
    domain: str

class ArchitectureValidator:
    def __init__(self):
        self.modules = []
        self.violations = []
        self.domain_boundaries = {
            'ai': ['physiology', 'cognition', 'social', 'perception'],
            'core': ['types', 'builders', 'constants'],
            'world': ['environment', 'objects'],
            'presentation': ['debug_ui', 'profiler', 'rendering']
        }
    
    def extract_module_info(self, file_path: str) -> Module:
        """Extract module information from a Rust file."""
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Determine domain from path
        path_parts = Path(file_path).parts
        domain = 'unknown'
        for domain_name in self.domain_boundaries.keys():
            if domain_name in path_parts:
                domain = domain_name
                break
        
        # Extract imports
        imports = set()
        import_pattern = r'use\s+([^;]+);'
        for match in re.finditer(import_pattern, content):
            import_path = match.group(1).strip()
            imports.add(import_path)
        
        # Extract exports (pub items)
        exports = set()
        export_patterns = [
            r'pub\s+struct\s+(\w+)',
            r'pub\s+enum\s+(\w+)',
            r'pub\s+trait\s+(\w+)',
            r'pub\s+fn\s+(\w+)',
            r'pub\s+const\s+(\w+)',
        ]
        
        for pattern in export_patterns:
            for match in re.finditer(pattern, content):
                exports.add(match.group(1))
        
        module_name = Path(file_path).stem
        return Module(
            name=module_name,
            path=file_path,
            imports=imports,
            exports=exports,
            domain=domain
        )
    
    def check_domain_boundaries(self) -> List[str]:
        """Check that domains don't have inappropriate cross-dependencies."""
        violations = []
        
        forbidden_dependencies = {
            'ai': ['presentation'],  # AI shouldn't depend on presentation
            'core': ['ai', 'world', 'presentation'],  # Core should be dependency-free
            'world': ['ai', 'presentation'],  # World shouldn't depend on AI or presentation
        }
        
        for module in self.modules:
            if module.domain in forbidden_dependencies:
                forbidden = forbidden_dependencies[module.domain]
                
                for import_path in module.imports:
                    for forbidden_domain in forbidden:
                        if forbidden_domain in import_path:
                            violations.append(
                                f"DOMAIN BOUNDARY VIOLATION: {module.path} "
                                f"(domain: {module.domain}) imports from forbidden domain: {import_path}"
                            )
        
        return violations
    
    def check_event_driven_communication(self) -> List[str]:
        """Check that systems use events instead of direct component access."""
        violations = []
        
        for module in self.modules:
            if 'system' in module.name.lower() and module.domain == 'ai':
                # Check for multiple mutable queries (anti-pattern)
                with open(module.path, 'r') as f:
                    content = f.read()
                
                # Look for multiple &mut in Query parameters
                query_pattern = r'Query<[^>]*&mut[^>]*&mut[^>]*>'
                if re.search(query_pattern, content):
                    violations.append(
                        f"EVENT-DRIVEN VIOLATION: {module.path} "
                        f"uses multiple mutable queries instead of events"
                    )
                
                # Check for direct cross-domain component access
                cross_domain_access = [
                    r'Query<[^>]*Presentation[^>]*>',
                    r'Query<[^>]*World[^>]*>',
                ]
                
                for pattern in cross_domain_access:
                    if re.search(pattern, content):
                        violations.append(
                            f"CROSS-DOMAIN ACCESS VIOLATION: {module.path} "
                            f"directly accesses components from other domains"
                        )
        
        return violations
    
    def check_circular_dependencies(self) -> List[str]:
        """Check for circular dependencies between modules."""
        violations = []
        
        # Build dependency graph
        dependencies = {}
        for module in self.modules:
            deps = set()
            for import_path in module.imports:
                # Extract module name from import path
                if '::' in import_path:
                    root_module = import_path.split('::')[0]
                    deps.add(root_module)
            dependencies[module.name] = deps
        
        # Detect cycles using DFS
        def has_cycle(node: str, visited: Set[str], rec_stack: Set[str]) -> bool:
            visited.add(node)
            rec_stack.add(node)
            
            for neighbor in dependencies.get(node, set()):
                if neighbor not in visited:
                    if has_cycle(neighbor, visited, rec_stack):
                        return True
                elif neighbor in rec_stack:
                    return True
            
            rec_stack.remove(node)
            return False
        
        visited = set()
        for module_name in dependencies:
            if module_name not in visited:
                if has_cycle(module_name, visited, set()):
                    violations.append(
                        f"CIRCULAR DEPENDENCY: Detected circular dependency involving {module_name}"
                    )
        
        return violations
    
    def check_plugin_architecture(self) -> List[str]:
        """Check that domains properly use Bevy plugin architecture."""
        violations = []
        
        domain_plugins = {}
        
        for module in self.modules:
            with open(module.path, 'r') as f:
                content = f.read()
            
            # Check for Plugin implementation
            plugin_pattern = r'impl\s+Plugin\s+for\s+(\w+Plugin)'
            plugin_match = re.search(plugin_pattern, content)
            
            if plugin_match:
                plugin_name = plugin_match.group(1)
                if module.domain not in domain_plugins:
                    domain_plugins[module.domain] = []
                domain_plugins[module.domain].append(plugin_name)
        
        # Each domain should have exactly one main plugin
        for domain, plugins in domain_plugins.items():
            if len(plugins) > 1:
                main_plugin = f"{domain.title()}Plugin"
                if main_plugin not in plugins:
                    violations.append(
                        f"PLUGIN ARCHITECTURE VIOLATION: Domain '{domain}' "
                        f"has multiple plugins but no main {main_plugin}"
                    )
        
        return violations

def main():
    validator = ArchitectureValidator()
    all_violations = []
    
    # Analyze all Rust files
    rust_files = []
    for arg in sys.argv[1:]:
        if arg.endswith('.rs'):
            rust_files.append(arg)
        else:
            # Directory - find all .rs files
            for rs_file in Path(arg).rglob('*.rs'):
                rust_files.append(str(rs_file))
    
    # Extract module information
    for file_path in rust_files:
        try:
            module = validator.extract_module_info(file_path)
            validator.modules.append(module)
        except Exception as e:
            all_violations.append(f"ERROR analyzing {file_path}: {e}")
    
    # Perform architecture validation
    all_violations.extend(validator.check_domain_boundaries())
    all_violations.extend(validator.check_event_driven_communication())
    all_violations.extend(validator.check_circular_dependencies())
    all_violations.extend(validator.check_plugin_architecture())
    
    # Report results
    if all_violations:
        print("❌ ARCHITECTURE VIOLATIONS DETECTED:")
        for violation in all_violations:
            print(f"  {violation}")
        print(f"\nTotal violations: {len(all_violations)}")
        sys.exit(1)
    else:
        print("✅ Architecture validation passed")

if __name__ == "__main__":
    main()
```

## Integration with CI/CD Pipeline

### Updated `.github/workflows/quality-gates.yml`
```yaml
name: Quality Gates - DRY and SOLID Enforcement

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  dry-solid-enforcement:
    name: DRY and SOLID Principles Enforcement
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Python dependencies
        run: |
          python -m pip install --upgrade pip
          pip install ast-tools pathlib2
      
      - name: Make scripts executable
        run: chmod +x scripts/*.py
      
      - name: Check DRY Principle Violations
        run: python scripts/dry_violation_detector.py src/
        
      - name: Check SOLID Principle Violations
        run: python scripts/solid_principle_checker.py src/
        
      - name: Validate Architecture Patterns
        run: python scripts/architecture_validator.py src/
      
      - name: Generate Quality Report
        run: |
          echo "# Code Quality Report" > quality_report.md
          echo "## DRY Principle Compliance: ✅ PASSED" >> quality_report.md
          echo "## SOLID Principle Compliance: ✅ PASSED" >> quality_report.md
          echo "## Architecture Validation: ✅ PASSED" >> quality_report.md
          echo "Generated at: $(date)" >> quality_report.md
      
      - name: Upload Quality Report
        uses: actions/upload-artifact@v3
        with:
          name: quality-report
          path: quality_report.md

  code-duplication-analysis:
    name: Advanced Code Duplication Analysis
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install jscpd (Copy-Paste Detector)
        run: npm install -g jscpd
      
      - name: Run duplication analysis
        run: |
          jscpd src/ --threshold 3 --format rust --output duplication_report.json
          
      - name: Check duplication threshold
        run: |
          python -c "
          import json
          with open('duplication_report.json') as f:
              report = json.load(f)
          duplication_percentage = report.get('statistics', {}).get('percentage', 0)
          if duplication_percentage > 2.0:
              print(f'❌ Code duplication {duplication_percentage}% exceeds 2% threshold')
              exit(1)
          else:
              print(f'✅ Code duplication {duplication_percentage}% within acceptable limits')
          "
      
      - name: Upload duplication report
        uses: actions/upload-artifact@v3
        with:
          name: duplication-report
          path: duplication_report.json

  dependency-analysis:
    name: Dependency Direction Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-modules
        run: cargo install cargo-modules
      
      - name: Generate dependency graph
        run: cargo modules generate graph --with-types > dependency_graph.dot
      
      - name: Validate dependency directions
        run: |
          python scripts/validate_dependency_directions.py dependency_graph.dot
      
      - name: Upload dependency graph
        uses: actions/upload-artifact@v3
        with:
          name: dependency-graph
          path: dependency_graph.dot
```

### Pre-commit Hook Integration

### `.pre-commit-config.yaml` (Updated)
```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml

  - repo: local
    hooks:
      - id: dry-principle-check
        name: DRY Principle Enforcement
        entry: python scripts/dry_violation_detector.py
        language: python
        files: \.rs$
        pass_filenames: true
        
      - id: solid-principle-check
        name: SOLID Principle Enforcement
        entry: python scripts/solid_principle_checker.py
        language: python
        files: \.rs$
        pass_filenames: true
        
      - id: architecture-validation
        name: Architecture Pattern Validation
        entry: python scripts/architecture_validator.py
        language: python
        files: \.rs$
        pass_filenames: true
        
      - id: magic-number-detection
        name: Magic Number Detection
        entry: python -c "
        import re, sys
        for f in sys.argv[1:]:
            with open(f) as file:
                content = file.read()
                if re.search(r'(?<![\\w\\.])[0-9]*\\.[0-9]+(?![\\w\\.])|(?<![\\w\\.])[3-9][0-9]*(?![\\w\\.])', content):
                    if not any(x in content for x in ['const', 'static', 'test']):
                        print(f'Magic numbers detected in {f}')
                        sys.exit(1)
        "
        language: python
        files: \.rs$
        pass_filenames: true

  - repo: https://github.com/domlysz/BlenderGIS
    rev: master
    hooks:
      - id: cargo-fmt
        args: ['--all', '--', '--check']
      - id: cargo-clippy
        args: ['--all-targets', '--all-features', '--', '-D', 'warnings']
```

This comprehensive enforcement system ensures that:

1. **DRY Violations** are caught immediately with zero tolerance
2. **SOLID Principles** are enforced through static analysis
3. **Architecture Patterns** maintain proper domain separation
4. **Code Duplication** is measured and limited to <2%
5. **Magic Numbers** are eliminated in favor of named constants
6. **Dependency Directions** follow proper abstraction hierarchies
7. **Pre-commit Hooks** prevent violations from entering the repository
8. **CI/CD Pipeline** provides comprehensive quality gates

The system fails fast and provides clear, actionable feedback to developers about how to fix violations while maintaining the high-performance, maintainable codebase required for the Artificial Society project.