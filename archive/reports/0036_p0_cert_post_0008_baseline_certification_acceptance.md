# P0-CERT post-0008 baseline certification acceptance artifact

Spec: specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
Repository: joeloverbeck/tracewake
Target commit: 9f1622244c91c5952bd735da76f29fbe58f39f4b
Freshness claim: user-supplied target commit only; not independently verified as latest main
Verdict: <pending>
Verdict scope: post-0008 baseline only
Archived evidence posture: historical only
EMERGE-OBS posture: observer-only, non-gating, non-threshold

## Evidence Status Legend

Every evidence item carries exactly one status:

- `pass` - the artifact actually certifies the checked claim.
- `fail` - the checked claim failed and requires remediation.
- `pending` - the check has not yet produced certifying evidence.
- `sampled` - representative, not exhaustive.
- `observer-only` - review evidence that cannot certify behavior.
- `historical` - archive/spec/ticket evidence used only as context.

`pending`, `sampled`, `observer-only`, and `historical` are not counted as pass.

## Fingerprint Scope Legend

Fingerprints and stable artifacts declare one scope:

- `raw bytes`
- `normalized serialization`
- `parsed semantic content`
- `command transcript`
- `run seed`
- `replay artifact`
- `context hash/frontier`
- `projection checksum`
- `event log checksum`
- `not applicable`

Byte-stable goldens are not semantic behavior proof unless paired with behavior
witnesses and replay/provenance ancestry.

## Command And Lock-Layer Transcript

| Command | Status | Fingerprint scope | Transcript summary |
|---|---|---|---|
| `cargo fmt --all --check` | pass | command transcript | Passed on 2026-06-16 with no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | command transcript | Passed on 2026-06-16; `tracewake-core` checked, dev profile finished successfully. |
| `cargo build --workspace --all-targets --locked` | pass | command transcript | Passed on 2026-06-16; workspace all-targets locked build finished successfully. |
| `cargo test --workspace --locked` | pass | command transcript | Passed on 2026-06-16; workspace unit, integration, TUI, content, and doctest suites all reported `ok`. |
| `cargo test -p tracewake-core --test anti_regression_guards` | pass | command transcript | Passed on 2026-06-16; 80 passed, 0 failed. |
| `cargo test -p tracewake-core --test ci_workflow_guards` | pass | command transcript | Passed on 2026-06-16; 1 passed, 0 failed. |
| `cargo test -p tracewake-core --test spine_conformance` | pass | command transcript | Passed on 2026-06-16; 6 passed, 0 failed. |
| `cargo test -p tracewake-core --test hidden_truth_gates` | pass | command transcript | Passed on 2026-06-16; 13 passed, 0 failed. |
| `cargo test -p tracewake-core --test event_schema_replay_gates` | pass | command transcript | Passed on 2026-06-16; 17 passed, 0 failed. |
| `cargo test -p tracewake-content --test golden_fixtures_run` | pass | replay artifact | Passed on 2026-06-16; 40 passed, 0 failed, including fixture fingerprints, no-human replay, context hash, and replay tamper negatives. |
| `cargo test -p tracewake-content --test forbidden_content` | pass | command transcript | Passed on 2026-06-16; 20 passed, 0 failed. |
| `cargo test -p tracewake-content --test schema_conformance` | pass | command transcript | Passed on 2026-06-16; 2 passed, 0 failed. |
| `cargo test -p tracewake-tui --test adversarial_gates` | pass | command transcript | Passed on 2026-06-16; 15 passed, 0 failed. |
| `cargo mutants --version` | pass | command transcript | `cargo-mutants 27.1.0` is installed. |
| `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle` | pending | command transcript | Attempted on 2026-06-16. It found 1128 mutants, passed the unmutated baseline in 9s build + 31s test, then emitted one miss (`crates/tracewake-core/src/projections.rs:336:5 replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]`) before the local interactive attempt was stopped as too large for this ticket turn. Full scheduled mutation baseline remains pending; this row is not counted as pass. |
| Static guard scans | sampled | command transcript | Bounded `rg` scans on 2026-06-16 covered event append/application call sites, hidden-truth/debug terms, diagnostic typing, controller/debug/player terms, and nondeterminism/collection APIs. Results were reviewed as supporting scan evidence only; lock-layer tests remain the certifying source for these guard classes. |

