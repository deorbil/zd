# Ask user to select a directory from stdin
_zd_picker() {
  fzf --print-query | tail -1
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
  local rc=${ZD_RC:-$HOME/.zdrc.sh}

  [[ -f $rc ]] || return

  local dirs
  # shellcheck disable=SC1090
  dirs=$(source "$rc")

  _zd_picker <<<"$dirs"
}

_zd_plugins
