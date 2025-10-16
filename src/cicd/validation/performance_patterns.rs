use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Detects performance anti-patterns that could impact 60fps target.
#[derive(Debug)]
pub struct PerformancePatternDetector {
    pub f64_usage_detector: F64UsageDetector,
    pub sync_io_detector: SyncIoDetector,
    pub memory_allocation_detector: MemoryAllocationDetector,
    pub inefficient_query_detector: InefficientQueryDetector,
}

impl PerformancePatternDetector {
    pub fn new() -> Self {
        Self {
            f64_usage_detector: F64UsageDetector::new(),
            sync_io_detector: SyncIoDetector::new(),
            memory_allocation_detector: MemoryAllocationDetector::new(),
            inefficient_query_detector: InefficientQueryDetector::new(),
        }
    }

    pub fn validate_file(&self, file_content: &str, file_path: &Path) -> PerformanceValidationResult {
        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();

        // Run all performance validation checks
        let f64_issues = self.f64_usage_detector.detect_f64_usage(file_content, file_path);
        all_issues.extend(f64_issues);

        let sync_io_issues = self.sync_io_detector.detect_sync_io(file_content, file_path);
        all_issues.extend(sync_io_issues);

        let memory_issues = self.memory_allocation_detector.detect_excessive_allocation(file_content, file_path);
        all_issues.extend(memory_issues);

        let query_issues = self.inefficient_query_detector.detect_inefficient_queries(file_content, file_path);
        all_issues.extend(query_issues);

        // Generate suggestions based on issues found
        all_suggestions.extend(self.generate_suggestions(&all_issues));

        PerformanceValidationResult {
            file_path: file_path.to_path_buf(),
            passed: all_issues.is_empty(),
            issues: all_issues,
            suggestions: all_suggestions,
            execution_time: Duration::from_millis(0), // Will be set by caller
        }
    }

    fn generate_suggestions(&self, issues: &[PerformanceIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for issue in issues {
            match issue.issue_type {
                PerformanceIssueType::F64InHotPath => {
                    suggestions.push("Consider using f32 instead of f64 in performance-critical code paths for better cache efficiency".to_string());
                }
                PerformanceIssueType::SyncIoInHotPath => {
                    suggestions.push("Use async I/O or move synchronous operations to background threads to avoid frame drops".to_string());
                }
                PerformanceIssueType::ExcessiveMemoryAllocation => {
                    suggestions.push("Reduce memory allocations in hot paths by reusing objects or using object pools".to_string());
                }
                PerformanceIssueType::InefficientQueryPattern => {
                    suggestions.push("Optimize Bevy query patterns to reduce iteration overhead and improve cache locality".to_string());
                }
                PerformanceIssueType::MissingParallelProcessing => {
                    suggestions.push("Consider using parallel processing for independent operations to improve performance".to_string());
                }
            }
        }
        
        suggestions.dedup();
        suggestions
    }
}

impl Default for PerformancePatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects f64 usage in performance-critical code paths.
#[derive(Debug)]
pub struct F64UsageDetector {
    pub hot_path_patterns: Vec<Regex>,
    pub allowed_f64_contexts: HashSet<String>,
}

impl F64UsageDetector {
    pub fn new() -> Self {
        let hot_path_patterns = vec![
            Regex::new(r"fn \w*update\w*_system").unwrap(),
            Regex::new(r"fn \w*run\w*_system").unwrap(),
            Regex::new(r"Query<.*>").unwrap(),
            Regex::new(r"for .* in .*\.iter").unwrap(),
            Regex::new(r"\.par_iter").unwrap(),
        ];

        let mut allowed_f64_contexts = HashSet::new();
        allowed_f64_contexts.insert("scientific_calculation".to_string());
        allowed_f64_contexts.insert("high_precision_math".to_string());
        allowed_f64_contexts.insert("external_api_compatibility".to_string());
        allowed_f64_contexts.insert("configuration_parsing".to_string());

        Self {
            hot_path_patterns,
            allowed_f64_contexts,
        }
    }

