use dirs::home_dir;
use serde_yaml::{Mapping, Value};

use std::path::{Path, PathBuf};

fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(path) => path,
        None => match std::env::var("HOME") {
            Ok(value) => PathBuf::from(value),
            Err(_) => panic!("failed to get home directory"),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hooks {
    run_before: Option<String>,
    run_after: Option<String>,
    depends_on: Option<String>,
    alias_as: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub hooks: Option<Hooks>,
}

/// Takes the key-value pair from a mapping with one entry
/// e.g. "run_before": "blah blah blah"
/// returns ("run_before", "blah blah blah")
fn extract_key_value_from_single_mapping(mapping: &Mapping) -> (&Value, &Value) {
    match mapping.iter().next() {
        Some(value) => value,
        None => {
            eprintln!(
                "failed to extract key-value pair from mapping: {:?}",
                mapping
            );
            eprintln!("your track.yaml file is likely malformed...");
            std::process::exit(1);
        }
    }
}

pub trait ForciblyString {
    fn to_string_or_crash(&self) -> String;
}

impl ForciblyString for Value {
    fn to_string_or_crash(&self) -> String {
        match self.as_str() {
            Some(s) => s.to_string(),
            None => panic!("error converting value {:#?} to string", self),
        }
    }
}

impl ForciblyString for PathBuf {
    fn to_string_or_crash(&self) -> String {
        match self.to_str() {
            Some(s) => s.to_string(),
            None => panic!("error converting path {:#?} to string", self),
        }
    }
}

fn recurse_to_entries(value: &Value, parent: &Path, entries: &mut Vec<Entry>) {
    let path = PathBuf::new().join(parent);

    match value {
        // Entry is a string, which corresponds to a file or directory
        Value::String(value) => entries.push(Entry {
            path: path.join(value),
            name: value.to_string(),
            hooks: None,
        }),
        // Entry is a mapping, which corresponds to a
        // file or directory with additional options
        Value::Mapping(value) => {
            let (key, value) = extract_key_value_from_single_mapping(value);

            let mut hooks = Hooks {
                run_before: None,
                run_after: None,
                depends_on: None,
                alias_as: None,
            };

            // This will have to be a sequence of options
            match value.as_sequence() {
                Some(sequence) => {
                    let mapping = match sequence.iter().next() {
                        Some(mapping) => mapping.as_mapping().expect("mapping"),
                        None => panic!("value {:#?} is not a mapping", value),
                    };

                    hooks.run_before = mapping
                        .get("run_before")
                        .map(|value| value.to_string_or_crash());

                    hooks.run_after = mapping
                        .get("run_after")
                        .map(|value| value.to_string_or_crash());

                    let key = key.to_string_or_crash();

                    entries.push(Entry {
                        name: key.to_string(),
                        path: path.join(key),
                        hooks: Some(hooks),
                    })
                }
                None => panic!("value {:#?} is not a sequence", value),
            }
        }
        // Entry is a sequence, which means is has multiple string
        // or mapping sequences, so we need to recurse downwards
        Value::Sequence(value) => {
            for entry in value {
                recurse_to_entries(entry, Path::new(parent), entries);
            }
        }
        _ => {}
    }
}

const ACCEPTABLE_HOME_ALIASES: [&str; 2] = ["home", "~"];

fn recurse_over_mapping(mapping: &Mapping, entries: &mut Vec<Entry>) {
    mapping.iter().for_each(|(key, value)| {
        let parent = get_home_dir().join({
            let mut key = key.to_string_or_crash();
            if ACCEPTABLE_HOME_ALIASES.contains(&key.to_lowercase().as_str()) {
                key = String::from("");
            }

            key
        });

        recurse_to_entries(value, &parent, entries)
    });
}

pub fn to_vec(mapping: &Mapping) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    recurse_over_mapping(mapping, &mut entries);

    entries
}
