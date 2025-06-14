<!-- markdownlint-disable MD013 -->

# Getting Started

## Configure zd

::: details Bash {open}

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

## Install a Plugin

Install the `zd-cd` plugin to start using `zd`.

```sh
zd plugin add cd https://github.com/deorbil/zd-cd.git
```

Then reload your shell and run the following command:

```sh
z
```
