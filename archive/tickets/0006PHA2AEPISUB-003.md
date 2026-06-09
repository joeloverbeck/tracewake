# 0006PHA2AEPISUB-003: Seal epistemic records into provenance-preserving abstract data types

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` epistemic records (`Belief`/`Observation`/`Contradiction`/`BeliefDraft`) sealed to private fields + accessors/builders; `tracewake-content` belief conversion routed through a builder instead of post-construction field mutation; workspace read sites migrated to accessors.
**Deps**: 0006PHA2AEPISUB-002

## Problem

`Belief`, `Observation`, `Contradiction`, and `BeliefDraft` expose every field as `pub` (`crates/tracewake-core/src/epistemics/belief.rs:39-55`, `:109-117`; `crates/tracewake-core/src/epistemics/observation.rs:107-123`; `crates/tracewake-core/src/epistemics/contradiction.rs:32-43`). Their smart constructors set provenance, schema version, and actor-private scope correctly, but because the fields are public the guarantee is not structural: downstream code can erase provenance, rewrite holder/scope, downgrade `schema_version`, or alter observation/contradiction links after validation. The content loader already does post-construction mutation (`crates/tracewake-content/src/schema.rs:404-407` rewrites `belief.channel`, `belief.last_verified_tick`, `belief.privacy_scope`, `belief.schema_version` directly). This is EPI-HARD-003: provenance-bearing proof records are mutable ADTs, so an illegal-state record is representable through safe Rust.

## Assumption Reassessment (2026-06-09)

1. All record fields are `pub`: confirmed `belief.rs:39-55` (`Belief`) and `:109-117` (`BeliefDraft` options), `observation.rs:107-123` (`Observation`), `contradiction.rs:32-43` (`Contradiction`). Smart constructors exist and set provenance: `Belief::new` (`belief.rs:57-91`) derives actor-private scope + requires `SourceRef`; `Observation::new` (`observation.rs:125-153`) sets schema version + actor-private scope; `BeliefDraft::build` rejects missing required fields (`belief.rs:119-133`). No literal `Belief {…}`/`Observation {…}`/`Contradiction {…}` construction exists outside `epistemics/` (grep `crates`); the only post-construction field mutation outside the module is `content/schema.rs:404-407`.
2. Spec authority: `specs/0006_…SPEC.md` §6 EPI-HARD-003; `INV-026` requires important beliefs to record full provenance (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:117-119`); architecture requires belief/observation fields with provenance and holder/visibility scope (`docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md:44-55`).
3. Shared boundary under audit: the epistemic-record public field API consumed workspace-wide. Privatizing fields breaks every **read** site (not just mutations), so the migration is a local compile-atomic retype: read sites span `tracewake-core` (`projections.rs`, `actions/pipeline.rs`, `epistemics/contradiction.rs`, `epistemics/projection.rs`, `events/*`, `replay/*`, `agent/actor_known.rs`, `view_models.rs`, `debug_reports.rs` + tests `golden_scenarios.rs`), `tracewake-content` (`schema.rs`, `golden_fixtures_run.rs`), and `tracewake-tui` (`app.rs`, `debug_panels.rs`, `render.rs`). All live in this Cargo workspace, so all must change in one diff or the tree will not compile.
4. Constitutional invariants motivating this ticket: `INV-026` (provenance completeness), `INV-023` (ground truth / belief / records kept separate), `INV-021` (typed claims are the epistemic currency), `INV-102` (cognition inputs require provenance; missing provenance is a rejection condition). A publicly-mutable belief record lets provenance be erased after validation, violating all of these.
5. Enforcement surface: actor-knowledge / provenance integrity. Making fields private with invariant-preserving `with_*`/builder methods means a belief without source/holder/scope/schema is unconstructable and unmodifiable into an illegal state. Deterministic replay is unaffected: the records' serialized form is produced by the canonical methods, which read the (now private) fields internally; no byte changes.
6. Visibility/shape change (reseal/retype, not extension): the field *set* of each record is unchanged; visibility moves `pub` → private + accessors, and `BeliefDraft`'s public option fields become builder methods. Consumers are the workspace read sites in item 3; the change is breaking for every external field read/write, so each migrates in this diff. The content loader's mutation (`schema.rs:404-407`) is replaced by builder methods (`with_channel`/`with_last_verified_tick`/… already exist in part — `Belief::with_channel` is used at `contradiction.rs`; confirm/extend the builder surface during implementation).
7. Removal blast radius of the old public surface: enumerated in item 3. `DebugBeliefEntry`/`DebugHolderBeliefs` reads in TUI `debug_panels.rs`/`view_models.rs` are debug-view-struct fields, not record fields — out of scope. Record-field reads inside `debug_reports.rs` source-guard string literals (`:991-992`) are meta-strings, not real reads — out of scope.

## Architecture Check

1. Sealing the records and exposing accessors + invariant-preserving `with_*`/builder methods makes "construct a belief without provenance" or "erase a source after the fact" unrepresentable, which is stronger than the current convention (good constructor, public fields). Replacing `content/schema.rs`'s direct field assignment with builder calls puts the one legitimate content-conversion path through the same validated surface as runtime construction.
2. This is a local compile-atomicity retype, not a spec-mandated atomic cutover: the records and their workspace consumers must change together because the compiler requires it. No backwards-compat aliasing/shims; the public fields are removed, not re-exported.

## Verification Layers

