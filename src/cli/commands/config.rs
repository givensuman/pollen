use crate::{PollenDirs, PollenError};
use seahorse::Context;
use std::path::PathBuf;

pub fn show_config(_c: &Context) -> Result<(), PollenError> {
    let dirs = PollenDirs::new()?;
    let config = dirs.load_config()?;
    
    println!("Pollen Configuration");
    println!("==================");
    println!();
    
    // Show directories
    if let Some(pollen_dir) = std::env::var_os("POLLEN_DIR") {
        println!("POLLEN_DIR: {}", PathBuf::from(pollen_dir).display());
    } else {
        println!("POLLEN_DIR: <not set> (using default)");
    }
    println!("Configuration directory: {}", dirs.config_dir.display());
    println!("Cache directory: {}", dirs.cache_dir.display());
    println!("Files directory: {}", dirs.files_dir.display());
    println!("Track file: {}", dirs.track_file.display());
    println!("Config file: {}", dirs.pollen_config_file.display());
    println!();
    
    // Show environment variables
    let undo_limit = std::env::var("POLLEN_UNDO_LIMIT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    
    if let Ok(limit_str) = std::env::var("POLLEN_UNDO_LIMIT") {
        println!("POLLEN_UNDO_LIMIT: {} (max undo operations)", limit_str);
    } else {
        println!("POLLEN_UNDO_LIMIT: <not set> (using default: {})", undo_limit);
    }
    println!();
    
    // Show pollen.yaml settings
    println!("Settings from pollen.yaml:");
    if let Some(ref track_file) = config.default_track_file {
        println!("  Default track file: {}", track_file);
    }
    println!("  Verbose mode: {}", config.verbose.unwrap_or(false));
    println!("  Cache expiration: {} seconds", config.cache_expiration.unwrap_or(86400));
    println!("  Max cache entries: {}", config.max_cache_entries.unwrap_or(100));
    println!("  Auto-commit: {}", config.auto_commit.unwrap_or(false));
    if let Some(ref message) = config.auto_commit_message {
        println!("  Auto-commit message: \"{}\"", message);
    }
    println!();
    
    // Show Git status
    if dirs.is_files_git_repo() {
        println!("Git repository: ✓ initialized in files directory");
    } else {
        println!("Git repository: ✗ not initialized (use 'pollen git init')");
    }
    
    Ok(())
}
