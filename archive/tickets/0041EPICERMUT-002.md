# 0041EPICERMUT-002: Kill `belief.rs` survivors — stale-frontier freshness and observation/contradiction witness links

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — behavior-witness tests by default; conditional production correction in `crates/tracewake-core/src/epistemics/belief.rs` (and its projection/view-model consumers) only if a survivor reveals a real contract/reachability defect
**Deps**: None

## Problem

Spec 0041 §5.1–§5.2 route four EPI mutation survivors in `crates/tracewake-core/src/epistemics/belief.rs` to existing projection/replay diagnostics. They are a seed floor (§3.6), not the finish line, and must be killed by a certified EPI consequence — not a getter-asserts-getter tautology (§4.9, §12.1).

**§5.1 — belief freshness policy (`Belief::stale_after_tick`, 2 identities):**
- `replace Belief::stale_after_tick -> Option with None`
- `replace Belief::stale_after_tick -> Option with Some(Default::default())`

**§5.2 — belief witness links (`Belief::observation_ids` / `Belief::contradiction_ids`, 2 identities):**
- `replace Belief::observation_ids -> &BTreeSet with Box::leak(Box::new(BTreeSet::new()))`
- `replace Belief::contradiction_ids -> &BTreeSet with Box::leak(Box::new(BTreeSet::new()))`

Per the §9.3 static survey, these are likely **contract/reachability gaps, not automatically killable getter cases**: `Belief::new` initializes `stale_after_tick` to `None` (`belief.rs:85`), `BeliefDraft` exposes no stale-frontier field, the projection's actor-known freshness classifier (`ActorKnownProjectionRecord::freshness`, `projection.rs:224`) is a *separate* record-level policy, and the inspected projection checksum and `DebugBeliefEntry` (`view_models.rs:718`, built by `debug_belief_entry` at `projection.rs:1520`) do not themselves serialize the two link sets. The kill therefore requires establishing the real production consumer or closing the evidence-path gap — never a test-only reader (§4.12).

## Assumption Reassessment (2026-06-19)

1. Codebase check: `Belief::stale_after_tick(&self) -> Option<SimTick>` (`belief.rs:149`), `observation_ids(&self) -> &BTreeSet<ObservationId>` (`belief.rs:153`), `contradiction_ids(&self) -> &BTreeSet<ContradictionId>` (`belief.rs:157`) exist; `Belief::new` (`belief.rs:59`) sets `stale_after_tick: None` (`belief.rs:85`); `BeliefDraft` (`belief.rs:171`) exposes no stale-frontier field. The separate record-level freshness model is `ActorKnownProjectionRecord::freshness` / `ClassifiedActorKnownProjectionRecord` (`projection.rs:224`, `:242`). `DebugBeliefEntry` lives at `view_models.rs:718`. The spec's `:150/:154/:158` line numbers are cargo-mutants identities; the verified symbols above are authoritative.
2. Specs/docs check: §5.1 requires a boundary-valued stale-frontier contract exercised at ticks before/at/after the frontier with a certified consequence (freshness classification, context inclusion, supersession, notebook/view evidence, or another replay-stable EPI decision); §5.2 requires an event-backed belief whose support contains a known observation ID and a genuinely contradicting observation yielding a known contradiction ID, carried through context-filtered projection and a real diagnostic/notebook/checksum/debug consumer. §5.14 group closeout and §3.6 floor-not-finish-line apply.
3. Cross-artifact shared boundary under audit: the `Belief` accessor contract ↔ its projection/replay serialization (canonical checksum input, `debug_belief_entry`) ↔ actor-known context inclusion. The audit must determine, at the final commit, whether each accessor has a doctrinally required production consumer; if not, reconcile/remove the dead API rather than manufacture one.
4. Motivating invariants (INV restate): §10 maps belief freshness/typed-belief/provenance to `INV-026`/`INV-028`/`INV-102`/`INV-112` (source/acquisition/freshness/frontier data stays attached; truth or scheduler time does not silently refresh actor knowledge) and `INV-021`–`INV-031` (epistemic currency is typed; important belief has provenance; wrong/stale belief remains possible; no telepathy via `INV-024`). The kill must witness one of these consequences, not a literal accessor value.
5. Fail-closed / actor-knowledge / replay surface: the enforcement surface is the epistemic projection rebuild and its canonical checksum input plus actor-known context filtering. Confirm the witness travels the production projection/replay path (live application + deterministic rebuild agree on belief IDs, link sets, stale policy, holder, source event) and that no other-actor observation/contradiction attaches across the privacy scope. The §5.1 negative control (authoritative truth changes after the belief forms, with no perception/notice event) must show the actor-known surface ages/retains the belief by its recorded frontier and does not refresh from truth (no epistemic leakage; deterministic replay preserved).
6. Schema extension (additive-vs-breaking): if killing §5.2 requires the projection canonical checksum input or `DebugBeliefEntry` (`view_models.rs:718`) to serialize the observation/contradiction link sets so a production consumer can observe witness-chain integrity, that is an **additive** extension of the projection evidence/view-model schema. Consumers: replay rebuild (checksum determinism) and the debug-beliefs view. It must remain additive (new serialized fields with deterministic ordering via the existing `BTreeSet` iteration) — never breaking, and never leaking another actor's links into a context-filtered read. If no production consumer is doctrinally required, no schema change is made and the redundant accessor is reconciled/removed instead (recorded in the triage register, ticket 009).

