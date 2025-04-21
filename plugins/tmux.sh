zd_tmux() {
  local dir
  dir=$(zd)

  [[ ! -d $dir ]] && return

  local name
  name=$(basename "$dir" | tr . _)

  # Create session if it doesn't exist
  if ! tmux has-session -t "$name" 2>/dev/null; then
    tmux new-session -s "$name" -c "$dir" -d
  fi

  # Attach or switch to session if already in tmux to prevent nesting
  if [[ -n $TMUX ]]; then
    tmux switch-client -t "$name"
  else
    tmux attach-session -t "$name"
  fi
}

[[ $ZD_DISABLE_ALIAS = "true" ]] || alias zt=zd_tmux
