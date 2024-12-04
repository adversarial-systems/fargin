pub mod cli;
pub mod config;
pub mod docs;
pub mod facts;
pub mod progress;
pub mod reset;
pub mod suggest;
pub mod suggestions;
pub mod validation;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use cli::Commands;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => config::init_project(path),
        Commands::Validate { path } => {
            validation::validate_project(path)?;
            Ok(())
        }
        Commands::Progress { path } => {
            progress::show_progress(path)?;
            Ok(())
        }
        Commands::Suggest {
            path,
            suggestion_type,
            verbosity,
        } => {
            suggest::generate_suggestions(&path, &suggestion_type, &verbosity)?;
            Ok(())
        }
        Commands::Reset { path, force } => reset::reset_project(path, force),
        Commands::Add { .. }
        | Commands::List { .. }
        | Commands::Show { .. }
        | Commands::Update { .. }
        | Commands::Search { .. }
        | Commands::Docs { .. } => {
            // These commands are handled directly in main.rs
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_suggest() {
        // Placeholder test for Suggest command
        use crate::suggest;
        let path = std::path::PathBuf::from(".");
        let suggestion_type = "all".to_string();
        let verbosity = "brief".to_string();

        // This will just ensure the function can be called without panicking
        let _ = suggest::generate_suggestions(&path, &suggestion_type, &verbosity);
    }
}
