use crate::utils;
use crate::yaml;

use fs_extra::copy_items;
use seahorse::Context;

use std::fs;

/// Scatters dotfiles from the current directory to the system
pub fn scatter(ctx: &Context) {
    let mut entries = yaml::get_entries_from_cwd();

    if !ctx.args.is_empty() {
        entries = entries
            .iter()
            .filter(|entry| ctx.args.iter().any(|arg| arg == &entry.name))
            .cloned()
            .collect();
    }

    let longest_entry_display_length = entries
        .iter()
        .map(|entry| {
            entry.name.len()
                + entry
                    .path
                    .parent()
                    .unwrap_or(&entry.path)
                    .display()
                    .to_string()
                    .len()
        })
        .max()
        .unwrap_or(0);

    for entry in entries {
        if fs::read_dir(&entry.name).is_err() && fs::read(&entry.name).is_err() {
            utils::print::warning(format!(
                "{} is being scattered but is not in the current directory",
                &entry.name
            ));

            continue;
        }

        let entry_display_length = entry.name.len()
            + entry
                .path
                .parent()
                .unwrap_or(&entry.path)
                .display()
                .to_string()
                .len();

        match fs::metadata(&entry.name) {
            Ok(metadata) => {
                print!(
                    "copying {} to {}...",
                    &entry.name,
                    &entry.path.parent().unwrap_or(&entry.path).display()
                );

                if metadata.is_dir() {
                    // Recurisvely copy a directory
                    let mut copy_options = fs_extra::dir::CopyOptions::new();
                    copy_options.overwrite = true;
                    copy_options.copy_inside = true;

                    copy_items(
                        &[&entry.name],
                        entry.path.parent().unwrap_or(&entry.path),
                        &copy_options,
                    )
                    .unwrap_or_else(|error| {
                        utils::print::error(format!("can't copy {}: {:?}", &entry.name, error));
                        std::process::exit(1);
                    });

                    utils::print::success(format!(
                        "{} done",
                        String::from(" ")
                            .repeat(longest_entry_display_length - entry_display_length),
                    ));
                } else if metadata.is_file() {
                    // Copy a file
                    fs::copy(&entry.name, &entry.path).unwrap_or_else(|error| {
                        utils::print::error(format!("can't copy {}: {:?}", &entry.name, error));
                        std::process::exit(1);
                    });
                }
            }
            Err(error) => {
                eprintln!("Error getting metadata for {:?}: {:?}", &entry.name, error);
                continue;
            }
        }
    }
}
