use log::{LevelFilter, info};
use simplelog::{CombinedLogger, Config, WriteLogger, TermLogger, TerminalMode, ColorChoice};
use std::fs::File;
use std::path::Path;

pub fn setup_logging(log_file: &str, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = Path::new(log_file);
    
    // Create log directory if it doesn't exist
    if let Some(parent) = log_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    CombinedLogger::init(vec![
        // Log to terminal with colors
        TermLogger::new(
            level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // Log to file
        WriteLogger::new(
            level,
            Config::default(),
            File::create(log_path)?,
        ),
    ])?;
    
    info!("Logging initialized at level {:?} to {}", level, log_file);
    Ok(())
}