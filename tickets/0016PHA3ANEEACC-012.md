# 0016PHA3ANEEACC-012: Lock-layer durability — workspace census, fixture census, clippy bans, mutants baseline

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — test-oracle layer only: workspace-wide guard census, negative-fixture/clippy parity censuses, `clippy.toml` additions, `cargo-mutants` configuration + CI job; no production simulation logic
**Deps**: `archive/tickets/0016PHA3ANEEACC-006.md`, `archive/tickets/0016PHA3ANEEACC-008.md`, 0016PHA3ANEEACC-009, 0016PHA3ANEEACC-010, 0016PHA3ANEEACC-011

## Problem

ORD-HARD-025: the anti-contamination lock layer itself is not durable (no doctrine breached directly; lock-layer durability continues ORD-HARD-013):

- `anti_regression_guards.rs::production_sources()` roots at `tracewake-core/src` only and `is_guarded_layer_source` covers `src/agent/`, `src/scheduler*`, `src/projections*` — cognition logic relocated to a sibling file, another crate, or a new crate escapes every targeted rule; the census only fails for unclassified files *inside* the three globs.
- Token bans are source-text scans defeated by import aliasing, wrapper functions in unguarded files, and macro indirection; the `production()` cfg-stripper recognizes only literal `#[cfg(test)]`.
- Several positive-presence guards assert substring presence (e.g. the transaction audit-gate guard) — a comment satisfies them; not all have runtime backstops.
- `negative_fixture_runner.rs::FIXTURES` is hand-maintained; nothing reconciles it against the repo-root `tests/negative-fixtures/` directory (25 == 25 today by discipline); no parity exists between `clippy.toml` ban entries and proving fixtures.
- `clippy.toml` lacks `rand` entry points, `std::fs::write`/`OpenOptions`, `std::env::var`; content/tui are uncovered for nondeterminism classes.

