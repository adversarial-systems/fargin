use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc};

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
    let dirs = [
        ".fargin/prompts",
        ".fargin/history",
        ".fargin/templates",
    ];
    
    for dir in dirs.iter() {
        fs::create_dir_all(path.join(dir))?;
    }
    
    Ok(())
}
