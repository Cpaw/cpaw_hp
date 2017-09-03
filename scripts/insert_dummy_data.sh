#!/usr/bin/env bash

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE:-$0}")" && pwd)"
cd "${script_dir}/.."
source "${script_dir}/../".env

echo "[+] Add dummy data"
find ./db/dummy/ -type f | while read line; do
  sqlite3 "${DATABASE_URL}" < "${line}"
done
