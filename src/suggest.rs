use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::facts::Fact;
use crate::progress::show_progress;
use crate::validation::{validate_project, ValidationCheck};

/// Represents a suggestion with its context and priority
#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub category: SuggestionCategory,
    pub priority: SuggestionPriority,
    pub title: String,
    pub description: String,
    pub recommended_actions: Vec<String>,
}

/// Categories of suggestions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SuggestionCategory {
    Technical,
    Documentation,
    Refactoring,
    Testing,
    ProjectManagement,
}

/// Priority levels for suggestions
#[derive(Debug, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
pub enum SuggestionPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Generates context-aware suggestions for the project
pub fn generate_suggestions(
    project_path: &Path,
    suggestion_type: &str,
    verbosity: &str,
) -> Result<Vec<Suggestion>> {
    // Convert &Path to PathBuf
    let project_path_buf = project_path.to_path_buf();

    // Validate the project first
    let validation_report = validate_project(project_path_buf.clone())?;

    // Get project progress
    let progress_report = show_progress(project_path_buf.clone())?;

    // Collect facts
    let prompts = Fact::list(crate::facts::FactType::Prompt, &project_path_buf)?;
    let templates = Fact::list(crate::facts::FactType::Template, &project_path_buf)?;

    let mut suggestions = Vec::new();

    // Technical suggestions
    if suggestion_type == "all" || suggestion_type == "technical" {
        suggestions.extend(generate_technical_suggestions(
            &validation_report,
            &progress_report,
        ));
    }

    // Documentation suggestions
    if suggestion_type == "all" || suggestion_type == "documentation" {
        suggestions.extend(generate_documentation_suggestions(&prompts, &templates));
    }

    // Refactoring suggestions
    if suggestion_type == "all" || suggestion_type == "refactoring" {
        suggestions.extend(generate_refactoring_suggestions(&validation_report));
    }

    // Testing suggestions
    if suggestion_type == "all" || suggestion_type == "testing" {
        suggestions.extend(generate_testing_suggestions(&progress_report));
    }

    // Filter and adjust based on verbosity
    let filtered_suggestions = match verbosity {
        "brief" => suggestions
            .into_iter()
            .filter(|s| s.priority >= SuggestionPriority::High)
            .collect(),
        _ => suggestions,
    };

    // Sort suggestions by priority
    let mut sorted_suggestions = filtered_suggestions;
    sorted_suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

    Ok(sorted_suggestions)
}

fn generate_technical_suggestions(
    validation_report: &crate::validation::ValidationReport,
    progress_report: &crate::progress::ProgressReport,
) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Suggestions based on validation checks
    for check in &validation_report.checks {
        match check.status {
            crate::validation::ValidationStatus::Error => {
                suggestions.push(Suggestion {
                    category: SuggestionCategory::Technical,
                    priority: SuggestionPriority::Critical,
                    title: format!("Critical Technical Issue: {}", check.name),
                    description: check.message.clone().unwrap_or_default(),
                    recommended_actions: vec![
                        "Immediately address the reported configuration issue".to_string(),
                        "Review and correct project configuration".to_string(),
                    ],
                });
            }
            crate::validation::ValidationStatus::Warning => {
                suggestions.push(Suggestion {
                    category: SuggestionCategory::Technical,
                    priority: SuggestionPriority::Medium,
                    title: format!("Technical Configuration Warning: {}", check.name),
                    description: check.message.clone().unwrap_or_default(),
                    recommended_actions: vec![
                        "Review and improve project configuration".to_string(),
                        "Consider potential optimizations".to_string(),
                    ],
                });
            }
            _ => {}
        }
    }

    // Suggestions based on progress
    if progress_report.completed_markers < progress_report.total_markers / 2 {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Technical,
            priority: SuggestionPriority::High,
            title: "Low Progress Detected".to_string(),
            description: "Project progress is below 50% of planned markers".to_string(),
            recommended_actions: vec![
                "Review project timeline and milestones".to_string(),
                "Identify and address bottlenecks".to_string(),
                "Consider breaking down complex tasks".to_string(),
            ],
        });
    }

    suggestions
}

