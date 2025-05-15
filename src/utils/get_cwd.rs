use std::env;
use std::io;
use std::path::PathBuf;

/// Gets the current working directory
/// the application is running in
pub fn get_cwd() -> io::Result<PathBuf> {
    env::current_dir()
}
