# 0021PHA3APOSREB-004: Mutation-CI family — live scheduled ratchet, exclusion-channel guard, honest rationale split, real baseline triage, concurrency exemption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `.github/workflows/ci.yml`, `.cargo/mutants.toml` (if perimeter widens), `.cargo/mutants-baseline-misses.txt`, `tracewake-core` test oracle (perimeter/baseline guards), baseline disposition ledger, conformance-index mutation rows
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-067, 071, 072, 073, 097)

## Problem

The scheduled mutation job's baseline ratchet is dead code: its `cargo mutants
--workspace` step has no exit-status capture, the 143 accepted misses guarantee exit 2
every run, the step fails, and the "Check missed-mutant baseline" step (no
`if: always()`) never executes — the job is permanently red and new misses are
indistinguishable from accepted ones (`ORD-HARD-067`, high). Around it, four more
lock-durability gaps: `exclude_globs` is an unguarded perimeter bypass and the filter
checks are decoy-able text matches (`ORD-HARD-071`); the mutation-claiming
`CORE_ACTION_RATIONALE` decorates nine non-perimeter defs files — the `ORD-HARD-055`
defect family recurring (`ORD-HARD-072`); the baseline disposition ledger is a bulk
accept in per-entry costume — 137/143 entries share one templated phrase, zero entries
retired, no warrants-test dispositions, contradicting the 0020 ticket's own "no
bulk-accepted refresh" contract (`ORD-HARD-073`); and the CI concurrency group cancels
in-flight scheduled mutation runs (`ORD-HARD-097`). One ticket per spec §8: one honest
baseline refresh after the rationale split and any perimeter widening.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the scheduled job's run step has no
   `set +e`/`mutants_status=$?` (the only capture in `ci.yml` belongs to the in-diff
   job); the baseline-check step lacks `if: always()`; the perimeter guard
   (`mutation_perimeter_consistency_violations`, anti_regression_guards.rs) forbids
   only the literal `crates/tracewake-core/src/actions/defs/**` in `mutants.toml`,
   checks scheduled filters via whole-file `contains()`, selects the in-diff line as
   the first `grep -E` + `actions/defs/` match, and exactly parses only the defs
   alternation group; `CORE_ACTION_RATIONALE` decorates 12 classified defs files —
   the 9 non-perimeter: `accuseprobe.rs`, `checkcontainer.rs`, `continue_routine.rs`,
   `inspect.rs`, `mod.rs`, `movement.rs`, `openclose.rs`, `takeplace.rs`, `wait.rs`;
   the ledger (`reports/0020_mutants_baseline_disposition.md`) carries 143
   `justified-baseline` entries with 137 sharing "warrants a future focused
   assertion", and its guard checks rationale presence (`" — "`) only; the
   concurrency group is `ci-${{ github.workflow }}-${{ github.ref }}` with
   `cancel-in-progress: true` and no event-type exemption.
2. Verified against spec 0021 (reassessed 2026-06-11; every finding
   operator-verified, including the verbatim ticket-contract quotes at
   `archive/tickets/0020PHA3ACOGSUR-003.md` lines 107/141): findings
   ORD-HARD-067/071/072/073/097; §9 warns the triage may surface real test debt —
   budget for follow-up test tickets this time.
3. Cross-artifact boundary under audit: the mutation-gate contract spanning
   `ci.yml` (both jobs), `mutants.toml`, the baseline file, the disposition ledger,
   and the perimeter/baseline guards — one perimeter, loud failure semantics on every
   invocation, and a governed (not merely pinned) accepted-miss set.
4. Lock-durability doctrine restated (no single INV — the lineage enforcement
   reading + risk R-28): a correction is complete only when every surface in the
   defect class is corrected or explicitly exempted with a true rationale; a
   permanently-red gate emits no signal; an allowlist with templated rationales is a
   bulk accept regardless of its per-entry costume.
5. CI/test-oracle enforcement surface touched (the mutation gate guards the guarded
   layers' test strength): no production simulation code, replay, or actor-knowledge
   surface changes; deterministic replay unaffected. The perimeter-vs-true-rationale
   decision for `takeplace.rs`/`wait.rs`/`continue_routine.rs` is a spec-assigned
   implementer-recorded choice (spec §4 ORD-HARD-072, §9: "Either is acceptable; the
   silent third option is not") — options: bring the three large defs into the
   perimeter with an honestly refreshed baseline, or exempt each with a rationale
   that is true; the choice and its grounds are recorded in this ticket's closure
   note and the acceptance artifact.

## Architecture Check

1. Replicating the in-diff job's status-capture pattern in the scheduled job (treat
   exit 0/2 as ratchet input, other codes as tool failure) makes the existing `comm
   -23` baseline comparison the deciding signal — reviving the designed ratchet
   instead of inventing a new reporting channel. Glob-matching every `exclude_globs`
   entry against the canary paths, anchoring filter checks to the executing step, and
   evaluating the real in-diff regex against each canary path close the bypass
   *channels*, not just today's bypass strings. The closed disposition-tag enum
   (`justified-baseline` / `warrants-test:<ticket-id>`) with repetition bounds makes
   ledger substance checkable — the guard-vacuity lesson applied to the ledger.
2. No backwards-compatibility aliasing/shims: the dead ratchet path is made live, not
   paralleled; the false rationale string is split into true ones, not annotated.

## Verification Layers

1. Live ratchet (ORD-HARD-067) -> widened guard requiring status capture on every
   `cargo mutants` invocation and flagging `|| true`; synthetic
   removed-status-capture case must fail; first post-fix scheduled run (or local
   `--workspace` dry-run) demonstrates accepted-misses → green, new miss → red.
2. Exclusion channel + decoys (ORD-HARD-071) -> synthetic cases: an `exclude_globs`
   entry covering a perimeter path, a commented-out decoy filter line, a narrowed
   in-diff regex — each must fail the guard.
3. Rationale family (ORD-HARD-072) -> guard rule: any `Exempt` rationale containing
   "mutation" must name a path matched by the perimeter filters; synthetic violating
   case; the nine enumerated files re-rationalized (or perimeter-absorbed).
4. Ledger substance (ORD-HARD-073) -> guard: closed tag enum, ticket-ID required on
   `warrants-test`, repetition bound on identical rationale strings; synthetic
   bulk-accept case must fail.
5. Concurrency (ORD-HARD-097) -> guard check on the concurrency expression exempting
   `schedule`/`workflow_dispatch`.

## What to Change

### 1. Scheduled-job status capture (ORD-HARD-067)

`set +e` + `mutants_status=$?` in the scheduled run step; 0/2 feed the baseline
comparison, other codes fail as tool errors; ensure the baseline-check step executes
whenever mutants ran. Widen the perimeter guard to require the capture pattern on
every `cargo mutants` line and flag `|| true`.

### 2. Exclusion + decoy closure (ORD-HARD-071)

Glob-match `exclude_globs` entries against `MUTATION_PERIMETER_CANARY_PATHS`; anchor
scheduled-filter checks to the scheduled job's step block; evaluate the in-diff
regex against each canary path (extend the stem canary beyond the defs group).

### 3. Rationale split + perimeter decision (ORD-HARD-072)

Split `CORE_ACTION_RATIONALE` into a perimeter-true variant (eat/sleep/work) and an
honest non-perimeter variant; for `takeplace.rs`, `wait.rs`, `continue_routine.rs`
make the recorded perimeter-or-true-rationale choice (see Assumption 5); refresh the
baseline honestly if widened.

### 4. Real baseline triage + ledger governance (ORD-HARD-073)

Triage all 143 entries: guarded-layer decision-logic/interruption-predicate entries
become `warrants-test:<ticket-id>` with follow-up test-debt tickets authored in
`tickets/` (next free numbers in this namespace, `-014` onward); the rest get
individually-true rationales; retire entries where tests are written in this ticket's
scope. Harden the ledger guard (closed enum, ticket IDs, repetition bound).

### 5. Concurrency exemption (ORD-HARD-097)

Exempt `schedule`/`workflow_dispatch` events from the cancel-in-progress group.

### 6. Documentation

Update the mutation conformance rows (live ratchet, exclusion-channel guard,
disposition-tag governance) in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `.cargo/mutants.toml` (modify — only if the perimeter decision widens it)
- `.cargo/mutants-baseline-misses.txt` (modify)
- `reports/0020_mutants_baseline_disposition.md` (modify — or superseded by a `reports/0021_mutants_baseline_disposition.md` (new) with the guard repointed; record the choice)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `tickets/0021PHA3APOSREB-014.md` onward (new — test-debt follow-ups as the triage surfaces them; count implementation-discovered)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Census mechanics (ticket 0021PHA3APOSREB-003) — same guard file, different guard
  functions; coordinate the mechanical merge.
- Writing the surfaced test-debt *tests* themselves where they exceed this ticket's
  reviewable size — those land via the authored follow-up tickets.
- Generative-tier locks (ticket 0021PHA3APOSREB-011).

## Acceptance Criteria

### Tests That Must Pass

1. Widened perimeter/baseline guards pass; every synthetic negative (status-capture
   removal, `|| true`, exclusion glob, decoy comment line, narrowed regex, false
   mutation rationale, bulk-accept ledger) individually fails.
2. The nine enumerated non-perimeter files (`accuseprobe`, `checkcontainer`,
   `continue_routine`, `inspect`, `mod`, `movement`, `openclose`, `takeplace`,
   `wait`) each carry either a perimeter-true or an honestly-scoped rationale; the
   three large defs carry the recorded choice.
3. Ledger guard proves: closed tags, ticket IDs on every `warrants-test`, repetition
   bound holds; baseline↔ledger parity intact at the new pinned count/hash.
4. A scheduled-shape mutation run (local `--workspace` invocation acceptable as
   evidence) exits green on accepted misses only and red on an injected new miss.
5. `cargo test --workspace` green.

### Invariants

1. Every `cargo mutants` invocation in CI captures and dispositions its exit status;
   no mutation gate can be silenced by `exclude_globs`, decoy text, or an unledgered
   baseline append.
2. Every accepted miss carries an individually-true disposition under the closed tag
   enum; `warrants-test` entries reference real ticket files.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened perimeter guard,
   exclusion/decoy/regex canaries, ledger-governance guard, concurrency check, with
   synthetic negatives per layer.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards mutation -- --nocapture`
2. `cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/eat.rs' --no-shuffle -j 2` (focused evidence run; full set per the scheduled job's filters)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
