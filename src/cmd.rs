use crate::yaml;

use fs_extra;
use seahorse::{Command, Context};

use std::env;
use std::fs;
use std::path::Path;

fn get_cwd() -> String {
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(error) => panic!("error trying to get current directory: {:#?}", error),
    };

    match cwd.to_str() {
        Some(path) => String::from(path),
        None => panic!("error converting current directory to string"),
    }
}

fn scatter(c: &Context) {
    let mut path = get_cwd();
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str("track.yaml");

    let entries = yaml::get_entries(&path);
    for entry in entries {
        if fs::read_dir(&entry.name).is_err() && fs::read(&entry.name).is_err() {
            eprintln!(
                "{:?} is being tracked but not in current directory",
                &entry.name
            );
        }

        match fs::metadata(&entry.name) {
            Ok(metadata) => {
                println!("copying {:?} to {:?}", &entry.name, &entry.path);
                if metadata.is_dir() {
                    let mut copy_options = fs_extra::dir::CopyOptions::new();
                    copy_options.overwrite = true;
                    copy_options.copy_inside = true;

                    fs_extra::copy_items(&[&entry.name], &entry.path, &copy_options)
                        .unwrap_or_else(|error| {
                            panic!("can't copy {:?}: {:?}", &entry.name, error)
                        });
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

pub fn scatter_cmd() -> Command {
    Command::new("scatter")
        .description("Scatter command")
        .action(scatter)
}

fn gather(c: &Context) {
    println!("hello from cmd.rs: {:?}", c.args);
}

pub fn gather_cmd() -> Command {
    Command::new("gather")
        .description("Gather command")
        .action(gather)
}
