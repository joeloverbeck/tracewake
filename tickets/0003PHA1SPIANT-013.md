# 0003PHA1SPIANT-013: Doc/invariant `INV-###` reference linter

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — new no-dependency `cargo test` (or small binary) parsing the constitution and scanning live specs/tickets/tests for `INV-###` references
**Deps**: None

## Problem

Specs and tickets cite many `INV-###` invariants, but nothing checks that every cited invariant actually exists in the constitution. A mistyped or stale invariant number (e.g. a `INV-1xx` that does not exist, or a reference to a renumbered heading) passes review silently and propagates misleading provenance. Spec `0003` §7 SPINE-AC-014 requires a no-dependency `cargo test` that fails on a dangling `INV-###` reference in live specs/tickets/tests.

## Assumption Reassessment (2026-06-08)

1. `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` defines `INV-001`…`INV-110` (verified: 110 `INV-###` headings, including the `## 2026 hardening invariants` set `INV-099`–`INV-110`). Live `INV-###` references appear across `docs/4-specs/`, `specs/` (this spec cites `INV-099`–`INV-108`), `tickets/` (this batch), and selected test names/comments. (Note: the repo-root `CLAUDE.md` still says "INV-001…INV-098", which is itself stale — out of scope here, but illustrates exactly the drift this linter catches.)
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-014 mandates: a no-dep Rust test/binary invoked by `cargo test` that parses the constitution for defined `INV-###` headings, scans `docs/4-specs`, `specs`, `tickets`, and selected test names/comments for `INV-###` references, fails on dangling references, optionally warns on unused invariants, treats gate-code strings as cross-references only (no gate-semantics definition), and ignores archived historical ledgers except for syntactic sanity (so old baseline evidence is not promoted to live authority).
3. Boundary under audit: the live-doctrine reference surface (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` as the source of truth) vs. its citing artifacts. No symbol is renamed; this adds a CI linter. The test reads repository markdown via paths relative to the crate/workspace root using only `std`.
4. INV motivating this ticket: all cited `INV-###` (reference integrity) plus the spec-posture discipline that invariants are named, never redefined, in specs (`docs/4-specs/README.md:30-38`). Restated: a cited invariant must resolve to a real heading; dangling references erode the doctrine spine's trustworthiness.

## Architecture Check

1. A no-dependency `std`-only test keeps the gate inside the pinned toolchain (no new crate dependency), parsing the constitution once and asserting every live `INV-###` citation resolves. This catches mistyped/stale invariant numbers at `cargo test` time rather than in human review, and excludes archived ledgers from authority promotion.
2. No backwards-compatibility shim: the linter does not special-case or auto-fix references; a dangling reference is a hard failure to be corrected at the source.

## Verification Layers

1. Reference integrity (all cited `INV-###`) -> codebase grep-proof / parse: the test parses defined headings and fails on any live citation that does not resolve.
2. Single-layer ticket: this is a self-contained linter whose sole invariant is reference integrity; no additional cross-invariant proof surface applies, because it asserts a syntactic/reference property over markdown rather than runtime simulation behavior.

## What to Change

### 1. Invariant-reference linter

Add a no-dep test (e.g. `crates/tracewake-core/tests/doc_invariant_references.rs`, or a workspace-level test target) that: parses `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` for defined `INV-###` headings; scans `docs/4-specs/`, `specs/`, `tickets/`, and selected test sources for `INV-###` tokens; asserts every referenced number is defined; optionally collects unused invariants as a warning. Archived ledgers are checked only for syntactic sanity, not for live authority.

### 2. Gate-code neutrality

Ensure the linter treats gate-code strings (`P0-CERT`, `SPINE-CERT`, etc.) as opaque cross-references and does not attempt to define or validate gate semantics.

## Files to Touch

- `crates/tracewake-core/tests/doc_invariant_references.rs` (new)

## Out of Scope

- Defining or validating gate-code semantics.
- Editing the stale `CLAUDE.md` invariant-range claim (separate concern; not a spec deliverable).
- Promoting archived ledgers to live authority.

## Acceptance Criteria

### Tests That Must Pass

1. `doc_invariant_references_are_live` — passes when all live `INV-###` citations resolve; fails when a dangling reference is introduced into a live spec/ticket/test.
2. `cargo test --workspace` passes.

### Invariants

1. Every live `INV-###` citation resolves to a defined heading in the constitution (reference integrity).
2. Gate codes remain cross-references; the linter defines no gate semantics.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/doc_invariant_references.rs` — parses the constitution and scans live specs/tickets/tests for dangling `INV-###`.

### Commands

1. `cargo test -p tracewake-core --test doc_invariant_references`
2. `cargo test --workspace`
