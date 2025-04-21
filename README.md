# zd

`zd` is a fuzzy directory selector, that allows you to quickly pick a directory from scriptable user-defined list of directories. It then integrates with other commands such as `cd` through plugins to allow quick navigation.

## How It Works

Each time `zd` is run, it will source the `.zdrc.sh` file in home directory, the rc file will output list of directories to `stdout` to be displayed by `fzf`. When user selects a directory, `fzf` will output the selected directory which will be passed to `zd` and then `zd` plugin.

## Quick Start

- Install [fzf](https://github.com/junegunn/fzf)

- Clone this repository to `~/.zd`

  ```bash
  git clone https://github.com/deorbil/zd.git ~/.zd
  ```

- Copy the following code to your `.bashrc` file

  ```bash
  export ZD_DIR="$HOME/.zd"

  # Define which plugins to enable
  ZD_PLUGINS=(cd)

  # Override the default rc file location
  # This is where you will define your directories
  # ZD_RC="$HOME/.zdrc.sh"

  # Uncomment the following line to disable default aliases
  # ZD_DISABLE_ALIAS="true"

  source "$ZD_DIR/zd.sh"
  ```

- Create `.zdrc.sh` file in your home directory and put all directories you want to display in `zd` with `echo` or any other command that output directories such as `find` and `fd`

  Here is a simple example of that:

  ```bash
  echo "$HOME"
  echo "$HOME/.dotfiles"
  # add all my projects
  fd --max-depth 1 --type dir . "$HOME/source/repos"
  ```

## Usage

On its own `zd` does almost nothing, it prompts user to select a directory and then output it to `stdout`.

```bash
zd
```

But when the output is passed to other program is where it shines! That's where plugins come into play.

### Plugins

By default, `zd` comes with a plugin system that allows you to add custom commands to be executed after selecting a directory.

There are also predefined plugins, but most of them are disabled by default, except for `cd`. All predefined plugins have `zd_` prefix, and their respective aliases that can be disabled with `ZD_DISABLE_ALIAS="true"`.

- `cd`

  Change current working directory to the selected directory.

  ```bash
  zd_cd
  # or alias
  z
  ```

- `tmux`

  It tries to create a new tmux session with the selected directory if it doesn't exist. Then it attaches to the associated session.

  ```bash
  zd_tmux
  # or alias
  zt
  ```

- More will be added in the future.

## Credits

`zd` is inspired from [zoxide](https://github.com/ajeetdsouza/zoxide) and [tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer).
