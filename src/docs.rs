use crate::config::ProjectConfig;
use crate::facts::{Fact, FactType};
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct LLMDocumentation {
    pub project_info: ProjectInfo,
    pub prompts_guide: PromptsGuide,
    pub templates_guide: TemplatesGuide,
    pub interaction_history: InteractionHistory,
    pub best_practices: Vec<String>,
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub goals: Vec<String>,
    pub progress_markers: Vec<String>,
}

#[derive(Serialize)]
pub struct PromptsGuide {
    pub available_prompts: Vec<PromptInfo>,
    pub prompt_categories: Vec<String>,
    pub recommended_usage: Vec<String>,
}

#[derive(Serialize)]
pub struct PromptInfo {
    pub id: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub version: Option<String>,
    pub example_usage: String,
}

#[derive(Serialize)]
pub struct TemplatesGuide {
    pub available_templates: Vec<TemplateInfo>,
    pub template_categories: Vec<String>,
    pub usage_patterns: Vec<String>,
}

#[derive(Serialize)]
pub struct TemplateInfo {
    pub id: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub version: Option<String>,
    pub typical_use_cases: Vec<String>,
}

#[derive(Serialize)]
pub struct InteractionHistory {
    pub common_patterns: Vec<String>,
    pub successful_approaches: Vec<String>,
    pub lessons_learned: Vec<String>,
}

pub fn generate_llm_documentation(project_path: &Path) -> Result<LLMDocumentation> {
    // Load project configuration
    let config =
        ProjectConfig::load(project_path).context("Failed to load project configuration")?;

    // Load facts by type
    let prompts = Fact::list(FactType::Prompt, project_path).context("Failed to load prompts")?;
    let templates =
        Fact::list(FactType::Template, project_path).context("Failed to load templates")?;
    let history = Fact::list(FactType::History, project_path).context("Failed to load history")?;

    // Build project info
    let project_info = ProjectInfo {
        name: config.name,
        description: config.description,
        goals: config.goals,
        progress_markers: config
            .progress_markers
            .iter()
            .map(|m| format!("{}: {}", m.name, m.description))
            .collect(),
    };

    // Analyze prompts
    let prompts_guide = analyze_prompts(&prompts);

    // Analyze templates
    let templates_guide = analyze_templates(&templates);

    // Analyze interaction history
    let interaction_history = analyze_history(&history);

    // Generate best practices
    let best_practices = generate_best_practices(&prompts, &templates, &history);

    Ok(LLMDocumentation {
        project_info,
        prompts_guide,
        templates_guide,
        interaction_history,
        best_practices,
    })
}

fn analyze_prompts(prompts: &[Fact]) -> PromptsGuide {
    let mut categories = std::collections::HashSet::new();
    let mut available_prompts = Vec::new();

    for prompt in prompts {
        // Extract categories from tags
        categories.extend(prompt.metadata.tags.iter().cloned());

        // Create prompt info
        available_prompts.push(PromptInfo {
            id: prompt.id.clone(),
            description: prompt.metadata.description.clone(),
            tags: prompt.metadata.tags.clone(),
            version: prompt.metadata.version.clone(),
            example_usage: prompt.content.clone(),
        });
    }

    PromptsGuide {
        available_prompts,
        prompt_categories: categories.into_iter().collect(),
        recommended_usage: vec![
            "Start with high-level prompts before diving into specifics".to_string(),
            "Include context from previous interactions when relevant".to_string(),
            "Reference specific project goals in your prompts".to_string(),
        ],
    }
}

fn analyze_templates(templates: &[Fact]) -> TemplatesGuide {
    let mut categories = std::collections::HashSet::new();
    let mut available_templates = Vec::new();

    for template in templates {
        // Extract categories from tags
        categories.extend(template.metadata.tags.iter().cloned());

        // Create template info
        available_templates.push(TemplateInfo {
            id: template.id.clone(),
            description: template.metadata.description.clone(),
            tags: template.metadata.tags.clone(),
            version: template.metadata.version.clone(),
            typical_use_cases: template.metadata.references.clone(),
        });
    }

    TemplatesGuide {
        available_templates,
        template_categories: categories.into_iter().collect(),
        usage_patterns: vec![
            "Use templates as starting points for common tasks".to_string(),
            "Customize templates based on specific project needs".to_string(),
            "Reference templates in prompts for consistent output".to_string(),
        ],
    }
}

fn analyze_history(history: &[Fact]) -> InteractionHistory {
    // Extract common patterns and successful approaches from history
    let mut common_patterns = Vec::new();
    let mut successful_approaches = Vec::new();
    let mut lessons_learned = Vec::new();

    for entry in history {
        if entry.metadata.tags.contains(&"success".to_string()) {
            successful_approaches.push(entry.content.clone());
        }
        if entry.metadata.tags.contains(&"pattern".to_string()) {
            common_patterns.push(entry.content.clone());
        }
        if entry.metadata.tags.contains(&"lesson".to_string()) {
            lessons_learned.push(entry.content.clone());
        }
    }

    // If no explicit patterns/approaches found, provide defaults
    if common_patterns.is_empty() {
        common_patterns = vec![
            "Start with clear project goals".to_string(),
            "Break down complex tasks".to_string(),
            "Iterate based on feedback".to_string(),
        ];
    }

    if successful_approaches.is_empty() {
        successful_approaches = vec![
            "Use specific, context-rich prompts".to_string(),
            "Maintain consistent project structure".to_string(),
            "Document decisions and rationale".to_string(),
        ];
    }

    if lessons_learned.is_empty() {
        lessons_learned = vec![
            "Keep prompts focused and specific".to_string(),
            "Maintain clear project context".to_string(),
            "Document successful patterns".to_string(),
        ];
    }

    InteractionHistory {
        common_patterns,
        successful_approaches,
        lessons_learned,
    }
}

fn generate_best_practices(prompts: &[Fact], templates: &[Fact], history: &[Fact]) -> Vec<String> {
    let mut practices = Vec::new();

    // Analyze prompt patterns
    if !prompts.is_empty() {
        practices.push("Use structured prompts with clear objectives".to_string());
        practices.push("Include relevant context in each prompt".to_string());
    }

    // Analyze template usage
    if !templates.is_empty() {
        practices.push("Leverage existing templates for consistency".to_string());
        practices.push("Customize templates based on project needs".to_string());
    }

    // Analyze historical patterns
    if !history.is_empty() {
        practices.push("Learn from past interactions and outcomes".to_string());
        practices.push("Document successful approaches for future reference".to_string());
    }

    // Add general best practices
    practices.extend(vec![
        "Maintain clear project goals and progress markers".to_string(),
        "Use consistent terminology across interactions".to_string(),
        "Document important decisions and their rationale".to_string(),
        "Keep interaction history organized and tagged".to_string(),
    ]);

    practices
}
