//! Module for reading and parsing the `track.yaml` file
//! part of the `pollen` workflow

use serde_yaml::Mapping;
use subprocess::Exec;

use std::{
    fs::File,
    io::{Error, Read},
    ops::Deref,
    path::{Path, PathBuf},
};

// An entry option of either run_before or run_after
struct YamlEntryHook {
    timing: YamlEntryHookTiming,
    commands: Vec<String>,
}

enum YamlEntryHookTiming {
    Before,
    After,
}

impl YamlEntryHook {
    fn execute(&self) {
        for command in &self.commands {
            Exec::shell(command);
        }
    }
}

impl YamlEntryHook {
    fn is_before(&self) -> bool {
        matches!(self.timing, YamlEntryHookTiming::Before)
    }
    fn is_after(&self) -> bool {
        matches!(self.timing, YamlEntryHookTiming::After)
    }
}

// An entry in the `track.yaml` file
pub struct YamlEntry {
    pub name: String,
    pub path: PathBuf,
    pub hooks: Option<Vec<YamlEntryHook>>,
}

/// Represents the state of the `track.yaml` file read by `pollen`
pub struct Yaml(Vec<YamlEntry>);

impl Yaml {
    /// Instantiate a new `Yaml` object with optional provided entries
    fn new(entries: Option<Vec<YamlEntry>>) -> Self {
        if let Some(entries) = entries {
            Yaml(entries)
        } else {
            Yaml(Vec::new())
        }
    }
}

impl Deref for Yaml {
    type Target = Vec<YamlEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn from_file(path: &Path) -> Result<Yaml, Error> {
    let mut content = String::new();
    let error: Error;

    match File::open(path) {
        Ok(file) => file.read_to_string(&mut content)?,
        Err(e) => error = e,
    };

    let mapping = serde_yaml::from_str::<Mapping>(&content).expect("error parsing yaml file");

    Yaml::new(None)
}
