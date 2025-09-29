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
  [[ -d "$dir" ]] || return

  local name="${dir##*/}"
  [[ -f "$dir/$name.zsh" ]] && source "$dir/$name.zsh"
}

__zd_plugin_load_all() {
  local dir="${ZD_DIR:-$HOME/.zd}/plugins"
  for plugin in "$dir"/*(N); do
    __zd_plugin_load "$plugin"
  done
}

__zd_plugin_load_all
