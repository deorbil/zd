<!-- markdownlint-disable MD013 -->

# Installation

## Building from Source

### Prerequisites

Install the following tools:

- [git](https://git-scm.com/)
- [rust](https://www.rust-lang.org/)

Install the following dependencies:

- [fzf](https://junegunn.github.io/fzf/)

### Building

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
