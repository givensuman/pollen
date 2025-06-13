use crate::{PollenDirs, PollenError};
use seahorse::Context;

pub fn change_to_pollen_dir(c: &Context) -> Result<(), PollenError> {
    let dirs = PollenDirs::new()?;
    
    let subdirectory = c.args.get(0).map(|s| s.as_str());
    
    let target_dir = match subdirectory {
        Some("files") => dirs.files_dir.clone(),
        Some("cache") => dirs.cache_dir.clone(),
        Some("config") => dirs.config_dir.clone(),
        Some(other) => {
            // Check if it's a subdirectory within the pollen directory
            let custom_path = dirs.config_dir.join(other);
            if custom_path.exists() && custom_path.is_dir() {
                custom_path
            } else {
                return Err(PollenError::InvalidEndpoint(
                    format!("Subdirectory '{}' does not exist in pollen directory", other)
                ));
            }
        }
        None => dirs.config_dir.clone(),
    };
    
    // Since we can't actually change the shell's working directory from within a Rust program,
    // we'll output the command for the user to run
    println!("To change to the Pollen directory, run:");
    println!();
    
    // Detect the shell and provide the appropriate command
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let shell_name = shell.split('/').last().unwrap_or("sh");
    
    match shell_name {
        "fish" => {
            println!("  cd {}", target_dir.display());
            println!();
            println!("Or copy this command to your clipboard:");
            println!("  echo 'cd {}' | pbcopy   # macOS", target_dir.display());
            println!("  echo 'cd {}' | xclip    # Linux", target_dir.display());
        }
        "zsh" | "bash" => {
            println!("  cd {}", target_dir.display());
            println!();
            println!("Or you can source this command:");
            println!("  cd {}", target_dir.display());
        }
        _ => {
            println!("  cd {}", target_dir.display());
        }
    }
    
    println!();
    println!("Directory contents:");
    if let Ok(entries) = std::fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            if path.is_dir() {
                println!("  {}/ ", name.to_string_lossy());
            } else {
                println!("  {}", name.to_string_lossy());
            }
        }
    }
    
    println!();
    
    // Show available subdirectories
    if subdirectory.is_none() {
        println!("Available subdirectories:");
        println!("  pollen cd files  - Go to files directory");
        println!("  pollen cd cache  - Go to cache directory");
        println!("  pollen cd config - Go to config directory (same as root)");
    }
    
    Ok(())
}
