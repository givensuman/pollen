use std::path::PathBuf;

/// Represents a configuration entry (file or directory) to be managed by Pollen
#[derive(Debug, Clone)]
pub struct Entry {
    /// Name of the entry
    pub name: String,
    /// Full path to the entry
    pub path: PathBuf,
    /// Command to run before processing this entry
    pub run_before: Option<String>,
    /// Command to run after processing this entry
    pub run_after: Option<String>,
    /// Names of entries this entry depends on
    pub depends_on: Vec<String>,
    /// Alias for this entry (optional shorter name)
    pub alias_as: Option<String>,
}

/// Arguments for creating a new Entry
pub struct EntryArgument {
    pub name: String,
    pub path: PathBuf,
}

impl Entry {
    /// Create a new Entry with the given arguments
    pub fn new(entry: EntryArgument) -> Self {
        Entry {
            name: entry.name,
            path: entry.path,
            run_before: None,
            run_after: None,
            depends_on: Vec::new(),
            alias_as: None,
        }
    }

    /// Set the command to run before processing this entry
    pub fn with_run_before(mut self, command: String) -> Self {
        self.run_before = Some(command);
        self
    }

    /// Set the command to run after processing this entry
    pub fn with_run_after(mut self, command: String) -> Self {
        self.run_after = Some(command);
        self
    }

    /// Add a dependency for this entry
    pub fn add_dependency(mut self, dependency: String) -> Self {
        self.depends_on.push(dependency);
        self
    }

    /// Set multiple dependencies for this entry
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.depends_on = dependencies;
        self
    }

    /// Set an alias for this entry
    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias_as = Some(alias);
        self
    }

    /// Get the display name (alias if available, otherwise the entry name)
    pub fn get_display_name(&self) -> &str {
        self.alias_as.as_ref().unwrap_or(&self.name)
    }

    /// Check if this entry matches the given name (either by name or alias)
    pub fn matches_name(&self, name: &str) -> bool {
        self.name == name || self.alias_as.as_ref() == Some(&name.to_string())
    }
}
