zd_cd() {
  local dir
  dir=$(zd)

  [[ -z $dir ]] && return

  # shellcheck disable=SC2164
  cd "$dir"
}

[[ $ZD_DISABLE_ALIAS = "true" ]] || alias z=zd_cd
