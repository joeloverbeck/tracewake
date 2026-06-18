# 0038SPICEREVE-001: Acceptance-artifact scaffold + full §4 command transcript and environment

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — creates the `archive/reports/` SPINE-CERT acceptance artifact and captures the §4 command transcript; runs existing tests only, changes no engine code.
**Deps**: None

## Problem

Spec 0038 requires the implementing session to produce a single SPINE-CERT acceptance artifact that certifies the spine seam by seam and renders the verdict the spec itself withholds (§9). Before any per-seam evidence section (SPINE-01…SPINE-08), the mutation package (§9.6), or the capstone verdict (§9.4) can be written, the artifact must exist with its §9.1 header, §9.2 evidence-status ledger, §9.3 command transcript and environment, and empty section headers for each downstream consumer to fill.

This ticket creates that scaffold and runs the §4 minimum command vocabulary (workspace `fmt`/`clippy`/`build`/`test` plus the targeted gate commands), capturing each command's exit status, tool versions, environment, and the exact implementation-commit fingerprint. The eight seam tickets (`-002`…`-009`), the mutation ticket (`-010`), and the capstone (`-011`) write their own sections into this artifact. This ticket renders **no verdict** — the verdict is the capstone's (`-011`).

## Assumption Reassessment (2026-06-16)

1. The acceptance-artifact target path `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` does not yet exist (`test -f` → absent, 2026-06-16); its parent `archive/reports/` exists. This ticket creates the file `(new)`.
2. Spec §9.1 fixes the header fields; §9.2 fixes the evidence-status vocabulary (`observed run | static review | negative fixture | mutation evidence | historical only | pending`) and the fingerprint-scope/behavior-witness/sampling/pending-caveat/certification-use/staged-abstraction fields; §9.3 enumerates the required environment and command transcript. The artifact must conform to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (verified present 2026-06-16). The §4 command vocabulary and targeted gate commands were verified to resolve to real test targets during this session's `/reassess-spec`.
3. Shared boundary under audit: the acceptance-artifact contract itself — the `0003` template fields plus the §9.2 evidence-status discipline that every sibling ticket writes against. The legends and section skeleton authored here are the contract `-002`…`-011` consume; their wording must match `0003` and §9.2/§9.3 exactly so sibling sections stay consistent and `historical only` / `static review` / `pending` are never silently counted toward a pass.
4. `INV-017` (randomness seedable/auditable) and `INV-018` (deterministic replay is foundational) motivate the transcript and fingerprint infrastructure: the §4 transcript and §9.3 environment exist to make every downstream replay/determinism witness reproducible at a named implementation commit. Recorded seeds / random-stream identifiers accompany any generated or property case (`generative_lock`).
5. This ticket touches deterministic-replay and fail-closed-validation surfaces as an **evidence consumer**, not a modifier: it runs the workspace lint/build/test set and the targeted gate suites, then records their output. The artifact's debug-derived rows stay `observer-only` per `INV-107`; no captured transcript becomes diegetic actor knowledge or feeds simulation. No enforcement surface is weakened — this ticket adds none.

## Architecture Check

1. A single scaffold-and-transcript ticket landing first is cleaner than letting each seam ticket re-run the full command vocabulary: the expensive `cargo` build/test pass runs once at a fixed implementation commit, its transcript is cited by every seam, and the legends are authored in one place so sibling sections cannot drift in evidence-status or fingerprint-scope wording. This mirrors the accepted `archive/tickets/0036P0CERPOS0008-001` scaffold precedent.
2. No backwards-compatibility aliasing or shims. The artifact is new; it introduces no alternate path to any existing surface.

## Verification Layers

1. `INV-018` deterministic replay -> replay/golden-fixture check: `cargo test --workspace --locked` plus `event_schema_replay_gates` run green at the recorded commit; their transcripts are captured for downstream citation.
2. `INV-017` seedable randomness -> manual review: recorded seeds / random-stream IDs present for any generated/property case the transcript exercises (`generative_lock`).
3. Evidence-honesty discipline -> invariants alignment check: every transcript row carries exactly one §9.2 status; `pending` / `static review` / `historical only` are never presented as `observed run`.
4. Command-vocabulary completeness -> codebase grep-proof: each §4 command resolves to a real target and its exit status is captured (enumerated in Acceptance Criteria).

## What to Change

