#!/usr/bin/env bash
set -uo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: tools/supervise-command.sh <output-dir> <wall-seconds> <grace-seconds> -- <command> [args...]

Runs a command without a PTY under GNU timeout, records stdout/stderr/status
metadata, and preserves any default mutants.out directory as a partial artifact.
USAGE
}

if [ "$#" -lt 5 ] || [ "${4:-}" != "--" ]; then
  usage
  exit 64
fi

output_dir="$1"
wall_seconds="$2"
grace_seconds="$3"
shift 4

mkdir -p "$output_dir"

stdout_path="$output_dir/stdout.txt"
stderr_path="$output_dir/stderr.txt"
status_path="$output_dir/status.env"
command_path="$output_dir/command.txt"
metadata_path="$output_dir/metadata.txt"

{
  printf 'cwd=%s\n' "$(pwd)"
  printf 'command='
  separator=''
  for argument in "$@"; do
    printf '%s%q' "$separator" "$argument"
    separator=' '
  done
  printf '\n'
} > "$command_path"

{
  date -Is | sed 's/^/start_time=/'
  printf 'wall_seconds=%s\n' "$wall_seconds"
  printf 'grace_seconds=%s\n' "$grace_seconds"
  command -v timeout | sed 's/^/timeout_path=/'
  timeout --version | head -1 | sed 's/^/timeout_version=/'
  cargo --version | sed 's/^/cargo_version=/'
  rustc --version | sed 's/^/rustc_version=/'
  if command -v cargo-mutants >/dev/null 2>&1; then
    cargo mutants --version | sed 's/^/cargo_mutants_version=/'
  else
    printf 'cargo_mutants_version=not-installed\n'
  fi
} > "$metadata_path"

timeout --kill-after="${grace_seconds}s" "${wall_seconds}s" "$@" > "$stdout_path" 2> "$stderr_path"
status=$?

case "$status" in
  0)
    supervisor_result="child_exit_0"
    ;;
  124)
    supervisor_result="wrapper_wall_timeout"
    ;;
  125|126|127)
    supervisor_result="supervisor_or_spawn_failure"
    ;;
  137)
    supervisor_result="wrapper_forced_kill"
    ;;
  *)
    if [ "$status" -gt 128 ]; then
      supervisor_result="child_signal_or_forced_exit"
    else
      supervisor_result="child_nonzero_exit"
    fi
    ;;
esac

{
  printf 'exit_status=%s\n' "$status"
  printf 'supervisor_result=%s\n' "$supervisor_result"
  date -Is | sed 's/^/end_time=/'
} > "$status_path"

if [ -d mutants.out ] && find mutants.out -type f -newer "$metadata_path" -print -quit | grep -q .; then
  rm -rf "$output_dir/mutants.out.partial"
  cp -a mutants.out "$output_dir/mutants.out.partial"
fi

exit "$status"
