# 0045FIRPROCER-003: Set-union mutation-completion merger tool + synthetic-negative suite

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — a new mutation-completion merger tool plus its synthetic-negative test suite; no production/simulation logic change.
**Deps**: None

## Problem

The certifying mutation campaign (-005) will run as a deterministic matrix of `k/n` cargo-mutants shards (built in -004), because a single unsharded invocation timed out at 2,384 of 2,901 mutants in 0044 (spec §3.1, §9.5). A sharded run is only certifying if its shards provably reconstruct the exact configured population: shard assigned-sets pairwise disjoint, their union equal to the canonical configured set, every assigned identity carrying exactly one final outcome, and aggregate counts reconciling to the denominator (spec §4.7.4). A summary count that adds to 2,901 while identities are duplicated and others absent is the false-confidence failure mode the merger exists to prevent (spec §12 "Aggregate-count false confidence").

No such merger exists today: `tools/` contains only `supervise-command.sh` (verified this session). This ticket builds the merger and — critically — its own fail-closed negative suite, since the merger is the gate that decides whether a sharded run may contribute a passing row at all. The merger reconciles by **identity set membership, not summary arithmetic**, normalizing each cargo-mutants identity on a stable key (path | enclosing symbol | mutation operator/replacement | normalized mutant text), treating `:line:column` as advisory locator data because it drifts under legitimate edits (spec §4.7.1, §4.11).

This ticket builds and tests the merger against synthetic shard manifests; the live CI wiring that invokes it is -004 and the live run that feeds it is -005.

(spec §4.7.1, §4.7.4, §4.11, §5.4)

## Assumption Reassessment (2026-06-21)

1. No merger tool exists. Verified this session: `ls tools/` returns only `supervise-command.sh`. The merger is therefore a new tool; its negative-test harness is new. The repo convention for asserting on CI/tooling text from Rust is `crates/tracewake-core/tests/ci_workflow_guards.rs` (a Rust integration test) — the recommended default location for the merger's negative suite, with the merger logic itself as a `tools/` script alongside `supervise-command.sh`. **Implementer-recorded choice** (spec leaves merger command/version to the implementer, §5.4 "merger command/version"): record the selected language, path, and version in the completion manifest; candidate set is `tools/merge-mutation-shards.*` (logic) + `crates/tracewake-core/tests/mutation_completion_merge.rs` (negatives), or an equivalent self-contained tool with co-located tests.
2. The merger's input contract is the per-shard output already produced by `tools/supervise-command.sh` plus cargo-mutants' own output files. Verified this session: `supervise-command.sh` emits `status.env` with `supervisor_result` (used to reject any shard whose result is not a normal child exit), and cargo-mutants writes `caught.txt`/`missed.txt`/`timeout.txt`/`unviable.txt` (verified in the retained `reports/0044_first_proof_cert_mutation_full.out/mutants.out/`) plus structured `outcomes.json`/`mutants.json` where the pinned version supports it. The merger reads these; it does not require -002 or -004 to be testable, because its negatives run against synthetic manifests.
3. Cross-artifact boundary under audit: the set relation between the canonical configured identity set `C` (from `cargo mutants --list` at the final commit), each shard's assigned identity set `S_k`, and each shard's outcome identity set `O_k`. The merger must independently compute and prove `S_k == O_k` per shard, pairwise `S_i ∩ S_j == ∅`, `⋃ S_k == C`, `⋃ O_k == C`, `Σ|S_k| == |C|`, and `Σ(caught_k + missed_k + timeout_k + unviable_k) == |C|` (spec §4.7.4). cargo-mutants documents output formats as subject to change, so the merger pins the version and fails closed on unknown fields (spec §12 "Unreviewed output-format assumption").
4. Invariant/rule motivation: the execution mutation-completion / evidence-honesty doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) — an incomplete, canceled, wall-timed-out, internally failed, or missing shard cannot contribute a passing mutation row (spec §4.1 item 12) — and the exact-commit splice prohibition (spec §1.4, §12 "Commit/evidence splice"): the merger rejects mismatched config/commit/toolchain fingerprints across shards.
5. Enforcement surface: the merger is itself the fail-closed completeness gate — it must emit a non-zero result for any incomplete or dishonest state (missing/duplicate/overlapping shard, mismatched fingerprints, truncated/malformed output, non-normal supervisor result, text/JSON outcome-set disagreement). It touches no simulation-world fail-closed validation, actor-knowledge filtering, or deterministic-replay surface and introduces no epistemic-leakage path; its determinism is set-membership reconciliation over normalized identities (stable, order-independent), so identical shard inputs always yield the identical verdict.