    pub fn detect_f64_usage(&self, file_content: &str, file_path: &Path) -> Vec<PerformanceIssue> {
        let mut issues = Vec::new();
        let f64_regex = Regex::new(r"\bf64\b").unwrap();
        
        for mat in f64_regex.find_iter(file_content) {
            let line_number = self.find_line_number(file_content, mat.start());
            let line_content = self.get_line_content(file_content, line_number);
            
            // Check if this is in a hot path
            if self.is_in_hot_path(file_content, line_number) && !self.is_allowed_context(line_content) {
                let performance_impact = self.estimate_performance_impact(line_content);
                
                issues.push(PerformanceIssue {
                    issue_type: PerformanceIssueType::F64InHotPath,
                    message: format!(
                        "f64 usage detected in hot path at line {}. Consider using f32 for better performance.",
                        line_number
                    ),
                    file_path: file_path.to_path_buf(),
                    line_number,
                    severity: IssueSeverity::High,
                    performance_impact,
                    suggestion: "Replace f64 with f32 unless double precision is absolutely required. f32 provides better cache efficiency and SIMD performance.".to_string(),
                    fix_example: Some(self.generate_fix_example(line_content)),
                });
            }
        }
        
        issues
    }
    
    fn is_in_hot_path(&self, file_content: &str, line_number: usize) -> bool {
        // Get context around the line to determine if it's in a hot path
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number.saturating_sub(10);
        let end = (line_number + 10).min(lines.len());
        let context = lines[start..end].join("\n");
        
        self.hot_path_patterns.iter().any(|pattern| pattern.is_match(&context)) ||
        context.contains("fn update") ||
        context.contains("fn run") ||
        context.contains("Query<") ||
        context.contains("for ") && context.contains("iter")
    }

    fn is_allowed_context(&self, line_content: &str) -> bool {
        // Check for comments or context that indicate f64 is intentionally used
        line_content.contains("// f64 required") ||
        line_content.contains("// high precision") ||
        line_content.contains("// external API") ||
        line_content.contains("scientific") ||
        line_content.contains("precision")
    }

    fn estimate_performance_impact(&self, line_content: &str) -> PerformanceImpact {
        if line_content.contains("for ") || line_content.contains("iter") {
            PerformanceImpact::FrameTimeIncrease(0.2) // Higher impact in loops
        } else if line_content.contains("Query") {
            PerformanceImpact::FrameTimeIncrease(0.1) // Medium impact in queries
        } else {
            PerformanceImpact::FrameTimeIncrease(0.05) // Lower impact for single operations
        }
    }

    fn generate_fix_example(&self, line_content: &str) -> String {
        let fixed_line = line_content.replace("f64", "f32");
        format!("Change:\n  {}\nTo:\n  {}", line_content.trim(), fixed_line.trim())
    }

    fn find_line_number(&self, content: &str, position: usize) -> usize {
        content[..position].lines().count() + 1
    }

    fn get_line_content<'a>(&self, content: &'a str, line_number: usize) -> &'a str {
        content.lines().nth(line_number - 1).unwrap_or("")
    }
}

impl Default for F64UsageDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects synchronous I/O operations in performance-critical code paths.
#[derive(Debug)]
pub struct SyncIoDetector {
    pub sync_io_patterns: Vec<Regex>,
    pub allowed_sync_contexts: HashSet<String>,
}

impl SyncIoDetector {
    pub fn new() -> Self {
        let sync_io_patterns = vec![
            Regex::new(r"std::fs::(read_to_string|read|write|create|open)").unwrap(),
            Regex::new(r"File::(open|create)").unwrap(),
            Regex::new(r"\.read_to_string\(").unwrap(),
            Regex::new(r"\.write_all\(").unwrap(),
            Regex::new(r"std::net::(TcpStream|UdpSocket)").unwrap(),
            Regex::new(r"\.connect\(").unwrap(),
            Regex::new(r"\.send\(").unwrap(),
            Regex::new(r"\.recv\(").unwrap(),
        ];

        let mut allowed_sync_contexts = HashSet::new();
        allowed_sync_contexts.insert("initialization".to_string());
        allowed_sync_contexts.insert("configuration_loading".to_string());
        allowed_sync_contexts.insert("asset_loading".to_string());
        allowed_sync_contexts.insert("debug_output".to_string());

        Self {
            sync_io_patterns,
            allowed_sync_contexts,
        }
    }

