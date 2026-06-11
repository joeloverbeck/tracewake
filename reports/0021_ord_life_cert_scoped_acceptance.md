# 0021 ORD-LIFE-CERT-Scoped Acceptance Artifact

Date: 2026-06-11

Target implementation commits under review:
`e1b94a10d04e736ce0ec7802b4d46948de78d0fa` through
`e0ff5f6e6d62a7320f7eee79fdbee918b5d42a7e`

Scope:
`archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md`

This report is scoped evidence toward `ORD-LIFE-CERT` for the 0021 Phase 3A
possession-rebind, provenance, replay-posture, event-perimeter, content-integrity,
embodied-surface, generative, and low-severity closure work. It is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`.

## Manifest Evidence

| Ticket | Commit | Evidence surface |
|---|---|---|
| `0021PHA3APOSREB-001` | `e1b94a1` | Possession rebind rejection hygiene and non-diegetic context rendering |
| `0021PHA3APOSREB-002` | `9737fc6` | Hidden-truth harness provenance fidelity |
| `0021PHA3APOSREB-003` | `6357f4c` | Apply-arm census guards and synthetic regressions |
| `0021PHA3APOSREB-004` | `549e332` | Mutation CI ratchet, baseline governance, and closed dispositions |
| `0021PHA3APOSREB-005` | `bb40d22` | Actor-known projection policy dispatch and sleep accessibility resolution |
| `0021PHA3APOSREB-006` | `1a8e947` | Reject-loudly replay posture and shared crossing emitter |
| `0021PHA3APOSREB-007` | `d46bc29` | Typed place visibility and perception guard extension |
| `0021PHA3APOSREB-008` | `77c7163` | Event/state perimeter, applier totality, and mutator deletion |
| `0021PHA3APOSREB-009` | `31d5ebc` | Content integrity, validation negatives, and contract prose reconciliation |
| `0021PHA3APOSREB-010` | `0e4fecc` | Embodied sweep and provenance partition |
| `0021PHA3APOSREB-011` | `08cf76d` | Generative parity, fabricator ban, ordering, and tamper coverage |
| `0021PHA3APOSREB-012` | `e0ff5f6` | Window-credit helper, typed trace diagnostics, helper clippy lock, and INV-087 decision record |

The capstone adds this report and the read-only EMERGE-OBS refresh. It introduces no
production logic, event schema, simulation input, fixture behavior, or test oracle.

## Verification Commands

Observed on the finished tree:

| Command | Result |
|---|---|
| `cargo test -p tracewake-core --test emergence_ledger -- --nocapture` | Passed; emitted the refreshed `emerge_obs_v1` table below. |
| `rg -n "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src` | Exited 1; no production call sites or source references. |
| `cargo fmt --all --check` | Passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed. |
| `cargo build --workspace --all-targets --locked` | Passed. |
| `cargo test --workspace` | Passed. |

## 1. Rebind-After-Rejection

Primary ticket: `archive/tickets/0021PHA3APOSREB-001.md`.

Possession rebinds no longer transfer a prior actor's rejection why-not state or
notebook/debug truth into the new actor's embodied surface. The proof surface is the
TUI adversarial gates:
`adversarial_gates_possession_rebind_does_not_transfer_rejection_why_not`,
`rejection_report_must_match_viewer_before_embodied_projection_renders_it`, and
`renderer_marks_knowledge_context_frontier_non_diegetic`.

## 2. Scheduled Ratchet And Guard Outputs

Primary ticket: `archive/tickets/0021PHA3APOSREB-002.md`.

Hidden-truth gates now build actor-known contexts from applied event-log records
rather than fixture-local context fabricators or fabricated event ids. The scheduled
ratchet is represented by the widened hidden-truth/provenance source guards:
`guard_0021_hidden_truth_gates_use_event_log_provenance`,
`guard_0021_fabricated_visible_local_event_id_is_retired`, and
`guard_0021_actor_known_context_producers_are_projection_backed`.

## 3. Hidden-Truth Gate Rebuild And Visibility Demotion

Primary ticket: `archive/tickets/0021PHA3APOSREB-002.md`.

