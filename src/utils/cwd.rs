use std::env;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

/// Utilities for getting and working with
/// the current working directory.
#[derive(Debug, Clone)]
pub struct Cwd(PathBuf);

impl Cwd {
    /// Get the CWD
    pub fn get() -> Self {
        Cwd(env::current_dir().expect("unable to get current directory"))
    }
    /// Join the CWD with a path
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.0.join(path)
    }
}

impl Deref for Cwd {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cwd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Cwd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl PartialEq for Cwd {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cwd() {
        use super::Cwd;
        let cwd = Cwd::get();
        assert!(cwd.exists());
        assert!(cwd.is_dir());
    }

    #[test]
    fn test_cwd_join() {
        use super::Cwd;
        let cwd = Cwd::get();
        let joined = cwd.join("test.txt");
        assert_eq!(joined.as_path(), cwd.0.join("test.txt"));
    }
}
