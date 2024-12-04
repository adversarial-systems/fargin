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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config_toml(name: &str, description: &str) -> String {
        format!(
            r#"name = "{}"
description = "{}"
created_at = "{}"
last_updated = "{}"
goals = []
progress_markers = []"#,
            name,
            description,
            Utc::now().to_rfc3339(),
            Utc::now().to_rfc3339()
        )
    }

    fn setup_test_project() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create required directories
        for dir in [
            ".fargin",
            ".fargin/prompts",
            ".fargin/history",
            ".fargin/templates",
        ] {
            fs::create_dir_all(project_path.join(dir)).unwrap();
        }

        (temp_dir, project_path)
    }

    fn write_config_file(project_path: &Path, config_content: &str) -> Result<()> {
        let config_path = project_path.join(".fargin").join("config.toml");
        fs::write(config_path, config_content)?;
        Ok(())
    }

    #[test]
    fn test_directory_structure_validation() {
        let (temp_dir, project_path) = setup_test_project();

        // Test valid structure
        let check = validate_directory_structure(&project_path).unwrap();
        assert!(matches!(check.status, ValidationStatus::Pass));

        // Test missing directory
        fs::remove_dir_all(project_path.join(".fargin/prompts")).unwrap();
        let check = validate_directory_structure(&project_path).unwrap();
        assert!(matches!(check.status, ValidationStatus::Error));
        assert!(check.message.unwrap().contains("prompts"));

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_configuration_validation() {
        let (temp_dir, project_path) = setup_test_project();

        // Test valid config
        let config_content = create_test_config_toml("test-project", "A test project");
        write_config_file(&project_path, &config_content).unwrap();
        let config = ProjectConfig::load(&project_path).unwrap();
        let check = validate_configuration(&config).unwrap();
        assert!(matches!(check.status, ValidationStatus::Pass));

        // Test empty name
        let config_content = create_test_config_toml("", "A test project");
        write_config_file(&project_path, &config_content).unwrap();
        let config = ProjectConfig::load(&project_path).unwrap();
        let check = validate_configuration(&config).unwrap();
        assert!(matches!(check.status, ValidationStatus::Error));
        assert!(check.message.unwrap().contains("name"));

        // Test empty description
        let config_content = create_test_config_toml("test-project", "");
        write_config_file(&project_path, &config_content).unwrap();
        let config = ProjectConfig::load(&project_path).unwrap();
        let check = validate_configuration(&config).unwrap();
        assert!(matches!(check.status, ValidationStatus::Warning));
        assert!(check.message.unwrap().contains("description"));

        // Test invalid TOML
        let invalid_toml = r#"
            name = "test
            description = "Invalid TOML
        "#;
        write_config_file(&project_path, invalid_toml).unwrap();
        assert!(ProjectConfig::load(&project_path).is_err());

        // Test missing required fields
        let incomplete_toml = r#"
            name = "test-project"
            # missing description and other required fields
        "#;
        write_config_file(&project_path, incomplete_toml).unwrap();
        assert!(ProjectConfig::load(&project_path).is_err());

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert!(!report.has_errors());

        // Add passing check
        report.add_check(ValidationCheck {
            name: "Test Check 1".to_string(),
            status: ValidationStatus::Pass,
            message: None,
        });
        assert!(!report.has_errors());

        // Add warning check
        report.add_check(ValidationCheck {
            name: "Test Check 2".to_string(),
            status: ValidationStatus::Warning,
            message: Some("Warning message".to_string()),
        });
        assert!(!report.has_errors());

        // Add error check
        report.add_check(ValidationCheck {
            name: "Test Check 3".to_string(),
            status: ValidationStatus::Error,
            message: Some("Error message".to_string()),
        });
        assert!(report.has_errors());
    }

    #[test]
    fn test_full_project_validation() {
        let (temp_dir, project_path) = setup_test_project();

        // Test valid project
        let config_content = create_test_config_toml("test-project", "A test project");
        write_config_file(&project_path, &config_content).unwrap();

        let report = validate_project(project_path.clone()).unwrap();
        assert!(!report.has_errors());
        assert_eq!(report.checks.len(), 2); // Directory and Config checks

        // Test with invalid config
        let config_content = create_test_config_toml("", "");
        write_config_file(&project_path, &config_content).unwrap();

        let report = validate_project(project_path).unwrap();
        assert!(report.has_errors());

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_malformed_toml() {
        let (temp_dir, project_path) = setup_test_project();

        // Test with malformed TOML syntax
        let malformed_toml = r#"
            name = "test-project
            description = "Unclosed string
            [invalid section
        "#;
        write_config_file(&project_path, malformed_toml).unwrap();
        assert!(validate_project(project_path.clone()).is_err());

        // Test with valid TOML but invalid types
        let invalid_types_toml = r#"
            name = 123
            description = ["not", "a", "string"]
            created_at = "not-a-date"
            last_updated = "not-a-date"
            goals = "not-an-array"
            progress_markers = "not-an-array"
        "#;
        write_config_file(&project_path, invalid_types_toml).unwrap();
        assert!(validate_project(project_path).is_err());

        drop(temp_dir); // Cleanup
    }
}
