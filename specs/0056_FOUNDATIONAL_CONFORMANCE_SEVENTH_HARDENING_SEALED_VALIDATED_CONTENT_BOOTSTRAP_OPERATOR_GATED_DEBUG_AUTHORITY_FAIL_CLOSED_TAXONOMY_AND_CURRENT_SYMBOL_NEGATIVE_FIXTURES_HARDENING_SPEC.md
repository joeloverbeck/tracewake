# 0056 Foundational Conformance Seventh Hardening: Sealed Validated-Content Bootstrap Authority, Operator-Gated (Non-Embodied) Debug Authority, a Doctrine-Complete Fail-Closed Acceptance Taxonomy with the Settled `solo-maintainer-compensating-control` Posture and a Closed Verdict Grammar, Taxonomy Self-Mutation Coverage, and Current-Symbol Negative Fixtures Hardening Spec

**Status**: PROPOSED

> Section set follows the sibling hardening spec
> `archive/specs/0054_FOUNDATIONAL_CONFORMANCE_SIXTH_HARDENING_*` (the immediate
> predecessor on this seam), not the canonical `specs/` default. This is a staged
> hardening spec in the parallel `specs/NNNN` series, promoted to `archive/specs/`
> on acceptance; it is never promoted to the live `docs/4-specs/` tier, and it does
> not amend constitutional invariants, define gate semantics, or weaken execution
> gates. The `Outcome`, deviation log, and gate-run results are added only at
> closeout (in the acceptance artifact and a `## Outcome` section), and are
> deliberately absent from this proposal-time skeleton. Per the sibling-spec
> convention, the `docs/4-specs/SPEC_LEDGER.md` entry for this package lands at
> acceptance/closeout, not at proposal; this spec authors no ledger row now.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-seventh-pass.md`, a
  recommendation-altitude independent post-`0054` static re-audit of the `0047`
  loaded-world / time-control / TUI-authority surface as hardened by `0048`, `0050`,
  `0051`, `0052`, `0053`, and `0054`, and read against the post-`0055` solo-maintainer
  governance doctrine. The report is the originating analysis; it is not itself doctrine
  and minted no spec, invariant, risk identifier, gate code, or glossary term. Its
  predecessors are the sixth-…first-pass reports (drove `0054`, `0053`, `0052`, `0051`,
  `0050`, and `0048` respectively), used by the driver only as pre-remediation baselines
  and explicitly re-derived rather than carried forward.
- **Report target commit.** The report was conducted against
  `2720167a0d1a60ac809ae1c670539a1846df031d` (`2720167`), the repository `HEAD` at report
  authoring time (the merge of the `0054` line plus the `0055` solo-maintainer governance
  amendment) and the `HEAD` at authoring time of this spec. Every load-bearing code claim
  cited below was independently re-verified against the live working tree at this spec's
  authoring time (operator-verified; the report's line numbers are research-tool artifacts
  and are not relied upon — only the named symbols, per the project memory that externally
  researched specs fabricate line numbers):
  - **F7-01:** `crates/tracewake-core/src/state.rs` now seals
    `PhysicalState::from_validated_seed_parts(...)` and
    `AgentState::from_validated_seed_parts(...)` as `pub(crate)`, but keeps
    `#[doc(hidden)] pub fn PhysicalState::from_validated_content_parts(...)` and
    `#[doc(hidden)] pub fn AgentState::from_validated_content_parts(...)`, each delegating
    straight to the sealed seed-part constructor. `#[doc(hidden)]` hides Rustdoc output, not
    item visibility — these are `pub fn` and remain externally callable.
    `crates/tracewake-core/src/runtime/session.rs` exposes
    `pub fn ValidatedLoadedWorldBootstrap::from_validated_content(...)` (accepting
    already-materialized `PhysicalState`, `AgentState`, `EventLog`, `EpistemicProjection`)
    and `pub fn LoadedWorldRuntime::from_bootstrap(...)`.
    `crates/tracewake-content/src/load.rs` `LoadedFixture` has public authoritative-state
    fields (`canonical_world`, `canonical_agent_state`, `epistemic_projection`,
    `seed_event_log`) and a public `into_runtime_bootstrap(...)`, and
    `crates/tracewake-content/src/manifest.rs` `ContentManifest` exposes a public `new(...)`.
    An external crate can therefore fabricate authoritative aggregate state, wrap it as
    "validated content," and reach the runtime constructor without traversing the content
    validation gate. The `0054` negative fixture
    `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` attacks the now
    crate-private `from_validated_seed_parts` route, **not** the live
    `from_validated_content_parts` / `from_validated_content` / `LoadedFixture` route, so that
    negative perimeter is vacuous for this exact attack. This **re-opens the loaded-world
    bootstrap authority class** the sixth pass re-sealed by named instance only.
  - **F7-02:** the direct core path was repaired — `DebugCapability::mint()` and
    `DebugSessionAuthority::mint()` are crate-private, and
    `RuntimeCommand::bind_debug_controller(...)`, `run_no_human_day(...)`, and
    `debug_view(...)` require a `DebugSessionAuthority`. But
    `crates/tracewake-core/src/runtime/session.rs`
    `pub fn LoadedWorldRuntime::local_operator_debug_authority(&self) -> DebugSessionAuthority`
    mints a token with no externally supplied operator proof;
    `crates/tracewake-tui/src/app.rs` `TuiApp::bind_debug_actor(...)` calls it; and
    `crates/tracewake-tui/src/input.rs` parses `bind-debug <actor_id>` into
    `UiCommand::BindDebugActor` from the ordinary command-loop text surface, dispatched by
    the same parser as `wait`, `look`, and semantic actions. Debug authority is therefore
    still **inducible from ordinary embodied input**, not gated behind an explicit operator
    entrypoint. This is the residual operator/TUI path, not a resurrection of the direct-mint
    bug.
  - **F7-03:** `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`
    recomputes a verdict from a machine-readable status block rather than trusting
    `overall_result`, which is real progress. But its accepted `governance_independence`
    values are only `independent-review` and `last-push-required-reviewer`; it treats
    `pending-governance` / `status-checks-only` / `zero-approval` as non-pass and **any other
    value as an unknown error**. It does **not** know the post-`0055`
    `solo-maintainer-compensating-control` posture that `docs/2-execution/10` now ratifies. A
    truthful post-`0055` solo-maintainer artifact would be rejected as unknown, or its author
    would have to mislabel the posture to obtain a pass — a fail-closed implementation failure
    against settled doctrine.
  - **F7-04:** `crates/tracewake-core/tests/acceptance_artifact_wording.rs` is a phrase
    denylist (`FORBIDDEN_RESULT_CLAIMS` plus `CONDITIONAL_CLOSURE_CLAIMS` such as `pass with`,
    `scoped pass`, `accepted`, `green canonical perimeter`, `canonical perimeter green`) with
    ad-hoc branch-protection / historical-results / display-string / green-perimeter checks. It
    is not a closed verdict grammar; paraphrases ("approved", "validated", "ready to merge", "no
    blockers remain", "all required evidence is satisfied") bypass it.
  - **F7-05:** the taxonomy guards (`compute_result(...)`,
    `governance_is_independent(...)`, the green-mutation-evidence validation, and the wording
    guard) have synthetic unit tests, but this static pass found no independent
    mutation-sensitive proof that mutants of the parser/guard functions themselves are killed.
    A guard whose own mutants survive is decorative.
  - **F7-06:** `.github/workflows/ci.yml` has a real forcing topology — an "Ingest changed
    acceptance artifacts" step rejects report/spec closure changes without a changed acceptance
    artifact and passes each changed artifact through `TRACEWAKE_ACCEPTANCE_ARTIFACT` into the
    manifest parser. The defect is that the **forced parser is stale and shallow** (F7-03/F7-04):
    it recomputes from self-authored scalar rows, cannot represent the settled `0055` posture,
    is paraphrase-bypassable, and can accept a cited negative test that attacks a retired symbol —
    so a closure artifact can be internally self-consistent while live code defects persist.
  - **F7-07:** the `0054`-era negative fixtures attack retired or narrow symbols
    (`from_validated_seed_parts`, direct `DebugSessionAuthority::mint()` / direct
    `RuntimeCommand::bind_debug_controller(...)` without a token). They do **not** attack the live
    validated-content alias (F7-01) or the ordinary-mode TUI debug-authority path (F7-02). This is
    classic negative-fixture drift: the fixture name says the right thing, but the fixture body no
    longer attacks the strongest reachable public route.
- **Present properties re-verified (preserve, do not rebuild).** The sealed `pub(crate)`
  `from_validated_seed_parts`; the sealed one-tick wait receipt (`OneTickRuntimeReceipt` with
  private fields and limited actor-safe accessors, internal `WorldAdvanceResult` conversion);
  crate-private `DebugCapability::mint()` / `DebugSessionAuthority::mint()` and token-gated
  `bind_debug_controller` / `run_no_human_day` / `debug_view`; sealed
  `TypedActorKnownIntervalSummary`; crate-owned embodied temporal/debug fields; the public
  actor-known `food_source` replacement witness (`actor_known_food_sources_for_context(...)`
  with `food_source_fact_supersedes(...)`, exercised through `build_embodied_view_model(...)`
  and `TuiApp::from_golden(...)`); and the real CI standing-gate topology
  (`lock-layer-gates`, `public-boundary-conformance`, `full-surface-mutation-trigger`,
  `mutation in-diff (lock layer)`, scheduled `mutants-lock-layer` jobs, and the acceptance-artifact
  ingestion step). These are recorded present (§4.8) and are not re-reported as defects; the fixes
  seal the residual holes beside them.
- **`food_source` residual.** Static-present (driver §6). The replacement rule lives in the
  embodied projection surface with a public actor-known/TUI witness, not only a private helper.
  No current equivalence claim is made about any historical mutant; the `0054` acceptance's
  `2679 caught / 766 unviable / 0 missed / 0 timeout` line is historical-artifact evidence only.
  This line carries no `food_source` survivor forward and requires no new `food_source` work
  beyond preserving the existing witnesses; future reports must not treat the historical `0054`
  mutation count as current green evidence.
- **Static-survey limitation inherited from the driver.** The report forbade execution, so its
  pass/fail statements are static readings of source/test/config shape, not command results.
  This spec states code *structure* and *visibility* facts as verified and defers all green/red
  command and mutation/governance-API claims to the implementing session (§8).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This spec
  recommends and scopes work; it does not declare latest-`main` certification or any phase entry.
  When executed, the implementation must name its own exact implementation commit, not assume
  `2720167` is latest `main`.
- **Archived-history discipline.** The archived `0047`…`0055` specs, their acceptance artifacts,
  the `0048FOUCONHAR` … `0054FOUCONSIX` ticket lines, and any passed certification are immutable,
  commit-pinned historical records. This spec does not edit them and does not treat their claims
  as automatic current-state proof. In particular the `0054` acceptance's `overall_result: pass`,
  `mutation_survivors: none`, and ruleset transcript are immutable; the correct response is this
  new remediation line plus a future acceptance artifact, never retrospective alteration.

## 1. Scope

### 1.1 In scope

The seven divergences and warranted gaps the driver found at `2720167`, on the `0047`…`0054`
seams and the post-`0055` acceptance-governance doctrine. Finding IDs are preserved from the
driver (`F7-01`…`F7-07`) for lineage cross-reference. Two are re-openings of code-authority
classes the sixth pass closed by named instance (F7-01 bootstrap, F7-02 debug authority); four
are acceptance/process-integrity gaps (F7-03 stale governance vocabulary, F7-04 open wording
grammar, F7-05 taxonomy self-mutation, F7-06 stale forced parser); one is a negative-fixture
drift gap (F7-07).

1. **F7-01 — A public validated-content alias still forges a loaded-world bootstrap
   (foundational code violation).** `PhysicalState::from_validated_content_parts`,
   `AgentState::from_validated_content_parts`, `ValidatedLoadedWorldBootstrap::from_validated_content`,
   `LoadedWorldRuntime::from_bootstrap`, and the public `LoadedFixture` state fields plus
   `into_runtime_bootstrap` let an external crate fabricate authoritative aggregates and mint a
   "validated" bootstrap outside the validation gate. The `0054` fixture attacks the sealed
   seed-part names and is vacuous for this live route.
2. **F7-02 — Debug-session authority is still inducible from ordinary TUI input (boundary
   violation / hardening gap).** `LoadedWorldRuntime::local_operator_debug_authority` publicly
   mints a token with no operator proof; `TuiApp::bind_debug_actor` calls it; and `bind-debug
   <actor_id>` is parsed and dispatched from the ordinary embodied command loop. Debug entry is
   not gated behind an explicit non-embodied operator entrypoint.
3. **F7-03 — The acceptance taxonomy does not implement the settled `0055` solo-maintainer
   posture (process-integrity gap).** The manifest parser accepts only `independent-review` /
   `last-push-required-reviewer` and rejects `solo-maintainer-compensating-control` as unknown,
   even though `docs/2-execution/10` now ratifies it when the compensating-control set is proven.
4. **F7-04 — The wording guard is an open phrase denylist, not a closed verdict grammar
   (evidence-honesty gap).** Paraphrased closure claims bypass the denylist; there is no single
   computed verdict line keyed to the manifest.
5. **F7-05 — The taxonomy guards lack self-mutation coverage (mutation-coverage gap).** No
   current mutation-sensitive proof shows that mutants of the parser/guard functions are killed.
6. **F7-06 — The CI-forced parser is doctrine-incomplete (forcing-function gap).** The real
   acceptance-artifact ingestion step forces a stale, shallow parser, so a `0054`-style closure
   artifact can remain self-consistent while live code defects persist.
7. **F7-07 — The negative fixtures attack retired/narrow symbols (hardening gap).** No fixture
   attacks the live validated-content alias (F7-01) or the ordinary-mode TUI debug-authority path
   (F7-02).

### 1.2 Out of scope

- **Any constitutional-invariant amendment, gate-semantics change, new gate rung, new risk-ID,
  or new glossary term.** The driver's tier determination is **no Tier-0 foundation amendment**
  (report §9): the constitution already forbids every live defect — forgeable validated
  bootstraps, ordinary-input debug authority, and self-scored/laundered acceptance. The warranted
  change is *implementation* drift repair plus narrow below-foundation doctrine **synchronization**
  at execution `10` / architecture `13` to the already-settled `0055` posture and a closed verdict
  grammar, at ordinary-owner approval altitude.
- **Re-litigating the `0055` solo-maintainer governance mechanism.** The driver's no-weakening
  check (report §9.1) confirms the `docs/1-architecture/13` and `docs/2-execution/10` edits did
  not weaken the tier as written: solo-maintainer mode is explicitly **not** a weaker
  behavioral-evidence rule, preserves the invalid-pass conditions for open/pending/unbounded/
  historical/unproven evidence, preserves rejection of self-authored-only behavioral evidence, and
  scopes the exception to the human-approval dimension only. This spec enforces that ratified
  doctrine; it does not re-open whether solo-maintainer mode is legitimate.
- Any edit to archived specs, tickets, acceptance artifacts, reports, or passed certifications, or
  to existing `SPEC_LEDGER.md` archived rows.
- Re-building the properties the report found structurally **present and to be preserved**
  (driver §4, §6; §0 of this spec): the sealed seed-part constructors, the sealed one-tick wait
  receipt, the crate-private debug-token mint, the sealed interval summaries, the crate-owned
  embodied fields, the public `food_source` actor-known witness, and the CI ingestion topology.
- Reclassifying the `food_source_fact_supersedes` family or asserting current zero-survivor
  mutation closure without an actual executed run; treating the historical `0054` mutation count
  as current evidence.
- Any new property-testing dependency (`proptest`/`quickcheck`); the existing deterministic corpus,
  generated lock corpus, integration seams, negative fixtures, and mutation campaigns are
  sufficient (driver §7, §12).
- Phase-4 entry, second-proof entry, latest-`main` certification, or feature expansion. No new gate
  code is minted; the barrier strengthens what counts as certifying evidence under the existing
  certification ladder and acceptance machinery.

## 2. Doctrine anchors

Authority order applied without inversion: `0-foundation` → `1-architecture` → `2-execution` →
`3-reference`. Controlling invariants by finding (re-verified against the live constitution
`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`):

- **Causality / event sourcing / replay authority / validated content / authored-seed
  provenance.** INV-009 … INV-020, INV-061, INV-062, INV-063, INV-092, INV-098 — drive F7-01.
- **TUI deauthority and client boundary.** INV-008, INV-065, INV-069 — drive F7-01/F7-02.
- **Embodied/debug split and non-diegesis / quarantine.** INV-031, INV-067, INV-068, INV-093,
  INV-107 — drive F7-02.
- **No-human authority and possession parity.** INV-004, INV-005, INV-006, INV-094, INV-108 —
  drive F7-02.
- **Harsh, path-under-test acceptance and evidence honesty.** INV-093, INV-098-class evidence
  discipline plus the architecture/execution evidence-honesty contract — drive
  F7-03/F7-04/F7-05/F7-06/F7-07.

Architecture homes (driver §5, §9): `04` (action proposal / validation / scheduling / feedback —
the sealed validated-bootstrap proof object and core-owned handoff), `10` (possession / TUI view
models / debug / client boundaries — the explicit operator/debug launch boundary), `13`
(validation / observability / acceptance / review artifacts — acceptance read-model doctrine and
closed verdict grammar). Execution homes: `05` (transaction scheduler / no direct dispatch —
external-crate inability to forge the validated-content handoff), `07` (epistemic view models /
possession / debug proof — ordinary-mode debug noninducibility proof and operator-mode positive
proof), `10` (testing / observability / diagnostics / review artifacts — **the doctrine-complete
fail-closed taxonomy carrying the `solo-maintainer-compensating-control` value and the closed
verdict grammar**). Reference homes: `00` (review checklist — reviewer prompts for stale negative
fixtures, validated-content aliases, ordinary-input debug induction, and paraphrase-bypassable
wording), `01` (R-27/R-28/R-29 status/evidence only — mint no risk ID). Template home:
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (the closed verdict grammar keyed to computed
state, plus the `solo-maintainer-compensating-control` field set).

## 3. Determination

**Not conformant as a closed surface; remediation warranted; no Tier-0 foundation amendment
warranted; below-foundation doctrine synchronization warranted at execution `10` / architecture
`13` (and truthed conformance rows at architecture `04`/`10`, execution `05`/`07`) after executable
closure, at ordinary-owner altitude.**

The `0054` line delivered real and necessary gains — sealed `pub(crate)` seed-part constructors,
a sealed actor-legible one-tick wait receipt, crate-private debug-token mint with token-gated
binding/run/debug commands, a recompute-from-block status manifest, a real CI ingestion topology,
and a public `food_source` actor-known witness. Those are recorded present (§0, §4.8) and not
re-reported as defects. They do **not** establish foundational conformance: `0054` again **closed
important instances without closing the authority class**, and the acceptance machinery it
installed is not doctrine-complete against the now-settled `0055` posture.

The root cause is the same structural pattern the fourth, fifth, and sixth passes diagnosed and the
seventh pass confirms (driver §7): a correct constructor *beside* a public alias (now the
`from_validated_content_parts` / `from_validated_content` / public-`LoadedFixture` family); a
token-gated core debug command *beside* a public `local_operator_debug_authority()` reachable from
ordinary embodied input; a recompute-from-block taxonomy that performs **self-authored scalar
recomputation, not evidence recomputation**, cannot represent the settled `0055` value, and guards
verdicts with a paraphrase-bypassable denylist; and a real CI forcing step pointed at that stale
parser. This is the **seventh** consecutive pass to find divergence on this seam after a remediation
claimed closure. The cycle-breaking change remains the layered barrier: (1) compile-time
unrepresentability on the *live* public symbols; (2) behavioral witnesses through the production
boundary on the required public-boundary lane; (3) an acceptance taxonomy that is doctrine-complete,
closed-grammar, current-symbol-bound, and self-mutation-covered, forced through CI against the
actual artifact under review.

The foundation already forbids every bad state (driver §9): blessing a public validated-content
alias, ordinary-input debug authority, a stale-vocabulary taxonomy, or a paraphrase-bypassable
wording guard would be a constitutional inversion. The defects are code and process falling *below*
the rules. The one warranted doctrinal change is **synchronizing** the already-settled below-foundation
acceptance doctrine (the `0055` `solo-maintainer-compensating-control` posture and a closed verdict
grammar) into the executable taxonomy and template, and truthing the conformance rows after closure —
substance, home, and implementation only, at ordinary-owner altitude, not a constitutional edit.

## 4. Findings and remediation requirements

Each finding names a **code/governance home**, a **remediation requirement**, and the **strongest
practical anti-regression guard** (compile-time/type ownership first, then production-path behavior,
then focused mutation, then — labeled — source/topology alarms). Source-text / `include_str!` /
workflow-YAML guards are permitted only as labeled topology alarms and never as sole proof of
unrepresentability, debug quarantine, production reachability, mutation sensitivity, or merge
enforcement (driver §4, §7). Every source guard for this surface must point to the compile-time or
executable witness it protects and include a synthetic negative proving the guard itself can fire.
Subsections are keyed by finding ID, not implementation order; §9 governs sequencing.

### 4.1 F7-01 — Sealed validated-content bootstrap construction (live API)

Make "validated loaded-world bootstrap" **unrepresentable outside the authority path** against the
*live* API, closing the class the sixth pass re-sealed by named instance only. Required:

1. Remove public reachability of `PhysicalState::from_validated_content_parts(...)` and
   `AgentState::from_validated_content_parts(...)` — make them crate-private, test-support-gated,
   or eliminate them. Leave **no** backwards-compatible alias or public raw-aggregate replacement.
2. Replace the cross-crate handoff with a sealed artifact external crates cannot forge. Because
   `tracewake-content` is a separate crate, `pub(crate)` alone may be insufficient; choose one
   authority topology (§10.1): (a) move content validation/materialization into `tracewake-core`
   so the authoritative-state constructors are crate-private; (b) have `tracewake-content` return a
   core-owned sealed `ValidatedContentArtifact` minted only by a core validation entrypoint; or (c)
   have `tracewake-content` pass raw authored data/events to a core validation/materialization
   function that owns final aggregate construction. **Cargo features are not security.**
3. `ValidatedLoadedWorldBootstrap::from_validated_content(...)` must accept only the sealed product,
   not raw `PhysicalState` / `AgentState` / `EventLog` / `EpistemicProjection` parts.
4. Make `LoadedFixture` state fields private if it remains the proof vehicle; construction must be
   possible only through the validating loader. `LoadedWorldRuntime::from_bootstrap(...)` may remain
   public only if the bootstrap type is genuinely unforgeable by external crates.

Code home: `crates/tracewake-core/src/state.rs`, `crates/tracewake-core/src/runtime/session.rs`,
`crates/tracewake-content/src/{load,schema,manifest,validate}.rs`, `tests/negative-fixtures/*`.

**Anti-regression guard.** A current-symbol external negative fixture (compiled by
`negative_fixture_runner.rs`, run by `public-boundary-conformance`) that imports the **live**
symbols and attempts the full attack: call `PhysicalState::from_validated_content_parts(...)` and
`AgentState::from_validated_content_parts(...)`; fabricate `ContentManifest` and
`LoadedFixture { canonical_world, canonical_agent_state, epistemic_projection, seed_event_log, .. }`
and call `into_runtime_bootstrap(...)`; pass that bootstrap to
`LoadedWorldRuntime::from_bootstrap(...)`. The expected failure must be a privacy/unconstructability
diagnostic on the authority proof, not a stale method-name error, under default and all supported
feature combinations. Add a positive content-loader integration test proving the real loader path
still loads content and derives due-work from core-owned scheduler discovery. Add mutation/in-diff
coverage for `state.rs`, `runtime/session.rs`, the content crate validation/materialization files,
and any new sealed proof type.

### 4.2 F7-02 — Operator-gated, non-embodied debug authority

Make `DebugSessionAuthority` impossible to obtain through the ordinary embodied command surface;
entering debug mode must require an independently-held operator/debug authority, not a self-bind
through ordinary input (§10.2). Required:

1. Remove `LoadedWorldRuntime::local_operator_debug_authority()` from the public ordinary runtime
   API, or make it require an unforgeable operator-mode capability that the command parser cannot
   produce.
2. Split TUI launch modes: an ordinary embodied TUI with **no** `bind-debug` command, **no** debug
   authority mint, and **no** debug panels; and an explicit operator/debug launch carrying an
   operator capability created at launch/session setup, visibly non-diegetic, never confused with
   embodied play.
3. The ordinary parser must not turn text into debug authority: if a debug command is typed in
   ordinary mode it must fail actor-safely and must not mutate controller bindings.
4. Keep `RuntimeCommand::bind_debug_controller(...)` token-gated (preserved property).

Code home: `crates/tracewake-core/src/runtime/session.rs`,
`crates/tracewake-core/src/runtime/command.rs`, `crates/tracewake-core/src/debug_capability.rs`,
`crates/tracewake-tui/src/{app,input,run,debug_panels,launch}.rs`, `tests/negative-fixtures/*`.

**Anti-regression guard.** A public TUI/runtime behavioral test that runs a normal command-loop
script containing `bind-debug actor_tomas` followed by a `debug ...` command and proves debug is
**not** enabled and no debug view is produced (this cannot be a source-text guard — it must submit
commands through the public loop). A positive test for the explicit operator launch proving debug
data is reachable only through that launch path and is visibly non-diegetic. An external-crate
fixture attempting to call any public runtime/TUI API capable of minting `DebugSessionAuthority`
without an operator proof, pinned to a privacy/authority diagnostic. Mutation coverage over
`debug_capability.rs`, `runtime/session.rs`, `runtime/command.rs`, and the TUI app/input/run/
debug-panels/launch files.

### 4.3 F7-03 — Doctrine-complete `solo-maintainer-compensating-control` posture

Teach the manifest parser the settled `0055` posture so a truthful solo-maintainer artifact is
representable and a false one fails closed (driver §8.2). Required:

1. Extend the manifest schema with `governance_independence: solo-maintainer-compensating-control`
   as an accepted value.
2. Require machine-readable compensating-control fields for that value: required checks present,
   active enforcement, no bypass actors, `current_user_can_bypass: never`, non-fast-forward
   protection, deletion protection, and strict/up-to-date enforcement.
3. Fail closed if **any** compensating-control field is absent, unproven, stale, or prose-only.
4. Preserve the rejection of bare `zero-approval` and `status-checks-only`, and continue rejecting
   self-authored-only behavioral evidence (the `0055` no-weakening conditions stand).

Code/doc home: `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`,
`crates/tracewake-core/tests/acceptance_status_manifest.rs`,
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, and the §6 doctrine homes.

**Anti-regression guard.** Deterministic synthetic cases (no new framework): a fully-proven
solo-maintainer artifact computes pass; an artifact with any missing/unproven/stale/prose-only
compensating-control field computes non-pass; bare `zero-approval` / `status-checks-only` remain
non-pass; an `independent-review` / `last-push-required-reviewer` artifact still computes pass. The
parser's compensating-control logic is added to the §4.5 self-mutation perimeter.

### 4.4 F7-04 — Closed verdict grammar (replace the phrase denylist)

Convert `acceptance_artifact_wording.rs` from an open denylist into a closed verdict grammar keyed
to computed state (driver §8.3). Required:

1. The template already requires a single computed verdict line generated from the manifest —
   `Computed result: pass` or `Computed result: non-pass` (present in
   `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, "Allowed summary wording" block). Retain
   and verify it; do not re-add it. The enforcement gap is in the **guard code**, not the template:
   `acceptance_artifact_wording.rs` is still a denylist and does not bind prose to this computed line.
2. Free prose may not contain a verdict-bearing closure claim outside the named computed line: the
   guard parses the allowed sections and rejects any other closure claim (paraphrase included), or
   forbids verdict-bearing sentences outside named sections.
3. The existing denylist is retained only as a **secondary** alarm, not the primary grammar.

Code/doc home: `crates/tracewake-core/tests/acceptance_artifact_wording.rs`,
`crates/tracewake-core/tests/support/acceptance_status_manifest.rs`,
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.

**Anti-regression guard.** Synthetic artifacts proving the guard rejects paraphrased closure
("approved", "validated", "ready to merge", "no blockers remain", "all required evidence is
satisfied") when the computed state is non-pass or the claim sits outside the computed verdict line,
and accepts a correctly-formed `Computed result:` artifact consistent with the manifest. The wording
guard is added to the §4.5 self-mutation perimeter.

### 4.5 F7-05 — Taxonomy self-mutation coverage

Add the parser/guard functions themselves to the mutation perimeter, or provide current in-diff
mutation evidence for changes to them (driver §8.4). Required: the in-diff and standing mutation
perimeter must cover `compute_result(...)`, `governance_is_independent(...)` (and the new
`solo-maintainer-compensating-control` arm), the green-mutation-evidence validation, and the closed
wording grammar, so a mutant of the verdict logic is killed by an existing test rather than
surviving. Code/config home:
`crates/tracewake-core/tests/support/acceptance_status_manifest.rs`,
`crates/tracewake-core/tests/{acceptance_status_manifest,acceptance_artifact_wording}.rs`,
`.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`, CI mutation lanes.

**Anti-regression guard.** A mutation campaign over the parser/guard functions with current
caught/unviable/missed/timeout disposition; no artifact may call the perimeter green while a
survivor on these functions remains.

### 4.6 F7-06 — Doctrine-complete CI-forced parser

Keep the existing CI acceptance-artifact ingestion step (preserved property) and make the parser it
forces doctrine-complete (driver §8.5). Required: the "Ingest changed acceptance artifacts" step
must force a parser that (a) represents the settled `0055` `solo-maintainer-compensating-control`
posture (F7-03); (b) enforces the closed verdict grammar (F7-04); (c) computes non-pass for any
open/pending/unbounded/historical/unproven row, any live survivor/timeout/missed mutant, missing
governance proof, or self-authored-only behavioral evidence; and (d) is itself mutation-covered
(F7-05). No new gate code is minted — the durable forcing function is the existing ingestion step
plus a doctrine-complete, closed-grammar, current-symbol-bound, self-mutation-covered parser.
Code/process home: `.github/workflows/ci.yml`, `crates/tracewake-core/tests/ci_workflow_guards.rs`,
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, execution `10`.

**Anti-regression guard.** `ci_workflow_guards.rs` topology assertions that the ingestion step
exists and forces the parser for changed closure artifacts (labeled topology alarm), plus the
F7-03/F7-04/F7-05 deterministic and mutation guards that prove the *forced* parser is
doctrine-complete rather than merely wired.

### 4.7 F7-07 — Current-symbol negative fixtures

Retire reliance on the drifted fixtures and attack the live public routes (driver §7, §10). The
F7-01 fixture must attack the live validated-content alias and `LoadedFixture` fabrication; the
F7-02 fixture must attack the ordinary-mode TUI debug-authority path and any public
`DebugSessionAuthority`-minting API without operator proof. Each must fail for a
privacy/unconstructability/authority diagnostic, not a stale method-name error, and must be wired
into `negative_fixture_runner.rs` and the `public-boundary-conformance` job. Code home:
`tests/negative-fixtures/*`, `crates/tracewake-core/tests/negative_fixture_runner.rs`, the
`public-boundary-conformance` CI job.

**Anti-regression guard.** The §4.1 and §4.2 fixtures are the realization of this finding; a source
guard may verify they are wired, but only as a labeled topology alarm. The negative perimeter must
fail to compile against the live symbols, proving the attack route is closed rather than renamed.

### 4.8 Preserved properties (regression-guard, not rebuild)

Recorded present in the live tree and to be preserved while fixing the above (driver §4, §6): the
sealed `pub(crate)` `from_validated_seed_parts` constructors; the runtime-centered loaded-world path
(`TuiApp::from_golden` → `LoadedFixture::into_runtime_bootstrap` → `from_validated_content` →
`from_bootstrap`); the sealed `OneTickRuntimeReceipt` with private fields and actor-safe accessors;
the crate-private `DebugCapability`/`DebugSessionAuthority` mint and token-gated
`bind_debug_controller` / `run_no_human_day` / `debug_view`; the sealed
`TypedActorKnownIntervalSummary`; the crate-owned embodied temporal/debug fields; the public
actor-known `food_source` witness (`actor_known_food_sources_for_context` /
`food_source_fact_supersedes` through `build_embodied_view_model` and `TuiApp::from_golden`); and the
CI standing-gate topology including the acceptance-artifact ingestion step. These are not
re-commissioned as absent — the fixes seal the residual holes beside them.

## 5. Standing-mutation residual disposition

This line carries no survivor family forward by default. The `food_source` family is static-present
(§0, driver §6) with a public actor-known witness; this line requires only that the witness be
preserved and that no future report treat the historical `0054` mutation count as current green
evidence. If the implementing session discovers any new survivor on the sealed surfaces (validated-
content bootstrap, debug-authority path, taxonomy parser/guard functions), it must kill it through a
public behavior witness or record it under a **bounded forcing function** per the §4.3/§4.6 fail-
closed taxonomy — naming owning surface, why not closed by this line, the next known execution move
in `SPEC_LEDGER.md`, a maximum number of remediation epochs or a concrete trigger after which it
becomes blocking, and the exact CI/mutation test that fails if it remains. **No artifact may call the
canonical standing perimeter green while any survivor remains**, and no `mutation_status` may compute
pass with survivors present.

## 6. Doctrine synchronization and live-doc truthing

This line warrants **synchronization** of below-foundation doctrine, not new doctrine substance —
substance and home only, at ordinary-owner approval altitude, attaching to the eventual tier edit,
not to this spec's write. **No Tier-0 constitutional amendment is warranted** (driver §9): the
foundation already forbids forgeable validated bootstraps, ordinary-input debug authority,
self-scored/laundered acceptance, and paraphrase-bypassable verdicts. The `0055` no-weakening check
(driver §9.1) confirms the architecture `13` / execution `10` solo-maintainer edits are sound at
their tier; the weakness is implementation drift, not doctrine text. This spec mints no invariant,
gate code, risk ID, or glossary term, and authors no ratified wording.

### 6.1 Below-foundation doctrine synchronization (F7-03/F7-04/F7-06 home; part of this line)

Land alongside the F7-03/F7-04 code so the rules are executable, not merely written:

- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — align the
  fail-closed taxonomy implementation details with the settled `0055`
  `solo-maintainer-compensating-control` value (and its proven compensating-control field set) and
  the **closed verdict grammar**; reaffirm that source-text guards are topology alarms only and that
  the forced CI parser must be doctrine-complete.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — keep
  acceptance artifacts as read models over current evidence; a `pass` claim requires the
  doctrine-complete computed verdict, the proven governance posture (independent-review,
  last-push-required-reviewer, or fully-proven solo-maintainer-compensating-control), no live
  survivor/pending rows, and no prose stronger than the computed state.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the closed verdict-grammar line
  (`Computed result: pass | non-pass`) already exists in the "Allowed summary wording" block;
  retain/verify it rather than re-adding it. The genuine template gap is the
  `solo-maintainer-compensating-control` field set, which must be added so the status block can
  record that posture's proven compensating controls. The status block is generated/verified from
  evidence, not authored as free-form certification.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — add reviewer prompts for stale
  negative fixtures, validated-content aliases beside sealed constructors, ordinary-input debug
  induction, and paraphrase-bypassable wording (navigation only).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — update existing **R-27/R-28/R-29** status/evidence
  rows only; mint no new risk ID.

### 6.2 Post-implementation live-doc truthing (after executable closure only)

Do not edit conformance rows first. After F7-01…F7-07 are implemented and executed at one exact
commit: `docs/1-architecture/04` and `docs/2-execution/05` truth the bootstrap row to name the
sealed validated-content proof object and its live-API compile-fail negative proof (not a public
alias); `docs/1-architecture/10` and `docs/2-execution/07` truth the TUI/debug-authority rows so an
ordinary embodied launch cannot induce debug authority and debug entry requires an explicit operator
capability; `docs/1-architecture/00` and `docs/3-reference/00` update the loaded-world / TUI
authority and reviewer rows; `SPEC_LEDGER.md` routes this remediation spec and closeout through the
normal ledger/archive process. Every updated evidence row must answer: which sealed constructor
created the runtime, which operator capability crossed the debug boundary, what state/event/projection
effect was observed, and which deliberate mutation or negative compile attempt proves sensitivity. No
archived spec/ticket/report/acceptance/certification is edited.

## 7. Required fixtures, tests, and CI

Extend existing machinery (driver §7); do **not** duplicate it or add a property-testing dependency:

- **External-crate negative fixtures** (`tests/negative-fixtures/*`, `negative_fixture_runner.rs`):
  live-API validated-content bootstrap fabrication (§4.1) and ordinary-mode / no-operator-proof debug
  authority induction (§4.2/§4.7) — each under **default and all supported feature combinations**,
  pinned to a privacy/constructor/authority diagnostic; plus positive in-crate witnesses that the
  single core owner can still perform each operation through the sealed/operator path.
- **Public-boundary conformance lane** (the required `public-boundary-conformance` CI job):
  deterministic tests through `TuiApp::from_golden`, public runtime commands, the world-step
  coordinator, the TUI command loop, parity/adversarial surfaces, and the negative fixtures.
- **Ordinary-mode debug noninducibility** (TUI tests): a public command-loop script proving
  `bind-debug` + `debug` in ordinary mode enables nothing, plus a positive operator-launch test
  proving debug data is reachable only through the explicit operator entrypoint and is non-diegetic.
- **Acceptance taxonomy** (`acceptance_status_manifest.rs` support + tests,
  `acceptance_artifact_wording.rs`): the `solo-maintainer-compensating-control` posture (§4.3), the
  closed verdict grammar (§4.4), and a mutation campaign over the parser/guard functions (§4.5).
- **CI ingestion** (`ci_workflow_guards.rs`, `.github/workflows/ci.yml`): topology guard that the
  ingestion step forces the doctrine-complete parser for changed closure artifacts (§4.6).
- **Mutation**: focused campaigns during implementation for fast feedback, then the configured
  standing campaign after all code/test work; the canonical perimeter may not be called green while
  any survivor remains.
- `include_str!` / workflow-YAML import/method guards remain labeled topology alarms only.

## 8. Acceptance artifact and evidence

On implementation the session must: begin from a clean baseline; name its own exact implementation
commit (not this proposal's baseline `2720167`); run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo build --workspace --all-targets --locked`, and `cargo test --workspace`; run focused mutation
campaigns for each in-surface change and then the full standing campaign **after** all code/test
work; and publish the selected denominator with the complete caught/missed/unviable/timeout
disposition. The acceptance artifact follows `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`,
**carries the machine-readable status manifest computed by the doctrine-complete fail-closed
parser** (with the closed verdict-grammar line), records per-finding closure with real
production-path evidence (sealed validated-content constructor, operator-gated debug authority,
ordinary-mode debug noninducibility, observed effect, sensitivity/negative-compile proof), records
the required-check names **and a ruleset API transcript** matching the declared governance posture
(independent-review, last-push-required-reviewer, or a fully-proven
`solo-maintainer-compensating-control` field set), and must not render `pass` unless the manifest
computes pass — i.e. unless every required finding is `closed`, the governance posture is proven
(or honestly computes non-pass), mutation evidence is current and green, and the actual artifact
under review was ingested. Doc-truthing (§6.2) lands only after the executable witnesses exist; the
§6.1 doctrine synchronization lands with the F7-03/F7-04 code. Every executable claim is the
implementing session's to produce — this spec asserts no green/red command result.

## 9. Implementation constraints

- Follow the driver's recommended **closure order** (report §10): (1) seal the validated-content
  bootstrap class and repair the negative fixture to attack current symbols (F7-01); (2) move debug
  operator authority outside ordinary TUI input and add the ordinary-mode noninducibility proof
  (F7-02); (3) add current-symbol negative fixtures across both surfaces (F7-07); (4) update the
  taxonomy parser for the `0055` `solo-maintainer-compensating-control` posture (F7-03); (5) convert
  the wording guard to a closed verdict grammar, keeping the denylist as a secondary alarm (F7-04);
  (6) mutation-cover the parser/guard and the sealed seam (F7-05) and confirm the CI-forced parser is
  doctrine-complete (F7-06); (7) run the full standing gate from a clean baseline; (8) only then
  truth live conformance docs (§6.2). Code-authority sealing lands first because those are the live
  foundational violations; the acceptance/taxonomy repair follows so the next line cannot launder a
  pass.
- No backwards-compatibility shim or public alias path in new work; a temporary internal adapter may
  exist only to migrate core tests and must be removed before closeout.
- Core must not depend on content or tui; the sealed bootstrap authority and operator/debug authority
  live in `tracewake-core` (or a dedicated internal authority crate per §10.1), preserving the
  one-way dependency direction.
- Worktree discipline: if implemented in a worktree, all paths resolve against the worktree root.
- Decompose into one ticket per reviewable diff (the closure-order steps are the natural ticket
  boundaries); the reassess/decomposition step determines the ticket prefix continuing the sibling
  convention (`0048FOUCONHAR` → `0050FOUCONSEC` → `0051FOUCONTHI` → `0052FOUCONFOU` →
  `0053FOUCONFIF` → `0054FOUCONSIX` → a `0056`-keyed successor such as `0056FOUCONSEV`).

## 10. Risks and open questions

These are implementation choices inside settled doctrine; none blocks the determination (driver §11
open maintainer decisions):

1. **Cross-crate bootstrap authority topology** — move materialization into core, introduce a
   core-owned unforgeable proof object, or have content deliver raw authored data to a core validator.
   The requirement is unforgeability, not a specific architecture; no backwards-compatible alias;
   Cargo features are not security.
2. **Operator/debug launch UX** — local-TUI debug attach may remain convenient for development, but it
   must be outside ordinary embodied command input, carry an operator capability created at
   launch/session setup, and be visibly non-diegetic.
3. **Solo-maintainer acceptor recommendation** — whether foundational-conformance acceptance artifacts
   in solo-maintainer mode require an artifact-specific independent acceptor, which architecture `13`
   leaves recommended but process-ratifiable. The taxonomy enforces the proven compensating-control
   set either way.
4. **Governance proof source** — whether the parser reads governance proof fields from the artifact
   block or from a checked-in machine-readable ruleset transcript file. Either is acceptable if
   fail-closed and current.
5. **Closed-grammar surface** — the exact named sections and the single computed verdict-line format,
   owned by template `0003` and execution `10`; non-negotiable is that no prose may assert a verdict
   stronger than the computed state.

Open recurrence risk: this is the **seventh** consecutive pass to find divergence on this seam after a
remediation claimed closure. `0054` proved that sealed seed-part constructors, a sealed wait receipt,
a crate-private debug-token mint, and a recompute-from-block parser are not the same as authority
closure while a public validated-content alias, an ordinary-input debug-authority path, a
stale-vocabulary taxonomy, and a paraphrase-bypassable wording guard sit beside them. The layered
barrier bound to a doctrine-complete, closed-grammar, current-symbol-bound, self-mutation-covered
acceptance taxonomy is the structural bet that breaks the cycle; if the implementation leaves any
public validated-content path, ordinary-mode debug induction, stale governance vocabulary, open
wording grammar, or un-mutation-covered guard, the seam will reopen for an eighth pass.

## 11. Invariants alignment

| Invariant(s) | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-009, INV-011, INV-018, INV-061, INV-062, INV-063, INV-092, INV-098 | aligns | §4.1 makes the validated loaded-world bootstrap unrepresentable outside the authority path against the live API (crate-private/sealed-package constructor + live-API compile-fail negative fixture), so authoritative state/log/projection cannot be fabricated from raw parts beside the validated path @ runtime/content boundary. |
| INV-008, INV-065, INV-069 | aligns | §4.1/§4.2 keep the client/TUI from owning simulation authority — it consumes runtime products and typed attempts only and cannot mint authoritative bootstraps or debug authority @ TUI/client boundary. |
| INV-031, INV-067, INV-068, INV-093, INV-107 | aligns | §4.2 moves debug-authority minting out of ordinary embodied input to an explicit, visibly non-diegetic operator entrypoint, with a public ordinary-mode noninducibility proof, preserving the embodied/debug split and quarantine @ runtime command + TUI boundary. |
| INV-004, INV-005, INV-006, INV-094, INV-108 | aligns | §4.2 ensures debug capability is independently held rather than self-bound through possession/ordinary input, preserving no-human authority and possession parity @ runtime command boundary. |
| INV-093, INV-098-class acceptance discipline | aligns | §4.3–§4.6 make acceptance doctrine-complete and fail-closed — the settled `solo-maintainer-compensating-control` posture with a proven compensating-control set, a closed verdict grammar, current-symbol-bound negative evidence, self-mutation coverage of the guards, and a CI-forced doctrine-complete parser — ending pass-shaped artifacts over open defects @ CI/governance + acceptance surface. |
| (all above) | N/A — no Tier-0 amendment | §3/§6/driver §9: the foundation already forbids every bad state; the only doctrinal change is synchronizing the already-settled below-foundation acceptance doctrine (`0055` posture + closed verdict grammar) into the executable taxonomy/template at execution `10`/architecture `13`, and truthing conformance rows after closure; no invariant is weakened, minted, or redefined. |
