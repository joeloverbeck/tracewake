# 0040EPICERHOL-001: Acceptance-artifact scaffold + §4 baseline command transcript and environment

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — creates the `reports/` EPI-CERT acceptance artifact and captures the §4 command transcript; runs existing tests only, changes no engine code.
**Deps**: None

## Problem

Spec 0040 requires the implementing session to produce a single EPI-CERT acceptance artifact that certifies the holder-known epistemic boundary audit-point by audit-point and renders the verdict the spec itself withholds (§9). Before any per-audit-point evidence section (EPI-01…EPI-11), the compile-fail corpus matrix (§6.1), the mutation package (§9.6), the relational capstone package (§9.7), or the aggregate verdict (§9.4/§9.9) can be written, the artifact must exist with its §9.1 header, §9.2 evidence-item ledger fields, §9.3 command-and-environment ledger, and empty section headers for each downstream consumer to fill.

This ticket creates that scaffold and runs the §4.1 clean baseline plus the §4.2 named test binaries, capturing each command's exact invocation, exit status, tool versions, determinism-relevant environment, and the exact implementation-commit fingerprint. The eleven audit-point tickets (`-002`…`-011`), the compile-fail corpus ticket (`-012`), the relational-capstone ticket (`-013`), the mutation ticket (`-014`), and the acceptance capstone (`-015`) write their own sections into this artifact. This ticket renders **no verdict** — the verdict is the capstone's (`-015`).

## Assumption Reassessment (2026-06-19)

1. The acceptance-artifact staging path `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` does not yet exist (`test -e` → absent, 2026-06-19); its parent `reports/` exists. This ticket creates the file `(new)`. The closeout path `archive/reports/…` is a spec-acceptance/archival move, deferred out of this batch (spec §9.1).
2. Spec §9.1 fixes the header fields; §9.2 fixes the evidence-item ledger fields and the evidence-status vocabulary (`pass | fail | pending | sampled | observer-only | historical` — this spec creates no new status); §9.3 enumerates the required command-and-environment ledger. The artifact must instantiate the fields from `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (verified present 2026-06-19). The §4.1 baseline commands and §4.2 named `--test` targets were verified to resolve to real test binaries during this session's `/reassess-spec` (every binary in `crates/{tracewake-core,tracewake-content,tracewake-tui}/tests/` enumerated).
3. Shared boundary under audit: the acceptance-artifact contract itself — the `0003` template fields plus the §9.2 evidence-status discipline that every sibling ticket writes against. The legends and section skeleton authored here are the contract `-002`…`-015` consume; their wording must match `0003` and §9.2/§9.3 exactly so sibling sections stay consistent and `pending` / `sampled` / `observer-only` / `historical` are never silently counted toward a pass (§9.2, §9.4). This mirrors the accepted `archive/tickets/0038SPICEREVE-001` scaffold precedent.
4. `INV-017` (randomness seedable/auditable) and `INV-018` (deterministic replay is foundational) motivate the transcript and fingerprint infrastructure: the §4 transcript and §9.3 environment exist to make every downstream replay/determinism witness reproducible at a named implementation commit. Recorded seeds / random-stream identifiers accompany any generated or property case (`generative_lock`). `INV-098` (harsh feature acceptance) is the discipline this scaffold serves — certifying evidence only, at the exact final commit.
5. This ticket touches deterministic-replay and fail-closed-validation surfaces as an **evidence consumer**, not a modifier: it runs the workspace lint/build/test set and the §4.2 named binaries, then records their output. No enforcement surface is weakened — this ticket adds none. Any debug-derived transcript row stays `observer-only` per `INV-107`; no captured transcript becomes diegetic actor knowledge or feeds simulation. A baseline command that fails or is flaky is recorded with its status and responsible layer per §4.1/§8; it is not retried-until-green without explanation, and remediation is routed out (§7.6/§8), not performed here.

## Architecture Check

1. A single scaffold-and-transcript ticket landing first is cleaner than letting each audit-point ticket re-run the full command vocabulary: the expensive `cargo` build/test pass runs once at a fixed implementation commit, its transcript is cited by every audit point, and the legends are authored in one place so sibling sections cannot drift in evidence-status or fingerprint-scope wording. This is the accepted certification-audit shape (`archive/tickets/0036P0CERPOS0008-001`, `0038SPICEREVE-001`).
2. No backwards-compatibility aliasing or shims. The artifact is new; it introduces no alternate path to any existing surface and mints no new gate code, invariant ID, status enum, or obligation code (spec preamble).

## Verification Layers

1. `INV-018` deterministic replay -> replay/golden-fixture check: `cargo test --workspace --locked` plus `event_schema_replay_gates` run at the recorded commit; their transcripts are captured for downstream citation.
2. `INV-017` seedable randomness -> manual review: recorded seeds / random-stream IDs present for any generated/property case the transcript exercises (`generative_lock`).
3. Evidence-honesty discipline (`INV-098`) -> invariants alignment check: every transcript row carries exactly one §9.2 status; `pending` / `sampled` / `observer-only` / `historical` are never presented as a certifying pass.
4. Command-vocabulary completeness -> codebase grep-proof: each §4.1/§4.2 command resolves to a real target and its exit status is captured (enumerated in Acceptance Criteria).

## What to Change

### 1. Create the acceptance artifact with header and legends

Create `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` with:

- the §9.1 header (this spec's exact path/number; target/source baseline `ba9fe1c9ec3cfc18bf911e5aff7d97b8476175ab`; the exact final implementation commit actually tested + clean/dirty worktree statement; `Spec posture consumed: P0-CERT passed (0037), SPINE-CERT passed (0039 @ 92ba47f)`; `Gate label under certification: EPI-CERT`; `Verdict: <pending>`; the non-executable-spec note; the limitation that neither workflow nor verdict verifies current `main`);
- the §9.2 evidence-item ledger legend (Evidence item ID; EPI cross-references; Evidence status from the fixed vocabulary; Fingerprint scope; Evidence summary; Path-under-test and behavior witness; Replay/provenance ancestry; Sampling/exhaustiveness; Pending/historical handling; Certification use; Staged-abstraction declaration);
- the §9.3 command-and-environment ledger table skeleton; and
- empty section headers for each of the eleven audit points (EPI-01…EPI-11), the §6.1 compile-fail corpus matrix, the §9.4 per-seam verdict table, the §9.5 replay/provenance package, the §9.6 mutation package, the §9.7 relational-capstone package, and the §9.8 EMERGE-OBS slot — so `-002`…`-015` fill their own sections.

### 2. Run and capture the §4 command transcript and §9.3 environment

Run and capture, with exact invocation, exit status, and output-artifact location for each: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo build --workspace --all-targets --locked`; `cargo test --workspace --locked`; `cargo test --locked -p tracewake-core --doc`; and the §4.2 named test binaries across `tracewake-core`, `tracewake-content`, and `tracewake-tui`. Record the §9.3 environment: `rustc --version --verbose`, `cargo --version`, `cargo mutants --version`, host OS/architecture, exact test filters used (a filtered run is paired with its unfiltered required command), `Cargo.lock` / `.cargo/mutants.toml` / CI-workflow fingerprints with stated scopes, and the exact implementation-commit fingerprint.

