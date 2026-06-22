# 0046 TUI/Simulation Playable-Capability Parity Contract and Two-Hop Drift-Guard Hardening Spec

This is a staged hardening spec in the parallel `specs/NNNN` series. It is staged in
`specs/` and is promoted to `archive/specs/` on acceptance; it is never promoted to the
live `docs/4-specs/` tier, and it does not amend constitutional invariants, define gate
semantics, or weaken execution gates. It uses the canonical hardening-spec house
structure of its sibling specs (e.g. `0025`), not the `docs/NN_*` narrative-document
style.

## 0. Baseline statement and source discipline

- **Driver.** `reports/tui-parity-research-report.md`, a recommendation-altitude deep-research
  report on TUI/simulation feature-parity drift. The report is the originating analysis; it is
  not itself doctrine and minted no spec, invariant, risk identifier, or gate code.
- **Report target commit.** The report was conducted against `1145e109aa50721b37f7d343caef518b3be5fc7f`,
  which is the current repository `HEAD` at authoring time. Every load-bearing code claim cited
  below was re-verified against that working tree (see §3). The named symbols are authoritative;
  any line numbers in the source report are not relied upon.
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-main certification, Phase-4 entry, or
  second-proof entry. When executed, the implementation must name its own exact implementation
  commit, not assume this baseline commit is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its `docs/4-specs/SPEC_LEDGER.md`
  archived-row entry at acceptance/closeout, not at proposal. This spec authors no ledger row now
  and makes no change to live `0001` or the ledger.

## 1. Scope

### 1.1 In scope

The standing **two-hop playable-capability parity contract**: a layered, change-coupled
obligation that no declared playable capability can be accepted without an explicit actor-facing
surface disposition and passing real-pipeline evidence, and no growth of the closed core→TUI
presentation contract can compile without a conscious renderer decision.

1. **Hop 2 (view model → rendered text), compiler-enforced** — exhaustive no-`..` destructure of
   `EmbodiedViewModel` at the renderer boundary, a local `unused_variables` deny, no-wildcard
   exhaustive matches at closed-enum presentation owners, and a source-conformance guard that
   protects the tripwire.
2. **Hop 1 (simulation capability → actor-filtered view model), behaviorally enforced** — focused
   real-pipeline golden scenarios per playable capability, asserting typed causal witnesses before
   rendered witnesses, with paired anti-leak cases for epistemic features.
3. **A test-side playable-capability registry and conformance runner** that binds every declared
   capability — including every registered semantic action — to a surface disposition, fixture,
   typed witness, rendered witness, and required anti-leak witness, and emits a deterministic
   coverage report.
4. **A standing per-feature obligation** — every future feature spec declares its parity impact and
   carries the resulting evidence; the conformance suite runs in ordinary CI on every change.
5. **The doctrine amendments** at the legitimate tier homes identified by the report (§5), and the
   acceptance-artifact and reference-tier updates that make the obligation discoverable and reviewable.
6. **A baseline classification** of the current base playable capabilities and every currently
   registered action, with explicit, justified debug-only/headless exceptions.

### 1.2 Out of scope (non-goals)

- TUI modernization, richer terminal layout, or any `ratatui`/`crossterm` uplift.
- New Phase-4 institutions, notices, travel, regional, or LOD behavior.
- Implementation of domain packs, or any fantasy/post-apocalyptic content.
- Any change that exposes hidden truth or debug truth to embodied play.
- TUI-owned simulation rules, an action-kind switch in the TUI, or direct event application.
- Marking `EmbodiedViewModel` (or any internal core→TUI contract type) `#[non_exhaustive]`.
- Minting a new constitutional invariant, risk identifier, or certification gate code; reopening
  any passed certification; amending any archived spec.
- Any `SPEC_LEDGER.md` declaration or rewrite of live `0001` by this proposal.
- Backwards-compatibility aliases or shims.

## 2. Doctrine anchors

The principle is already constitutional; the defect is missing structural enforcement, not missing
intent. This spec operationalizes — and does not amend — the following:

