![logo](/assets/pollen.png)

🐝 a friendly dotfile manager

Pollen is a modern, Rust-based configuration file manager that helps you organize, backup, and synchronize your dotfiles across systems. It provides intelligent dependency tracking, Git integration, and a clean CLI interface for managing your configuration files.

### Installation

```bash
# Clone and build from source
git clone https://github.com/givensuman/pollen
cd pollen
cargo install --path .
```

### Initialize Pollen

```bash
# Create Pollen directories and default configuration
pollen init

# View current configuration
pollen config
```

### Basic Operations

```bash
# Gather dotfiles from system into Pollen
pollen gather

# Scatter dotfiles from Pollen to system
pollen scatter

# List all managed entries
pollen list

# Validate configuration
pollen validate

# Undo last operation
pollen undo
```

### Commands

| Command    | Description                             | Examples                |
| ---------- | --------------------------------------- | ----------------------- |
| `init`     | Initialize Pollen configuration         | `pollen init`           |
| `parse`    | Parse and display configuration entries | `pollen parse`          |
| `list`     | List all entries with aliases           | `pollen list --paths`   |
| `validate` | Validate configuration file             | `pollen validate`       |
| `gather`   | Copy files from system to Pollen        | `pollen gather zsh vim` |
| `scatter`  | Copy files from Pollen to system        | `pollen scatter tmux`   |
| `undo`     | Undo the last operation                 | `pollen undo`           |
| `git`      | Git operations for files directory      | `pollen git status`     |
| `config`   | Display current configuration           | `pollen config`         |

### Configuration File Structure

Your `track.yaml` file defines which configuration files Pollen manages:

```yaml
qualified/path/from/home:
  - subpaths/are/fine:
    - turtles/all/the/way:
        - down
            - alias_as: "baz"
  - config_folder:
      - alias_as: "something else"
      - run_before: "echo 'foo'"
      - run_after: "echo 'bar'"
      - depends_on: "baz"
```

**Flexible Field Formats:**

The `depends_on`, `run_before`, and `run_after` fields support both single strings and sequences:

```yaml
# Single values
depends_on: "some-dependency"
run_before: "echo 'single command'"
run_after: "systemctl reload service"

# Multiple values
depends_on: ["dep1", "dep2", "dep3"]
run_before:
  - "echo 'first command'"
  - "mkdir -p ~/.backup"
run_after:
  - "source ~/.config"
  - "echo 'done'"
```

### Directory Structure

Pollen organizes files in a clean directory structure:

```
~/.config/pollen/          # Main configuration directory
├── pollen.yaml           # Pollen settings
├── track.yaml            # Your dotfile definitions
├── files/                # Managed configuration files
│   ├── zshrc
│   ├── vimrc
│   └── tmux.conf
├── cache/                # Backup files and temporary data
└── operations.json       # Operation history for undo
```

### Environment Variables

- `POLLEN_DIR`: Override default configuration directory
- `POLLEN_UNDO_LIMIT`: Maximum number of operations to keep for undo (default: 10)

### Git Integration

Pollen includes powerful Git integration for version control:

```bash
# Initialize Git repository in files directory
pollen git init

# Check Git status
pollen git status

# Commit changes
pollen git commit "Updated configurations"

# Any Git command works
pollen git push origin main
pollen git pull
pollen git log --oneline
```

Enable auto-commit in `pollen.yaml`:

```yaml
auto_commit: true
auto_commit_message: "Pollen auto-commit"
```

### Dependency Management

Pollen automatically handles dependencies between configuration files:

```yaml
".profile":
  - alias_as: "shell-environment"

".zshrc":
  - alias_as: "zsh.config"
  - depends_on: "shell-environment" # Single dependency

".config":
  - "zsh/plugins": # ~/.config/zsh/plugins
      - alias_as: "zsh.plugins"
      - depends_on: # Multiple dependencies
          - "zsh.config"
          - "shell-environment"
      - run_after: "source ~/.zshrc"
```

### Pre/Post Processing Commands

Execute commands before or after putting them on your system:

```yaml
entries:
  - name: "nginx-config"
    path: "/etc/nginx/nginx.conf"
    run_before: "sudo nginx -t" # Single command
    run_after: "sudo systemctl reload nginx"

  - name: "complex-setup"
    path: "~/.myconfig"
    run_before: # Multiple commands
      - "echo 'Starting setup'"
      - "mkdir -p ~/.backup"
    run_after:
      - "source ~/.myconfig"
      - "echo 'Setup complete'"
```

### Selective Operations

Work with specific entries using names or aliases:

```bash
# Gather only specific entries
pollen gather zsh vim tmux

# Scatter using aliases
pollen scatter shell editor
```

### Configuration Options

Customize Pollen behavior in `pollen.yaml`:

```yaml
verbose: true # Enable verbose output
cache_expiration: 86400 # Cache expiration in seconds
max_cache_entries: 100 # Maximum cached backups
auto_commit: true # Auto-commit to Git
auto_commit_message: "Auto-sync" # Default commit message
default_track_file: "track.yaml" # Default configuration file
```

## License

MIT license. Go nuts.