The hidden-truth harness rebuilds cognition from `EventLog` plus
`apply_epistemic_event`. Fixture-local visible-event id shortcuts are retired. The
visibility demotion path keeps debug-only truth out of holder-known contexts while
leaving debug panels explicitly non-diegetic.

## 4. Per-Arm Census

Primary ticket: `archive/tickets/0021PHA3APOSREB-003.md`.

`materialized_agent_apply_arms_require_payload_schema_version` derives checksum-covered
`AgentState` writes from `events/apply.rs`, handles rebinding shapes, and contains
synthetics for ungated sibling arms, retained payloads, and unlisted consumed keys.
The correction is source-derived rather than manually enumerated.

## 5. Mutation Perimeter And Rationale Split

Primary ticket: `archive/tickets/0021PHA3APOSREB-004.md`.

The mutation perimeter separates true guarded action-definition rationale from
non-perimeter action-code rationale. CI captures cargo-mutants status without
swallowing tool failure, scheduled/manual runs are not canceled by ordinary pushes,
and in-diff filters include the guarded Phase 3A action definitions.

## 6. Baseline Triage

Primary ticket: `archive/tickets/0021PHA3APOSREB-004.md`.

Mutation baseline misses are pinned by normalized count/hash and the disposition
ledger rejects unledgered misses, stale entries, unsupported tags, missing
`warrants-test` tickets, and bulk-repeated rationales. Retired entries and closed
tags are enforced by `mutation_baseline_misses_are_pinned_and_ledgered`.

## 7. Policy Dispatch And Sleep Accessibility

Primary ticket: `archive/tickets/0021PHA3APOSREB-005.md`.

Actor-known projection classification, freshness, accessibility-fact minting, and
consumer inclusion dispatch through `ActorKnownProjectionRecord::policy` and the
policy table. Food and sleep-place accessibility use the same remembered-place
mirror where appropriate; embodied current-place views retain their latest-current
scope.

## 8. Shared Crossing Emitter And Corrupt-History Rejection

Primary ticket: `archive/tickets/0021PHA3APOSREB-006.md`.

Sleep/work duration completions emit threshold crossings through the shared need
event emitter, malformed completion payloads return typed errors, and corrupt replay
histories fail loudly rather than panicking or silently applying defaults.

## 9. Typed Place Concealment

Primary ticket: `archive/tickets/0021PHA3APOSREB-007.md`.

Place visibility is authored through typed `VisibilityDefault`, not id/display-label
prose. The fixture schema requires the field, perception emission gates on the typed
value, and source guards reject label/id-based visibility logic.

## 10. Event/State Perimeter

Primary ticket: `archive/tickets/0021PHA3APOSREB-008.md`.

World and agent mutation paths are routed through event application and replayable
apply arms. The old direct mutator seam is deleted or capability-gated, derived
event-kind lists back totality guards, and replay rejects unsupported agent/world
event schema versions loudly.

## 11. Content Integrity And Contract Prose

Primary ticket: `archive/tickets/0021PHA3APOSREB-009.md`.

Content validation rejects duplicate and dangling references, no-sleep diagnostics
are typed, source causes serialize through stable canonical forms, and fixture
contract prose was reconciled with authored epistemic seeds.

## 12. Embodied Provenance And Dead-Surface Sweep

Primary ticket: `archive/tickets/0021PHA3APOSREB-010.md`.

Embodied availability provenance uses actor-visible facts rather than unfiltered
validator truth. The expanded two-sided sweep covers view-model option/collection
fields and TUI consumers, including `typed_leads` render coverage and the
`debug_available` disposition.

## 13. Generative Tier Deltas

Primary ticket: `archive/tickets/0021PHA3APOSREB-011.md`.

Generated no-human runs now flush due duration completions as scheduler time
advances, displacement precedes scheduled work completion in the generated block,
fabricator bans scan both the lock and support files, terminal tamper covers all
duration terminal kinds, and severe-band constants are range asserted.

## 14. Low-Group Closures And Deferrals

Primary ticket: `archive/tickets/0021PHA3APOSREB-012.md`.

