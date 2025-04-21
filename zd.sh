[[ -z $ZD_RC ]] && ZD_RC="$HOME/.zdrc.sh"

# Ask user to select a directory from user defined list
zd() {
  [[ -f $ZD_RC ]] || return

  local dirs
  # shellcheck disable=SC1090
  dirs=$(source "${ZD_RC}")

  echo "$dirs" | fzf --print-query | tail -1
}
