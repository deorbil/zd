<!-- markdownlint-disable MD013 MD036 -->

# Getting Started

## Installation

:::: details Arch Linux

::: code-group

```sh [paru]
mkdir -p ~/.cache/paru/clone/zd-git
cd ~/.cache/paru/clone/zd-git
curl -O https://raw.githubusercontent.com/deorbil/aur/master/zd-git/PKGBUILD
paru -Bi .
```

:::

::::

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

## Configuring

:::: details Bash {open}

Add the following to your `~/.bashrc`:

```sh
eval "$(zd init bash)"
```

Create a new file `~/.zdrc` with the following content to add your first directory:

```bash
#!/usr/bin/env bash
echo "$HOME"
```

::: tip

You can add more directories using `echo` or any other command that outputs directory paths. For example, use `find` to add multiple directories:

```bash
find "$HOME/source" -maxdepth 1 -type d
```

:::

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