    pub fn detect_sync_io(&self, file_content: &str, file_path: &Path) -> Vec<PerformanceIssue> {
        let mut issues = Vec::new();
        
        for pattern in &self.sync_io_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                let line_content = self.get_line_content(file_content, line_number);
                
                // Check if this is in a system or hot path
                if self.is_in_system_context(file_content, line_number) && !self.is_allowed_sync_context_in_function(file_content, line_number) {
                    issues.push(PerformanceIssue {
                        issue_type: PerformanceIssueType::SyncIoInHotPath,
                        message: format!(
                            "Synchronous I/O detected in system at line {}: '{}'. This can cause frame drops.",
                            line_number, mat.as_str()
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        severity: IssueSeverity::Critical,
                        performance_impact: PerformanceImpact::FrameDrops,
                        suggestion: self.generate_async_suggestion(mat.as_str()),
                        fix_example: Some(self.generate_async_fix_example(line_content, mat.as_str())),
                    });
                }
            }
        }
        
        issues
    }

    fn is_in_system_context(&self, file_content: &str, line_number: usize) -> bool {
        // Get context around the line to determine if it's in a Bevy system
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number.saturating_sub(20);
        let end = (line_number + 5).min(lines.len());
        let context = lines[start..end].join("\n");
        
        // Check for system function signatures
        let system_patterns = [
            r"fn \w+_system\s*\(",
            r"Query<",
            r"Res<",
            r"ResMut<",
            r"EventReader<",
            r"EventWriter<",
        ];
        
        system_patterns.iter().any(|pattern| {
            Regex::new(pattern).unwrap().is_match(&context)
        })
    }

    fn is_allowed_sync_context(&self, line_content: &str) -> bool {
        // Check for comments or context that indicate sync I/O is intentional
        line_content.contains("// sync I/O allowed") ||
        line_content.contains("// initialization") ||
        line_content.contains("// config loading") ||
        line_content.contains("// debug only") ||
        line_content.contains("#[cfg(debug_assertions)]")
    }

    fn is_allowed_sync_context_in_function(&self, file_content: &str, line_number: usize) -> bool {
        // Get the function context to check for allowed comments
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number.saturating_sub(10);
        let end = (line_number + 5).min(lines.len());
        let context = lines[start..end].join("\n");
        
        // Check for allowed context comments in the function
        context.contains("// sync I/O allowed") ||
        context.contains("// initialization") ||
        context.contains("// config loading") ||
        context.contains("// debug only") ||
        context.contains("#[cfg(debug_assertions)]")
    }

    fn generate_async_suggestion(&self, sync_operation: &str) -> String {
        match sync_operation {
            op if op.contains("std::fs::read") => {
                "Use async file reading with tokio::fs::read or Bevy's asset loading system".to_string()
            }
            op if op.contains("std::fs::write") => {
                "Use async file writing with tokio::fs::write or move to background thread".to_string()
            }
            op if op.contains("File::open") => {
                "Use tokio::fs::File::open for async file operations".to_string()
            }
            op if op.contains("TcpStream") || op.contains("UdpSocket") => {
                "Use tokio::net for async networking operations".to_string()
            }
            _ => {
                "Use async I/O or move to background thread. Consider using Bevy's asset loading system for file operations.".to_string()
            }
        }
    }

    fn generate_async_fix_example(&self, line_content: &str, sync_operation: &str) -> String {
        let async_replacement = match sync_operation {
            op if op.contains("std::fs::read_to_string") => {
                line_content.replace("std::fs::read_to_string", "tokio::fs::read_to_string")
            }
            op if op.contains("std::fs::write") => {
                line_content.replace("std::fs::write", "tokio::fs::write")
            }
            op if op.contains("File::open") => {
                line_content.replace("File::open", "tokio::fs::File::open")
            }
            _ => {
                format!("// Move to async context:\n{}", line_content)
            }
        };
        
        format!("Change:\n  {}\nTo:\n  {}", line_content.trim(), async_replacement.trim())
    }

    fn find_line_number(&self, content: &str, position: usize) -> usize {
        content[..position].lines().count() + 1
    }

    fn get_line_content<'a>(&self, content: &'a str, line_number: usize) -> &'a str {
        content.lines().nth(line_number - 1).unwrap_or("")
    }
}

