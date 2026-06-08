# 0002TUIPROOSUR-001: Sealed holder-known context packet + deterministic context hash

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (extends `epistemics::KnowledgeContext`; new typed context id in `ids.rs`; deterministic context hash in `checksum.rs`)
**Deps**: None

## Problem

The architecture (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:31-61`, `:93-98`; `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md:21-37`) requires a *sealed* holder-known context packet — stable context id, deterministic context hash over canonical actor-known inputs, holder + bound-actor identity, tick/frontier, provenance entries, a forbidden-truth audit result, and stale/invalid status — built and sealed before cognition/proposal selection. Today `epistemics::KnowledgeContext` (`crates/tracewake-core/src/epistemics/knowledge_context.rs:37-49`) is only a source-policy filter (`ViewMode`, allowed/forbidden source enums, scope filters, `permits_scope()`); it carries no hash, no provenance edges, no audit *result*, and no holder/bound-actor split or frontier. Spec 0002 §4 TUI-AC-001 requires completing it into the full sealed packet rather than introducing a parallel type (§9 no-duplicate rule). This ticket builds that packet type and its deterministic hash; downstream tickets route view models (002), affordances (003), and proposals (006) through it.

## Assumption Reassessment (2026-06-08)

1. `epistemics::KnowledgeContext` exists at `crates/tracewake-core/src/epistemics/knowledge_context.rs:37-49` with `embodied()`/`debug()` constructors (`:51-81`), `AllowedKnowledgeSource`/`ForbiddenKnowledgeSource` enums (`:13-29`), `ScopeFilter` (`:31-35`), and `permits_scope()` (`:83-92`); it is re-exported from `epistemics/mod.rs:17`. It has no `context_hash`, provenance edges, forbidden-truth audit result, holder/bound-actor distinction, or frontier.
2. Spec 0002 §4 TUI-AC-001 (`archive/specs/0002_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`) and `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:31-61` enumerate the packet's required fields; `:93-98` requires the packet be immutable after sealing.
3. Cross-artifact boundary under audit: the relationship between `epistemics::KnowledgeContext` (this ticket's target, the TUI/view-model-facing context) and `agent::ActorKnownPlanningContext` (`crates/tracewake-core/src/agent/actor_known.rs:185`, the agent-decision sealed context). The packet must stay conceptually consistent with `ActorKnownPlanningContext` (same allowed/forbidden source vocabulary, same provenance discipline) and must not become a third parallel notion of "what the actor knows".
4. Invariants under audit, restated: **INV-101** — action-proposal generation must consume a *sealed* actor-known context that contains no validator-only truth; **INV-102** — every cognition input must carry source/provenance sufficient for replay/debug; **INV-100** — no hidden world state may enter cognition input; **INV-017** — random/derived auditable values that affect outcomes must be reproducible (the context hash must be deterministic).
5. Fail-closed / replay / no-leak surface (substrate-only): this packet is the *input* to 006's fail-closed proposal validation and to the no-leak firewall, but no validator consumes it yet. The context hash must be computed over canonical, sorted, actor-known inputs only — no wall-clock, no `std::time`, no incidental hash-map iteration order — so identical inputs+versions yield byte-identical hashes (INV-017/INV-018). The embodied packet must exclude all `ForbiddenKnowledgeSource` truth and record its exclusions in the forbidden-truth audit result, introducing no leakage path that 006 (the enforcer) would later have to undo.
6. Schema extension: this extends the `KnowledgeContext` cognition/view contract. Current consumer: `build_notebook_view` (`crates/tracewake-core/src/projections.rs:390`) and `TuiApp::current_view` (`crates/tracewake-tui/src/app.rs:166-175`). The extension is additive to `KnowledgeContext` (new fields + a sealed constructor); the breaking consumer changes (EmbodiedViewModel typed id/hash, builder rewrites) are owned by ticket 002, not here.

## Architecture Check

1. Extending `KnowledgeContext` into the sealed packet — rather than adding a new `HolderKnownContext` type alongside it — keeps a single holder-known notion across the codebase, honoring Spec 0002 §9's explicit no-duplicate-API rule and keeping the view-model context aligned with `agent::ActorKnownPlanningContext`. A parallel type would create two drifting definitions of actor-known truth, the exact contamination risk this phase exists to close.
2. No backwards-compatibility aliasing/shims: the legacy `EmbodiedViewModel.knowledge_context_id: Option<String>` display string is **not** wrapped or aliased; it is replaced by typed fields in ticket 002. This ticket adds the sealed type and hash only and leaves the cutover atomic in 002.

## Verification Layers

1. INV-101 (sealed context) -> codebase grep-proof: forgery-enabling fields (provenance edges, audit result, hash) are private behind the sealing constructor and typed accessors; no public mutable setter exists.
2. INV-102 (provenance) -> schema validation + unit test: the sealed packet carries non-empty provenance entries for each actor-known fact and a forbidden-truth audit result enumerating excluded sources.
3. INV-017/INV-018 (deterministic hash) -> replay/golden-fixture check: same canonical inputs produce identical `holder_known_context_hash` across two builds; hash inputs are sorted/canonical with no wall-clock.
4. INV-100 (no hidden-truth cognition) -> manual review + unit test: the embodied packet excludes every `ForbiddenKnowledgeSource` and the audit result records the exclusion; the debug packet is the only `ViewMode` permitted broader scope.

## What to Change

### 1. Sealed holder-known context packet

Extend `KnowledgeContext` (or a sealed `HolderKnownContext` produced from it within the same module) to carry: `holder_known_context_id` (new typed id), `holder_known_context_hash`, holder identity + bound actor identity, current tick / event frontier, provenance entries, a forbidden-truth audit result, and stale/invalid status. Provide a sealing constructor that consumes actor-known inputs and returns an immutable packet; keep audit/provenance/hash fields private with typed read accessors.

### 2. Deterministic context hash

Add a hash routine in `crates/tracewake-core/src/checksum.rs` that hashes the canonical, sorted actor-known inputs (holder/bound actor, tick/frontier, allowed/forbidden sources, provenance keys). Forbid nondeterministic inputs.

### 3. Typed context id

Add a `HolderKnownContextId` newtype in `crates/tracewake-core/src/ids.rs` following the existing id-newtype conventions; re-export the packet from `epistemics/mod.rs`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify)
- `crates/tracewake-core/src/epistemics/mod.rs` (modify)
- `crates/tracewake-core/src/ids.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify)

