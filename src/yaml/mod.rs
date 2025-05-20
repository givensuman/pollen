mod parse;
mod read;

use crate::utils;
use parse::Entry;

use std::path::Path;

pub fn get_entries(path: &Path) -> Vec<Entry> {
    let yaml = read::to_mapping(path);
    parse::to_vec(&yaml)
}

pub fn get_entries_from_cwd() -> Vec<Entry> {
    let mut path = match utils::get_cwd() {
        Ok(path) => path,
        Err(error) => panic!("unable to determine current directory: {:?}", error),
    };
    path.push("track.yaml");

    get_entries(path.as_path())
}