1. `INV-026`/`INV-102` (provenance unerasable) -> codebase grep-proof: no `pub` field on `Belief`/`Observation`/`Contradiction`/`BeliefDraft` (asserted by the source guard in 0006PHA2AEPISUB-005) + unit test that a belief is unconstructable/unmodifiable without source/holder/scope.
2. `INV-023` (truth/belief/records separated) -> manual review: content conversion preserves holder/scope/provenance through the builder; no path downgrades schema or clears links.
3. `INV-018` (deterministic replay) -> replay/golden-fixture check: `cargo test -p tracewake-core --test event_schema_replay_gates` + projection checksum tests green (canonical serialization unchanged).

## What to Change

### 1. Privatize record fields and add accessors

In `belief.rs`, `observation.rs`, `contradiction.rs`: remove `pub` from all fields; add read-only accessors for every field consumers read. Keep/extend invariant-preserving `with_*` methods only where they cannot violate provenance.

### 2. Replace `BeliefDraft` public options with a builder

Make `BeliefDraft`'s option fields private; expose setter methods (or a staged builder) so `build()` remains the only path to a `Belief` and still rejects missing required fields.

### 3. Route content conversion through the builder

In `crates/tracewake-content/src/schema.rs`, replace the direct `belief.channel = …` / `belief.last_verified_tick = …` / `belief.privacy_scope = …` / `belief.schema_version = …` assignments (`:404-407`) with builder/`with_*` methods.

### 4. Migrate workspace read sites to accessors

Update every read site enumerated in Assumption Reassessment item 3 to call accessors instead of fields. Implementation-discovered set: the candidate files are named there; the compiler enumerates the exact lines.

## Files to Touch

- `crates/tracewake-core/src/epistemics/belief.rs` (modify)
- `crates/tracewake-core/src/epistemics/observation.rs` (modify)
- `crates/tracewake-core/src/epistemics/contradiction.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — checksum/test reads; **shared file, also touched by 0006PHA2AEPISUB-002**)
- `crates/tracewake-core/src/projections.rs` (modify — record reads)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — record reads)
- `crates/tracewake-core/src/events/apply.rs` (modify — record reads/construction)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — test reads)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify — record reads)
- `crates/tracewake-core/src/view_models.rs` (modify — record reads)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — record reads)
- `crates/tracewake-content/src/schema.rs` (modify — builder conversion)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — record reads)
- `crates/tracewake-tui/src/app.rs` (modify — record reads; **shared file, also touched by 0006PHA2AEPISUB-002**)
- `crates/tracewake-tui/src/render.rs` (modify — record reads, as surfaced)
- Additional read sites as surfaced by the compiler (implementation-discovered set; candidates named in Assumption Reassessment item 3).

## Out of Scope

- `KnowledgeContext` sealing (0006PHA2AEPISUB-001) and projection sealing (0006PHA2AEPISUB-002).
- Compile-fail negative fixtures (0006PHA2AEPISUB-004) and source-guard/clippy extension (0006PHA2AEPISUB-005).
- Any change to the canonical serialization / checksum semantics of records.

## Acceptance Criteria

### Tests That Must Pass

1. A unit test that a `Belief`/`Observation`/`Contradiction` without provenance/schema/holder/scope/required links is unconstructable through public API, and that no public method can clear `source` or downgrade `schema_version` after construction.
2. A content-conversion test that `to_belief` produces a fully-provenanced belief through the builder (no direct field assignment), and that content validation still rejects missing provenance (`forbidden_content.rs` stays green).
3. `cargo build --workspace --all-targets --locked`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No `pub` field remains on `Belief`/`Observation`/`Contradiction`/`BeliefDraft`; all reads go through accessors and all mutation through validated builders/`with_*`.
2. Canonical serialization/checksum of records is byte-identical to pre-change; deterministic replay unaffected.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/belief.rs` / `observation.rs` / `contradiction.rs` (in-module tests) — sealing/provenance-integrity unit tests.
2. `crates/tracewake-content/src/schema.rs` (in-module test) — builder-based `to_belief` conversion test.

### Commands

1. `cargo test -p tracewake-core --lib epistemics`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p tracewake-content --test forbidden_content --test golden_fixtures_run` — proves content conversion + provenance gates stay green after the retype.

## Outcome

Completed: 2026-06-09

What changed:
- Sealed `Belief`, `Observation`, `Contradiction`, and `BeliefDraft` fields behind accessors and invariant-preserving constructors/builders.
- Added `BeliefDraft` builder methods and `Belief::with_last_verified_tick` so content conversion can preserve optional metadata without public field mutation.
- Migrated workspace consumers from direct record field reads to accessor calls.
- Replaced `InitialBeliefSchema::to_belief` post-construction record mutation with builder-style construction.
- Added provenance/schema/scope unit coverage for belief, observation, contradiction, and content conversion.

Deviations from original plan:
- No arbitrary `privacy_scope` or `schema_version` mutators were added for `Belief`; the content path now relies on validated schema input matching constructor-derived actor-private scope and canonical schema version.
- Compile-fail negative fixtures and source guards remain deferred to 0006PHA2AEPISUB-004 and 0006PHA2AEPISUB-005 as planned.

Verification:
- `cargo check --workspace --all-targets`
- `cargo fmt --all --check`
- `cargo test -p tracewake-core --lib epistemics`
- `cargo test -p tracewake-content --test forbidden_content --test golden_fixtures_run`
- `cargo test -p tracewake-core --test event_schema_replay_gates`
- `cargo test --workspace`
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
- Structural sweeps found no remaining public epistemic record fields and no content-side direct `belief.* =` mutation.
