use crate::{PollenDirs, PollenError, OperationType, OperationEntry};
use crate::cli::utils::copy_dir_all;
use seahorse::Context;
use std::{fs, path::Path};

pub fn undo_last_operation(_c: &Context) -> Result<(), PollenError> {
    let dirs = PollenDirs::new()?;
    
    // Get the last operation
    let last_operation = match dirs.get_last_operation()? {
        Some(op) => op,
        None => {
            println!("No operations to undo.");
            return Ok(());
        }
    };
    
    println!("Undoing last {} operation with {} entries...", 
             match last_operation.operation_type {
                 OperationType::Gather => "gather",
                 OperationType::Scatter => "scatter",
             },
             last_operation.entries.len());
    
    let mut restored_count = 0;
    let mut failed_count = 0;
    let mut removed_count = 0;
    
    for op_entry in &last_operation.entries {
        match undo_single_operation_entry(op_entry, &last_operation.operation_type) {
            Ok(undo_result) => {
                match undo_result {
                    UndoResult::Restored => {
                        restored_count += 1;
                        println!("✓ Restored: {}", op_entry.entry_name);
                    }
                    UndoResult::Removed => {
                        removed_count += 1;
                        println!("✓ Removed: {}", op_entry.entry_name);
                    }
                    UndoResult::NoBackup => {
                        println!("• No backup to restore for: {}", op_entry.entry_name);
                    }
                }
            }
            Err(e) => {
                failed_count += 1;
                eprintln!("✗ Failed to undo {}: {}", op_entry.entry_name, e);
            }
        }
    }
    
    // Remove the operation from history
    dirs.remove_last_operation()?;
    
    println!("\nUndo complete:");
    if restored_count > 0 {
        println!("  Restored from backup: {}", restored_count);
    }
    if removed_count > 0 {
        println!("  Removed files: {}", removed_count);
    }
    if failed_count > 0 {
        println!("  Failed: {}", failed_count);
    }
    
    Ok(())
}

#[derive(Debug)]
enum UndoResult {
    Restored,    // File was restored from backup
    Removed,     // File was removed (no backup existed)
    NoBackup,    // No action taken, no backup found
}

fn undo_single_operation_entry(op_entry: &OperationEntry, operation_type: &OperationType) -> Result<UndoResult, PollenError> {
    match operation_type {
        OperationType::Gather => {
            // For gather operations, we need to:
            // 1. Remove the file from the files directory (target_path)
            // 2. Restore the backup if it exists
            
            let target_path = Path::new(&op_entry.target_path);
            
            // Remove the gathered file
            if target_path.exists() {
                if target_path.is_dir() {
                    fs::remove_dir_all(target_path).map_err(PollenError::Io)?;
                } else {
                    fs::remove_file(target_path).map_err(PollenError::Io)?;
                }
            }
            
            // Restore backup if it exists
            if let Some(backup_path_str) = &op_entry.backup_path {
                let backup_path = Path::new(backup_path_str);
                if backup_path.exists() {
                    // Ensure target directory exists
                    if let Some(target_parent) = target_path.parent() {
                        fs::create_dir_all(target_parent).map_err(PollenError::Io)?;
                    }
                    
                    if backup_path.is_dir() {
                        copy_dir_all(backup_path, target_path)?;
                    } else {
                        fs::copy(backup_path, target_path).map_err(PollenError::Io)?;
                    }
                    
                    // Remove the backup file
                    if backup_path.is_dir() {
                        fs::remove_dir_all(backup_path).map_err(PollenError::Io)?;
                    } else {
                        fs::remove_file(backup_path).map_err(PollenError::Io)?;
                    }
                    
                    return Ok(UndoResult::Restored);
                } else {
                    return Ok(UndoResult::NoBackup);
                }
            }
            
            Ok(UndoResult::Removed)
        }
        
        OperationType::Scatter => {
            // For scatter operations, we need to:
            // 1. Remove the scattered file from the target location (source_path in the op_entry represents the original target)
            // 2. Restore the backup if it exists
            
            let target_path = Path::new(&op_entry.source_path); // This is actually the target location for scatter
            
            // Remove the scattered file
            if target_path.exists() {
                if target_path.is_dir() {
                    fs::remove_dir_all(target_path).map_err(PollenError::Io)?;
                } else {
                    fs::remove_file(target_path).map_err(PollenError::Io)?;
                }
            }
            
            // Restore backup if it exists
            if let Some(backup_path_str) = &op_entry.backup_path {
                let backup_path = Path::new(backup_path_str);
                if backup_path.exists() {
                    // Ensure target directory exists
                    if let Some(target_parent) = target_path.parent() {
                        fs::create_dir_all(target_parent).map_err(PollenError::Io)?;
                    }
                    
                    if backup_path.is_dir() {
                        copy_dir_all(backup_path, target_path)?;
                    } else {
                        fs::copy(backup_path, target_path).map_err(PollenError::Io)?;
                    }
                    
                    // Remove the backup file
                    if backup_path.is_dir() {
                        fs::remove_dir_all(backup_path).map_err(PollenError::Io)?;
                    } else {
                        fs::remove_file(backup_path).map_err(PollenError::Io)?;
                    }
                    
                    return Ok(UndoResult::Restored);
                } else {
                    return Ok(UndoResult::NoBackup);
                }
            }
            
            Ok(UndoResult::Removed)
        }
    }
}
