use crate::yaml;

use seahorse::{Command, Context};

use std::env;

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
