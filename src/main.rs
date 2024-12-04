use anyhow::Result;
use clap::Parser;
use fargin::cli::{Cli, Commands, FactTypeArg};
use fargin::config::init_project;
use fargin::docs::generate_llm_documentation;
use fargin::facts::{Fact, FactMetadata, FactType};
use fargin::progress::show_progress;
use fargin::reset::reset_project;
use fargin::suggest;
use fargin::validation::{validate_project, ValidationStatus};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => init_project(path),
        Commands::Validate { path } => {
            let report = validate_project(path)?;
            println!("Validation Report:");
            println!("=================");
            for check in report.checks {
                println!(
                    "{}: {}",
                    check.name,
                    match check.status {
                        ValidationStatus::Pass => "✅ Pass",
                        ValidationStatus::Warning => "⚠️  Warning",
                        ValidationStatus::Error => "❌ Error",
                    }
                );
                if let Some(msg) = check.message {
                    println!("  {}", msg);
                }
            }
            Ok(())
        }
        Commands::Progress { path } => {
            let report = show_progress(path)?;
            println!("Progress Report:");
            println!("===============");
            println!("Project: {}", report.project_name);
            println!("Total Markers: {}", report.total_markers);
            println!("Completed: {}", report.completed_markers);
            println!("Last Updated: {}", report.last_updated);
            println!("\nMarkers:");
            for marker in report.markers {
                println!(
                    "- {} ({})",
                    marker.name,
                    if marker.completed {
                        format!(
                            "✅ Completed at {}",
                            marker.completed_at.unwrap_or_default()
                        )
                    } else {
                        "⏳ In Progress".to_string()
                    }
                );
                if !marker.description.is_empty() {
                    println!("  Description: {}", marker.description);
                }
            }
            Ok(())
        }
        Commands::Reset { path, force } => reset_project(path, force),
        Commands::Add {
            fact_type,
            content,
            path,
            description,
            tags,
            version,
            references,
        } => {
            let fact_type = match fact_type {
                FactTypeArg::Prompt => FactType::Prompt,
                FactTypeArg::History => FactType::History,
                FactTypeArg::Template => FactType::Template,
            };

            let fact = Fact::new(
                fact_type,
                content,
                FactMetadata {
                    description,
                    tags: tags.unwrap_or_default(),
                    version,
                    references: references.unwrap_or_default(),
                },
            );

            fact.save(&path)?;
            println!("Added new fact with ID: {}", fact.id);
            Ok(())
        }
        Commands::List { fact_type, path } => {
            let fact_type = match fact_type {
                FactTypeArg::Prompt => FactType::Prompt,
                FactTypeArg::History => FactType::History,
                FactTypeArg::Template => FactType::Template,
            };

            let facts = Fact::list(fact_type, &path)?;
            if facts.is_empty() {
                println!("No facts found.");
                return Ok(());
            }

            println!("Found {} facts:", facts.len());
            for fact in facts {
                println!("ID: {}", fact.id);
                if let Some(desc) = fact.metadata.description {
                    println!("Description: {}", desc);
                }
                println!("Created: {}", fact.created_at);
                if !fact.metadata.tags.is_empty() {
                    println!("Tags: {}", fact.metadata.tags.join(", "));
                }
                println!("---");
            }
            Ok(())
        }
        Commands::Show {
            fact_id,
            fact_type,
            path,
        } => {
            let fact_type = match fact_type {
                FactTypeArg::Prompt => FactType::Prompt,
                FactTypeArg::History => FactType::History,
                FactTypeArg::Template => FactType::Template,
            };

            let fact = Fact::load(fact_id.as_str(), fact_type, &path)?;
            println!("ID: {}", fact.id);
            println!("Type: {:?}", fact.fact_type);
            println!("Content:\n{}", fact.content);
            if let Some(desc) = fact.metadata.description {
                println!("\nDescription: {}", desc);
            }
            println!("Created: {}", fact.created_at);
            println!("Updated: {}", fact.updated_at);
            if !fact.metadata.tags.is_empty() {
                println!("Tags: {}", fact.metadata.tags.join(", "));
            }
            if let Some(version) = fact.metadata.version {
                println!("Version: {}", version);
            }
            if !fact.metadata.references.is_empty() {
                println!("References:");
                for reference in fact.metadata.references {
                    println!("- {}", reference);
                }
            }
            Ok(())
        }
        Commands::Update {
            fact_id,
            fact_type,
            content,
            path,
            description,
            tags,
            version,
            references,
        } => {
            let fact_type = match fact_type {
                FactTypeArg::Prompt => FactType::Prompt,
                FactTypeArg::History => FactType::History,
                FactTypeArg::Template => FactType::Template,
            };

            let mut fact = Fact::load(fact_id.as_str(), fact_type, &path)?;

            // Update content if provided
            if let Some(new_content) = content {
                fact.content = new_content;
            }

            // Update metadata fields if provided
            if description.is_some() || tags.is_some() || version.is_some() || references.is_some()
            {
                fact.metadata = FactMetadata {
                    description: description.or(fact.metadata.description),
                    tags: tags.unwrap_or(fact.metadata.tags),
                    version: version.or(fact.metadata.version),
                    references: references.unwrap_or(fact.metadata.references),
                };
            }

            fact.save(&path)?;
            println!("Updated fact with ID: {}", fact.id);
            Ok(())
        }
        Commands::Search {
            query,
            fact_type,
            path,
        } => {
            let fact_type = fact_type.map(|ft| match ft {
                FactTypeArg::Prompt => FactType::Prompt,
                FactTypeArg::History => FactType::History,
                FactTypeArg::Template => FactType::Template,
            });

            let facts = Fact::list(fact_type.unwrap_or(FactType::Prompt), &path)?
                .into_iter()
                .filter(|f| {
                    f.content.to_lowercase().contains(&query.to_lowercase())
                        || f.metadata
                            .description
                            .as_ref()
                            .is_some_and(|d| d.to_lowercase().contains(&query.to_lowercase()))
                        || f.metadata
                            .tags
                            .iter()
                            .any(|t| t.to_lowercase().contains(&query.to_lowercase()))
                })
                .collect::<Vec<_>>();

            if facts.is_empty() {
                println!("No matching facts found.");
                return Ok(());
            }

            for fact in facts {
                println!("ID: {}", fact.id);
                println!("Type: {:?}", fact.fact_type);
                if let Some(desc) = fact.metadata.description {
                    println!("Description: {}", desc);
                }
                println!("Created: {}", fact.created_at);
                if !fact.metadata.tags.is_empty() {
                    println!("Tags: {}", fact.metadata.tags.join(", "));
                }
                println!("---");
            }
            Ok(())
        }
        Commands::Suggest {
            path,
            suggestion_type,
            verbosity,
        } => {
            let suggestions = suggest::generate_suggestions(&path, &suggestion_type, &verbosity)?;

            // Serialize based on verbosity
            let output = match suggestion_type.as_str() {
                "json" => suggest::serialize_suggestions(&suggestions)?,
                _ => suggest::serialize_suggestions_markdown(&suggestions)?,
            };

            println!("{}", output);
            Ok(())
        }
        Commands::Docs {
            path,
            format,
            focus,
        } => {
            let docs = generate_llm_documentation(&path)?;

            match format.as_str() {
                "json" => {
                    let output = match focus.as_str() {
                        "prompts" => serde_json::to_string_pretty(&docs.prompts_guide)?,
                        "templates" => serde_json::to_string_pretty(&docs.templates_guide)?,
                        "history" => serde_json::to_string_pretty(&docs.interaction_history)?,
                        _ => serde_json::to_string_pretty(&docs)?,
                    };
                    println!("{}", output);
                }
                "markdown" => {
                    if focus == "all" || focus == "project" {
                        println!("# Project: {}", docs.project_info.name);
                        println!("\n## Description\n{}", docs.project_info.description);
                        println!("\n## Goals");
                        for goal in docs.project_info.goals {
                            println!("- {}", goal);
                        }
                        println!("\n## Progress Markers");
                        for marker in docs.project_info.progress_markers {
                            println!("- {}", marker);
                        }
                    }

                    if focus == "all" || focus == "prompts" {
                        println!("\n# Prompts Guide");
                        println!("\n## Available Prompts");
                        for prompt in &docs.prompts_guide.available_prompts {
                            println!("\n### {}", prompt.id);
                            if let Some(desc) = &prompt.description {
                                println!("\nDescription: {}", desc);
                            }
                            println!("\nTags: {}", prompt.tags.join(", "));
                            println!("\nExample Usage:\n```\n{}\n```", prompt.example_usage);
                        }
                        println!("\n## Recommended Usage");
                        for usage in docs.prompts_guide.recommended_usage {
                            println!("- {}", usage);
                        }
                    }

                    if focus == "all" || focus == "templates" {
                        println!("\n# Templates Guide");
                        println!("\n## Available Templates");
                        for template in &docs.templates_guide.available_templates {
                            println!("\n### {}", template.id);
                            if let Some(desc) = &template.description {
                                println!("\nDescription: {}", desc);
                            }
                            println!("\nTags: {}", template.tags.join(", "));
                            println!("\nTypical Use Cases:");
                            for use_case in &template.typical_use_cases {
                                println!("- {}", use_case);
                            }
                        }
                    }

                    if focus == "all" || focus == "history" {
                        println!("\n# Interaction History");
                        println!("\n## Common Patterns");
                        for pattern in docs.interaction_history.common_patterns {
                            println!("- {}", pattern);
                        }
                        println!("\n## Successful Approaches");
                        for approach in docs.interaction_history.successful_approaches {
                            println!("- {}", approach);
                        }
                        println!("\n## Lessons Learned");
                        for lesson in docs.interaction_history.lessons_learned {
                            println!("- {}", lesson);
                        }
                    }

                    if focus == "all" {
                        println!("\n# Best Practices");
                        for practice in docs.best_practices {
                            println!("- {}", practice);
                        }
                    }
                }
                _ => {
                    println!("Unsupported format. Use 'json' or 'markdown'.");
                }
            }
            Ok(())
        }
    }
}