### 1. Create the acceptance artifact with header and legends

Create `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` with the §9.1 header (Title; Spec under execution; Implementation repository; Implementation commit; Commit freshness claim; `Spec posture consumed: P0-CERT passed`; `Gate label under certification: SPINE-CERT`; `Verdict: <pending>`; the non-executable spec note), the §9.2 evidence-status ledger legend, and empty section headers for each of the eight seams (SPINE-01…SPINE-08), the §9.4 per-seam verdict table, the §9.5 replay/provenance package, the §9.6 mutation package, and the §9.7 EMERGE-OBS slot — so `-002`…`-011` fill their own sections.

### 2. Run and capture the §4 command transcript and §9.3 environment

Run and capture, with exit status and output-artifact location for each: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace --locked`; and the §4 targeted gate commands across `tracewake-core`, `tracewake-content`, and `tracewake-tui`. Record the §9.3 environment: `rustc --version --verbose`, `cargo --version`, `cargo-mutants --version`, host OS/architecture, exact test filters used (a filtered run is paired with its unfiltered required command), and the exact implementation-commit fingerprint.

## Files to Touch

- `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` (new)
- transcript/checksum capture files under `archive/reports/` (new — implementation-discovered set; embed in the artifact or link as machine-readable siblings per §9.3)

## Out of Scope

- Per-seam evidence sections SPINE-01…SPINE-08 (owned by `-002`…`-009`).
- The mutation package — Wave A / Wave B / triage register (owned by `-010`).
- Capstone consolidation, replay/provenance package assembly, EMERGE-OBS packaging, and the final verdict (owned by `-011`).
- Remediation of any command/lint/test failure surfaced here — routed to a separate `SPINE-CERT scoped remediation` ticket/spec (spec §8); this ticket records the failure with status and responsible layer, it does not fix it.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check` — transcript captured with exit status.
2. `cargo clippy --workspace --all-targets -- -D warnings` — transcript captured.
3. `cargo build --workspace --all-targets --locked` — transcript captured.
4. `cargo test --workspace --locked` — transcript captured.
5. Each §4 targeted gate command run and its exit status + artifact location captured (the seam-specific subsets are exercised in depth by `-002`…`-010`).
6. §9.3 environment recorded: `rustc`, `cargo`, `cargo-mutants` versions, host OS/arch, and the exact implementation-commit fingerprint.

### Invariants

1. The artifact carries the §9.1 header, §9.2 evidence-status ledger, and §9.3 environment verbatim-consistent with `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and spec §9.1/§9.2/§9.3.
2. `Verdict` reads `<pending>` until `-011` renders it; no seam result is asserted by this ticket.
3. Every transcript row carries exactly one §9.2 evidence status; no `pending` row is presented as a pass.

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification is command-based (run the §4 command vocabulary and record the §9.3 environment), and the captured transcripts plus the scaffolded artifact are the deliverable.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace --locked`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` (replay lock layer — narrower boundary for the global replay/fingerprint capture)

## Outcome

Completed: 2026-06-18

Created `archive/reports/0038_spine_cert_event_log_replay_projection_pipeline_and_no_direct_dispatch_certification_acceptance.md` with the required §9.1 header, §9.2 evidence-status ledger fields, §9.3 command transcript/environment section, empty SPINE-01 through SPINE-08 sections, §9.4 per-seam verdict table, §9.5 replay/provenance package placeholder, §9.6 mutation package placeholder, and §9.7 EMERGE-OBS placeholder.

Captured the §4 command transcript and environment in `archive/reports/0038_spine_cert_command_transcripts/` for implementation commit `b4b59c92d126692c9f2fa4c986695b9f2e20db2c`. The artifact records `cargo mutants --version` as the available Cargo subcommand form; standalone `cargo-mutants --version` is not available in this environment and exits through Cargo argument parsing.

Verification run:

- `cargo fmt --all --check` — passed, transcript captured.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed, transcript captured.
- `cargo build --workspace --all-targets --locked` — passed, transcript captured.
- `cargo test --workspace --locked` — passed, transcript captured.
- All §4 targeted gate commands listed in the scaffold artifact — passed, transcripts captured.

Deviations: none for this ticket's scope. The report verdict and SPINE seam sections remain intentionally pending for `0038SPICEREVE-002` through `0038SPICEREVE-011`.
