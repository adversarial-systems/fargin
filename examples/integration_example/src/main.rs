use anyhow::Result;
use fargin::{
    config::ProjectConfig,
    progress::show_progress,
    validation::validate_project,
};
use std::path::PathBuf;

struct MyAIProject {
    config: ProjectConfig,
    project_dir: PathBuf,
}

impl MyAIProject {
    async fn new(name: &str, description: &str) -> Result<Self> {
        let project_dir = PathBuf::from(format!("projects/{}", name));
        let config = ProjectConfig::new(name.to_string(), description.to_string());
        config.save(&project_dir)?;
        
        Ok(Self {
            config,
            project_dir,
        })
    }
    
    async fn add_goal(&mut self, goal: &str) -> Result<()> {
        self.config.goals.push(goal.to_string());
        self.config.save(&self.project_dir)?;
        Ok(())
    }
    
    async fn track_progress(&mut self, marker: &str, description: &str) -> Result<()> {
        self.config.progress_markers.push(fargin::config::ProgressMarker {
            name: marker.to_string(),
            description: description.to_string(),
            completed: false,
            completed_at: None,
        });
        self.config.save(&self.project_dir)?;
        Ok(())
    }
    
    async fn validate(&self) -> Result<()> {
        fargin::validation::validate_project(self.project_dir.clone())?;
        Ok(())
    }
    
    async fn show_progress(&self) -> Result<()> {
        fargin::progress::show_progress(self.project_dir.clone())?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize a new project
    let project_dir = PathBuf::from("test_project");
    let config = ProjectConfig::new(
        "Test Project".to_string(),
        "A test project using Fargin".to_string(),
    );
    
    // Save the configuration
    std::fs::create_dir_all(&project_dir)?;
    config.save(&project_dir)?;
    
    // Validate the project
    let validation = fargin::validation::validate_project(project_dir.clone())?;
    if validation.has_errors() {
        println!("Project validation failed!");
        return Ok(());
    }
    
    // Show progress
    let progress = fargin::progress::show_progress(project_dir.clone())?;
    println!("Project is {}% complete", 
             (progress.completed_markers as f32 / progress.total_markers as f32) * 100.0);
    
    Ok(())
}
