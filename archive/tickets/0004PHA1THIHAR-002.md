# 0004PHA1THIHAR-002: Negative-fixture runner harness + banned-API clippy fixtures

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new negative-fixture runner harness (test target) + isolated non-workspace fixture crates; no new dependency
**Deps**: None

## Problem

`clippy.toml` declares `disallowed-types` (`HashMap`, `HashSet`, `SystemTime`, `Instant`) and `disallowed-methods` (`thread::spawn`, `process::Command::new`, `fs::read`/`read_to_string`/`File::open`, `net::TcpStream::connect`, `net::UdpSocket::bind`), and CI runs `cargo clippy --workspace --all-targets -- -D warnings` on the pinned toolchain. But no negative fixture proves the policy actually fires on the pinned toolchain for aliases, re-exports, type aliases, macro expansions, or higher-order references — the assurance is assumed, not demonstrated (spec §6 F-010/F-011, §8 THIRD-AC-004). This ticket builds the reusable subprocess runner that asserts each banned-API family fails to lint, establishing the harness tickets 003 and 009 reuse for their compile-fail fixtures.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, determinism-substrate ticket. -->

1. `clippy.toml:1` declares the disallowed type/method set above; `.github/workflows/ci.yml` runs `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`; `rust-toolchain.toml` pins `1.93.0` with `clippy` + `rustfmt`. The established subprocess pattern is `#[allow(clippy::disallowed_methods, reason = …)]` + `std::process::Command::new` at `crates/tracewake-tui/tests/command_loop_session.rs:9` and `readme_sample_session.rs:79`.
2. The remediation is spec §8 `THIRD-AC-004` + the §9.2 banned-API fixture list (reassessed this session). The §9 preamble fixes the mechanism: isolated fixture crates linted by a subprocess harness, reusing the allow + `Command::new` pattern; **no** new dependency (`trybuild`/`compiletest`).
3. Shared boundary under audit: the runner harness this ticket builds is the reusable contract consumed by ticket 003 (seal-bypass compile-fail fixtures) and ticket 009 (debug-construction compile-fail). Its contract: compile/lint an isolated, non-workspace fixture crate under the pinned toolchain and assert the expected failure **and lint category**, not merely non-zero exit.
4. Motivating invariants (restated): `INV-017` — randomness must be seedable/auditable; `INV-018` — deterministic replay is foundational. The banned APIs are nondeterminism vectors (`HashMap`/`HashSet` random-seeded iteration order, wall-clock `SystemTime`/`Instant`, thread scheduling, ad hoc fs/net), so the policy that bans them in outcome paths guards determinism.
5. Determinism-substrate surface: this ticket builds the *prover* for the determinism bans; the enforcement surface is the `clippy.toml` policy itself (a sibling-ticket-independent existing lock). The runner introduces no nondeterminism into what it asserts — it runs the pinned toolchain and asserts on lint category, never on wall-clock or thread timing.

## Architecture Check

1. A subprocess runner over **isolated, non-workspace** fixture crates is the only sound way to assert that code *fails* to lint under the real pinned toolchain: a `disallowed-methods`/`disallowed-types` violation cannot be unit-tested from inside a workspace crate (it would fail that crate's own build and the whole `cargo build --workspace`). Reusing the existing `Command::new` + `#[allow(clippy::disallowed_methods)]` pattern keeps `tracewake-core` zero-dependency and avoids adding `trybuild`/`compiletest`.
2. No backwards-compatibility shims. The substring token scanner (`anti_regression_guards.rs`) is retained only as a documented smoke guard (hardened in ticket 004), not removed; this runner is the stronger primary enforcement layer.

## Verification Layers

1. `INV-017` / `INV-018` (banned nondeterminism caught by policy) -> negative-fixture check: each banned-API fixture crate fails `cargo clippy` under the pinned toolchain.
2. Runner correctness -> manual review + CI: each assertion checks the emitted lint category (`disallowed_types` / `disallowed_methods`), not just a non-zero exit status.
3. Zero-dependency posture preserved -> codebase grep-proof: `! grep -rq 'trybuild\|compiletest' --include=Cargo.toml .`.

## What to Change

### 1. Build the negative-fixture runner harness

Add a test target that, for each fixture crate, invokes the pinned toolchain's `cargo clippy` via `std::process::Command` (guarded by `#[allow(clippy::disallowed_methods, reason = …)]`) and asserts the build fails with the expected lint category. Fixture crates live under a new non-workspace directory so `cargo build --workspace` never tries to compile them.

### 2. Add banned-API negative fixtures (§9.2 clippy families)

One fixture per family: `banned_hashmap_direct_path`, `banned_hashmap_import_alias`, `banned_hashset_reexport`, `banned_systemtime_alias`, `banned_instant_alias`, `banned_thread_spawn_direct`, `banned_thread_spawn_reexport`, `banned_fs_read_and_file_open`, `banned_tcp_udp_network_calls`, `banned_process_command_new`, and `banned_macro_expands_to_spawn_or_fs` (with an explicit note if clippy cannot catch the macro form and a disallowed-macro policy or the scanner is the fallback layer).

## Files to Touch

- `crates/tracewake-core/tests/negative_fixture_runner.rs` (new) — subprocess harness asserting each fixture fails clippy with the expected lint category
- `tests/negative-fixtures/` (new, implementation-discovered set) — one isolated non-workspace fixture crate per banned-API family listed above

## Out of Scope

- The `source_guard_*` discovery fixtures and the token-scanner walk/match hardening (ticket 004).
- Seal-bypass compile-fail fixtures (ticket 003, reuses this runner) and debug-construction compile-fail (ticket 009, reuses this runner).
- `.github/workflows/ci.yml` changes (ticket 012); the runner is a `cargo test` target already covered by CI's existing `cargo test --workspace`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — the runner passes only when every banned-API fixture crate fails clippy as expected; a fixture that unexpectedly *passes* fails the runner.
2. `cargo clippy --workspace --all-targets -- -D warnings` — the runner harness itself is clean (its `Command::new` use is allow-annotated).
3. `! grep -rq 'trybuild\|compiletest' --include=Cargo.toml .` — no new dependency introduced.

### Invariants

1. Each `clippy.toml`-banned API family has ≥1 negative fixture asserting the pinned-toolchain lint fires.
2. Assertions check the lint category, not just process exit status.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/negative_fixture_runner.rs` — the harness; rationale: turns the assumed `clippy.toml` coverage into demonstrated coverage under the pinned toolchain.
2. `tests/negative-fixtures/*` — one fixture crate per banned-API family; rationale: each is a "this must fail" example the runner asserts.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:
- Added `crates/tracewake-core/tests/negative_fixture_runner.rs`, a subprocess-based runner that invokes the pinned `cargo clippy` on isolated negative fixture crates and asserts each fixture fails with the expected lint category.
- Added isolated non-workspace fixture crates under `tests/negative-fixtures/` for direct paths, aliases, re-exports, type aliases, macro expansion, thread/process/fs/net calls, and unordered collection types.
- Verified the macro expansion fixture is caught by the current Clippy policy as `disallowed_methods`; no fallback scanner-only exception was needed.

Deviations from original plan:
- None. The runner uses `std::process::Command` with an explicit Clippy allow in the test harness and introduces no `trybuild` or `compiletest` dependency.

Verification results:
- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `if rg -q "trybuild|compiletest" -g Cargo.toml .; then exit 1; else exit 0; fi` — passed.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace` — passed.
