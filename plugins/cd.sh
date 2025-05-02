_zd_plugins_cd() {
  local dir
  dir=$(_zd_pick)

  [[ ! -d $dir ]] && return

  # shellcheck disable=SC2164
  cd "$dir"
}

[[ $ZD_PLUGINS_DISABLE_ALIAS = "true" ]] || alias z=_zd_plugins_cd
