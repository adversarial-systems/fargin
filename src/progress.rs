use crate::config::ProjectConfig;
use anyhow::Result;
use std::path::PathBuf;

pub fn show_progress(path: PathBuf) -> Result<ProgressReport> {
    let config = ProjectConfig::load(&path)?;

    let total_markers = config.progress_markers.len();
    let completed_markers = config
        .progress_markers
        .iter()
        .filter(|m| m.completed)
        .count();

    let report = ProgressReport {
        project_name: config.name,
        total_markers,
        completed_markers,
        last_updated: config.last_updated,
        markers: config.progress_markers,
    };

    // Print the report
    println!("Progress Report for {}", report.project_name);
    println!("Last updated: {}", report.last_updated);
    println!(
        "Progress: {}/{} markers completed",
        report.completed_markers, report.total_markers
    );

    if !report.markers.is_empty() {
        println!("\nProgress Markers:");
        for marker in &report.markers {
            let status = if marker.completed { "✓" } else { "×" };
            println!("{} {} - {}", status, marker.name, marker.description);
            if let Some(completed_at) = marker.completed_at {
                println!("  Completed at: {}", completed_at);
            }
        }
    }

    Ok(report)
}

#[derive(Debug)]
pub struct ProgressReport {
    pub project_name: String,
    pub total_markers: usize,
    pub completed_markers: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub markers: Vec<crate::config::ProgressMarker>,
}