## Global Replay And Provenance Fingerprints

| Evidence item ID | Evidence status | Fingerprint scope | Evidence summary | Certification use |
|---|---|---|---|---|
| `0036-GLOBAL-MANIFEST` | pass | raw bytes | `cargo test -p tracewake-content --test golden_fixtures_run` passed `fixture_fingerprints_match_frozen_goldens`; fixture manifest fingerprints match the frozen table. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-SEED` | sampled | run seed | `cargo test -p tracewake-core --test generative_lock` passed inside `cargo test --workspace --locked`; detailed seed enumeration remains for capstone/gate evidence. | not counted as certifying evidence |
| `0036-GLOBAL-EVENT-LOG` | pass | event log checksum | `golden_fixtures_run`, `golden_scenarios`, and `event_schema_replay_gates` exercised canonical event-log serialization, tamper negatives, and replay checksums. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-LIVE-PROJECTION` | pass | projection checksum | `golden_fixtures_run` and `golden_scenarios` computed live physical/agent checksums for replay comparisons. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-REPLAY-PROJECTION` | pass | projection checksum | Replay tests reported matching live/replay checksums for positive cases and mismatch for tampered/corrupt cases. Gate-specific divergence rows remain owned by `-002`...`-011`. | counted as certifying pass for scaffold transcript only |
| `0036-GLOBAL-PROVENANCE` | sampled | context hash/frontier | `golden_fixtures_run`, `hidden_truth_gates`, and `event_schema_replay_gates` exercised context hash/frontier and provenance negatives. Full per-gate provenance rows remain owned by `-002`...`-011`. | not counted as certifying evidence |
| `0036-GLOBAL-DEBUG` | observer-only | replay artifact | Debug-only truth/belief comparison rows, if present, remain observer-only and quarantined from certification. | not counted as certifying evidence |

## Per-Requirement Acceptance Evidence

Each requirement row cites evidence item IDs from this artifact. The result is
computed only from certifying evidence items.

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `P0-01` world-affecting behavior enters through proposal -> validation -> event -> application -> projection -> replay | `proposal_construction`, `action_validation`, `event_append`, `event_application`, `projection`, `replay` | pending gate evidence from `0036P0CERPOS0008-002` | pending |
| `P0-02` autonomous decisions use sealed actor-known contexts with provenance | `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction` | pending gate evidence from `0036P0CERPOS0008-003` | pending |
| `P0-03` human-origin commands bind to ordinary actors and share action rules | `tui_input_binding`, `view_model`, `proposal_construction`, `action_validation`, `event_append` | pending gate evidence from `0036P0CERPOS0008-004` | pending |
| `P0-04` possession never resets needs, intentions, memories, routines, or world-fact access | `tui_input_binding`, `holder_known_context`, `intention_lifecycle`, `view_model`, `projection` | pending gate evidence from `0036P0CERPOS0008-005` | pending |
| `P0-05` scheduler paths cannot emit primitive world actions from raw truth, routine labels, need thresholds, or fixture tables | `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`, `local_planning`, `proposal_construction`, `action_validation`, `event_append`, `replay` | pending gate evidence from `0036P0CERPOS0008-006` | pending |
| `P0-06` validation truth may accept/reject/mutate through events but may not propose fallback plans or actor-visible hidden facts | `action_validation`, `event_application`, `holder_known_context`, `candidate_generation`, `local_planning`, `proposal_construction`, `debug_quarantine` | pending gate evidence from `0036P0CERPOS0008-007` | pending |
| `P0-07` debug surfaces are non-diegetic and cannot feed embodied/world surfaces | `debug_quarantine`, `view_model`, `holder_known_context`, `tui_input_binding`, `test_oracle` | pending gate evidence from `0036P0CERPOS0008-008` | pending |
| `P0-08` golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases | `fixture_contract`, `content_schema`, `content_validation`, `holder_known_context`, `test_oracle`, `view_model`, `debug_quarantine`, `replay` | pending gate evidence from `0036P0CERPOS0008-009` | pending |
| `P0-09` failure diagnostics name responsible layer | `doctrine`, `content_schema`, `content_validation`, `fixture_contract`, `holder_known_context`, `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, `replay`, `view_model`, `debug_quarantine`, `tui_input_binding`, `test_oracle` | pending gate evidence from `0036P0CERPOS0008-010` | pending |
| `P0-10` archived specs and tickets are cited only as history | `doctrine`, `documentation status`, `test_oracle` | pending gate evidence from `0036P0CERPOS0008-011` | pending |