- **Foundation.** `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (kernel and TUI advance together;
  per-feature review rule); `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` (TUI/view-model
  reachability, actor filtering, replay, regression, no-human continuation are part of "done");
  `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` (hidden truth is not an embodied
  proof shortcut).
- **Invariants** (verified wording): `INV-065` TUI is a primary product interface; `INV-066` every
  runnable phase has a TUI/view-model gate; `INV-067` embodied mode shows actor-known reality;
  `INV-068` debug mode is visibly non-diegetic; `INV-069` the TUI must not implement simulation
  rules; `INV-070` why-not explanations are mandatory; `INV-078` kernel is genre-agnostic; `INV-080`
  domain packs own flavor; `INV-091` no-human tests are mandatory; `INV-094` possession parity is
  tested; `INV-095` TUI/view-model tests are acceptance tests; `INV-099`/`INV-100`/`INV-101` truth
  firewall and sealed actor-known context; `INV-107` debug omniscience is quarantined.
- **Architecture.** `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`,
  `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`.
- **Execution.** `00_EXECUTION_INDEX_AND_AUTHORITY.md`, `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`,
  `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`,
  `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.
- **Reference.** `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`; `01_DESIGN_RISK_REGISTER.md` `R-15`
  (TUI-First Playability Erosion); `02_GLOSSARY.md`.
- **Specs tier.** `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`; `README.md`.

## 3. Determination

### 3.1 Verified holdings (no action; recorded so they are not re-litigated)

Re-verified against the working tree at the baseline commit:

- `crates/tracewake-core/src/view_models.rs::EmbodiedViewModel` is a closed workspace struct with
  21 public fields and **no** `#[non_exhaustive]`. `SemanticActionEntry` is a struct, not an
  enum of action kinds. Closed presentation enums in `view_models.rs`: `ActionAvailability`,
  `ActionAvailabilityProvenanceKind`, `WhyNotFailureKind`, `VisibleItemSource`, `DebugViewModel`.
  `ActionEffect` is a separate closed enum, but it lives in
  `crates/tracewake-core/src/actions/registry.rs` as an action-pipeline effect classifier
  (`QueryOnly`, `Move`, `Open`, `Close`, `Take`, `Place`, `Wait`, `CheckContainer`, `Sleep`, `Eat`,
  `Work`, `ContinueRoutine`); it currently has **no** actor-facing or debug-facing textual
  presentation owner (no `ActionEffect` reference exists in `render.rs` or `view_models.rs`).
- `crates/tracewake-tui/src/render.rs::render_embodied_view(&EmbodiedViewModel) -> String` reads
  fields à la carte with **no** leading exhaustive destructure. `visible_item_source_label`
  already matches `VisibleItemSource` exhaustively with **no** wildcard arm — the pattern this spec
  generalizes.
- `crates/tracewake-tui/tests/transcript_snapshot.rs` compares two runs of the same implementation
  to each other (`assert_eq!(first_sections, second_sections)`); it is a determinism test, **not**
  an approval/golden test against a checked-in accepted rendering.
- `crates/tracewake-core/src/actions/registry.rs::ActionRegistry::definitions()` enumerates
  registered actions via `ActionDefinition { action_id, scope, effect }`.
- `crates/tracewake-core/src/projections.rs::build_embodied_view_model` gathers actor-known routes,
  doors, containers, items, local actors, and semantic actions; `build_notebook_view` exists.
- Existing suites to **retain** prove adjacent safety but not completeness under feature growth:
  `tui_seam_conformance.rs`, `embodied_flow.rs`, `adversarial_gates.rs`, `tui_acceptance.rs` (with
  its source-conformance guards that public render functions have production callers), the
  `GoldenFixture` `all`/`by_id` substrate, and the CI lock-layer suites.

### 3.2 Validated — no action

- **Foundation already owns the principle.** No foundation amendment. Adding test mechanics or
  registry fields to the constitution would overfit it and route below the highest legitimate
  owner. The existing foundation clauses are cited as drivers for the architecture/execution
  amendments, not replaced.
- **No new invariant, risk ID, or gate code.** A new top-level parity gate would duplicate the
  ladder and invite audit lag; folding parity only into the passed `EPI-CERT`/`FIRST-PROOF-CERT`
  packages would not bind future expansion. The correct posture is a standing per-feature
  obligation plus the ordinary CI lane.
- **`#[non_exhaustive]` is the wrong tool.** It exists to grant downstream tolerance to schema
  growth; this contract wants deliberate breakage at the controlled in-workspace core→TUI boundary.
- **Live ledger and `0001` unchanged now.** This spec recommends but does not declare numbered
  execution work into the ledger; live fixture/ontology doctrine is reused, not rewritten.

