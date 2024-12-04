use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Priority levels for features
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, ValueEnum)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "critical" => Ok(Priority::Critical),
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// Current status of a feature
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, ValueEnum, Hash)]
pub enum FeatureStatus {
    Proposed,
    InProgress,
    Implemented,
    Blocked,
    Deprecated,
}

impl FromStr for FeatureStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(FeatureStatus::Proposed),
            "inprogress" => Ok(FeatureStatus::InProgress),
            "implemented" => Ok(FeatureStatus::Implemented),
            "blocked" => Ok(FeatureStatus::Blocked),
            "deprecated" => Ok(FeatureStatus::Deprecated),
            _ => Err(format!("Invalid feature status: {}", s)),
        }
    }
}

/// Detailed feature representation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    /// Unique identifier for the feature
    pub id: String,

    /// Human-readable name of the feature
    pub name: String,

    /// Detailed description of the feature
    pub description: Option<String>,

    /// Current status of the feature
    pub status: FeatureStatus,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Priority of the feature
    pub priority: Priority,

    /// Who the feature is assigned to
    pub assigned_to: Option<String>,

    /// Estimated effort or complexity
    pub complexity: Option<u8>,

    /// Timestamp of feature creation
    pub created_at: DateTime<Utc>,

    /// Timestamp of last update
    pub updated_at: DateTime<Utc>,

    /// Related features or dependencies
    pub related_features: Vec<String>,

    /// Acceptance criteria
    pub acceptance_criteria: Vec<String>,
}

/// Feature management system
pub struct FeatureManager {
    /// Path to the project's .fargin directory
    project_path: PathBuf,

    /// In-memory cache of features
    features: HashMap<String, Feature>,
}

impl FeatureManager {
    /// Create a new feature manager
    pub fn new(project_path: &Path) -> Result<Self> {
        let mut feature_manager = Self {
            project_path: project_path.to_path_buf(),
            features: HashMap::new(),
        };

        feature_manager.load_features()?;

        Ok(feature_manager)
    }

    /// Load features from filesystem
    fn load_features(&mut self) -> Result<()> {
        let features_path = self.project_path.join(".fargin/features");
        fs::create_dir_all(&features_path)?;

        // Clear existing features
        self.features.clear();

        // Load markdown features, sorted by filename (which includes timestamp)
        let mut feature_files: Vec<_> = fs::read_dir(&features_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("md"))
            .collect();

        // Sort files by name to maintain chronological order
        feature_files.sort_by_key(|a| a.file_name());

        for entry in feature_files {
            let content = fs::read_to_string(entry.path())?;

            // Extract ID from filename
            let id = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
                .context("Invalid feature filename")?;

            // Extract name from content
            let name = content
                .lines()
                .find(|line| line.starts_with("# Feature: "))
                .map(|line| line.replace("# Feature: ", ""))
                .unwrap_or_else(|| id.clone());

            // Placeholder for parsing other fields
            let feature = Feature {
                id,
                name,
                description: None,
                status: FeatureStatus::Proposed,
                tags: Vec::new(),
                priority: Priority::Medium,
                assigned_to: None,
                complexity: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                related_features: Vec::new(),
                acceptance_criteria: Vec::new(),
            };

            self.features.insert(feature.id.clone(), feature);
        }

        Ok(())
    }

    /// Add a new feature
    pub fn add_feature(
        &mut self,
        name: String,
        description: Option<String>,
        tags: Option<Vec<String>>,
        priority: Option<Priority>,
        assigned_to: Option<String>,
    ) -> Result<String> {
        // Generate unique ID
        let id = self.generate_feature_id(&name);

        // Validate feature doesn't already exist
        if self.features.contains_key(&id) {
            return Err(anyhow::anyhow!("Feature with this name already exists"));
        }

        // Create feature
        let now = Utc::now();
        let feature = Feature {
            id: id.clone(),
            name,
            description,
            status: FeatureStatus::Proposed,
            tags: tags.unwrap_or_default(),
            priority: priority.unwrap_or(Priority::Medium),
            assigned_to,
            complexity: None,
            created_at: now,
            updated_at: now,
            related_features: Vec::new(),
            acceptance_criteria: Vec::new(),
        };

        // Save feature
        self.save_feature(&feature)?;

        // Cache feature
        self.features.insert(id.clone(), feature);

        Ok(id)
    }

    /// Update an existing feature
    pub fn update_feature(&mut self, id: &str, updates: FeatureUpdateRequest) -> Result<()> {
        let feature = self.features.get_mut(id).context("Feature not found")?;

        // Update feature details
        if let Some(description) = updates.description {
            feature.description = Some(description);
        }
        if let Some(status) = updates.status {
            feature.status = status;
        }
        if let Some(tags) = updates.tags {
            feature.tags = tags;
        }
        if let Some(priority) = updates.priority {
            feature.priority = priority;
        }
        if let Some(assigned_to) = updates.assigned_to {
            feature.assigned_to = Some(assigned_to);
        }

        // Save updated feature
        let feature_clone = feature.clone();
        self.save_feature(&feature_clone)?;

        Ok(())
    }

