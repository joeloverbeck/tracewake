# 0003 Acceptance Artifact Template

Use this template for implementation review of
`archive/specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`.

## Exact commit under test

- Commit: `<full git sha>`
- Branch or PR: `<branch-or-pr>`

## Gates run

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`

## Changed files

List each changed file that belongs to the implementation under review. Exclude
unrelated local worktree changes.

## Per-requirement acceptance evidence

| Requirement | Responsible layer | Evidence | Result |
|---|---|---|---|
| `SPINE-AC-001` | `core/state`, `core/events` | private authoritative fields, mutation capability guards, named tests | `<pass/fail>` |
| `SPINE-AC-002` | `core/replay`, `core/events` | schema-version registry and loud unknown-version replay test | `<pass/fail>` |
| `SPINE-AC-003` | `core/events`, `core/replay` | total event-kind metadata and non-world checksum guard | `<pass/fail>` |
| `SPINE-AC-004` | `core/replay` | authoritative checksum coverage manifest test | `<pass/fail>` |
| `SPINE-AC-005` | `workspace/ci` | nondeterminism banned-API source scan and clippy gate | `<pass/fail>` |
| `SPINE-AC-006` | `core/scheduler` | no-direct-dispatch scheduler source/runtime guards | `<pass/fail>` |
| `SPINE-AC-007` | `core/actions` | source-context binding adversarial tests | `<pass/fail>` |
| `SPINE-AC-008` | `core/actions`, `core/events` | append-before-apply and no alternate accepted mutation path tests | `<pass/fail>` |
| `SPINE-AC-009` | `core/agent`, `tui/debug` | actor-known/debug capability unforgeability and TUI debug quarantine tests | `<pass/fail>` |
| `SPINE-AC-010` | `content/schema`, `content/validation` | content field registry, canonical serialization, and prose-born fact rejection tests | `<pass/fail>` |
| `SPINE-AC-011` | `core/actions` | typed diagnostic and actor/debug provenance tests | `<pass/fail>` |
| `SPINE-AC-012` | `tui/view-model`, `tui/debug` | stale-view, debug command, direct-apply, and transcript snapshot tests | `<pass/fail>` |
| `SPINE-AC-013` | `workspace/ci` | named spine/content/TUI conformance suites map every requirement to named evidence | `<pass/fail>` |
| `SPINE-AC-014` | `workspace/ci` | doc/invariant reference linter under `cargo test` | `<pass/fail>` |
| `SPINE-AC-015` | `workspace/ci` | this scoped acceptance artifact with file/line evidence | `<pass/fail>` |

## Residual convention-only items

List any remaining item that cannot be structurally or mechanically enforced in
Rust tests. For each item, state why it remains review-required and name the
review surface that carries it.

## Scoped certification wording

Allowed wording:

- "Phase 1 / Phase 1A spine hardening remediation accepted for this commit."

Forbidden wording:

- "Project is P0 certified"
- "SPINE-CERT passed"

Do not use the forbidden wording as a result claim unless a separate upstream
certification process declares that outcome.