impl Default for SyncIoDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects excessive memory allocation patterns that could impact performance.
#[derive(Debug)]
pub struct MemoryAllocationDetector {
    pub allocation_patterns: Vec<Regex>,
    pub hot_path_patterns: Vec<Regex>,
}

impl MemoryAllocationDetector {
    pub fn new() -> Self {
        let allocation_patterns = vec![
            Regex::new(r"Vec::new\(\)").unwrap(),
            Regex::new(r"vec!\[").unwrap(),
            Regex::new(r"HashMap::new\(\)").unwrap(),
            Regex::new(r"HashSet::new\(\)").unwrap(),
            Regex::new(r"String::new\(\)").unwrap(),
            Regex::new(r"\.to_string\(\)").unwrap(),
            Regex::new(r"\.clone\(\)").unwrap(),
            Regex::new(r"Box::new\(").unwrap(),
            Regex::new(r"Arc::new\(").unwrap(),
            Regex::new(r"Rc::new\(").unwrap(),
        ];

        let hot_path_patterns = vec![
            Regex::new(r"fn \w*update\w*_system").unwrap(),
            Regex::new(r"fn \w*run\w*_system").unwrap(),
            Regex::new(r"for .* in .*\.iter").unwrap(),
            Regex::new(r"\.par_iter").unwrap(),
        ];

        Self {
            allocation_patterns,
            hot_path_patterns,
        }
    }

    pub fn detect_excessive_allocation(&self, file_content: &str, file_path: &Path) -> Vec<PerformanceIssue> {
        let mut issues = Vec::new();
        
        for pattern in &self.allocation_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                let line_content = self.get_line_content(file_content, line_number);
                
                // Check if this allocation is in a hot path
                if self.is_in_hot_path(file_content, line_number) && !self.is_allowed_allocation(line_content) {
                    let severity = self.determine_allocation_severity(mat.as_str());
                    let suggestion = self.generate_allocation_suggestion(mat.as_str());
                    
                    issues.push(PerformanceIssue {
                        issue_type: PerformanceIssueType::ExcessiveMemoryAllocation,
                        message: format!(
                            "Memory allocation detected in hot path at line {}: '{}'. Consider reusing objects or using object pools.",
                            line_number, mat.as_str()
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        severity,
                        performance_impact: PerformanceImpact::MemoryPressure,
                        suggestion,
                        fix_example: Some(self.generate_allocation_fix_example(line_content, mat.as_str())),
                    });
                }
            }
        }
        
