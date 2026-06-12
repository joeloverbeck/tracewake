# 0022PHA3ABASTRI-003: Mutation-CI bypass-channel closure and perimeter parity

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — CI workflow (`.github/workflows/ci.yml`), mutation config (`.cargo/mutants.toml`), test-oracle guards (`anti_regression_guards.rs`)
**Deps**: `specs/0022_PHASE_3A_BASELINE_TRIAGE_INTEGRITY_EMBODIED_DEBUG_QUARANTINE_ADVERSARIAL_GATE_RESTORATION_AND_LOCK_BEHAVIORAL_PROOF_HARDENING_SPEC.md`

## Problem

Three mutation-CI residue findings: a push to `main` cancels an in-flight scheduled
mutation baseline run because the concurrency group omits `github.event_name`
(`ORD-HARD-100`); the perimeter guards remain bypassable — `exclude_re`/`examine_re`
config keys are unchecked, `simple_glob_matches` fails open on mid-pattern wildcards
(`**/eat.rs`), capture counting runs over the raw file (comment decoys balance a gutted
step), the swallow rule catches only literal `|| true`, and the in-diff filter check
evaluates the first matching line (`ORD-HARD-101`); and `mutants.toml`'s comment claims
local/CI perimeter parity while `exclude_globs` omits the non-perimeter defs — the
post-audit `-014` run mutated `wait.rs` for exactly this reason (`ORD-HARD-118`).

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `concurrency.group: ci-${{ github.workflow }}-${{ github.ref }}`
   with `cancel-in-progress` exempting only the *incoming* schedule/dispatch events
   (`.github/workflows/ci.yml`); no `exclude_re`/`examine_re` handling anywhere in
   `anti_regression_guards.rs` (`parse_exclude_globs` is the sole mutants.toml
   inspection); `simple_glob_matches` recognizes only exact, `/**`-suffix, and
   `*`-suffix forms and returns `false` otherwise;
   `status_captures = ci_yml.matches("mutants_status=$?").count()` counts the raw file;
   `.cargo/mutants.toml` `exclude_globs` lists content/tui crates and core
   non-action modules but no `actions/defs/` non-perimeter file.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-100`/`101`/`118` (all
   operator-verified) and `archive/tickets/0021PHA3APOSREB-014.md`'s doctrine note,
   which records the `wait.rs` perimeter leak and explicitly defers the correction to
   `ORD-HARD-118`.
3. Shared contract under audit: the mutation-CI lock layer — `ci.yml` job semantics ↔
   `mutants.toml` config ↔ `MUTATION_PERIMETER_CANARY_PATHS` and
   `mutation_perimeter_consistency_violations` in `anti_regression_guards.rs`.
4. Constitutional motivation restated: INV-017's auditability posture extended by the
   lineage's lock-durability rule — a gate whose run can be silently cancelled, or
   whose config has an unchecked bypass channel, enforces nothing (R-29).
5. This ticket touches fail-closed CI validation surfaces: every new check must fail
   closed (unknown mutants.toml keys rejected from a closed allowlist; unrecognized
   glob shapes are violations, not non-matches); no replay or epistemic surface is
   touched.
6. Either-or resolution (recorded): `ORD-HARD-118` offered derive-the-exclusion-set vs
   correct-the-comment; the decomposition applies the recommended derive option — the
   exclusion set is derived from the perimeter classification table, with the parity
   check as its lock (Step-4 Q4 default, announced at approval).
7. Change rationale (no silent retcon): the `ORD-HARD-097` concurrency exemption and
   `ORD-HARD-071` filter checks landed correct-but-incomplete; this ticket closes their
   named residue per the R-28 family-closure rule.

## Architecture Check

1. Fail-closed key allowlisting (only `exclude_globs` + `additional_cargo_args`
   recognized) beats per-key blocklisting — a future cargo-mutants config key cannot
   silently open a new channel. Deriving local exclusions from the classification table
   makes local/CI parity structural rather than commented.
2. No backwards-compatibility aliasing/shims: the glob matcher's unrecognized shapes
   become violations, not silently-tolerated patterns.

## Verification Layers

1. Scheduled-run survivability (`ORD-HARD-100`) -> codebase grep-proof: the
   `concurrency:` block includes `github.event_name` in `group`; guard parses the block
   and a synthetic negative removing it fires.
2. Config-channel closure (`ORD-HARD-101a`) -> guard check: unknown mutants.toml keys
   fail from a closed allowlist; synthetic `exclude_re` case fires.
3. Glob fail-closure (`ORD-HARD-101b`) -> guard check: any exclude glob containing `*`
   outside the three recognized forms is itself a violation; synthetic `**/eat.rs`
   case fires.
4. Capture/decoy closure (`ORD-HARD-101c`) -> guard check: captures counted over
   `non_comment_lines` scoped per invocation step block; any `||`/`&&` suffix on a
   `cargo mutants` line flagged; in-diff filter check anchored to the
   `git diff --name-only` line; synthetic comment-capture, `|| echo ok`, and
   uncommented-decoy cases fire.
5. Local/CI parity (`ORD-HARD-118`) -> guard check: `exclude_globs` derived-set parity
   against the perimeter classification; the `-014`-documented `wait.rs` leak closes.

## What to Change

### 1. `ci.yml` concurrency isolation

Add `github.event_name` to the concurrency `group` expression so scheduled/dispatch
runs occupy their own group and ordinary pushes cannot cancel them.

### 2. Harden `mutation_perimeter_consistency_violations` and helpers

(a) Closed mutants.toml key allowlist — any unrecognized top-level key is a violation.
(b) `simple_glob_matches` fails closed: an exclude glob containing `*` not in the
exact / `/**`-suffix / `*`-suffix forms is a violation. (c) Count `mutants_status=$?`
captures over `non_comment_lines` scoped to each step block containing a
`cargo mutants` invocation; flag any `||` or `&&` suffix on such lines. (d) Anchor the
in-diff filter check to the line containing `git diff --name-only` (or assert exactly
one matching line). (e) Parse the `concurrency:` block and assert `event_name` appears
in `group`. Synthetic negatives for every new rule, routed through the production scan
paths.

### 3. Derive local exclusion parity in `mutants.toml`

Extend `exclude_globs` to cover the non-perimeter action defs (derived from the
perimeter classification table the guards already hold), and add the parity check:
the derived exclusion set and the classification table must agree; correct the file's
comment to describe the derived contract.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `.cargo/mutants.toml` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- The baseline-ledger triage and governance rules (`0022PHA3ABASTRI-001`).
- The two-sided baseline ratchet (`0022PHA3ABASTRI-004`).
- The acceptance-report §7-item-2 scheduled-run record (`0022PHA3ABASTRI-002`).
- Widening or narrowing the mutation perimeter membership itself (the classification
  stands; this ticket makes its local/CI encoding parity-checked).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test anti_regression_guards` — all new rules green
   on the real tree; each synthetic negative (`exclude_re` key, `**/eat.rs` glob,
   comment-only capture, `|| echo ok` swallow, uncommented decoy line, concurrency
   `event_name` removal) fails its guard.
2. `grep -n "event_name" .github/workflows/ci.yml` shows it inside the concurrency
   `group` expression.
3. The parity check proves `exclude_globs` ∪ perimeter classification covers all
   `crates/tracewake-core/src/actions/defs/*.rs` with no file in both sets.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every `cargo mutants` invocation in `ci.yml` has step-scoped status capture with no
   shell suffix that can swallow its exit code.
2. `mutants.toml` carries no key outside the closed allowlist; unrecognized glob
   shapes are build failures, not silent non-matches.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — hardened perimeter guard
   rules + six synthetic negatives.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo mutants --list -f crates/tracewake-core/src/actions/defs/wait.rs --no-shuffle | head -5` (expect: excluded after parity derivation — empty selection)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
