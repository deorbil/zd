# Ask user to select a directory from user defined list
zd() {
  local rc=${ZD_RC:-$HOME/.zdrc.sh}

  [[ -f $rc ]] || return

  local dirs
  # shellcheck disable=SC1090
  dirs=$(source "$rc")

  echo "$dirs" | fzf --print-query | tail -1
}

# Source enabled plugins
for plugin in "${ZD_PLUGINS[@]}"; do
  if [[ -f $ZD_DIR/plugins/$plugin.sh ]]; then
    # shellcheck disable=SC1090
    source "$ZD_DIR/plugins/$plugin.sh"
  fi
done
unset plugin
