__zd_picker() {
  eval "${ZD_PICKER:-fzf}" "${ZD_PICKER_ARGS}"
}

__zd() {
  if [[ $# -eq 0 ]]; then
    local rc="${ZD_RC:-$HOME/.zdrc}"
    if [[ -x "$rc" ]]; then
      "$rc" | __zd_picker
    elif [[ -f "$rc" ]]; then
      source "$rc" | __zd_picker
    else
      echo "$HOME"
    fi
  elif [[ $# -eq 1 ]]; then
    echo "$1"
  elif [[ $# -eq 2 && "$1" == "--" ]]; then
    echo "$2"
  fi
}

__zd_plugin_load() {
  local dir="$1"
  if [[ -d "$dir" ]]; then
    local name="${dir##*/}"
    if [[ -f "$dir/$name.bash" ]]; then
      source "$dir/$name.bash"
    fi
  fi
}

__zd_plugin_load_all() {
  local dir="${ZD_DIR:-$HOME/.zd}/plugins"
  if [[ -d "$dir" ]]; then
    for plugin in "$dir"/*; do
      __zd_plugin_load "$plugin"
    done
  fi
}

__zd_plugin_load_all
