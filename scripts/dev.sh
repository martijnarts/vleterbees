#!/usr/bin/env bash

# Run the local development environment

# Enable strict mode for Bash
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

# Path to the script and the project root
script_path=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
project_path=$(cd "${script_path}" && cd ".." && pwd)

# Change to the project root
cd "${project_path}"

# Kill all child processes when receiving Ctrl + C
function stop_processes() {
  # shellcheck disable=SC2046
  [[ -z "$(jobs -p)" ]] || kill $(jobs -p)
}
trap stop_processes INT

# Start development server
cargo leptos watch &

# Wait for processes to finish
wait < <(jobs -p)
