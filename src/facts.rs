use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum FactType {
    Prompt,
    History,
    Template,
}

impl std::fmt::Display for FactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactType::Prompt => write!(f, "prompts"),
            FactType::History => write!(f, "history"),
            FactType::Template => write!(f, "templates"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    pub id: String,
    pub fact_type: FactType,
    pub content: String,
    pub metadata: FactMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactMetadata {
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub references: Vec<String>,
}

impl Fact {
    pub fn new(fact_type: FactType, content: String, metadata: FactMetadata) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            fact_type,
            content,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn save(&self, project_path: &Path) -> Result<()> {
        let fact_dir = project_path
            .join(".fargin")
            .join(self.fact_type.to_string());
        fs::create_dir_all(&fact_dir)?;

        let file_path = fact_dir.join(format!("{}.json", self.id));
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    fn list_json_files(project_path: &Path, fact_type: FactType) -> Result<Vec<DirEntry>> {
        let facts_dir = project_path.join(".fargin").join(fact_type.to_string());

        // Create directory if it doesn't exist
        fs::create_dir_all(&facts_dir)?;

        let entries = fs::read_dir(&facts_dir)?
            .filter_map(|entry| {
                entry
                    .ok()
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
            })
            .collect();

        Ok(entries)
    }

    fn search_facts(&self, query: &str) -> bool {
        self.content.to_lowercase().contains(&query.to_lowercase())
            || self
                .metadata
                .description
                .as_ref()
                .is_some_and(|d| d.to_lowercase().contains(&query.to_lowercase()))
            || self
                .metadata
                .tags
                .iter()
                .any(|t| t.to_lowercase().contains(&query.to_lowercase()))
    }

    pub fn load(fact_id: &str, fact_type: FactType, project_path: &Path) -> Result<Self> {
        let file_path = project_path
            .join(".fargin")
            .join(fact_type.to_string())
            .join(format!("{}.json", fact_id));
        let content = fs::read_to_string(file_path)?;
        let fact = serde_json::from_str(&content)?;
        Ok(fact)
    }

    pub fn list(fact_type: FactType, project_path: &Path) -> Result<Vec<Self>> {
        let entries = Self::list_json_files(project_path, fact_type)?;

        let mut facts = Vec::new();
        for entry in entries {
            let content = fs::read_to_string(entry.path())?;
            let fact: Fact = serde_json::from_str(&content)?;
            facts.push(fact);
        }

        // Sort by creation date, newest first
        facts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(facts)
    }

    pub fn update(&mut self, content: String, metadata: Option<FactMetadata>) -> Result<()> {
        self.content = content;
        if let Some(meta) = metadata {
            self.metadata = meta;
        }
        self.updated_at = Utc::now();
        Ok(())
    }
}

pub fn search_facts(
    query: &str,
    fact_type: Option<FactType>,
    project_path: &Path,
) -> Result<Vec<Fact>> {
    let fact_types = match fact_type {
        Some(ft) => vec![ft],
        None => vec![FactType::Prompt, FactType::History, FactType::Template],
    };

    let mut results = Vec::new();
    for ft in fact_types {
        let facts = Fact::list(ft, project_path)?;
        for fact in facts {
            if fact.search_facts(query) {
                results.push(fact);
            }
        }
    }

    // Sort by relevance (for now, just by date)
    results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(results)
}
