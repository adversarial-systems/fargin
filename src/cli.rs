use crate::features;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Fargin - LLM-driven project development assistant
#[derive(Parser)]
#[command(
    author = "LLM Sidekick Contributors",
    version,
    about = "Streamline LLM-driven project development",
    long_about = "A tool to help manage, develop, and optimize projects using large language models"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Primary commands for project development workflow
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init {
        /// Subcommand for initialization operations
        #[command(subcommand)]
        operation: InitOperation,
    },

    /// Manage and develop project features
    Feature {
        /// Subcommand for feature operations
        #[command(subcommand)]
        operation: FeatureOperation,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Design and architect project components
    Design {
        /// Subcommand for design operations
        #[command(subcommand)]
        operation: DesignOperation,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Check project health, validate configurations
    Check {
        /// Subcommand for various checks
        #[command(subcommand)]
        operation: CheckOperation,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Reset project state or configurations
    Reset {
        /// Reset scope
        #[arg(default_value = "soft")]
        scope: String,

        /// Force reset without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Provide guidance and best practices
    Howto {
        /// Topic or area to get guidance on
        topic: Option<String>,

        /// Verbosity of guidance
        #[arg(short, long, default_value = "normal")]
        verbosity: String,

        /// Output format for the howto guide
        #[arg(long, value_enum, default_value_t = HowtoOutputFormat::Terminal)]
        output: HowtoOutputFormat,

        /// Path to save the howto documentation
        #[arg(long)]
        save_path: Option<PathBuf>,

        /// List all available howto topics
        #[arg(long, short)]
        list_topics: bool,
    },
}

/// Project initialization options
#[derive(Subcommand)]
pub enum InitOperation {
    /// Create a new Rust project using Cargo
    Rust {
        /// Project name
        name: String,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,

        /// Cargo binary to use (default: cargo)
        #[arg(long, default_value = "cargo")]
        cargo_bin: String,

        /// Cargo template or preset
        #[arg(short, long)]
        template: Option<String>,

        /// Initialize with Fargin management structure
        #[arg(short, long, default_value = "true")]
        with_fargin: bool,

        /// Perform a dry run without creating actual files
        #[arg(long)]
        dry_run: bool,
    },

    /// Create a new project from a template
    Template {
        /// Template name or source
        template: String,

        /// Project name
        name: String,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,

        /// Initialize with Fargin management structure
        #[arg(short, long, default_value = "true")]
        with_fargin: bool,

        /// Perform a dry run without creating actual files
        #[arg(long)]
        dry_run: bool,
    },

    /// Create a minimal Fargin project structure
    Minimal {
        /// Project name
        name: String,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,

        /// Project type (rust, python, js, etc.)
        #[arg(short = 't', long, default_value = "rust")]
        project_type: String,

        /// Include Fargin management structure
        #[arg(short, long)]
        with_fargin: bool,

        /// Perform a dry run without creating actual files
        #[arg(long)]
        dry_run: bool,
    },
}

/// Feature management operations
#[derive(Subcommand)]
pub enum FeatureOperation {
    /// Add a new feature to the project
    Add {
        /// Feature name
        name: String,

        /// Optional detailed description
        #[arg(short, long)]
        description: Option<String>,

        /// Tags or categories for the feature
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Priority of the feature
        #[arg(short, long, value_enum)]
        priority: Option<features::Priority>,

        /// Assign feature to a specific person/team
        #[arg(short, long)]
        assigned_to: Option<String>,
    },

    /// List existing features
    List {
        /// Filter features by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter features by status
        #[arg(short, long, value_enum)]
        status: Option<features::FeatureStatus>,

        /// Filter features by priority
        #[arg(short, long, value_enum)]
        priority: Option<features::Priority>,
    },

    /// Show details of a specific feature
    Show {
        /// Feature ID
        id: String,
    },

    /// Update an existing feature
    Update {
        /// Feature ID
        id: String,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// Update feature status
        #[arg(short, long, value_enum)]
        status: Option<features::FeatureStatus>,

        /// Update feature priority
        #[arg(short, long, value_enum)]
        priority: Option<features::Priority>,

        /// Update feature tags
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Reassign feature
        #[arg(short, long)]
        assigned_to: Option<String>,
    },

    /// Remove a feature from the project
    Remove {
        /// Feature ID
        id: String,
    },

    /// Generate intelligent suggestions for a feature
    Suggest {
        /// Feature ID to generate suggestions for
        id: String,

        /// Type of suggestion to generate
        #[arg(short, long, value_enum)]
        suggestion_type: Option<features::SuggestionType>,

        /// Verbosity of suggestions
        #[arg(short, long, default_value = "normal")]
        verbosity: String,

        /// Output format for suggestions
        #[arg(long, value_enum, default_value_t = HowtoOutputFormat::Terminal)]
        output: HowtoOutputFormat,

        /// Save suggestions to a file
        #[arg(long)]
        save_path: Option<PathBuf>,
    },
}

/// Design operations for project architecture
#[derive(Subcommand)]
pub enum DesignOperation {
    /// Create a new architectural design
    Create {
        /// Design name
        name: String,

        /// Optional design description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List existing architectural designs
    List,

    /// Show details of a specific design
    Show {
        /// Design identifier
        id: String,
    },
}

/// Check operations for project health and consistency
#[derive(Debug, Subcommand)]
pub enum CheckOperation {
    /// Run comprehensive project health checks
    ///
    /// Preferred option when inside the LLM agent chat for quick,
    /// non-blocking project validation and health assessment.
    Run {
        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Continuously run project checks in a loop
    Loop {
        /// Project path (default: current directory)
        #[arg(long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,

        /// Interval between checks (in seconds)
        #[arg(short = 'i', long, default_value = "60")]
        interval: u64,

        /// Stop after a specific number of iterations (0 = infinite)
        #[arg(short = 'n', long, default_value = "0")]
        iterations: u64,
    },

    /// Verify code formatting
    Fmt {
        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Run linting checks
    Lint {
        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Run unit tests
    Test {
        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },

    /// Check Git repository status
    Git,

    /// Evaluate and generate a comprehensive project progress summary
    Progress {
        /// Verbosity of the progress summary
        #[arg(short, long, default_value = "normal")]
        verbosity: String,

        /// Output format for the progress summary
        #[arg(long, value_enum, default_value_t = HowtoOutputFormat::Terminal)]
        output: HowtoOutputFormat,

        /// Project path (default: current directory)
        #[arg(short, long, default_value = ".", value_name = "PROJECT_PATH")]
        path: PathBuf,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum HowtoOutputFormat {
    Terminal,
    Markdown,
    Html,
}
