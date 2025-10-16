#[cfg(test)]
mod tests {
    use crate::cicd::git_hooks::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_hook_manager_creation() {
        let manager = GitHookManager::new();

        // Verify default configuration
        assert!(
            manager
                .hook_configurations
                .contains_key(&HookType::PreCommit)
        );
        assert!(manager.hook_configurations.contains_key(&HookType::PrePush));

        // Verify pre-commit timeout
        let pre_commit_config = manager
            .hook_configurations
            .get(&HookType::PreCommit)
            .unwrap();
        assert_eq!(pre_commit_config.timeout_seconds, 45);

        // Verify pre-push timeout
        let pre_push_config = manager.hook_configurations.get(&HookType::PrePush).unwrap();
        assert_eq!(pre_push_config.timeout_seconds, 600);

        // Verify bypass mechanisms
        assert!(manager.bypass_mechanisms.emergency_bypass_enabled);
        assert_eq!(
            manager.bypass_mechanisms.bypass_environment_variable,
            Some("EMERGENCY_BYPASS".to_string())
        );
    }

    #[test]
    fn test_hook_installation_error_handling() {
        let mut manager = GitHookManager::new();

        // Test installation in non-git directory
        let temp_dir = TempDir::new().unwrap();
        let result = manager.install_hooks(temp_dir.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            HookInstallationError::RepositoryNotFound(_) => {
                // Expected error
            }
            _ => panic!("Expected RepositoryNotFound error"),
        }
    }

    #[test]
    fn test_hook_type_filename() {
        assert_eq!(HookType::PreCommit.filename(), "pre-commit");
        assert_eq!(HookType::PrePush.filename(), "pre-push");
        assert_eq!(HookType::CommitMsg.filename(), "commit-msg");
        assert_eq!(HookType::PostCheckout.filename(), "post-checkout");
    }

    #[test]
    fn test_error_formatter() {
        let formatter = DefaultErrorFormatter;

        let timeout_error = HookError::Timeout { timeout: 30 };
        let formatted = formatter.format_error(&timeout_error);
        assert!(formatted.contains("timed out after 30s"));

        let suggestion = formatter.suggest_fix(&timeout_error);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("increase timeout"));

        let missing_tool_error = HookError::MissingTool {
            tool: "cargo".to_string(),
        };
        let suggestion = formatter.suggest_fix(&missing_tool_error);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("rustup.rs"));
    }

    #[test]
    fn test_progress_reporter() {
        let reporter = ConsoleProgressReporter;

        // These should not panic
        reporter.report_progress("test", 0.5, "Testing progress");
        reporter.report_completion("test", std::time::Duration::from_secs(1), true);
        reporter.report_completion("test", std::time::Duration::from_secs(1), false);
    }

    #[test]
    fn test_bypass_condition_types() {
        // Test that bypass conditions can be created
        let _env_condition =
            BypassCondition::EnvironmentVariable("TEST".to_string(), "1".to_string());
        let _msg_condition = BypassCondition::CommitMessageContains("BYPASS".to_string());
        let _file_condition = BypassCondition::FileExists(std::path::PathBuf::from("bypass.txt"));
        let _branch_condition = BypassCondition::BranchName("emergency".to_string());
    }

    #[test]
    fn test_hook_configuration() {
        let mut manager = GitHookManager::new();

        // Test configuring a hook
        let custom_config = HookConfiguration {
            script_path: std::path::PathBuf::from("custom-hook"),
            timeout_seconds: 120,
            required_tools: vec!["custom-tool".to_string()],
            environment_variables: std::collections::HashMap::new(),
            bypass_conditions: vec![],
        };

        manager.configure_hook(HookType::CommitMsg, custom_config);

        let configured = manager
            .hook_configurations
            .get(&HookType::CommitMsg)
            .unwrap();
        assert_eq!(configured.timeout_seconds, 120);
        assert_eq!(configured.required_tools, vec!["custom-tool".to_string()]);
    }
}
