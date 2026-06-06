# 0003PHA1AEXETUI-004: Reconcile README, spec ledger, and specs index with the executable binary

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — docs (`README.md`, `docs/4-specs/SPEC_LEDGER.md`, `docs/4-specs/README.md`) plus a new README-fidelity test.
**Deps**: 0003PHA1AEXETUI-002, 0003PHA1AEXETUI-003

## Problem

Per spec §"Gap statement" and §"Documentation requirements", the working-tree `README.md` was already rewritten to describe the interactive loop (and `docs/4-specs/SPEC_LEDGER.md` already carries the Spec 0003 entry) *ahead of the code* — so until 0003PHA1AEXETUI-002 lands the README over-claims, documenting commands the binary rejects. The ledger also records the spec home as `docs/4-specs/0003_…` while the file lives at `specs/0003_…` (spec finding M2), and `docs/4-specs/README.md` "Current specs" omits Spec 0003.

## Assumption Reassessment (2026-06-06)

1. `README.md:66-133` already documents the loop, bare `<n>`, and the `debug …` family with a `tracewake>` sample session (read this session); the committed `1d27a01` README instead said "renders **one** embodied frame … and exits" and "Command vocabulary and debug panels (implemented, not yet wired into the binary)" (via `git show HEAD:README.md`). This ticket *reconciles* README claims with the binary that 002 lands — it does not re-author the section from scratch (spec §"Documentation requirements" `README.md`).
2. `docs/4-specs/SPEC_LEDGER.md:84` records `Spec file: docs/4-specs/0003_…`; the actual path is `specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` (confirmed via `ls`). `docs/4-specs/README.md:9-12` "Current specs" omits 0003. These are spec reassessment finding M2.
3. Shared boundary under audit: the documentation surfaces that describe the `tracewake-tui` command vocabulary which 001/002 implement; the new README-fidelity test couples documented command names to actual binary behavior.
4. INV-066 (every runnable phase has a TUI/view-model gate) and INV-065 (the TUI is a primary product interface) motivate this ticket — restated: Phase 1 mechanics must be reachable *and* documented honestly; a README that documents commands the binary rejects fails the spec §"Acceptance checklist" bullet "README … aligned with the executable state." INV-066 at `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:287`; INV-065 at :283.

## Architecture Check

1. A single trailing docs ticket lands the README / ledger / index reconciliation atomically once the binary exists, preventing the doc-ahead-of-code window from reappearing. The README-fidelity test (spec §"Required tests" #5) makes "docs match binary" a regression rather than a manual promise.
2. No backwards-compatibility shims: the ledger path is pinned to the real location rather than aliasing both `docs/4-specs/` and `specs/` homes.

## Verification Layers

1. INV-066 (mechanics reachable + documented) -> README-sample test: every command name in the README §Commands block is accepted (or deliberately rejected as documented) by the binary.
2. Doc-path integrity -> codebase grep-proof: the ledger's Spec 0003 `Spec file:` path resolves to `specs/0003_…`, and `docs/4-specs/README.md` lists Spec 0003 under "Current specs".
3. Single docs-surface ticket: each invariant maps to its own proof surface (sample-session test vs. grep), not collapsed into one generic "review".

## What to Change

### 1. `README.md`

Verify and reconcile the §"Running the TUI" / §Commands / sample-session blocks against the landed binary's actual command names and output; ensure no documented command is rejected (notably `<n>`); keep the non-diegetic-debug note. Do not re-author from scratch.

### 2. `docs/4-specs/SPEC_LEDGER.md`

Pin the Spec 0003 `Spec file:` path to wherever the spec actually lives (`specs/0003_…`). Verify the "Spec 0002 landed the kernel/facade … Spec 0003 closes the loop gap", not-Phase-2, and Phase-2-block statements remain intact.

### 3. `docs/4-specs/README.md`

Add Spec 0003 to the "Current specs" list, consistent with the canonical location pinned in the ledger.

### 4. New `tests/readme_sample_session.rs`

Drive the binary with the README sample commands (or representative fragments) and fail if a README-documented command name stops working (spec §"Required tests" #5).

## Files to Touch

- `README.md` (modify)
- `docs/4-specs/SPEC_LEDGER.md` (modify)
- `docs/4-specs/README.md` (modify)
- `crates/tracewake-tui/tests/readme_sample_session.rs` (new)

## Out of Scope

- Production loop / parser logic (0003PHA1AEXETUI-001 / 002).
- Rewriting archived Spec 0002 or archived tickets (spec §"Documentation requirements" — "Do not rewrite archived tickets or archived Spec 0002").
- Any Phase 2 ledger entry or next-spec guidance beyond the Spec 0003 block.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --test readme_sample_session` — README command names are live against the binary.
2. `grep -n "specs/0003_PHASE_1A" docs/4-specs/SPEC_LEDGER.md` shows the corrected path; `grep -n 0003 docs/4-specs/README.md` shows the Current-specs entry.
3. `cargo test --workspace`.

### Invariants

1. No command documented in `README.md` is rejected by the binary.
2. The ledger's Spec 0003 file path resolves to an existing file.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/readme_sample_session.rs` — README sample-session liveness guard (spec §"Required tests" #5).

### Commands

1. `cargo test -p tracewake-tui --test readme_sample_session`
2. `test -f "$(grep -oE 'specs/0003_[A-Z0-9_]+\.md' docs/4-specs/SPEC_LEDGER.md | head -1)" && cargo test --workspace`

## Outcome

Completion date: 2026-06-06

What changed:

- Reconciled the README example session with the implemented stdout shape: prompt lines are rendered as `tracewake>`, and accepted actions print `Accepted: <semantic_action_id>`.
- Corrected the Spec 0003 ledger path to the actual canonical `specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md` file.
- Added Spec 0003 to `docs/4-specs/README.md` under Current specs.
- Added `crates/tracewake-tui/tests/readme_sample_session.rs`, which launches the real binary and exercises representative README-documented command forms.

Deviations from original plan:

- The README liveness test uses a representative command script rather than scraping and replaying the prose sample verbatim, because the sample intentionally contains ellipses and illustrative output.

Verification results:

- `cargo test -p tracewake-tui --test readme_sample_session` — passed.
- `grep -n "specs/0003_PHASE_1A" docs/4-specs/SPEC_LEDGER.md` — showed the corrected path.
- `grep -n "0003" docs/4-specs/README.md` — showed the Current-specs entry.
- `test -f "$(grep -oE 'specs/0003_[A-Z0-9_]+\.md' docs/4-specs/SPEC_LEDGER.md | head -1)"` — passed.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace` — passed.
