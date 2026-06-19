# 0041EPICERMUT-007: Kill `proposition.rs` canonical rendering survivors — `Display` and `render_location`

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/proposition.rs` only if a survivor reveals a real defect
**Deps**: None

## Problem

Spec 0041 §5.8 and §5.11 route three EPI mutation survivors in the proposition rendering path (`impl fmt::Display for Proposition`, `proposition.rs:279`; `render_location`, `proposition.rs:356`) to the projection/replay canonical-rendering and view-model-rendering layers. They are a seed floor (§3.6), killed only by a real responsible-layer explanation/evidence surface — not a non-empty-string check (§5.11) or format-only unit test (§5.8).

**§5.8 — canonical proposition display (1 identity):**
- `replace <Proposition as Display>::fmt -> fmt::Result with Ok(Default::default())`

**§5.11 — expected-location rendering (2 identities):**
- `replace render_location -> String with "xyzzy".into()`
- `replace render_location -> String with String::new()`

`Display` delegates to `self.render()` (`proposition.rs:279`–`:283`); `render_location` returns `place {id}` / `container {id}` / `actor {id}` (`:356`). A `format!("{p}") == p.render()` check or a non-empty-string assertion is supporting unit evidence only.

## Assumption Reassessment (2026-06-19)

1. Codebase check: `impl fmt::Display for Proposition { fn fmt(&self, f) { f.write_str(&self.render()) } }` (`proposition.rs:279`); `render(&self) -> String` (`:165`); `render_location(&Location) -> String` (`:356`) with `AtPlace → "place {id}"`, `InContainer → "container {id}"`, `CarriedBy → "actor {id}"`. The projection debug contradiction summary calls `render` directly (per §5.8). The spec's `:281/:357` entries are cargo-mutants identities; the verified symbols are authoritative.
2. Specs/docs check: §5.8 requires driving every proposition variant through a production diagnostic/notebook/why-not/transcript that relies on `Display`, comparing the rendered meaning with the typed proposition carried by the same event/projection evidence (≥ an expected-location contradiction and one non-contradiction); the empty/default formatter must be proven to erase a real responsible-layer explanation while the typed record stays intact, or the formatter is removed/narrowed or a reviewed exception is filed. §5.11 requires rendering expected-missing propositions for `AtPlace`/`InContainer`/`CarriedBy` through a real contradiction/notebook/view-model/typed diagnostic surface that retains both the relation kind (place/container/actor) and the concrete identifier, paired with the underlying typed proposition + source evidence; the member matrix shows the `"xyzzy"` and empty mutants both destroy the explanation, and a structured-identity assertion accompanies any snapshot so a broad snapshot update cannot launder the mutant.
3. Cross-artifact shared boundary under audit: the rendering chain `Display`→`render`→`render_location` ↔ the production diagnostic/view-model/transcript consumer ↔ the typed proposition + source evidence it explains. The witness must enumerate the final `Display`/`render_location` call sites, not assume the formatters are consequential.
4. Motivating invariants (INV restate): §10 maps EPI-02/06/09 rendering to `INV-024` (no telepathy — embodied rendering shows only holder-known reality) and `INV-095`/`INV-105` (leakage is high severity; view-model reachability/diagnostics are acceptance evidence). The kill must witness a semantic explanation surface, not arbitrary text.
5. Fail-closed / actor-knowledge / view-model leakage surface: the enforcement surface is the embodied/diagnostic rendering of expected-missing propositions. Confirm embodied rendering reports only the holder-known expectation and contradiction and does **not** reveal the actual hidden location of the missing item (§5.11 control); the embodied diagnostic adds no hidden/debug-only facts to make the message meaningful (§5.8 control); a debug view may expose additional trace detail only with the debug capability. No epistemic leakage; replay reproduces the rendered structured identity deterministically.

## Architecture Check

1. Pairing rendered text with the typed proposition + source evidence (asserting structured identity alongside any snapshot) is cleaner than a non-empty-string or `format!==render` check: it ties the kill to the explanation the renderer exists to produce, so an empty/`"xyzzy"` formatter destroys a real responsible-layer diagnostic rather than only a unit string, and a snapshot update cannot launder the mutant.
2. No backwards-compatibility aliasing/shims: no test-only renderer, no `#[mutants::skip]`; a redundant formatter (if found) is removed/narrowed with reconciliation.

## Verification Layers