This ticket runs last by design (spec §8): the census and mutants baseline lock the *finished* 0016 surface.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `production_sources()` roots at core `src/` (`anti_regression_guards.rs:231–268`); `is_guarded_layer_source` globs at :331–334; `assert_absent` text scans at :179–184; literal-`#[cfg(test)]` stripper at :136–177; substring positive-presence guard example at :1715 (`transaction.contains("!selection.trace.hidden_truth_audit_result.actor_known_only")` — runtime backstop arrives with `archive/tickets/0016PHA3ANEEACC-005.md`'s ordering test); `FIXTURES` 25 entries (`negative_fixture_runner.rs:11–112`) vs 25 directories on disk at repo-root `tests/negative-fixtures/`, no census; `clippy.toml` bans HashMap/HashSet/SystemTime/Instant/f32/f64/thread::spawn/process::Command/fs-reads/net but not `rand`, `std::fs::write`, `std::fs::OpenOptions`, `std::env::var`; CI is `.github/workflows/ci.yml` (fmt/clippy/test/lock-layer jobs on every `pull_request`).
2. Spec/docs: spec 0016 §ORD-HARD-025 (six-part required correction) and §5 (consolidated lock tiers); external research (cargo-mutants, dylint) is consulted-not-authority. The mutants baseline is a mandatory acceptance input for `reports/0016_ord_life_cert_scoped_acceptance.md` (spec §7 item 7 — ticket 0016PHA3ANEEACC-014 consumes it).
3. Shared boundary under audit: the lock layer's own perimeter — which files the guards see, which fixtures the runner runs, which bans clippy enforces, and whether any of it would notice its own erosion. The census tables become the contract: tree state ⇄ classification, fixtures-on-disk ⇄ `FIXTURES`, clippy entries ⇄ proving fixtures.
4. Fail-closed test-oracle surface: every census is a blocking test (a new unclassified production file, an unregistered fixture, or an unproven clippy ban fails the suite). This ticket adds detection, not simulation behavior — no replay, checksum, or epistemic surface changes; the guarded layers' *content* was finished by the upstream tickets, which is why the leaf-set `Deps` covers the whole batch transitively.
5. Adjacent contradictions: the spec's correction 2 (type-level boundaries over text bans) is *delivered by* tickets 004/005 (witness types, computed accessors, sealed constructors); this ticket's contribution is keeping the residual text scans as documented tripwires — their known evasion modes (import aliasing, wrapper indirection, non-literal cfg forms) recorded inline — rather than pretending they are airtight. Classified as a required consequence split across tickets by surface ownership.

## Architecture Check

1. A census that must equal the actual tree converts "the guards cover the right files" from a reviewer's memory into a failing test: relocating cognition out of a guarded glob now forces a reviewed reclassification instead of silently exiting the perimeter. Mutation testing is the only measure of whether the lock layer *detects* anything — a guard that kills no mutants is theater; running it as a non-PR scheduled/manual job keeps PR latency intact while making lock effectiveness a recorded, comparable number across hardening passes (spec §9 explicitly justifies this as making the anticipated fifth audit cheaper).
2. No backwards-compatibility aliasing/shims: the census replaces the partial-glob perimeter; no "legacy unclassified" category exists — every production file across all three crates is guarded-layer or exempt-with-recorded-rationale.

## Verification Layers

1. Census perimeter → workspace census test: every production source file across `tracewake-core`, `tracewake-content`, `tracewake-tui` is classified; the table equals the tree.
2. Fixture-registry integrity → negative-fixture census: `read_dir` of repo-root `tests/negative-fixtures/` (resolved via `CARGO_MANIFEST_DIR/../..` from the core test crate) equals `FIXTURES` exactly.
3. Ban-proof parity → clippy parity test: every `clippy.toml` `disallowed-types`/`disallowed-methods` entry has at least one proving negative fixture.
4. Lock effectiveness → `cargo mutants` baseline over the guarded layers (`agent/**`, `scheduler*`, `projections*`, `pipeline.rs`): caught/missed counts recorded; misses on guard-relevant code become accepted-risk entries or new tests (dispositioned in the acceptance artifact).

## What to Change

### 1. Workspace-wide census with classification

Extend `anti_regression_guards.rs`: a classification table (guarded-layer / exempt-with-recorded-rationale) covering every production source file in all three crates, asserted equal to the actual tree. Document the retained text scans' known evasion modes inline where each scan is defined.

### 2. Negative-fixture and clippy parity censuses

The `read_dir`-equals-`FIXTURES` census; the clippy-entry ⇄ proving-fixture parity test (adding minimal proving fixtures for entries that lack one).

### 3. `clippy.toml` additions + cross-crate nondeterminism coverage

Add `rand::*` entry points, `std::fs::write`, `std::fs::OpenOptions`, `std::env::var`; extend the banned-token scan (or clippy config) to `tracewake-content` and `tracewake-tui` production sources for the nondeterminism classes.

### 4. Mutants baseline

A `cargo-mutants` configuration (`.cargo/mutants.toml`) scoped to the guarded layers; a non-PR scheduled/manual CI job in `.github/workflows/ci.yml`; the baseline run executed once and its caught/missed counts + per-miss dispositions recorded for ticket 014's acceptance artifact.

### 5. Runtime backstops

Audit the positive-presence guards: where one has no runtime backstop, add one or replace the string check with the runtime test (the audit-gate guard's backstop is 005's ordering test — cross-check the remainder).

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `clippy.toml` (modify)
- `.github/workflows/ci.yml` (modify — scheduled/manual mutants job)
- `.cargo/mutants.toml` (new — cargo-mutants scope configuration)
- `tests/negative-fixtures/` (new entries as required by the clippy parity test, as surfaced)

## Out of Scope

- The type-level boundary conversions themselves (`archive/tickets/0016PHA3ANEEACC-004.md`/`archive/tickets/0016PHA3ANEEACC-005.md` — already landed upstream of this ticket).
- Any production simulation logic change (this ticket is test-oracle/CI only).
- The acceptance artifact assembly (0016PHA3ANEEACC-014 — consumes this ticket's census inventory and mutants baseline).

## Acceptance Criteria

### Tests That Must Pass

1. Workspace census: zero unclassified production files across all three crates; relocating any guarded-layer file fails the suite until reclassified.
2. Fixture census and clippy parity: `FIXTURES` equals the directory exactly; every clippy ban entry has a proving fixture.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`; the mutants baseline run completes with recorded counts and per-miss dispositions.

### Invariants

1. The lock layer's perimeter is itself locked: file relocation, fixture deregistration, and unproven bans are all blocking test failures.
2. Lock effectiveness is measured (mutants baseline recorded), never assumed.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — workspace census + evasion-mode documentation + runtime-backstop audit.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — directory census + clippy parity test.
3. `.cargo/mutants.toml` + CI job — the measured-lock infrastructure (validation by executing the baseline run).

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards && cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' --no-shuffle` (baseline; exact invocation finalized against the installed cargo-mutants version)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