## Out of Scope

- `EmbodiedViewModel` typed id/hash fields and the view-model builder rewrite (ticket 002).
- Affordance derivation from the sealed context (ticket 003).
- `Proposal` context fields and validation that consumes the packet (ticket 006).
- Any `ratatui`/`crossterm` dependency or visual uplift (Spec 0002 §6 — deferred, not in this phase).

## Acceptance Criteria

### Tests That Must Pass

1. A unit test builds an embodied sealed packet and asserts it carries a `holder_known_context_id`, a non-empty provenance set, holder + bound-actor identity, frontier, and a forbidden-truth audit result that lists every excluded `ForbiddenKnowledgeSource`.
2. A determinism test builds the packet twice from identical inputs and asserts byte-identical `holder_known_context_hash`; a second variant mutates one actor-known input and asserts the hash changes.
3. `cargo test -p tracewake-core` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. The sealed packet is immutable after construction; no public path mutates its provenance, audit, or hash (INV-101).
2. The `holder_known_context_hash` is a pure function of canonical actor-known inputs — no wall-clock, no unsorted iteration (INV-017/INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/knowledge_context.rs` (unit tests module) — packet field presence, forbidden-truth audit completeness, immutability.
2. `crates/tracewake-core/src/checksum.rs` (unit tests) — context-hash determinism and input-sensitivity.

### Commands

1. `cargo test -p tracewake-core epistemics::knowledge_context`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
3. The targeted module test is the correct boundary for the packet shape; the full-workspace run guards against consumer breakage from the additive change.

## Outcome

Completed: 2026-06-08

What changed:
- Added `HolderKnownContextId` and `HolderKnownContextHash` typed substrate.
- Extended `KnowledgeContext` with sealed holder-known context identity, deterministic hash, holder/bound actor identity, event frontier, provenance entries, forbidden-truth audit result, and status accessors.
- Added deterministic context-hash support over sorted canonical inputs.
- Added unit coverage for sealed packet fields, forbidden-source audit completeness, hash determinism, and input sensitivity.

Deviations from original plan:
- The existing raw view-model builders are intentionally unchanged here; ticket 002 owns routing `EmbodiedViewModel` construction through the sealed packet.

Verification results:
- `cargo test -p tracewake-core epistemics::knowledge_context` passed.
- `cargo test -p tracewake-core checksum::tests::holder_known_context_hash_is_order_independent_and_input_sensitive` passed.
- `cargo test -p tracewake-core` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
