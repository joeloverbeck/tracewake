# 0002PHA1KERTUI-001: Cargo workspace scaffold and crate boundaries

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — introduces the `tracewake-core`, `tracewake-content`, `tracewake-tui` crates and the workspace root `Cargo.toml`.
**Deps**: none

## Problem

Phase 1 has no Rust workspace yet; the repository is paper-spec + doctrine only. Spec 0002 §9 requires a compact workspace with a small authoritative kernel and a doctrine-mandated dependency direction (`tracewake-core` ← `tracewake-content` ← `tracewake-tui`), enforced so no presentation/content layer can invert authority. Every later Phase 1 ticket builds inside this scaffold, so the crate boundaries and dependency edges must land first.

## Assumption Reassessment (2026-06-06)

1. No Rust tree exists: `find . -name Cargo.toml -o -name '*.rs'` returns empty (verified 2026-06-06). This is the repo's first code; all files are `(new)`.
2. The recommended workspace shape is `specs/0002_…_SPEC.md` §9.1 (`crates/tracewake-core` library, `crates/tracewake-content` library, `crates/tracewake-tui` binary/library) and §9.5 dependency rules; the optional `tracewake-cli` (§9.1) is explicitly not required for Phase 1 exit and is out of scope here.
3. Shared boundary under audit: the crate dependency graph. `docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` is the authority for kernel-owns-rules and dependency direction; this ticket encodes that as Cargo dependency edges plus a compile-time guard.
4. Invariant motivating this ticket: INV-008 (UI assistance is not authority) and the §7.2 dependency-direction constraint — `tracewake-core` must not depend on `tracewake-content`, `tracewake-tui`, terminal libraries, network libraries, LLM clients, or wall-clock APIs for outcome logic.

## Architecture Check

1. A compact three-crate workspace (not a single crate, not a per-module crate explosion) is the §9.1 recommendation: it gives a real compile-time kernel boundary without "architecture cosplay". The dependency direction is enforced by Cargo itself (core declares no dependency on content/tui), which is stronger than a lint.
2. No backwards-compatibility shims: this is greenfield; there is nothing to alias.

## Verification Layers

1. Kernel boundary / dependency direction (INV-008; arch 01) -> codebase grep-proof: `tracewake-core/Cargo.toml` lists no `tracewake-content`/`tracewake-tui` dependency and no terminal/network/LLM/time crate.
2. Workspace builds -> replay/golden-fixture check is N/A at scaffold stage; proven by `cargo build --workspace` compiling all three crates.
3. Single-layer-beyond-build mapping is not applicable: this ticket only establishes crate edges; behavioral invariants attach in later tickets.

## What to Change

### 1. Workspace root

Create a root `Cargo.toml` declaring a `[workspace]` with the three members and a shared `resolver = "2"`. Pin the Rust edition workspace-wide.

### 2. Crate skeletons

Create each crate's `Cargo.toml` and a minimal `src/lib.rs` (and `src/main.rs` for the TUI binary). `tracewake-content` depends on `tracewake-core`; `tracewake-tui` depends on `tracewake-core` and `tracewake-content`. `tracewake-core` depends on nothing in-workspace and pulls in no terminal/network/LLM/wall-clock crate.

## Files to Touch

- `Cargo.toml` (new) — workspace root, members list
- `crates/tracewake-core/Cargo.toml` (new)
- `crates/tracewake-core/src/lib.rs` (new) — empty crate root, module declarations added by later tickets
- `crates/tracewake-content/Cargo.toml` (new) — depends on `tracewake-core`
- `crates/tracewake-content/src/lib.rs` (new)
- `crates/tracewake-tui/Cargo.toml` (new) — depends on `tracewake-core`, `tracewake-content`
- `crates/tracewake-tui/src/lib.rs` (new)
- `crates/tracewake-tui/src/main.rs` (new) — thin binary entry

## Out of Scope

- Any domain types, IDs, events, or actions (later tickets).
- The optional `tracewake-cli` crate (§9.1 — not required for Phase 1 exit).
- Any terminal-UI library selection for the TUI (deferred to the TUI shell ticket).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build --workspace` compiles all three crates.
2. `cargo metadata --format-version 1` shows `tracewake-core` with no path dependency on `tracewake-content` or `tracewake-tui`.
3. `cargo tree -p tracewake-core` shows no terminal/network/LLM/time crate.

### Invariants

1. `tracewake-core` is a sink in the in-workspace dependency graph (nothing the kernel depends on can depend back on presentation/content).
2. Dependency edges point only kernel ← content ← tui.

## Test Plan

### New/Modified Tests

1. `None — scaffold ticket; verification is build/metadata command-based.` Behavioral tests begin in dependent tickets.

### Commands

1. `cargo build --workspace`
2. `cargo metadata --format-version 1 | rg -q 'tracewake-core' && cargo tree -p tracewake-core`
3. A build + metadata check is the correct verification boundary because the deliverable is the crate graph itself, which Cargo resolves and enforces.

## Outcome

Completed: 2026-06-06

What changed:
- Added the root Cargo workspace with `tracewake-core`, `tracewake-content`, and `tracewake-tui` members.
- Added minimal crate manifests and crate roots.
- Wired dependencies so content depends on core, and TUI depends on content plus core.
- Added a thin `tracewake-tui` binary entry.

Deviations from original plan:
- None.

Verification results:
- `cargo build --workspace` passed.
- `cargo metadata --format-version 1` showed `tracewake-core` with no dependencies.
- `cargo tree -p tracewake-core` showed only `tracewake-core`, with no terminal, network, LLM, or time crate.