## 4. Findings and remediation requirements

Requirement IDs use the `PAR-` prefix. Each requirement names its failure mode: the build, a
source-conformance guard, a typed assertion, an anti-leak assertion, or a coverage-matrix entry
must fail before a non-conforming change can be accepted.

### 4.1 Hop 2 — compile-time view-model exhaustiveness

**PAR-001 — Exhaustive `EmbodiedViewModel` destructure at the renderer boundary.**
At the start of `render_embodied_view`, destructure `EmbodiedViewModel` naming every field with no
`..`. Render from the bound names rather than scattered `view.field` reads. Place an
`unused_variables` deny local to the renderer (or its module), independent of workspace CI. Every
field receives exactly one disposition: **rendered/consumed** by the embodied output or a delegated
helper, or **intentionally not rendered on this surface**, expressed as an underscore-prefixed named
binding with an adjacent rationale identifying the correct alternate owner or why it is internal
metadata. A bare `_`, `field: _`, or rest pattern for omission is forbidden.

**PAR-002 — Source-conformance guard for the destructure.**
Add a TUI source-conformance test, in the existing seam/conformance style, that asserts the real
renderer contains the exhaustive destructure and that the pattern contains no `..` and no bare
wildcard field disposition. This protects the tripwire against a later "cleanup" that removes it
while preserving output.

**PAR-003 — Exhaustive no-wildcard matches at closed-enum presentation owners.**
Where a closed enum variant selects actor-facing or debug-facing presentation, the presentation
owner matches exhaustively with no `_` arm. Preserve the existing `VisibleItemSource` handling and
extend the rule to presentation over `ActionAvailability`, `ActionAvailabilityProvenanceKind`,
`WhyNotFailureKind`, and `DebugViewModel` at rendering boundaries. `ActionEffect` (in
`actions/registry.rs`) is included **forward-conditionally**: it has no render-boundary presentation
owner today, so the obligation binds only if and when a presentation owner for it is introduced — no
actor-facing presentation may be invented merely to satisfy this guard (see §8). The rule binds at
the **semantic presentation owner**: if core owns an actor-safe textual summary, core's match must
be exhaustive and the TUI may call it. The TUI must not duplicate simulation rules to obtain an
exhaustive match. `SemanticActionEntry` action-ID completeness is **not** covered here — it belongs
to PAR-007.

*Catches:* a new view-model field; a new closed presentation variant; a named-but-unused field;
removal of the tripwire (via PAR-002). *Does not catch:* a wrongly rendered value; a poorly
justified underscore omission; a missing projection; a data-driven action ID absent from the
projection (→ §4.2/§4.3).

### 4.2 Hop 1 — focused real-pipeline capability goldens

**PAR-004 — Per-capability real-pipeline scenario.**
For each playable capability, define at least one focused positive scenario running the real path:
obtain a manifest-backed `GoldenFixture`; load through the ordinary content path; possess/bind the
designated actor through `TuiApp`; advance or replay through the normal scheduler/event path as
required; build `TuiApp::current_view` and call `render_current_view`; for actionable capabilities,
select and submit a real `SemanticActionEntry` through `TuiApp::submit_semantic_action`; re-render
and, where doctrine requires, rebuild/replay and compare the actor-filtered result.

**PAR-005 — Ordered witnesses, typed before rendered, with anti-leak pairing.**
Each scenario asserts in this order: (1) **typed causal witness** — expected event, proposal,
validation report, projection record, view-model field, or action entry; (2) **actor-knowledge
witness** — the fact is available through the embodied context, with source/freshness where
relevant; (3) **rendered witness** — checked-in expected text; (4) **negative/anti-leak witness** —
for epistemic or hidden-state features, an unknown/stale/contradictory/unobserved case stays absent
or disabled with an actor-safe why-not; (5) **debug witness, if required** — separate evidence that
debug diagnoses the mechanism without satisfying the embodied assertion. For autonomous-agent
features, the embodied witness proves only legitimately observable consequence (presence, movement,
work, environmental change, communication, records); private intentions and hidden truth must not be
surfaced to make autonomy visible.

