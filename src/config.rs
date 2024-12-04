use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub goals: Vec<String>,
    pub progress_markers: Vec<ProgressMarker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressMarker {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
}

impl ProjectConfig {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            goals: Vec::new(),
            progress_markers: Vec::new(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let config_dir = path.join(".fargin");
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");
        let config_str = toml::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let config_path = path.join(".fargin").join("config.toml");
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

/// Project development lifecycle configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevCycleConfig {
    /// Formatting tool and configuration
    pub format: FormatConfig,

    /// Linting tool and configuration
    pub lint: LintConfig,

    /// Testing configuration
    pub test: TestConfig,

    /// Git hooks for enforcing development practices
    pub git_hooks: GitHooksConfig,
}

/// Formatting configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatConfig {
    /// Formatting tool (e.g., rustfmt)
    pub tool: String,

    /// Configuration file for formatting
    pub config_file: Option<String>,

    /// Whether formatting is mandatory before commits
    pub mandatory: bool,
}

/// Linting configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LintConfig {
    /// Linting tool (e.g., clippy)
    pub tool: String,

    /// Linting configuration file
    pub config_file: Option<String>,

    /// Severity levels to enforce
    pub severity_levels: Vec<String>,

    /// Whether linting is mandatory
    pub mandatory: bool,
}

/// Testing configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestConfig {
    /// Test runner (e.g., cargo test)
    pub runner: String,

    /// Minimum test coverage percentage
    pub min_coverage: Option<f32>,

    /// Types of tests to run
    pub test_types: Vec<String>,

    /// Whether tests are mandatory before commits
    pub mandatory: bool,
}

/// Git hooks configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHooksConfig {
    /// Pre-commit hooks to enforce development practices
    pub pre_commit: Vec<String>,

    /// Pre-push hooks for additional checks
    pub pre_push: Vec<String>,
}

/// Default development cycle configuration
impl Default for DevCycleConfig {
    fn default() -> Self {
        DevCycleConfig {
            format: FormatConfig {
                tool: "rustfmt".to_string(),
                config_file: Some("rustfmt.toml".to_string()),
                mandatory: true,
            },
            lint: LintConfig {
                tool: "clippy".to_string(),
                config_file: Some(".clippy.toml".to_string()),
                severity_levels: vec!["warn".to_string(), "deny".to_string()],
                mandatory: true,
            },
            test: TestConfig {
                runner: "cargo test".to_string(),
                min_coverage: Some(70.0),
                test_types: vec!["unit".to_string(), "integration".to_string()],
                mandatory: true,
            },
            git_hooks: GitHooksConfig {
                pre_commit: vec!["format".to_string(), "lint".to_string()],
                pre_push: vec!["test".to_string()],
            },
        }
    }
}

/// Create initial project configuration
pub fn init_project_config(project_path: &Path) -> Result<()> {
    // Ensure .fargin directory exists
    let fargin_dir = project_path.join(".fargin");
    fs::create_dir_all(&fargin_dir)?;

    // Create default development cycle configuration
    let dev_cycle_config = DevCycleConfig::default();

    // Serialize configuration to TOML
    let toml_config = toml::to_string_pretty(&dev_cycle_config)
        .context("Failed to serialize dev cycle configuration")?;

    // Write configuration to file
    let config_path = fargin_dir.join("dev_cycle.toml");
    let mut config_file =
        fs::File::create(&config_path).context("Failed to create dev cycle configuration file")?;

    config_file
        .write_all(toml_config.as_bytes())
        .context("Failed to write dev cycle configuration")?;

    // Create initial git hooks
    create_git_hooks(project_path, &dev_cycle_config.git_hooks)?;

    // Create initial configuration files for format and lint tools
    create_tool_config_files(project_path)?;

    Ok(())
}

/// Create initial git hooks
fn create_git_hooks(project_path: &Path, git_hooks: &GitHooksConfig) -> Result<()> {
    let git_hooks_dir = project_path.join(".git/hooks");

    // Ensure git hooks directory exists
    fs::create_dir_all(&git_hooks_dir)?;

    // Pre-commit hook
    let pre_commit_path = git_hooks_dir.join("pre-commit");
    let mut pre_commit_file = fs::File::create(&pre_commit_path)?;
    pre_commit_file.write_all(b"#!/bin/sh\n\n# Pre-commit hooks\n")?;

    for hook in &git_hooks.pre_commit {
        let hook_content = match hook.as_str() {
            "format" => "cargo fmt\n",
            "lint" => "cargo clippy -- -D warnings\n",
            _ => continue,
        };
        pre_commit_file.write_all(hook_content.as_bytes())?;
    }

    // Make pre-commit hook executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&pre_commit_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_path, perms)?;
    }

    Ok(())
}

/// Create initial configuration files for formatting and linting
fn create_tool_config_files(project_path: &Path) -> Result<()> {
    // Rustfmt configuration
    let rustfmt_path = project_path.join("rustfmt.toml");
    let mut rustfmt_file = fs::File::create(&rustfmt_path)?;
    rustfmt_file.write_all(b"max_width = 100\nindent_style = \"Block\"\n")?;

    // Clippy configuration
    let clippy_path = project_path.join(".clippy.toml");
    let mut clippy_file = fs::File::create(&clippy_path)?;
    clippy_file.write_all(b"cognitive-complexity-threshold = 30\n")?;

    Ok(())
}

pub fn init_project(path: PathBuf) -> Result<()> {
    let config = ProjectConfig::new(
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        "A new LLM-driven project".to_string(),
    );
    config.save(&path)?;

    // Create standard project structure
    let dirs = [".fargin/prompts", ".fargin/history", ".fargin/templates"];

    for dir in dirs.iter() {
        fs::create_dir_all(path.join(dir))?;
    }

    init_project_config(&path)?;

    Ok(())
}
