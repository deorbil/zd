---
outline: deep
---

# Commands

## `init`

```sh
zd init <SHELL>
```

Print the initialization script.

::: warning

This command should be added to your shell configuration file (such as `~/.bashrc`), see [Getting Started](/guide/getting-started#setup) for more details.

:::

## `plugin`

### `install`

```sh
zd plugin install <PLUGINS>...
```

Install plugins.

### `list`

```sh
zd plugin list
```

List installed plugins.

### `uninstall`

```sh
zd plugin uninstall <PLUGINS>...
```

Uninstall plugins.

### update

```sh
zd plugin update [PLUGINS]...
```

Update plugins.

If no plugins are specified, all plugins will be updated.
