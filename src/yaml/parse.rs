use dirs::home_dir;
use serde_yaml::{Mapping, Value};

use std::path::{Path, PathBuf};

fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(path) => path,
        None => panic!("unable to determine home directory"),
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hooks {
    run_before: Option<String>,
    run_after: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub name: String,
    path: String,
    hooks: Option<Hooks>,
}

/// Takes the key-value pair from a mapping with one entry
/// e.g. "run_before": "blah blah blah"
/// returns ("run_before", "blah blah blah")
fn extract_key_value_from_single_mapping(mapping: &Mapping) -> (&Value, &Value) {
    match mapping.iter().next() {
        Some(value) => value,
        None => panic!(
            "failed to extract key-value pair from mapping: {:#?}",
            mapping
        ),
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

fn recurse_to_entries(value: &Value, parent: &PathBuf, entries: &mut Vec<Entry>) {
    match value {
        Value::String(value) => entries.push(Entry {
            name: value.as_str().to_string(),
            path: get_home_dir().join(value).to_string_or_crash(),
            hooks: None,
        }),
        Value::Mapping(value) => {
            let (key, value) = extract_key_value_from_single_mapping(value);
            println!("KEY: {:#?}", key);
            println!("VALUE: {:#?}", value);

            let mut hooks = Hooks {
                run_before: None,
                run_after: None,
            };

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
                        path: get_home_dir().join(key).to_string_or_crash(),
                        hooks: Some(hooks),
                    })
                }
                None => panic!("value {:#?} is not a sequence", value),
            }
        }
        Value::Sequence(value) => {
            for entry in value {
                recurse_to_entries(entry, &Path::new(parent).to_path_buf(), entries);
            }
        }
        _ => {}
    }
}

fn recurse_over_mapping(mapping: &Mapping, entries: &mut Vec<Entry>) {
    mapping.iter().for_each(|(key, value)| {
        let parent = get_home_dir().join(key.to_string_or_crash());

        recurse_to_entries(value, &parent, entries)
    });
}

pub fn to_vec(mapping: &Mapping) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    recurse_over_mapping(mapping, &mut entries);

    println!("{:#?}", entries);

    entries
}
