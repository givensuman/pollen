use std::fs::{File, metadata};

/// Takes in a path and returns a Vec<String> containing all the lines
/// in the file or directory, recursively traversed
pub fn lines_from_path(path: Path) -> Result<Vec<String>> {}

fn lines_from_file(file: File) -> Result<Vec<String>> {
    let content = file.read_to_string();
}
