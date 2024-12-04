pub mod cli;
pub mod config;
pub mod progress;
pub mod reset;
pub mod suggestions;
pub mod validation;

use anyhow::Result;
use cli::Cli;

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        cli::Commands::Init { path } => {
            config::init_project(path)?;
            Ok(())
        }
        cli::Commands::Validate { path } => {
            validation::validate_project(path)?;
            Ok(())
        }
        cli::Commands::Progress { path } => {
            progress::show_progress(path)?;
            Ok(())
        }
        cli::Commands::Suggest { path } => {
            suggestions::generate_suggestions(path)?;
            Ok(())
        }
        cli::Commands::Reset { path, force } => {
            reset::reset_project(path, force)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
