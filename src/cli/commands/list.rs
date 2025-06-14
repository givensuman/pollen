use crate::{ConfigParser, PollenDirs, PollenError};
use seahorse::Context;

pub fn list_entries(c: &Context) -> Result<(), PollenError> {
    let config_file = c.string_flag("config").ok();
    let show_paths = c.bool_flag("paths");
    
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

    if show_paths {
        for entry in &entries {
            let display_name = entry.get_display_name();
            if display_name != entry.name {
                println!("{} ({})", entry.path.display(), display_name);
            } else {
                println!("{}", entry.path.display());
            }
        }
    } else {
        for entry in &entries {
            let display_name = entry.get_display_name();
            if display_name != entry.name {
                println!("{} ({})", display_name, entry.name);
            } else {
                println!("{}", entry.name);
            }
        }
    }
    
    let aliases = parser.list_aliases(&entries);
    if !aliases.is_empty() {
        println!();
        if show_paths {
            println!("Aliases (alias -> path):");
            for (alias, _path) in &aliases {
                // Find the entry with this alias to get its path
                if let Some(entry) = entries.iter().find(|e| e.alias_as.as_ref().map(|a| a.as_str()) == Some(*alias)) {
                    println!("  {} -> {}", alias, entry.path.display());
                }
            }
        } else {
            println!("Aliases:");
            for (alias, path) in aliases {
                println!("  {} -> {}", alias, path);
            }
        }
    }
    
    Ok(())
}