fn generate_documentation_suggestions(prompts: &[Fact], templates: &[Fact]) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Prompt documentation suggestions
    if prompts.is_empty() {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Documentation,
            priority: SuggestionPriority::High,
            title: "No Project Prompts Documented".to_string(),
            description: "No prompts have been captured for this project".to_string(),
            recommended_actions: vec![
                "Create initial project prompts".to_string(),
                "Document key interaction patterns".to_string(),
                "Capture successful prompt strategies".to_string(),
            ],
        });
    } else if prompts.len() < 5 {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Documentation,
            priority: SuggestionPriority::Medium,
            title: "Limited Prompt Documentation".to_string(),
            description: "Few prompts have been documented for this project".to_string(),
            recommended_actions: vec![
                "Expand prompt documentation".to_string(),
                "Add more context to existing prompts".to_string(),
                "Tag and categorize prompts".to_string(),
            ],
        });
    }

    // Template documentation suggestions
    if templates.is_empty() {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Documentation,
            priority: SuggestionPriority::Medium,
            title: "No Project Templates Documented".to_string(),
            description: "No templates have been created for this project".to_string(),
            recommended_actions: vec![
                "Create initial project templates".to_string(),
                "Identify common interaction patterns".to_string(),
                "Develop reusable template structures".to_string(),
            ],
        });
    }

    suggestions
}

fn generate_refactoring_suggestions(
    validation_report: &crate::validation::ValidationReport,
) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Look for potential refactoring opportunities
    let code_quality_checks: Vec<&ValidationCheck> = validation_report
        .checks
        .iter()
        .filter(|check| check.name.contains("code") || check.name.contains("style"))
        .collect();

    if !code_quality_checks.is_empty() {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Refactoring,
            priority: SuggestionPriority::Medium,
            title: "Code Quality Improvement Opportunities".to_string(),
            description: "Multiple code quality checks suggest potential refactoring".to_string(),
            recommended_actions: code_quality_checks
                .iter()
                .map(|check| check.message.clone().unwrap_or_default())
                .collect(),
        });
    }

    suggestions
}

fn generate_testing_suggestions(
    progress_report: &crate::progress::ProgressReport,
) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Testing coverage suggestions
    if progress_report.total_markers > 0 && progress_report.completed_markers == 0 {
        suggestions.push(Suggestion {
            category: SuggestionCategory::Testing,
            priority: SuggestionPriority::High,
            title: "No Progress Markers Completed".to_string(),
            description: "No project milestones have been marked as complete".to_string(),
            recommended_actions: vec![
                "Create initial test coverage for project milestones".to_string(),
                "Develop comprehensive test suite".to_string(),
                "Implement continuous integration checks".to_string(),
            ],
        });
    }

    suggestions
}

/// Serialize suggestions to JSON
pub fn serialize_suggestions(suggestions: &[Suggestion]) -> Result<String> {
    serde_json::to_string_pretty(suggestions).context("Failed to serialize suggestions")
}

/// Serialize suggestions to Markdown
pub fn serialize_suggestions_markdown(suggestions: &[Suggestion]) -> Result<String> {
    let mut markdown = String::new();

    for (i, suggestion) in suggestions.iter().enumerate() {
        markdown.push_str(&format!(
            "## Suggestion {}: {}\n\n",
            i + 1,
            suggestion.title
        ));
        markdown.push_str(&format!("**Category:** {:?}\n\n", suggestion.category));
        markdown.push_str(&format!("**Priority:** {:?}\n\n", suggestion.priority));
        markdown.push_str(&format!("**Description:** {}\n\n", suggestion.description));

        markdown.push_str("**Recommended Actions:**\n");
        for action in &suggestion.recommended_actions {
            markdown.push_str(&format!("- {}\n", action));
        }
        markdown.push_str("\n---\n\n");
    }

    Ok(markdown)
}
