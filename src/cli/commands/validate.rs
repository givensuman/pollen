use crate::{ConfigParser, PollenError, PollenDirs};
use seahorse::Context;

pub fn validate_config(c: &Context) -> Result<(), PollenError> {
    let config_file = c.string_flag("config").ok();
    
    let parser = ConfigParser::new()?;
    let dirs = PollenDirs::new()?;
    let config = dirs.load_config()?;
    
    let (config_path, used_config_file) = match config_file.as_deref() {
        Some(path) => (path.to_string(), path.to_string()),
        None => {
            let track_file = dirs.get_track_file_path(&config);
            let path_str = track_file.to_str().ok_or(PollenError::InvalidEndpoint("Invalid config file path".into()))?;
            (path_str.to_string(), track_file.display().to_string())
        }
    };
    
    let entries = parser.parse_file(&config_path)?;
    
    // Validation is done during parsing, so if we get here, it's valid
    println!("Configuration file '{}' is valid!", used_config_file);
    println!("Found {} entries", entries.len());
    
    let aliases = parser.list_aliases(&entries);
    if !aliases.is_empty() {
        println!("Found {} aliases", aliases.len());
    }
    
    Ok(())
}
