use anyhow::Result;
use fargin::config::{ProgressMarker, ProjectConfig};
use fargin::progress::show_progress;
use fargin::validation::validate_project;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize a new project programmatically
    let project_dir = PathBuf::from("my_llm_project");

    // Create a custom project configuration
    let mut config = ProjectConfig::new(
        "My LLM Project".to_string(),
        "A custom LLM-driven project".to_string(),
    );

    // Add project goals
    config.goals = vec![
        "Implement natural language processing".to_string(),
        "Create API endpoints".to_string(),
        "Add documentation".to_string(),
    ];

    // Add progress markers
    config.progress_markers.push(ProgressMarker {
        name: "Initial Setup".to_string(),
        description: "Project structure and dependencies".to_string(),
        completed: true,
        completed_at: Some(chrono::Utc::now()),
    });

    // Save the configuration
    fs::create_dir_all(&project_dir)?;
    config.save(&project_dir)?;

    // Validate the project
    println!("Validating project...");
    let validation_result = validate_project(project_dir.clone())?;
    println!("Validation complete!");

    // Show progress
    println!("\nProject Progress:");
    show_progress(project_dir)?;

    Ok(())
}
