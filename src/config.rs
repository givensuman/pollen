use crate::{
    error::PollenError,
    entry::{Entry, EntryArgument},
    yaml_ext::{CanForceIntoString, Endpoint},
};
use serde_yaml::{Mapping, Value};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// Configuration parser for Pollen YAML files
pub struct ConfigParser {
    home_dir: PathBuf,
}

impl ConfigParser {
    /// Create a new ConfigParser
    pub fn new() -> Result<Self, PollenError> {
        let home_dir = get_home_dir()?;
        Ok(ConfigParser { home_dir })
    }

    /// Parse a configuration file and return a list of entries
    pub fn parse_file(&self, file_path: &str) -> Result<Vec<Entry>, PollenError> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(PollenError::Io)?;

        self.parse_content(&content)
    }

    /// Parse YAML content and return a list of entries
    pub fn parse_content(&self, content: &str) -> Result<Vec<Entry>, PollenError> {
        let values: Mapping = serde_yaml::from_str(content)?;
        let mut entries = Vec::new();
        self.recurse_over_mapping(&mut entries, &values, &self.home_dir)?;
        
        // Sort entries based on dependencies to ensure proper ordering
        self.topological_sort(entries)
    }

    /// Recursively process a YAML mapping
    fn recurse_over_mapping(
        &self,
        entries: &mut Vec<Entry>,
        mapping: &Mapping,
        parent_path: &Path,
    ) -> Result<(), PollenError> {
        for (key, value) in mapping.iter() {
            match value {
                Value::String(string) => {
                    self.handle_string(entries, string.to_string(), parent_path);
                }
                Value::Sequence(sequence) => {
                    let key_str = key.force_into_string()?;
                    self.handle_sequence(entries, sequence, &parent_path.join(key_str))?;
                }
                _ => {
                    let key_str = key.force_into_string()
                        .unwrap_or_else(|_| "unknown".to_string());
                    return Err(PollenError::InvalidMapping(format!(
                        "Unsupported value type in mapping at key '{}'", 
                        key_str
                    )));
                }
            }
        }
        Ok(())
    }

    /// Handle a sequence of values in the YAML
    fn handle_sequence(
        &self,
        entries: &mut Vec<Entry>,
        sequence: &[Value],
        parent_path: &Path,
    ) -> Result<(), PollenError> {
        for value in sequence.iter() {
            match value {
                Value::String(string) => {
                    self.handle_string(entries, string.to_string(), parent_path);
                }
                Value::Mapping(mapping) => {
                    if mapping.is_an_endpoint()? {
                        self.handle_endpoint(entries, mapping, parent_path)?;
                    } else {
                        self.recurse_over_mapping(entries, mapping, parent_path)?;
                    }
                }
                _ => {
                    return Err(PollenError::InvalidMapping(
                        "Sequence contains unsupported value type".to_string()
                    ));
                }
            }
        }
        Ok(())
    }

    /// Handle an endpoint configuration
    fn handle_endpoint(
        &self,
        entries: &mut Vec<Entry>,
        endpoint: &Mapping,
        parent_path: &Path,
    ) -> Result<(), PollenError> {
        let key = endpoint.get_key()?;
        let mut entry = Entry::new(EntryArgument {
            name: key.clone(),
            path: parent_path.join(&key),
        });

        let sequence = endpoint
            .get_value()?
            .as_sequence()
            .ok_or_else(|| PollenError::InvalidEndpoint(
                "Endpoint value is not a sequence".to_string()
            ))?;

        for mapping in sequence.iter() {
            let mapping = mapping
                .as_mapping()
                .ok_or_else(|| PollenError::InvalidEndpoint(
                    "Option in sequence is not a mapping".to_string()
                ))?;
            
            let key = mapping.get_key()?;
            let value = mapping.get_value()?.force_into_string()?;

            match key.as_str() {
                "run_before" => entry.run_before = Some(value),
                "run_after" => entry.run_after = Some(value),
                "depends_on" => {
                    // depends_on can be a single string or a list of strings
                    let dependencies = self.parse_dependencies(mapping.get_value()?)?;
                    entry.depends_on = dependencies;
                }
                "alias_as" => {
                    let alias = value;
                    entry.alias_as = Some(alias);
                }
                _ => {
                    return Err(PollenError::InvalidOption(format!(
                        "Unknown option: {}", key
                    )));
                }
            }
        }

        entries.push(entry);
        Ok(())
    }

    /// Handle a simple string entry
    fn handle_string(&self, entries: &mut Vec<Entry>, string: String, parent_path: &Path) {
        entries.push(Entry::new(EntryArgument {
            name: string.clone(),
            path: parent_path.join(string),
        }));
    }

    /// Parse dependencies from YAML value (can be string or array)
    fn parse_dependencies(&self, value: &Value) -> Result<Vec<String>, PollenError> {
        match value {
            Value::String(s) => Ok(vec![s.clone()]),
            Value::Sequence(seq) => {
                let mut deps = Vec::new();
                for item in seq {
                    match item {
                        Value::String(s) => deps.push(s.clone()),
                        _ => return Err(PollenError::InvalidEndpoint(
                            "Dependencies must be strings".to_string()
                        )),
                    }
                }
                Ok(deps)
            }
            _ => Err(PollenError::InvalidEndpoint(
                "Dependencies must be a string or array of strings".to_string()
            )),
        }
    }

    /// Find an entry by name or alias
    pub fn find_entry_by_name<'a>(&self, entries: &'a [Entry], name: &str) -> Option<&'a Entry> {
        entries.iter().find(|entry| entry.matches_name(name))
    }

    /// List all aliases and their corresponding paths
    pub fn list_aliases<'a>(&self, entries: &'a [Entry]) -> Vec<(&'a str, String)> {
        entries
            .iter()
            .filter_map(|entry| {
                entry.alias_as.as_ref().map(|alias| (alias.as_str(), entry.path.display().to_string()))
            })
            .collect()
    }

    /// Validate that aliases don't conflict with paths or each other
    pub fn validate_aliases(&self, entries: &[Entry]) -> Result<(), PollenError> {
        let mut aliases = HashSet::new();
        let mut names = HashSet::new();
        
        // Collect all entry names
        for entry in entries {
            names.insert(&entry.name);
        }
        
        // Check for duplicate aliases and alias conflicts with names
        for entry in entries {
            if let Some(alias) = &entry.alias_as {
                // Check if alias conflicts with an existing entry name
                if names.contains(alias) {
                    return Err(PollenError::InvalidEndpoint(format!(
                        "Alias '{}' conflicts with existing entry name", alias
                    )));
                }
                
                // Check for duplicate aliases
                if !aliases.insert(alias) {
                    return Err(PollenError::InvalidEndpoint(format!(
                        "Duplicate alias: '{}'", alias
                    )));
                }
            }
        }
        
        Ok(())
    }

    /// Resolve a dependency name (could be an alias) to the actual entry name
    fn resolve_dependency_name(&self, entries: &[Entry], dep_name: &str) -> Option<String> {
        entries
            .iter()
            .find(|entry| entry.matches_name(dep_name))
            .map(|entry| entry.name.clone())
    }

    /// Perform topological sort to order entries based on dependencies
    fn topological_sort(&self, entries: Vec<Entry>) -> Result<Vec<Entry>, PollenError> {
        // First validate aliases
        self.validate_aliases(&entries)?;

        let mut entry_map: HashMap<String, Entry> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

        // Build the dependency graph
        for entry in &entries {
            entry_map.insert(entry.name.clone(), entry.clone());
            in_degree.insert(entry.name.clone(), 0);
            adjacency_list.insert(entry.name.clone(), Vec::new());
        }

        // Calculate in-degrees and build adjacency list
        for entry in &entries {
            for dep_name in &entry.depends_on {
                // Resolve dependency name (could be an alias)
                if let Some(resolved_dep) = self.resolve_dependency_name(&entries, dep_name) {
                    *in_degree.get_mut(&entry.name).unwrap() += 1;
                    adjacency_list.get_mut(&resolved_dep).unwrap().push(entry.name.clone());
                } else {
                    return Err(PollenError::MissingDependency(format!(
                        "Entry '{}' depends on '{}' which does not exist",
                        entry.name, dep_name
                    )));
                }
            }
        }

        // Kahn's algorithm for topological sorting
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut result: Vec<Entry> = Vec::new();

        // Add all entries with no dependencies to the queue
        for (name, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(name.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            result.push(entry_map[&current].clone());

            // Reduce in-degree of dependent entries
            for dependent in &adjacency_list[&current] {
                let degree = in_degree.get_mut(dependent).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(dependent.clone());
                }
            }
        }

        // Check for circular dependencies
        if result.len() != entries.len() {
            let remaining: Vec<String> = entries
                .iter()
                .filter(|e| !result.iter().any(|r| r.name == e.name))
                .map(|e| e.name.clone())
                .collect();
            
            return Err(PollenError::CircularDependency(format!(
                "Circular dependency detected involving entries: {:?}",
                remaining
            )));
        }

        Ok(result)
    }
}

impl Default for ConfigParser {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ConfigParser")
    }
}

/// Get the user's home directory
fn get_home_dir() -> Result<PathBuf, PollenError> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(PollenError::HomeDirectoryNotSet)
}
