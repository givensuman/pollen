//! Parse a YAML file into usable data entries
use crate::utils;

use dirs::home_dir;
use serde_yaml::{Mapping, Sequence, Value};

use std::path::{Path, PathBuf};

fn recurse_to_entries(value: &Value, parent: &Path, entries: &mut Vec<Entry>) {
    match value {
        // Entry is a string, which corresponds to a file or directory
        Value::String(value) => handle_string(value, parent, entries),
        // Entry is a mapping, which is either an endpoint with options
        // or a point to recurse furhter down
        Value::Mapping(value) => handle_mapping(value, parent, entries),
        // Entry is a sequence, which means it has multiple `String` or `Mapping`
        // sequences, so we need to recurse downwards
        Value::Sequence(value) => handle_sequence(value, parent, entries),
        _ => {}
    };
}

fn handle_string(value: &String, parent: &Path, entries: &mut Vec<Entry>) {
    entries.push(Entry {
        path: parent.join(value),
        name: value.to_owned(),
        hooks: Hooks::new(),
        depends_on: None,
    })
}

fn extract_mapping_key(value: &Mapping) -> String {
    if value.keys().len() > 1 {
        utils::error("malformed yaml");
    }

    let key = value.keys().next().expect("malformed yaml");
    key.as_str().expect("malformed yaml").to_string()
}

const ACCEPTABLE_OPTIONS: [&str; 3] = ["run_before", "run_after", "depends_on"];

fn handle_mapping(value: &Mapping, parent: &Path, entries: &mut Vec<Entry>) {
    let key = extract_mapping_key(value);
    let mut entry = Entry {
        name: key.clone(),
        path: parent.join(&key),
        hooks: Hooks::new(),
        depends_on: None,
    };

    if !value.values().enumerate().all(|(index, value)| {
        if !value.is_sequence() {
            return false;
        }

        let is_acceptable = value.as_sequence().unwrap().iter().all(|value| {
            if !value.is_mapping() {
                return false;
            }

            ACCEPTABLE_OPTIONS.contains(&extract_mapping_key(value.as_mapping().unwrap()).as_str())
        });

        if !is_acceptable && index != 0 {
            utils::warning("cannot add options to an entry unless its an endpoint");
        }

        is_acceptable
    }) {
        println!("value {:#?} did not pass muster", value);
        handle_sequence(
            &value.values().cloned().collect::<Vec<Value>>(),
            &parent.join(key),
            entries,
        )
    }
    // All options are valid, this is an endpoint
    else {
        let value = value.values().next().expect("malformed yaml");

        entry.hooks.run_before = value
            .get("run_before")
            .and_then(|v| v.as_str().map(String::from));
        entry.hooks.run_after = value
            .get("run_after")
            .and_then(|v| v.as_str().map(String::from));
        entry.depends_on = value
            .get("depends_on")
            .and_then(|v| v.as_str().map(String::from));

        entries.push(entry);
    }
}

fn handle_sequence(value: &[Value], parent: &Path, entries: &mut Vec<Entry>) {
    value
        .iter()
        .for_each(|value| recurse_to_entries(value, parent, entries));
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

    println!("{:?}", entries);

    entries
}

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
}

impl Hooks {
    fn new() -> Hooks {
        Hooks {
            run_before: None,
            run_after: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub hooks: Hooks,
    pub depends_on: Option<String>,
}

pub trait ForciblyString {
    /// Force a type into a `String` or `panic!`
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
