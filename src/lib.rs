pub mod check;
pub mod cli;
pub mod config;
pub mod features;
pub mod howto;

use crate::check::ProjectChecker;
use crate::cli::{
    CheckOperation, Cli, Commands, DesignOperation, FeatureOperation, HowtoOutputFormat,
    InitOperation,
};
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "{}: {}", record.level(), record.args())
        })
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Init { operation } => match operation {
            InitOperation::Rust {
                name,
                path,
                cargo_bin,
                template,
                with_fargin,
                dry_run,
            } => config::init_rust_project(name, path, cargo_bin, template, with_fargin, dry_run),
            InitOperation::Template {
                template,
                name,
                path,
                with_fargin,
                dry_run,
            } => config::init_template_project(template, name, path, with_fargin, dry_run),
            InitOperation::Minimal {
                name,
                path,
                project_type,
                with_fargin,
                dry_run,
            } => config::init_minimal_project(name, path, project_type, with_fargin, dry_run),
        },
        Commands::Feature { operation, path } => {
            // Create feature manager for the project
            let mut feature_manager = features::FeatureManager::new(&path)?;

            match operation {
                FeatureOperation::Add {
                    name,
                    description,
                    tags,
                    priority,
                    assigned_to,
                } => {
                    let feature_id = feature_manager.add_feature(
                        name,
                        description,
                        tags,
                        priority,
                        assigned_to,
                    )?;
                    println!("Feature added with ID: {}", feature_id);
                    Ok(())
                }
                FeatureOperation::List {
                    tag,
                    status,
                    priority,
                } => {
                    let features = feature_manager.list_features(tag.as_deref(), status, priority);

                    if features.is_empty() {
                        println!("No features found.");
                    } else {
                        println!("Features:");
                        for feature in features {
                            println!(
                                "ID: {}, Name: {}, Status: {:?}, Priority: {:?}",
                                feature.id, feature.name, feature.status, feature.priority
                            );
                        }
                    }
                    Ok(())
                }
                FeatureOperation::Show { id } => match feature_manager.get_feature(&id) {
                    Some(feature) => {
                        println!("Feature Details:");
                        println!("ID: {}", feature.id);
                        println!("Name: {}", feature.name);
                        println!(
                            "Description: {}",
                            feature.description.as_deref().unwrap_or("No description")
                        );
                        println!("Status: {:?}", feature.status);
                        println!("Priority: {:?}", feature.priority);
                        println!("Tags: {:?}", feature.tags);
                        println!(
                            "Assigned To: {}",
                            feature.assigned_to.as_deref().unwrap_or("Unassigned")
                        );
                        Ok(())
                    }
                    None => Err(anyhow::anyhow!("Feature not found")),
                },
                FeatureOperation::Update {
                    id,
                    description,
                    status,
                    tags,
                    priority,
                    assigned_to,
                } => {
                    feature_manager.update_feature(
                        &id,
                        features::FeatureUpdateRequest {
                            description,
                            status,
                            tags,
                            priority,
                            assigned_to,
                            ..Default::default()
                        },
                    )?;
                    println!("Feature {} updated successfully", id);
                    Ok(())
                }
                FeatureOperation::Remove { id } => {
                    feature_manager.delete_feature(&id)?;
                    println!("Feature {} deleted successfully", id);
                    Ok(())
                }
            }
        }
        Commands::Design { operation, path: _ } => {
            // Placeholder for design operations
            match operation {
                DesignOperation::Create { name, description } => {
                    println!(
                        "Creating design: {} with description: {:?}",
                        name, description
                    );
                    Ok(())
                }
                DesignOperation::List => {
                    println!("Listing existing designs");
                    Ok(())
                }
                DesignOperation::Show { id } => {
                    println!("Showing design details for: {}", id);
                    Ok(())
                }
            }
        }
        Commands::Check { operation, path } => {
            let project_path = path.clone();

            let project_checker = ProjectChecker::new(project_path.as_path());

            match operation {
                CheckOperation::Run { .. } => {
                    println!("🔍 Running comprehensive project checks...");
                    match project_checker.run_project_checks() {
                        Ok(_) => {
                            println!("✅ All project checks completed successfully!");
                            Ok(())
                        }
                        Err(e) => {
                            eprintln!("❌ Project checks failed: {}", e);
                            Err(e)
                        }
                    }
                }
                CheckOperation::Loop {
                    path: _,
                    interval,
                    iterations,
                } => {
                    use std::thread;
                    use std::time::Duration;

                    println!("🔁 Starting continuous project checks");
                    println!("   Interval: {} seconds", interval);
                    println!("   Max Iterations: {}", iterations);

                    let mut iteration_count = 0;
                    loop {
                        iteration_count += 1;
                        println!("\n🕒 Check Iteration {}", iteration_count);

                        match project_checker.run_project_checks() {
                            Ok(_) => {
                                println!("✅ Project checks completed successfully");
                            }
                            Err(e) => {
                                eprintln!("❌ Project checks failed: {}", e);
                            }
                        }

                        // Check iteration limit
                        if iterations > 0 && iteration_count >= iterations {
                            println!("🏁 Reached maximum iterations. Stopping.");
                            break;
                        }

                        // Wait before next iteration
                        thread::sleep(Duration::from_secs(interval));
                    }

                    Ok(())
                }
                CheckOperation::Fmt { path } => {
                    println!("🧹 Running code formatting check...");
                    let mut fmt_cmd = std::process::Command::new("cargo");
                    fmt_cmd.arg("fmt").current_dir(path);

                    match fmt_cmd.output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✅ Code formatting check passed");
                                Ok(())
                            } else {
                                eprintln!("❌ Code formatting check failed");
                                Err(anyhow::anyhow!("Formatting check failed"))
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error running formatting check: {}", e);
                            Err(anyhow::anyhow!(e))
                        }
                    }
                }
                CheckOperation::Lint { path } => {
                    println!("🕵️ Running linting checks...");
                    let mut clippy_cmd = std::process::Command::new("cargo");
                    clippy_cmd
                        .args(["clippy", "--", "-D", "warnings"])
                        .current_dir(path);

                    match clippy_cmd.output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✅ Linting checks passed");
                                Ok(())
                            } else {
                                eprintln!("❌ Linting checks failed");
                                Err(anyhow::anyhow!("Linting check failed"))
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error running linting checks: {}", e);
                            Err(anyhow::anyhow!(e))
                        }
                    }
                }
                CheckOperation::Test { path } => {
                    println!("🧪 Running unit tests...");
                    let mut test_cmd = std::process::Command::new("cargo");
                    test_cmd.arg("test").current_dir(path);

                    match test_cmd.output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✅ All unit tests passed");
                                Ok(())
                            } else {
                                eprintln!("❌ Some unit tests failed");
                                Err(anyhow::anyhow!("Unit tests failed"))
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error running unit tests: {}", e);
                            Err(anyhow::anyhow!(e))
                        }
                    }
                }
                CheckOperation::Git => {
                    let git_report = project_checker.check_git_status()?;
                    println!("🌿 Git Repository Health Report:");
                    println!("Is Git Repository: {}", git_report.is_git_repo);
                    println!(
                        "Current Branch: {}",
                        git_report
                            .branch_name
                            .unwrap_or_else(|| "Unknown".to_string())
                    );
                    println!("Uncommitted Changes: {}", git_report.uncommitted_changes);
                    println!("Unpushed Commits: {}", git_report.unpushed_commits);
                    Ok(())
                }
                CheckOperation::Progress {
                    verbosity,
                    output,
                    path: _,
                } => {
                    let project_checker = ProjectChecker::new(project_path.as_path());
                    let progress_summary = project_checker.generate_progress_summary(&verbosity)?;

                    // Apply output formatting
                    let formatted_summary = match output {
                        HowtoOutputFormat::Terminal => progress_summary,
                        HowtoOutputFormat::Markdown => {
                            format!("```markdown\n{}\n```", progress_summary)
                        }
                        HowtoOutputFormat::Html => {
                            format!("<pre>{}</pre>", progress_summary)
                        }
                    };

                    println!("{}", formatted_summary);
                    Ok(())
                }
            }
        }
        Commands::Reset { scope, force } => {
            println!("Resetting project with scope: {} (Force: {})", scope, force);
            Ok(())
        }
        Commands::Howto {
            topic,
            verbosity,
            output,
            save_path,
            list_topics,
        } => {
            if list_topics {
                println!("Available Howto Topics:");
                for topic in howto::HowtoGenerator::list_topics() {
                    println!("  - {}", topic);
                }
                return Ok(());
            }

            let generator = howto::HowtoGenerator::new(topic, verbosity, output, save_path);

            let doc = generator.generate()?;
            println!("{}", doc);

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn test_feature_management() {
        // TODO: Implement comprehensive feature management tests
    }
}