## Architecture Check

1. A member-set merger with its own adversarial negative suite is cleaner and safer than trusting cargo-mutants' aggregate counts: counts can sum to the denominator while identities are duplicated and others absent. Reconciling by normalized identity set — with `:line:column` treated as advisory — makes the completeness claim survive legitimate source edits and makes every gap loud. Building the negative suite *with* the merger means the gate is proven to fail closed before it ever guards a real run.
2. No backwards-compatibility aliasing/shims. The merger pins the cargo-mutants version and validates the output schema rather than silently tolerating format drift; it does not paper over a missing shard with a count check.

## Verification Layers

1. Exact set reconciliation → tool self-test: against a synthetic complete shard set, the merger proves pairwise disjointness, `⋃ S_k == C`, `S_k == O_k` per shard, and both count equations, and exits zero.
2. Fail-closed on every dishonest state → synthetic-negative tests (one per failure mode): missing shard, duplicate shard index, overlapping identity across shards, mismatched commit fingerprint, mismatched config/toolchain fingerprint, truncated/malformed structured output, and a shard whose `supervisor_result` is not a normal child exit each make the merger exit non-zero with a specific diagnostic.
3. Identity normalization stability → tool self-test: two manifests differing only in `:line:column` for the same `path | enclosing symbol | operator | normalized text` reconcile as the same identity (advisory locator drift does not break the union).
4. No simulation surface touched → single-layer note: per item 5, no replay/golden-fixture/actor-knowledge mapping applies; verification is the merger's own self-tests and negatives.

## What to Change

### 1. Build the set-union completion merger

Author a merger (location per the item-1 implementer-recorded choice) that, given the canonical configured `--list` population and a set of shard output trees, independently computes `C`, each `S_k`, and each `O_k`; normalizes every identity on the stable key `path | enclosing symbol/function | mutation operator/replacement | normalized mutant text` (retaining `:line:column` and any structured internal name/diff as recorded-but-not-sole-identity); and proves the spec §4.7.4 relations. It pins and records the cargo-mutants version and fails closed on unknown/extra structured-output fields.

### 2. Emit the completion manifest (human + machine readable)

On success, the merger writes the `reports/0045_first_proof_cert_mutation_completion_manifest.md` (explanatory) and `…_completion_manifest.json` (member sets + fingerprints) shapes per spec §5.4 — final SHA, canonical fingerprint/count, shard denominator `N` and indices received, per-shard command/config/toolchain fingerprints, per-shard assigned/outcome set fingerprints and counts, pairwise-intersection result, union-equality result, required-empty missing/duplicate lists, per-shard supervisor + cargo-mutants results, aggregate `caught`/`missed`/`timeout`/`unviable` counts, the count equation, malformed/truncated checks, output-tree + artifact checksums, merger command/version, and certification disposition. (The live manifest is produced by -005; this ticket produces it over synthetic inputs to prove the emitter.)

### 3. Build the synthetic-negative suite

Add the merger's own unit/integration negatives covering, at minimum: synthetic missing-shard, duplicate-identity, mismatched-commit, mismatched-config, truncated-JSON, and overlapping-shard (spec §5.4), plus a non-normal `supervisor_result` shard and a text-vs-JSON outcome-set disagreement. Each negative asserts a non-zero merger result and a specific diagnostic.

## Files to Touch

- `tools/merge-mutation-shards.sh` (new — merger logic; exact name/language per the item-1 implementer-recorded choice, recorded in the completion manifest)
- `crates/tracewake-core/tests/mutation_completion_merge.rs` (new — synthetic-negative suite; location per the item-1 implementer-recorded choice, recommended to mirror `ci_workflow_guards.rs`)

## Out of Scope