        issues
    }

    fn is_in_hot_path(&self, file_content: &str, line_number: usize) -> bool {
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number.saturating_sub(15);
        let end = (line_number + 5).min(lines.len());
        let context = lines[start..end].join("\n");
        
        self.hot_path_patterns.iter().any(|pattern| pattern.is_match(&context))
    }

    fn is_allowed_allocation(&self, line_content: &str) -> bool {
        line_content.contains("// allocation allowed") ||
        line_content.contains("// initialization") ||
        line_content.contains("// setup") ||
        line_content.contains("// one-time")
    }

    fn determine_allocation_severity(&self, allocation: &str) -> IssueSeverity {
        match allocation {
            alloc if alloc.contains("clone") => IssueSeverity::High,
            alloc if alloc.contains("to_string") => IssueSeverity::High,
            alloc if alloc.contains("Vec::new") || alloc.contains("vec!") => IssueSeverity::Medium,
            alloc if alloc.contains("HashMap") || alloc.contains("HashSet") => IssueSeverity::Medium,
            _ => IssueSeverity::Low,
        }
    }

    fn generate_allocation_suggestion(&self, allocation: &str) -> String {
        match allocation {
            alloc if alloc.contains("Vec::new") => {
                "Consider reusing Vec with clear() or using Vec::with_capacity() if size is known".to_string()
            }
            alloc if alloc.contains("HashMap::new") => {
                "Consider reusing HashMap with clear() or using HashMap::with_capacity()".to_string()
            }
            alloc if alloc.contains("clone") => {
                "Avoid cloning in hot paths. Consider using references or Cow<T>".to_string()
            }
            alloc if alloc.contains("to_string") => {
                "Use &str or format! with pre-allocated buffer instead of to_string()".to_string()
            }
            _ => {
                "Consider object pooling or reusing allocations to reduce memory pressure".to_string()
            }
        }
    }

    fn generate_allocation_fix_example(&self, line_content: &str, allocation: &str) -> String {
        let suggestion = match allocation {
            alloc if alloc.contains("Vec::new()") => {
                line_content.replace("Vec::new()", "Vec::with_capacity(expected_size)")
            }
            alloc if alloc.contains(".clone()") => {
                format!("// Consider using references instead:\n{}", line_content.replace(".clone()", ""))
            }
            alloc if alloc.contains(".to_string()") => {
                line_content.replace(".to_string()", " // Use &str if possible")
            }
            _ => format!("// Consider object pooling:\n{}", line_content),
        };
        
        format!("Change:\n  {}\nTo:\n  {}", line_content.trim(), suggestion.trim())
    }

    fn find_line_number(&self, content: &str, position: usize) -> usize {
        content[..position].lines().count() + 1
    }

    fn get_line_content<'a>(&self, content: &'a str, line_number: usize) -> &'a str {
        content.lines().nth(line_number - 1).unwrap_or("")
    }
}

impl Default for MemoryAllocationDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects inefficient Bevy query patterns and suggests optimizations.
#[derive(Debug)]
pub struct InefficientQueryDetector {
    pub inefficient_patterns: Vec<Regex>,
    pub parallel_opportunity_patterns: Vec<Regex>,
}

impl InefficientQueryDetector {
    pub fn new() -> Self {
        let inefficient_patterns = vec![
            Regex::new(r"Query<.*>.*Query<.*>").unwrap(), // Multiple queries that could be combined
            Regex::new(r"\.get\(.*\)\.unwrap\(\)").unwrap(), // Unsafe query access
            Regex::new(r"for .* in .*\.iter\(\).*for .* in .*\.iter\(\)").unwrap(), // Nested iterations
            Regex::new(r"\.iter\(\).*\.filter\(.*\).*\.collect\(\)").unwrap(), // Inefficient filtering
            Regex::new(r"\.single\(\)").unwrap(), // Single queries that could be cached
        ];

        let parallel_opportunity_patterns = vec![
            Regex::new(r"for .* in .*\.iter_mut\(\)").unwrap(),
            Regex::new(r"for .* in .*\.iter\(\)").unwrap(),
        ];

        Self {
            inefficient_patterns,
            parallel_opportunity_patterns,
        }
    }

    pub fn detect_inefficient_queries(&self, file_content: &str, file_path: &Path) -> Vec<PerformanceIssue> {
        let mut issues = Vec::new();
        
        // Check for inefficient query patterns
        for pattern in &self.inefficient_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                let line_content = self.get_line_content(file_content, line_number);
                
                if self.is_in_system_context(file_content, line_number) {
                    issues.push(PerformanceIssue {
                        issue_type: PerformanceIssueType::InefficientQueryPattern,
                        message: format!(
                            "Inefficient query pattern detected at line {}: Consider optimizing for better performance.",
                            line_number
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        severity: IssueSeverity::Medium,
                        performance_impact: PerformanceImpact::FrameTimeIncrease(0.1),
                        suggestion: self.generate_query_optimization_suggestion(mat.as_str()),
                        fix_example: Some(self.generate_query_fix_example(line_content, mat.as_str())),
                    });
                }
            }
        }

