use crate::{ConfigParser, PollenDirs, Entry, PollenError, Operation, OperationType, OperationEntry};
use crate::cli::utils::{execute_shell_command, copy_dir_all};
use seahorse::Context;
use std::{fs, time::SystemTime};

pub fn gather_files(c: &Context) -> Result<(), PollenError> {
    let config_file = c.string_flag("config").ok();
    let target_entries: Vec<String> = c.args.clone();
    
    let parser = ConfigParser::new()?;
    let entries = parser.parse_file(config_file.as_deref().unwrap_or("config.toml"))?;
    
    // Filter entries based on command line arguments
    let entries_to_gather: Vec<_> = if target_entries.is_empty() {
        // If no specific entries specified, gather all
        entries
    } else {
        entries
            .into_iter()
            .filter(|entry| {
                // Match by name or alias
                target_entries.iter().any(|target| {
                    entry.name == *target || 
                    entry.alias_as.as_ref() == Some(target) ||
                    entry.get_display_name() == target
                })
            })
            .collect()
    };
    
    if entries_to_gather.is_empty() {
        println!("No matching entries found to gather.");
        return Ok(());
    }
    
    println!("Gathering {} entries...", entries_to_gather.len());
    
    let dirs = PollenDirs::new()?;
    let mut gathered_count = 0;
    let mut backed_up_count = 0;
    let mut failed_count = 0;
    let mut operation_entries = Vec::new();
    
    for entry in &entries_to_gather {
        match gather_single_entry(entry, &dirs) {
            Ok((gathered, backed_up, backup_path)) => {
                if gathered {
                    gathered_count += 1;
                    println!("🌻 Gathered: {}", entry.get_display_name());
                    
                    // Record the operation
                    let target_filename = entry.name.replace('/', "_").replace('\\', "_");
                    let target_path = dirs.get_files_path(&target_filename);
                    operation_entries.push(OperationEntry {
                        entry_name: entry.name.clone(),
                        source_path: entry.path.display().to_string(),
                        target_path: target_path.display().to_string(),
                        backup_path,
                    });
                }
                if backed_up {
                    backed_up_count += 1;
                    println!("  → Backed up existing file to cache");
                }
            }
            Err(e) => {
                failed_count += 1;
                eprintln!("✗ Failed to gather {}: {}", entry.get_display_name(), e);
            }
        }
    }
    
    println!("\nGather complete:");
    println!("  Gathered: {}", gathered_count);
    if backed_up_count > 0 {
        println!("  Backed up: {}", backed_up_count);
    }
    if failed_count > 0 {
        println!("  Failed: {}", failed_count);
    }
    
    // Save the operation to history if any entries were gathered
    if !operation_entries.is_empty() {
        let operation = Operation {
            operation_type: OperationType::Gather,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            entries: operation_entries,
        };
        
        if let Err(e) = dirs.save_operation(&operation) {
            eprintln!("Warning: Failed to save operation to history: {}", e);
        }
        
        // Auto-commit if enabled
        let config = dirs.load_config()?;
        if config.auto_commit.unwrap_or(false) && dirs.is_files_git_repo() {
            let message = config.auto_commit_message
                .unwrap_or_else(|| "Pollen gather operation".to_string());
            if let Err(e) = dirs.commit_files_changes(&message) {
                eprintln!("Warning: Failed to auto-commit changes: {}", e);
            } else {
                println!("✓ Auto-committed changes to Git");
            }
        }
    }
    
    Ok(())
}

fn gather_single_entry(entry: &Entry, dirs: &PollenDirs) -> Result<(bool, bool, Option<String>), PollenError> {
    let source_path = &entry.path;
    let target_filename = entry.name.replace('/', "_").replace('\\', "_");
    let target_path = dirs.get_files_path(&target_filename);
    
    // Check if source exists
    if !source_path.exists() {
        return Err(PollenError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source file does not exist: {}", source_path.display())
        )));
    }
    
    // Execute run_before command if specified
    if let Some(run_before) = &entry.run_before {
        println!("  → Running pre-gather command: {}", run_before);
        if let Err(e) = execute_shell_command(run_before) {
            return Err(PollenError::InvalidEndpoint(
                format!("Failed to execute run_before command '{}': {}", run_before, e)
            ));
        }
    }
    
    let mut backed_up = false;
    let mut backup_path_str = None;
    
    // If target already exists, back it up to cache
    if target_path.exists() {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| PollenError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get system time"
            )))?
            .as_secs();
        
        let backup_filename = format!("{}_{}.backup", target_filename, timestamp);
        let backup_path = dirs.get_cache_file_path(&backup_filename);
        
        // Ensure cache directory exists
        if let Some(cache_parent) = backup_path.parent() {
            fs::create_dir_all(cache_parent)
                .map_err(PollenError::Io)?;
        }
        
        fs::copy(&target_path, &backup_path)
            .map_err(PollenError::Io)?;
        backed_up = true;
        backup_path_str = Some(backup_path.display().to_string());
    }
    
    // Ensure target directory exists
    if let Some(target_parent) = target_path.parent() {
        fs::create_dir_all(target_parent)
            .map_err(PollenError::Io)?;
    }
    
    // Copy source to target
    if source_path.is_dir() {
        copy_dir_all(source_path, &target_path)?;
    } else {
        fs::copy(source_path, &target_path)
            .map_err(PollenError::Io)?;
    }
    
    // Execute run_after command if specified
    if let Some(run_after) = &entry.run_after {
        println!("  → Running post-gather command: {}", run_after);
        if let Err(e) = execute_shell_command(run_after) {
            return Err(PollenError::InvalidEndpoint(
                format!("Failed to execute run_after command '{}': {}", run_after, e)
            ));
        }
    }
    
    Ok((true, backed_up, backup_path_str))
}
