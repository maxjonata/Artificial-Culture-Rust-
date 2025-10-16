use regex::Regex;
use std::path::Path;
use std::time::Instant;

use super::{ValidationResult, ValidationIssue, IssueType};
use super::ai_patterns::{IssueSeverity, CheckType};

/// Validates documentation quality and code quality metrics.
#[derive(Debug)]
pub struct DocumentationQualityValidator {
    pub doc_validator: DocumentationValidator,
    pub complexity_analyzer: ComplexityAnalyzer,
    pub duplication_detector: DuplicationDetector,
    pub coverage_analyzer: CoverageAnalyzer,
}

impl DocumentationQualityValidator {
    pub fn new() -> Self {
        Self {
            doc_validator: DocumentationValidator::new(),
            complexity_analyzer: ComplexityAnalyzer::new(),
            duplication_detector: DuplicationDetector::new(),
            coverage_analyzer: CoverageAnalyzer::new(),
        }
    }

    pub fn validate_file(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Run documentation validation
        let doc_result = self.doc_validator.validate_documentation(file_content, file_path);
        all_issues.extend(doc_result.issues);
        all_suggestions.extend(doc_result.suggestions);

        // Run complexity analysis
        let complexity_result = self.complexity_analyzer.analyze_complexity(file_content, file_path);
        all_issues.extend(complexity_result.issues);
        all_suggestions.extend(complexity_result.suggestions);

        // Run duplication detection
        let duplication_result = self.duplication_detector.detect_duplication(file_content, file_path);
        all_issues.extend(duplication_result.issues);
        all_suggestions.extend(duplication_result.suggestions);

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    pub fn validate_project_documentation(&self, project_root: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Check for required documentation files
        let required_docs = vec![
            "README.md",
            "CHANGELOG.md",
            "docs/API.md",
            "docs/ARCHITECTURE.md",
        ];

        for doc_file in required_docs {
            let doc_path = project_root.join(doc_file);
            if !doc_path.exists() {
                all_issues.push(ValidationIssue {
                    issue_type: IssueType::MissingDocumentation,
                    message: format!("Required documentation file missing: {}", doc_file),
                    file_path: doc_path.clone(),
                    line_number: 0,
                    column_number: None,
                    severity: IssueSeverity::Medium,
                    suggestion: Some(format!("Create {} with appropriate content", doc_file)),
                    code_example: None,
                });
            }
        }

        // Add general suggestions if issues found
        if !all_issues.is_empty() {
            all_suggestions.push("Ensure all required documentation files are present and up-to-date".to_string());
            all_suggestions.push("Consider using documentation templates for consistency".to_string());
        }

        ValidationResult {
            file_path: project_root.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }
}

/// Validates documentation completeness and quality.
#[derive(Debug)]
pub struct DocumentationValidator {
    pub public_api_patterns: Vec<Regex>,
    pub ai_system_patterns: Vec<Regex>,
    pub behavioral_purpose_patterns: Vec<Regex>,
}

impl DocumentationValidator {
    pub fn new() -> Self {
        Self {
            public_api_patterns: vec![
                Regex::new(r"pub\s+fn\s+(\w+)").unwrap(),
                Regex::new(r"pub\s+struct\s+(\w+)").unwrap(),
                Regex::new(r"pub\s+enum\s+(\w+)").unwrap(),
                Regex::new(r"pub\s+trait\s+(\w+)").unwrap(),
            ],
            ai_system_patterns: vec![
                Regex::new(r"fn\s+(\w*_system)\s*\(").unwrap(),
                Regex::new(r"fn\s+(.*_ai_.*)\s*\(").unwrap(),
                Regex::new(r"fn\s+(.*_behavior.*)\s*\(").unwrap(),
            ],
            behavioral_purpose_patterns: vec![
                Regex::new(r"# Behavioral Purpose").unwrap(),
                Regex::new(r"/// Behavioral Purpose:").unwrap(),
                Regex::new(r"/// This system implements").unwrap(),
            ],
        }
    }

    pub fn validate_documentation(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Only validate Rust files
        if !file_path.extension().map_or(false, |ext| ext == "rs") {
            return ValidationResult {
                file_path: file_path.to_path_buf(),
                passed: true,
                issues: vec![],
                suggestions: vec![],
                execution_time: start_time.elapsed(),
                check_type: CheckType::Manual,
            };
        }

        // Check public API documentation
        self.check_public_api_documentation(file_content, file_path, &mut issues);

        // Check AI system behavioral documentation
        self.check_ai_system_documentation(file_content, file_path, &mut issues);

        // Check doc comment quality
        self.check_doc_comment_quality(file_content, file_path, &mut issues);

        // Generate suggestions based on issues
        for issue in &issues {
            match issue.issue_type {
                IssueType::MissingDocumentation => {
                    suggestions.push("Add comprehensive doc comments explaining the purpose, parameters, and behavior".to_string());
                }
                IssueType::MissingBehavioralDocumentation => {
                    suggestions.push("Add '# Behavioral Purpose' section explaining how this affects AI behavior".to_string());
                }
                _ => {}
            }
        }

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn check_public_api_documentation(&self, file_content: &str, file_path: &Path, issues: &mut Vec<ValidationIssue>) {
        let lines: Vec<&str> = file_content.lines().collect();
        
        for pattern in &self.public_api_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                
                // Check if there's a doc comment above this line
                if !self.has_doc_comment_above(&lines, line_number) {
                    let item_name = pattern.captures(mat.as_str())
                        .and_then(|caps| caps.get(1))
                        .map(|m| m.as_str())
                        .unwrap_or("unknown");

                    issues.push(ValidationIssue {
                        issue_type: IssueType::MissingDocumentation,
                        message: format!("Public API item '{}' missing documentation", item_name),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        column_number: Some(mat.start()),
                        severity: IssueSeverity::High,
                        suggestion: Some(format!("Add doc comment: /// Description of {}", item_name)),
                        code_example: Some(format!("/// Description of what {} does\npub fn {}(...) {{", item_name, item_name)),
                    });
                }
            }
        }
    }

    fn check_ai_system_documentation(&self, file_content: &str, file_path: &Path, issues: &mut Vec<ValidationIssue>) {
        let lines: Vec<&str> = file_content.lines().collect();
        
        for pattern in &self.ai_system_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                let system_name = pattern.captures(mat.as_str())
                    .and_then(|caps| caps.get(1))
                    .map(|m| m.as_str())
                    .unwrap_or("unknown");

                // Check if there's behavioral purpose documentation
                if !self.has_behavioral_purpose_documentation(&lines, line_number) {
                    issues.push(ValidationIssue {
                        issue_type: IssueType::MissingBehavioralDocumentation,
                        message: format!("AI system '{}' missing behavioral purpose documentation", system_name),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        column_number: Some(mat.start()),
                        severity: IssueSeverity::High,
                        suggestion: Some("Add '# Behavioral Purpose' section explaining how this system affects AI behavior".to_string()),
                        code_example: Some(format!(
                            "/// {}\n/// \n/// # Behavioral Purpose\n/// Explain how this system affects agent behavior and believability\nfn {}(...) {{",
                            system_name, system_name
                        )),
                    });
                }
            }
        }
    }

    fn check_doc_comment_quality(&self, file_content: &str, file_path: &Path, issues: &mut Vec<ValidationIssue>) {
        let doc_comment_regex = Regex::new(r"///\s*(.*)").unwrap();
        
        for mat in doc_comment_regex.find_iter(file_content) {
            let line_number = self.find_line_number(file_content, mat.start());
            let comment_text = mat.as_str();
            
            // Check for low-quality doc comments
            if self.is_low_quality_doc_comment(comment_text) {
                issues.push(ValidationIssue {
                    issue_type: IssueType::LowQualityDocumentation,
                    message: "Doc comment lacks detail or is too generic".to_string(),
                    file_path: file_path.to_path_buf(),
                    line_number,
                    column_number: Some(mat.start()),
                    severity: IssueSeverity::Medium,
                    suggestion: Some("Provide more specific documentation explaining purpose, parameters, and behavior".to_string()),
                    code_example: None,
                });
            }
        }
    }

    fn has_doc_comment_above(&self, lines: &[&str], line_number: usize) -> bool {
        if line_number == 0 {
            return false;
        }
        
        // Check the lines above for doc comments
        for i in (0..line_number).rev() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                return true;
            }
            if !line.is_empty() && !line.starts_with("//") && !line.starts_with("#[") {
                break;
            }
        }
        false
    }

    fn has_behavioral_purpose_documentation(&self, lines: &[&str], line_number: usize) -> bool {
        // Look for behavioral purpose documentation in the preceding doc comments
        for i in (0..line_number).rev() {
            let line = lines[i].trim();
            if line.contains("# Behavioral Purpose") || 
               line.contains("Behavioral Purpose:") ||
               line.contains("This system implements") {
                return true;
            }
            if !line.is_empty() && !line.starts_with("///") && !line.starts_with("//") && !line.starts_with("#[") {
                break;
            }
        }
        false
    }

    fn is_low_quality_doc_comment(&self, comment: &str) -> bool {
        let content = comment.trim_start_matches("///").trim();
        
        // Check for generic or unhelpful comments
        let low_quality_patterns = [
            "TODO",
            "FIXME",
            "Function",
            "Method",
            "Struct",
            "Enum",
            "Does something",
            "Handles",
        ];
        
        for pattern in &low_quality_patterns {
            if content.to_lowercase().contains(&pattern.to_lowercase()) && content.len() < 50 {
                return true;
            }
        }
        
        // Check if comment is too short and generic
        content.len() < 20 && !content.contains(":")
    }

    fn find_line_number(&self, content: &str, byte_offset: usize) -> usize {
        content[..byte_offset].lines().count()
    }
}