        // Check for parallel processing opportunities
        for pattern in &self.parallel_opportunity_patterns {
            for mat in pattern.find_iter(file_content) {
                let line_number = self.find_line_number(file_content, mat.start());
                let line_content = self.get_line_content(file_content, line_number);
                
                if self.is_in_system_context(file_content, line_number) && 
                   self.can_be_parallelized(file_content, line_number) {
                    issues.push(PerformanceIssue {
                        issue_type: PerformanceIssueType::MissingParallelProcessing,
                        message: format!(
                            "Parallel processing opportunity detected at line {}: Consider using par_iter for better performance.",
                            line_number
                        ),
                        file_path: file_path.to_path_buf(),
                        line_number,
                        severity: IssueSeverity::Low,
                        performance_impact: PerformanceImpact::FrameTimeIncrease(0.05),
                        suggestion: "Use par_iter() for parallel processing when operations are independent".to_string(),
                        fix_example: Some(self.generate_parallel_fix_example(line_content)),
                    });
                }
            }
        }
        
        issues
    }

    fn is_in_system_context(&self, file_content: &str, line_number: usize) -> bool {
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number.saturating_sub(10);
        let end = (line_number + 5).min(lines.len());
        let context = lines[start..end].join("\n");
        
        context.contains("fn ") && context.contains("_system") && context.contains("Query")
    }

    fn can_be_parallelized(&self, file_content: &str, line_number: usize) -> bool {
        let lines: Vec<&str> = file_content.lines().collect();
        let start = line_number;
        let end = (line_number + 10).min(lines.len());
        let loop_body = lines[start..end].join("\n");
        
        // Check if loop body has dependencies that prevent parallelization
        !loop_body.contains("EventWriter") && // Events can't be written in parallel
        !loop_body.contains("ResMut") && // Mutable resources can't be accessed in parallel
        !loop_body.contains("Commands") // Commands can't be used in parallel
    }

    fn generate_query_optimization_suggestion(&self, pattern: &str) -> String {
        if pattern.contains("Query<.*>.*Query<.*>") {
            "Consider combining multiple queries into a single query with joins or using With/Without filters".to_string()
        } else if pattern.contains(".get(") && pattern.contains(".unwrap()") {
            "Use safe query access with if let Ok() or .get().is_ok() instead of unwrap()".to_string()
        } else if pattern.contains("nested") {
            "Avoid nested iterations. Consider flattening or using more efficient algorithms".to_string()
        } else if pattern.contains(".single()") {
            "Cache single query results in a resource if accessed frequently".to_string()
        } else {
            "Optimize query pattern for better cache locality and reduced iteration overhead".to_string()
        }
    }

    fn generate_query_fix_example(&self, line_content: &str, pattern: &str) -> String {
        if line_content.contains(".get(") && line_content.contains(".unwrap()") {
            let safe_version = line_content.replace(".get(", ".get(").replace(".unwrap()", "?");
            format!("Change:\n  {}\nTo:\n  {}", line_content.trim(), safe_version.trim())
        } else if line_content.contains(".iter()") {
            let parallel_version = line_content.replace(".iter()", ".par_iter()");
            format!("Consider:\n  {}\nTo:\n  {}", line_content.trim(), parallel_version.trim())
        } else {
            format!("Optimize:\n  {}", line_content.trim())
        }
    }

    fn generate_parallel_fix_example(&self, line_content: &str) -> String {
        let parallel_version = line_content.replace(".iter()", ".par_iter()").replace(".iter_mut()", ".par_iter_mut()");
        format!("Change:\n  {}\nTo:\n  {}", line_content.trim(), parallel_version.trim())
    }

    fn find_line_number(&self, content: &str, position: usize) -> usize {
        content[..position].lines().count() + 1
    }

    fn get_line_content<'a>(&self, content: &'a str, line_number: usize) -> &'a str {
        content.lines().nth(line_number - 1).unwrap_or("")
    }
}

impl Default for InefficientQueryDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a performance validation check.
#[derive(Debug, Clone)]
pub struct PerformanceValidationResult {
    pub file_path: PathBuf,
    pub passed: bool,
    pub issues: Vec<PerformanceIssue>,
    pub suggestions: Vec<String>,
    pub execution_time: Duration,
}

/// Individual performance issue found in code.
#[derive(Debug, Clone)]
pub struct PerformanceIssue {
    pub issue_type: PerformanceIssueType,
    pub message: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub severity: IssueSeverity,
    pub performance_impact: PerformanceImpact,
    pub suggestion: String,
    pub fix_example: Option<String>,
}

