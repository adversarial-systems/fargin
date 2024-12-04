use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::cli::HowtoOutputFormat;
use crate::features::FeatureStatus;

/// Project Health and Checks
///
/// # Overview
/// This module provides comprehensive documentation and examples for project health checking
/// and management in the Fargin CLI tool.

/// Implement Display for FeatureStatus
#[allow(clippy::empty_line_after_doc_comments)]
impl fmt::Display for FeatureStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureStatus::Proposed => write!(f, "Proposed"),
            FeatureStatus::InProgress => write!(f, "InProgress"),
            FeatureStatus::Implemented => write!(f, "Implemented"),
            FeatureStatus::Deprecated => write!(f, "Deprecated"),
            FeatureStatus::Blocked => write!(f, "Blocked"),
        }
    }
}

/// ## Example: Basic Project Health Check
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use std::path::Path;
/// use fargin::check::ProjectChecker;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let project_root = Path::new(".");
///     let checker = ProjectChecker::new(project_root);
///
///     // Run comprehensive project checks
///     let health_report = checker.run_all_checks()?;
///     println!("{}", health_report.generate_report());
///     Ok(())
/// }
/// ```
///
/// ## Advanced Usage: Customizing Project Checks
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use std::path::Path;
/// use fargin::check::ProjectChecker;
/// use fargin::features::FeatureStatus;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let project_root = Path::new(".");
///     let checker = ProjectChecker::new(project_root);
///
///     // Detailed feature health check
///     let feature_health = checker.check_feature_health()?;
///     
///     // Analyze feature status distribution
///     let status_distribution = feature_health.status_distribution;
///     println!("Implemented Features: {}",
///         status_distribution.iter()
///             .filter(|(status, _)| **status == FeatureStatus::Implemented)
///             .map(|(_, count)| count)
///             .sum::<usize>()
///     );
///     Ok(())
/// }
/// ```
///
/// ## Example: Dependency Analysis
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use std::path::Path;
/// use fargin::check::ProjectChecker;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let project_root = Path::new(".");
///     let checker = ProjectChecker::new(project_root);
///     
///     // Analyze project dependencies
///     let dependency_report = checker.check_dependencies()?;
///     
///     // Print dependency insights
///     println!("Total Dependencies: {}", dependency_report.total_dependencies);
///     println!("Outdated Dependencies: {:?}", dependency_report.outdated_dependencies);
///     Ok(())
/// }
/// ```
///
/// ## Example: Git Health Check
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use std::path::Path;
/// use fargin::check::ProjectChecker;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let project_root = Path::new(".");
///     let checker = ProjectChecker::new(project_root);
///     
///     // Analyze Git repository health
///     let git_report = checker.check_git_status()?;
///     
///     // Display Git insights
///     println!("Is Git Repository: {}", git_report.is_git_repo);
///     println!("Current Branch: {}", git_report.branch_name.unwrap_or_else(|| "Unknown".to_string()));
///     println!("Uncommitted Changes: {}", git_report.uncommitted_changes);
///     println!("Unpushed Commits: {}", git_report.unpushed_commits);
///     Ok(())
/// }
/// ```
///
/// ## Example: Howto Documentation Generation
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use fargin::howto::HowtoGenerator;
/// use fargin::cli::HowtoOutputFormat;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let generator = HowtoGenerator::new(
///         Some("check".to_string()),
///         "normal".to_string(),
///         HowtoOutputFormat::Terminal,
///         None
///     );
///     
///     let doc = generator.generate()?;
///     println!("{}", doc);
///     Ok(())
/// }
/// ```
///
/// ## Example: Project Progress and Next Steps
#[allow(clippy::empty_line_after_doc_comments)]
/// ```rust
/// use std::path::Path;
/// use fargin::check::ProjectChecker;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let project_root = Path::new(".");
///     let checker = ProjectChecker::new(project_root);
///
///     // Run comprehensive project checks
///     let report = checker.run_all_checks()?;
///
///     // Generate detailed health report
///     println!("{}", report.generate_report());
///
///     // Get AI-powered recommendations for next steps
///     let next_steps = checker.generate_next_steps(&report);
///     println!("Recommended Next Steps:");
///     for (i, step) in next_steps.iter().enumerate() {
///         println!("{}. {}", i + 1, step);
///     }
///
///     Ok(())
/// }
/// ```
/// Howto documentation generator and retriever
pub struct HowtoGenerator {
    topic: Option<String>,
    verbosity: String,
    output_format: HowtoOutputFormat,
    save_path: Option<PathBuf>,
}

impl HowtoGenerator {
    /// Create a new HowtoGenerator
    pub fn new(
        topic: Option<String>,
        verbosity: String,
        output_format: HowtoOutputFormat,
        save_path: Option<PathBuf>,
    ) -> Self {
        Self {
            topic,
            verbosity,
            output_format,
            save_path,
        }
    }