**PAR-006 — Checked-in accepted renderings; preserve determinism test.**
Add focused per-capability checked-in expected renderings generated through the real TUI path.
Prefer the repository's explicit byte-stable style; `insta` is an optional ergonomics choice, not a
requirement. Keep snapshots short and reviewable. Preserve `transcript_snapshot.rs` as the
determinism check; do not reinterpret run-A-equals-run-B as content acceptance. A single broad
integrated transcript may supplement, but must not be the only completeness evidence.

*Catches:* projection blindness in `build_embodied_view_model`; app glue that fails to supply/refresh
an actor-known source; action definitions not appearing as semantic entries under the right
conditions; actor-known facts lost between projection and render; hidden truth leaked to fake
visibility; stable-but-incomplete output. *Does not catch:* an undeclared capability; every
interaction among capabilities; a dishonest witness accepted by reviewers.

### 4.3 Playable-capability registry and conformance runner

**PAR-007 — Test-side capability registry.**
Create a **test-side** registry in a downstream crate or integration-test support module that may
depend on both `tracewake-core` and `tracewake-content`. It must not be authoritative simulation
state, must not be consumed by cognition/scheduling/validation/event application, and must not make
core depend on content or TUI. Each entry declares: a stable capability key; ownership scope (base
now, or a namespaced future pack); capability class (semantic action / actor-observable state /
actor-observable consequence / notebook-record surface / debug-only infrastructure / headless
infrastructure); the required surface disposition and why it is legitimate; one or more fixture IDs;
the possessed viewer actor and setup/advance operation; the typed witness; the rendered witness or
golden; required negative/anti-leak fixtures for epistemic capabilities; and replay/no-human
evidence flags where doctrine applies. "Debug-only" and "headless" are explicit dispositions, not
escape hatches: a feature intended for ordinary play may not be classified debug-only merely because
no embodied design was implemented. The final ratified term for this mechanism is an owner decision
(see §9); "playable-capability parity contract" is a working label.

**PAR-008 — Generic conformance runner with deterministic coverage report.**
The runner fails when: capability keys are duplicated, empty, or non-deterministically ordered; an
entry lacks a fixture, typed witness, or required rendered witness; a referenced fixture ID is
absent from `fixtures::by_id`; an epistemic capability lacks an anti-leak witness; an actionable
capability cannot find its declared action in the actor's `semantic_actions` under its positive
scenario; a registered action from `ActionRegistry::definitions` has no capability disposition and
fixture coverage; a capability marked embodied renders empty/unchanged at its declared observation
point; a capability declared debug-only is classified ordinary-playable by its spec/architecture; or
a future active pack has no profile or contains uncovered entries. The runner emits a deterministic
coverage report (capability count, class, fixture IDs, typed witness, rendered witness, negative
witness, replay/no-human status, pass/fail) suitable for the acceptance artifact.

**PAR-009 — Baseline capability and action census.**
Enumerate every current `ActionRegistry::definitions` entry and every current base playable
capability family, mapping each to a registry entry with coverage. Likely minimum non-action
families: epistemic filtering, no-human autonomy, needs/routines, notebook/leads, rejection/why-not,
and debug quarantine. Document each explicit debug-only/headless exception with its justification.
The baseline matrix must show zero uncovered entries at acceptance.

*Catches:* declared capabilities with no fixture/surface proof; new registered actions with no TUI
coverage; stale fixture references; per-pack holes once packs exist; "tests exist somewhere" claims
that map to nothing. *Does not catch:* a developer falsely declaring a genuine new capability as "no
parity impact"; semantic interactions outside the selected scenarios; an incorrect classification
accepted by reviewers — these are governance/review failures addressed by §4.4.

### 4.4 Standing per-feature obligation

**PAR-010 — Per-feature parity-impact declaration.**
Every future feature spec must carry a parity-impact section that lists each added/changed playable
capability and its surface disposition; identifies any Hop-2 schema/variant changes; names its
positive, negative, replay, and no-human witnesses; states whether embodied, notebook, and debug
surfaces change; states "no parity impact" only with a reason tied to architecture; and includes the
conformance report in its acceptance artifact. A playable feature is not accepted on core tests
alone; if it is intentionally headless infrastructure, the spec must establish why no actor-facing
surface is owed.

**PAR-011 — CI integration.**
Wire the conformance suite and the focused goldens into the ordinary workspace/CI evidence lane,
under the existing warnings-denied posture, so the parity obligation runs on every change rather than
as a periodic audit.

