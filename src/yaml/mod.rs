mod parse;
mod read;

use crate::utils::cwd;
use parse::Entry;

use std::path::Path;

/// Get the entries of a `track.yaml` from a path
pub fn get_entries(path: &Path) -> Vec<Entry> {
    let yaml = read::to_mapping(path);
    parse::to_vec(&yaml)
}

/// Get the entries of a `track.yaml` from the CWD
pub fn get_entries_from_cwd() -> Vec<Entry> {
    let path = cwd::Cwd::get().join("track.yaml");
    get_entries(path.as_path())
}
