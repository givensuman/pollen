use crate::yaml;

use fs_extra::copy_items;
use seahorse::Context;

use std::fs;

pub fn gather(ctx: &Context) {
    let mut entries = yaml::get_entries_from_cwd();

    if !ctx.args.is_empty() {
        entries = entries
            .iter()
            .filter(|entry| ctx.args.iter().any(|arg| arg == &entry.name))
            .cloned()
            .collect();
    }

    for entry in entries {
        if fs::read_dir(&entry.path).is_err() && fs::read(&entry.path).is_err() {
            eprintln!(
                "{:?} is being gathered but not in the expected place of {:?}",
                &entry.name, &entry.path,
            );
        }

        match fs::metadata(&entry.path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let mut copy_options = fs_extra::dir::CopyOptions::new();
                    copy_options.overwrite = true;
                    copy_options.copy_inside = true;

                    copy_items(&[&entry.path], &entry.name, &copy_options).unwrap_or_else(
                        |error| panic!("can't copy {:?}: {:?}", &entry.name, error),
                    );
                } else if metadata.is_file() {
                    fs::copy(&entry.path, &entry.name)
                        .unwrap_or_else(|_| panic!("can't copy {:?}", &entry.name));
                }

                println!("copied {:?} to {:?}", &entry.path, &entry.name);
            }
            Err(error) => {
                eprintln!("Error getting metadata for {:?}: {:?}", &entry.name, error);
                continue;
            }
        }
    }
}
