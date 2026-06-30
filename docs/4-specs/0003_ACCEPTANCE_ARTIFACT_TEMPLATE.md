# 0003 Acceptance Artifact Template

Use this template for implementation review of
`archive/specs/0010_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md`.

## Exact commit under test

- Commit: `<full git sha>`
- Branch or PR: `<branch-or-pr>`

## Gates run

Record the command, result, and concise output summary for each gate:

- `cargo fmt --all --check` — `<result and output summary>`
- `cargo clippy --workspace --all-targets -- -D warnings` — `<result and output summary>`
- `cargo build --workspace --all-targets --locked` — `<result and output summary>`
- `cargo test --workspace` — `<result and output summary>`

## Changed files

List each changed file that belongs to the implementation under review. Exclude
unrelated local worktree changes.

## Computed acceptance status

Artifacts that claim closure over protected authority seams must include exactly
one fenced `tracewake-acceptance-status` block. The block is a computed read
model over current evidence artifacts and the expected-finding manifest; it is
not free-form certification prose. The artifact's human verdict must use only
the closed grammar below and must not claim more than the computed block
supports:

- `pass` — every required finding is `closed`, the artifact under review was
  ingested, independent acceptance is proven where required, mutation evidence
  is current and green with denominator/caught/unviable/missed/timeout counts,
  baseline reconciliation is current, and no survivor or pending row remains.
- `non-pass` — any required finding is `open`, `routed-forward`,
  `pending-governance`, `historical-only`, or `not-in-scope`; governance
  independence is absent or zero-approval/status-checks-only; actual-artifact
  ingestion is missing; mutation evidence is stale, trigger-only, survivorful,
  timed out, or unreconciled; or certifying evidence is only sampled,
  observer-only, historical, self-authored, display-only, or topology-only.

Allowed summary wording must say either `Computed result: pass` or
`Computed result: non-pass`, followed by the specific blocker or exact evidence
role. Do not use open-ended prose such as "accepted", "certified", "green", or
"closed" unless the computed block supports that exact status for the exact
commit under test.

When the governance posture is `solo-maintainer-compensating-control`, the
computed block must include the full field set that proves the posture:

- `governance_posture: solo-maintainer-compensating-control`
- `required_approving_review_count: 0`
- `required_status_checks: <all required standing checks and current result>`
- `mutation_in_diff_lock_layer: <present and green|not present>`
- `bypass_actors: []`
- `current_user_can_bypass: never`
- `non_fast_forward_protection: true`
- `deletion_protection: true`
- `strict_required_status_checks_policy: true`
- `ruleset_api_transcript: <path, digest, or inline evidence summary>`

Missing or historical ruleset evidence, bypass actors, mutable bypass by the
current user, absent standing checks, stale check results, or branch-policy
topology without an API/ruleset transcript makes the computed result
`non-pass`.

## Parity evidence block

For any feature with TUI-facing simulation impact, record the playable-capability
parity evidence. If the feature has no parity impact, state why.

- Target implementation commit: `<full git sha>`
- Fixture/content fingerprints: `<fixture ids, content fingerprints, or not applicable>`
- Capability entries in scope: `<added/changed playable capabilities and non-playable classifications>`
- Generated coverage report: `<report path, digest, or summary>`
- Typed causal witnesses: `<event/proposal/validation/replay identifiers>`
- Actor-known witnesses: `<holder, context/provenance ids, positive and negative evidence>`
- Rendered golden paths/digests: `<checked-in render references and digests>`
- Anti-leak and debug-quarantine evidence: `<negative evidence and debug/embodied separation>`
- Replay/no-human disposition: `<required evidence, not applicable, or deferred with owner>`
- Compiler/source-conformance evidence: `<compile/source guard evidence>`
- Exact commands and verdicts: `<commands run and pass/fail result>`

This block must not reduce parity acceptance to screenshots, display strings, or
manual visual inspection. Rendered evidence is useful only when paired with
typed causal and actor-filtered witnesses.

## Per-requirement acceptance evidence

