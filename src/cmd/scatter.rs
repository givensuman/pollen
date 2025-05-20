use crate::utils;
use crate::yaml;

use fs_extra::copy_items;
use seahorse::Context;

use std::fs;

pub fn scatter(ctx: &Context) {
    let mut path = match utils::get_cwd() {
        Ok(path) => path,
        Err(error) => panic!("unable to determine current directory: {:?}", error),
    };
    path.push("track.yaml");

    let mut entries = yaml::get_entries(path.as_path());

    if !ctx.args.is_empty() {
        entries = entries
            .iter()
            .filter(|entry| ctx.args.iter().any(|arg| arg == &entry.name))
            .cloned()
            .collect();
    }

    for entry in entries {
        if fs::read_dir(&entry.name).is_err() && fs::read(&entry.name).is_err() {
            eprintln!(
                "{:?} is being scattered but not in current directory",
                &entry.name
            );
        }

        match fs::metadata(&entry.name) {
            Ok(metadata) => {
                println!(
                    "copying {:?} to {:?}",
                    &entry.name,
                    &entry.path.parent().unwrap_or(&entry.path)
                );
                if metadata.is_dir() {
                    let mut copy_options = fs_extra::dir::CopyOptions::new();
                    copy_options.overwrite = true;
                    copy_options.copy_inside = true;

                    copy_items(
                        &[&entry.name],
                        entry.path.parent().unwrap_or(&entry.path),
                        &copy_options,
                    )
                    .unwrap_or_else(|error| panic!("can't copy {:?}: {:?}", &entry.name, error));
                } else if metadata.is_file() {
                    fs::copy(&entry.name, &entry.path)
                        .unwrap_or_else(|_| panic!("can't copy {:?}", &entry.name));
                }
            }
            Err(error) => {
                eprintln!("Error getting metadata for {:?}: {:?}", &entry.name, error);
                continue;
            }
        }
    }
}
