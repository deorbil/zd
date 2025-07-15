<!-- markdownlint-disable MD013 MD036 -->

# Getting Started

## Installation

::: details Arch Linux

```sh
git clone https://github.com/deorbil-aur/zd-git.git
cd zd-git
makepkg -si
```

:::

:::: details Building from Source {open}

**Prerequisites**

Install the following tools:

- [git](https://git-scm.com/)
- [rust](https://www.rust-lang.org/)

Install the following dependencies:

- [fzf](https://junegunn.github.io/fzf/)

**Building**

This will clone the repository, build the project, and create an executable in the `target/release` directory.

```sh
git clone https://github.com/deorbil/zd.git
cd zd
cargo build --release --locked
```

::: tip

Move the executable to a directory in your `PATH` (such as `/usr/local/bin` or `~/.local/bin`):

```sh
sudo cp target/release/zd /usr/local/bin/
cp target/release/zd ~/.local/bin/
```

:::

::::

## Setup

:::: details Bash {open}

Add the following to your `~/.bashrc`:

::: code-group

```bash [.bashrc]
eval "$(zd init bash)"
```

:::

::::

## Adding Directories

By default, zd will use your default shell as the script interpreter.

::::: details Bash {open}

Create a new file `~/.zdrc` with the following content to add your first directory:

::: code-group

```bash [.zdrc]
#!/usr/bin/env bash
echo "$HOME"
```

:::

:::: tip

You can add more directories using `echo` or any other command that outputs directory paths. For example, use `find` to add multiple directories:

::: code-group

```bash [.zdrc]
#!/usr/bin/env bash
echo "$HOME"
find "$HOME/source" -maxdepth 1 -type d # [!code ++]
```

:::

::::

:::::

Alternatively, replace `~/.zdrc` with an executable. This can be either a binary or a shebang script.

:::: details Binary

Move the binary to `~/.zdrc`, then make it executable:

```sh
chmod +x ~/.zdrc
```

::::

:::: details Lua

Add the following to your `~/.zdrc`:

::: code-group

```lua [.zdrc]
#!/usr/bin/env lua
print(os.getenv("HOME"))
```

:::

Make `~/.zdrc` executable:

```sh
chmod +x ~/.zdrc
```

::::

:::: details Node.js

Add the following to your `~/.zdrc`:

::: code-group

```js [.zdrc]
#!/usr/bin/env node
const os = require("os");
console.log(os.homedir());
```

Make `~/.zdrc` executable:

:::

```sh
chmod +x ~/.zdrc
```

::::

:::: details Python

Add the following to your `~/.zdrc`:

::: code-group

```py [.zdrc]
#!/usr/bin/env python3
import os
print(os.path.expanduser("~"))
```

:::

Make `~/.zdrc` executable:

```sh
chmod +x ~/.zdrc
```

::::

## Installing a Plugin

Install the [zd-cd](https://github.com/deorbil/zd-cd) plugin:

```sh
zd plugin add cd https://github.com/deorbil/zd-cd.git
```

Then reload your shell and run the following command:

```sh
z
```
