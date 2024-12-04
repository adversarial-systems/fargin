use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn reset_project(path: PathBuf, force: bool) -> Result<()> {
    let fargin_dir = path.join(".fargin");
    
    if !fargin_dir.exists() {
        println!("No Fargin configuration found in the specified directory.");
        return Ok(());
    }
    
    if !force {
        print!("This will remove all Fargin related files and directories. Are you sure? [y/N] ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Reset cancelled.");
            return Ok(());
        }
    }
    
    // List of directories to remove
    let dirs = [
        ".fargin/prompts",
        ".fargin/history",
        ".fargin/templates",
    ];
    
    // Remove all subdirectories first
    for dir in dirs.iter() {
        let dir_path = path.join(dir);
        if dir_path.exists() {
            fs::remove_dir_all(&dir_path)
                .with_context(|| format!("Failed to remove directory: {}", dir_path.display()))?;
        }
    }
    
    // Remove any remaining files in .fargin
    for entry in fs::read_dir(&fargin_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove file: {}", path.display()))?;
        }
    }
    
    // Finally remove the .fargin directory itself
    fs::remove_dir(&fargin_dir)
        .with_context(|| format!("Failed to remove directory: {}", fargin_dir.display()))?;
    
    println!("Successfully reset Fargin configuration.");
    Ok(())
}
