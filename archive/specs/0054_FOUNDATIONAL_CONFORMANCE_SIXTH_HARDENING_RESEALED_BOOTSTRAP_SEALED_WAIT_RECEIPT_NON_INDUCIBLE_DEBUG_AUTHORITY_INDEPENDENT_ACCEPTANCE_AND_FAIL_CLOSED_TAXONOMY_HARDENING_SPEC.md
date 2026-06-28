# 0054 Foundational Conformance Sixth Hardening: Re-Sealed Validated-Bootstrap Constructors, an Actor-Legible Sealed One-Tick Wait Receipt, Non-Inducible Debug-Session Authority, a Fail-Closed Acceptance State Machine, Independent-Acceptance Governance, PR-Blocking Mutation Proof, and a Publicly-Forced `food_source` Witness Hardening Spec

**Status**: COMPLETED

> Section set follows the sibling hardening spec
> `archive/specs/0053_FOUNDATIONAL_CONFORMANCE_FIFTH_HARDENING_*` (the immediate
> predecessor on this seam), not the canonical `specs/` default. This is a staged
> hardening spec in the parallel `specs/NNNN` series, promoted to `archive/specs/`
> on acceptance; it is never promoted to the live `docs/4-specs/` tier, and it does
> not amend constitutional invariants, define gate semantics, or weaken execution
> gates. The `Outcome`, deviation log, and gate-run results are added only at
> closeout (in the acceptance artifact and a `## Outcome` section), and are
> deliberately absent from this proposal-time skeleton.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-sixth-pass.md`, a
  recommendation-altitude independent post-`0053` static re-audit of the `0047`
  loaded-world / time-control / TUI-authority surface as hardened by `0048`, `0050`,
  `0051`, `0052`, and `0053`. The report is the originating analysis; it is not itself
  doctrine and minted no spec, invariant, risk identifier, gate code, or glossary term.
  Its predecessors are the fifth-, fourth-, third-, second-, and first-pass reports
  (drove `0053`, `0052`, `0051`, `0050`, and `0048` respectively), used by the driver
  only as pre-remediation baselines and explicitly re-derived rather than carried
  forward.
- **Report target commit.** The report was conducted against
  `7660051747aaa1c768cca9dc73385b16573ebe67` (`7660051`), the repository `HEAD` at
  report authoring time (the merge of the `0053` closeout, PR #66) and the `HEAD` at
  authoring time of this spec. Every load-bearing code claim cited below was
  independently re-verified against the live working tree at this spec's authoring time
  (operator-verified; the report's line numbers are research-tool artifacts and are not
  relied upon — only the named symbols, per the project memory that externally
  researched specs fabricate line numbers):
  - **F6-01:** `crates/tracewake-core/src/state.rs` exposes
    `pub fn PhysicalState::from_validated_seed_parts(...)` and
    `pub fn AgentState::from_validated_seed_parts(...)`, each accepting arbitrary
    aggregate maps and installing them directly into authoritative state.
    `crates/tracewake-core/src/runtime/session.rs` exposes
    `pub fn ValidatedLoadedWorldBootstrap::from_validated_content(...)` (taking a
    registry, `PhysicalState`, `AgentState`, `EventLog`, `EpistemicProjection`, and
    content IDs) and `pub fn LoadedWorldRuntime::from_bootstrap(ValidatedLoadedWorldBootstrap, SimTick)`.
    An external crate can compose these public constructors to mint a runtime from
    caller-authored state/log/projection. The 0053 negative fixture
    `external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` attacks the
    obsolete names `from_seed_parts` / `from_loaded_state`, **not** the live
    `from_validated_seed_parts` / `from_validated_content` route, so that negative
    perimeter is vacuous for this exact attack. The validated content path
    (`tracewake-content` → `LoadedFixture::into_runtime_bootstrap` →
    `from_validated_content`) is therefore a convention beside an equally public raw
    constructor, not a type boundary. This **re-opens the F5-01 / F4-01 authority
    class** that the fifth pass renamed but did not close.
  - **F6-02:** `crates/tracewake-core/src/runtime/receipt.rs` `RuntimeReceiptKind` is a
    public enum carrying `OneTickAdvanced(WorldAdvanceResult)`, and
    `pub fn RuntimeReceipt::kind(&self) -> &RuntimeReceiptKind` returns it to external
    callers. `WorldAdvanceResult` (from `scheduler.rs`) exposes public fields — prior
    tick, resulting tick, appended event IDs, actor-known interval delta, due-duration
    candidates, due-work summary, actor step summaries, and controlled pipeline results;
    `ActorStepSummary` exposes actor id, proposal id, decision-trace id, local-plan id,
    proposal ancestry, pipeline status, committed event IDs, diagnostic IDs, and status.
    `LoadedWorldRuntime::wait_one_tick` is public and returns
    `RuntimeReceipt::one_tick_advanced(result)`. By contrast `ContinuedRuntimeReceipt`
    hides the raw `AdvanceUntilResult` and exposes only `advanced`, an appended-event
    count, and an optional actor-known interval summary — the one-tick wait path did not
    receive the same seal. The public core API hands debug-grade scheduler internals to
    any caller performing a one-tick advance; TUI render discipline cannot repair an
    information-flow leak already present in the public product.
  - **F6-03:** `crates/tracewake-core/src/debug_capability.rs` keeps
    `DebugCapability`/`DebugSessionAuthority` fields private and `mint()` `pub(crate)`
    (good Rust privacy). But `crates/tracewake-core/src/runtime/command.rs`
    `pub fn RuntimeCommand::bind_debug_controller(controller_id, actor_id)` requires no
    prior authority; after `LoadedWorldRuntime::submit_command` binds the controller in
    `ControllerMode::Debug`, `pub fn LoadedWorldRuntime::debug_session_authority_for(...)`
    returns a runtime-minted `DebugSessionAuthority`. `TuiApp::bind_debug_actor` /
    `TuiApp::debug_authority` delegate to the same path. The 0053 negative fixture
    `external_crate_cannot_submit_debug_command_without_token` proves only that a caller
    cannot call `DebugSessionAuthority::mint()` directly; it does **not** prove that a
    caller cannot first submit a public debug bind and then obtain the runtime-minted
    authority. The token is unforgeable as a value but **inducible** through the
    unprivileged public command surface it is meant to guard. This re-opens the
    F5-03 / F4-02 / F4-07 authority class.
  - **F6-04:** `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`
    extracts a fenced `tracewake-acceptance-status` block and recomputes
    `overall_result` from statuses, but: `REQUIRED_FINDINGS` is hardcoded to
    `F5-01`…`F5-06` (not generic for the next line); a `closed` finding requires only
    presence of `evidence=`/`negative=` fields, not validation that the evidence names
    live code/current method names/public-boundary behavior/mutation sensitivity;
    `branch_protection` is a scalar string with no parsed ruleset transcript; and the
    `mutation_status: non-blocking-bounded-forcing` arm can compute `Pass` even with
    explicit survivor rows present (survivor-pass hole). `acceptance_artifact_wording.rs`
    is a phrase denylist (paraphrase-evadable), not a closed grammar. The block is
    self-authored and not independently derived from CI/ruleset/mutation artifacts, and
    nothing forces the **actual** acceptance artifact under review to be ingested and
    made merge-blocking.
  - **F6-05:** the 0053 closeout ruleset transcript (`main-standing-conformance-barrier`)
    shows required status checks, no bypass actors, and strict up-to-date policy — real
    status-check enforcement — but also `required_approving_review_count: 0`,
    `require_last_push_approval: false`, and `required_reviewers: []`. The
    `governance-required-checks-audit` job in `.github/workflows/ci.yml` (pinned by
    `crates/tracewake-core/tests/ci_workflow_guards.rs`) checks required contexts and
    bypass actors, but does **not** fail on zero required approvals, missing last-push
    approval, missing required reviewers, or implementer-as-acceptor self-merge. F5-04 is
    re-opened as an independent-acceptance gap, not closed.
  - **F6-06:** `.github/workflows/ci.yml` defines a `full-surface mutation trigger (lock
    layer)` topology/signaling job and a `mutation in-diff (lock layer)` (`mutants-in-diff`)
    job. The 0053 transcript's seven required contexts include the trigger and shard
    reconciliation but not `mutation in-diff`. A topology trigger going green is an alarm,
    not proof a full mutation perimeter is green; in-diff mutation is closer to an
    enforceable PR check but is not one of the required contexts for guarded changes.
  - **F6-07:** `crates/tracewake-core/src/projections.rs` `food_source_fact_supersedes`
    and `crates/tracewake-core/tests/food_source_projection.rs` now exercise
    source-bearing / source-less replacement and deterministic source-key ordering through
    `build_embodied_view_model`; `.cargo/mutants.toml` includes the relevant projection
    files in the standing perimeter. The named direct semantics are materially repaired,
    but closure is via a private/projection helper; no public actor-known/TUI witness with
    event/provenance ancestry forces the routed-forward pattern, and no current zero-survivor
    mutation run may be certified by static survey.
- **Static-survey limitation inherited from the driver.** The report forbade execution,
  so its pass/fail statements are static readings of source/test/config shape, not command
  results. This spec states code *structure* and *visibility* facts as verified and defers
  all green/red command and mutation/governance-API claims to the implementing session
  (§8).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This
  spec recommends and scopes work; it does not declare latest-`main` certification or any
  phase entry. When executed, the implementation must name its own exact implementation
  commit, not assume `7660051` is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` entry at acceptance/closeout, not at proposal. This spec
  authors no ledger row now and makes no change to live `0001`/`0003` or the ledger.
