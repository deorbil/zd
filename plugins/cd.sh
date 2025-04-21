zd_cd() {
  local dir
  dir=$(zd)

  [[ ! -d $dir ]] && return

  # shellcheck disable=SC2164
  cd "$dir"
}

[[ $ZD_PLUGINS_DISABLE_ALIAS = "true" ]] || alias z=zd_cd