## Architecture Check

1. Establishing a real replay/provenance-backed consumer (or reconciling a genuinely dead API) is cleaner than asserting `stale_after_tick()`/`observation_ids()` literals: it ties the kill to the EPI behavior the accessor exists to serve (freshness classification, witness-chain integrity in evidence) and prevents a getter-tautology that would pass while the contract rots. A boundary-valued frontier test (before/at/after) and a member-distinguishing link assertion are non-tautological by construction.
2. No backwards-compatibility aliasing/shims: no test-only accessor, no alternate freshness model bridging `Belief::stale_after_tick` to `ActorKnownProjectionRecord::freshness`, no `#[mutants::skip]`, no exclusion glob. A removed redundant API is deleted with semantic reconciliation, not aliased.

## Verification Layers

1. INV-026/102/112 (freshness/frontier; no silent refresh from truth) -> replay/golden-fixture check: a stale-frontier belief reclassifies across the before/at/after-frontier boundary and a no-new-observation advance does not refresh it.
2. INV-021–031 + INV-024 (typed belief, provenance, no telepathy) -> replay/golden-fixture check: the support observation and contradiction resolve to the correct holder/channel/source event and link to the belief; another actor's observation does not attach.
3. Deterministic replay of belief link sets / stale policy -> replay/golden-fixture check: live application and rebuild agree on belief IDs, link sets, stale frontier, and canonical checksum input.
4. Member-level mutant catch -> `cargo mutants -f crates/tracewake-core/src/epistemics/belief.rs` reports all four identities `caught` (or a §4.11-reviewed disposition).

## What to Change

### 1. Establish/witness the belief stale-frontier consequence (§5.1)

Determine the final-commit constructibility and call-site census for `Belief::stale_after_tick`. Where upstream belief doctrine requires the field, establish one checked production path that derives or admits a belief stale frontier from typed event/schema policy, persists it through the real projection/replay path, and consumes it in an actor-known or evidence surface. Add a boundary-valued test exercising ticks before, exactly at, and after the frontier, proving a certified consequence (freshness classification / context inclusion / supersession / notebook-view evidence). Include: an event-backed belief with an explicit nonzero stale frontier; the same holder/proposition observed at a newer tick to prove supersession where applicable; an event/replay rebuild whose belief representation matches the live application; and a no-new-observation advance proving stale knowledge does not refresh from truth. Under that contract the `None` mutant fails (frontier disappears) and the `Some(Default::default())` mutant fails (frontier wrong). If the reachable domain cannot produce `Some(_)` or no consumer observes the accessor, treat it as a dead/unfulfilled contract — narrow/remove the API with reconciliation, or submit the rare §4.11 equivalence/non-critical case with full-domain + call-site proof and independent signoff (recorded in ticket 009's register).

### 2. Witness the observation/contradiction link chain (§5.2)

Build one event-backed belief whose support contains a known observation ID, then apply a genuinely contradicting observation yielding a known contradiction ID. Carry the belief through the public/context-filtered projection and a real diagnostic, notebook, checksum/evidence record, or debug view that exposes witness-chain integrity — closing the evidence-path gap if the current checksum/`DebugBeliefEntry` omit the link sets (additive extension per AR item 6). Prove: the support observation exists in the replayed projection and belongs to the correct holder/channel/source event; the belief links to that observation, not merely to a count; the contradiction links the same expectation and contradicting observation; and replay reproduces the same link sets and canonical evidence/fingerprint. A single table-driven family may kill both mutants but must identify which observation-link assertion fails for `observation_ids` and which contradiction-link assertion fails for `contradiction_ids`.

### 3. Negative/contamination controls

§5.1: change authoritative truth after the belief forms without a perception/notice event; the actor-known/embodied surface must retain or age the belief by its recorded frontier and not refresh from truth. §5.2: an observation for another actor, an observation outside the privacy scope, an unrelated contradiction, and a forged raw ID/prose assertion must fail closed; debug may inspect the chain but may not supply or repair it.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — primary belief link/replay witness, as surfaced)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — stale-frontier no-refresh-from-truth witness, as surfaced)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — context-inclusion/evidence consequence, as surfaced)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — debug-view witness-chain verification, only if the chain is surfaced there)
- `crates/tracewake-core/src/epistemics/belief.rs` (modify — only if a survivor reveals a real defect or a redundant accessor must be reconciled/removed)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — only if the canonical checksum/`debug_belief_entry` must serialize the link sets to establish a consumer)
- `crates/tracewake-core/src/view_models.rs` (modify — only if `DebugBeliefEntry` must serialize the link sets)

