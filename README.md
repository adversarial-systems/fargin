# Fargin

Fargin is a Rust library that acts as a progress-driven design document manager for LLM-assisted development. It keeps LLMs focused on your project's specific goals and design intentions, preventing them from getting sidetracked or caught in local optimization loops.

## Purpose

The core purpose of Fargin is to "point to the sign" - maintaining a clear reference point for:
- Project-specific design goals and architectural decisions
- Established coding practices and standards
- Task management and progress tracking

This allows developers to:
1. Stay in the elevated headspace of their project's actual use cases
2. Prevent LLMs from waffling on minor issues or reverting established decisions
3. Maintain consistent progress towards project goals without getting caught in implementation details

## Features

- **Project Configuration**: Maintain a clear record of project goals, design decisions, and architectural choices
- **Progress Tracking**: Track development milestones and completed features
- **Design Guidance**: Keep LLMs aligned with your project's specific requirements and standards
- **Task Management**: Organize and prioritize development tasks while maintaining focus on core objectives

## Installation

### As a CLI Tool

```bash
cargo install fargin
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
fargin = "0.1.60"

# Or for minimal installation without CLI features:
fargin = { version = "0.1.60", default-features = false, features = ["minimal"] }
```

## Usage

### Command Line Interface

The library provides a CLI with the following commands:

1. Initialize a new project:
```bash
fargin init [path]
```

2. Validate project structure and configuration:
```bash
fargin validate [path]
```

3. Show project progress:
```bash
fargin progress [path]
```

4. Get suggestions for next steps:
```bash
fargin suggest [path]
```

5. Reset project (remove all LLM-sidekick files):
```bash
fargin reset [path] [--force]
```
Use the `--force` flag to skip confirmation prompt.

### Library Usage

#### Basic Example

```rust
use anyhow::Result;
use fargin::config::ProjectConfig;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize project with clear goals and design principles
    let mut config = ProjectConfig::new(
        "My Project".to_string(),
        "A focused project with clear design goals".to_string(),
    );
    
    // Add specific project goals
    config.goals.push("Implement efficient data processing pipeline".to_string());
    config.goals.push("Maintain strict type safety across interfaces".to_string());
    
    // Save configuration
    let project_dir = PathBuf::from("my_project");
    config.save(&project_dir)?;
    
    // Validate project
    fargin::validation::validate_project(project_dir.clone())?;
    
    // Show progress
    fargin::progress::show_progress(project_dir)?;
    
    Ok(())
}
```

#### Integration Example

```rust
use fargin::config::{ProjectConfig, ProgressMarker};

struct MyAIProject {
    config: ProjectConfig,
    project_dir: PathBuf,
}

impl MyAIProject {
    async fn new(name: &str, description: &str) -> Result<Self> {
        let project_dir = PathBuf::from(format!("projects/{}", name));
        let config = ProjectConfig::new(name.to_string(), description.to_string());
        config.save(&project_dir)?;
        
        Ok(Self { config, project_dir })
    }
    
    async fn track_progress(&mut self, marker: &str) -> Result<()> {
        self.config.progress_markers.push(ProgressMarker {
            name: marker.to_string(),
            description: "".to_string(),
            completed: false,
            completed_at: None,
        });
        self.config.save(&self.project_dir)?;
        Ok(())
    }
}

### Progress Tracking

```rust
use fargin::progress::show_progress;

// Check current progress against defined goals
let progress = show_progress(project_dir)?;
println!("Project completion: {}%", 
    (progress.completed_markers as f32 / progress.total_markers as f32) * 100.0);
```

## Why Fargin?

When working with LLMs on software projects, it's easy for the AI to get caught in local minima - focusing too much on optimizing small details while losing sight of the bigger picture. Fargin solves this by:

1. **Maintaining Context**: Keeping a clear record of project goals and design decisions
2. **Guiding Progress**: Ensuring development stays aligned with the project's intended direction
3. **Preventing Churn**: Reducing unnecessary back-and-forth on already-decided implementation details

## Project Structure

When initialized, the project creates the following directory structure:

```
.fargin/
├── config.toml     # Project configuration
├── prompts/        # LLM prompt templates
├── history/        # Development history
└── templates/      # Project templates
```

## Configuration

The project configuration is stored in `.fargin/config.toml` and includes:

- Project metadata (name, description)
- Project goals
- Progress markers
- Development history

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Authors

Copyright (c) 2024 adversarial.systems -- Fargin Contributors
