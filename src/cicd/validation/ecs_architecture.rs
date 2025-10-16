use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Validates Bevy ECS patterns and domain separation architecture.
#[derive(Debug)]
pub struct BevyEcsValidator {
    pub component_validator: ComponentValidator,
    pub system_validator: SystemValidator,
    pub domain_separation_validator: DomainSeparationValidator,
}

impl BevyEcsValidator {
    pub fn new() -> Self {
        Self {
            component_validator: ComponentValidator::new(),
            system_validator: SystemValidator::new(),
            domain_separation_validator: DomainSeparationValidator::new(),
        }
    }

    pub fn validate_file(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Run all validation checks
        let component_result = self.component_validator.validate_component_design(file_content, file_path);
        all_issues.extend(component_result.issues);
        all_suggestions.extend(component_result.suggestions);

        let system_result = self.system_validator.validate_system_design(file_content, file_path);
        all_issues.extend(system_result.issues);
        all_suggestions.extend(system_result.suggestions);

        let domain_result = self.domain_separation_validator.validate_domain_separation(file_content, file_path);
        all_issues.extend(domain_result.issues);
        all_suggestions.extend(domain_result.suggestions);

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
        }
    }
}

impl Default for BevyEcsValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates component design patterns for pure data enforcement.
#[derive(Debug)]
pub struct ComponentValidator {
    pub allowed_component_patterns: Vec<Regex>,
    pub forbidden_behavior_patterns: Vec<Regex>,
    pub allowed_method_names: HashSet<String>,
}

impl ComponentValidator {
    pub fn new() -> Self {
        let mut allowed_method_names = HashSet::new();
        allowed_method_names.insert("new".to_string());
        allowed_method_names.insert("default".to_string());
        allowed_method_names.insert("validate".to_string());
        allowed_method_names.insert("is_valid".to_string());
        
        // Allow getter methods
        for prefix in ["get_", "is_", "has_", "can_"] {
            allowed_method_names.insert(prefix.to_string());
        }

        Self {
            allowed_component_patterns: vec![
                Regex::new(r"#\[derive\([^)]*Component[^)]*\)\]").unwrap(),
            ],
            forbidden_behavior_patterns: vec![
                Regex::new(r"pub fn (\w+)\(&mut self").unwrap(), // Mutable methods (behavior)
                Regex::new(r"fn update").unwrap(),
                Regex::new(r"fn process").unwrap(),
                Regex::new(r"fn execute").unwrap(),
                Regex::new(r"fn run").unwrap(),
            ],
            allowed_method_names,
        }
    }

