<img src="./assets/logo.png" alt="Logo"/>

<div align="center">
  <h3>🐝 a friendly dotfile manager</h3>
</div>

#### 🌻 What

`pollen` creates a pretty straightforward, but opinionated, way of managing dotfiles for your system. It leverages the fact that most people keep their dotfiles in version control as top-level folders, e.g.:

```txt
some-repo
  - config-one
    └ whole-buncha-configs
  - config-two
```

All it asks is that you place among this structure a `track.yaml` file which specifies what goes where, and provides some hooks on which to operate. Here's what mine looks like:

```yaml
%YAML 1.2
---
home: # This gets evaluated to $HOME
  - .tool-versions # asdf
  - .asdf:
      - run_after: "~/.asdf/bin/asdf install"

.config:
  - bat:
      - run_after: "bat cache --build"
  - eza
  - cosmic
  - lazygit
  - fish:
      - run_after: "source ~/.config/fish/config.fish && fisher update"
  - ghostty
  - nvim:
      - run_before: "cp -r ~/.config/nvim ~/.config/nvim.bak"
Pictures:
  - Wallpapers
```

Then you can update your repository with `pollen gather`, or update your system with `pollen scatter`. Sync up with `git pull/push` and you're all set.

#### 🌻 How

| command | usage           | alias(es) | description                                      |
| ------- | --------------- | --------- | ------------------------------------------------ |
| scatter | scatter \<args> | s         | copies files from pollen's root to the system    |
| gather  | gather \<args>  | g         | copies files from the system to pollen's root    |
| status  | status \<args>  |           | diffs files between pollen's root and the system |

> If no `<args>` are specified, every entry in your `track.yaml` will be processed. Otherwise, you can pass any number of entries by name

#### 🌻 Why

There are some tools that do similar things, most notably [chezmoi](https://www.chezmoi.io/), but I found this workflow the one I liked and after dealing with increasingly complicated bash scripts I decided to write in in Rust. That's all to say, this is primarily a tool for me, but one anyone is welcome to try out.

#### License

[Same old, same old](https://mit-license.org/). Do whatever you want with this.
