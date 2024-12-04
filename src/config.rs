use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl ProjectConfig {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let config_dir = path.join(".fargin");
        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let config_str = toml::to_string_pretty(self)?;

        // Ensure we write with full permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_dir)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&config_dir, perms)?;
        }

        fs::write(&config_path, config_str)?;

        println!("Project configuration saved to: {}", config_path.display());
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

/// Initialize a new Rust project using Cargo
pub fn init_rust_project(
    name: String,
    path: PathBuf,
    cargo_bin: String,
    template: Option<String>,
    with_fargin: bool,
    dry_run: bool,
) -> Result<()> {
    // Ensure path is relative to project root
    let project_root = std::env::current_dir()?;
    let absolute_path = project_root.join(path);

    println!(
        "Initializing Rust project: {} in project path: {}",
        name,
        absolute_path.display()
    );

    // Ensure project path exists
    if !dry_run {
        fs::create_dir_all(&absolute_path)?;
    }

    // Construct project path with project name
    let project_path = absolute_path.join(&name);

    // Construct Cargo command
    let mut cargo_cmd = Command::new(cargo_bin);
    cargo_cmd.arg("new").arg(&name).current_dir(&absolute_path);

    // Add template if specified
    if let Some(tmpl) = template {
        cargo_cmd.arg("--template").arg(tmpl);
    }

    // Execute Cargo command
    if !dry_run {
        println!("Executing Cargo command: {:?}", cargo_cmd);
        let status = cargo_cmd
            .status()
            .context("Failed to execute Cargo command")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Cargo project initialization failed"));
        }

        println!(
            "Project created successfully at: {}",
            project_path.display()
        );
    } else {
        println!("Dry run: Would execute command: {:?}", cargo_cmd);
    }

    // Create Fargin management structure if requested
    if with_fargin && !dry_run {
        println!(
            "Creating Fargin management structure in: {}",
            project_path.display()
        );
        create_fargin_structure(&project_path)?;
    } else if dry_run && with_fargin {
        println!(
            "Dry run: Would create Fargin management structure in: {:?}",
            project_path
        );
    }

    Ok(())
}

/// Initialize a project from a template
pub fn init_template_project(
    template: String,
    name: String,
    path: PathBuf,
    with_fargin: bool,
    dry_run: bool,
) -> Result<()> {
    // Ensure path is relative to project root
    let project_root = std::env::current_dir()?;
    let absolute_path = project_root.join(path);

    println!(
        "Initializing Template project: {} from template {} in project path: {}",
        name,
        template,
        absolute_path.display()
    );

    // Ensure project path exists
    if !dry_run {
        fs::create_dir_all(&absolute_path)?;
    }

    // Construct project path with project name
    let project_path = absolute_path.join(&name);

    // Example: Use cargo-generate for Rust templates
    if !dry_run {
        println!(
            "Executing cargo generate command for template: {}",
            template
        );
        let status = Command::new("cargo")
            .args(["generate", "--name", &name, "--git", &template])
            .current_dir(&absolute_path)
            .status()
            .context("Failed to generate project from template")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Template project initialization failed"));
        }

        println!(
            "Template project created successfully at: {}",
            project_path.display()
        );
    } else {
        println!(
            "Dry run: Would generate project from template: {} with name: {}",
            template, name
        );
    }

    if with_fargin && !dry_run {
        println!(
            "Creating Fargin management structure in: {}",
            project_path.display()
        );
        create_fargin_structure(&project_path)?;
    } else if dry_run && with_fargin {
        println!(
            "Dry run: Would create Fargin management structure in: {:?}",
            project_path
        );
    }

    Ok(())
}