1. INV-024 (no telepathy in embodied rendering) -> replay/golden-fixture check: the expected-missing message reveals only the holder-known expectation/contradiction, not the hidden actual location.
2. INV-095/105 (view-model diagnostics are acceptance evidence) -> manual review + snapshot: rendered text preserves relation kind + concrete identifier and matches the typed proposition; empty/`"xyzzy"` output destroys it.
3. Canonical rendering determinism -> replay/golden-fixture check: replay reproduces the structured identity behind the rendered text.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F 'Proposition::fmt' -F render_location` reports the three identities `caught`.

## What to Change

### 1. Canonical `Display` consequence (§5.8)

Drive every proposition variant through a production diagnostic/notebook/why-not/transcript that relies on `Display`, comparing rendered meaning against the typed proposition from the same event/projection evidence (include an expected-location contradiction and a non-contradiction proposition). Enumerate the final `Display` call sites and prove that an empty/default formatter erases a real responsible-layer explanation while the typed record stays intact. If no such consumer exists, remove/narrow the redundant formatter or submit the rare reviewed exception (recorded in ticket 009's register) rather than certify a format-only unit test.

### 2. Expected-location rendering consequence (§5.11)

Render expected-missing propositions for `AtPlace`/`InContainer`/`CarriedBy` through a real contradiction/notebook/view-model/typed diagnostic surface that retains the relation kind and concrete identifier, paired with the typed proposition + source evidence. The member matrix shows the `"xyzzy"` and empty-string mutants both destroy the explanation; a structured-identity assertion accompanies any snapshot.

### 3. Negative/contamination controls

Embodied rendering reports only holder-known expectation/contradiction, never the hidden actual location; the embodied diagnostic adds no hidden/debug-only facts; richer debug rendering requires the debug capability.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — diagnostic/why-not `Display` consequence comparing rendered meaning to the typed record, as surfaced)
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — rendered proposition text + structured-identity assertion where already displayed)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — embodied no-hidden-location leakage control, as surfaced)
- `crates/tracewake-core/src/epistemics/proposition.rs` (modify — only if a survivor reveals a real defect or a redundant formatter must be reconciled/removed)

## Out of Scope

- Killing the other proposition families — contradiction relations (005), parser/deserialize (006), reference-validation/diagnostics (008) — or `belief.rs`/`contradiction.rs`/`observation.rs`.
- Solving a survivor by copying debug truth into an embodied message (§6.10 mutation coupling).
- Adding any proposition survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and EPI-01…11 re-proof / acceptance artifact (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F 'Proposition::fmt' -F render_location` — the three identities `caught`:
   - `<Proposition as Display>::fmt -> Ok(default)` — killed by a production explanation surface (or a recorded reviewed exception).
   - `render_location -> "xyzzy"` and `render_location -> ""` — each killed by a structured-identity + relation-kind assertion.
2. Rendered text is paired with the typed proposition/source evidence (not a bare non-empty check); a snapshot update alone cannot launder the mutants; member-level evidence retained.
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. Embodied rendering reveals only holder-known expectation/contradiction, never the hidden actual location.
2. Rendered text preserves relation kind + concrete identifier and matches the typed proposition; replay reproduces the structured identity.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` — `Display`-driven diagnostic comparing rendered meaning to the typed proposition across variants (expected-location contradiction + non-contradiction).
2. `crates/tracewake-tui/tests/transcript_snapshot.rs` / `crates/tracewake-tui/tests/adversarial_gates.rs` — expected-location rendering with structured-identity assertion + the no-hidden-location leakage control.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/proposition.rs -F 'Proposition::fmt' -F render_location`
2. `cargo test --workspace --locked`
3. The `-f … -F` filter is the correct per-ticket boundary: it regenerates exactly the rendering mutants in isolation so this family's catch is provable before the full campaign (ticket 009) reconciles the whole file.

## Outcome

Completed: 2026-06-19

Implemented the rendering certification in `crates/tracewake-core/src/epistemics/proposition.rs`. Existing ticket 005/006 projection checks already carry expected-location rendering through typed contradiction debug summaries and canonical checksum evidence; ticket 007 adds a retained `Display` corpus over every proposition variant, including all three expected-location relation kinds, and pairs each rendered explanation with the typed canonical round-trip. This catches empty/default formatter output and both `render_location` replacement identities while preserving structured relation kind and concrete ID assertions.

No production correction was needed: `render_location` already preserves `place`, `container`, and `actor` relation labels. I searched the live call sites and found projection/notebook consumers use `Proposition::render()` directly, while TUI debug panels receive already-rendered proposition strings. I therefore treated `<impl Display for Proposition>::fmt` as a redundant public formatter certification rather than a production explanation-surface certification; no alias/fallback path was added.

Verification:

- `cargo test -p tracewake-core epistemics::proposition` — passed, 11 proposition tests including the new Display corpus and prior relation/parser consumer checks.
- `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/proposition.rs -F '<impl fmt::Display for Proposition>::fmt' -F render_location --test-workspace true -C=--locked` — passed, 3 mutants tested, 3 caught.
- `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/proposition.rs -F 'Proposition::fmt' -F render_location --test-workspace true -C=--locked` — passed for the two `render_location` mutants, 2 tested, 2 caught; this confirmed the requested `Proposition::fmt` spelling does not match cargo-mutants' generated Display name.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace --locked` — passed.