**PAR-012 — Mutation/adversarial closure.**
Prove the guards bite: removing the exhaustive destructure or the PAR-002 source guard, dropping a
capability witness, leaking debug/hidden truth into an embodied assertion, or registering an action
with no coverage must each cause a failure. Include a controlled compile-break transcript showing a
temporary added view-model field and a temporary added closed presentation variant are rejected until
dispositioned.

## 5. Doctrine amendments

These amendments are deliverables of this spec, enacted via the ticket decomposition (mirroring how
sibling amendment specs were enacted by their ticket series). Each routes to the highest legitimate
owner and no lower; none amends an invariant or mints a gate code.

**PAR-DOC-001 — Architecture `10`.** Own, at architecture altitude: the two-hop decomposition; that
closed core→client presentation contracts are deliberately breaking when they grow; an explicit
presentation disposition for every view-model field and closed presentation variant; that every
playable capability has an actor-filtered surface contract or an architecture-justified non-playable
classification; a prohibition on using debug or raw truth to satisfy embodied parity; and that
semantic actions remain data-driven pipeline entries, not TUI-owned rules. No test filenames,
snapshot libraries, or command lines.

**PAR-DOC-002 — Architecture `13`.** Own that a feature's declared playable capabilities form a
conformance set, each member mapping to typed causal evidence, actor-knowledge evidence, a surface
disposition, rendered evidence where playable, negative evidence where hidden/stale states matter,
and replay/no-human evidence where applicable; require a deterministic conformance report; make a
missing entry or witness an acceptance failure.

**PAR-DOC-003 — Architecture `00`.** Register the standing parity-completeness obligation in the
conformance index, pointing to `10` for client/view-model boundaries and `13` for evidence closure.
Concise; no execution mechanics duplicated.

**PAR-DOC-004 — Execution `07`/`09`/`10`.** `07` owns the actor-known positive/negative and
debug-quarantine proof shape; `09` owns fixture coverage, focused golden scenarios, and fixture-ID
resolution; `10` owns real-pipeline invocation, typed-before-rendered assertions, checked-in render
references, the determinism-vs-golden clarification, CI commands, and report capture.

**PAR-DOC-005 — Execution `00`/`03` and specs `README.md`.** State that every Expansion feature spec
must carry a parity-impact declaration and passing parity evidence before acceptance; aggregate the
evidence into future phase/cert packets; run the conformance suite in ordinary CI. No new gate
identifier; no change to any passed rung's verdict. Register the obligation as a standing acceptance
dimension that must pass before any future feature spec, Phase-4 entry package, or second-proof
package is accepted.

**PAR-DOC-006 — Specs `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.** Add a parity evidence block: target
implementation commit and fixture/content fingerprints; capability entries in scope; the generated
coverage report; typed causal and actor-known witnesses; rendered golden paths/digests; anti-leak and
debug-quarantine evidence; replay/no-human disposition; compiler/source-conformance evidence; exact
commands and verdicts. The package must not reduce to screenshots or display strings.

**PAR-DOC-007 — Reference `00`/`01` (and `02` route-forward).** `00` checklist asks: "For every
capability added or changed, where is its declared surface disposition and real-pipeline
actor-filtered witness?" Update the substance of existing `R-15` to name both relapse modes —
projection blindness and renderer blindness — and the expected controls; mint no new risk ID. Add a
glossary term only after the owner ratifies the final mechanism name (§9).

## 6. Required fixtures and tests

- Reuse the existing `GoldenFixture` substrate and `TuiApp` seam; add focused per-capability fixtures
  only where the baseline census (PAR-009) shows a gap, including paired anti-leak fixtures for
  epistemic capabilities.
- Add the exhaustive-destructure source-conformance guard (PAR-002) and the closed-enum exhaustiveness
  expectations (PAR-003) alongside the existing seam-conformance tests.
- Add the capability registry, the conformance runner, and the deterministic coverage-report emitter
  (PAR-007/PAR-008) as test-side / downstream code.
- Add the controlled compile-break and guard-removal adversarial closure tests (PAR-012).
- Preserve all retained suites in §3.1 unchanged in intent.

## 7. Acceptance artifact and evidence

The implementation package must carry: full workspace `fmt --check`, `clippy --workspace
--all-targets -D warnings`, `build --locked`, and `test` results; named TUI parity/conformance test
output (no new canonical gate code invented); the controlled compile-break transcript (temporary
view-model field and presentation variant rejected until handled); source-conformance output proving
no `..`/wildcard escape at protected presentation points; the generated baseline capability matrix
with zero uncovered entries; evidence that every `ActionRegistry::definitions` entry maps to a
passing capability case; focused checked-in goldens and their reviewable diffs; paired
actor-known/hidden-state tests for epistemic cases; debug-quarantine, no-direct-dispatch, replay,
no-human, and determinism results from existing suites; content/fixture fingerprints and the exact
implementation commit. The artifact uses the `0003` template extended per PAR-DOC-006.

## 8. Implementation constraints

- Honor the one-way crate dependency direction: core depends on nothing; content depends on core;
  TUI depends on core + content. The registry/runner live test-side or downstream and never invert
  this.
- No new top-level certification rung; no reopening of passed certs; no new invariant/risk/gate code.
- No `#[non_exhaustive]` on internal core→TUI contract types.
- The parity mechanism tests ratified behavior only; it must not invent perception-on-bind or any
  other behavior to make a scenario pass (see §9).