Window completion credit uses one helper keyed by `contains_tick`, routine execution
selection uses one deadline-aware selector, legacy decision traces type absent
context hashes as absent, failed/legacy trace diagnostics derive from outcomes, eat
access failures keep unobservable distinctions in `absence_ancestry`, and blanket
known-food seeding is clippy-banned except for locally justified legacy fixtures.

INV-087 / `ORD-HARD-095` is recorded as a deferred owner decision in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`. This pass does not
amend `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`; any doctrine
clarification requires explicit owner approval.

## 15. Risk Register And Conformance Index

Conformance rows added or updated under
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` include:

| Row | Locked claim |
|---|---|
| `0021 possession-rebind rejection hygiene` | Rebinds cannot transfer rejection facts or unmarked global context into embodied actor surfaces. |
| `0021 harness-provenance fidelity` | Hidden-truth gates use modeled event-log provenance, not fabricated context inputs. |
| `0021 actor-known projection policy dispatch` | Projection records dispatch per-kind policy through a table and shared consumers. |
| `0021 INV-087 bind-time perception decision` | The possession/perception tension is explicitly deferred, with no silent constitutional edit. |

Risk-register entries `R-27` and `R-28` continue to govern this artifact: reachability
claims must name emitted-path evidence, and family corrections must enumerate sibling
surfaces or explicit exemptions.

## 16. EMERGE-OBS Refresh

Primary command:
`cargo test -p tracewake-core --test emergence_ledger -- --nocapture`.

The EMERGE-OBS ledger remains observer-only. The production-source grep for
`EmergeObs|emerge_obs|emergence_ledger` over all three crate `src/` trees exited 1.
This is measurement only: no thresholds, ratchets, or pass/fail semantics attach to
the counter values.

| Corpus row | Ticks | Contradictions | Replans/fallbacks | Interruptions | Intention switches | Stuck blockers | Belief divergences | Modeled corrections | Distinct kinds |
|---|---:|---:|---:|---:|---:|---|---:|---:|---:|
| `generative_seed_18000010` | 1-13 | 0 | 0 | 0 | 0 | none | 0 | 0 | 5 |
| `generative_seed_18000011` | 0-21 | 0 | 0 | 3 | 0 | none | 0 | 0 | 6 |
| `generative_seed_18000012` | 0-10 | 0 | 0 | 0 | 0 | none | 0 | 0 | 6 |
| `generative_seed_18000013` | 0-10 | 0 | 0 | 2 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000014` | 2-20 | 0 | 0 | 0 | 0 | none | 0 | 0 | 6 |
| `generative_seed_18000015` | 2-21 | 0 | 0 | 0 | 0 | none | 0 | 0 | 10 |
| `generative_seed_18000021` | 1-22 | 0 | 0 | 3 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000023` | 0-10 | 0 | 0 | 2 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000024` | 0-16 | 0 | 0 | 0 | 0 | none | 0 | 0 | 6 |
| `generative_seed_18000029` | 1-24 | 0 | 0 | 0 | 0 | none | 0 | 0 | 9 |
| `generative_seed_1800002a` | 1-7 | 0 | 0 | 1 | 0 | none | 0 | 0 | 7 |
| `generative_seed_18000057` | 0-13 | 0 | 0 | 0 | 0 | none | 0 | 0 | 7 |
| `total` | n/a | 0 | 0 | 11 | 0 | none | 0 | 0 | 13 |

The test also proves canonical event-log serialization/deserialization byte identity
for every row.

## 17. Explicit Non-Certification Boundary

This artifact is scoped evidence toward `ORD-LIFE-CERT` for spec `0021`. It records
current-tree evidence for the implementation commits listed above. It is not
full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`, and not a
claim that later, unrelated changes are covered by this report.

## Deviations From Plan

No production or test-logic changes were made for this capstone. The only artifact
created by this ticket is this report, and the only repository state changes still
pending are ticket archival/bookkeeping.

## Finished-Tree Gate Record

The finished tree for implementation commits `e1b94a1` through `e0ff5f6` was checked
with:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```
