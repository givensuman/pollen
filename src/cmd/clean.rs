use crate::utils;
use crate::yaml;

use fs_extra::remove_items;
use seahorse::Context;

pub fn clean(_: &Context) {
    let path = utils::Cwd::get().join("track.yaml");

    let entries = yaml::get_entries(path.as_path());
    let mut from_paths = Vec::new();
    for entry in entries {
        from_paths.push(entry.path.to_str().unwrap().to_string());
    }

    remove_items(&from_paths).unwrap();
}
