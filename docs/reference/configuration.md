---
outline: deep
---

<!-- markdownlint-disable MD013 -->

# Configuration

## Environment Variables

### `ZD_DIR`

The path to the directory where zd stores its data. This includes plugins.

Defaults to `$HOME/.zd`.

### `ZD_PICKER`

The directory picker program.

Defaults to `fzf`.

### `ZD_PICKER_ARGS`

The arguments passed to the directory picker program.

### `ZD_RC`

The path to the zd configuration file. This file can be either a shell script or an executable. This file should output directories separated by new line to be displayed by the directory picker program.

Defaults to `$HOME/.zdrc`.

::: info

See [Getting Started](/guide/getting-started#configuring) for more details.

:::