    /// List features with optional filtering
    pub fn list_features(
        &self,
        tag: Option<&str>,
        status: Option<FeatureStatus>,
        priority: Option<Priority>,
    ) -> Vec<&Feature> {
        self.features
            .values()
            .filter(|feature| {
                tag.is_none_or(|t| feature.tags.contains(&t.to_string()))
                    && status.is_none_or(|s| feature.status == s)
                    && priority.is_none_or(|p| feature.priority == p)
            })
            .collect()
    }

    /// Generate a unique feature ID
    fn generate_feature_id(&self, name: &str) -> String {
        // Use timestamp + slugified name for sortable, unique ID
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let slug = name
            .to_lowercase()
            .replace(char::is_whitespace, "_")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>();

        format!("{}__{}", timestamp, slug)
    }

    /// Save feature to filesystem
    fn save_feature(&self, feature: &Feature) -> Result<()> {
        let features_path = self.project_path.join(".fargin/features");
        fs::create_dir_all(&features_path)?;

        // Convert feature to markdown
        let markdown_content = format!(
            "# Feature: {}\n\n\
            ## Details\n\
            - **ID**: {}\n\
            - **Status**: {:?}\n\
            - **Priority**: {:?}\n\
            - **Assigned To**: {}\n\
            - **Created At**: {}\n\
            - **Updated At**: {}\n\n\
            ## Description\n\
            {}\n\n\
            ## Acceptance Criteria\n\
            {}\n\n\
            ## Related Features\n\
            {}\n\n\
            ## Tags\n\
            {}",
            feature.name,
            feature.id,
            feature.status,
            feature.priority,
            feature.assigned_to.as_deref().unwrap_or("Unassigned"),
            feature.created_at.to_rfc3339(),
            feature.updated_at.to_rfc3339(),
            feature.description.as_deref().unwrap_or("No description"),
            feature.acceptance_criteria.join("\n- "),
            feature.related_features.join(", "),
            feature.tags.join(", ")
        );

        let file_path = features_path.join(format!("{}.md", feature.id));
        fs::write(file_path, markdown_content)?;

        Ok(())
    }

    /// Get a specific feature by ID
    pub fn get_feature(&self, id: &str) -> Option<&Feature> {
        self.features.get(id)
    }

    /// Delete a feature
    pub fn delete_feature(&mut self, id: &str) -> Result<()> {
        // Remove from filesystem
        let feature_path = self
            .project_path
            .join(".fargin/features")
            .join(format!("{}.md", id));

        if feature_path.exists() {
            fs::remove_file(feature_path)?;
        }

        // Remove from in-memory cache
        self.features.remove(id);

        Ok(())
    }

    /// Generate implementation suggestions for a feature
    pub fn generate_feature_suggestions(
        &self,
        feature: &Feature,
        suggestion_type: Option<SuggestionType>,
        verbosity: &str,
    ) -> Vec<FeatureSuggestion> {
        // More intelligent suggestion generation
        let base_suggestions = match suggestion_type {
            Some(st) => self.generate_specific_suggestions(feature, st),
            None => self.generate_comprehensive_suggestions(feature),
        };

        // Apply verbosity filtering
        self.filter_suggestions_by_verbosity(base_suggestions, verbosity)
    }

    fn generate_specific_suggestions(
        &self,
        feature: &Feature,
        suggestion_type: SuggestionType,
    ) -> Vec<FeatureSuggestion> {
        match suggestion_type {
            SuggestionType::Implementation => self.generate_implementation_suggestions(feature),
            SuggestionType::Testing => self.generate_testing_suggestions(feature),
            SuggestionType::Optimization => self.generate_optimization_suggestions(feature),
            SuggestionType::Documentation => self.generate_documentation_suggestions(feature),
            SuggestionType::Architecture => self.generate_architecture_suggestions(feature),
            SuggestionType::Performance => self.generate_performance_suggestions(feature),
            SuggestionType::Security => self.generate_security_suggestions(feature),
            SuggestionType::Refactoring => self.generate_refactoring_suggestions(feature),
            SuggestionType::UserExperience => self.generate_ux_suggestions(feature),
        }
    }

    fn generate_comprehensive_suggestions(&self, feature: &Feature) -> Vec<FeatureSuggestion> {
        let mut suggestions = Vec::new();

        // Generate suggestions across different types
        suggestions.extend(self.generate_implementation_suggestions(feature));
        suggestions.extend(self.generate_testing_suggestions(feature));
        suggestions.extend(self.generate_documentation_suggestions(feature));

        // Add context-specific suggestions based on feature attributes
        if feature.priority == Priority::High {
            suggestions.extend(self.generate_optimization_suggestions(feature));
        }

        suggestions
    }

