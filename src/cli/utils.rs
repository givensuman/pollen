use crate::PollenError;
use std::path::Path;

pub fn execute_shell_command(command: &str) -> Result<(), std::io::Error> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
        
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed with exit code: {:?}", output.status.code())
        ));
    }
    
    // Print stdout if there's any
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    // Print stderr if there's any
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), PollenError> {
    use std::fs;
    
    fs::create_dir_all(dst).map_err(PollenError::Io)?;
    
    for entry in fs::read_dir(src).map_err(PollenError::Io)? {
        let entry = entry.map_err(PollenError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(PollenError::Io)?;
        }
    }
    
    Ok(())
}
