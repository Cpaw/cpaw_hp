#!/usr/bin/env bash

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE:-$0}")" && pwd)"
cd "${script_dir}/.."

# install diesel
if ! type diesel >/dev/null 2>&1 ; then
  echo '[+] Install diesel_cli'
  cargo install diesel_cli --no-default-features --features "sqlite"
fi

echo '[+] Migration'
diesel migration run
