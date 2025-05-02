<!-- markdownlint-disable MD013 -->

# zd

zd is a fuzzy directory selector, that allows you to quickly pick a directory from scriptable user-defined list of directories. It then integrates with other commands such as `cd` and `tmux` through plugins to allow quick navigation.

## How It Works

Each time zd is run, it will source the `ZD_RC` file, the rc file will output list of directories to stdout to be displayed by `ZD_PICKER`. When user selects a directory, the picker will output the selected directory which will be passed to zd and then zd plugins.

## Quick Start

- **Install dependencies**

  By default zd uses [`fzf`][fzf] as its picker. It can be installed from [here][fzf-install].

- **Install zd**

  Clone this repository to `ZD_DIR` directory.

  ```bash
  git clone https://github.com/deorbil/zd.git ~/.zd
  ```

- **Setup zd on your shell**

  Copy the following code to your shell rc file, such as `~/.bashrc`

  ```bash
  export ZD_DIR="$HOME/.zd"

  # Override the default picker
  # ZD_PICKER="fzf --print-query"

  # Override the picker args
  # ZD_PICKER_ARGS=""

  # Define which plugins to enable
  ZD_PLUGINS=(cd)

  # Uncomment the following line to disable default aliases
  # ZD_PLUGINS_DISABLE_ALIAS="true"

  # Override the default rc file location
  # This is where you will define your directories
  # ZD_RC="$HOME/.zdrc.sh"

  source "$ZD_DIR/zd.sh"
  ```

- **Setup zd directories**

  Create `ZD_RC` file which is `~/.zdrc.sh` by default and put all directories you want to display in zd. You can use combination of `echo`, `find` or `fd` and any other program that outputs directories.

  Here is a simple example of that:

  ```bash
  echo "$HOME"
  echo "$HOME/.dotfiles"
  # add all my projects
  fd --max-depth 1 --type dir . "$HOME/source/repos"
  ```

## Usage

On its own zd does almost nothing, it prompts user to select a directory and then output it to stdout.

```bash
zd
```

But when the output is passed to other program is where it shines! That's where plugins come into play.

### Plugins

By default, zd comes with a plugin system that allows you to add custom commands to be executed after selecting a directory.

There are also predefined plugins, but most of them are disabled by default, except for `cd`. All predefined plugins have `_zd_plugins_` prefix, and their respective aliases that can be disabled with `ZD_PLUGINS_DISABLE_ALIAS="true"`.

- `cd`

  Change current working directory to the selected directory.

  ```bash
  z
  ```

- `tmux`

  It tries to create a new tmux session with the selected directory if it doesn't exist. Then it attaches to the associated session.

  ```bash
  zt
  ```

- More will be added in the future.

## Configuration

- `ZD_DIR` (required)

  This is the directory where zd is installed.

- `ZD_PICKER`

  Set the command to be used as picker. By default it is set to `fzf`.

- `ZD_PICKER_ARGS`

  Add additional arguments to the picker command. This only works if `ZD_PICKER` is not set!

- `ZD_PLUGINS`

  An array of plugins to be enabled.

- `ZD_PLUGINS_DISABLE_ALIAS`

  Set to `true` to disable default aliases provided for plugins.

- `ZD_RC`

  The rc file location which will contains the directories to be displayed by the picker. By default it is set to `~/.zsrc.sh`

## Credits

zd is inspired from [zoxide] and [tmux-sessionizer].

[fzf]: https://github.com/junegunn/fzf
[fzf-install]: https://github.com/junegunn/fzf#installation
[tmux-sessionizer]: https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer
[zoxide]: https://github.com/ajeetdsouza/zoxide
