#!/usr/bin/env bash
set -uo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: tools/clean-build-scratch.sh [--force]

Reclaims disk eaten by leaked build scratch from local development. Interrupted
`cargo mutants` runs are the main offender: each parallel build job copies the
whole workspace (with its own target/) into $TMPDIR and only deletes it on a
clean exit, so a Ctrl-C / OOM / timeout leaks multi-GB directories. On WSL2 this
silently grows the ext4.vhdx, which never shrinks on its own.

By default this prints what it WOULD remove and reclaim; nothing is deleted.
Pass --force to actually delete.

Targets (all regenerable):
  1. $TMPDIR/cargo-mutants-tracewake-*.tmp   leaked cargo-mutants scratch copies
  2. registered git worktrees under $TMPDIR  stale evidence/bisect checkouts
                                             (removed via `git worktree remove`,
                                             preserving commits)
  3. tests/negative-fixtures/*/target        compile-fail fixture build output

After a large reclaim on WSL2, the freed space stays inside the vhdx — it does
not return to C: until the disk is compacted. This is a MANUAL, human-only step
(it shuts WSL down); never run it from an automated/agent session. From an
elevated Windows PowerShell, with the vhdx path from `wsl --list --verbose`:

  wsl --shutdown
  diskpart
    select vdisk file="C:\path\to\ext4.vhdx"
    attach vdisk readonly
    compact vdisk
    detach vdisk
    exit

(or `Optimize-VHD -Path "C:\path\to\ext4.vhdx" -Mode Full` on Hyper-V hosts.)
Compacting only releases unused blocks; it never deletes the disk.
USAGE
}

FORCE=0
case "${1:-}" in
  --force) FORCE=1 ;;
  "" ) ;;
  -h|--help) usage; exit 0 ;;
  *) usage; exit 2 ;;
esac

TMPDIR="${TMPDIR:-/tmp}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

note() { printf '%s\n' "$*"; }
size_of() { du -sh "$1" 2>/dev/null | cut -f1; }

# 1. Leaked cargo-mutants scratch copies.
note "== cargo-mutants scratch in ${TMPDIR} =="
shopt -s nullglob
mutant_scratch=("${TMPDIR}"/cargo-mutants-*.tmp)
shopt -u nullglob
if ((${#mutant_scratch[@]})); then
  total="$(du -sch "${mutant_scratch[@]}" 2>/dev/null | tail -1 | cut -f1)"
  note "  ${#mutant_scratch[@]} dir(s), ${total} reclaimable"
  if ((FORCE)); then rm -rf "${mutant_scratch[@]}" && note "  removed."; fi
else
  note "  none."
fi

# 2. Stale git worktrees registered under $TMPDIR.
note "== git worktrees under ${TMPDIR} =="
stale_wt=()
while IFS= read -r line; do
  wt_path="${line%% *}"
  case "$wt_path" in "${TMPDIR}"/*) stale_wt+=("$wt_path") ;; esac
done < <(git -C "$REPO_ROOT" worktree list 2>/dev/null)
if ((${#stale_wt[@]})); then
  for wt in "${stale_wt[@]}"; do note "  $wt ($(size_of "$wt"))"; done
  if ((FORCE)); then
    for wt in "${stale_wt[@]}"; do git -C "$REPO_ROOT" worktree remove --force "$wt"; done
    git -C "$REPO_ROOT" worktree prune
    note "  removed and pruned."
  fi
else
  note "  none."
fi

# 3. Compile-fail fixture target/ dirs.
note "== negative-fixture target/ dirs =="
mapfile -t fixture_targets < <(find "${REPO_ROOT}/tests/negative-fixtures" -type d -name target -prune 2>/dev/null)
if ((${#fixture_targets[@]})); then
  total="$(du -sch "${fixture_targets[@]}" 2>/dev/null | tail -1 | cut -f1)"
  note "  ${#fixture_targets[@]} dir(s), ${total} reclaimable"
  if ((FORCE)); then rm -rf "${fixture_targets[@]}" && note "  removed."; fi
else
  note "  none."
fi

if ((FORCE)); then
  note ""
  note "Done. On WSL2 the freed space stays in the vhdx until a human compacts it"
  note "from Windows (manual, shuts WSL down) — see this script's --help."
else
  note ""
  note "Dry run — nothing deleted. Re-run with --force to delete."
fi
