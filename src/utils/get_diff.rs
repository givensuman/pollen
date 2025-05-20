use std::path::Path;
use std::{fmt, fs, ops};

use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
enum DiffType {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct Diff {
    value: usize,
    diff_type: DiffType,
}

impl ops::AddAssign<usize> for Diff {
    fn add_assign(&mut self, rhs: usize) {
        self.value += rhs;
    }
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.diff_type {
            DiffType::Plus => write!(f, "{}", format!("+{}", self.value).green()),
            DiffType::Minus => write!(f, "{}", format!("-{}", self.value).red()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffResult {
    plus: Diff,
    minus: Diff,
}

impl DiffResult {
    pub fn plus(&self) -> String {
        format!("{}", self.plus)
    }
    pub fn minus(&self) -> String {
        format!("{}", self.minus)
    }

    pub fn plus_value(&self) -> usize {
        self.plus.value
    }
    // pub fn minus_value(&self) -> usize {
    //     self.minus.value
    // }
}

/// Get the diffs of two files or directories
/// for usage in the `status` command
///
/// Returns Option<DiffResult> in the form of "+x\t-y"
pub fn get_diff(a: &Path, b: &Path) -> Option<DiffResult> {
    // Determine if a and b are files or directories
    let is_file = match a.is_file() && b.is_file() {
        true => true,
        false => {
            if a.is_dir() && b.is_dir() {
                false
            } else {
                eprintln!("tried to diff {:#?} against {:#?},", a, b);
                eprintln!("but one is a directory and one is a file");

                return None;
            }
        }
    };

    // Create TextDiff<...> from their contents
    let mut contents = (String::new(), String::new());
    let diff_result = if is_file {
        contents.0 = get_lines_from_file(a);
        contents.1 = get_lines_from_file(b);
        diff(contents.0.as_str(), contents.1.as_str())
    } else {
        contents.0 = get_lines_from_dir(a);
        contents.1 = get_lines_from_dir(b);
        diff(contents.0.as_str(), contents.1.as_str())
    };

    let mut plus = Diff {
        value: 0,
        diff_type: DiffType::Plus,
    };
    let mut minus = Diff {
        value: 0,
        diff_type: DiffType::Minus,
    };
    for change in diff_result.iter_all_changes() {
        match change.tag() {
            ChangeTag::Insert => plus += 1,
            ChangeTag::Delete => minus += 1,
            _ => {}
        }
    }

    Some(DiffResult { plus, minus })
}

/// Wrapper around similar::TextDiff crate
fn diff<'a, 'b, 'c>(old: &'a str, new: &'b str) -> TextDiff<'a, 'b, 'c, str> {
    TextDiff::from_lines(old, new)
}

fn get_lines_from_file(file: &Path) -> String {
    match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read file -> {:#?} -> {:#?}", file, e);
            String::new()
        }
    }
}

pub fn get_lines_from_dir(dir: &Path) -> String {
    let mut lines = Vec::new();

    let dir_str = match dir.to_str() {
        Some(s) => s,
        None => {
            eprintln!("failed to convert path to string -> {:#?}", dir);
            return String::new();
        }
    };

    for entry in WalkDir::new(dir_str).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path_str = match entry.path().to_str() {
                Some(s) => s,
                None => {
                    eprintln!("failed to convert path to string -> {:#?}", entry.path());
                    continue;
                }
            };

            match fs::read_to_string(entry.path()) {
                Ok(s) => lines.push(s),
                Err(e) => {
                    eprintln!("failed to read file -> {:#?} -> {:#?}", path_str, e);
                    continue;
                }
            };
        }
    }

    lines.join("\n")
}
