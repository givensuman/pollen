mod parse;
mod read;

use parse::Entry;

pub fn get_entries(path: &str) -> Vec<Entry> {
    let yaml = read::to_mapping(path);
    parse::to_vec(&yaml)
}