/// Types of performance issues that can be detected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerformanceIssueType {
    F64InHotPath,
    SyncIoInHotPath,
    ExcessiveMemoryAllocation,
    InefficientQueryPattern,
    MissingParallelProcessing,
}

/// Severity levels for performance issues.
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    Critical,  // Blocks commit/push - causes frame drops
    High,      // Should be fixed soon - significant performance impact
    Medium,    // Should be addressed - moderate performance impact
    Low,       // Nice to fix - minor performance impact
}

/// Estimated performance impact of an issue.
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceImpact {
    FrameDrops,                    // Causes frame drops below 60fps
    FrameTimeIncrease(f32),        // Increases frame time by X milliseconds
    MemoryPressure,                // Increases memory allocation pressure
    CacheInefficiency,             // Reduces cache efficiency
    ThreadContention,              // Causes thread contention issues
}

impl std::fmt::Display for PerformanceIssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceIssueType::F64InHotPath => write!(f, "F64InHotPath"),
            PerformanceIssueType::SyncIoInHotPath => write!(f, "SyncIoInHotPath"),
            PerformanceIssueType::ExcessiveMemoryAllocation => write!(f, "ExcessiveMemoryAllocation"),
            PerformanceIssueType::InefficientQueryPattern => write!(f, "InefficientQueryPattern"),
            PerformanceIssueType::MissingParallelProcessing => write!(f, "MissingParallelProcessing"),
        }
    }
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Critical => write!(f, "Critical"),
            IssueSeverity::High => write!(f, "High"),
            IssueSeverity::Medium => write!(f, "Medium"),
            IssueSeverity::Low => write!(f, "Low"),
        }
    }
}