- The in-diff trigger / CI guard convergence (→ -001).
- The supervisor retention contract and completion diagnostic (→ -002).
- Wiring the merger into a CI reconciliation job and the sharded matrix (→ -004).
- Running the live campaign and producing the real completion manifest, census, and triage register (→ -005).
- Live FIRST-PROOF re-proof and the replacement acceptance artifact (→ -006).
- Any change to `.cargo/mutants.toml` or `.cargo/mutants-baseline-misses.txt` (the denominator is preserved).

## Acceptance Criteria

### Tests That Must Pass

1. The merger reconciles a synthetic complete shard set to the canonical set (disjoint union, `S_k == O_k`, both count equations) and exits zero, emitting both completion-manifest shapes.
2. Each synthetic negative — missing shard, duplicate shard index, overlapping identity, mismatched commit, mismatched config/toolchain, truncated/malformed output, non-normal supervisor result, text-vs-JSON disagreement — makes the merger exit non-zero with a specific diagnostic.
3. An identity differing only in `:line:column` reconciles as the same normalized identity.
4. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass.

### Invariants

1. The merger reconciles by normalized identity-set membership; aggregate counts are a secondary cross-check, never the proof.
2. The merger fails closed (non-zero) on any incomplete, overlapping, mismatched-fingerprint, truncated, or supervisor-failed shard state — a summary count without member-set equality can never satisfy it.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/mutation_completion_merge.rs` (new) — the synthetic complete-set reconciliation plus one negative per failure mode (missing-shard, duplicate-identity, overlapping-shard, mismatched-commit, mismatched-config, truncated-JSON, non-normal-supervisor, text-vs-JSON disagreement) and the line/column-advisory normalization case.

### Commands

1. `cargo test --locked -p tracewake-core --test mutation_completion_merge` — runs the merger's complete-set and negative cases. (New target created by this ticket; it cannot be `--list`-dry-run before creation — that is the stated reason it is not pre-listed.)
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. A direct merger invocation over a checked-in synthetic-fixture shard set (the narrower verification boundary), asserting exit zero on the complete set and non-zero on each tampered fixture — exact command per the recorded merger CLI.

## Outcome

Completed: 2026-06-21

Implemented the merger as `tools/merge-mutation-shards.py` (Python 3,
version label `0045FIRPROCER-003`) and recorded that language/path choice here.
The CLI accepts a canonical configured mutant list, repeated shard directories,
expected shard count, commit/config/toolchain fingerprints, and output paths for
both manifest shapes:

```text
python3 tools/merge-mutation-shards.py \
  --canonical <canonical-mutants.json> \
  --expected-shards <N> \
  --commit <sha> \
  --config-fingerprint <fingerprint> \
  --toolchain-fingerprint <fingerprint> \
  --out-md <completion_manifest.md> \
  --out-json <completion_manifest.json> \
  --shard <shard-dir> ...
```

Each shard directory is expected to contain `shard.env`, `status.env`,
`assigned-mutants.json`, and `mutants.out/outcomes.json` plus the four text
outcome files. The merger normalizes identities on path, enclosing function,
mutation genre, replacement, and normalized diff text while treating
`line:column` as advisory. It emits both human and machine manifests on success
and fails closed on missing/duplicate shards, overlapping identities,
mismatched commit/config/toolchain fingerprints, malformed JSON, non-normal
supervisor status, assigned/outcome mismatch, text/JSON disagreement, and any
final missed/timeout survivor floor.

Added `crates/tracewake-core/tests/mutation_completion_merge.rs`, which
generates synthetic cargo-mutants-like shard fixtures at test time and invokes
the checked-in merger CLI. The suite covers a complete successful union,
line/column-only drift reconciliation, and synthetic negatives for missing
shard, duplicate shard index, overlapping identity, mismatched commit,
mismatched config, mismatched toolchain, truncated JSON, non-normal supervisor,
text-vs-JSON disagreement, and survivor floor.

Verification run:

- `cargo test --locked -p tracewake-core --test mutation_completion_merge` — passed.
- `python3 -m py_compile tools/merge-mutation-shards.py` — passed; generated `tools/__pycache__/` was removed afterward.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace --locked` — passed.
- `git diff --check` — passed.

No production/simulation logic, mutation denominator, CI workflow, or baseline
miss file was changed.
