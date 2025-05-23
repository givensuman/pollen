use crate::utils;
use crate::yaml;

use seahorse::Context;

pub fn echo(ctx: &Context) {
    let path = utils::Cwd::get().join("track.yaml");

    let mut entries = yaml::get_entries(path.as_path());

    if !ctx.args.is_empty() {
        entries = entries
            .iter()
            .filter(|entry| ctx.args.iter().any(|arg| arg == &entry.name))
            .cloned()
            .collect();
    }

    for entry in entries {
        match &entry.path.to_str() {
            Some(path) => println!("{} {}", &entry.name, path),
            None => continue,
        }
    }
}