impl std::fmt::Display for PerformanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceImpact::FrameDrops => write!(f, "Frame drops"),
            PerformanceImpact::FrameTimeIncrease(ms) => write!(f, "Frame time increase: {:.2}ms", ms),
            PerformanceImpact::MemoryPressure => write!(f, "Memory pressure"),
            PerformanceImpact::CacheInefficiency => write!(f, "Cache inefficiency"),
            PerformanceImpact::ThreadContention => write!(f, "Thread contention"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_sync_io_detector_finds_issues_in_systems() {
        let detector = SyncIoDetector::new();
        
        let test_code = r#"
use bevy::prelude::*;
use std::fs::File;

fn bad_system(
    mut query: Query<&mut Transform>,
) {
    // This should be detected as sync I/O in hot path
    let data = std::fs::read_to_string("config.txt").unwrap();
    
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}

fn another_bad_system(
    query: Query<&Transform>,
) {
    // Network I/O in system
    let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
}

fn initialization_function() {
    // This should be allowed (not in system context)
    let config = std::fs::read_to_string("config.txt").unwrap();
}

fn debug_system(
    query: Query<&Transform>,
) {
    // sync I/O allowed - debug output
    std::fs::write("debug.log", "debug info").unwrap();
}
"#;
        
        let issues = detector.detect_sync_io(test_code, Path::new("test.rs"));
        
        // Should find at least 2 issues (read_to_string and TcpStream::connect in systems)
        assert!(issues.len() >= 2, "Should detect at least 2 sync I/O issues, found: {}", issues.len());
        
        // Check that we found the std::fs::read_to_string in bad_system
        let has_read_issue = issues.iter().any(|issue| 
            issue.message.contains("read_to_string")
        );
        assert!(has_read_issue, "Should detect std::fs::read_to_string in system");
        
        // Check that we found the TcpStream::connect in another_bad_system
        let has_tcp_issue = issues.iter().any(|issue| 
            issue.message.contains("connect")
        );
        assert!(has_tcp_issue, "Should detect TcpStream::connect in system");
        
        // Verify suggestions are provided
        for issue in &issues {
            assert!(!issue.suggestion.is_empty(), "Each issue should have a suggestion");
            assert!(issue.fix_example.is_some(), "Each issue should have a fix example");
        }
    }

    #[test]
    fn test_sync_io_detector_ignores_allowed_contexts() {
        let detector = SyncIoDetector::new();
        
        let test_code = r#"
fn initialization_function() {
    // This should be allowed (not in system context)
    let config = std::fs::read_to_string("config.txt").unwrap();
}

fn debug_system(
    query: Query<&Transform>,
) {
    // sync I/O allowed - debug output
    std::fs::write("debug.log", "debug info").unwrap();
}
"#;
        
        let issues = detector.detect_sync_io(test_code, Path::new("test.rs"));
        
        // Should find 0 issues because debug_system has allowed comment
        // and initialization_function is not in system context
        assert_eq!(issues.len(), 0, "Should not detect issues in allowed contexts, found: {}", issues.len());
    }

    #[test]
    fn test_sync_io_detector_patterns() {
        let detector = SyncIoDetector::new();
        
        // Test that all expected patterns are included
        let expected_patterns = [
            ("std::fs::read(\"test\")", "std::fs::read"),
            ("std::fs::write(\"test\", \"data\")", "std::fs::write"), 
            ("File::open(\"test\")", "File::open"),
            ("std::net::TcpStream::connect(\"addr\")", "connect"),
            ("file.read_to_string(&mut buf)", "read_to_string"),
            ("file.write_all(b\"data\")", "write_all"),
            ("stream.send(data)", "send"),
            ("stream.recv(&mut buf)", "recv"),
        ];
        
        for (pattern_code, expected_match) in &expected_patterns {
            let test_code = format!(r#"
fn test_system(query: Query<&Transform>) {{
    let result = {};
}}
"#, pattern_code);
            
            let issues = detector.detect_sync_io(&test_code, Path::new("test.rs"));
            assert!(issues.len() > 0, "Should detect pattern: {}", pattern_code);
            assert!(issues[0].message.contains(expected_match), 
                   "Issue message should contain '{}' for pattern '{}'", expected_match, pattern_code);
        }
    }

    #[test]
    fn test_sync_io_detector_suggestions() {
        let detector = SyncIoDetector::new();
        
        let test_cases = [
            ("std::fs::read", "tokio::fs::read"),
            ("std::fs::write", "tokio::fs::write"),
            ("File::open", "tokio::fs::File::open"),
            ("TcpStream", "tokio::net"),
            ("UdpSocket", "tokio::net"),
        ];
        
        for (sync_op, expected_suggestion) in &test_cases {
            let suggestion = detector.generate_async_suggestion(sync_op);
            assert!(suggestion.contains(expected_suggestion), 
                   "Suggestion for '{}' should contain '{}'", sync_op, expected_suggestion);
        }
    }

    #[test]
    fn test_performance_pattern_detector_integration() {
        let detector = PerformancePatternDetector::new();
        
        let test_code = r#"
use bevy::prelude::*;

fn problematic_system(
    mut query: Query<&mut Transform>,
) {
    // Multiple performance issues
    let precision: f64 = 10.0; // f64 in hot path
    let data = std::fs::read_to_string("config.txt").unwrap(); // sync I/O
    let mut vec = Vec::new(); // allocation in hot path
    
    for mut transform in query.iter_mut() {
        vec.push(transform.translation.clone()); // more allocation + clone
        transform.translation.x = precision as f32;
    }
}
"#;
        
        let result = detector.validate_file(test_code, Path::new("test.rs"));
        
        // Should find multiple types of issues
        assert!(!result.passed, "Should fail validation due to performance issues");
        assert!(result.issues.len() >= 3, "Should find at least 3 different performance issues");
        
        // Check that different issue types are detected
        let issue_types: std::collections::HashSet<_> = result.issues.iter()
            .map(|issue| &issue.issue_type)
            .collect();
        
        assert!(issue_types.contains(&PerformanceIssueType::F64InHotPath), "Should detect f64 usage");
        assert!(issue_types.contains(&PerformanceIssueType::SyncIoInHotPath), "Should detect sync I/O");
        assert!(issue_types.contains(&PerformanceIssueType::ExcessiveMemoryAllocation), "Should detect allocations");
        
        // Verify suggestions are provided
        assert!(!result.suggestions.is_empty(), "Should provide suggestions for fixing issues");
    }
}