    /// List all available howto topics
    pub fn list_topics() -> Vec<String> {
        vec![
            "check".to_string(),
            "feature-status".to_string(),
            "dependency-management".to_string(),
            "git-health".to_string(),
            "logging".to_string(),
            "cli-usage".to_string(),
        ]
    }

    /// Generate howto documentation
    pub fn generate(&self) -> Result<String, anyhow::Error> {
        // Select documentation based on topic and verbosity
        let doc = match self.topic.as_deref() {
            Some("check") => self.generate_project_checks_doc(),
            Some("feature-status") => self.generate_feature_status_doc(),
            Some("dependency-management") => self.generate_dependency_doc(),
            Some("git-health") => self.generate_git_health_doc(),
            Some("logging") => self.generate_logging_doc(),
            Some("cli-usage") => self.generate_cli_usage_doc(),
            None => self.generate_overview_doc(),
            _ => anyhow::bail!("Unknown howto topic"),
        };

        // Transform documentation based on output format
        let formatted_doc = match self.output_format {
            HowtoOutputFormat::Terminal => doc,
            HowtoOutputFormat::Markdown => self.to_markdown(&doc),
            HowtoOutputFormat::Html => self.to_html(&doc),
        };

        // Save documentation if save path is provided
        if let Some(path) = &self.save_path {
            self.save_documentation(&formatted_doc, path)?;
        }

        Ok(formatted_doc)
    }

    /// Generate project checks documentation
    fn generate_project_checks_doc(&self) -> String {
        let detail = match self.verbosity.as_str() {
            "high" => "Comprehensive project check documentation with advanced details.",
            "low" => "Basic overview of project check.",
            _ => "Detailed project check documentation.",
        };

        format!(
            "# Check Documentation\n\n{}\n\n\
            ## Supported Check\n\
              - Code Formatting\n\
              - Linting\n\
              - Unit Testing\n\n\
            ## Usage Example\n\
            ```bash\n\
            fargin check\n\
            ```\n",
            detail
        )
    }

    /// Generate feature status documentation
    fn generate_feature_status_doc(&self) -> String {
        "# Feature Status Documentation\n\n\
        Track and manage project features effectively.\n\n\
        ## Feature Statuses\n\
        - Proposed\n\
        - In Progress\n\
        - Implemented\n\
        - Blocked"
            .to_string()
    }

    /// Generate dependency management documentation
    fn generate_dependency_doc(&self) -> String {
        "# Dependency Management\n\n\
        Monitor and manage project dependencies.\n\n\
        ## Key Metrics\n\
        - Version compatibility\n\
        - Security checks\n\
        - Update recommendations"
            .to_string()
    }

    /// Generate Git health documentation
    fn generate_git_health_doc(&self) -> String {
        "# Git Repository Health\n\n\
        Track repository development workflow.\n\n\
        ## Monitored Aspects\n\
        . Commit frequency\n\
        . Branch management\n\
        . Merge conflict potential"
            .to_string()
    }

    /// Generate logging documentation
    fn generate_logging_doc(&self) -> String {
        "# Logging and Observability\n\n\
        Configure and use logging effectively.\n\n\
        ## Log Levels\n\
        - `ERROR`: Critical issues\n\
        - `WARN`: Potential problems\n\
        - `INFO`: Important events\n\
        - `DEBUG`: Diagnostic information"
            .to_string()
    }

    /// Generate CLI usage documentation
    fn generate_cli_usage_doc(&self) -> String {
        "# CLI Usage Guide\n\n\
        Comprehensive guide to Fargin CLI commands.\n\n\
        ## Available Commands\n\
        . `check`: Run project health check\n\
        . `config`: Manage project configuration\n\
        . `status`: Display project status"
            .to_string()
    }

    /// Generate overview documentation
    fn generate_overview_doc(&self) -> String {
        format!(
            "# Fargin CLI Documentation\n\n\
            Comprehensive guide to using the Fargin CLI tool.\n\n\
            ## Available Topics\n\
            {}\n\
            Use `fargin howto <topic>` for detailed information.\n",
            Self::list_topics().join("\n")
        )
    }

    /// Convert documentation to Markdown
    fn to_markdown(&self, doc: &str) -> String {
        doc.to_string() // In a real implementation, add Markdown-specific formatting
    }

    /// Convert documentation to HTML
    fn to_html(&self, doc: &str) -> String {
        format!("<html><body><pre>{}</pre></body></html>", doc)
    }

    /// Save documentation to a file
    fn save_documentation(&self, doc: &str, path: &PathBuf) -> Result<(), anyhow::Error> {
        let mut file = File::create(path).context("Failed to create documentation file")?;

        file.write_all(doc.as_bytes())
            .context("Failed to write documentation to file")?;

        Ok(())
    }
}

// Re-export key types for documentation purposes
pub use crate::check::{FeatureHealthReport, ProjectChecker, ProjectHealthReport};