    pub fn validate_component_design(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Find component definitions
        let component_regex = Regex::new(r"#\[derive\([^)]*Component[^)]*\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub\s+)?struct\s+(\w+)").unwrap();
        
        for captures in component_regex.captures_iter(file_content) {
            let component_name = &captures[1];
            
            // Check for behavior methods in components
            if let Some(impl_block) = self.find_impl_block(file_content, component_name) {
                let behavior_methods = self.find_behavior_methods(&impl_block);
                
                for method in behavior_methods {
                    issues.push(ValidationIssue {
                        issue_type: IssueType::ComponentWithBehavior,
                        message: format!(
                            "Component '{}' contains behavior method '{}'. Components should only contain data.",
                            component_name, method.name
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: method.line_number,
                        suggestion: Some(format!(
                            "Move method '{}' to a system. Components should be pure data structures.",
                            method.name
                        )),
                    });
                }
            }
            
            // Check for proper field types
            if let Some(struct_body) = self.find_struct_body(file_content, component_name) {
                let invalid_fields = self.find_invalid_field_types(&struct_body);
                
                for field in invalid_fields {
                    issues.push(ValidationIssue {
                        issue_type: IssueType::InvalidComponentFieldType,
                        message: format!(
                            "Component '{}' field '{}' has invalid type '{}' for ECS component",
                            component_name, field.name, field.field_type
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number: field.line_number,
                        suggestion: Some(self.suggest_field_type_fix(&field)),
                    });
                }
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
        }
    }
    
    fn find_impl_block(&self, content: &str, struct_name: &str) -> Option<String> {
        let impl_pattern = format!(r"impl\s+(?:<[^>]*>\s+)?{}\s*(?:<[^>]*>\s*)?{{", regex::escape(struct_name));
        let impl_regex = Regex::new(&impl_pattern).unwrap();
        
        if let Some(start_match) = impl_regex.find(content) {
            let start_pos = start_match.end() - 1; // Position of opening brace
            let mut brace_count = 1;
            let current_pos = start_pos + 1;
            
            for (i, ch) in content[current_pos..].char_indices() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some(content[start_pos..current_pos + i + 1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        None
    }

    fn find_struct_body(&self, content: &str, struct_name: &str) -> Option<String> {
        let struct_pattern = format!(r"struct\s+{}\s*(?:<[^>]*>\s*)?{{", regex::escape(struct_name));
        let struct_regex = Regex::new(&struct_pattern).unwrap();
        
        if let Some(start_match) = struct_regex.find(content) {
            let start_pos = start_match.end() - 1; // Position of opening brace
            let mut brace_count = 1;
            let current_pos = start_pos + 1;
            
            for (i, ch) in content[current_pos..].char_indices() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some(content[start_pos..current_pos + i + 1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        None
    }
    
    fn find_behavior_methods(&self, impl_block: &str) -> Vec<MethodInfo> {
        let mut methods = Vec::new();
        let method_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?fn\s+(\w+)\s*\(&mut self").unwrap();
        
        for captures in method_regex.captures_iter(impl_block) {
            let method_name = &captures[1];
            
            // Skip allowed methods (getters, validation, etc.)
            if !self.is_allowed_method(method_name) {
                methods.push(MethodInfo {
                    name: method_name.to_string(),
                    line_number: self.find_line_number(impl_block, &captures[0]),
                    is_behavior: true,
                });
            }
        }
        
        // Also check for specific behavior patterns
        for pattern in &self.forbidden_behavior_patterns {
            for mat in pattern.find_iter(impl_block) {
                if let Some(captures) = pattern.captures(mat.as_str()) {
                    if captures.len() > 1 {
                        let method_name = &captures[1];
                        if !self.is_allowed_method(method_name) {
                            methods.push(MethodInfo {
                                name: method_name.to_string(),
                                line_number: self.find_line_number(impl_block, mat.as_str()),
                                is_behavior: true,
                            });
                        }
                    }
                }
            }
        }
        
        methods
    }

    fn find_invalid_field_types(&self, struct_body: &str) -> Vec<FieldInfo> {
        let mut invalid_fields = Vec::new();
        let field_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?(\w+):\s*([^,\n}]+)").unwrap();
        
        for captures in field_regex.captures_iter(struct_body) {
            let field_name = &captures[1];
            let field_type = captures[2].trim();
            
            // Check for problematic field types
            if self.is_invalid_field_type(field_type) {
                invalid_fields.push(FieldInfo {
                    name: field_name.to_string(),
                    field_type: field_type.to_string(),
                    line_number: self.find_line_number(struct_body, &captures[0]),
                });
            }
        }
        
        invalid_fields
    }
    
    fn is_allowed_method(&self, method_name: &str) -> bool {
        // Allow specific method names
        if self.allowed_method_names.contains(method_name) {
            return true;
        }
        
        // Allow methods that start with allowed prefixes
        for allowed_name in &self.allowed_method_names {
            if allowed_name.ends_with('_') && method_name.starts_with(allowed_name) {
                return true;
            }
        }
        
        false
    }

    fn is_invalid_field_type(&self, field_type: &str) -> bool {
        // Check for types that suggest behavior rather than data
        let behavior_indicators = [
            "fn(",           // Function pointers
            "Box<dyn",       // Trait objects
            "Arc<Mutex",     // Shared mutable state
            "RefCell",       // Interior mutability
            "Rc<RefCell",    // Shared interior mutability
        ];
        
        behavior_indicators.iter().any(|indicator| field_type.contains(indicator))
    }

    fn suggest_field_type_fix(&self, field: &FieldInfo) -> String {
        if field.field_type.contains("fn(") {
            "Consider using events or resources instead of function pointers in components".to_string()
        } else if field.field_type.contains("Box<dyn") {
            "Consider using enums or separate components instead of trait objects".to_string()
        } else if field.field_type.contains("Arc<Mutex") || field.field_type.contains("RefCell") {
            "Components should not contain shared mutable state. Use separate components or resources".to_string()
        } else {
            "Consider if this field represents data or behavior. Components should only contain data".to_string()
        }
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::ComponentWithBehavior => {
                    suggestions.push("Components should be pure data structures. Move behavior to systems.".to_string());
                }
                IssueType::InvalidComponentFieldType => {
                    suggestions.push("Use simple data types in components. Complex behavior should be in systems.".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for ComponentValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a method found in component implementation.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub line_number: usize,
    pub is_behavior: bool,
}

/// Information about a field in a component.
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub line_number: usize,
}
/// Validates system architecture patterns for proper ECS design.
#[derive(Debug)]
pub struct SystemValidator {
    pub query_pattern_validator: QueryPatternValidator,
    pub event_communication_validator: EventCommunicationValidator,
}

impl SystemValidator {
    pub fn new() -> Self {
        Self {
            query_pattern_validator: QueryPatternValidator::new(),
            event_communication_validator: EventCommunicationValidator::new(),
        }
    }

    pub fn validate_system_design(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Find system functions
        let system_regex = Regex::new(r"fn\s+(\w+_system)\s*\(").unwrap();
        
        for captures in system_regex.captures_iter(file_content) {
            let system_name = &captures[1];
            
            if let Some(system_body) = self.find_system_body(file_content, system_name) {
                // Check for direct component access violations
                let direct_access_issues = self.check_direct_component_access(&system_body, system_name, file_path);
                issues.extend(direct_access_issues);
                
                // Check for proper query usage
                let query_issues = self.query_pattern_validator.validate_queries(&system_body, system_name, file_path);
                issues.extend(query_issues);
                
                // Check for event-driven communication
                let communication_issues = self.event_communication_validator.validate_communication(&system_body, system_name, file_path);
                issues.extend(communication_issues);
            }
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
        }
    }
    
    fn find_system_body(&self, content: &str, system_name: &str) -> Option<String> {
        let system_start_pattern = format!(r"fn\s+{}\s*\([^{{]*\{{", regex::escape(system_name));
        let system_start_regex = Regex::new(&system_start_pattern).unwrap();
        
        if let Some(start_match) = system_start_regex.find(content) {
            let start_pos = start_match.end() - 1; // Position of opening brace
            let mut brace_count = 1;
            let current_pos = start_pos + 1;
            
            for (i, ch) in content[current_pos..].char_indices() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some(content[start_pos..current_pos + i + 1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        None
    }
    
    fn check_direct_component_access(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Look for direct component access patterns that violate ECS principles
        let direct_access_patterns = [
            r"\.get_component::<",
            r"\.component::<",
            r"world\.get::<",
            r"world\.entity\(",
        ];
        
        for pattern_str in &direct_access_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            
            for mat in pattern.find_iter(system_body) {
                issues.push(ValidationIssue {
                    issue_type: IssueType::DirectComponentAccess,
                    message: format!(
                        "System '{}' uses direct component access: '{}'. Use queries instead.",
                        system_name, mat.as_str()
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number: self.find_line_number(system_body, mat.as_str()),
                    suggestion: Some(
                        "Replace direct component access with proper Query parameters in system signature".to_string()
                    ),
                });
            }
        }
        
        issues
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::DirectComponentAccess => {
                    suggestions.push("Use Query parameters in system signatures instead of direct world access".to_string());
                }
                IssueType::ImproperQueryUsage => {
                    suggestions.push("Follow Bevy query patterns for efficient component access".to_string());
                }
                IssueType::MissingEventCommunication => {
                    suggestions.push("Use events for communication between systems instead of direct access".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for SystemValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates query patterns for efficiency and correctness.
#[derive(Debug)]
pub struct QueryPatternValidator {
    pub inefficient_patterns: Vec<Regex>,
}

impl QueryPatternValidator {
    pub fn new() -> Self {
        Self {
            inefficient_patterns: vec![
                Regex::new(r"Query<.*>.*\.iter\(\).*\.collect\(\)").unwrap(), // Collecting all results
                Regex::new(r"Query<.*>.*\.for_each.*\.for_each").unwrap(),     // Nested iterations
            ],
        }
    }

    pub fn validate_queries(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check for inefficient query patterns
        for pattern in &self.inefficient_patterns {
            for mat in pattern.find_iter(system_body) {
                issues.push(ValidationIssue {
                    issue_type: IssueType::ImproperQueryUsage,
                    message: format!(
                        "System '{}' uses inefficient query pattern: '{}'",
                        system_name, mat.as_str()
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number: self.find_line_number(system_body, mat.as_str()),
                    suggestion: Some(
                        "Use direct iteration over queries instead of collecting or nested iterations".to_string()
                    ),
                });
            }
        }
        
        issues
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }
}

impl Default for QueryPatternValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates event-driven communication patterns.
#[derive(Debug)]
pub struct EventCommunicationValidator {
    pub required_event_patterns: Vec<Regex>,
}

impl EventCommunicationValidator {
    pub fn new() -> Self {
        Self {
            required_event_patterns: vec![
                Regex::new(r"EventReader<").unwrap(),
                Regex::new(r"EventWriter<").unwrap(),
            ],
        }
    }

    pub fn validate_communication(&self, system_body: &str, system_name: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check if system modifies state but doesn't use events for communication
        if self.modifies_state(system_body) && !self.uses_events(system_body) {
            issues.push(ValidationIssue {
                issue_type: IssueType::MissingEventCommunication,
                message: format!(
                    "System '{}' modifies state but doesn't use events for communication",
                    system_name
                ),
                file_path: file_path.to_path_buf(),
                line_number: 1, // Default to line 1 if we can't find specific location
                suggestion: Some(
                    "Add EventWriter parameters to communicate state changes to other systems".to_string()
                ),
            });
        }
        
        issues
    }

    fn modifies_state(&self, system_body: &str) -> bool {
        let mutation_patterns = [
            r"&mut\s+\w+",
            r"\.insert\(",
            r"\.remove\(",
            r"\.despawn\(",
        ];
        
        mutation_patterns.iter().any(|pattern| {
            Regex::new(pattern).unwrap().is_match(system_body)
        })
    }

    fn uses_events(&self, system_body: &str) -> bool {
        self.required_event_patterns.iter().any(|pattern| {
            pattern.is_match(system_body)
        })
    }
}

impl Default for EventCommunicationValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates domain separation architecture.
#[derive(Debug)]
pub struct DomainSeparationValidator {
    pub valid_domains: HashSet<String>,
    pub cross_domain_patterns: Vec<Regex>,
}

impl DomainSeparationValidator {
    pub fn new() -> Self {
        let mut valid_domains = HashSet::new();
        valid_domains.insert("ai".to_string());
        valid_domains.insert("world".to_string());
        valid_domains.insert("presentation".to_string());
        valid_domains.insert("core".to_string());

        Self {
            valid_domains,
            cross_domain_patterns: vec![
                Regex::new(r"use\s+crate::(ai|world|presentation)::").unwrap(),
                Regex::new(r"use\s+super::(ai|world|presentation)::").unwrap(),
            ],
        }
    }

    pub fn validate_domain_separation(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Determine current domain from file path
        let current_domain = self.get_domain_from_path(file_path);
        
        if let Some(domain) = current_domain {
            // Check for cross-domain imports
            let cross_domain_issues = self.check_cross_domain_imports(file_content, &domain, file_path);
            issues.extend(cross_domain_issues);
            
            // Check for proper plugin structure
            let plugin_issues = self.check_plugin_structure(file_content, &domain, file_path);
            issues.extend(plugin_issues);
            
            // Check file organization
            let organization_issues = self.check_file_organization(file_path);
            issues.extend(organization_issues);
        }
        
        let suggestions = self.generate_suggestions(&issues);
        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
        }
    }

    fn get_domain_from_path(&self, file_path: &Path) -> Option<String> {
        let path_str = file_path.to_string_lossy();
        
        for domain in &self.valid_domains {
            if path_str.contains(&format!("/{}/", domain)) || path_str.contains(&format!("\\{}\\", domain)) {
                return Some(domain.clone());
            }
        }
        
        None
    }

    fn check_cross_domain_imports(&self, file_content: &str, current_domain: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        for pattern in &self.cross_domain_patterns {
            for captures in pattern.captures_iter(file_content) {
                if let Some(imported_domain) = captures.get(1) {
                    let imported_domain_str = imported_domain.as_str();
                    
                    // Check if this is a cross-domain import
                    if imported_domain_str != current_domain && imported_domain_str != "core" {
                        issues.push(ValidationIssue {
                            issue_type: IssueType::DomainSeparationViolation,
                            message: format!(
                                "Domain '{}' imports from domain '{}'. Domains should communicate only through events.",
                                current_domain, imported_domain_str
                            ),
                            file_path: file_path.to_path_buf(),
                            line_number: self.find_line_number(file_content, captures.get(0).unwrap().as_str()),
                            suggestion: Some(
                                "Use events for cross-domain communication instead of direct imports".to_string()
                            ),
                        });
                    }
                }
            }
        }
        
        issues
    }

    fn check_plugin_structure(&self, file_content: &str, domain: &str, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Check if this is a mod.rs file in a domain
        if file_path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") {
            // Should have a plugin definition
            let plugin_regex = Regex::new(&format!(r"pub struct {}Plugin", regex::escape(&domain.to_ascii_uppercase()))).unwrap();
            
            if !plugin_regex.is_match(file_content) {
                issues.push(ValidationIssue {
                    issue_type: IssueType::MissingDomainPlugin,
                    message: format!(
                        "Domain '{}' should have a single Plugin struct",
                        domain
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number: 1,
                    suggestion: Some(format!(
                        "Add 'pub struct {}Plugin;' and implement Plugin trait",
                        domain.to_ascii_uppercase()
                    )),
                });
            }
        }
        
        issues
    }

    fn check_file_organization(&self, file_path: &Path) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let path_str = file_path.to_string_lossy();
        
        // Check if file is in correct domain structure
        if path_str.contains("/src/") || path_str.contains("\\src\\") {
            let valid_structure = self.valid_domains.iter().any(|domain| {
                path_str.contains(&format!("/src/{}/", domain)) || 
                path_str.contains(&format!("\\src\\{}\\", domain)) ||
                path_str.contains("/src/main.rs") ||
                path_str.contains("\\src\\main.rs") ||
                path_str.contains("/src/lib.rs") ||
                path_str.contains("\\src\\lib.rs")
            });
            
            if !valid_structure {
                issues.push(ValidationIssue {
                    issue_type: IssueType::InvalidFileOrganization,
                    message: format!(
                        "File '{}' is not in a valid domain directory",
                        file_path.display()
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number: 1,
                    suggestion: Some(
                        "Move file to appropriate domain directory (ai/, world/, presentation/, or core/)".to_string()
                    ),
                });
            }
        }
        
        issues
    }

    fn find_line_number(&self, content: &str, target: &str) -> usize {
        content[..content.find(target).unwrap_or(0)]
            .lines()
            .count()
    }

    fn generate_suggestions(&self, issues: &[ValidationIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                IssueType::DomainSeparationViolation => {
                    suggestions.push("Maintain domain separation by using events for cross-domain communication".to_string());
                }
                IssueType::MissingDomainPlugin => {
                    suggestions.push("Each domain should expose a single Plugin that registers all components and systems".to_string());
                }
                IssueType::InvalidFileOrganization => {
                    suggestions.push("Organize files by domain (ai/, world/, presentation/) not by code type".to_string());
                }
                _ => {}
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for DomainSeparationValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a validation check.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub file_path: PathBuf,
    pub passed: bool,
    pub issues: Vec<ValidationIssue>,
    pub suggestions: Vec<String>,
}

/// Individual validation issue found in code.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub issue_type: IssueType,
    pub message: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub suggestion: Option<String>,
}

/// Types of validation issues that can be detected.
#[derive(Debug, Clone)]
pub enum IssueType {
    // ECS Architecture Issues
    ComponentWithBehavior,
    InvalidComponentFieldType,
    DirectComponentAccess,
    ImproperQueryUsage,
    MissingEventCommunication,
    DomainSeparationViolation,
    MissingDomainPlugin,
    InvalidFileOrganization,
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::ComponentWithBehavior => write!(f, "ComponentWithBehavior"),
            IssueType::InvalidComponentFieldType => write!(f, "InvalidComponentFieldType"),
            IssueType::DirectComponentAccess => write!(f, "DirectComponentAccess"),
            IssueType::ImproperQueryUsage => write!(f, "ImproperQueryUsage"),
            IssueType::MissingEventCommunication => write!(f, "MissingEventCommunication"),
            IssueType::DomainSeparationViolation => write!(f, "DomainSeparationViolation"),
            IssueType::MissingDomainPlugin => write!(f, "MissingDomainPlugin"),
            IssueType::InvalidFileOrganization => write!(f, "InvalidFileOrganization"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_with_behavior_detection() {
        let validator = ComponentValidator::new();
        
        // Test code with behavior in component
        let bad_component = r#"
        #[derive(Component)]
        pub struct PersonalityVector {
            pub openness: f32,
            pub conscientiousness: f32,
        }
        
        impl PersonalityVector {
            pub fn update_from_stress(&mut self, stress_level: f32) {
                self.openness -= stress_level * 0.1;
            }
            
            pub fn process_social_interaction(&mut self, other: &PersonalityVector) {
                // This is behavior, not data access
            }
        }
        "#;
        
        let result = validator.validate_component_design(bad_component, Path::new("test.rs"));
        assert!(!result.passed, "Component with behavior should fail validation");
        assert_eq!(result.issues.len(), 2); // Two behavior methods
        
        // Test code with only data access methods
        let good_component = r#"
        #[derive(Component)]
        pub struct PersonalityVector {
            pub openness: f32,
            pub conscientiousness: f32,
        }
        
        impl PersonalityVector {
            pub fn new(openness: f32, conscientiousness: f32) -> Self {
                Self { openness, conscientiousness }
            }
            
            pub fn get_openness(&self) -> f32 {
                self.openness
            }
            
            pub fn is_introverted(&self) -> bool {
                self.openness < 0.3
            }
            
            pub fn validate(&self) -> Result<(), String> {
                if self.openness < 0.0 || self.openness > 1.0 {
                    Err("Invalid openness".to_string())
                } else {
                    Ok(())
                }
            }
        }
        "#;
        
        let result = validator.validate_component_design(good_component, Path::new("test.rs"));
        assert!(result.passed, "Component with only data access should pass validation");
    }

    #[test]
    fn test_invalid_field_types_detection() {
        let validator = ComponentValidator::new();
        
        // Test component with invalid field types
        let bad_component = r#"
        #[derive(Component)]
        pub struct BadComponent {
            pub callback: fn() -> bool,
            pub behavior: Box<dyn SomeTrait>,
            pub shared_state: Arc<Mutex<i32>>,
            pub interior_mut: RefCell<String>,
        }
        "#;
        
        let result = validator.validate_component_design(bad_component, Path::new("test.rs"));
        assert!(!result.passed, "Component with invalid field types should fail validation");
        assert_eq!(result.issues.len(), 4); // Four invalid field types
        
        // Test component with valid field types
        let good_component = r#"
        #[derive(Component)]
        pub struct GoodComponent {
            pub value: f32,
            pub name: String,
            pub position: Vec3,
            pub enabled: bool,
        }
        "#;
        
        let result = validator.validate_component_design(good_component, Path::new("test.rs"));
        assert!(result.passed, "Component with valid field types should pass validation");
    }

    #[test]
    fn test_direct_component_access_detection() {
        let validator = SystemValidator::new();
        
        // Test system with direct component access
        let bad_system = r#"
        fn bad_system(world: &mut World) {
            let entity = world.spawn().id();
            let component = world.get_component::<Transform>(entity);
            world.entity(entity).insert(Health::new());
        }
        "#;
        
        let result = validator.validate_system_design(bad_system, Path::new("test.rs"));
        assert!(!result.passed, "System with direct component access should fail validation");
        assert!(result.issues.len() > 0);
        
        // Test system with proper query usage
        let good_system = r#"
        fn good_system(
            mut agents: Query<(&mut Transform, &Health)>,
            mut commands: Commands,
        ) {
            for (mut transform, health) in agents.iter_mut() {
                if health.is_low() {
                    transform.translation.y -= 1.0;
                }
            }
        }
        "#;
        
        let result = validator.validate_system_design(good_system, Path::new("test.rs"));
        assert!(result.passed, "System with proper queries should pass validation");
    }

    #[test]
    fn test_domain_separation_validation() {
        let validator = DomainSeparationValidator::new();
        
        // Test cross-domain import violation
        let bad_import = r#"
        use crate::ai::personality::PersonalityVector;
        use crate::world::environment::Environment;
        
        // This file is in presentation domain but imports from ai and world
        "#;
        
        let result = validator.validate_domain_separation(bad_import, Path::new("src/presentation/ui.rs"));
        assert!(!result.passed, "Cross-domain imports should fail validation");
        assert_eq!(result.issues.len(), 2); // Two cross-domain imports
        
        // Test proper domain separation
        let good_import = r#"
        use crate::core::types::*;
        use bevy::prelude::*;
        
        // Only imports from core domain and external crates
        "#;
        
        let result = validator.validate_domain_separation(good_import, Path::new("src/presentation/ui.rs"));
        assert!(result.passed, "Proper domain separation should pass validation");
    }

    #[test]
    fn test_plugin_structure_validation() {
        let validator = DomainSeparationValidator::new();
        
        // Test missing plugin structure
        let bad_mod = r#"
        pub mod personality;
        pub mod decision_making;
        
        // Missing AiPlugin struct
        "#;
        
        let result = validator.validate_domain_separation(bad_mod, Path::new("src/ai/mod.rs"));
        assert!(!result.passed, "Missing domain plugin should fail validation");
        assert_eq!(result.issues.len(), 1);
        
        // Test proper plugin structure
        let good_mod = r#"
        pub mod personality;
        pub mod decision_making;
        
        use bevy::prelude::*;
        
        pub struct AIPlugin;
        
        impl Plugin for AIPlugin {
            fn build(&self, app: &mut App) {
                app.add_systems(Update, (
                    personality_system,
                    decision_making_system,
                ));
            }
        }
        "#;
        
        let result = validator.validate_domain_separation(good_mod, Path::new("src/ai/mod.rs"));
        assert!(result.passed, "Proper plugin structure should pass validation");
    }

    #[test]
    fn test_event_communication_validation() {
        let validator = EventCommunicationValidator::new();
        
        // Test system that modifies state without events
        let bad_system_body = r#"
        {
            for mut agent in agents.iter_mut() {
                agent.stress_level += 0.1;
                // Modifies state but doesn't communicate via events
            }
        }
        "#;
        
        let issues = validator.validate_communication(bad_system_body, "stress_system", Path::new("test.rs"));
        assert!(issues.len() > 0, "System modifying state without events should have issues");
        
        // Test system that uses events properly
        let good_system_body = r#"
        {
            for mut agent in agents.iter_mut() {
                agent.stress_level += 0.1;
                stress_events.send(StressChanged {
                    entity: agent_entity,
                    new_level: agent.stress_level,
                });
            }
        }
        "#;
        
        let issues = validator.validate_communication(good_system_body, "stress_system", Path::new("test.rs"));
        assert_eq!(issues.len(), 0, "System using events properly should have no issues");
    }

    #[test]
    fn test_full_ecs_validator() {
        let validator = BevyEcsValidator::new();
        
        // Test file with multiple ECS violations
        let bad_code = r#"
        use crate::ai::personality::PersonalityVector; // Cross-domain import
        
        #[derive(Component)]
        pub struct BadComponent {
            pub callback: fn() -> bool, // Invalid field type
        }
        
        impl BadComponent {
            pub fn update(&mut self) { // Behavior in component
                // Do something
            }
        }
        
        fn bad_system(world: &mut World) { // Direct world access
            let entity = world.spawn().id();
            world.get_component::<BadComponent>(entity);
        }
        "#;
        
        let result = validator.validate_file(bad_code, Path::new("src/presentation/bad.rs"));
        assert!(!result.passed, "File with multiple ECS violations should fail validation");
        assert!(result.issues.len() >= 4); // At least 4 different types of issues
        
        // Test file with proper ECS patterns
        let good_code = r#"
        use bevy::prelude::*;
        use crate::core::types::*;
        
        #[derive(Component)]
        pub struct GoodComponent {
            pub value: f32,
            pub enabled: bool,
        }
        
        impl GoodComponent {
            pub fn new(value: f32) -> Self {
                Self { value, enabled: true }
            }
            
            pub fn get_value(&self) -> f32 {
                self.value
            }
        }
        
        fn good_system(
            mut components: Query<&mut GoodComponent>,
            mut events: EventWriter<ComponentChanged>,
        ) {
            for mut component in components.iter_mut() {
                component.value += 1.0;
                events.send(ComponentChanged);
            }
        }
        "#;
        
        let result = validator.validate_file(good_code, Path::new("src/presentation/good.rs"));
        assert!(result.passed, "File with proper ECS patterns should pass validation");
    }
}