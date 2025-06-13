# Dependency Management

Pollen automatically handles dependencies between configuration files:

```yaml
".profile":
  - alias_as: "shell-environment"

".zshrc":
  - alias_as: "zsh.config"
  - depends_on: "shell-environment"

".config":
  - "zsh/plugins": # ~/.config/zsh/plugins
      - alias_as: "zsh.plugins"
      - depends_on: "zsh.config"
      - run_after: "source ~/.zshrc"
```