## Gate Evidence Sections

### P0-01 - Proposal, Validation, Event, Projection, Replay

Status: pending

Evidence will be filled by `0036P0CERPOS0008-002`.

### P0-02 - Actor-Known Contexts And Provenance

Status: pending

Evidence will be filled by `0036P0CERPOS0008-003`.

### P0-03 - Human-Origin Ordinary Actor Rules

Status: pending

Evidence will be filled by `0036P0CERPOS0008-004`.

### P0-04 - Possession Non-Reset

Status: pending

Evidence will be filled by `0036P0CERPOS0008-005`.

### P0-05 - Scheduler And No-Human Boundaries

Status: pending

Evidence will be filled by `0036P0CERPOS0008-006`.

### P0-06 - Validation Truth Boundary

Status: pending

Evidence will be filled by `0036P0CERPOS0008-007`.

### P0-07 - Debug Quarantine

Status: pending

Evidence will be filled by `0036P0CERPOS0008-008`.

### P0-08 - Golden And Adversarial Fixture Corpus

Status: pending

Evidence will be filled by `0036P0CERPOS0008-009`.

### P0-09 - Responsible-Layer Diagnostics

Status: pending

Evidence will be filled by `0036P0CERPOS0008-010`.

### P0-10 - Historical-Only Archive Use

Status: pending

Evidence will be filled by `0036P0CERPOS0008-011`.

## Sampling And Exhaustiveness

| Evidence class | Status | Scope |
|---|---|---|
| Static seam scans | sampled | Bounded source scans ran for append/apply entry points, hidden-truth/debug terms, diagnostic typing, controller/player/debug terms, and nondeterminism/collection APIs. The scans are review evidence; the certifying lock-layer is the named guard tests. |
| Golden behavior witnesses | sampled | `golden_fixtures_run`, `golden_scenarios`, and `event_schema_replay_gates` passed. Per-gate semantic witness rows remain owned by gate tickets `-002`...`-011`. |
| Property/generative evidence | sampled | `generative_lock` passed as part of `cargo test --workspace --locked`; explicit seed/accounting rows remain capstone-owned. |
| TUI transcript evidence | sampled | `adversarial_gates` passed separately and all TUI tests passed inside `cargo test --workspace --locked`; exhaustive carrier census remains gate-owned. |
| Mutation testing | pending | `cargo-mutants 27.1.0` is installed and the scheduled CI baseline command was attempted locally. The run found 1128 mutants and passed the unmutated baseline, then was stopped after emitting one missed mutant because the configured baseline is too large for this interactive ticket turn. Full scheduled baseline remains pending and is not counted as pass. |

## Pending And Historical Evidence

Archived specs, tickets, and reports are `historical` only. They may explain
lineage and prior remediation context, but they do not supply live P0-CERT pass
evidence. Unavailable command runs, unsupported mutation execution, untriaged
survived mutants, or incomplete property/generative evidence remain `pending`
or `fail`, never pass.

Current pending item from ticket `0036P0CERPOS0008-001`: the full configured
mutation baseline did not complete locally on 2026-06-16. The local attempt
emitted one miss before interruption:
`crates/tracewake-core/src/projections.rs:336:5 replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]`.
That miss is untriaged in this artifact revision and must not be counted as a
certifying pass.

## Certification Use

