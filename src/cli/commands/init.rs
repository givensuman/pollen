use crate::{PollenDirs, PollenError};
use std::{env, path::PathBuf};

pub fn init_pollen() -> Result<(), PollenError> {
    let dirs = PollenDirs::new()?;
    
    println!("Initializing Pollen configuration...");
    
    // Show if using custom POLLEN_DIR
    if let Some(pollen_dir) = env::var_os("POLLEN_DIR") {
        println!("Using custom POLLEN_DIR: {}", PathBuf::from(pollen_dir).display());
    } else {
        println!("Using default location (set POLLEN_DIR to customize)");
    }
    
    println!("Configuration directory: {}", dirs.config_dir.display());
    println!("Cache directory: {}", dirs.cache_dir.display());
    println!("Files directory: {}", dirs.files_dir.display());
    
    // Create default track file if it doesn't exist
    if !dirs.track_file_exists() {
        dirs.create_default_track_file()?;
        println!("Created default track.yaml file: {}", dirs.track_file.display());
    } else {
        println!("Track file already exists: {}", dirs.track_file.display());
    }
    
    // Load and display config
    let config = dirs.load_config()?;
    println!("Configuration file: {}", dirs.pollen_config_file.display());
    
    if config.verbose.unwrap_or(false) {
        println!("Default verbose mode: enabled");
    }
    
    // Offer to initialize Git repository
    if !dirs.is_files_git_repo() {
        println!("\nWould you like to initialize a Git repository in the files directory?");
        println!("This allows you to backup your configuration files to GitHub or other Git hosts.");
        println!("You can also do this later with 'pollen git init'");
        
        // For now, just show the command they can run
        println!("\nTo initialize Git later, run:");
        println!("  pollen git init");
    } else {
        println!("\nGit repository already exists in files directory");
    }
    
    println!("\nPollen has been initialized successfully!");
    println!("You can now edit {} to define your configuration files to track.", 
             dirs.track_file.display());
    println!("Store configuration files in {} for Pollen to manage.", 
             dirs.files_dir.display());
    
    Ok(())
}
