use crate::utils;
use crate::yaml;

use seahorse::Context;

#[derive(Debug, Clone)]
struct DiffEntry {
    name: String,
    diff: utils::DiffResult,
}

/// Outputs the current state of sync between
/// "here" and "there" dotfiles
pub fn status(_c: &Context) {
    let entries = yaml::get_entries_from_cwd();
    let mut diffs: Vec<DiffEntry> = Vec::new();

    for entry in entries {
        let local_path = utils::Cwd::get().join(&entry.name);

        let diff = utils::diff(local_path.as_path(), &entry.path);
        if let Some(value) = diff {
            diffs.push(DiffEntry {
                name: entry.name,
                diff: value,
            });
        }
    }

    // Figure out some stuff for prettier printout
    let longest_name = diffs
        .iter()
        .map(|entry| entry.name.len())
        .max()
        .unwrap_or(0);

    let longest_plus = diffs
        .iter()
        .map(|entry| entry.diff.plus_value().to_string().len())
        .max()
        .unwrap_or(0);

    match diffs.len() {
        0 => println!("Everything is up to date."),
        _ => {
            println!(
                "{}{} 🌻",
                format_args!("🐝{}", String::from(" ").repeat(longest_name - 1).as_str()),
                format_args!("🍯{}", String::from(" ").repeat(longest_plus - 1).as_str()),
            );

            diffs.iter().for_each(|entry| {
                println!(
                    "{} {} {}",
                    entry.name.clone()
                        + String::from(" ")
                            .repeat(longest_name - entry.name.len())
                            .as_str(),
                    format_args!(
                        "{}{}",
                        entry.diff.plus(),
                        String::from(" ")
                            .repeat(longest_plus - entry.diff.plus_value().to_string().len())
                            .as_str()
                    ),
                    entry.diff.minus(),
                );
            })
        }
    }
}
