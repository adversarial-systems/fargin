use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// LLM Sidekick - A project management tool for LLM-driven development
///
/// This tool helps manage and track the progress of projects that use Large Language Models (LLMs)
/// for development. It provides project initialization, validation, progress tracking, and
/// intelligent suggestions for next steps.
#[derive(Parser)]
#[command(
    author = "LLM Sidekick Contributors",
    version,
    about,
    long_about = "A comprehensive project management tool for LLM-driven development. \
    It helps maintain project structure, track progress, and provide guidance \
    throughout the development lifecycle."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new LLM-driven project
    ///
    /// Creates a new project structure with necessary configuration files and directories.
    /// This includes setting up the .llm-sidekick directory with prompts, history, and templates.
    Init {
        /// Path to the project directory
        ///
        /// If not specified, uses the current directory. The directory will be created
        /// if it doesn't exist.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },
    /// Validate project configuration and structure
    ///
    /// Checks that all required files and directories are present and properly configured.
    /// Also validates the content of configuration files and project structure.
    Validate {
        /// Path to the project directory
        ///
        /// The directory to validate. Must contain a .llm-sidekick configuration.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },
    /// Show project progress and status
    ///
    /// Displays detailed information about project progress, including completed
    /// and pending tasks, milestones, and recent activity.
    Progress {
        /// Path to the project directory
        ///
        /// The directory containing the project to analyze. Must have a .llm-sidekick configuration.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },
    /// Generate suggestions for next steps
    ///
    /// Analyzes the current project state and provides intelligent suggestions
    /// for what to work on next, based on progress and project goals.
    Suggest {
        /// Path to the project directory
        ///
        /// The directory containing the project to analyze. Must have a .llm-sidekick configuration.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },
    /// Reset project by removing all LLM-sidekick related files
    ///
    /// Removes all LLM-sidekick configuration, history, and generated files from the project.
    /// This action cannot be undone. Use with caution.
    Reset {
        /// Path to the project directory
        ///
        /// The directory containing the LLM-sidekick configuration to remove.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
        /// Force reset without confirmation
        ///
        /// Skip the confirmation prompt and immediately remove all LLM-sidekick files.
        /// Use with caution as this operation cannot be undone.
        #[arg(short, long, help = "Skip confirmation prompt")]
        force: bool,
    },
}
