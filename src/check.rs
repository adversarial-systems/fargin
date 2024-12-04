use crate::features::FeatureStatus;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Comprehensive project health and consistency checker
pub struct ProjectChecker {
    project_root: PathBuf,
}

impl ProjectChecker {
    /// Create a new project checker
    pub fn new(project_root: &Path) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
        }
    }

    /// Run all project checks
    pub fn run_all_checks(&self) -> Result<ProjectHealthReport> {
        Ok(ProjectHealthReport {
            feature_health: self.check_feature_health()?,
            file_structure: self.check_file_structure()?,
            dependency_health: self.check_dependencies()?,
            git_health: self.check_git_status()?,
        })
    }

    /// Run comprehensive project checks similar to ./check.sh
    pub fn run_project_checks(&self) -> Result<()> {
        println!("🔍 Starting comprehensive project checks");
        info!("Starting comprehensive project checks");
        debug!("Project path: {}", self.project_root.display());

        // Helper function to run command and stream output
        fn run_command_with_streaming(
            cmd: &mut std::process::Command,
            stage: String,
        ) -> Result<()> {
            use log::{debug, error, info, warn};
            use std::io::{BufRead, BufReader};
            use std::process::Stdio;
            use std::sync::mpsc;

            info!("Running {}...", stage);
            debug!("Executing command: {:?}", cmd);
            println!("\n🚀 {}", stage);

            let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

            // Create channels for stdout and stderr
            let (stdout_tx, stdout_rx) = mpsc::channel();
            let (stderr_tx, stderr_rx) = mpsc::channel();

            // Stream stdout
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stdout_stage = stage.clone();
            std::thread::spawn(move || {
                let stdout_reader = BufReader::new(stdout);
                for line in stdout_reader.lines().map_while(Result::ok) {
                    let _ = stdout_tx.send(line);
                }
            });

            // Stream stderr
            let stderr = child.stderr.take().expect("Failed to capture stderr");
            std::thread::spawn(move || {
                let stderr_reader = BufReader::new(stderr);
                for line in stderr_reader.lines().map_while(Result::ok) {
                    let _ = stderr_tx.send(line);
                }
            });

            // Receive and print stdout
            std::thread::spawn(move || {
                while let Ok(line) = stdout_rx.recv() {
                    println!("{}", line);
                    debug!("{} stdout: {}", stdout_stage, line);
                }
            });

            // Receive and print stderr
            let stage_clone = stage.clone();
            std::thread::spawn(move || {
                while let Ok(line) = stderr_rx.recv() {
                    eprintln!("{}", line);
                    warn!("{} stderr: {}", stage_clone, line);
                }
            });

            // Wait for command to complete
            let status = child.wait()?;

            if !status.success() {
                error!("{} failed", stage);
                println!("❌ {} failed", stage);
                return Err(anyhow::anyhow!("{} failed", stage));
            }

            info!("{} passed", stage);
            println!("✅ {} passed", stage);
            Ok(())
        }

        // Ensure we're in the correct directory
        std::env::set_current_dir(&self.project_root)?;
        info!(
            "Changed working directory to: {}",
            self.project_root.display()
        );

        // Run cargo fmt
        let mut fmt_cmd = std::process::Command::new("cargo");
        fmt_cmd.arg("fmt");
        run_command_with_streaming(&mut fmt_cmd, "Cargo Formatting Check".to_string())?;

        // Run cargo clippy
        let mut clippy_cmd = std::process::Command::new("cargo");
        clippy_cmd.args(["clippy", "--", "-D", "warnings"]);
        run_command_with_streaming(&mut clippy_cmd, "Cargo Clippy Linting".to_string())?;

        // Run tests
        let mut test_cmd = std::process::Command::new("cargo");
        test_cmd.arg("test");
        run_command_with_streaming(&mut test_cmd, "Cargo Test Suite".to_string())?;

        info!("All project checks completed successfully");
        println!("🎉 All project checks completed successfully!");
        Ok(())
    }

    /// Check the health and status of project features
    pub fn check_feature_health(&self) -> Result<FeatureHealthReport> {
        let features_dir = self.project_root.join(".fargin/features");

        if !features_dir.exists() {
            return Ok(FeatureHealthReport {
                total_features: 0,
                status_distribution: HashMap::new(),
                stale_features: Vec::new(),
            });
        }

        let mut total_features = 0;
        let mut status_distribution = HashMap::new();
        let mut stale_features = Vec::new();

        for entry in fs::read_dir(features_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                let content = fs::read_to_string(entry.path())?;

                // Basic parsing of feature status
                let status = if content.contains("Status: Implemented") {
                    FeatureStatus::Implemented
                } else if content.contains("Status: InProgress") {
                    FeatureStatus::InProgress
                } else if content.contains("Status: Blocked") {
                    FeatureStatus::Blocked
                } else {
                    FeatureStatus::Proposed
                };

                // Count status distribution
                *status_distribution.entry(status).or_insert(0) += 1;
                total_features += 1;

                // Check for stale features (no updates in 30 days)
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let days_since_update = SystemTime::now()
                            .duration_since(modified)
                            .map(|d| d.as_secs() / (24 * 3600))
                            .unwrap_or(0);

                        if days_since_update > 30 {
                            stale_features.push(entry.file_name().to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }

        Ok(FeatureHealthReport {
            total_features,
            status_distribution,
            stale_features,
        })
    }

    /// Check project file structure and recommended directories
    pub fn check_file_structure(&self) -> Result<FileStructureReport> {
        let recommended_dirs = vec![
            ".fargin",
            ".fargin/features",
            ".fargin/docs",
            ".fargin/templates",
            ".fargin/artifacts",
            "src",
            "tests",
            "docs",
        ];

        let mut missing_dirs = Vec::new();
        let mut existing_dirs = Vec::new();

        for dir in recommended_dirs {
            let full_path = self.project_root.join(dir);
            if full_path.exists() {
                existing_dirs.push(dir.to_string());
            } else {
                missing_dirs.push(dir.to_string());
            }
        }

        Ok(FileStructureReport {
            existing_dirs,
            missing_dirs,
        })
    }

    /// Check project dependencies and their health
    pub fn check_dependencies(&self) -> Result<DependencyHealthReport> {
        let cargo_toml_path = self.project_root.join("Cargo.toml");

        if !cargo_toml_path.exists() {
            return Ok(DependencyHealthReport {
                total_dependencies: 0,
                outdated_dependencies: Vec::new(),
            });
        }

        // This is a placeholder. In a real implementation, you'd parse Cargo.toml
        // and potentially use `cargo outdated` to check for updates
        Ok(DependencyHealthReport {
            total_dependencies: 0,
            outdated_dependencies: Vec::new(),
        })
    }

    /// Check Git repository status
    pub fn check_git_status(&self) -> Result<GitHealthReport> {
        let git_dir = self.project_root.join(".git");

        if !git_dir.exists() {
            return Ok(GitHealthReport {
                is_git_repo: false,
                uncommitted_changes: false,
                unpushed_commits: false,
                branch_name: None,
            });
        }

        // This is a placeholder. In a real implementation, you'd use git commands
        Ok(GitHealthReport {
            is_git_repo: true,
            uncommitted_changes: false,
            unpushed_commits: false,
            branch_name: Some("main".to_string()),
        })
    }

    /// Generate a comprehensive project progress summary
    pub fn generate_progress_summary(&self, verbosity: &str) -> Result<String> {
        let health_report = self.run_all_checks()?;

        // Determine verbosity level
        let summary = match verbosity {
            "high" => self.generate_detailed_progress_summary(&health_report),
            "low" => self.generate_brief_progress_summary(&health_report),
            _ => self.generate_standard_progress_summary(&health_report),
        };

        Ok(summary)
    }

    fn generate_brief_progress_summary(&self, report: &ProjectHealthReport) -> String {
        format!(
            "Project Progress Summary:\n\
            - Features: {} total ({} implemented)\n\
            - Dependencies: {} total\n\
            - Git Status: {}\n",
            report.feature_health.total_features,
            report
                .feature_health
                .status_distribution
                .get(&FeatureStatus::Implemented)
                .cloned()
                .unwrap_or(0),
            report.dependency_health.total_dependencies,
            if report.git_health.is_git_repo {
                "✅ Healthy"
            } else {
                "❌ Not a Git Repo"
            }
        )
    }

    fn generate_standard_progress_summary(&self, report: &ProjectHealthReport) -> String {
        let feature_summary = report.feature_health.status_distribution.iter().fold(
            String::new(),
            |mut acc, (status, count)| {
                acc.push_str(&format!("  - {}: {}\n", status, count));
                acc
            },
        );

        format!(
            "🚀 Project Progress Summary 🚀\n\n\
            Feature Health:\n\
            Total Features: {}\n\
            Feature Status Distribution:\n{}\
            Stale Features: {}\n\n\
            Dependency Health:\n\
            Total Dependencies: {}\n\
            Outdated Dependencies: {}\n\n\
            Git Repository Health:\n\
            Is Git Repository: {}\n\
            Current Branch: {}\n\
            Uncommitted Changes: {}\n\
            Unpushed Commits: {}\n",
            report.feature_health.total_features,
            feature_summary,
            report.feature_health.stale_features.join(", "),
            report.dependency_health.total_dependencies,
            report.dependency_health.outdated_dependencies.len(),
            report.git_health.is_git_repo,
            report
                .git_health
                .branch_name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            report.git_health.uncommitted_changes,
            report.git_health.unpushed_commits
        )
    }

    fn generate_detailed_progress_summary(&self, report: &ProjectHealthReport) -> String {
        let feature_summary = report.feature_health.status_distribution.iter().fold(
            String::new(),
            |mut acc, (status, count)| {
                acc.push_str(&format!("  - {}: {}\n", status, count));
                acc
            },
        );

        let stale_features_details =
            report
                .feature_health
                .stale_features
                .iter()
                .fold(String::new(), |mut acc, feature| {
                    acc.push_str(&format!("  - {}\n", feature));
                    acc
                });

        let outdated_dependencies_details = report
            .dependency_health
            .outdated_dependencies
            .iter()
            .fold(String::new(), |mut acc, dep| {
                acc.push_str(&format!("  - {}\n", dep));
                acc
            });

        format!(
            "🌟 Comprehensive Project Progress Summary 🌟\n\n\
            🔍 Feature Health:\n\
            Total Features: {}\n\
            Feature Status Distribution:\n{}\
            Stale Features (>30 days):\n{}\n\
            Potential Actions:\n\
              - Review and update stale features\n\
              - Close or reactivate inactive features\n\n\
            📦 Dependency Health:\n\
            Total Dependencies: {}\n\
            Outdated Dependencies:\n{}\
            Potential Actions:\n\
              - Update dependencies to latest versions\n\
              - Review security and compatibility\n\n\
            🌳 Git Repository Health:\n\
            Is Git Repository: {}\n\
            Current Branch: {}\n\
            Uncommitted Changes: {}\n\
            Unpushed Commits: {}\n\
            Potential Actions:\n\
              - Commit or stash uncommitted changes\n\
              - Push local commits to remote\n\
              - Consider creating feature branches\n\n\
            💡 Recommendations:\n\
              1. Prioritize features with 'Blocked' or 'InProgress' status\n\
              2. Address stale features and outdated dependencies\n\
              3. Maintain consistent Git workflow\n",
            report.feature_health.total_features,
            feature_summary,
            stale_features_details,
            report.dependency_health.total_dependencies,
            outdated_dependencies_details,
            report.git_health.is_git_repo,
            report
                .git_health
                .branch_name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            report.git_health.uncommitted_changes,
            report.git_health.unpushed_commits
        )
    }
}

/// Comprehensive project health report
#[derive(Default)]
pub struct ProjectHealthReport {
    pub feature_health: FeatureHealthReport,
    pub file_structure: FileStructureReport,
    pub dependency_health: DependencyHealthReport,
    pub git_health: GitHealthReport,
}

/// Feature health metrics
#[derive(Default)]
pub struct FeatureHealthReport {
    pub total_features: usize,
    pub status_distribution: HashMap<FeatureStatus, usize>,
    pub stale_features: Vec<String>,
}

/// File structure report
#[derive(Default)]
pub struct FileStructureReport {
    pub existing_dirs: Vec<String>,
    pub missing_dirs: Vec<String>,
}

/// Dependency health report
#[derive(Default)]
pub struct DependencyHealthReport {
    pub total_dependencies: usize,
    pub outdated_dependencies: Vec<String>,
}

/// Git repository health report
#[derive(Default)]
pub struct GitHealthReport {
    pub is_git_repo: bool,
    pub uncommitted_changes: bool,
    pub unpushed_commits: bool,
    pub branch_name: Option<String>,
}

/// Detailed project health report formatter
impl ProjectHealthReport {
    /// Generate a human-readable health report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        // Feature Health
        report.push_str("🔍 Feature Health:\n");
        report.push_str(&format!(
            "   Total Features: {}\n",
            self.feature_health.total_features
        ));
        report.push_str("   Status Distribution:\n");
        for (status, count) in &self.feature_health.status_distribution {
            report.push_str(&format!("     - {:?}: {}\n", status, count));
        }
        if !self.feature_health.stale_features.is_empty() {
            report.push_str("   Stale Features:\n");
            for feature in &self.feature_health.stale_features {
                report.push_str(&format!("     - {}\n", feature));
            }
        }

        // File Structure
        report.push_str("\n📂 Project Structure:\n");
        report.push_str("   Existing Directories:\n");
        for dir in &self.file_structure.existing_dirs {
            report.push_str(&format!("     - {}\n", dir));
        }
        if !self.file_structure.missing_dirs.is_empty() {
            report.push_str("   Missing Recommended Directories:\n");
            for dir in &self.file_structure.missing_dirs {
                report.push_str(&format!("     - {}\n", dir));
            }
        }

        // Dependency Health
        report.push_str("\n📦 Dependency Health:\n");
        report.push_str(&format!(
            "   Total Dependencies: {}\n",
            self.dependency_health.total_dependencies
        ));
        if !self.dependency_health.outdated_dependencies.is_empty() {
            report.push_str("   Outdated Dependencies:\n");
            for dep in &self.dependency_health.outdated_dependencies {
                report.push_str(&format!("     - {}\n", dep));
            }
        }

        // Git Health
        report.push_str("\n🌿 Git Repository Health:\n");
        report.push_str(&format!(
            "   Is Git Repository: {}\n",
            self.git_health.is_git_repo
        ));
        report.push_str(&format!(
            "   Current Branch: {}\n",
            self.git_health.branch_name.as_deref().unwrap_or("Unknown")
        ));
        report.push_str(&format!(
            "   Uncommitted Changes: {}\n",
            self.git_health.uncommitted_changes
        ));
        report.push_str(&format!(
            "   Unpushed Commits: {}\n",
            self.git_health.unpushed_commits
        ));

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_project_checker_initialization() {
        let temp_dir = tempdir().unwrap();
        let checker = ProjectChecker::new(temp_dir.path());

        // Basic initialization test
        assert_eq!(checker.project_root, temp_dir.path());
    }

    #[test]
    fn test_run_all_checks_on_empty_project() {
        let temp_dir = tempdir().unwrap();
        let checker = ProjectChecker::new(temp_dir.path());

        let report = checker.run_all_checks().unwrap();

        // Verify basic report generation
        assert_eq!(report.feature_health.total_features, 0);
        assert!(report
            .file_structure
            .missing_dirs
            .contains(&".fargin".to_string()));
    }
}
