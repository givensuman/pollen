# Git Integration

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
