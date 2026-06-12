# 0024PHA3ACONSCH-005: Raw-line validation on the production load path and ID-field shortcut scanning

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`validate.rs`, `load.rs`) plus content tests.
**Deps**: 0024PHA3ACONSCH-004

## Problem

Two between-boundaries gaps in content validation (spec 0024 §4):

- `ORD-HARD-150` (medium): the raw-line validation layer — prose-born-fact
  (INV-022), unknown-field, and forbidden top-level keys — runs only in
  `validate_fixture_bytes`; `load_fixture_package` calls `deserialize_fixture` →
  `validate_fixture` (the struct path) and never reaches `validate_raw_lines`. The
  `prose_born_fact` diagnostic is structurally unreachable in production (its
  trigger tags fail earlier as `BadLine`), so the INV-022 proof
  (`content_prose_born_fact_rejected`) exercises a path no real load takes.
- `ORD-HARD-151` (medium): the shortcut/script-marker scan
  (`PHASE3A_SHORTCUT_MARKERS`, `reject_text_by_policy`) applies to `String`-typed
  fields registered in `SCANNED_STRING_FIELDS`; stable-ID newtype fields are checked
  only by `reject_reserved_or_display`'s seven-word list, and the
  `discover_schema_fields` conformance census cannot see ID newtypes — so an item id
  like `appear_at_workshop` carries a script marker into authored content unflagged
  (INV-097).

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: `load_fixture_package`
   (`load.rs`) calls `validate_fixture`, not `validate_fixture_bytes`;
   `validate_raw_lines` is invoked only from `validate_fixture_bytes`
   (`validate.rs`); `reject_reserved_or_display`, `PHASE3A_SHORTCUT_MARKERS`,
   `SCANNED_STRING_FIELDS`, and `discover_schema_fields` confirmed at their cited
   homes in `validate.rs`.
2. Verified against spec 0024 §4 (`ORD-HARD-150`/`151`), INV-022 and INV-097
   headings in the constitution, and `docs/2-execution/08_*` (validation-phase
   assignments).
3. Cross-artifact boundary: the bytes-vs-struct validation split — after this
   ticket, `validate_fixture_bytes` is the single production entry (raw-line scan +
   struct validation), and the ID-scan census closes the typed-field complement.
4. INV-022 / INV-097 restated: raw prose may not define facts — the typed
   `prose_born_fact` rejection must be reachable where fixtures actually load; no
   authored outcome chains — the forbidden-marker family is enforced per authored
   value, not per type convention.
5. Enforcement surface: strengthens fail-closed content validation on the
   production path; no existing acceptance is weakened — current committed fixtures
   carry no prose markers or marker-bearing ids (verified by the audit's loader
   sweep), so the suite stays green. Any latent fixture surfaced by the newly
   reachable scan is triaged in-ticket as a masked defect first (Enforcement
   reading), not silenced.
6. Adjacent-contradiction classification: `BadLine`-vs-raw-scan ordering — routing
   bytes through `validate_fixture_bytes` means unknown-section rejection and the
   raw-line scan must keep a deterministic error order; if the existing `BadLine`
   path pre-empts a clearer typed diagnostic, adjusting the order is a required
   consequence of this ticket (recorded in implementation notes), not a separate
   bug.

## Architecture Check

1. Routing `load_fixture_package` through `validate_fixture_bytes` (it already holds
   `primary.bytes`) reuses the existing two-phase validator instead of duplicating
   the raw-line checks on the struct path — one production entry point, no parallel
   validation stacks. Extending `discover_schema_fields` to enumerate ID-typed
   fields and routing every stable-ID check through the same shortcut predicate
   closes the census complement structurally (the `ORD-HARD-123` lesson applied to
   content), rather than growing a second hand-list.
2. No backwards-compatibility aliasing/shims: the struct-only load path is
   replaced, not kept as a fallback; the seven-word display list is subsumed, not
   duplicated.

## Verification Layers

1. INV-022 production reachability → a prose-marker fixture loaded through
   `load_fixture_package` fails with code `prose_born_fact` (replay/golden-fixture
   check); one forbidden top-level key likewise.
2. INV-097 ID coverage → a negative fixture with a marker-bearing id
   (`appear_at_workshop`) fails validation; the extended census asserts every
   ID-typed field is shortcut-scanned (codebase grep-proof + test).
3. Census closure → `discover_schema_fields` extended-enumeration test fails if a
   new ID-typed field escapes registration.
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Production raw-line routing (`ORD-HARD-150`)

`load_fixture_package` validates via `validate_fixture_bytes` on `primary.bytes`;
struct-path validation remains as phase two within it. Add load-path negatives for
`prose_born_fact` and one forbidden top-level key.

### 2. ID-field shortcut scanning (`ORD-HARD-151`)

Route every stable-ID validation through `is_phase3a_shortcut_marker` +
authored-outcome predicates (substring/token match on the id); extend the
`discover_schema_fields` census to enumerate ID-newtype fields and assert each is
scanned; add the marker-bearing-id negative fixture.

## Files to Touch

- `crates/tracewake-content/src/load.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify)

## Out of Scope

- The schema-version gate (ticket -004, dependency).
- Widening `PHASE3A_SHORTCUT_MARKERS`' word list itself (the closed hand-list's
  contents are unchanged; this ticket changes where it is enforced).
- Need-seed defaults and serialization goldens (ticket -006).

## Acceptance Criteria

### Tests That Must Pass

1. A `notes|…culprit…` (prose-marker) fixture and an `appear_at|…` top-level-key
   fixture each fail `load_fixture_package` with their typed codes — not `BadLine`
   masking, unless the recorded error-order decision says otherwise (Assumption
   item 6).
2. An `item|appear_at_workshop|…` fixture fails validation with the shortcut-marker
   code; the extended census fails when an ID-typed field is unregistered (synthetic).
3. All committed fixtures still load; `cargo test -p tracewake-content` and the four
   workspace gates pass.

### Invariants

1. No production load path bypasses the raw-line validation layer.
2. The shortcut-marker family is enforced uniformly across `String`-typed and
   ID-typed authored values, with census-derived coverage.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — load-path prose-born-fact and
   forbidden-key negatives.
2. `crates/tracewake-content/tests/forbidden_content.rs` — marker-bearing-id
   negative; census-extension synthetic.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Closed both validation gaps:

1. `load_fixture_package` now validates the primary fixture bytes through
   `validate_fixture_bytes`, so raw-line validation is on the production load path
   before manifest construction. The loader derives the action registry from the
   raw `fixture_scope|...` line first, avoiding a pre-deserialize that would mask
   raw-line diagnostics.
2. Load-path negatives now prove `notes|...culprit...` rejects with
   `prose_born_fact`, and `appear_at|...` rejects with `forbidden_form`, both as
   `LoadError::Validation`.
3. Stable-ID authored values are now scanned through the same union shortcut/script
   policy used for String fields. A negative fixture with
   `item|appear_at_workshop|...` rejects at `items[0].item_id` with
   `authored_shortcut_effect`.
4. The schema-field census now derives ID-typed fields from `schema.rs`, checks
   every ID field has a shortcut-scan registration and rationale, and includes a
   synthetic unregistered ID field proof.

Verification:

1. `cargo test -p tracewake-content` passed.
2. `cargo fmt --all --check` passed.
3. `cargo clippy --workspace --all-targets -- -D warnings` passed.
4. `cargo build --workspace --all-targets --locked` passed.
5. `cargo test --workspace` passed.

No shortcut marker names were added; enforcement was widened to the production
load path and ID-typed authored fields.
