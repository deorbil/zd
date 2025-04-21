# Get the list of directories
_zd_dirs() {
  local rc=${ZD_RC:-$HOME/.zdrc.sh}
  # shellcheck disable=SC1090
  [[ -f $rc ]] && source "$rc"
}

# Ask user to select a directory from stdin
_zd_picker() {
  eval "${ZD_PICKER:-fzf --print-query}" "${ZD_PICKER_ARGS}" | tail -1
}

# Source a plugin
_zd_plugin() {
  local plugin="$ZD_DIR/plugins/$1.sh"
  # shellcheck disable=SC1090
  [[ -f $plugin ]] && source "$plugin"
}

# Source enabled plugins
_zd_plugins() {
  local plugin
  for plugin in "${ZD_PLUGINS[@]}"; do
    _zd_plugin "$plugin"
  done
}

# Ask user to select a directory from user defined list
zd() {
  _zd_dirs | _zd_picker
}

_zd_plugins
