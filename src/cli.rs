use clap::{Parser, Subcommand, ValueEnum};
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FactTypeArg {
    Prompt,
    History,
    Template,
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

        /// Type of suggestions to generate
        ///
        /// Allows filtering suggestions by type. Options include:
        /// - 'all': Generate all types of suggestions (default)
        /// - 'technical': Focus on technical implementation suggestions
        /// - 'documentation': Prioritize documentation and knowledge capture
        /// - 'refactoring': Suggest code improvements and optimizations
        /// - 'testing': Recommend additional test coverage
        #[arg(short, long, default_value = "all", value_name = "TYPE")]
        suggestion_type: String,

        /// Verbosity level for suggestions
        ///
        /// Controls the level of detail in generated suggestions.
        /// - 'brief': Concise, high-level suggestions
        /// - 'detailed': In-depth analysis and recommendations
        #[arg(short, long, default_value = "brief", value_name = "VERBOSITY")]
        verbosity: String,
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
    /// Add a new fact (prompt, history, or template)
    Add {
        /// Type of fact to add
        #[arg(value_enum)]
        fact_type: FactTypeArg,

        /// Content of the fact
        #[arg(value_name = "CONTENT")]
        content: String,

        /// Description of the fact
        #[arg(short, long, value_name = "DESC")]
        description: Option<String>,

        /// Tags to associate with the fact
        #[arg(short, long, value_name = "TAGS", value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Version of the fact
        #[arg(short, long, value_name = "VER")]
        version: Option<String>,

        /// References to other facts or external resources
        #[arg(short, long, value_name = "REFS", value_delimiter = ',')]
        references: Option<Vec<String>>,

        /// Path to the project directory
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },

    /// List facts of a specific type
    List {
        /// Type of facts to list
        #[arg(value_enum)]
        fact_type: FactTypeArg,

        /// Path to the project directory
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },

    /// Show details of a specific fact
    Show {
        /// ID of the fact to show
        #[arg(value_name = "ID")]
        fact_id: String,

        /// Type of the fact
        #[arg(value_enum)]
        fact_type: FactTypeArg,

        /// Path to the project directory
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },

    /// Update an existing fact
    Update {
        /// ID of the fact to update
        #[arg(value_name = "ID")]
        fact_id: String,

        /// Type of the fact
        #[arg(value_enum)]
        fact_type: FactTypeArg,

        /// Optional new content for the fact
        #[arg(short, long, value_name = "CONTENT")]
        content: Option<String>,

        /// New description for the fact
        #[arg(short, long, value_name = "DESC")]
        description: Option<String>,

        /// New tags for the fact
        #[arg(short, long, value_name = "TAGS", value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// New version for the fact
        #[arg(short, long, value_name = "VER")]
        version: Option<String>,

        /// New references for the fact
        #[arg(short, long, value_name = "REFS", value_delimiter = ',')]
        references: Option<Vec<String>>,

        /// Path to the project directory
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },

    /// Search facts
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Type of facts to search (optional)
        #[arg(value_enum)]
        fact_type: Option<FactTypeArg>,

        /// Path to the project directory
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,
    },

    /// Generate LLM-friendly documentation
    ///
    /// Creates comprehensive documentation about the project, including project details,
    /// prompts, templates, interaction history, and best practices.
    Docs {
        /// Path to the project directory
        ///
        /// The directory containing the project to generate documentation for.
        /// Must have a .llm-sidekick configuration.
        #[arg(default_value = ".", value_name = "DIR")]
        path: PathBuf,

        /// Output format for documentation
        ///
        /// Specifies the output format for the generated documentation.
        /// Supports 'markdown' (default) and 'json' formats.
        #[arg(
            long = "format",
            short = 'o',
            default_value = "markdown",
            value_name = "FORMAT"
        )]
        format: String,

        /// Focus area for documentation
        ///
        /// Allows generating documentation for a specific section or the entire project.
        /// Options include: 'all' (default), 'project', 'prompts', 'templates', 'history'.
        #[arg(
            long = "focus",
            short = 'a',
            default_value = "all",
            value_name = "FOCUS"
        )]
        focus: String,
    },
}
