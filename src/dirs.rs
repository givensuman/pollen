use crate::error::PollenError;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

/// Pollen directory structure and configuration management
pub struct PollenDirs {
    /// The main pollen config directory (~/.config/pollen)
    pub config_dir: PathBuf,
    /// The cache directory (~/.config/pollen/cache)
    pub cache_dir: PathBuf,
    /// The files directory (~/.config/pollen/files)
    pub files_dir: PathBuf,
    /// Path to track.yaml file
    pub track_file: PathBuf,
    /// Path to pollen.yaml config file
    pub pollen_config_file: PathBuf,
}

/// Configuration settings for Pollen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollenConfig {
    /// Default configuration file name (default: "track.yaml")
    pub default_track_file: Option<String>,
    /// Enable verbose logging by default
    pub verbose: Option<bool>,
    /// Custom home directory override
    pub home_override: Option<String>,
    /// Cache expiration time in seconds
    pub cache_expiration: Option<u64>,
    /// Maximum number of cached entries
    pub max_cache_entries: Option<usize>,
    /// Auto-commit changes to Git after gather/scatter operations
    pub auto_commit: Option<bool>,
    /// Default commit message for auto-commits
    pub auto_commit_message: Option<String>,
}

impl Default for PollenConfig {
    fn default() -> Self {
        Self {
            default_track_file: Some("track.yaml".to_string()),
            verbose: Some(false),
            home_override: None,
            cache_expiration: Some(86400), // 24 hours
            max_cache_entries: Some(100),
            auto_commit: Some(false),
            auto_commit_message: Some("Pollen auto-sync".to_string()),
        }
    }
}

impl PollenDirs {
    /// Initialize the Pollen directory structure
    pub fn new() -> Result<Self, PollenError> {
        let config_dir = if let Some(pollen_dir) = std::env::var_os("POLLEN_DIR") {
            // Use POLLEN_DIR environment variable if set
            PathBuf::from(pollen_dir)
        } else {
            // Default to ~/.config/pollen
            let home_dir = std::env::var_os("HOME")
                .map(PathBuf::from)
                .ok_or(PollenError::HomeDirectoryNotSet)?;
            
            home_dir.join(".config").join("pollen")
        };
        
        let cache_dir = config_dir.join("cache");
        let files_dir = config_dir.join("files");
        let track_file = config_dir.join("track.yaml");
        let pollen_config_file = config_dir.join("pollen.yaml");
        
        let dirs = Self {
            config_dir,
            cache_dir,
            files_dir,
            track_file,
            pollen_config_file,
        };
        
        dirs.ensure_directories_exist()?;
        dirs.ensure_config_exists()?;
        
        Ok(dirs)
    }
    
    /// Ensure all required directories exist
    fn ensure_directories_exist(&self) -> Result<(), PollenError> {
        fs::create_dir_all(&self.config_dir)
            .map_err(|e| PollenError::Io(e))?;
        
        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| PollenError::Io(e))?;
        
        fs::create_dir_all(&self.files_dir)
            .map_err(|e| PollenError::Io(e))?;
        