## Files to Touch

- `reports/0040_epi_cert_holder_known_contexts_beliefs_observations_provenance_possession_parity_view_models_and_debug_quarantine_certification_acceptance.md` (new)
- transcript/checksum capture files under `reports/` (new — implementation-discovered set; embed in the artifact or link as machine-readable siblings per §9.3)

## Out of Scope

- Per-audit-point evidence sections EPI-01…EPI-11 (owned by `-002`…`-011`).
- The §6.1 compile-fail boundary corpus matrix (owned by `-012`).
- The EPI-11 relational-capstone harness and §6.2 generated/metamorphic evidence (owned by `-013`).
- The mutation package — Wave A / Wave B / triage register (owned by `-014`).
- Capstone consolidation, §9.4 verdict table, §9.5 replay/provenance package, §9.7 relational package, §9.8 EMERGE-OBS packaging, and the final verdict (owned by `-015`).
- Remediation of any command/lint/test failure surfaced here — routed to a separate `EPI-CERT scoped remediation` spec (spec §7.6/§8); this ticket records the failure with status and responsible layer, it does not fix it.
- The `archive/reports/` closeout move and the `docs/4-specs/SPEC_LEDGER.md` row — deferred to spec acceptance/archival (`docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check` — transcript captured with exit status.
2. `cargo clippy --workspace --all-targets -- -D warnings` — transcript captured.
3. `cargo build --workspace --all-targets --locked` — transcript captured.
4. `cargo test --workspace --locked` and `cargo test --locked -p tracewake-core --doc` — transcripts captured.
5. Each §4.2 named test binary run and its exit status + artifact location captured (the audit-point subsets are exercised in depth by `-002`…`-014`).
6. §9.3 environment recorded: `rustc`, `cargo`, `cargo mutants` versions, host OS/arch, `Cargo.lock`/`.cargo/mutants.toml`/CI fingerprints, and the exact implementation-commit fingerprint.

### Invariants

1. The artifact carries the §9.1 header, §9.2 evidence-item ledger, and §9.3 ledger verbatim-consistent with `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` and spec §9.1/§9.2/§9.3.
2. `Verdict` reads `<pending>` until `-015` renders it; no audit-point result is asserted by this ticket.
3. Every transcript row carries exactly one §9.2 evidence status; no `pending` / `sampled` / `observer-only` / `historical` row is presented as a certifying pass.

## Test Plan

### New/Modified Tests

1. `None — documentation/evidence-only ticket; verification is command-based (run the §4.1 baseline + §4.2 named binaries and record the §9.3 environment), and the captured transcripts plus the scaffolded artifact are the deliverable.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace --locked && cargo test --locked -p tracewake-core --doc`
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` (replay lock layer — narrower boundary for the global replay/fingerprint capture)
