use crate::config::ProjectConfig;
use anyhow::Result;
use std::path::PathBuf;

pub fn generate_suggestions(path: PathBuf) -> Result<Vec<Suggestion>> {
    let config = ProjectConfig::load(&path)?;
    let mut suggestions = Vec::new();

    // Analyze progress markers
    let incomplete_markers = config
        .progress_markers
        .iter()
        .filter(|m| !m.completed)
        .count();

    if incomplete_markers > 0 {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Progress,
            priority: SuggestionPriority::High,
            description: format!("Complete {} remaining progress markers", incomplete_markers),
            details: Some(
                "Focus on completing existing progress markers before adding new ones".to_string(),
            ),
        });
    }

    // Check for project goals
    if config.goals.is_empty() {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Planning,
            priority: SuggestionPriority::High,
            description: "Define project goals".to_string(),
            details: Some("Add clear, measurable goals to guide project development".to_string()),
        });
    }

    // Print suggestions
    if !suggestions.is_empty() {
        println!("\nSuggested Next Steps:");
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!(
                "{}. [{}] {}",
                i + 1,
                suggestion.priority,
                suggestion.description
            );
            if let Some(details) = &suggestion.details {
                println!("   {}", details);
            }
        }
    } else {
        println!("No suggestions at this time. Project is progressing well!");
    }

    Ok(suggestions)
}

#[derive(Debug)]
pub struct Suggestion {
    pub category: SuggestionCategory,
    pub priority: SuggestionPriority,
    pub description: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum SuggestionCategory {
    Planning,
    Progress,
    Quality,
    Documentation,
}

#[derive(Debug)]
pub enum SuggestionPriority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for SuggestionPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestionPriority::Low => write!(f, "Low"),
            SuggestionPriority::Medium => write!(f, "Medium"),
            SuggestionPriority::High => write!(f, "High"),
        }
    }
}
