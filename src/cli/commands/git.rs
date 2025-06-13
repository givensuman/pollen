use crate::{PollenDirs, PollenError};
use seahorse::Context;

pub fn handle_git_command(c: &Context) -> Result<(), PollenError> {
    let dirs = PollenDirs::new()?;
    
    let subcommand = c.args.get(0).map(|s| s.as_str()).unwrap_or("status");
    
    match subcommand {
        "init" => {
            println!("Initializing Git repository in files directory...");
            match dirs.init_files_git_repo()? {
                true => {
                    println!("✓ Git repository initialized in {}", dirs.files_dir.display());
                    println!("✓ Created .gitignore file");
                    println!("\nYou can now:");
                    println!("  cd {}", dirs.files_dir.display());
                    println!("  git remote add origin <your-repo-url>");
                    println!("  pollen git commit 'Initial commit'");
                    println!("  git push -u origin main");
                }
                false => {
                    println!("Git repository already exists in {}", dirs.files_dir.display());
                }
            }
            Ok(())
        }
        
        "status" => {
            if !dirs.is_files_git_repo() {
                println!("Files directory is not a Git repository.");
                println!("Use 'pollen git init' to initialize one.");
                return Ok(());
            }
            
            println!("Git status for files directory:");
            let output = std::process::Command::new("git")
                .args(&["status", "--short"])
                .current_dir(&dirs.files_dir)
                .output()
                .map_err(PollenError::Io)?;
                
            if output.status.success() {
                let status_text = String::from_utf8_lossy(&output.stdout);
                if status_text.trim().is_empty() {
                    println!("  Working directory clean");
                } else {
                    println!("{}", status_text);
                }
            }
            Ok(())
        }
        
        "commit" => {
            let message = c.args.get(1)
                .map(|s| s.as_str())
                .unwrap_or("Update configuration files");
                
            println!("Committing changes to files directory...");
            dirs.commit_files_changes(message)?;
            println!("✓ Changes committed with message: '{}'", message);
            Ok(())
        }
        
        "auto-commit" => {
            if !dirs.is_files_git_repo() {
                println!("Files directory is not a Git repository.");
                println!("Use 'pollen git init' to initialize one.");
                return Ok(());
            }
            
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| PollenError::InvalidEndpoint("Failed to get timestamp".to_string()))?
                .as_secs();
                
            let message = format!("Auto-commit: Pollen sync at {}", timestamp);
            dirs.commit_files_changes(&message)?;
            println!("✓ Auto-committed changes");
            Ok(())
        }
        
        "help" | "--help" | "-h" => {
            println!("Pollen Git Integration");
            println!();
            println!("Pollen-specific git subcommands:");
            println!("  init        - Initialize Git repository in files directory");
            println!("  status      - Show Git status of files directory");
            println!("  commit [msg] - Commit changes with optional message");
            println!("  auto-commit - Commit with automatic timestamp message");
            println!("  help        - Show this help message");
            println!();
            println!("Any other git command will be passed through to git in the files directory:");
            println!("  pollen git push          - Run 'git push' in files directory");
            println!("  pollen git pull          - Run 'git pull' in files directory");
            println!("  pollen git log --oneline - Run 'git log --oneline' in files directory");
            println!("  pollen git remote -v     - Run 'git remote -v' in files directory");
            println!("  pollen git branch        - Run 'git branch' in files directory");
            println!();
            println!("Files directory: {}", dirs.files_dir.display());
            Ok(())
        }
        
        _ => {
            // Pass through any other git commands directly to git
            if !dirs.is_files_git_repo() {
                println!("Files directory is not a Git repository.");
                println!("Use 'pollen git init' to initialize one.");
                return Ok(());
            }
            
            // Build the git command with all arguments
            let mut git_args = vec![subcommand];
            git_args.extend(c.args.iter().skip(1).map(|s| s.as_str()));
            
            println!("Running: git {} in {}", git_args.join(" "), dirs.files_dir.display());
            
            let mut command = std::process::Command::new("git");
            command.args(&git_args).current_dir(&dirs.files_dir);
            
            // Execute the command and let it inherit stdio so output goes directly to terminal
            let status = command.status().map_err(PollenError::Io)?;
            
            if !status.success() {
                return Err(PollenError::InvalidEndpoint(
                    format!("Git command failed with exit code: {:?}", status.code())
                ));
            }
            
            Ok(())
        }
    }
}