        Ok(())
    }
    
    /// Ensure pollen.yaml config file exists with defaults
    fn ensure_config_exists(&self) -> Result<(), PollenError> {
        if !self.pollen_config_file.exists() {
            let default_config = PollenConfig::default();
            let config_content = serde_yaml::to_string(&default_config)
                .map_err(|e| PollenError::Yaml(e))?;
            
            fs::write(&self.pollen_config_file, config_content)
                .map_err(|e| PollenError::Io(e))?;
        }
        
        Ok(())
    }
    
    /// Load the pollen configuration
    pub fn load_config(&self) -> Result<PollenConfig, PollenError> {
        if !self.pollen_config_file.exists() {
            return Ok(PollenConfig::default());
        }
        
        let mut content = String::new();
        File::open(&self.pollen_config_file)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| PollenError::Io(e))?;
        
        let config: PollenConfig = serde_yaml::from_str(&content)
            .map_err(|e| PollenError::Yaml(e))?;
        
        Ok(config)
    }
    
    /// Save the pollen configuration
    pub fn save_config(&self, config: &PollenConfig) -> Result<(), PollenError> {
        let config_content = serde_yaml::to_string(config)
            .map_err(|e| PollenError::Yaml(e))?;
        
        fs::write(&self.pollen_config_file, config_content)
            .map_err(|e| PollenError::Io(e))?;
        
        Ok(())
    }
    
    /// Get the track file path, optionally overridden by config
    pub fn get_track_file_path(&self, config: &PollenConfig) -> PathBuf {
        if let Some(ref track_file) = config.default_track_file {
            if Path::new(track_file).is_absolute() {
                PathBuf::from(track_file)
            } else {
                self.config_dir.join(track_file)
            }
        } else {
            self.track_file.clone()
        }
    }
    
    /// Check if track.yaml exists
    pub fn track_file_exists(&self) -> bool {
        self.track_file.exists()
    }
    
    /// Create a default track.yaml file if it doesn't exist
    pub fn create_default_track_file(&self) -> Result<(), PollenError> {
        if !self.track_file_exists() {
            let default_content = r#"
# Pollen configuration tracking file
# This file defines which configuration files and directories to track
#
# Environment variables:
#   POLLEN_DIR       - Custom directory location (default: ~/.config/pollen)
#   POLLEN_UNDO_LIMIT - Maximum number of undo operations to keep (default: 10)
#
#                      __
#                     // \
#                     \\_/ //
#   ''-.._.-''-.._.. -(||)(')
#                     '''
#
# Files can be stored in the files/ directory and referenced by relative paths
# or you can reference absolute paths to existing files

# Example configuration:
# ".config":
#   - nvim:
#       - alias_as: "neovim"
#       - run_after: "nvim --headless +PackerSync +qa"
#   - bat:
#       - depends_on: "fish"
#       - run_after: "bat cache --build"
#   - fish

# "Documents":
#   - important.txt:
#       - alias_as: "docs"

# You can also reference files stored in the pollen files directory:
# "files/my-config.conf": []
"#;
            
            fs::write(&self.track_file, default_content)
                .map_err(|e| PollenError::Io(e))?;
        }
        
        Ok(())
    }
    
    /// Get a cache file path for a given key
    pub fn get_cache_file_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.cache", key))
    }

    /// Get a file path within the files directory
    pub fn get_files_path(&self, filename: &str) -> PathBuf {
        self.files_dir.join(filename)
    }

    /// Check if a file exists in the files directory
    pub fn files_contains(&self, filename: &str) -> bool {
        self.get_files_path(filename).exists()
    }
    
    /// Clean up old cache files based on configuration
    pub fn cleanup_cache(&self, config: &PollenConfig) -> Result<(), PollenError> {
        let cache_expiration = config.cache_expiration.unwrap_or(86400); // 24 hours
        let now = std::time::SystemTime::now();
        
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(elapsed) = now.duration_since(modified) {
                            if elapsed.as_secs() > cache_expiration {
                                let _ = fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Create or update .gitignore in the files directory
    pub fn manage_files_gitignore(&self) -> Result<(), PollenError> {
        let gitignore_path = self.files_dir.join(".gitignore");
        
        let gitignore_content = r#"# Pollen auto-generated .gitignore
# This file is managed by Pollen to help with Git integration

# Temporary files
*.tmp
*.temp
*~
.DS_Store
Thumbs.db

# Backup files (these are handled by Pollen's cache system)
*.backup
*.bak

# OS-specific files
.directory
desktop.ini

# Common editor files
.vscode/
.idea/
*.swp
*.swo
*~

# You can add your own exclusions below this line:
# Add custom patterns here...
"#;

        std::fs::write(&gitignore_path, gitignore_content)
            .map_err(PollenError::Io)?;
            
        Ok(())
    }
    
    /// Initialize Git repository in files directory if not already present
    pub fn init_files_git_repo(&self) -> Result<bool, PollenError> {
        let git_dir = self.files_dir.join(".git");
        
        if git_dir.exists() {
            return Ok(false); // Already initialized
        }
        
        // Try to initialize git repo
        let output = std::process::Command::new("git")
            .args(&["init"])
            .current_dir(&self.files_dir)
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    self.manage_files_gitignore()?;
                    Ok(true)
                } else {
                    Err(PollenError::InvalidEndpoint(
                        "Failed to initialize Git repository".to_string()
                    ))
                }
            }
            Err(_) => {
                // Git not available, that's okay
                Ok(false)
            }
        }
    }
    
    /// Check if files directory is a Git repository
    pub fn is_files_git_repo(&self) -> bool {
        self.files_dir.join(".git").exists()
    }
    
    /// Stage and commit changes in files directory
    pub fn commit_files_changes(&self, message: &str) -> Result<(), PollenError> {
        if !self.is_files_git_repo() {
            return Err(PollenError::InvalidEndpoint(
                "Files directory is not a Git repository. Use 'pollen git init' first.".to_string()
            ));
        }
        
        // Stage all changes
        let add_output = std::process::Command::new("git")
            .args(&["add", "."])
            .current_dir(&self.files_dir)
            .output()
            .map_err(|e| PollenError::Io(e))?;
            
        if !add_output.status.success() {
            return Err(PollenError::InvalidEndpoint(
                "Failed to stage files for commit".to_string()
            ));
        }
        
        // Check if there are changes to commit
        let status_output = std::process::Command::new("git")
            .args(&["diff", "--cached", "--quiet"])
            .current_dir(&self.files_dir)
            .output()
            .map_err(|e| PollenError::Io(e))?;
            
        if status_output.status.success() {
            // No changes to commit
            return Ok(());
        }
        
        // Commit changes
        let commit_output = std::process::Command::new("git")
            .args(&["commit", "-m", message])
            .current_dir(&self.files_dir)
            .output()
            .map_err(|e| PollenError::Io(e))?;
            
        if !commit_output.status.success() {
            return Err(PollenError::InvalidEndpoint(
                "Failed to commit changes".to_string()
            ));
        }
        
        Ok(())
    }

    /// Save an operation to the operation history
    pub fn save_operation(&self, operation: &Operation) -> Result<(), PollenError> {
        let operation_file = self.cache_dir.join("operations.json");
        
        // Get the undo limit from environment variable or use default
        let undo_limit = std::env::var("POLLEN_UNDO_LIMIT")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10);
        
        // Load existing operations
        let mut operations: Vec<Operation> = if operation_file.exists() {
            let content = std::fs::read_to_string(&operation_file)
                .map_err(PollenError::Io)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        
        // Add new operation
        operations.push(operation.clone());
        
        // Keep only the last N operations based on the limit
        if operations.len() > undo_limit {
            operations.drain(0..operations.len() - undo_limit);
        }
        
        // Save back to file
        let content = serde_json::to_string_pretty(&operations)
            .map_err(|e| PollenError::InvalidMapping(format!("JSON serialization error: {}", e)))?;
        
        std::fs::write(&operation_file, content)
            .map_err(PollenError::Io)?;
        
        Ok(())
    }
    
    /// Get the last operation from history
    pub fn get_last_operation(&self) -> Result<Option<Operation>, PollenError> {
        let operation_file = self.cache_dir.join("operations.json");
        
        if !operation_file.exists() {
            return Ok(None);
        }
        
        let content = std::fs::read_to_string(&operation_file)
            .map_err(PollenError::Io)?;
        
        let operations: Vec<Operation> = serde_json::from_str(&content)
            .map_err(|e| PollenError::InvalidMapping(format!("JSON deserialization error: {}", e)))?;
        
        Ok(operations.last().cloned())
    }
    
    /// Remove the last operation from history
    pub fn remove_last_operation(&self) -> Result<(), PollenError> {
        let operation_file = self.cache_dir.join("operations.json");
        
        if !operation_file.exists() {
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&operation_file)
            .map_err(PollenError::Io)?;
        
        let mut operations: Vec<Operation> = serde_json::from_str(&content)
            .map_err(|e| PollenError::InvalidMapping(format!("JSON deserialization error: {}", e)))?;
        
        if !operations.is_empty() {
            operations.pop();
            
            let content = serde_json::to_string_pretty(&operations)
                .map_err(|e| PollenError::InvalidMapping(format!("JSON serialization error: {}", e)))?;
            
            std::fs::write(&operation_file, content)
                .map_err(PollenError::Io)?;
        }
        
        Ok(())
    }
}

/// Represents an operation that was performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_type: OperationType,
    pub timestamp: u64,
    pub entries: Vec<OperationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Gather,
    Scatter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationEntry {
    pub entry_name: String,
    pub source_path: String,
    pub target_path: String,
    pub backup_path: Option<String>,
}
