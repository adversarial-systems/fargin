use crate::config::ProjectConfig;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn validate_project(path: PathBuf) -> Result<ValidationReport> {
    let config = ProjectConfig::load(&path).context("Failed to load project configuration")?;

    let mut report = ValidationReport::new();

    // Validate project structure
    report.add_check(validate_directory_structure(&path)?);

    // Validate configuration
    report.add_check(validate_configuration(&config)?);

    Ok(report)
}

#[derive(Debug)]
pub struct ValidationReport {
    pub checks: Vec<ValidationCheck>,
}

#[derive(Debug)]
pub struct ValidationCheck {
    pub name: String,
    pub status: ValidationStatus,
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum ValidationStatus {
    Pass,
    Warning,
    Error,
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationReport {
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    pub fn add_check(&mut self, check: ValidationCheck) {
        self.checks.push(check);
    }

    pub fn has_errors(&self) -> bool {
        self.checks
            .iter()
            .any(|check| matches!(check.status, ValidationStatus::Error))
    }
}

fn validate_directory_structure(path: &Path) -> Result<ValidationCheck> {
    let required_dirs = [
        ".fargin",
        ".fargin/prompts",
        ".fargin/history",
        ".fargin/templates",
    ];

    for dir in required_dirs.iter() {
        if !path.join(dir).exists() {
            return Ok(ValidationCheck {
                name: "Directory Structure".to_string(),
                status: ValidationStatus::Error,
                message: Some(format!("Missing required directory: {}", dir)),
            });
        }
    }

    Ok(ValidationCheck {
        name: "Directory Structure".to_string(),
        status: ValidationStatus::Pass,
        message: None,
    })
}

fn validate_configuration(config: &ProjectConfig) -> Result<ValidationCheck> {
    if config.name.is_empty() {
        return Ok(ValidationCheck {
            name: "Configuration".to_string(),
            status: ValidationStatus::Error,
            message: Some("Project name cannot be empty".to_string()),
        });
    }

    if config.description.is_empty() {
        return Ok(ValidationCheck {
            name: "Configuration".to_string(),
            status: ValidationStatus::Warning,
            message: Some("Project description is empty".to_string()),
        });
    }

    Ok(ValidationCheck {
        name: "Configuration".to_string(),
        status: ValidationStatus::Pass,
        message: None,
    })
}
