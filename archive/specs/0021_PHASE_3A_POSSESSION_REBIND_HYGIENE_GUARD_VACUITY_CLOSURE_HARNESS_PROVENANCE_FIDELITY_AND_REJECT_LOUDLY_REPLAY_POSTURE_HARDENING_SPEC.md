# 0021 Phase 3A Possession-Rebind Hygiene, Guard-Vacuity Closure, Harness-Provenance Fidelity, and Reject-Loudly Replay Posture Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `89059a5` (post-0020 closeout; all `0020PHA3ACOGSUR` tickets landed, merge PR #28).
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the tenth Phase 3A alignment pass — the verification-leaning pass that 0020
§9 itself warranted, extended with a fresh-eyes residual sweep of the under-audited
corners of the spec-0005 surface. Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed
`008`–`013`; 0016 closed `014`–`025`; 0017 closed `026`–`034`; 0018 closed `035`–`043`;
0019 closed `044`–`052`; 0020 closed `053`–`065`. This audit re-derived the normative
contract from `docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, and
`docs/3-reference/*`, re-examined every 0020 correction at the post-0020 baseline
(treating each as unverified until proven in code, per the lineage rule), and deep-swept
three surfaces the lineage had never audited at depth: the TUI possession/host seam, the
content-loader internals, and the world-applier catch-alls. Findings continue the
`ORD-HARD` series at `066`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The
audit was conducted as a nine-slice delegated review (cognition supersession, derived
apply-arm census, mutation perimeter/baseline governance, generative lock tier, content
seeding containment, TUI/embodied surface, scheduler/kernel, evidence-document honesty,
fresh-eyes residual sweep — the last delegating sub-sweeps over the content crate, the
TUI crate, and events/state). Every blocker/high finding and seventeen of nineteen
medium findings were independently operator-verified at their load-bearing cited
symbols during the audit; a reassessment pass at the same baseline then
operator-verified every remaining delegated claim at source (zero refuted — the
per-finding **Verification:** lines record the history), so all thirty-three findings
are operator-verified and no re-verification step is owed at implementation time.

The root patterns this pass:

1. **Guard vacuity.** A structural lock that asserts the *artifact* (a policy table, a
   filter list, a pinned hash, a ledger's entry count) rather than the *behavior* passes
   green while enforcing little or nothing. Five independent instances: the apply-arm
   census's per-arm anchor refinement is dead code (`ORD-HARD-069`), the scheduled
   mutation job's baseline ratchet is unreachable (`ORD-HARD-067`), the per-kind
   supersession policy table has zero behavioral callers (`ORD-HARD-074`), the exemption
   `typed_columns` lists are decorative (`ORD-HARD-070`), and the baseline disposition
   ledger is a bulk-accept in per-entry costume (`ORD-HARD-073`).
2. **The unaudited seam.** The highest-severity product-behavior finding
   (`ORD-HARD-066`) lives in the TUI host seam — a surface every prior pass treated as
   render-only plumbing. The fresh-sweep frontier is now exhausted: all three crates
   have been deep-audited at least once.

Two notes that distinguish this pass from its predecessors:

- The kernel product-behavior surface verified clean for the **fourth consecutive
  pass**, and the evidence documents verified **fully honest for the second consecutive
  pass** — every checkable claim in the 0020 acceptance report (including the EMERGE-OBS
  table, re-run and reproduced byte-for-byte), the disposition ledger's bidirectional
  baseline parity, the four conformance rows, the R-27/R-28 risk entries, and the spec
  ledger row matched the code.
- Every 0020 correction verified substantive at source; none regressed. The residue is
  family members the corrections did not reach (`ORD-HARD-069`/`074`/`075`/`086`) plus
  the genuinely new guard-vacuity and unaudited-seam findings.

## 1. Scope

### In scope

- The TUI possession/host seam (`crates/tracewake-tui/src/app.rs`, `render.rs`) and the
  embodied view-model boundary (`crates/tracewake-core/src/projections.rs`).
- The hidden-truth gate harness's own context construction
  (`crates/tracewake-core/src/agent/actor_known.rs::observe_visible_local`,
  `crates/tracewake-core/tests/hidden_truth_gates.rs`).
- The derived apply-arm census and its enforcement mechanics
  (`crates/tracewake-core/tests/anti_regression_guards.rs`,
  `crates/tracewake-core/src/events/apply.rs`).
- Mutation-testing CI semantics, perimeter governance, and baseline-ledger substance
  (`.github/workflows/ci.yml`, `.cargo/mutants.toml`,
  `.cargo/mutants-baseline-misses.txt`, `reports/0020_mutants_baseline_disposition.md`).
- The actor-known supersession policy layer
  (`crates/tracewake-core/src/epistemics/projection.rs`,
  `src/agent/no_human_surface.rs`, `src/agent/perception.rs`).
- Replay reject-loudly posture in the event appliers (`src/events/apply.rs`,
  `src/state.rs`) and the action-def completion builders (`src/actions/defs/`).
- Content-loader collision/repair behavior and fixture-contract honesty
  (`crates/tracewake-content/src/schema.rs`, `src/fixtures/`).
- The generative lock tier's fabricator-ban scope and flush-parity residue
  (`crates/tracewake-core/tests/generative_lock.rs`, `tests/support/generative.rs`).
- Risk-register memory for the guard-vacuity pattern
  (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`).

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic internals
  (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0020 fixed that this audit verified as holding
  (see §3 "Verified holding").

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-006/008, INV-009…INV-022,
  INV-024/026/028, INV-043, INV-067/069/070/071, INV-087, INV-093/094, INV-102…INV-106,
  INV-108, and the Enforcement reading ("tests must be corrected when they reward …
  actor proposals derived from raw physical state").
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — the
  one-freshness-rule and the supersession contract (`ORD-HARD-042`/`046`/`053`).
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — replay
  rejects unsupported history loudly; rejection is a typed error channel, never a
  process abort and never a silent default.
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` and
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — a
  gate's evidence must be produced by the path under test.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-27 (acceptance-evidence
  reachability overstatement), R-28 (incomplete correction closure); this pass adds the
  guard-vacuity pattern (§6).
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale — and a lock is real only if its negative case can actually fire.

No new external research was consulted this pass; the 0017/0018 lock-design corpus
remains the reference for lock design.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not maximally locked, and three
findings are letter-level foundational violations: `ORD-HARD-066` (possession transfers
rejection knowledge across actors, INV-006/067/093/108), `ORD-HARD-068` (fabricated
provenance inside the gate harness, INV-102 + Enforcement reading), and `ORD-HARD-078`
(perceptual ground truth decided by display-label prose, INV-022). Thirty-three
findings follow, `ORD-HARD-066` … `ORD-HARD-098`: three high, nineteen medium, eleven
low, zero blockers.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-053/058 (supersession parity) are substantive.** Supersession lives once,
  in `projection.rs::classified_actor_known_records_for_context` (helpers
  `newest_workplace_records`, `workplace_record_is_newer`: `source_tick` then
  `source_event_id`), consumed by both `NoHumanActorKnownSurfaceBuilder` and
  `current_place_knowledge_context`; the per-surface duplicate logic is gone;
  `guard_015` structurally bans truth reads in the builder. Proving tests
  (`workplace_notices_supersede_before_no_human_facts_are_minted`,
  `same_tick_workplace_notice_tie_uses_source_event_id_on_no_human_surface`,
  `workplace_same_tick_tie_breaks_on_source_event_id_for_both_surfaces`) build
  projections from real event logs. (The decorative policy table is `ORD-HARD-074`;
  the exported bypass is `ORD-HARD-068`.)
- **ORD-HARD-057 (remembered food) is substantive.** The `FoodSource` place-mismatch
  early-return is removed; `food_record_from_other_place_surfaces_as_remembered_find_food_input`
  proves HTN reachability from remembered-only knowledge. (The accessibility-fact
  asymmetry that survives is `ORD-HARD-075`.)
- **ORD-HARD-054 (derived census) landed its derivation.** The census derives map names
  from `AGENT_STATE_CHECKSUM_COVERAGE` and scans `events/apply.rs` source; no
  hand-enumerated map list remains; all six `TYPED_COLUMN_CLOSURE_EXEMPTIONS` rationales
  verified TRUE against the actual apply functions; the version-gated arms verified
  (`payload_schema_version`/`trace_schema_version`/`diagnostic_schema_version`, all
  hardcoded `"1"`, fail-closed on a bump); `checksum_coverage_is_total_for_authoritative_state`
  genuinely derives from `state.rs` source with synthetic negatives. (The vacuous
  per-arm anchor is `ORD-HARD-069`; the secondary closure gaps are `ORD-HARD-070`.)
- **ORD-HARD-055/056 (mutation perimeter/baseline) landed mechanically.** `eat.rs` is
  in the perimeter constant, both CI jobs' filters, and not excluded; the pinned
  normalized count (143) and FNV-1a64 hash (`bd1855a5ee82b428`) were independently
  recomputed and match; bidirectional baseline↔ledger parity holds; all five synthetic
  canaries in the perimeter guard genuinely fire; the in-diff job's exit-status
  semantics are correct. (The dead scheduled ratchet is `ORD-HARD-067`; the
  exclusion-channel and rationale-family residue are `ORD-HARD-071`/`072`; the ledger's
  substance is `ORD-HARD-073`.)
- **ORD-HARD-059/061 (generative fidelity) are substantive.**
  `assert_duration_terminal_payload_tamper_poisons_replay` hard-asserts poisoning per
  payload field of a selected duration terminal (no existential early return);
  contributor sets are derived from emitted-event predicates with exact-equality
  assertions; the `NeedBand::Severe` assertions bind the same constants
  `initial_agent_state` injects; the seed corpus grew (+`0x18_00_00_2A`) with no
  removals. (Flush-parity residue is `ORD-HARD-086`; minor locks are `ORD-HARD-092`.)
- **ORD-HARD-060's displacement case is genuinely advance-emitted.** The
  `wait_work_displace` mask drives a real `move` proposal through the shared movement
  path and `work_completion_failure` emits `actor_displaced` on the scheduler path,
  counted from `run.log` only. (The once-at-end flush divergence persists —
  `ORD-HARD-086`.)
- **ORD-HARD-062 (transitive helper census) is substantive at depth 1.** The blanket
  call is gone from `hidden_truth_adversarial_fixture`;
  `with_actor_mara_known_hidden_food` seeds exactly one explicit actor→supply edge; the
  census discovers wrappers generically by enclosing-function extraction (a new wrapper
  is detected without code changes); the synthetic indirect-consumer regression is
  real; the five-consumer list verified exact by workspace grep. (Depth-2 closure and
  workspace-scope residue are `ORD-HARD-093`.)
- **ORD-HARD-063 (exit blocker) is produced and consumed.** `blocker_summary` is
  derived in `build_embodied_view_model` for doors adjacent to the viewer's place,
  the unperceived-blocker omission test is real, and the TUI render test proves
  consumption. No hidden-truth leak today: the surfaced door is always co-located with
  the viewer. (The sweep's universe gaps are `ORD-HARD-083`; the geometry-implied
  perception gate's fragility is recorded there.)
- **ORD-HARD-064/065 (window progress / routine eligibility) hold.**
  `append_due_completions` returns appended terminals and both crediting sites feed
  `progress_by_window_actor` before the `WindowNoProgress` loops; `routine_window_family`
  and `active_routine_execution_for_actor` both use
  `execution.start_tick <= window.start_tick`; the mid-window-start test exists; TUI
  behavior rows updated consistently. (The attribution divergence between the two
  crediting sites is `ORD-HARD-088`.)
- **Cross-cutting kernel checks clean for the fourth consecutive pass:** every
  production state write in `scheduler.rs` goes through
  `apply_agent_event`/`append_and_apply_agent_event`/`run_pipeline` (all direct
  inserts are `#[cfg(test)]`); every autonomous proposal is built through
  `ActorDecisionTransaction::run` with `source_context_check` and typed stuck
  diagnostics; `planner_goal_for` and `htn::resolve_condition` read only actor-known
  context (compile-fail doctest locks out `From<PhysicalState>`); single-charge
  accounting and typed `DuplicateDurationTerminal` hold; work/sleep need-gates read
  authoritative state in validators only; possession parity and debug quarantine hold;
  `routine.rs`/`need.rs`/`htn.rs` core verified clean; `wait.rs` clean (requires a
  reason, applies real passive decay); `movement.rs`/`sleep.rs`/`eat.rs` validator-side
  truth reads only; `events/log.rs` append-only with strict position checks;
  `envelope.rs` rejects unknown kinds/streams/versions; checksum coverage total.
- **Evidence documents verified honest — second consecutive clean pass.**
  `reports/0020_ord_life_cert_scoped_acceptance.md`: every cited test exists, the
  EMERGE-OBS table reproduced byte-for-byte by re-running the test, the observer-only
  claim verified (`rg` over all `src/` trees returns nothing), the manifest commits
  match git. `reports/0020_mutants_baseline_disposition.md`: full bidirectional entry
  parity with the baseline file. The four 0020 conformance rows match enforcement; the
  0017 "pinned mutation baseline" wording was corrected to "governed" as required; the
  0019 rows were proactively kept true. R-27/R-28 exist with guardrails and escalation
  triggers. The SPEC_LEDGER 0020 row is accurate and per convention.
  (`ORD-HARD-073` concerns the ledger's *substance*, not its honesty: it accurately
  records dispositions that are themselves boilerplate.)

### Validated — no action (checked, found intentional or correctly scoped)

- Content serialization is fully hand-rolled and fail-closed: no serde, no defaults;
  unknown lines, missing fields, bad hex, schema-version mismatches, and roundtrip
  drift are all rejected; manifest fingerprint covers canonical bytes (INV-017/020).
- INV-022/062/063 clean in the loader: no prose field drives loading; every seed event
  carries `source_kind: authored_prehistory` (the only representable variant); the
  schema has no triggers, completion flags, or outcome chains.
- `SocialNormPlaceholder`, `ReadingPlaceholderSchemaOnly`, `service_completed_placeholder`,
  `InstitutionPlaceholder` are deliberate later-phase schema staging (pipeline comment
  confirms), not gaps.
- Authored affordances may target items no actor knows (truth-side possibility space,
  INV-044) — re-confirmed, not re-litigated.
- The 148→143 baseline arithmetic is normalization dedupe (155 raw lines, line:col
  stripped, sorted unique), honestly worded as "normalized" in the acceptance report.
- Deterministic trace/execution IDs from (actor, tick, family) in `htn.rs` are safe
  under one-decision-per-actor-per-window.
- `pipeline.rs`'s poisoned-log defensive branch (append succeeded, apply failed) is
  surfaced by replay validation; acceptable.
- TUI `run no-human-day` drives ordinary pipelines, output marked non-diegetic,
  gate-tested; `transcript.rs` and `input.rs` are pure presentation/parsing.

## 4. Findings and remediation requirements

Severity calibration note: `ORD-HARD-067` is rated high where its closest precedent
(`ORD-HARD-055`, medium) was an incomplete perimeter — the divergence is named in the
finding: this gate is non-functional, not merely incomplete. `ORD-HARD-068` is rated
high per the R-27 lineage precedent (harness-fabricated evidence under gates) despite
having no production caller. `ORD-HARD-069` stays medium per the `ORD-HARD-047`/`054`
census precedent.

### ORD-HARD-066 — Possession rebind retains the previous actor's why-not rejection on the new actor's embodied surface

**Severity:** high. **Verification:** operator-verified.

**Responsible layers:** TUI host seam, `projection` (view-model boundary).

**Doctrine breached:** INV-006 (possession transfers no world knowledge), INV-067
(embodied mode shows actor-known reality), INV-093 (actor-knowledge leakage is a
high-severity defect), INV-108 (possession is cognition-neutral).

**Evidence:** `TuiApp::bind_actor` (`crates/tracewake-tui/src/app.rs`) attaches the
controller, records perception, and sets `bound_actor_id` — but never clears
`self.last_rejection`. `run_no_human_day` in the same file *does* clear it (the
asymmetry proves the omission). `current_view` passes `self.last_rejection.as_ref()`
into `build_embodied_view_model`, which maps it unconditionally into
`last_rejection_summary`/`last_rejection_why_not` with no check against
`context.viewer_actor_id()`, even though `ValidationReport` carries
`actor_id: Option<ActorId>`. Repro: bind actor A → submit a blocked action whose
rejection carries actor-visible world facts (e.g. a door's locked state) → `bind`
actor B → B's embodied view renders A's why-not facts, which B never acquired through
any modeled channel. Mitigation keeping this high rather than blocker: display-only —
the stale report never feeds proposal construction or cognition.

**Why existing gates miss it:** the possession-parity gate
(`adversarial_gates_possession_rebind_does_not_transfer_notebook_or_debug_truth`)
rebinds only after *accepted* actions, so `last_rejection` is `None` in the proving
test.

**Required correction:** clear `last_rejection` in `bind_actor`; additionally guard in
core by `report.actor_id` — `build_embodied_view_model` must drop a rejection report
whose `actor_id` is not the viewer (defense in depth, so a future host seam cannot
reintroduce the leak).

**Structural lock:** a rebind-after-rejection gate: bind A, submit a blocked action,
rebind B, assert B's view model carries no rejection fields; plus a core unit test
that a mismatched-`actor_id` report yields `None` fields regardless of host behavior.

### ORD-HARD-067 — The scheduled mutation job's baseline ratchet is unreachable: the job is permanently red and its baseline check is dead code

**Severity:** high (calibration divergence from `ORD-HARD-055` named above).
**Verification:** operator-verified.

**Responsible layers:** CI, `test_oracle`.

**Doctrine breached:** lock durability (a permanently-red gate emits no signal and
trains alarm fatigue; the baseline-comparison logic that distinguishes new misses from
accepted misses never executes); the `ORD-HARD-045` correction's loud-failure
semantics were applied to the in-diff job only — incomplete correction closure (R-28).

**Evidence:** the scheduled job's run step (`.github/workflows/ci.yml`, "Run
guarded-layer mutation baseline") invokes `cargo mutants --workspace -f …` with no
`set +e`/status capture — the only `mutants_status=$?` pattern in the file belongs to
the in-diff job. cargo-mutants exits 2 on any missed mutant; the 143 accepted baseline
misses guarantee exit 2 on every scheduled run; the step fails; the next step ("Check
missed-mutant baseline") has no `if: always()` and never executes. Every scheduled run
is red regardless of whether new misses exist. Additionally, the perimeter guard's
failure-semantics checks police only `cargo mutants --in-diff` lines — appending
`|| true` to the scheduled `cargo mutants --workspace` invocation passes every guard.

**Why existing gates miss it:** the guard checks filter strings and the in-diff job's
shell semantics; nothing asserts the scheduled job's status-capture pattern or that
its baseline-check step is reachable.

**Required correction:** replicate the in-diff status-capture pattern in the scheduled
job (`set +e`, capture `$?`, treat 0/2 as ratchet input and other codes as tool
failure), so the baseline `comm -23` comparison actually decides the job's outcome.

**Structural lock:** widen the perimeter guard to require status capture on every
`cargo mutants` invocation in `ci.yml` and to flag `|| true` on any of them; a
synthetic case removing the scheduled job's status capture must fail the guard.

### ORD-HARD-068 — An exported planning-context builder fabricates provenance and mints role facts from visibility, and the hidden-truth gates build their contexts through it

**Severity:** high (calibration: R-27 precedent — harness-fabricated evidence under
gates — despite no production caller). **Verification:** operator-verified.

**Responsible layers:** `agent_cognition`, `test_oracle`.

**Doctrine breached:** INV-102 (cognition inputs require provenance sufficient for
replay review — a fabricated constant is not provenance), INV-026, INV-024 in spirit
(a `workplace_assignment_active` fact minted from mere visibility forges the notice
channel); the constitution's Enforcement reading (tests must be corrected when they
reward cognition derived from raw physical state).

**Evidence:** `observe_visible_local` (`crates/tracewake-core/src/agent/actor_known.rs`,
`pub`) and its callers `build_actor_known_planning_state` /
`build_actor_known_planning_state_with_projection_limitation` (both `pub`, re-exported
from `agent/mod.rs`) stamp every produced fact with the fabricated constant
`EventId::new("event_visible_local_planning_state")` — an id that passes `audit_with`
but corresponds to no logged event — perform no supersession or freshness
classification, and mint `workplace_assignment_active` (satisfying
`RoutineCondition::WorkplaceAssignmentActive`) from workplace *visibility* with no
notice event. No production caller exists (the production planner path is
single-channel: `scheduler.rs` → `NoHumanActorKnownSurfaceBuilder` →
`ActorDecisionTransaction::run`; the `planner.rs` import is consumed only by its
`#[cfg(test)]` module). But `tests/hidden_truth_gates.rs` builds its planner-gate
contexts through it, so several hidden-truth gates assert provenance over the
harness's own fabricated source event — and the exported functions are an open
classifier-bypassing third surface for any future caller.

**Why existing gates miss it:** the hidden-truth audit checks that facts carry
provenance and are not validator-only truth; a fabricated-but-well-formed `EventId`
satisfies both checks. `guard_015` bans truth reads in the no-human builder, not in
this parallel builder.

**Required correction:** demote `observe_visible_local` and both
`build_actor_known_planning_state*` functions to `#[cfg(test)]`-gated or `pub(crate)`
test-support — or, if a legitimate need exists, route them through
`classified_actor_known_records_for_context` and real perception events. Rebuild the
`hidden_truth_gates.rs` contexts from real event logs via `apply_epistemic_event` (the
pattern the 0020 supersession tests already use). The `workplace_assignment_active`
visibility-minting must be deleted in either case.

**Structural lock:** a source guard asserting no `pub` producer of
`ActorKnownPlanningContext` exists outside the builder/classifier pair, and that
`hidden_truth_gates.rs` contains no call to the fabricating builder; a guard banning
the fabricated-constant `EventId` literal workspace-wide outside the definition site
during the migration window.

### ORD-HARD-069 — The apply-arm census's per-arm enforcement is vacuous for inline arms: cross-arm gate contamination

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `test_oracle`, `replay`.

**Doctrine breached:** INV-020; lock durability — the `ORD-HARD-054` correction's
"per-arm gate-or-exemption" contract is not delivered for inline arms (R-28 incomplete
correction closure, the census's *own correction* this time).

**Evidence:** `enclosing_apply_anchor` (`anti_regression_guards.rs`) refines a write
site's anchor to its `EventKind::` match arm only when the enclosing function is
exactly `"apply_agent_event"` — but every inline write lives in
`apply_agent_event_with_capability` (`events/apply.rs`), so the refinement is dead code
and all six inline arms share one function-level anchor.
`apply_write_site_has_version_gate` then scans from the *function start* to the write
site, so any earlier arm's `require_payload_version` call satisfies any later arm's
check. Concretely: deleting the gate from the ContinueRoutine arm leaves the census
green; only the first arm is genuinely enforced; a new unversioned inline write to any
covered map placed after arm one also passes.

**Why existing gates miss it:** the synthetic regression case exercises only the
helper-function anchor path; the behavioral forged-version tests cover only some kinds.

**Required correction:** anchor inline sites to the nearest preceding `EventKind::`
token whenever the enclosing function name starts with `apply_agent_event`, and start
the gate-scan segment at that arm token.

**Structural lock:** a synthetic two-arm inline match (arm 1 gated, arm 2 ungated)
that must fail the census; per-arm behavioral forged-version tests for every gated
kind.

### ORD-HARD-070 — Census secondary closure gaps: the write-shape universe is open and exemption `typed_columns` are decorative

**Severity:** medium. **Verification:** operator-verified (the typed_columns half at
reassessment: the meta-guard checks only `!typed_columns.is_empty()`, and `status` is
consumed by `apply_intention_started` yet absent from its exemption's list).

**Responsible layers:** `test_oracle`, `replay`.

**Doctrine breached:** INV-020/INV-018; lock durability.

**Evidence:** (a) the census scanner matches only literal `state.<map>.` followed by
`insert(`/`entry(`/`get_mut(`; writes via `remove`/`retain`/`extend`, a rebound
`&mut state.<map>` local, or a helper taking `&mut BTreeMap` are invisible — and
set-equality requires only ≥1 *visible* site per map, so a future hidden write fails
nothing (no current bypass instance exists; verified by sweep). (b) the
`TYPED_COLUMN_CLOSURE_EXEMPTIONS` `typed_columns` lists are checked for non-emptiness
and anchor liveness only — nothing derives the keys an exempted helper actually
consumes (e.g. `status` and `intention_source` are consumed by
`apply_intention_started` but absent from its list), and nothing would detect an
exempted helper later retaining `payload_fields(event)`.

**Why existing gates miss it:** the meta-guard checks exemption shape, not exemption
truth; the scanner's positive whitelist cannot see what it does not match.

**Required correction:** invert the scanner to flag any `state.<map>.` followed by a
method not in a read allowlist, and assert `&mut state.<map>` bindings and
`payload_fields(` are absent outside registered sites; derive consumed payload keys
per exempted anchor (scan for `required(payload, "…")`/`payload.get("…")`) and assert
they are a subset of `typed_columns`.

**Structural lock:** synthetic cases for a `retain`-shaped hidden write and an
exempted helper consuming an unlisted key — both must fail.

### ORD-HARD-071 — `exclude_globs` is an unguarded perimeter bypass, and the CI filter checks are decoy-able text matches

**Severity:** medium. **Verification:** operator-verified (the decoy half at
reassessment: scheduled filters are whole-file `contains()` checks, the in-diff line
is the first `grep -E` + `actions/defs/` match, and only the defs alternation group is
exactly parsed and stem-matched).

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability — the instance/family gap applied to the
exclusion channel (`ORD-HARD-055`'s correction covered filter *presence*, not
exclusion *absence*).

**Evidence:** the perimeter guard forbids only the literal
`crates/tracewake-core/src/actions/defs/**` in `mutants.toml`; adding
`…/defs/eat.rs`, `…/agent/**`, or `**/scheduler.rs` to `exclude_globs` silences those
files with every guard green (config excludes govern `--in-diff` runs outright and
subtract from the scheduled job's CLI `-f` selections). Separately, the guard's
scheduled-filter checks are `ci_yml.contains("-f '…'")` anywhere in the file and the
in-diff check matches the first line containing `grep -E` + `actions/defs/` — a
comment containing the canonical strings satisfies the guard while the real step is
gutted; only the `defs/(…)` alternation group is actually parsed and stem-matched,
so narrowing `src/agent/` to `src/agent/planning/` in the real regex passes.

**Why existing gates miss it:** the lock covers one literal string and textual
presence, not the exclusion channel or the executing step.

**Required correction:** glob-match every `exclude_globs` entry against
`MUTATION_PERIMETER_CANARY_PATHS` and fail on any intersection; anchor filter checks
to the specific job/step block; evaluate the actual in-diff regex against each canary
path (extend the existing stem canary to the non-defs alternatives).

**Structural lock:** synthetic cases — an exclusion glob covering a perimeter path, a
commented-out decoy filter line, a narrowed regex — each must fail the guard.

### ORD-HARD-072 — Mutation-claiming exemption rationale decorates nine non-perimeter action defs: the `ORD-HARD-055` defect family recurs

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `test_oracle`.

**Doctrine breached:** lock durability; R-28 — `ORD-HARD-055` was fixed for the
instance (eat.rs), not the family (a rationale claiming mutation coverage on files
mutation never reaches).

**Evidence:** `takeplace.rs` (~740 LOC), `wait.rs` (~555), `continue_routine.rs`
(~479) — all larger than the eat.rs that warranted perimeter inclusion — are outside
the perimeter yet classified `Exempt { rationale: CORE_ACTION_RATIONALE }`, whose text
claims coverage by "targeted action, pipeline, and duration-definition mutation
guards". The rationale-must-mention-mutation check applies only to perimeter canary
paths, so a false mutation-claiming rationale on non-perimeter files is unchecked.

**Why existing gates miss it:** exactly the `ORD-HARD-055` miss pattern, one
allowlist row over.

**Required correction:** split the rationale constant (perimeter-covered defs vs.
non-perimeter defs with an honest rationale), and either bring the three large defs
into the perimeter with an honestly refreshed baseline or exempt them with a rationale
that is true.

**Structural lock:** a guard rule that any `Exempt` rationale containing "mutation"
must name a path matched by the perimeter filters; synthetic violating case.

### ORD-HARD-073 — The baseline disposition ledger is a bulk accept in per-entry costume; the promised triage did not happen

**Severity:** medium. **Verification:** operator-verified (boilerplate count).

**Responsible layers:** `test_oracle`, evidence honesty (substance, not accuracy).

**Doctrine breached:** lock durability; the 0020 ticket's own contract ("no
bulk-accepted refresh"; per-entry disposition justified-baseline / warrants-test →
follow-up ticket).

**Evidence:** all 143 entries in `reports/0020_mutants_baseline_disposition.md` are
`justified-baseline`; 137 of 143 share the identical templated phrase "warrants a
future focused assertion"; zero warrants-test dispositions; `tickets/` contains no
follow-ups; zero baseline entries were retired by writing tests (git shows 148 raw →
155 raw: 7 eat additions, 0 removals; the 143 is normalization dedupe). The
governance guard's rationale check is presence of `" — "` only, so templated text
passes forever.

**Why existing gates miss it:** the guard pins count/hash/parity and rationale
*presence*; nothing distinguishes a disposition from a template.

**Required correction:** perform the real triage once: entries in interruption
predicates and decision logic of guarded-layer files get focused tests and baseline
retirement; the rest get individually-true rationales; file the follow-up test-debt
tickets the 0020 ticket promised.

**Structural lock:** harden the guard to demand a disposition tag from a closed enum
(`justified-baseline` / `warrants-test:<ticket-id>`), require ticket IDs on
warrants-test entries, and bound identical-rationale repetition (e.g. no rationale
string may cover more than N entries without a grouped-rationale registration).

### ORD-HARD-074 — The per-kind supersession policy table is decorative: `policy()` has zero callers and the embodied path silently implements an undeclared policy

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `holder_known_context`, `test_oracle`.

**Doctrine breached:** lock durability — the `ORD-HARD-053`/`057` correction promised
"per-kind gating moves from ad-hoc arm logic to a declared per-kind policy table
asserted by a symmetry test"; the lock asserts the table's *contents*, not the
classifier's *behavior*, so behavior can drift while the test stays green (guard
vacuity).

**Evidence:** `ActorKnownProjectionRecord::policy()` (`epistemics/projection.rs`) has
zero callers; `ActorKnownProjectionPolicy::CurrentPlaceLatestOnly` is a dead variant
never returned or matched; `classified_actor_known_records_for_context` hardcodes
`match record { Workplace => supersede, _ => pass }`. Meanwhile
`current_place_knowledge_context` drops any non-workplace record where
`!classified.is_latest_current_place_record()` — the embodied path silently implements
exactly the `CurrentPlaceLatestOnly` policy the dead variant was drafted to name,
undeclared. Also in this family: `actor_knows_sleep_place` is hardcoded
`remembered_belief` even when currently perceived, while the adjacent
`actor_knows_sleep_affordance` fact is freshness-classified — an unexplained per-fact
divergence inside one arm (conservative direction; fold into the table).

**Why existing gates miss it:** `actor_known_projection_policy_table_declares_every_record_kind`
checks the table declares every kind; nothing checks the classifier or either surface
consults it.

**Required correction:** make the classifier (and the embodied context's per-kind
gating) dispatch on `record.policy()`; implement or delete `CurrentPlaceLatestOnly` —
if the embodied affordance-context semantics are intentional, declare them as that
kind's embodied policy in the table; resolve the `actor_knows_sleep_place` stamping in
the same table.

**Structural lock:** a behavioral test per record kind driven *from* the table
(iterate `actor_known_projection_policy_kinds()`, assert observed
supersession/reclassification behavior per kind on both surfaces); a source guard that
`policy()` has at least one production caller.

### ORD-HARD-075 — Sleep-place accessibility is place-gated on the no-human surface while food accessibility is not: the `ORD-HARD-057` asymmetry survives on the accessibility fact

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `holder_known_context`, `agent_cognition`.

**Doctrine breached:** INV-028 in spirit (the gate suppresses the actionable form of
remembered knowledge); R-28 — the 057 correction reached the knowledge fact but not
the accessibility fact.

**Evidence:** in `no_human_surface.rs`, `add_food_source_knowledge` mints
`food_source_believed_accessible` from anywhere, while `add_sleep_place_knowledge`
mints `sleep_place_believed_accessible` only when `place_id == self.current_place_id`.
`routine_sleep_night` (methods) requires `SleepPlaceBelievedAccessible` to expand and
its first step is `move_toward_place` — so a remembered sleep place is
planner-unreachable from elsewhere while remembered food is reachable.

**Why existing gates miss it:** the remembered-from-elsewhere test covers food only.

**Required correction:** declare per-kind accessibility gating in the `ORD-HARD-074`
policy table and either mirror food's treatment for sleep places or record an explicit
deferral with rationale — the two kinds must encode the same doctrine or a declared
difference.

**Structural lock:** the sleep mirror of
`food_record_from_other_place_surfaces_as_remembered_find_food_input` (a remembered
sleep place at place A is planner-reachable for `routine_sleep_night` after moving to
B), or the registered deferral.

### ORD-HARD-076 — Sleep/work-induced need-band crossings emit no `NeedThresholdCrossed` event

**Severity:** medium. **Verification:** operator-verified (absence confirmed by
sweep of `sleep.rs`/`work.rs` builders).

**Responsible layers:** `events`, actions kernel.

**Doctrine breached:** INV-012/INV-013 (mental/threshold events are real when they
affect later reasoning; meaningful events leave traces); foundation doc 05 lists
need-threshold crossings as a replanning trigger — that trigger has no causal record
when it happens during a duration block.

**Evidence:** `build_sleep_end_events` and `build_work_completion_events` emit
`NeedDeltaApplied` for fatigue/hunger but never a crossing event; `build_wait_events`
(via `build_threshold_events`) and the scheduler's pre-decision window path both do.
Hunger rising into Urgent/Severe during a sleep or work block leaves no eventful
trigger; the next decision reacts only because it re-reads bands from state. Related
(fold into the same correction): `build_threshold_events` uses `delta.max(0)`, so a
decreasing-pressure crossing from an authored negative passive delta would be
silently dropped while the delta still applies.

**Why existing gates miss it:** replay determinism passes (live and replay share the
omission); hidden-truth audits check cognition inputs, not emission completeness.

**Required correction:** emit crossing events from the completion builders using the
same `NeedState::threshold_crossing(current, next)` pattern as wait.rs, handling both
crossing directions.

**Structural lock:** a single shared "need delta + crossing" emitter so no builder can
apply a band-changing delta without the paired crossing event; a source guard that
`NeedDeltaApplied` construction outside the shared emitter fails.

### ORD-HARD-077 — `continue_routine`'s terminal-intention gate trusts proposer self-report and defaults to "active"

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** actions kernel (validation).

**Doctrine breached:** INV-043 (validation is ordinary-agent validation against
authoritative state, not proposer self-report); a fallback default inventing a
cognition fact.

**Evidence:** `build_continue_routine_event` reads
`proposal.parameters["intention_status"]` with a default of `"active"` and rejects
only self-reported terminal values. No production code sets `intention_status` on a
proposal (only tests do), so the gate can never fire from the autonomous path, and a
malformed proposal omitting the field passes as active. The validator never consults
`AgentState.intentions`/`active_intention_by_actor`, both available to the pipeline.

**Why existing gates miss it:** tests exercise the rejection by supplying the terminal
parameter, rewarding the self-report design.

**Required correction:** validate against the authoritative intention record keyed by
the actor's active intention; remove the parameter or derive it
authoritatively.

**Structural lock:** a rejection test for a terminal authoritative intention with the
parameter omitted.

### ORD-HARD-078 — Perceptual visibility is decided by display-label prose

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** perception channel, `events`.

**Doctrine breached:** INV-022 (raw prose is not authoritative state — letter-level: a
display label defines a hidden fact); INV-008 in spirit.

**Evidence:** `is_visible_exit_target` (`agent/perception.rs`) gates `visible_exit`
`ObservationRecorded` emission on
`!place.place_id.as_str().contains("hidden") && !place.display_label.to_lowercase().contains("hidden")`.
A place whose display label happens to contain "hidden" (or a hidden place relabeled)
silently changes perceptual ground truth.

**Why existing gates miss it:** `guard_014_no_human_metrics_do_not_scan_display_text`
bans display-text scans in projections only; no guard covers perception.rs; the
`hidden_*` fixtures bake the naming convention in, so tests pass.

**Required correction:** add a typed concealment attribute to places (schema +
`PhysicalState`) and gate visibility on it; fixtures author the attribute explicitly.
The attribute's authoring contract must itself be explicit — either required on every
place (the loader fails closed on absence) or a documented, registered schema default —
never a silent fallback, which would recreate the `ORD-HARD-085` silent-default shape
here. The `place_id` substring check goes too — IDs are identifiers, not semantics.

**Structural lock:** extend guard_014's family to perception.rs (no
`display_label`/id-substring branching in any emission path); a fixture with a
misleading label proving visibility follows the typed attribute.

### ORD-HARD-079 — The world applier's catch-all fails open

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `events`, `replay`.

**Doctrine breached:** INV-009/INV-020 (latent: a future World-stream kind added
without an apply arm silently no-ops in both live and replay — checksums agree, so the
silent repair is invisible).

**Evidence:** `apply_event_with_capability` (`events/apply.rs`) ends in
`_ => Ok(ApplyOutcome::NonWorldNoOp)`. The agent and epistemic appliers fail closed
(`Err(NonAgentEvent)` / `Err(NonEpistemicEvent)`). All 12 current World kinds are
handled, so this is latent, not active.

**Why existing gates miss it:** `event_kind_metadata_is_total` checks metadata only;
no test asserts every `physical_mutating` kind has an explicit arm.

**Required correction:** make the catch-all explicit — match the known non-world kinds
to `NonWorldNoOp` and return a typed error for anything else; or derive a totality
guard from `EventKind` metadata.

**Structural lock:** a guard asserting every kind whose metadata declares physical
mutation has an explicit apply arm (synthetic new-kind negative case).

### ORD-HARD-080 — `PhysicalState::set_need_model` is a public event-free mutator, and the state-visibility guards omit four authoritative families

**Severity:** medium. **Verification:** operator-verified (mutator + zero callers at
audit; the guard-list omissions confirmed at reassessment — both `guard_001` function
bodies contain zero mentions of the four families).

**Responsible layers:** `events` (mutation perimeter), `test_oracle`.

**Doctrine breached:** INV-009 (meaningful state changes require events) — a `pub`
escape hatch contradicting the capability discipline, callable from content/tui.

**Evidence:** `PhysicalState::set_need_model` (`state.rs`) is `pub fn (&mut self)`
rewriting `need_model` (checksummed, and the input to passive need-delta generation)
with no event and no capability token. Zero callers workspace-wide — a dead escape
hatch. `guard_001_authoritative_state_fields_are_not_publicly_mutable` and the
direct-insert guard check visibility/forbidden-write lists that omit `need_model`,
`sleep_affordances`, `need_tick_charges`, and the episode maps.

**Why existing gates miss it:** the guards enumerate field lists by hand; the lists
predate the newer families.

**Required correction:** delete `set_need_model` (or gate it behind the seed-token
path if content genuinely needs it); derive the guard lists from the same source-scan
that powers `checksum_coverage_is_total_for_authoritative_state` so a new family can
never be omitted.

**Structural lock:** the derived guard lists; a synthetic public-mutator negative
case.

### ORD-HARD-081 — The content loader silently drops colliding seeded intentions and routine executions

**Severity:** medium. **Verification:** operator-verified (collision mechanism
confirmed at reassessment: `strip_prefix("actor_")`, `routine_family_assignment_suffix`,
and the `entry().or_insert` discard sites verified at source).

**Responsible layers:** `content_seeding`.

**Doctrine breached:** INV-020 (silent repair — authored, gameplay-affecting state
vanishes with no rejection); INV-009.

**Evidence:** `FixtureSchema::to_agent_state` (`schema.rs`) synthesizes
intention/execution IDs as `intention_{actor_suffix}_{family_suffix}`, where the actor
suffix strips `actor_` and the family suffix collapses families. Two routine
assignments for the same actor and family (e.g. breakfast and dinner as two `EatMeal`
templates), or actors named `actor_mara` and `mara`, collide;
`intentions.entry(id).or_insert(...)` and the executions analog then silently discard
the later assignment. No validation phase rejects duplicate (actor, family)
assignments. Sub-point: `active_intention_by_actor.entry(...).or_insert_with(...)`
picks the canonically-first assignment as the active intention — a loader-invented
default neither authored nor rejected. Also in this family (fold in): the dead
`else { continue; }` on a missing template, unreachable behind the validation token
but silent-repair-shaped if ever reached.

**Why existing gates miss it:** `validate_references` checks actor/template existence
and tick order only; all current fixtures author one assignment per (actor, family).

**Required correction:** validate-and-reject duplicate (actor, family) assignments and
ambiguous actor-ID suffixes at `validate_references`; make the active-intention choice
explicit in the schema (an authored flag) or derive it by a documented rule; convert
the dead silent-skip to unreachable-by-construction or an error.

**Structural lock:** negative fixtures for the duplicate-assignment and
suffix-collision cases, asserting typed validation errors.

### ORD-HARD-082 — Embodied availability provenance carries the pipeline's unfiltered fact set, including the debug-only partition

**Severity:** medium. **Verification:** operator-verified.

**Responsible layers:** `projection`, action pipeline.

**Doctrine breached:** INV-067/INV-093 direction (structural leak channel: the
embodied view model carries the set doc 08 says must remain quarantined); no concrete
leak today — current checked facts are co-located/metadata.

**Evidence:** `with_validator_availability` (`projections.rs`) copies all
`report.checked_facts` into embodied `provenance_refs` as `ValidatorFact` entries. The
pipeline itself partitions facts (`is_actor_visible_fact` admits only
ActionId/ActorId/Reason/TickCount; ground-truth facts like `door_open` go to
`debug_only_facts`) — and the embodied path then ignores the partition. Any future
validator fact carrying hidden truth (true item location, true occupant) leaks
silently.

**Why existing gates miss it:** hidden-truth guards check `state.*` reads in
projection helpers, not report-fact flow.

**Required correction:** source embodied provenance from the actor-visible partition
(mirroring the render path's why-not choice).

**Structural lock:** a guard asserting `projections.rs` production code never reads
`.checked_facts`; a test seeding a debug-only validator fact and asserting it is
absent from embodied `provenance_refs`.

### ORD-HARD-083 — The embodied dead-surface family outlives the 0020 sweep: the sweep's universe excludes consumers, scalars, and enums

**Severity:** medium. **Verification:** operator-verified (core symbols:
`typed_leads`, `debug_available`, census skip behavior).

**Responsible layers:** `projection`, TUI, `test_oracle`.

**Doctrine breached:** INV-070/INV-071 — the fourth-plus dead embodied surface in
three passes; the `ORD-HARD-063` family lock covered producer-side `Option`/collection
fields only.

**Evidence:** (a) `NotebookView.typed_leads` is produced (`build_notebook_view`
populates staleness labels, `how_this_may_be_wrong`, `possible_next_actions`,
source/confidence — the doctrine-mandated lead anatomy) but `render_notebook` prints
only `possible_leads` bare summaries; no TUI file reads `typed_leads`. (b)
`EmbodiedViewModel.debug_available` is hardwired `true` at its sole producer with no
production consumer — a constant scalar outside the sweep's Option/collection
universe, and semantically wrong (availability should derive from debug
capability/binding). (c) the sweep's struct list is hand-enumerated and a listed name
whose `pub struct` marker is missing is silently skipped — `ActionAvailability` is an
enum, listed yet never checked, and its `debug_only_diagnostics` is always empty in
production. (d) `VisibleDoor.endpoint_a/endpoint_b` and `VisibleItem.source` have no
TUI consumer. (e) the "reachable producer" predicate is syntactic (field-name match,
any non-default initializer text, no struct scoping) — re-hardwiring
`visible_exit_blocker_summary` to return `None` would still pass the sweep. (f) the
`blocker_summary` perception gate is geometry-implied and vacuous; if visibility ever
becomes finer than co-location, the function reverts to truth leakage with the
omission test still green.

**Why existing gates miss it:** the sweep is producer-side, Option/collection-only,
hand-listed, silently-skipping, and unscoped by struct.

**Required correction:** render `typed_leads`; derive or delete `debug_available`;
populate or defer `debug_only_diagnostics` and the unconsumed fields with cites;
extend the sweep to (i) derive the struct list from the view-model source, (ii) fail
on unmatched listed names and handle enums, (iii) scope producer matches by struct,
(iv) add a consumer-side sweep requiring every embodied census field to appear in a
render/app arm or carry a cited deferral, (v) include constant-scalar producers in the
universe. Pin the `blocker_summary` co-location assumption with a guard or route door
state through a viewer-keyed perceived-door record.

**Structural lock:** the extended two-sided sweep with synthetic negatives for each
new universe dimension.

### ORD-HARD-084 — The generative fabricator ban scans one file and bans builder names only

**Severity:** medium. **Verification:** operator-verified (include list).

**Responsible layers:** `test_oracle` (generative tier).

**Doctrine breached:** lock durability (R-27 family); the 0020 changes added new
surface precisely in the unscanned `tests/support/generative.rs`.

**Evidence:** `generative_lock_cannot_fabricate_duration_terminals` bans the four
completion-builder symbols in `generative_lock.rs` only. A helper in
`support/generative.rs` synthesizing `EventEnvelope::new_v1(..., EventKind::WorkBlockFailed, ...)`
and called from the lock file under an innocuous name evades all banned tokens; so
would direct envelope construction in the lock file itself (`EventEnvelope::new_v1` is
importable; only builder names are banned).

**Why existing gates miss it:** the ban is a textual symbol list over one file.

**Required correction:** include `support/generative.rs` in the scan and ban
`EventEnvelope::new` plus terminal `EventKind::` constructor tokens in both test
sources (support legitimately needs neither — it builds proposals and seed state).

**Structural lock:** the widened two-file ban with a synthetic violating helper case.

### ORD-HARD-085 — Replay reject-loudly posture gaps: panics, silent skips, and silent defaults on log-derived data

**Severity:** medium. **Verification:** operator-verified (`parse().ok()` skip and
duplicate-charge `assert!` at their apply.rs sites at audit; the action-def
`payload_value` and scheduler completion `.expect` sites confirmed at reassessment).

**Responsible layers:** `replay`, `events`, actions kernel.

**Doctrine breached:** INV-020 (replay must reject unsupported history loudly — a
typed error, not a process abort and not a tolerated malformation); INV-092/098.

**Evidence:** (a) `assert_single_tick_delta_charge` (`events/apply.rs`) panics via
`assert!` on a duplicate (actor, need, tick) charge — replaying a double-charging log
aborts the process instead of producing a typed `ApplyError` (the `need_accounting`
pre-pass covers duplicate duration terminals only); and silently skips the whole guard
when `elapsed_ticks` is malformed (`.parse::<u64>().ok()` → early return), accepting
the malformation and omitting checksummed charge records. (b) silent payload defaults
repair missing history where the rest of the file uses `required(...)`:
`RoleAssignmentNoticeRecorded` missing `access_open` → `true` (invents a belief-level
access fact, INV-102-adjacent); `IntentionContinued` missing `current_step` →
`"continue_current_intention"`; `apply_routine_step_transition` missing
`progress_tick` → `"0"`. (c) log-sourced `.expect`s in `work.rs::payload_value` and
the scheduler completion paths abort on malformed payloads.

**Why existing gates miss it:** all tests feed well-formed logs; the panic and
default paths never fire.

**Required correction:** typed `ApplyError` variants for duplicate charges and
malformed `elapsed_ticks`; make the three defaulted fields `required(...)` (or
document them as schema-optional in the v1 payload contract — per-field decision, no
silent third option); convert log-sourced `.expect`s on the completion paths to typed
errors.

**Structural lock:** corrupt-history fixtures (duplicate charge, malformed
elapsed_ticks, missing required fields) asserting typed rejection live and on replay;
a review-checklist/guard rule against `assert!`/`expect` on event-log-derived data in
apply/completion paths.

### ORD-HARD-086 — Generative continuity evidence is reachable only through the advance path's flush divergence from production, and access-closure reasons are dead corpus branches

**Severity:** medium. **Verification:** operator-verified at reassessment (bare
per-tick loop with a single `append_due_completions` flush at the final tick; the
displacing Move is sequenced one step after Work with an inter-step gap never below
the workplace's `work_duration_ticks`, so it cannot strictly precede the scheduled
completion; `sleep_affordance_closed`/`workplace_unusable` accepted with no closing
mask in the corpus).

**Responsible layers:** `test_oracle` (generative tier), scheduler test-fidelity.

**Doctrine breached:** docs/2-execution/09 (evidence must be produced by the path
under test) — residual root of `ORD-HARD-060`: `advance_no_human` still flushes
completions once at the final tick while production `run_no_human_day` flushes per
window plus final.

**Evidence:** the generator places the displacing Move at or after the work block's
scheduled completion; under production per-window flushing the block would flush
before the move executes (and with Severe hunger would fail as
`severe_need_pressure`, not `actor_displaced`) — the displacement-during-block regime
the continuity floor counts is produced only by the deferred flush. Additionally,
`TerminalCounts`/`has_continuity_failure_terminal` accept
`sleep_affordance_closed`/`workplace_unusable`, but no mask ever closes an affordance,
so those arms are unfalsifiable decoration.

**Why existing gates miss it:** the floor only requires the reason string from the
advance path.

**Required correction:** schedule the displacing move strictly inside the block
(`requested_tick < work.requested_tick + work_duration_ticks`) and assert the
movement's `sim_tick` precedes the terminal's scheduled completion tick; give
`advance_no_human` per-tick or per-window flushing for production parity (preferred);
add a mask closing workplace access via the shared open/close action path or trim the
accepted-reason set to what the corpus asserts.

**Structural lock:** the in-block ordering assertion; per-reason continuity floors
matching the corpus's actual masks.

### ORD-HARD-087 — Five fixture contracts carry prose that contradicts their authored epistemic seed

**Severity:** medium. **Verification:** operator-verified
(`hidden_food_closed_container_001` setup text vs the wrapper seed).

**Responsible layers:** `content_seeding`, evidence honesty.

**Doctrine breached:** the lineage's "no simulation fact born from prose" inverted —
the contract surface asserts an epistemic state the fixture does not have; INV-063's
spirit (authored prehistory should be visible, not denied).

**Evidence:** `hidden_food_closed_container_001` (and its four wrapper siblings)
states "no observation or belief reveals the hidden food" while
`with_actor_mara_known_hidden_food` seeds exactly that belief
(`household_food_source` for `food_hidden_pantry`). Pre-existing under the old blanket
call; the 0020 ticket made the seeding explicit without reconciling the prose.
Related: four of the five wrapper consumers have purposes unrelated to the seeded
edge — the edge was kept solely to preserve behavior; minimization was not attempted.

**Why existing gates miss it:** contract validation checks no-human contracts only;
nothing cross-validates setup strings against authored sections.

**Required correction:** correct the setup/acceptance strings in the five fixtures to
name the authored belief; drop the wrapper from consumers where the seeded edge is not
load-bearing (verify per fixture).

**Structural lock:** a contract-vs-sections consistency check: any fixture with
non-empty `known_food_sources` must name each seeded knowledge edge in its setup
lines (or at minimum must not carry "no … belief" prose).

### Low findings

Each low finding retains the full remediation obligation; the compact format lists
evidence → correction → lock inline. All were operator-verified at source during the
reassessment pass at the same baseline.

**ORD-HARD-088 — Window completion-credit attribution diverges between the two
crediting sites** (`scheduler` / low). The mid-loop site credits the *sweeping* window
unconditionally, ignoring the returned completion tick; the final sweep credits every
window containing the tick — a completion processed late can credit a window the actor
did nothing in, masking a genuine `WindowNoProgress` (the inverse of `ORD-HARD-064`).
The sibling defect: `routine_window_family` applies the `deadline_tick` filter while
`active_routine_execution_for_actor` does not — two sites, one question, again.
Correction: one shared `credit_completion` helper keyed by `contains_tick`, one shared
eligible-execution iterator. Lock: a three-window test with a strictly-mid-window
completion; a deadline-expired-execution selection test.

**ORD-HARD-089 — `requires_cause` covers Phase-3A kinds only** (`events` / low;
INV-010). `ActorMoved`, `ActorWaited`, door/container/item/takeplace events are built
with empty `causes`, ancestry recoverable only via optional `proposal_id`. No deferral
cite exists in docs/2-execution/03/05/12. Correction: extend the allowlist to all
action-emitted kinds and switch builders to `new_caused_v1`. Lock: a guard deriving
the required-cause set from action-emitted kind metadata.

**ORD-HARD-090 — Typed decision diagnostics are constant defaults and legacy trace
shapes fabricate an empty context hash** (`agent_cognition` / low; INV-105, INV-020).
`DecisionTraceRecord::from_trace` always writes `TypedDiagnosticFields::decision_default`
(Failed traces still say blocker none — real reasons live only in unserialized
strings); `deserialize_canonical`'s 9/15-field legacy shapes substitute
`HolderKnownContextHash::from_canonical_lines(&[])` — a synthesized hash
indistinguishable from a genuinely empty context, brushing the no-shims convention if
those shapes exist in no retained logs. Correction: derive layer/blocker from the
trace outcome; type the absent hash as absent (`Option`); delete the legacy shapes if
no retained log carries them. Lock: a Failed-trace test asserting a non-default typed
blocker; round-trip tests for the absent-hash representation.

**ORD-HARD-091 — eat.rs failure reasons encode actor-unobservable distinctions in the
actor-facing slot** (actions kernel / low; latent INV-106). `access_failure`
distinguishes "carried by another actor" vs "carrier absent" — facts the actor cannot
observe — in the `reason` payload field (the actor-facing slot). No current consumer
routes `EatFailed.reason` into actor-known context, so latent. Correction: collapse to
one actor-observable reason; move the distinction to `absence_ancestry`. Lock: a guard
asserting failure-event actor-facing fields carry only actor-observable reason codes.

**ORD-HARD-092 — Generative tier minor locks** (`test_oracle` / low). (a) only the
first duration terminal per case is tampered and the floor is corpus-level ≥1 — drift
could collapse targeted-tamper coverage to one kind; accumulate tampered
`stable_id`s and assert all four kinds. (b) the synthetic-removal check
`assert_ne!(missing_one, expected)` is tautological; replace with a non-emptiness
assert and rely on the exact-equality lock. (c) `NeedBand::for_value(CONST as u16)`
wraps negatives; assert the constants' range first or band via `NeedState::band()`.

**ORD-HARD-093 — Known-food census closure residue** (`content_seeding` / low; lock
durability of `ORD-HARD-062`). (a) wrapper discovery is depth-1: a wrapper-calling-
wrapper chain inside an allowlisted file fails open; iterate discovery to a fixpoint
and add a depth-2 synthetic. (b) `populate_known_food_sources_for_all_actors` is `pub`
with census scope limited to `src/fixtures/*.rs`; add it to `clippy.toml`
`disallowed-methods` with `#[expect]` at allowlisted sites — compiler-enforced,
rationale-carrying (`clippy.toml` already carries a populated `disallowed-methods`
list with per-entry path + reason, so this is established repo convention, not new
machinery). (c) the 50-entry legacy allowlist carries one generic rationale;
per-entry rationale (or the clippy mechanism subsumes it).

**ORD-HARD-094 — The replay divergence diff omits two checksummed families**
(`replay` / low; INV-018 diagnostics). `diff_physical_state` and
`classify_state_diff_family` omit `sleep_affordances` and `need_model`; a divergence
there is detected by checksum but yields an empty `state_diff` — detected yet
unexplainable. Correction: add the two families and `ReplayDivergenceFieldFamily`
variants. Lock: derive the diff-family list from the checksum coverage array.

**ORD-HARD-095 — Bind-time perception events tension INV-087's letter** (TUI seam /
low; doctrine decision required). `bind_actor` appends perception events solely
because a human bound — INV-087 says possession "may not create events", while doc 07
arguably sanctions deriving the view from immediate perceptions, and the events are
ordinary NPC-possible perceptions via the modeled channel. Resolve explicitly: either
move the perception into the actor's own decision transaction, or record a
doctrine-clarification (a deliberate constitutional touch — requires explicit owner
sign-off; do not amend silently). This spec takes no position; the finding is the
unresolved tension.

**ORD-HARD-096 — The embodied "Knowledge context" line prints truth-derived engine
metadata unmarked** (TUI / low; INV-067/068 hygiene). The frontier is the global
event-log length including events the actor never witnessed; currently
non-exploitable (no concurrent autonomous activity during embodied play) but becomes
an off-screen-activity detector the moment interleaving lands. Correction: mark the
line `DEBUG NON-DIEGETIC` or scope the displayed frontier to the viewer's projection.
Lock: the embodied-render gate asserts no unmarked global-state-derived scalars.

**ORD-HARD-097 — The CI concurrency group can cancel scheduled mutation runs**
(CI / low). `ci-${{ github.workflow }}-${{ github.ref }}` with cancel-in-progress: a
push to main cancels an in-flight scheduled/dispatch mutation run. Correction: exempt
`schedule`/`workflow_dispatch` events from the cancel group. Lock: the perimeter
guard checks the concurrency expression.

**ORD-HARD-098 — Content-crate hygiene group** (`content_seeding` / low). (a)
`serialization.rs::source_id` writes Rust `Debug` output into the canonical byte
format for `SourceRef::Cause` (currently fail-closed behind seed validation; breaks
the moment a second source kind is admitted) — replace with a stable typed encoding.
(b) `validate.rs::has_explicit_no_sleep_diagnostic` keys a gate decision off free-text
diagnostic substrings (INV-105 spirit) — introduce a typed diagnostic kind. (c) folded
into `ORD-HARD-081`: the dead missing-template silent-skip.

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0020 layer.

1. **Compile-time impossibility:** demote the fabricating planning-context builders to
   test-gated visibility (`ORD-HARD-068`); `clippy.toml` `disallowed-methods` for the
   blanket food helper (`ORD-HARD-093`); delete `set_need_model` (`ORD-HARD-080`).
2. **Runtime gates:** typed `ApplyError` variants for duplicate charges, malformed
   `elapsed_ticks`, and the previously-defaulted payload fields (`ORD-HARD-085`);
   fail-closed world-applier totality (`ORD-HARD-079`); authoritative-state
   terminal-intention validation (`ORD-HARD-077`); viewer-scoped rejection reports in
   `build_embodied_view_model` (`ORD-HARD-066`); typed place concealment
   (`ORD-HARD-078`); duplicate-assignment rejection in the content validator
   (`ORD-HARD-081`).
3. **Test-oracle corrections:** the rebind-after-rejection possession gate
   (`ORD-HARD-066`); hidden-truth gate contexts rebuilt from real event logs
   (`ORD-HARD-068`); per-arm census anchoring with the two-arm synthetic
   (`ORD-HARD-069`); inverted write-shape scan and exemption-key derivation
   (`ORD-HARD-070`); behavioral per-kind policy dispatch tests driven from the table
   (`ORD-HARD-074`); the sleep mirror of the remembered-food test (`ORD-HARD-075`);
   corrupt-history typed-rejection fixtures (`ORD-HARD-085`); the in-block displacement
   ordering assertion and per-reason continuity floors (`ORD-HARD-086`); the extended
   two-sided dead-surface sweep (`ORD-HARD-083`); the shared-emitter crossing tests
   (`ORD-HARD-076`); the three-window completion-credit test (`ORD-HARD-088`);
   per-kind tamper coverage (`ORD-HARD-092`); depth-2 wrapper fixpoint synthetic
   (`ORD-HARD-093`).
4. **Source guards:** no public `ActorKnownPlanningContext` producer outside the
   builder/classifier pair (`ORD-HARD-068`); `exclude_globs`-vs-perimeter intersection
   check, job-scoped filter anchoring, regex-vs-canary evaluation (`ORD-HARD-071`);
   mutation-claiming-rationale-implies-perimeter rule (`ORD-HARD-072`); closed
   disposition-tag enum with repetition bounds (`ORD-HARD-073`); derived
   state-visibility guard lists (`ORD-HARD-080`); two-file fabricator ban with
   envelope-constructor tokens (`ORD-HARD-084`); guard_014 family extension to
   perception (`ORD-HARD-078`); checksum-derived divergence-diff family list
   (`ORD-HARD-094`); contract-vs-sections consistency check (`ORD-HARD-087`).
5. **Mutation/CI posture:** scheduled-job status capture making the ratchet live
   (`ORD-HARD-067`); honest rationale split and perimeter decision for the three
   large defs (`ORD-HARD-072`); the real baseline triage with follow-up test tickets
   (`ORD-HARD-073`); concurrency-group exemption (`ORD-HARD-097`).
6. **Evidence honesty:** fixture-contract prose reconciliation (`ORD-HARD-087`); none
   otherwise owed — the evidence documents verified clean for the second consecutive
   pass.

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: add a Watch risk — **Guard Vacuity /
  Decorative Lock** — naming the pattern this pass surfaced five times: a structural
  lock that asserts the artifact (table contents, filter presence, pinned hash, ledger
  shape) rather than the behavior, passing green while enforcing little or nothing
  (`ORD-HARD-067`/`069`/`070`/`073`/`074`). Guardrail: every lock ships with a
  synthetic negative that proves the lock can fire on the *behavior*, per enforcement
  dimension. Escalation trigger: any new guard whose only assertions are
  presence/shape/count checks over a hand-maintained artifact.
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  update the `0020 derived apply-arm payload census` row once per-arm anchoring is
  real (`ORD-HARD-069`); update the mutation rows for the live scheduled ratchet,
  exclusion-channel guard, and disposition-tag governance
  (`ORD-HARD-067`/`071`/`073`); add rows for the possession rebind-rejection gate
  (`ORD-HARD-066`), the harness-provenance fidelity contract (`ORD-HARD-068`), the
  behavioral per-kind policy dispatch (`ORD-HARD-074`), the typed place-concealment
  attribute (`ORD-HARD-078`), and the two-sided dead-surface sweep (`ORD-HARD-083`).
- No doctrine amendment; INV-001…INV-110 are applied, not changed. `ORD-HARD-095`
  flags an INV-087 tension for an explicit owner decision; if the resolution chosen is
  a doctrine clarification, that is a separate, deliberately-flagged constitutional
  change outside this spec.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0021_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The rebind-after-rejection proof: the possession gate output and the
   mismatched-actor report unit test (`ORD-HARD-066`).
2. The scheduled mutation job's first green-with-live-ratchet run (or its honest red
   with newly caught misses), plus the widened guard outputs (`ORD-HARD-067`/`071`).
3. The hidden-truth gate rebuild: confirmation that no gate context is built through
   the fabricating builder, and the visibility demotion (`ORD-HARD-068`).
4. The per-arm census proof: the two-arm synthetic failing, then the per-arm
   dispositions with the corrected anchoring (`ORD-HARD-069`/`070`).
5. The honest rationale split and the perimeter decision for
   takeplace/wait/continue_routine, with refreshed baseline if widened
   (`ORD-HARD-072`).
6. The real baseline triage: per-entry dispositions under the closed tag enum, the
   follow-up test tickets filed, entries retired (`ORD-HARD-073`).
7. The behavioral policy-table dispatch proof and the sleep-accessibility resolution
   (or registered deferral) (`ORD-HARD-074`/`075`).
8. The shared need-delta+crossing emitter proof and the corrupt-history
   typed-rejection fixture outputs (`ORD-HARD-076`/`085`).
9. The typed place-concealment migration and the perception guard extension
   (`ORD-HARD-078`).
10. The world-applier totality guard, the deleted/gated mutator, and the derived
    visibility lists (`ORD-HARD-079`/`080`).
11. The content-validator duplicate-rejection negative fixtures and the
    contract-prose reconciliation diff (`ORD-HARD-081`/`087`).
12. The embodied provenance partition proof and the extended dead-surface sweep
    output, including the typed_leads render and the `debug_available` resolution
    (`ORD-HARD-082`/`083`).
13. The generative tier deltas: two-file fabricator ban, in-block displacement
    ordering, per-kind tamper coverage (`ORD-HARD-084`/`086`/`092`).
14. The low-group closures or registered deferrals with cites
    (`ORD-HARD-088`–`098`), including the explicit INV-087 decision record for
    `ORD-HARD-095`.
15. The risk-register and conformance-index diffs, quoted (§6).
16. An updated `EMERGE-OBS` ledger derivation over the corrected surface (measurement
    only, no thresholds).
17. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`; not
    full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment (the `ORD-HARD-095` decision, if it touches doctrine, is a
  separately-approved change).
- Crate direction preserved: core depends on nothing at runtime; content on core; tui
  on core + content. No new dependencies, dev or production.
- Every finding's evidence is operator-verified at source (the audit pass plus a
  reassessment pass at the same baseline; zero claims refuted); no re-verification
  step is owed. A finding whose premise nonetheless fails at implementation time is
  recorded as refuted in the acceptance artifact, not silently dropped.
- Recommended ticket ordering:
  1. `ORD-HARD-066` + `082` + `096` (possession/embodied epistemic hygiene — small,
     high-value, one surface).
  2. `ORD-HARD-068` (harness provenance + gate rebuild; touches many gate tests, do it
     early before other tickets add gates on the old harness).
  3. `ORD-HARD-069` + `070` (census mechanics, one guard file).
  4. `ORD-HARD-067` + `071` + `072` + `073` + `097` (the mutation-CI family — one
     honest baseline refresh after the rationale split and any perimeter widening).
  5. `ORD-HARD-074` + `075` (policy-table behavioral dispatch; batch any embodied
     golden churn with the per-actor ledger treatment).
  6. `ORD-HARD-076` + `077` + `085` (kernel event/validation posture; batch golden
     repricing once if crossing events reprice canonical days).
  7. `ORD-HARD-078` + `079` + `080` + `089` + `094` (events/perception/state
     perimeter).
  8. `ORD-HARD-081` + `087` + `098` (content crate).
  9. `ORD-HARD-083` (dead-surface sweep extension + surfacing work).
  10. `ORD-HARD-084` + `086` + `092` (generative tier).
  11. `ORD-HARD-088` + `090` + `091` + `093` + `095` (remainder; `095` lands as a
      decision record).
  Documentation corrections (§6) land with group 1. The acceptance artifact lands
  last, measuring the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-068`'s gate rebuild is the riskiest ticket.** Rebuilding
  `hidden_truth_gates.rs` contexts from real event logs may surface gates that only
  passed *because* of the fabricated provenance — treat any newly failing gate as a
  potential latent product defect, not a test problem, and triage it under the
  Enforcement reading before adjusting the test.
- **`ORD-HARD-072`'s perimeter decision has real cost.** Bringing
  takeplace/wait/continue_routine into the perimeter likely adds a meaningful batch of
  honest baseline entries and lengthens the scheduled run; exempting them requires a
  rationale that is actually true. Either is acceptable; the silent third option is
  not.
- **`ORD-HARD-073`'s triage may surface real test debt** — the 0020 pass already
  warned of this and it did not happen; budget for follow-up test tickets this time.
- **`ORD-HARD-076`'s crossing events may reprice golden logs.** Crossings emitted
  during sleep/work blocks add events to canonical days; batch the repricing once with
  the per-actor ledger treatment.
- **`ORD-HARD-086`'s preferred fix (per-window flushing in `advance_no_human`)
  changes the generative corpus's event sequences** — every floor and contributor
  assertion must be re-derived honestly, not adjusted to pass.
- **`ORD-HARD-095` requires an owner decision** on INV-087's letter vs. doc 07's
  practice. The spec deliberately does not pre-decide it.
- **Next-iteration assessment (the recurring question):** findings were found, so an
  eleventh pass is warranted by the lineage's own rule. The honest trend: zero
  blockers for four consecutive passes; the kernel clean for four; the evidence
  documents clean for two; and the fresh-sweep frontier is now exhausted — all three
  crates have been deep-audited. This pass's highs came from previously-unaudited
  seams (the TUI host seam, CI job semantics, the gate harness itself) plus the
  guard-vacuity pattern, not from kernel rot. The eleventh pass should be a **narrow
  verification-only audit of 0021's deliverables** under the R-28 family-closure rule
  and the new guard-vacuity rule (does every new lock's synthetic negative actually
  fire?). If it returns clean or low-only, the lineage's standing recommendation to
  drop per-pass cadence and move to per-phase-entry audits (Phase 3B / Phase 4
  boundaries) is, for the first time, well-supported.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability) and names responsible layers from the execution diagnostic
  vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding was independently operator-verified at its
  load-bearing symbols (the delegated-evidence remainder was operator-verified at the
  reassessment pass at the same baseline; zero claims refuted).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014–0020 are recorded to prevent re-litigation; the
  second consecutive clean evidence pass is recorded; the guard-vacuity pattern is
  named and given a risk-register entry.
- [x] Severity calibrations that diverge from lineage precedent are named in §4's
  preamble (`ORD-HARD-067`, `ORD-HARD-068`).
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved. The single doctrine tension found (`ORD-HARD-095`) is flagged
  for explicit owner decision, not designed against.
- [x] Scope stays within the Phase 3A ordinary-life surface and its lock/evidence
  layer.

## Outcome

Completed: 2026-06-11

Spec 0021 landed as thirteen ticket commits, ending with the scoped acceptance
artifact commit `a466d13`. The implementation closed the possession-rebind,
harness-provenance, census, mutation-CI, policy-dispatch, reject-loudly,
typed-place-visibility, event/state-perimeter, content-integrity, embodied-sweep,
generative-parity, and low-severity remainder findings recorded by this spec.

The final artifact is `reports/0021_ord_life_cert_scoped_acceptance.md`; it records
the implementation commit manifest, the §7 evidence map, the refreshed observer-only
EMERGE-OBS table, the deferred INV-087 owner-decision boundary, and the explicit
non-certification posture. No doctrine amendment was made.

Deviations from the original plan: `ORD-HARD-095` landed as an explicit deferred
owner-decision record rather than a constitutional edit, and the capstone made no
production or test-oracle changes.

Verification passed on 2026-06-11:

```sh
cargo test -p tracewake-core --test emergence_ledger -- --nocapture
rg -n "EmergeObs|emerge_obs|emergence_ledger" crates/tracewake-core/src crates/tracewake-content/src crates/tracewake-tui/src
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```