    fn generate_implementation_suggestions(&self, feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![FeatureSuggestion {
            id: format!("{}-impl-1", feature.id),
            suggestion_type: SuggestionType::Implementation,
            content: format!(
                "Recommended implementation approach for feature: {}",
                feature.name
            ),
            confidence: 0.8,
            complexity: 6,
            impact: SuggestionImpact::High,
            tags: vec!["design".to_string(), "architecture".to_string()],
            next_steps: vec![
                "Create detailed design document".to_string(),
                "Break down into smaller tasks".to_string(),
            ],
        }]
    }

    fn generate_testing_suggestions(&self, feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![FeatureSuggestion {
            id: format!("{}-test-1", feature.id),
            suggestion_type: SuggestionType::Testing,
            content: "Comprehensive test strategy for feature coverage".to_string(),
            confidence: 0.7,
            complexity: 5,
            impact: SuggestionImpact::Medium,
            tags: vec!["quality".to_string(), "validation".to_string()],
            next_steps: vec![
                "Define unit test cases".to_string(),
                "Create integration test plan".to_string(),
            ],
        }]
    }

    fn generate_documentation_suggestions(&self, feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![FeatureSuggestion {
            id: format!("{}-doc-1", feature.id),
            suggestion_type: SuggestionType::Documentation,
            content: "Recommended documentation approach and structure".to_string(),
            confidence: 0.9,
            complexity: 3,
            impact: SuggestionImpact::High,
            tags: vec!["docs".to_string(), "communication".to_string()],
            next_steps: vec![
                "Create user guide".to_string(),
                "Write technical documentation".to_string(),
            ],
        }]
    }

    // Add placeholder methods for other suggestion types
    fn generate_optimization_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn generate_architecture_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn generate_performance_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn generate_security_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn generate_refactoring_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn generate_ux_suggestions(&self, _feature: &Feature) -> Vec<FeatureSuggestion> {
        vec![]
    }

    fn filter_suggestions_by_verbosity(
        &self,
        suggestions: Vec<FeatureSuggestion>,
        verbosity: &str,
    ) -> Vec<FeatureSuggestion> {
        match verbosity {
            "low" => suggestions
                .into_iter()
                .filter(|s| s.confidence < 0.5)
                .collect(),
            "high" => suggestions
                .into_iter()
                .filter(|s| s.confidence > 0.7)
                .collect(),
            _ => suggestions, // Normal/default verbosity
        }
    }
}

/// Struct for feature update requests
#[derive(Default)]
pub struct FeatureUpdateRequest {
    pub description: Option<String>,
    pub status: Option<FeatureStatus>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<Priority>,
    pub assigned_to: Option<String>,
    pub complexity: Option<u8>,
    pub related_features: Option<Vec<String>>,
    pub acceptance_criteria: Option<Vec<String>>,
}

/// Types of feature suggestions
#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum SuggestionType {
    Implementation,
    Testing,
    Optimization,
    Documentation,
    Architecture,
    Performance,
    Security,
    Refactoring,
    UserExperience,
}

/// Detailed suggestion for feature implementation
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSuggestion {
    /// Unique identifier for the suggestion
    pub id: String,

    /// Type of suggestion
    pub suggestion_type: SuggestionType,

    /// Detailed suggestion content
    pub content: String,

    /// Confidence level of the suggestion
    pub confidence: f32,

    /// Estimated complexity (1-10 scale)
    pub complexity: u8,

    /// Potential impact of the suggestion
    pub impact: SuggestionImpact,

    /// Related tags or keywords
    pub tags: Vec<String>,

    /// Recommended next steps
    pub next_steps: Vec<String>,
}

/// Impact level of a suggestion
#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum SuggestionImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_feature_creation() -> Result<()> {
        let temp_dir = tempdir()?;
        let mut manager = FeatureManager::new(temp_dir.path())?;

        let feature_id = manager.add_feature(
            "User Authentication".to_string(),
            Some("Implement secure user login".to_string()),
            Some(vec!["security".to_string()]),
            Some(Priority::High),
            Some("dev-team".to_string()),
        )?;

        let feature = manager
            .get_feature(&feature_id)
            .expect("Feature should exist");

        assert_eq!(feature.name, "User Authentication");
        assert_eq!(feature.status, FeatureStatus::Proposed);
        assert_eq!(feature.tags, vec!["security"]);

        Ok(())
    }

    #[test]
    fn test_feature_update() -> Result<()> {
        let temp_dir = tempdir()?;
        let mut manager = FeatureManager::new(temp_dir.path())?;

        let feature_id = manager.add_feature(
            "Payment Integration".to_string(),
            Some("Add payment gateway".to_string()),
            None,
            None,
            None,
        )?;

        manager.update_feature(
            &feature_id,
            FeatureUpdateRequest {
                status: Some(FeatureStatus::InProgress),
                priority: Some(Priority::Critical),
                ..Default::default()
            },
        )?;

        let updated_feature = manager
            .get_feature(&feature_id)
            .expect("Feature should exist");

        assert_eq!(updated_feature.status, FeatureStatus::InProgress);
        assert_eq!(updated_feature.priority, Priority::Critical);

        Ok(())
    }
}
