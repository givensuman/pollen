use crate::{ConfigParser, PollenError};
use seahorse::Context;

pub fn list_entries(c: &Context) -> Result<(), PollenError> {
    let config_file = c.string_flag("config").unwrap();
    let show_paths = c.bool_flag("paths");
    
    let parser = ConfigParser::new()?;
    let entries = parser.parse_file(&config_file)?;

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