## Out of Scope

- Killing survivors in `contradiction.rs`, `observation.rs`, or `proposition.rs` (tickets 003–008).
- Inventing a test-only consumer or routing belief data through an arbitrary new UI label to improve the mutation count (§4.12).
- Adding any belief survivor to `.cargo/mutants-baseline-misses.txt` (§4.3).
- The full standing campaign / triage register (ticket 009) and the EPI-01…11 re-proof / acceptance artifact (ticket 010).
- Any `ORD-LIFE-CERT` or later-gate belief-domain expansion (§11.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/epistemics/belief.rs` — each of the four historical identities is `caught` by a behavior witness, OR carries a §4.11 equivalent/non-critical disposition with full-domain + call-site proof and independent signoff:
   - `stale_after_tick -> None` — caught (required frontier disappears)
   - `stale_after_tick -> Some(Default::default())` — caught (frontier wrong)
   - `observation_ids -> empty` — caught (support-observation link lost)
   - `contradiction_ids -> empty` — caught (contradiction link lost)
2. The new/modified witnesses pass against the unmutated final implementation and fail when each specific mutant is active (mutant-active failure evidence retained per member).
3. `cargo test --workspace --locked` — clean baseline.

### Invariants

1. No witness asserts a `Belief` accessor literal in isolation; each observes a projection/replay/actor-known consequence (freshness classification, context inclusion, witness-chain integrity).
2. Deterministic replay holds: live application and rebuild agree on belief IDs, link sets, stale frontier, and canonical checksum input; no other-actor link or hidden-truth refresh enters the context-filtered read.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — event-backed belief with observation/contradiction links carried through context-filtered projection + replay rebuild; member-distinguishing assertions for `observation_ids`/`contradiction_ids`.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — stale-frontier boundary (before/at/after) + no-refresh-from-truth negative control for `stale_after_tick`.
3. `crates/tracewake-core/tests/acceptance_gates.rs` / `crates/tracewake-tui/tests/adversarial_gates.rs` — context-inclusion / debug-view witness-chain consequence, as surfaced by the final consumer.

### Commands

1. `cargo mutants -f crates/tracewake-core/src/epistemics/belief.rs`
2. `cargo test --workspace --locked`
3. `cargo mutants -f <file>` is the correct per-ticket boundary: it regenerates exactly this file's mutants so the four identities' catch/disposition is provable in isolation before the full campaign (ticket 009) reconciles them against the complete denominator.

## Outcome

Completed: 2026-06-19

Implemented an additive production evidence path for the `belief.rs` stale-frontier and witness-link contract. `BeliefUpdated` event payload parsing now admits `last_verified_tick`, `stale_after_tick`, `observation_ids`, and `contradiction_ids`; `Belief` gained a `with_stale_after_tick` builder; projection checksums now serialize `stale_after`, observation links, and contradiction links; and `DebugBeliefEntry` now exposes the same witness-chain fields for debug evidence.

Added `belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay` in `crates/tracewake-core/tests/event_schema_replay_gates.rs`. The witness applies event-backed observation, contradiction, and belief records; checks before/at/after stale-frontier ticks; proves the observation and contradiction IDs survive through context-filtered projection, canonical checksum input, debug belief view, and replay rebuild; and verifies another actor's embodied context does not receive the private belief.

Verification:

- `cargo test -p tracewake-core --test event_schema_replay_gates belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay` passed.
- `cargo test -p tracewake-core --test event_schema_replay_gates` passed.
- `cargo test --workspace --locked` passed.
- `cargo fmt --all --check` passed.
- `cargo mutants --no-config -f crates/tracewake-core/src/epistemics/belief.rs --test-workspace true -C=--locked` completed with 34 mutants tested: 17 caught, 17 unviable, 0 missed. The four historical identities are listed in `mutants.out/caught.txt`:
  - `Belief::stale_after_tick -> None`
  - `Belief::stale_after_tick -> Some(Default::default())`
  - `Belief::observation_ids -> Box::leak(Box::new(BTreeSet::new()))`
  - `Belief::contradiction_ids -> Box::leak(Box::new(BTreeSet::new()))`

Deviations: the literal `cargo mutants -f crates/tracewake-core/src/epistemics/belief.rs` command loaded the checked-in `.cargo/mutants.toml` and began the full configured 2776-mutant campaign; it was interrupted before final output because ticket 009 owns that full campaign. The file-local proof was rerun with `--no-config` so `-f` actually scoped to `belief.rs`; the checked-in config remains unchanged and will be exercised by ticket 009.

No `.cargo/mutants-baseline-misses.txt` entry was added. No §4.11 equivalent or non-critical disposition was used.
