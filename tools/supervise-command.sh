#!/usr/bin/env bash
set -uo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: tools/supervise-command.sh <output-dir> <wall-seconds> <grace-seconds> -- <command> [args...]

Runs a command without a PTY in its own process group, records stdout/stderr and
machine-readable status metadata, and preserves cargo-mutants output when present.
USAGE
}

if [ "$#" -lt 5 ] || [ "${4:-}" != "--" ]; then
  usage
  exit 64
fi

requested_output_dir="$1"
wall_seconds="$2"
grace_seconds="$3"
shift 4

if ! [[ "$wall_seconds" =~ ^[0-9]+$ ]] || ! [[ "$grace_seconds" =~ ^[0-9]+$ ]]; then
  echo "wall-seconds and grace-seconds must be non-negative integers" >&2
  exit 64
fi

output_dir="$requested_output_dir"
if [ -e "$output_dir" ] && find "$output_dir" -mindepth 1 -print -quit | grep -q .; then
  attempt=1
  while [ -e "${requested_output_dir}.attempt-${attempt}" ]; do
    attempt=$((attempt + 1))
  done
  output_dir="${requested_output_dir}.attempt-${attempt}"
fi

mkdir -p "$output_dir"

stdout_path="$output_dir/stdout.txt"
stderr_path="$output_dir/stderr.txt"
status_path="$output_dir/status.env"
command_path="$output_dir/command.txt"
metadata_path="$output_dir/metadata.txt"
env_path="$output_dir/environment.env"
artifact_marker_path="$output_dir/artifact-start.marker"

child_pid=""
timer_pid=""
child_status=""
child_signal=""
exit_status=125
supervisor_result="supervisor_or_spawn_failure"
termination_signal=""
forced_kill_sent="false"
artifact_copy_status="not_attempted"
artifact_copy_error=""
finalized="false"

quote_command() {
  local separator=""
  for argument in "$@"; do
    printf '%s%q' "$separator" "$argument"
    separator=' '
  done
}

record_environment() {
  env | LC_ALL=C sort | grep -E '^(CARGO|RUST|RUSTUP|RUSTC|RUSTFLAGS|MUTANTS|CI|GITHUB_ACTIONS|RUNNER_OS|RUNNER_ARCH|RUNNER_TEMP|RUNNER_TOOL_CACHE|CARGO_TARGET_DIR|SCCACHE|CCACHE)=' > "$env_path" || true
}

mutants_output_dirs() {
  local output_next="false"
  local arg
  for arg in "$@"; do
    if [ "$output_next" = "true" ]; then
      printf '%s\n' "$arg"
      output_next="false"
      continue
    fi
    case "$arg" in
      -o|--output)
        output_next="true"
        ;;
      -o*)
        printf '%s\n' "${arg#-o}"
        ;;
      --output=*)
        printf '%s\n' "${arg#--output=}"
        ;;
    esac
  done
  printf '%s\n' "mutants.out"
}

copy_artifacts() {
  local copied="false"
  local source_dir
  while IFS= read -r source_dir; do
    [ -n "$source_dir" ] || continue
    [ -d "$source_dir" ] || continue
    if ! find "$source_dir" -type f -newer "$artifact_marker_path" -print -quit | grep -q .; then
      continue
    fi
    local label
    label="$(printf '%s' "$source_dir" | sed -E 's#^\./##; s#[^A-Za-z0-9_.-]+#_#g')"
    local destination="$output_dir/${label}.partial"
    if cp -a "$source_dir" "$destination" 2>> "$stderr_path"; then
      copied="true"
    else
      artifact_copy_status="failed"
      artifact_copy_error="failed_to_copy_${source_dir}"
      return
    fi
  done < <(mutants_output_dirs "$@" | awk '!seen[$0]++')

  if [ "$artifact_copy_status" = "failed" ]; then
    return
  fi
  if [ "$copied" = "true" ]; then
    artifact_copy_status="copied"
  else
    artifact_copy_status="none_present"
  fi
}

write_status() {
  {
    printf 'exit_status=%s\n' "$exit_status"
    printf 'supervisor_result=%s\n' "$supervisor_result"
    printf 'child_pid=%s\n' "${child_pid:-}"
    printf 'process_group_id=%s\n' "${child_pid:-}"
    printf 'child_exit_status=%s\n' "$child_status"
    printf 'child_signal=%s\n' "$child_signal"
    printf 'termination_signal=%s\n' "$termination_signal"
    printf 'forced_kill_sent=%s\n' "$forced_kill_sent"
    printf 'artifact_copy_status=%s\n' "$artifact_copy_status"
    printf 'artifact_copy_error=%s\n' "$artifact_copy_error"
    date -Is | sed 's/^/end_time=/'
  } > "$status_path"
}

