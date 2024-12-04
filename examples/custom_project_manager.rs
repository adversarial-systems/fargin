use anyhow::Result;
use fargin::config::ProjectConfig;

fn main() -> Result<()> {
    let config = ProjectConfig::new(
        "Example Project".to_string(),
        "A demonstration of project management".to_string(),
    );

    // Simplified project configuration
    println!("Project Name: {}", config.name);
    println!("Project Description: {}", config.description);
    println!("Created At: {:?}", config.created_at);
    println!("Last Updated: {:?}", config.last_updated);

    Ok(())
}
