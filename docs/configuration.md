# Configuration

## Configuration File Structure

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

## Customize Behavior

Customize Pollen behavior in `pollen.yaml`:

```yaml
verbose: true # Enable verbose output
cache_expiration: 86400 # Cache expiration in seconds
max_cache_entries: 100 # Maximum cached backups
auto_commit: true # Auto-commit to Git
auto_commit_message: "Auto-sync" # Default commit message
default_track_file: "track.yaml" # Default configuration file
```
