# 0063 Core Actor-Known Co-Present Observed-Activity Enrichment Spec

**Status**: PROPOSED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec C** — the **headline, crate-crossing** item of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §4, §7). It makes co-present actor activity
glanceable ("Mara is working; Tomas's activity is not apparent; someone was heard, uncertain") by
enriching the **core** actor-known view model, produced only through the modeled
perception/projection/truth-firewall seam — never derived by the TUI.

## 0. Baseline statement and source discipline

- **Driver.** The research report's central product finding: `EmbodiedViewModel.local_actors:
  Vec<VisibleActor>` exists, but `VisibleActor` is identity-only, so activity is not glanceable.
  Verified: `crates/tracewake-core/src/view_models.rs` defines `pub struct VisibleActor { pub
  actor_id: ActorId }`; the renderer prints only `actor.actor_id`
  (`crates/tracewake-tui/src/render.rs`); perception records only
  `ActorKnownLocalActorFact::new(observed_actor_id, source_key)` from
  `ActorKnownProjectionRecord::LocalActor` (`crates/tracewake-core/src/agent/perception.rs`,
  `crates/tracewake-core/src/epistemics/knowledge_context.rs`).
- **Baseline commit.** Re-verified against repository `HEAD`
  `85dd8836508a58305eb84d70caf52bda088a9bde`. Named symbols are authoritative; line numbers are
  provenance only.
- **Source discipline.** Commit hashes are audit/provenance only. No latest-`main` certification,
  Phase-4 entry, or second-proof entry is declared. A manifest is path inventory only.
- **Authority posture.** Subordinate to foundation → architecture → execution → reference. Routes
  doctrine sharpening as *substance + home* (§5); ratifies no wording; mints no id.
- **Execution admissibility posture.** `P0-CERT not applicable`.
- **Ledger timing.** Archived-row entry at acceptance/closeout, not at proposal. No ledger row now.

## 1. Scope

### 1.1 In scope

Extend the actor-known co-present awareness data across `tracewake-core` (and supporting
`tracewake-content` fixtures and `tracewake-tui` rendering/tests):

1. **View-model enrichment.** Extend `VisibleActor` from identity-only to a typed actor-known
   presence/activity view: a `display_label`, a presence descriptor (source summary, observed
   tick, staleness label, optional uncertainty label), and an optional observed-activity view
   (closed activity kind, actor-safe summary, source kind, source summary, observed tick, staleness,
   uncertainty). Names are authoring choices; the required properties are **closed enums**,
   **actor-safe summaries**, **source/freshness labels**, and **no UI-framework types**.
2. **Closed activity taxonomy.** A closed `ObservedActorActivityKind` (start coarse: `Sleeping`,
   `Eating`, `Working`, `Moving`, `Speaking`, `Waiting`, `ContinuingRoutine`, `ApparentIdle`,
   `ActivityNotApparent`) and a closed `ActorKnownActivitySourceKind` (`DirectPerception`,
   `IndirectPerception`, `Memory`, `Testimony`, `Record`, `Inference`) mapped to foundation-04
   information sources.
3. **Projection/perception source.** Model public/observable activity as a typed
   observation/projection record; include it during actor-known context construction **only** when
   the possessed actor's modeled perception can capture it; seal it into holder-known context with
   provenance and frontier/freshness. Extend the `ActorKnownProjectionRecord::LocalActor` family (or
   an adjacent record) rather than adding a TUI-side lookup.
4. **`build_embodied_view_model` transfer.** `build_embodied_view_model`
   (`crates/tracewake-core/src/projections.rs`) transfers the sealed actor-known activity into
   `VisibleActor`, preserving deterministic ordering.
5. **Row taxonomy semantics.** Direct observed activity; indirect/uncertain; stale remembered
   (mandatory stale wording); visible-but-activity-unknown (`activity not apparent`); and
   hidden/occluded/unperceived → **no row** (omission is correct).
6. **Fixture support.** `tracewake-content` gains typed fixtures exercising visible sleeping/eating/
   working, present-but-unknown, hidden/occluded (absent), and stale-remembered cases.

### 1.2 Out of scope (non-goals)

- No `ratatui`/`crossterm` or any UI-framework type in core view models.
- No TUI derivation of activity from `actor_id`, action registry, routine state, scheduler state,
  or physical actor state.
- No collapsing of hidden internal intentions into visible activity ("wants to steal bread" is not
  an observable activity absent a modeled actor-known source).
- No exact hidden routine/target/job/culprit label exposure.
- The final rendered actor pane layout is Spec `0064`; the fullscreen shell is Spec `0065`.
- Identity-uncertain "someone nearby" rows are added **only** if the current epistemic model can
  represent presence without a known `ActorId` (report open-question §9.1 #3); otherwise start with
  identified visible actors and defer uncertain-presence rows.

## 2. Dependencies and ordering

- **Depends on:** `0061` for the `EmbodiedScreenModel`/`ScreenDump` field contract that will carry
  the new activity fields into dumps; may begin once that contract is published.
- **Blocks:** the *final* state of the actor pane in Spec `0064`, and the co-present parity rows in
  Spec `0069`.
- **Crate-crossing:** `tracewake-core` (view model, projection, perception, knowledge context),
  `tracewake-content` (fixtures), `tracewake-tui` (rendering + tests). This is the only roadmap spec
  that changes `tracewake-core`.

## 3. Doctrine anchors

- **Foundation 14** (`…/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`) and
  **Foundation 04** (`…/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`): actor-known context
  governs cognition/presentation; legitimate sources are perception, memory, inference, testimony,
  records — never hidden truth.
- **Architecture 03** (`…/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`): co-present
  activity is holder-known context material with mandatory provenance/freshness.
- **Architecture 06** (`…/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`):
  public-activity observations are typed records that may later become beliefs/memories/leads under
  contradiction/staleness.
- **Architecture 10**: expands the embodied view-model contract from visible-actor identity to
  visible-actor awareness; every new field/variant needs an explicit two-hop presentation
  disposition.
- **Execution 04 / 07** (`…/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`,
  `…/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`): anti-contamination and view-model
  proof obligations.

## 4. Findings and remediation requirements

- **4.1 Activity is a modeled observation, not a display string (seam:
  `epistemics/projection.rs`, `agent/perception.rs`).** A co-present activity row is legal only when
  core has produced an actor-known observation/projection item with provenance and freshness.
- **4.2 Perception gate (seam: `current_place_knowledge_context` in `perception.rs`).** Observed
  activity is included only when the possessed actor's modeled perception captures it; otherwise the
  row is omitted or labeled unknown. No TUI physical-state handle is ever introduced.
- **4.3 Freshness discipline (seam: holder-known context provenance).** Stale activity must carry
  mandatory stale/uncertain wording; core carries enough provenance for tests, and the embodied UI
  displays actor-relevant labels (report open-question §9.1 #2), consistent with INV-112 (holder-known
  time).
- **4.4 Closed types force explicit presentation (seam: `view_models.rs`).** No wildcard/default
  renderer path; each activity kind/source needs explicit presentation disposition, enforced by the
  Spec `0069` conformance guard.
- **4.5 What not to do (seam: whole change).** No activity from `actor_id`/registry/routine/scheduler/
  physical state; no UI types in core; no intention-to-activity collapse; no hidden routine/target/job/
  culprit; no stale-looks-current; no wildcard-to-debug-string laundering.
- **4.6 Implementation decomposition.** Closed enums + `VisibleActor` fields; observation/projection
  record + perception capture; holder-known sealing + provenance/freshness; `build_embodied_view_model`
  transfer; content fixtures; TUI render disposition + anti-leak tests. Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — embodied view-model contract expands to visible-actor awareness; two-hop
  disposition required for every new field/variant.
- **Architecture 03** — co-present public-activity observations are holder-known context material when
  captured by modeled observation, with mandatory provenance/freshness.
- **Architecture 06** — define public-activity observation flow into observations/beliefs/memories/
  leads. All routed also through Spec `0070`; substance only.

## 6. Required fixtures and tests

- **Positive:** co-present actor visibly sleeping/eating/working renders a typed activity row with
  source + freshness.
- **Negative (anti-leak):** truth has an activity but the possessed actor does not know it → no
  activity row (omission), proving the firewall.
- **Occlusion:** actor sleeping behind a closed door with no modeled cue → no row.
- **Stale:** previously observed activity with no fresh observation shows mandatory stale wording;
  removing the stale label fails a test.
- **Possession parity:** the same fixture under two possessions yields each actor's own actor-known
  activity view; neither sees the other's hidden state.
- **Two-hop:** Hop 1 core event/perception → `VisibleActor.observed_activity`; Hop 2
  `VisibleActor.observed_activity` → rendered/dumped actor pane. A mutant removing either hop fails.

## 7. Acceptance artifact and evidence

A review artifact recording positive/negative/occlusion/stale/possession-parity fixtures, the two-hop
evidence rows, and the anti-contamination results, at the exact implementation commit. No decorative
locks: each guard carries a behavior witness (report §6.5).

## 8. Implementation constraints

- `tracewake-core` view models remain plain domain types with zero UI dependency; the crate's
  zero-normal-dependency posture is preserved.
- Additive schema change: consumers of `VisibleActor` (`build_embodied_view_model`, `render.rs`,
  tests) are updated in the same package; no back-compat shim.
- Deterministic ordering of actors and their activity rows.

## 9. Risks and open questions

- **Risk: co-present activity leaks truth** — the feature's value lives exactly where hidden-truth
  temptation is strongest. Mitigation: core-only actor-known projection, anti-leak fixtures,
  stale/uncertain labels, no TUI physical-state handle.
- **Risk: activity becomes prose-born fact.** Mitigation: closed enums + actor-safe summaries bound to
  a typed source; no freeform TUI derivation.
- **Open question (§9.1 #1):** taxonomy granularity — start coarse, attach actor-safe summary text for
  flavor.
- **Open question (§9.1 #3):** identity-uncertain presence — added only if core can model it.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-099 (truth may validate, not plan) | aligns | Activity enters presentation only via a modeled actor-known channel. |
| INV-100 (hidden-truth cognition forbidden) | aligns | Perception gate; no physical-state read for activity. |
| INV-101 (actor-known context is sealed) | aligns | Activity is sealed into holder-known context, not validator truth. |
| INV-102 (cognition inputs require provenance) | aligns | Each activity view carries source + freshness provenance. |
| INV-006 (possession transfers no world knowledge) | aligns | Possession-parity fixtures prove each actor sees only its own actor-known activity. |
| INV-022 (raw prose is not authoritative state) | aligns | Actor-safe summaries attach to typed sources; they create no facts. |
| INV-112 (holder-known time must plan) | aligns | Staleness labels derive from holder-known freshness, not free truth time. |