/// Analyzes code complexity and suggests refactoring.
#[derive(Debug)]
pub struct ComplexityAnalyzer {
    pub complexity_threshold: usize,
    pub function_length_threshold: usize,
}

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self {
            complexity_threshold: 10,
            function_length_threshold: 50,
        }
    }

    pub fn analyze_complexity(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Only analyze Rust files
        if !file_path.extension().map_or(false, |ext| ext == "rs") {
            return ValidationResult {
                file_path: file_path.to_path_buf(),
                passed: true,
                issues: vec![],
                suggestions: vec![],
                execution_time: start_time.elapsed(),
                check_type: CheckType::Manual,
            };
        }

        // Find function definitions and analyze complexity
        let function_regex = Regex::new(r"fn\s+(\w+)\s*\([^)]*\)\s*(?:->\s*[^{]+)?\s*\{").unwrap();
        
        for mat in function_regex.find_iter(file_content) {
            let function_name = function_regex.captures(mat.as_str())
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str())
                .unwrap_or("unknown");
            
            let line_number = self.find_line_number(file_content, mat.start());
            
            // Extract function body
            if let Some(function_body) = self.extract_function_body(file_content, mat.end()) {
                // Calculate cyclomatic complexity
                let complexity = self.calculate_cyclomatic_complexity(&function_body);
                
                if complexity > self.complexity_threshold {
                    issues.push(ValidationIssue {
                        issue_type: IssueType::ExcessiveComplexity,
                        message: format!("Function '{}' has high cyclomatic complexity: {}", function_name, complexity),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        column_number: Some(mat.start()),
                        severity: IssueSeverity::Medium,
                        suggestion: Some("Consider breaking this function into smaller, more focused functions".to_string()),
                        code_example: None,
                    });
                }
                
                // Check function length
                let line_count = function_body.lines().count();
                if line_count > self.function_length_threshold {
                    issues.push(ValidationIssue {
                        issue_type: IssueType::ExcessiveFunctionLength,
                        message: format!("Function '{}' is too long: {} lines", function_name, line_count),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        column_number: Some(mat.start()),
                        severity: IssueSeverity::Medium,
                        suggestion: Some("Consider breaking this function into smaller functions".to_string()),
                        code_example: None,
                    });
                }
            }
        }

        // Generate general suggestions
        if !issues.is_empty() {
            suggestions.push("Consider using the Extract Method refactoring pattern to reduce complexity".to_string());
            suggestions.push("Break complex functions into smaller, single-purpose functions".to_string());
        }

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn calculate_cyclomatic_complexity(&self, function_body: &str) -> usize {
        let mut complexity = 1; // Base complexity
        
        // Count decision points
        let decision_patterns = [
            r"\bif\b",
            r"\belse\s+if\b",
            r"\bwhile\b",
            r"\bfor\b",
            r"\bmatch\b",
            r"\b\|\s*\w+\s*=>", // match arms
            r"\?\s*\{", // try operator with block
            r"&&",
            r"\|\|",
        ];
        
        for pattern_str in &decision_patterns {
            let pattern = Regex::new(pattern_str).unwrap();
            complexity += pattern.find_iter(function_body).count();
        }
        
        complexity
    }

    fn extract_function_body(&self, content: &str, start_pos: usize) -> Option<String> {
        let remaining = &content[start_pos..];
        let mut brace_count = 0;
        let mut body_start = None;
        let mut body_end = None;
        
        for (i, ch) in remaining.char_indices() {
            match ch {
                '{' => {
                    if body_start.is_none() {
                        body_start = Some(i);
                    }
                    brace_count += 1;
                }
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 && body_start.is_some() {
                        body_end = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        
        if let (Some(start), Some(end)) = (body_start, body_end) {
            Some(remaining[start..end].to_string())
        } else {
            None
        }
    }

    fn find_line_number(&self, content: &str, byte_offset: usize) -> usize {
        content[..byte_offset].lines().count()
    }
}

/// Detects code duplication and suggests refactoring opportunities.
#[derive(Debug)]
pub struct DuplicationDetector {
    pub min_duplicate_lines: usize,
    pub similarity_threshold: f32,
}

impl DuplicationDetector {
    pub fn new() -> Self {
        Self {
            min_duplicate_lines: 3,
            similarity_threshold: 0.8,
        }
    }

    pub fn detect_duplication(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Only analyze Rust files
        if !file_path.extension().map_or(false, |ext| ext == "rs") {
            return ValidationResult {
                file_path: file_path.to_path_buf(),
                passed: true,
                issues: vec![],
                suggestions: vec![],
                execution_time: start_time.elapsed(),
                check_type: CheckType::Manual,
            };
        }

        // Find duplicate code blocks
        let duplicates = self.find_duplicate_blocks(file_content);
        
        for duplicate in duplicates {
            issues.push(ValidationIssue {
                issue_type: IssueType::CodeDuplication,
                message: format!("Duplicate code block found ({} lines)", duplicate.line_count),
                file_path: file_path.to_path_buf(),
                line_number: duplicate.first_occurrence,
                column_number: None,
                severity: IssueSeverity::Medium,
                suggestion: Some("Extract common code into a shared function or method".to_string()),
                code_example: None,
            });
        }

        // Generate suggestions
        if !issues.is_empty() {
            suggestions.push("Use the Extract Method refactoring pattern to eliminate duplication".to_string());
            suggestions.push("Consider creating utility functions for common operations".to_string());
        }

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }

    fn find_duplicate_blocks(&self, content: &str) -> Vec<DuplicateBlock> {
        let lines: Vec<&str> = content.lines().collect();
        let mut duplicates = Vec::new();
        
        // Simple duplicate detection - look for identical line sequences
        for i in 0..lines.len() {
            for j in (i + self.min_duplicate_lines)..lines.len() {
                let mut match_length = 0;
                
                // Count consecutive matching lines
                while i + match_length < lines.len() && 
                      j + match_length < lines.len() &&
                      self.lines_similar(lines[i + match_length], lines[j + match_length]) {
                    match_length += 1;
                }
                
                if match_length >= self.min_duplicate_lines {
                    duplicates.push(DuplicateBlock {
                        first_occurrence: i + 1,
                        second_occurrence: j + 1,
                        line_count: match_length,
                    });
                }
            }
        }
        
        duplicates
    }

    fn lines_similar(&self, line1: &str, line2: &str) -> bool {
        let normalized1 = self.normalize_line(line1);
        let normalized2 = self.normalize_line(line2);
        
        if normalized1.is_empty() && normalized2.is_empty() {
            return true;
        }
        
        normalized1 == normalized2
    }

    fn normalize_line(&self, line: &str) -> String {
        // Remove whitespace and comments for comparison
        line.trim()
            .replace(" ", "")
            .split("//")
            .next()
            .unwrap_or("")
            .to_string()
    }
}

#[derive(Debug)]
struct DuplicateBlock {
    first_occurrence: usize,
    second_occurrence: usize,
    line_count: usize,
}

/// Analyzes test coverage for AI behavioral logic.
#[derive(Debug)]
pub struct CoverageAnalyzer {
    pub min_coverage_threshold: f32,
}

impl CoverageAnalyzer {
    pub fn new() -> Self {
        Self {
            min_coverage_threshold: 0.8, // 80% coverage
        }
    }

    pub fn analyze_coverage(&self, file_content: &str, file_path: &Path) -> ValidationResult {
        let start_time = Instant::now();
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Only analyze Rust files
        if !file_path.extension().map_or(false, |ext| ext == "rs") {
            return ValidationResult {
                file_path: file_path.to_path_buf(),
                passed: true,
                issues: vec![],
                suggestions: vec![],
                execution_time: start_time.elapsed(),
                check_type: CheckType::Manual,
            };
        }

        // Check for AI behavioral functions that need testing
        let ai_function_regex = Regex::new(r"fn\s+(\w*(?:ai|behavior|personality|emotion|social)\w*)\s*\(").unwrap();
        let test_function_regex = Regex::new(r"#\[test\]").unwrap();
        
        let ai_functions: Vec<_> = ai_function_regex.find_iter(file_content).collect();
        let test_count = test_function_regex.find_iter(file_content).count();
        
        if !ai_functions.is_empty() && test_count == 0 {
            issues.push(ValidationIssue {
                issue_type: IssueType::MissingTestCoverage,
                message: format!("AI behavioral functions found but no tests present ({} functions)", ai_functions.len()),
                file_path: file_path.to_path_buf(),
                line_number: 1,
                column_number: None,
                severity: IssueSeverity::Medium,
                suggestion: Some("Add unit tests for AI behavioral logic to ensure consistency".to_string()),
                code_example: Some("#[test]\nfn test_behavioral_function() {\n    // Test AI behavior\n}".to_string()),
            });
        }

        // Generate suggestions
        if !issues.is_empty() {
            suggestions.push("Focus test coverage on AI behavioral logic that affects agent believability".to_string());
            suggestions.push("Test personality modulation and emotional state changes".to_string());
        }

        ValidationResult {
            file_path: file_path.to_path_buf(),
            passed: issues.is_empty(),
            issues,
            suggestions,
            execution_time: start_time.elapsed(),
            check_type: CheckType::Manual,
        }
    }
}