- **Archived-history discipline.** The archived `0047`…`0053` specs, their acceptance
  artifacts, the `0048FOUCONHAR` / `0050FOUCONSEC` / `0051FOUCONTHI` / `0052FOUCONFOU` /
  `0053FOUCONFIF` ticket lines, and any passed certification are immutable, commit-pinned
  historical records. This spec does not edit them and does not treat their claims as
  automatic current-state proof. In particular, the 0053 acceptance's `overall_result:
  pass`, `mutation_survivors: none`, and ruleset transcript (including its
  `required_approving_review_count: 0`) are immutable; the correct response is this new
  remediation line plus a future acceptance artifact, never retrospective alteration.

## 1. Scope

### 1.1 In scope

The seven divergences and warranted gaps the driver found at `7660051`, on the
`0047`…`0053` seams and their named conformance collaborators. Finding IDs are preserved
from the driver (`F6-01`…`F6-07`) for lineage cross-reference. Three are re-openings of
authority classes the fifth pass claimed closed by instance (F6-01↔F5-01, F6-02↔F5-02,
F6-03↔F5-03); two are process/governance re-openings (F6-04↔F5-06 taxonomy keystone,
F6-05↔F5-04 governance layer); one is a standing-gate enforcement gap (F6-06); one is the
`food_source` residual carried forward with a public-forcing requirement (F6-07).

1. **F6-01 — Public raw "validated" seed-part / bootstrap constructors still forge a
   loaded-world bootstrap (foundational code violation).** `PhysicalState::from_validated_seed_parts`,
   `AgentState::from_validated_seed_parts`, and `ValidatedLoadedWorldBootstrap::from_validated_content`
   are public; `LoadedWorldRuntime::from_bootstrap` consumes their product. An external
   crate can fabricate state/log/projection aggregates and pass them in as "validated."
   The 0053 negative fixture targets stale method names and is vacuous for the live API.
2. **F6-02 — The public one-tick wait receipt exposes debug-grade world-step internals
   (foundational code violation / information-flow leak).** `RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)`
   is public through `RuntimeReceipt::kind()`; `WorldAdvanceResult` / `ActorStepSummary`
   expose exact ticks, event IDs, due-work summaries, actor-step summaries, proposal
   ancestry, decision-trace IDs, local-plan IDs, and pipeline status. The `continue` path
   is sealed; the one-tick wait path is not.
3. **F6-03 — Debug-session authority is non-forgeable but publicly inducible through an
   unguarded debug bind (boundary violation / authority gap).** `bind_debug_controller`
   is public and requires no prior authority; after submitting it a caller obtains a
   runtime-minted `DebugSessionAuthority` via `debug_session_authority_for`. Direct-mint
   compile-fail is proven; token induction through the public command surface is not
   blocked.
4. **F6-04 — The acceptance taxonomy is self-consistency checking, not fail-closed
   verification (process-integrity gap; keystone).** The parser checks an author-written
   block for internal consistency over hardcoded F5 labels and field presence; the wording
   guard is a phrase denylist; `non-blocking-bounded-forcing` with survivor rows can still
   compute `Pass`; governance is a scalar; and the exact artifact under review is not
   forced through the mechanism as a merge-blocking current artifact.
5. **F6-05 — Governance enforces status checks but not independent acceptance (governance
   gap).** The ruleset requires checks but allows `required_approving_review_count: 0`, no
   last-push approval, and no required reviewers, permitting implementer-as-acceptor
   self-merge. The audit job does not treat this as a failure or compute
   `pending-governance`.
6. **F6-06 — Mutation proof is partly declarative rather than PR-enforced for guarded
   changes (enforced standing-gate gap).** The required required-check set can be satisfied
   by a topology trigger alarm rather than actual in-diff mutation proof or a fail-closed
   current-mutation-artifact check.
7. **F6-07 — The `food_source_fact_supersedes` family is materially addressed but lacks a
   public actor-known forcing witness (present with hardening gap).** Direct/projection
   tests encode the semantics, but no public actor-known/TUI fixture with event/provenance
   ancestry forces the routed-forward pattern, and current zero-survivor mutation closure is
   not certified.

### 1.2 Out of scope

- **Any constitutional-invariant amendment, gate-semantics change, new gate rung, new
  risk-ID, or new glossary term.** The driver's tier determination is **no Tier-0
  foundation amendment** — the constitution already forbids every live defect (§3, §9 of
  the report). The warranted change is doctrine *strengthening below foundation* (§6),
  at ordinary-owner approval altitude.
- Any edit to archived specs, tickets, acceptance artifacts, reports, or passed
  certifications, or to existing `SPEC_LEDGER.md` archived rows.
- Re-litigating the properties the report found structurally **present and to be
  preserved, not rebuilt** (driver §4): the runtime-centered loaded-world path
  (`TuiApp::from_golden` → `LoadedFixture::into_runtime_bootstrap` → `from_bootstrap`);
  the closed `RuntimeCommand`/`pub(crate) RuntimeCommandKind`; the sealed
  `ContinuedRuntimeReceipt`; the crate-private `DebugRuntimeReceipt::new` carrying a
  capability; the crate-private `DebugCapability`/`DebugSessionAuthority` struct seal and
  its direct-mint compile-fail fixtures; the crate-private embodied view temporal/debug
  fields with read-only accessors; the `ActorDecisionTransaction::run` actor-known flow;
  the scheduler-owned due-work derivation in `transact_world_one_tick`; the strengthened
  `food_source` replacement tests; and the existing acceptance-taxonomy parser scaffold
  (§4.8).
- Reclassifying the `food_source_fact_supersedes` family into a lower tier or asserting
  equivalence without semantic proof.
- Any new property-testing dependency (`proptest`/`quickcheck`); the existing deterministic
  corpus, generated lock corpus, integration seams, negative fixtures, and mutation
  campaigns are sufficient (driver §7, §10).
- Phase-4 entry, second-proof entry, latest-`main` certification, or feature expansion. No
  new gate code is minted; the barrier strengthens what counts as certifying evidence under
  the existing certification ladder and acceptance machinery (driver §7).

## 2. Doctrine anchors

Authority order applied without inversion: `0-foundation` → `1-architecture` →
`2-execution` → `3-reference`. Controlling invariants by finding (re-verified against the
live constitution `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`):

- **Causality / event sourcing / replay authority / validated content.** INV-001, INV-009,
  INV-011, INV-018, INV-022, INV-092 — drive F6-01.
- **Truth firewall / belief-before-truth / records separation / no-telepathy.** INV-002,
  INV-023, INV-024, INV-030, INV-099, INV-101 — drive F6-01/F6-02/F6-07.
- **TUI/embodied/debug boundary and non-diegesis.** INV-008, INV-031, INV-067, INV-068,
  INV-069, INV-107 — drive F6-02/F6-03.
- **Temporal authority (time validates, holder-known time plans).** INV-112 — drives F6-02
  (exact ticks/frontiers/due queues are not actor-legible on a normal wait receipt).
- **No-human authority and possession parity.** INV-004, INV-005, INV-006, INV-094, INV-108
  — drive F6-03.
- **Harsh, path-under-test acceptance and validation.** INV-093, INV-098-class evidence
  discipline plus the architecture/execution evidence-honesty contract — drive
  F6-04/F6-05/F6-06.

Architecture homes (driver §5, §9): `04` (action proposal / validation / scheduling /
feedback — bootstrap unforgeability evidence row), `10` (possession / TUI view models /
debug / client boundaries — sealed one-tick wait receipt and non-inducible debug authority),
`13` (validation / observability / acceptance / review artifacts — the acceptance state
machine and read-model-not-evidence doctrine). Execution homes: `05` (transaction scheduler
/ no direct dispatch — bootstrap unforgeability + debug authority), `07` (epistemic view
models / possession / debug proof — sealed wait receipt proof), `10` (testing / observability
/ diagnostics / review artifacts — the **fail-closed acceptance state machine, governance
independence computation, PR-blocking mutation proof, and the `food_source` public-forcing
requirement**). Reference homes: `00` (review checklist — reviewer prompts for stale
negative fixtures, public-constructor composition, debug-token induction, and survivor-pass
taxonomy holes), `01` (R-27/R-28/R-29 status/evidence only — mint no risk ID). Spec/template
homes: `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` (closed verdict grammar keyed to
computed state) and `SPEC_LEDGER.md` next-known move.

## 3. Determination

**Not conformant as a closed surface; remediation warranted; no Tier-0 foundation amendment
warranted; doctrine strengthening warranted below foundation at architecture `13`, `10`,
`04`, execution `10`, `07`, `05`, reference `00`/`01`, and template `0003`.**

The 0053 line delivered real and necessary gains — a runtime-centered loaded-world path, a
closed command enum, a sealed `continue` receipt, a capability-carrying debug receipt, a
crate-private debug-token struct with direct-mint compile-fail fixtures, crate-private
embodied temporal/debug fields, strengthened `food_source` replacement tests, and a
status-manifest parser. Those are recorded as present (§4.8) and not re-reported as defects.
They do **not** establish foundational conformance: 0053 again **closed important instances
without closing the authority class**, and the new process machinery does not make a
laundered "pass" impossible.

The root cause is the same structural pattern the fourth and fifth passes diagnosed and the
sixth pass confirms (driver §7): a correct constructor *beside* a public raw one (now the
`from_validated_*` family); a sealed `continue` receipt *beside* an unsealed one-tick wait
receipt; a non-forgeable debug token *beside* a public bind that induces it; a self-authored,
field-presence acceptance block that recomputes consistency but not truth; status-check
governance mistaken for independent acceptance; and a topology mutation alarm mistaken for
mutation proof. This is the **sixth** consecutive pass to find divergence on this seam after
a remediation claimed closure. The cycle-breaking change remains the §3-of-0053 three-layer
barrier — (1) compile-time unrepresentability on the real public symbols; (2) behavioral
witnesses through the production boundary on a required public-boundary lane; (3) governance
enforcement that the implementer cannot self-satisfy — now bound to a **fail-closed
acceptance state machine** that ingests the actual artifact under review and computes
non-pass for any open/pending/routed-forward row, any live survivor, missing independent
acceptance, or stale/non-current evidence.

The foundation already forbids every bad state (driver §9): blessing public raw bootstrap
constructors, debug-grade wait receipts, publicly-inducible debug authority, self-scored
pass artifacts, or implementer-as-acceptor governance would be a constitutional inversion.
The defects are code and process falling *below* the rules. The one warranted doctrinal
change is to operationalize the existing evidence-honesty rules into an executable state
machine and a closed verdict grammar at architecture `13` / execution `10` (and the truthed
rows at architecture `04`/`10`, execution `05`/`07` after closure) — substance and home
only, at ordinary-owner altitude, not a constitutional edit.

## 4. Findings and remediation requirements

Each finding names a **code/governance home**, a **remediation requirement**, and the
**strongest practical anti-regression guard** (compile-time/type ownership first, then
production-path behavior, then focused mutation, then — labeled — source/topology alarms).
Source-text / `include_str!` / workflow-YAML guards are permitted only as labeled topology
alarms and never as sole proof of unrepresentability, replay continuation, information-flow
noninterference, production reachability, mutation sensitivity, or merge enforcement (driver
§4, §7). Every source guard for this surface must point to the compile-time or executable
witness it protects and include a synthetic negative proving the guard itself can fire.
Subsections are keyed by finding ID, not implementation order; §9 governs sequencing.

### 4.1 F6-01 — Re-sealed validated-bootstrap construction

Make "validated loaded-world bootstrap" **unrepresentable outside the authority path**
against the *live* API, closing the class the fifth pass renamed. Because `tracewake-content`
is a separate crate, `pub(crate)` alone may be insufficient; choose one authority topology
(§10.1): (a) make `PhysicalState::from_validated_seed_parts` /
`AgentState::from_validated_seed_parts` crate-private/test-support-only and move validated
content assembly behind a production API that never accepts caller-built aggregate maps; (b)
introduce an unforgeable validation/assembly witness token obtainable only by the content
validation path, required for any cross-crate state assembly that must remain public; or (c)
move raw aggregate assembly fully into a content-owned package type whose fields are private
and whose constructor is bound to schema/provenance validation, with core consuming only that
sealed package. **Cargo features are not security.** `from_validated_content` must accept
only a sealed product, not raw state/log/projection parts; leave **no** backwards-compatible
alias. Code home: `crates/tracewake-core/src/state.rs`,
`crates/tracewake-core/src/runtime/session.rs`, `crates/tracewake-content/src/load.rs`,
`tests/negative-fixtures/*`.

**Anti-regression guard.** Replace/extend the stale
`external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts` fixture (compiled by
`negative_fixture_runner.rs`) so it imports the **live** symbols
(`PhysicalState::from_validated_seed_parts`, `AgentState::from_validated_seed_parts`,
`EventLog::new`, `EpistemicProjection::new`, `ValidatedLoadedWorldBootstrap::from_validated_content`)
and attempts to reach `LoadedWorldRuntime::from_bootstrap` — and must **fail to compile** for
a privacy/constructor diagnostic (not a stale "cannot find function" or missing import),
under default and all supported feature combinations. Add a positive content-loader
integration test proving `LoadedFixture::into_runtime_bootstrap` still reaches the runtime
through the sealed validated path with no caller-injected scheduler actor/process
registration.

### 4.2 F6-02 — Sealed actor-legible one-tick wait receipt

Seal the one-tick wait receipt so external callers receive only actor-legible progress and
cannot read debug-grade scheduler internals. Required: (1) replace
`RuntimeReceiptKind::OneTickAdvanced(WorldAdvanceResult)` with a sealed actor-legible
receipt (a new `OneTickRuntimeReceipt`, or a generalized `WorldAdvanceRuntimeReceipt`
reused by both wait and continue) carrying only actor-visible progress, an
actor-legible appended-event count if appropriate, and a core-built actor-known interval
summary — mirroring the existing `ContinuedRuntimeReceipt` seal; (2) move the full
`WorldAdvanceResult` (and `ActorStepSummary` and the exact tick/event/due-work/ancestry/
trace/pipeline fields) to crate-private scheduler/debug code, reachable only through a
`DebugSessionAuthority`-gated debug receipt (ties to §4.3); (3) ensure `RuntimeReceipt::kind()`
can no longer return a variant that exposes `WorldAdvanceResult` to external callers. Code
home: `crates/tracewake-core/src/runtime/receipt.rs`,
`crates/tracewake-core/src/runtime/session.rs`, `crates/tracewake-core/src/scheduler.rs`,
and the TUI wait path (`crates/tracewake-tui/src/app.rs`, `crates/tracewake-tui/src/run.rs`,
`crates/tracewake-tui/src/render.rs`).

**Anti-regression guard.** An external negative fixture proving an external crate cannot
match a wait receipt and read `prior_tick`, `resulting_tick`, `appended_event_ids`,
`due_work_summary`, `actor_step_summaries`, `decision_trace_id`, or `pipeline_status` from a
normal embodied wait (pinned to a privacy diagnostic). A behavioral TUI/runtime test proving
the normal wait path receives only an actor-legible summary and renders no exact internals,
plus a debug/operator-mode test proving the full step report is still reachable through the
token-gated debug API. Render/transcript assertions remain labeled witnesses, not sole proof
of sealing.

### 4.3 F6-03 — Non-inducible debug-session authority

Make `DebugSessionAuthority` impossible to obtain through the ordinary public command
surface; entering debug mode must require an independently-held operator/debug authority,
not a self-bind (§10.2). Options (driver §5 F6-03): (a) remove public
`bind_debug_controller` and expose debug binding only behind a separate operator/debug
bootstrap token not available to ordinary embodied clients; (b) require an already-held
operator authority to switch a controller into debug mode, after which the
`DebugSessionAuthority` may be scoped to that controller/actor; (c) if local-TUI debug attach
is a deliberate operator action, make it an explicit operator entrypoint outside the embodied
command surface that cannot be mistaken for actor authority or ordinary play. After the fix,
`debug_session_authority_for` must not mint a token for a controller that entered debug mode
through an unprivileged public command. Code home:
`crates/tracewake-core/src/runtime/command.rs`,
`crates/tracewake-core/src/runtime/session.rs`,
`crates/tracewake-core/src/debug_capability.rs`, `crates/tracewake-tui/src/app.rs`,
`crates/tracewake-tui/src/run.rs`, `crates/tracewake-tui/src/input.rs`,
`tests/negative-fixtures/*`.

**Anti-regression guard.** A **bypass-shaped** external negative fixture that performs the
actual attack: build a runtime from a golden fixture, call
`RuntimeCommand::bind_debug_controller`, submit it, call `debug_session_authority_for`, then
attempt `run_no_human_day(authority)` or `debug_view(authority)` — and it must fail for lack
of operator authority (not because the runtime setup is stale). A positive behavioral test
for the approved operator/debug entrypoint proving debug data is reachable only through the
real authority path. A fixture that only asserts the TUI checks debug mode is vacuous — the
protected claim is the public runtime boundary.

### 4.4 F6-04 — Fail-closed acceptance state machine (process-integrity keystone)

Convert the acceptance-taxonomy parser from a self-authored consistency check into a
positive, fail-closed state machine that computes from current evidence (extend existing
machinery; no new test framework, gate code, invariant, risk ID, or glossary term). Required
(driver §4 F6-04, §8):
1. **Expected findings come from an explicit review/artifact manifest**, not hardcoded F5/F6
   labels — the parser must accept the current line's finding set (`F6-01`…`F6-07`) without a
   source edit per line.
2. **`Pass` is impossible** if any finding is open, pending, routed-forward, historical-only
   for a current requirement, not-in-scope for an in-scope requirement, or if evidence cannot
   be tied to current exact-commit files/tests.
3. **Any** mutation survivor, missed mutant, timeout, untriaged baseline miss, or
   bounded-forcing survivor computes `NonPass` for a green-closure artifact. A bounded-forcing
   survivor may be honestly recorded but never as pass — close the survivor-pass hole in the
   `non-blocking-bounded-forcing` arm.
4. **Governance status is computed from a parsed ruleset transcript / machine-readable
   artifact**, including required checks, bypass actors, `required_approving_review_count`,
   `require_last_push_approval`, `required_reviewers`, and current-user bypass (ties to §4.5).
5. **Wording is validated against a closed grammar keyed to computed state** (replace the
   phrase denylist): free prose may explain but cannot introduce a contradictory or stronger
   verdict than the computed state.
6. **CI must ingest the actual acceptance artifact proposed for merge/archive**, parse it
   with the status parser, the closed wording grammar, the governance-transcript parser, the
   mutation-evidence parser, and the expected-finding manifest, and fail if no current
   acceptance artifact is present for a closure PR, if it is not at the expected path, if any
   status is not pass-eligible, if governance independence is pending, or if mutation evidence
   is stale/non-green.

Code/doc home: `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`,
`crates/tracewake-core/tests/acceptance_status_manifest.rs`,
`crates/tracewake-core/tests/acceptance_artifact_wording.rs`,
`crates/tracewake-core/tests/ci_workflow_guards.rs`, `.github/workflows/ci.yml`,
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, and the §6 doctrine homes.

**Anti-regression guard.** Extend existing deterministic tests (no new framework) with
synthetic adversarial artifacts for: survivor-pass under bounded-forcing; paraphrased closure
over open rows; stale method-name negative evidence; branch/ruleset scalar without transcript;
zero-approval governance; missing actual-artifact ingestion; self-authored evidence strings;
historical-current conflation; and display-only evidence. Add a mutation campaign over the
parser/guard functions themselves — a guard whose own mutants survive is decorative.

### 4.5 F6-05 — Independent-acceptance governance

Make the governance audit fail unless foundational-conformance closure PRs satisfy
independent acceptance, or honestly compute `pending-governance`/`NonPass` (driver §4 F6-05,
§6). The `governance-required-checks-audit` job must parse
`pull_request.parameters.required_approving_review_count`, `require_last_push_approval`,
`required_reviewers`, and bypass/current-user fields, and fail unless the ruleset either: (a)
requires at least one approving review by a non-author/non-latest-pusher; or (b) requires
last-push approval plus a required reviewer/team rule; or (c) the artifact explicitly records
governance independence as unavailable and the manifest computes `NonPass`/`pending-governance`
rather than pass. A sole-maintainer repository may honestly say "mechanically checked,
governance independence pending" — it may not claim a sound fail-closed acceptance. Code/process
home: `.github/workflows/ci.yml`, `crates/tracewake-core/tests/ci_workflow_guards.rs`,
`crates/tracewake-core/tests/support/acceptance_status_manifest.rs`, and docs `architecture/13`,
`execution/10`, template `0003`.

**Anti-regression guard.** `ci_workflow_guards.rs` assertions that the audit fails on synthetic
zero-approval ruleset JSON; status-manifest negative cases where `branch_protection: enforced`
plus a zero-approval transcript computes non-pass for an independent-acceptance-required
artifact. A ruleset transcript with active status checks must not be accepted as evidence of
independent acceptance — the artifact must distinguish "merge-required CI" from "independent
acceptor constraint."

### 4.6 F6-06 — PR-blocking mutation proof for guarded changes

Make actual mutation proof PR-blocking for guarded code (driver §4 F6-06, §7). The required
PR contexts for guarded paths must include `mutation in-diff (lock layer)` (or an equivalent
actual mutation-result gate), or a fail-closed check that refuses guarded changes without a
current accepted mutation artifact. The scheduled full-campaign reconciliation remains
valuable as standing-perimeter reconciliation but a skipped/scheduled/informational trigger
must not count as PR pass for a code-changing closure artifact. Code/process home:
`.github/workflows/ci.yml`, `crates/tracewake-core/tests/ci_workflow_guards.rs`,
`.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`,
`crates/tracewake-core/tests/mutation_completion_merge.rs`,
`crates/tracewake-core/tests/support/acceptance_status_manifest.rs`, and execution `10`.

**Anti-regression guard.** A CI topology guard verifying `mutation in-diff (lock layer)` (or
an equivalent actual mutation-result gate) is in the required ruleset contexts for guarded
code. Manifest-parser logic that rejects `mutation_status: killed` unless the artifact cites
current in-diff or full-campaign evidence with denominator and caught/unviable/missed/timeout
counts plus baseline-miss reconciliation. The trigger remains a labeled alarm, never proof.

### 4.7 F6-07 — Publicly-forced `food_source` actor-known witness

Keep the existing projection tests and add a public actor-known/TUI fixture where
event/provenance ancestry creates two competing food-source facts and proves the
source-bearing later/stronger fact survives into the actor-visible view **without reading raw
truth** (driver §4 F6-07, §6). Code home: `crates/tracewake-core/src/projections.rs`,
`crates/tracewake-core/src/epistemics/projection.rs`,
`crates/tracewake-core/tests/food_source_projection.rs`, content-fixture tests, and TUI
seam/conformance tests. Mutation/config home: `.cargo/mutants.toml`,
`.cargo/mutants-baseline-misses.txt`. Run the existing mutation machinery from a clean
baseline; do not claim current zero survivors until that run is actually executed; accept no
"equivalent mutant" claim without a semantic proof tied to the actor-known output.

**Anti-regression guard.** Public projection/actor-known tests that introduce two food-source
facts with controlled source presence and source keys through modeled observation/record
paths, then assert the resulting actor-known belief and rendered/decision consequence through
a public boundary, killing the constant true/false and ordering mutants.

### 4.8 Preserved properties (regression-guard, not rebuild)

Recorded present in the live tree and to be preserved while fixing the above (driver §4): the
runtime-centered loaded-world path and core-owned `LoadedWorldRuntime`; the validated content
path `LoadedFixture::into_runtime_bootstrap` → `from_validated_content` → `from_bootstrap`;
the closed `RuntimeCommand` with `pub(crate) RuntimeCommandKind`; the sealed
`ContinuedRuntimeReceipt`; the crate-private `DebugRuntimeReceipt::new` carrying a
`DebugCapability`; the crate-private `DebugCapability`/`DebugSessionAuthority` struct seal and
its direct-mint compile-fail fixtures; the crate-private embodied view temporal/debug fields
with read-only accessors; the `ActorDecisionTransaction::run` actor-known transaction flow;
the scheduler-owned due-work derivation; the strengthened `food_source` replacement tests; and
the acceptance-taxonomy parser scaffold. These are not re-commissioned as absent — the fixes
seal the residual holes beside them.

## 5. Standing-mutation residual disposition

This line does not carry a survivor family forward by default. The `food_source` family was
materially addressed by 0053 (§4.7/§4.8); F6-07 requires a public actor-known forcing witness
and a fresh executed mutation run rather than a routed-forward survivor record. If the
implementing session discovers any new survivor on the sealed surfaces (bootstrap constructor,
wait-receipt accessors, debug-authority path, parser/guard functions), it must kill it through
a public behavior witness or record it under a **bounded forcing function** per the §4.4 state
machine — naming owning surface, why not closed by this line, the next known execution move in
`SPEC_LEDGER.md`, a maximum number of remediation epochs or a concrete trigger after which it
becomes blocking, and the exact CI/mutation test that fails if it remains. **No artifact may
call the canonical standing perimeter green while any survivor remains**, and no
`mutation_status` may compute pass with survivors present (§4.4.3).

## 6. Doctrine strengthening and live-doc truthing

Unlike a no-amendment line, this line **does** warrant doctrine strengthening below
foundation — substance and home only, at ordinary-owner approval altitude, attaching to the
eventual tier edit, not to this spec's write. **No Tier-0 constitutional amendment is
warranted** (driver §9): the constitution already forbids forgeable bootstraps, debug-grade
embodied receipts, publicly-inducible debug authority, self-scored pass, and
implementer-as-acceptor governance. This spec mints no invariant, gate code, risk ID, or
glossary term, and authors no ratified wording.

### 6.1 Below-foundation doctrine strengthening (F6-04/F6-05/F6-06 home; part of this line)

Land as *new doctrine substance* alongside the F6-04/F6-05/F6-06 code so the rules are
executable, not merely written (driver §9):

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **acceptance artifacts are read models over current evidence, not evidence themselves.** A
  `pass` claim requires current exact-commit evidence ingestion, independent acceptance where
  required, no live survivor/pending rows, and no prose stronger than the computed state.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — the
  **fail-closed taxonomy** computes non-pass for survivors, timeouts, pending governance,
  self-authored-only evidence, missing actual-artifact ingestion, and zero-approval
  independence gaps; source-text guards are topology alarms only; PR-blocking mutation proof
  is required for guarded changes.
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — replace open prose verdict shapes with
  a **closed grammar keyed to computed status**; the status block must be generated/verified
  from evidence artifacts and the expected-finding manifest, not authored as free-form
  certification.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — add reviewer prompts for
  stale negative fixtures, public-constructor composition, debug-token induction, and
  survivor-pass taxonomy holes (navigation only).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — update existing **R-27/R-28/R-29**
  status/evidence rows only; mint no new risk ID.

### 6.2 Post-implementation live-doc truthing (after executable closure only)

Do not edit conformance rows first. After F6-01…F6-07 are implemented and executed at one
exact commit: `docs/1-architecture/04` and `docs/2-execution/05` truth the bootstrap row to
name the re-sealed validated constructor and its live-API negative proof (not a public raw
constructor); `docs/1-architecture/10` and `docs/2-execution/07` truth the runtime-receipt /
debug-authority rows so a normal one-tick wait cannot expose debug-grade scheduler internals
and debug-mode entry requires a real operator authority (not public self-bind);
`SPEC_LEDGER.md` routes this remediation spec and closeout through the normal ledger/archive
process. Every updated evidence row must answer: which sealed constructor created the runtime,
which public command/token crossed the client boundary, what state/event/projection effect was
observed, and which deliberate mutation or negative compile attempt proves sensitivity. No
archived spec/ticket/report/acceptance/certification is edited.

## 7. Required fixtures, tests, and CI

Extend existing machinery (driver §7); do **not** duplicate it or add a property-testing
dependency:

- **External-crate negative fixtures** (`tests/negative-fixtures/*`,
  `negative_fixture_runner.rs`): live-API bootstrap fabrication (§4.1), one-tick wait-receipt
  internal extraction (§4.2), and debug-token induction via the public bind (§4.3) — each
  under **default and all supported feature combinations**, pinned to a privacy/constructor
  diagnostic; plus positive in-crate witnesses that the single core owner can still perform
  each operation through the sealed path.
- **Public-boundary conformance lane** (the required `public-boundary conformance` CI job):
  deterministic tests through `TuiApp::from_golden`, public runtime commands/tokens, the
  world-step coordinator, replay frontier, the TUI command loop, parity/adversarial surfaces,
  and the negative fixtures.
- **Sealed wait-receipt / embodied-debug separation** (`receipt.rs`/`scheduler.rs` + TUI
  tests): default-transcript exact-leak-absence assertions for the wait path, debug-token
  presence assertions, and a hidden-context/debug-receipt information-flow differential.
- **Food-source actor-known witness** (`projections.rs`/`epistemics/projection.rs` + public
  tests): the §4.7 public observation/record-driven competing-fact witness.
- **Governance audit job**: the ruleset-detail API check parsing approval/last-push/reviewer
  fields (§4.5).
- **Acceptance state machine + closed wording grammar**: the §4.4 parser conversion,
  expected-finding manifest, actual-artifact ingestion, and `ci_workflow_guards.rs` checks,
  plus a mutation campaign over the parser/guard functions.
- **Mutation**: focused campaigns during implementation for fast feedback, then the configured
  standing campaign after all code/test work; make in-diff mutation PR-blocking for guarded
  changes (§4.6); the canonical perimeter may not be called green while any survivor remains.
- `include_str!` / workflow-YAML import/method guards remain labeled topology alarms only.

## 8. Acceptance artifact and evidence

On implementation the session must: begin from a clean baseline; name its own exact
implementation commit (not this proposal's baseline `7660051`); run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo build --workspace --all-targets --locked`, and `cargo test --workspace`; run focused
mutation campaigns for each in-surface change and then the full standing campaign **after** all
code/test work; and publish the selected denominator with the complete
caught/missed/unviable/timeout disposition. The acceptance artifact follows
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, **carries the §4.4 machine-readable status
manifest computed by the fail-closed state machine**, records per-finding closure with real
production-path evidence (sealed constructor, actor-legible wait receipt, non-inducible debug
authority, observed effect, sensitivity/negative-compile proof), records the required-check
names **and a ruleset API transcript including the approval/last-push/required-reviewer fields**,
and must not render `pass` unless the manifest computes pass — i.e. unless every required
finding is `closed`, governance independence is enforced (or honestly `pending-governance`
computing non-pass), mutation evidence is current and green, and the actual artifact under
review was ingested. Doc-truthing (§6.2) lands only after the executable witnesses exist; the
§6.1 doctrine substance lands with the F6-04/F6-05/F6-06 code. Every executable claim is the
implementing session's to produce — this spec asserts no green/red command result.

## 9. Implementation constraints

- Follow the driver's recommended **closure order** (report §10): (1) seal bootstrap authority
  and repair the negative fixture to attack current symbols (F6-01); (2) seal the one-tick wait
  receipt and move full scheduler detail behind debug/operator authority (F6-02); (3) repair
  debug-authority acquisition and add a bypass-shaped external negative fixture (F6-03); (4)
  strengthen public-boundary tests across bootstrap, wait-receipt internals, debug-token
  induction, and embodied field mutation using live APIs; (5) convert the acceptance taxonomy
  to an expected-finding/evidence-ingestion state machine with a closed wording grammar
  (F6-04); (6) require independent approval or last-push/required-reviewer mechanics or compute
  `pending-governance` (F6-05); (7) make in-diff mutation PR-blocking for guarded changes
  (F6-06); (8) harden the `food_source` residual through a public actor-known path and run the
  mutation perimeter (F6-07); (9) only then truth live conformance docs (§6.2). The report
  installs code-authority sealing first this pass (bootstrap/wait/debug) because those are the
  live foundational violations; the process/governance/taxonomy repair follows so the next line
  cannot launder a pass.
- No backwards-compatibility shim or public alias path in new work; a temporary internal
  adapter may exist only to migrate core tests and must be removed before closeout.
- Core must not depend on content or tui; the sealed bootstrap authority, wait receipt, and
  debug authority live in `tracewake-core` (or a dedicated internal authority crate per §10.1,
  preserving the one-way dependency direction).
- Worktree discipline: if implemented in a worktree, all paths resolve against the worktree
  root.
- Decompose into one ticket per reviewable diff (the closure-order steps are the natural ticket
  boundaries); the reassess/decomposition step determines the ticket prefix continuing the
  sibling convention (`0048FOUCONHAR` → `0050FOUCONSEC` → `0051FOUCONTHI` → `0052FOUCONFOU` →
  `0053FOUCONFIF` → a `0054`-keyed successor such as `0054FOUCONSIX`).

## 10. Risks and open questions

These are implementation choices inside settled doctrine; none blocks the determination
(driver §11 open maintainer decisions):

1. **Cross-crate bootstrap authority topology** — how to express the content-validation witness
   without a backwards-compatible alias path. Because `tracewake-content` is a separate crate,
   pure `pub(crate)` may be insufficient; a sealed token/package boundary is likely the cleanest
   route. Non-negotiable: no external crate can fabricate a validated bootstrap; Cargo features
   are not security.
2. **Debug-attach model** — whether local-TUI debug attach is an operator action in development
   builds, a runtime authority granted at app launch, or an explicit debug-session mode. Any
   choice must be structurally non-embodied and not mintable through ordinary public commands.
3. **Wait-receipt actor-legible API** — which temporal/progress fields, if any, are legitimately
   actor-known on a normal one-tick wait; everything else is debug/operator-only.
4. **Independent acceptance for a sole-maintainer repository** — if no independent reviewer
   exists, the honest result is non-pass/`pending-governance` for independent-acceptance closure,
   not a self-accepted pass. The maintainer records the governance model and API proof.
5. **Mutation freshness window** — how fresh a full standing campaign must be for closure over
   guarded surfaces; the report recommends in-diff PR blocking plus scheduled full
   reconciliation, with the exact freshness window owned by execution `10`/CI.
6. **Acceptance state-machine format/home** — the expected-finding manifest format, the
   actual-artifact ingestion mechanism, and which existing Rust test/CI job consumes them;
   non-negotiable: the overall result is computed from current evidence, not asserted in prose.

Open recurrence risk: this is the **sixth** consecutive pass to find divergence on this seam
after a remediation line claimed closure. 0053 proved that a sealed `continue` receipt, a
crate-private debug-token struct, and a status-block parser are not the same as authority
closure while a public raw validated constructor, an unsealed wait receipt, a publicly-inducible
debug bind, and a self-authored field-presence taxonomy sit beside them. The §3 three-layer
barrier bound to the §4.4 fail-closed acceptance state machine is the structural bet that breaks
the cycle; if the implementation leaves any public raw bootstrap path, debug-grade wait receipt,
publicly-inducible debug token, self-consistency-only taxonomy, zero-approval-self-merge
governance, or alarm-only mutation gate, the seam will reopen for a seventh pass.

## 11. Invariants alignment

| Invariant(s) | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-001, INV-009, INV-011, INV-018, INV-022, INV-092 | aligns | §4.1 re-seals the validated loaded-world bootstrap behind the authority path against the live API (crate-private/sealed-package constructor + live-API compile-fail negative fixture), so authoritative state/log/projection cannot be fabricated from raw parts beside the validated path @ runtime/content boundary. |
| INV-002, INV-023, INV-024, INV-030, INV-099, INV-101, INV-112 | aligns | §4.2 replaces the public `OneTickAdvanced(WorldAdvanceResult)` with a sealed actor-legible wait receipt and confines exact tick/frontier/event/due-work/ancestry/trace/pipeline detail to token-gated debug receipts, so a normal wait exposes no hidden temporal/world truth as cognition-grade output @ runtime-receipt/scheduler boundary. |
| INV-004, INV-005, INV-006, INV-031, INV-068, INV-094, INV-107, INV-108 | aligns | §4.3 makes `DebugSessionAuthority` non-inducible through the ordinary public command surface (operator-authority-gated debug entry), so debug capability is independently held and non-diegetic rather than self-bound, preserving possession parity @ runtime command boundary. |
| INV-008, INV-067, INV-069, INV-093 | aligns | §4.2/§4.3 keep the embodied surface actor-known and the debug surface separate at the type boundary (not by render discipline), and §4.4 tests leakage paths @ TUI/view-model + acceptance surface. |
| INV-098-class acceptance discipline | aligns | §4.4/§4.5/§4.6 convert acceptance into a fail-closed state machine computing `pass` only from current, independently-acceptable, mutation-green evidence (expected-finding manifest + closed wording grammar + ruleset transcript parse + PR-blocking mutation + actual-artifact ingestion), ending pass-shaped artifacts over open defects @ CI/governance + acceptance surface. |
| (all above) | N/A — no Tier-0 amendment | §3/§6/driver §9: the foundation already forbids every bad state; the only doctrinal change is operationalizing existing evidence-honesty rules below foundation at architecture `13`/`10`/`04`, execution `10`/`07`/`05`, reference `00`/`01`, and template `0003`; no invariant is weakened, minted, or redefined. |

## Outcome

Completed: 2026-06-28

Implemented and archived by tickets `0054FOUCONSIX-001` through
`0054FOUCONSIX-013`. The series re-sealed validated loaded-world bootstrap
construction, replaced the normal one-tick wait product with an actor-legible
sealed receipt, made debug-session authority non-inducible through ordinary
public commands, converted acceptance reporting into a fail-closed status
manifest, wired in-diff mutation and governance checks into the standing
barrier, forced the public actor-known `food_source` witness, and truthed the
below-foundation doctrine/reference rows.

The capstone evidence artifact is archived at
`archive/reports/0054_foundational_conformance_sixth_hardening_acceptance.md`.
It computes `pass` for the scoped 0054 hardening line at exact implementation
commit `24a458243b2d8bcc08c833824cc75cec1c904f42`, with current standing
mutation evidence from `6d7009f61e3f7d55f81da3be3297160c6f2fb402`: denominator
`3445`, `2679` caught, `766` unviable, `0` missed, and `0` timeout. The live
GitHub ruleset `main-standing-conformance-barrier` (`18200914`) is active,
has no bypass actors, records `current_user_can_bypass: never`, enforces the
standing required status checks, and requires one approving review, satisfying
the independent-review governance path.

This closeout does not certify latest main, Phase-4 entry, second-proof entry,
institutions, notices, travel, LOD, LLM/speech, story-sifting, future feature
surfaces, or any unqualified whole-project status.
