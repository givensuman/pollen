use crate::{ConfigParser, PollenError, PollenDirs};
use seahorse::Context;

pub fn parse_config(c: &Context) -> Result<(), PollenError> {
    let config_file = c.string_flag("config").ok();
    let verbose = c.bool_flag("verbose");
    
    let parser = ConfigParser::new()?;
    let dirs = PollenDirs::new()?;
    let config = dirs.load_config()?;
    
    let (config_path, _used_config_file) = match config_file.as_deref() {
        Some(path) => (path.to_string(), path.to_string()),
        None => {
            let track_file = dirs.get_track_file_path(&config);
            let path_str = track_file.to_str().ok_or(PollenError::InvalidEndpoint("Invalid config file path".into()))?;
            (path_str.to_string(), track_file.display().to_string())
        }
    };

    let entries = parser.parse_file(&config_path)?;
    
    if verbose {
        let used_config_file = if let Some(ref path) = config_file {
            path.clone()
        } else {
            dirs.get_track_file_path(&config).display().to_string()
        };
        println!("Parsed {} entries from {}", entries.len(), used_config_file);
        
        // Show aliases
        let aliases = parser.list_aliases(&entries);
        if !aliases.is_empty() {
            println!("\nAliases:");
            for (alias, path) in aliases {
                println!("  {} -> {}", alias, path);
            }
        }
        println!();
    }
    
    for entry in &entries {
        println!("Entry: {}", entry.name);
        if let Some(alias) = &entry.alias_as {
            println!("  Alias: {}", alias);
        }
        println!("  Path: {}", entry.path.display());
        
        if !entry.depends_on.is_empty() {
            println!("  Dependencies: {}", entry.depends_on.join(", "));
        }
        
        if let Some(run_before) = &entry.run_before {
            println!("  Run before: {}", run_before);
        }
        
        if let Some(run_after) = &entry.run_after {
            println!("  Run after: {}", run_after);
        }
        
        println!();
    }
    
    Ok(())
}