- Placement: this hardening prerequisite should be accepted before the first next Expansion feature
  (Phase-4 entry or second-proof) is accepted — as a short shared prerequisite or the first bounded
  package of that campaign — not deferred until after such features accumulate.

## 9. Risks and open questions

These are implementation/reassessment choices the report and external research do not uniquely
determine; none blocks the spec:

1. **Capability granularity** — per-action-ID is mechanically clear; non-action features need a
   principled grouping rule favoring actor-observable contracts over module names.
2. **Registry location** — a TUI integration-test support module is least invasive; a content-side
   conformance module may better reuse fixture metadata. Either is acceptable if core stays unaware.
3. **Golden storage** — inline strings, adjacent `.golden` files, or `insta`; the decisive
   requirement is a checked-in accepted reference with review discipline, not a particular library.
4. **Intentional field omissions** — the implementation must settle the exact convention for named
   underscore bindings and rationale enforcement so omission is not a dumping ground.
5. **Baseline census** — which non-action capabilities deserve separate entries is an
   implementation-first decision (PAR-009 lists the likely minimum families).
6. **Possession bind and perception** — if a future scenario expects bind-time perception, that
   behavior is decided by the existing architecture owner, not added as a parity-test convenience.
7. **Final mechanism term** — ratification of the glossary term (PAR-DOC-007) is an owner decision.
8. **Smoke transcript** — whether a broad integrated transcript supplements the focused matrix is an
   owner choice.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| `INV-066` / `INV-095` | aligns | PAR-004/008 make the TUI/view-model acceptance gate complete per capability rather than per remembered test, at the projection→render seam. |
| `INV-067` / `INV-099`–`INV-101` | aligns | PAR-005 asserts actor-knowledge witnesses through the sealed actor-known context and forbids satisfying embodied parity from hidden/validator-only truth. |
| `INV-068` / `INV-107` | aligns | PAR-005/PAR-007 keep debug a separate, explicit disposition; a debug witness may diagnose but never satisfy the embodied obligation, preserving debug quarantine. |
| `INV-069` | aligns | PAR-003/PAR-008 keep actions data-driven pipeline entries and forbid a TUI action-kind switch that would duplicate core rules. |
| `INV-070` | aligns | Anti-leak witnesses (PAR-005) require an actor-safe why-not for blocked/unknown/stale states at the embodied surface. |
| `INV-078` / `INV-080` | aligns | PAR-007 namespaces future pack capabilities into composable profiles; core stays genre-agnostic, packs own flavor, none bypasses the event/projection/action pipeline. |
| `INV-091` / `INV-094` | aligns | Replay/no-human and possession-parity flags (PAR-007) carry into the coverage matrix where doctrine applies. |

No invariant is tensioned; this spec adds structural enforcement of existing constitutional intent
and amends no invariant.

## Outcome

On acceptance, this spec lands the standing two-hop playable-capability parity contract — compiler
enforcement of Hop 2, real-pipeline goldens for Hop 1, a test-side capability registry/conformance
runner, the per-feature obligation, and the supporting doctrine amendments — as a change-coupled
prerequisite before the next Expansion feature. It is then promoted to `archive/specs/` and receives
its `SPEC_LEDGER.md` archived-row entry per the sibling-spec convention, naming its own exact
implementation commit and claiming no certification beyond its scope.