finalize() {
  if [ "$finalized" = "true" ]; then
    return
  fi
  finalized="true"
  copy_artifacts "$@"
  write_status
  rm -f "$output_dir/supervisor-event.tmp"
  rm -f "$artifact_marker_path"
}

handle_supervisor_signal() {
  local signal="$1"
  termination_signal="$signal"
  supervisor_result="supervisor_canceled"
  exit_status=130
  if [ -n "$child_pid" ] && kill -0 "$child_pid" 2>/dev/null; then
    kill -TERM "-$child_pid" 2>/dev/null || true
    sleep "$grace_seconds"
    if kill -0 "$child_pid" 2>/dev/null; then
      forced_kill_sent="true"
      kill -KILL "-$child_pid" 2>/dev/null || true
    fi
    wait "$child_pid" 2>/dev/null || true
  fi
  if [ -n "$timer_pid" ] && kill -0 "$timer_pid" 2>/dev/null; then
    kill "$timer_pid" 2>/dev/null || true
    wait "$timer_pid" 2>/dev/null || true
  fi
  finalize "$@"
  exit "$exit_status"
}

trap 'handle_supervisor_signal INT "$@"' INT
trap 'handle_supervisor_signal TERM "$@"' TERM
trap 'handle_supervisor_signal HUP "$@"' HUP
trap 'finalize "$@"' EXIT

{
  printf 'cwd=%s\n' "$(pwd)"
  printf 'requested_output_dir=%s\n' "$requested_output_dir"
  printf 'actual_output_dir=%s\n' "$output_dir"
  printf 'command='
  quote_command "$@"
  printf '\n'
} > "$command_path"

{
  date -Is | sed 's/^/start_time=/'
  printf 'wall_seconds=%s\n' "$wall_seconds"
  printf 'grace_seconds=%s\n' "$grace_seconds"
  printf 'supervisor_pid=%s\n' "$$"
  command -v setsid | sed 's/^/setsid_path=/' || true
  cargo --version | sed 's/^/cargo_version=/' || true
  rustc --version | sed 's/^/rustc_version=/' || true
  if command -v cargo-mutants >/dev/null 2>&1; then
    cargo mutants --version | sed 's/^/cargo_mutants_version=/'
  else
    printf 'cargo_mutants_version=not-installed\n'
  fi
} > "$metadata_path"
record_environment

touch "$artifact_marker_path"
setsid "$@" > "$stdout_path" 2> "$stderr_path" &
child_pid=$!
supervisor_event_path="$output_dir/supervisor-event.tmp"

printf 'child_pid=%s\n' "$child_pid" >> "$metadata_path"
printf 'process_group_id=%s\n' "$child_pid" >> "$metadata_path"

(
  sleep "$wall_seconds"
  if kill -0 "$child_pid" 2>/dev/null; then
    printf 'wrapper_wall_timeout\n' > "$supervisor_event_path"
    kill -TERM "-$child_pid" 2>/dev/null || true
    sleep "$grace_seconds"
    if kill -0 "$child_pid" 2>/dev/null; then
      printf 'wrapper_forced_kill\n' > "$supervisor_event_path"
      kill -KILL "-$child_pid" 2>/dev/null || true
    fi
  fi
) &
timer_pid=$!

wait "$child_pid"
child_status=$?
if [ -n "$timer_pid" ] && kill -0 "$timer_pid" 2>/dev/null; then
  kill "$timer_pid" 2>/dev/null || true
  wait "$timer_pid" 2>/dev/null || true
fi

if [ "$child_status" -gt 128 ]; then
  child_signal=$((child_status - 128))
fi

if [ -f "$supervisor_event_path" ]; then
  supervisor_result="$(tail -n 1 "$supervisor_event_path")"
  if [ "$supervisor_result" = "wrapper_forced_kill" ]; then
    forced_kill_sent="true"
    termination_signal="KILL"
    exit_status=137
  else
    termination_signal="TERM"
    exit_status=124
  fi
elif [ "$child_status" -eq 0 ]; then
  exit_status=0
  supervisor_result="child_exit_0"
elif [ "$child_status" -gt 128 ]; then
  exit_status=$child_status
  supervisor_result="child_signal_or_forced_exit"
else
  exit_status=$child_status
  supervisor_result="child_nonzero_exit"
fi

finalize "$@"
exit "$exit_status"