Each requirement row must cite one or more evidence item IDs from the evidence
item ledger below. Compute the requirement result only from certifying evidence
items. A row may carry useful pending, sampled, observer-only, or historical
evidence while the requirement result remains pending or failed.

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `SPINE-AC-001` | `core/state`, `core/events` | private authoritative fields, mutation capability guards, named tests | `<pass/fail/pending>` |
| `SPINE-AC-002` | `core/replay`, `core/events` | schema-version registry and loud unknown-version replay test | `<pass/fail/pending>` |
| `SPINE-AC-003` | `core/events`, `core/replay` | total event-kind metadata and non-world checksum guard | `<pass/fail/pending>` |
| `SPINE-AC-004` | `core/replay` | authoritative checksum coverage manifest test | `<pass/fail/pending>` |
| `SPINE-AC-005` | `workspace/ci` | nondeterminism banned-API source scan and clippy gate | `<pass/fail/pending>` |
| `SPINE-AC-006` | `core/scheduler` | no-direct-dispatch scheduler source/runtime guards | `<pass/fail/pending>` |
| `SPINE-AC-007` | `core/actions` | source-context binding adversarial tests | `<pass/fail/pending>` |
| `SPINE-AC-008` | `core/actions`, `core/events` | append-before-apply and no alternate accepted mutation path tests | `<pass/fail/pending>` |
| `SPINE-AC-009` | `core/agent`, `tui/debug` | actor-known/debug capability unforgeability and TUI debug quarantine tests | `<pass/fail/pending>` |
| `SPINE-AC-010` | `content/schema`, `content/validation` | content field registry, canonical serialization, and prose-born fact rejection tests | `<pass/fail/pending>` |
| `SPINE-AC-011` | `core/actions` | typed diagnostic and actor/debug provenance tests | `<pass/fail/pending>` |
| `SPINE-AC-012` | `tui/view-model`, `tui/debug` | stale-view, debug command, direct-apply, and transcript snapshot tests | `<pass/fail/pending>` |
| `SPINE-AC-013` | `workspace/ci` | named spine/content/TUI conformance suites map every requirement to named evidence | `<pass/fail/pending>` |
| `SPINE-AC-014` | `workspace/ci` | doc/invariant reference linter under `cargo test` | `<pass/fail/pending>` |
| `SPINE-AC-015` | `workspace/ci` | this scoped acceptance artifact with file/line evidence | `<pass/fail/pending>` |

## Evidence item ledger

For every cited evidence item, add a row or block with the fields below. These
fields make execution `10`'s evidence-status and fingerprint-scope honesty rule
visible in this packet; they do not define a new gate, obligation, invariant,
or status vocabulary.

- Evidence item ID: `<stable local ID>`
- Requirement IDs: `<requirements supported>`
- Evidence status: `<pass|fail|pending|sampled|observer-only|historical>`
- Fingerprint scope, when a fingerprint/checksum/snapshot is cited:
  `<raw bytes|normalized serialization|parsed semantic content|command transcript|run seed|replay artifact|not applicable>`
- Evidence summary: `<command, artifact, file/line, report section, or replay reference>`
- Path under test and behavior witness, for behavioral pass claims:
  - path under test;
  - command, event, trigger, emitter, or scheduler entry that exercised it;
  - responsible layer;
  - accepted/rejected action or validation stage witnessed;
  - live negative, mutation-style failure, or reason no negative is applicable;
  - checked facts or invariants the witness supports.
- Replay/provenance ancestry, for behavioral or derived claims:
  - event-log segment or event identifiers;
  - replay artifact or serialized-log reference;
  - seed, randomness, content version, or ruleset version where applicable;
  - extraction/projection version for derived evidence;
  - source provenance for any claim crossing from artifact to semantic behavior.
- Sampling/exhaustiveness scope:
  `<exhaustive perimeter>` or `<sampled population, sample basis, omitted cases, and why representative>`.
- Pending or historical handling:
  pending rows name the missing proof and owner/blocker; historical rows state
  whether the artifact is context, lineage, or archived precedent.
- Certification use:
  `<counted as certifying pass|counted as certifying fail|not counted as certifying evidence>`.

Pending, sampled, observer-only, and historical evidence cannot be silently
counted as a pass. A fingerprint must not be cited beyond its actual scope.
`EMERGE-OBS` and other observer-only emergence evidence must be labeled
`status = observer-only`; those rows may inform review but must not become
certification, gate pass/fail thresholds, scheduler objectives, scenario goals,
or code-quality substitutes unless a future upstream spec changes that doctrine.

## Staged-abstraction declaration

When an acceptance artifact relies on a bounded staged abstraction, add a
declaration that states:

- what the artifact proves now;
- what the artifact deliberately abstracts;
- what the implementation or proof must not fake while using that abstraction;
- what future feature or doctrine tier the abstraction must not block;
- what evidence prevents overclaiming from the current artifact; and
- what failure diagnostics distinguish "not implemented yet",
  "intentionally abstracted", "implemented but broken", and "overclaimed".

This declaration certifies nothing by itself. Its fields are observer-only and
non-certifying unless a future scoped spec promotes a specific check through the
normal authority chain. The declaration remains subordinate to
`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
and the reference staged-abstraction terms and risk memory enacted by archived
spec `0034`; it does not define a gate, obligation, status enum, fingerprint
scope, temporal value, schema, threshold, or fixture name.

## Residual convention-only items

List any remaining item that cannot be structurally or mechanically enforced in
Rust tests. For each item, state why it remains review-required and name the
review surface that carries it.

## Scoped certification wording

Allowed wording:

- "Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit `<commit>`. This contributes scoped evidence toward `SPINE-CERT`, `EPI-CERT`, and `P0-CERT`; it does not certify latest main, later-phase scope, or the full project."

Forbidden wording:

- "Tracewake is fully certified."
- "Latest main was independently verified."
- "Later Phase 2+ / Phase 3A+ systems are certified by this pass."
- "Archived specs are live authority."
- "Project is P0 certified."
- "SPINE-CERT passed."

Do not use the forbidden wording as a result claim unless a separate upstream
certification process declares that outcome.