No later spec may cite this artifact as `P0-CERT passed` while the verdict is
`<pending>`. If the capstone renders `P0-CERT passed`, later specs may cite this
artifact and name the certified gates consumed, but still must satisfy stricter
entry gates for `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`,
`FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or `SECOND-PROOF-ENTRY` as applicable. If
the capstone renders `P0-CERT scoped remediation`, only remediation specs
addressing named failures are admissible until a replacement certification
artifact passes.

No later spec may cite `EMERGE-OBS` counters as gate pass/fail thresholds. No
later spec may cite archived specs or tickets as live certification.

## Staged-Abstraction Declarations

Staged abstraction: `EMERGE-OBS`
Evidence status: observer-only
What it proves now: Pending execution of observer-only emergence ledger evidence, if the corpus exercises it.
What it abstracts: It does not model or certify drama goals, thresholds, or phase-entry criteria.
What it must not fake: It must not turn observer counters into pass/fail thresholds or simulation inputs.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, and any gameplay expansion gate.
Diagnostic that fails if it leaks: `debug_quarantine` / `test_oracle` if observer rows feed cognition, scheduling, validation, authoring, content selection, LOD promotion, or pacing.

Staged abstraction: no-human canonical corpus sampling
Evidence status: pending
What it proves now: Pending sampled/golden no-human behavior evidence.
What it abstracts: Exhaustive coverage over all fixtures, seeds, schedules, and ordinary-life interactions.
What it must not fake: Friendly no-human success must not hide scheduler primitive-action shortcuts or missing actor-known provenance.
Future tier/feature it must not certify by implication: `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or second-proof ordinary-life expansion.
Diagnostic that fails if it leaks: `scheduler` / `holder_known_context` if no-human metrics or routine labels become action sources.

Staged abstraction: mutation testing
Evidence status: pending
What it proves now: Pending configured mutation posture against `.cargo/mutants.toml`.
What it abstracts: Exhaustive semantic proof over every protected seam.
What it must not fake: A green or unavailable mutation run must not substitute for behavior witnesses, provenance rows, or replay evidence.
Future tier/feature it must not certify by implication: Any future cert gate beyond the seams actually mutated and triaged.
Diagnostic that fails if it leaks: `test_oracle` if survived/uncovered critical mutants are counted as pass without triage.

Staged abstraction: TUI transcript evidence
Evidence status: pending
What it proves now: Pending possession/debug-quarantine transcript evidence.
What it abstracts: Exhaustive coverage over every actor-visible carrier, debug panel, and input path.
What it must not fake: Transcript snapshots must not rely on debug truth as actor knowledge or imply carrier surfaces outside the run.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, second-proof UI surfaces, or LLM/speech surfaces.
Diagnostic that fails if it leaks: `debug_quarantine` / `tui_input_binding` if debug output or player knowledge feeds embodied affordances.

Staged abstraction: temporal evidence in first-proof surfaces
Evidence status: pending
What it proves now: Pending first-proof temporal evidence where current tests exercise scheduler/replay time.
What it abstracts: Phase-4 procedural time, calendars, LOD time acceleration, and second-proof temporal ancestry.
What it must not fake: Event/replay time evidence must not become holder-known planning authority without modeled provenance.
Future tier/feature it must not certify by implication: `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, and LOD/time-acceleration work.
Diagnostic that fails if it leaks: `scheduler` / `holder_known_context` if truth-time validates by becoming cognition input.

## Implementer Self-Check

- [ ] Every exact source path used for evidence is present in the manifest and fetched from `joeloverbeck/tracewake` at `9f1622244c91c5952bd735da76f29fbe58f39f4b` or from an exact exported tree supplied by the user.
- [ ] No branch-name fetch, default-branch lookup, repository metadata, code search, snippets, prior chat memory, or connector namespace label was used as content proof.
- [ ] All ten P0-CERT proof requirements in Section 7 have evidence-status entries.
- [ ] Every canonical gate and first-proof acceptance label is composed only as a cross-reference; no new gate code/status vocabulary is minted.
- [ ] `EMERGE-OBS` is observer-only, non-gating, non-threshold, replay-derived, and quarantined from simulation inputs.
- [ ] Positive and negative fixtures both ran, and negatives failed for the intended responsible layer.
- [ ] Event/replay/projection evidence includes semantic behavior witnesses, not only bytes/checksums.
- [ ] Actor-known context evidence includes provenance, freshness/staleness, source IDs, and hidden-truth exclusion.
- [ ] Debug quarantine evidence includes actor-visible/debug/transcript/observer carrier separation.
- [ ] Diagnostics name responsible layer, expected input source, actual input source, actor-visible output, debug-only output, hidden truth excluded/leaked, replay divergence if any, and remediation category.
- [ ] Archived specs/tickets/reports are labeled historical only.
- [ ] Deferrals are tied to `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`, or observer-only obligations as appropriate.
- [ ] No pass/fail result relies on this spec's existence rather than this generated acceptance artifact.