/// Create a minimal project structure
pub fn init_minimal_project(
    name: String,
    path: PathBuf,
    project_type: String,
    with_fargin: bool,
    dry_run: bool,
) -> Result<()> {
    // Ensure path is relative to project root
    let project_root = std::env::current_dir()?;
    let absolute_path = project_root.join(path);

    println!(
        "Initializing Minimal {} project: {} in project path: {}",
        project_type,
        name,
        absolute_path.display()
    );

    // Construct project path with project name
    let project_path = absolute_path.join(&name);

    if !dry_run {
        fs::create_dir_all(&project_path)?;
    }

    // Create basic project structure based on type
    match project_type.as_str() {
        "rust" => {
            if !dry_run {
                // Create minimal Rust project structure
                println!("Creating minimal Rust project structure");
                fs::write(
                    project_path.join("Cargo.toml"),
                    format!(
                        r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
                        name
                    ),
                )?;

                fs::create_dir_all(project_path.join("src"))?;
                fs::write(
                    project_path.join("src/main.rs"),
                    "fn main() {\n    println!(\"Hello, world!\");\n}\n",
                )?;

                println!(
                    "Minimal Rust project created successfully at: {}",
                    project_path.display()
                );
            } else {
                println!("Dry run: Would create Rust project structure for: {}", name);
            }
        }
        "python" => {
            if !dry_run {
                // Create minimal Python project structure
                println!("Creating minimal Python project structure");
                fs::write(
                    project_path.join("pyproject.toml"),
                    format!(
                        r#"
[tool.poetry]
name = "{}"
version = "0.1.0"
description = ""
authors = []

[tool.poetry.dependencies]
python = "^3.8"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#,
                        name
                    ),
                )?;

                fs::create_dir_all(project_path.join("src"))?;
                fs::write(project_path.join("src/__init__.py"), "")?;
                fs::write(project_path.join("src/main.py"), "def main():\n    print('Hello, world!')\n\nif __name__ == '__main__':\n    main()\n")?;

                println!(
                    "Minimal Python project created successfully at: {}",
                    project_path.display()
                );
            } else {
                println!(
                    "Dry run: Would create Python project structure for: {}",
                    name
                );
            }
        }
        _ => return Err(anyhow::anyhow!("Unsupported project type")),
    }

    // Create Fargin management structure
    if with_fargin && !dry_run {
        println!(
            "Creating Fargin management structure in: {}",
            project_path.display()
        );
        create_fargin_structure(&project_path)?;
    } else if dry_run && with_fargin {
        println!(
            "Dry run: Would create Fargin management structure in: {:?}",
            project_path
        );
    }

    Ok(())
}

/// Create Fargin management structure
fn create_fargin_structure(project_path: &Path) -> Result<()> {
    // Create .fargin directory
    let fargin_dir = project_path.join(".fargin");
    fs::create_dir_all(&fargin_dir)?;

    // Ensure project path is absolute
    let absolute_project_path = fs::canonicalize(project_path)?;

    // Create subdirectories with more descriptive purposes
    let subdirs = ["prompts", "templates", "history", "artifacts", "docs"];

    for subdir in subdirs.iter() {
        let subdir_path = fargin_dir.join(subdir);
        fs::create_dir_all(&subdir_path)?;

        // Create a README for each subdirectory with descriptive content
        fs::write(
            subdir_path.join("README.md"),
            format!(
                "# {}\n\nThis directory is used for storing {} related to the project.",
                subdir.to_uppercase(),
                match *subdir {
                    "prompts" => "AI and human prompts",
                    "templates" => "project templates and boilerplate code",
                    "history" => "project changes and evolution",
                    "artifacts" => "generated files, logs, and build outputs",
                    "docs" => "project documentation and design notes",
                    _ => "project-related files",
                }
            ),
        )?;
    }

    // Create initial config file
    let project_name = absolute_project_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unnamed Project")
        .to_string();

    let config = ProjectConfig::new(
        project_name.clone(),
        "A project managed with Fargin CLI".to_string(),
    );
    config.save(&absolute_project_path)?;

    // Create a comprehensive README for the .fargin directory
    fs::write(
        fargin_dir.join("README.md"),
        format!(
            r#"# Fargin Project Management for {}

## Overview
This directory contains Fargin-specific project management artifacts and tools.

## Directory Structure
- `prompts/`: Store project-specific AI and human prompts
- `templates/`: Project templates and boilerplate code
- `history/`: Track project evolution and changes
- `artifacts/`: Store generated files, logs, and build outputs
- `docs/`: Project documentation and design notes

## Usage
Fargin helps manage project complexity, track features, and streamline development workflows.

### Recommended Practices
1. Use prompts to capture project requirements
2. Store reusable templates
3. Document project changes in history
4. Keep generated artifacts organized
5. Maintain comprehensive documentation

*Managed by Fargin CLI*
"#,
            project_name
        ),
    )?;

    // Create a basic .gitignore for the .fargin directory
    fs::write(
        fargin_dir.join(".gitignore"),
        r#"# Ignore sensitive or large artifacts
artifacts/large_files/
history/backups/
*.log
"#,
    )?;

    // Print debug information
    println!(
        "Fargin management structure created in: {}",
        fargin_dir.display()
    );
    println!("Subdirectories:");
    for subdir in subdirs.iter() {
        println!("- {}", subdir);
    }

    Ok(())
}
