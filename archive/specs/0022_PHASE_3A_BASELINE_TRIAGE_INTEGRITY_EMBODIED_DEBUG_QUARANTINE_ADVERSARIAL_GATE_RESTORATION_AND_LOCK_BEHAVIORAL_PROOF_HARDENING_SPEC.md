# 0022 Phase 3A Baseline-Triage Integrity, Embodied Debug Quarantine, Adversarial-Gate Restoration, and Lock-Behavioral-Proof Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `9ce820f` (merge PR #29: post-0021 closeout with all `0021PHA3APOSREB` tickets landed — including the post-audit `-014` mutation-gate closure — and the acceptance artifact `reports/0021_ord_life_cert_scoped_acceptance.md` truthed at `78c0973`). The audit ran at pre-merge `ad39160`; every finding was re-verified at `9ce820f` — the `-014` delta is test-only (four mutation-killing inline tests in `scheduler.rs`, `agent/no_human_surface.rs`, and `actions/defs/wait.rs`; no production, ledger, or baseline-file changes) and refutes no finding.
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the eleventh Phase 3A alignment pass — the narrow verification-only audit
of 0021's deliverables that 0021 §9 itself prescribed, conducted under the R-28
family-closure rule and the R-29 guard-vacuity rule ("does every new lock's synthetic
negative actually fire on the behavior?"). Spec 0014 closed `ORD-HARD-001`–`007`; 0015
closed `008`–`013`; 0016 closed `014`–`025`; 0017 closed `026`–`034`; 0018 closed
`035`–`043`; 0019 closed `044`–`052`; 0020 closed `053`–`065`; 0021 closed
`066`–`098`. This audit re-derived the normative contract from `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`, and
re-examined every 0021 correction at the post-0021 baseline, treating each as
unverified until proven in code, per the lineage rule. It additionally verified the
0005 feature contract end-to-end (the original needs/routines/no-human-day surface)
against HEAD. Findings continue the `ORD-HARD` series at `099`.

All evidence below was verified against local sources at the target baseline.
Citations use named symbols, which are grep-stable; line numbers are omitted
deliberately. The audit was conducted as a nine-slice delegated review (possession/TUI
seam, harness provenance, census/state perimeter, mutation CI, policy/supersession,
kernel event posture, perception/content, generative tier, evidence honesty) plus a
0005 feature-contract conformance slice and an external lock-design research pass.
Every high and medium finding was independently operator-verified at its
load-bearing cited symbols during the audit; a reassessment pass at the post-merge
baseline `9ce820f` then operator-verified the nine delegated low findings at source
(eight confirmed; one evidence sub-claim refuted and reshaped — `ORD-HARD-120`), so
all twenty-two findings are operator-verified and no re-verification step is owed at
implementation time.

The root patterns this pass:

1. **The unperformed correction in completed costume.** The highest-severity finding
   (`ORD-HARD-099`) is that 0021's required baseline triage — the second time the
   lineage has required it — did not happen: the ticket commit reworded the ledger
   header, changed zero entries, retired zero baseline misses, filed none of the
   promised baseline test-debt tickets (the post-audit `-014` ticket closed new
   in-diff mutants, not the ledger triage), and the acceptance report's "Baseline
   Triage" section describes guard
   *capabilities* in place of the absent work. This breaks the evidence-document
   honesty streak at two passes and violates 0021 §8's own constraint that an unmet
   finding "is recorded as refuted in the acceptance artifact, not silently dropped."
2. **Locks that cannot fire on the behavior (R-29, again).** Several 0021 structural
   locks landed in artifact-asserting form: the policy-table dispatch test compares
   the table to itself (`ORD-HARD-106`), the planner-level hidden-truth gates dropped
   their adversarial-truth dimension during the rebuild (`ORD-HARD-105`), the shared
   crossing-emitter guard excludes the one remaining builder that applies hunger
   deltas (`ORD-HARD-107`), and the dead-surface sweep accepts a hardcoded `true` as
   a "reachable producer" (`ORD-HARD-104`).

Two notes that distinguish this pass from its predecessors:

- The kernel product-behavior core verified clean for the **fifth consecutive pass**:
  all production state writes capability-gated, all autonomous proposals through
  `ActorDecisionTransaction::run` from sealed actor-known context, possession parity
  and debug quarantine holding at the projection layer, replay reject-loudly posture
  delivered (`ORD-HARD-085`'s typed errors verified live and on replay). The 0005
  feature contract verified substantially intact after ten hardening passes — no
  delivered-feature regression was found.
- The fresh findings concentrate in the **test-oracle, CI, and evidence layers**, not
  in product cognition. The single product-behavior doctrine gap (`ORD-HARD-107`) is
  the residual family member of `ORD-HARD-076`, not a new defect class.

## 1. Scope

### In scope

- The 0021 baseline-triage obligation, the mutation-CI guard family, and the
  acceptance-report evidence map (`.github/workflows/ci.yml`, `.cargo/mutants.toml`,
  `.cargo/mutants-baseline-misses.txt`, `reports/0020_mutants_baseline_disposition.md`,
  `reports/0021_ord_life_cert_scoped_acceptance.md`,
  `crates/tracewake-core/tests/anti_regression_guards.rs`).
- The embodied render boundary and the dead-surface sweep
  (`crates/tracewake-tui/src/render.rs`, `src/app.rs`,
  `crates/tracewake-core/src/projections.rs`).
- The rebuilt hidden-truth gate harness (`crates/tracewake-core/tests/hidden_truth_gates.rs`)
  and the actor-known constructor guards.
- The actor-known policy table and its behavioral locks
  (`crates/tracewake-core/src/epistemics/projection.rs`, `src/agent/no_human_surface.rs`,
  `src/agent/perception.rs`).
- The shared need-delta/crossing emitter perimeter and reject-loudly remainder
  (`crates/tracewake-core/src/actions/defs/eat.rs`, `src/actions/defs/need_events.rs`,
  `src/scheduler.rs`, `src/events/apply.rs`).
- Census scan mechanics (consumed-key derivation, apply-arm totality, cause-set
  derivation) in `anti_regression_guards.rs`.
- Canonical no-human-day evidence provenance
  (`crates/tracewake-content/tests/golden_fixtures_run.rs`, fixture
  `no_human_day_001`).
- Content/generative hygiene remainder (`crates/tracewake-core/src/agent/routine.rs`
  typed-diagnostic vocabulary, `tests/support/generative.rs`,
  `tests/generative_lock.rs`).
- Meta-lock machinery for the R-29 pattern (lock↔negative bijection, nonzero-witness
  rule, two-sided ratchets) — new this pass, motivated by the external research below.

### Out of scope

- Re-auditing Phase 1 / 1A spine internals (0010–0012) and Phase 2A epistemic
  internals (0013), except where Phase 3A consumes them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0021 fixed that this audit verified as holding
  (see §3 "Verified holding").
- The `ORD-HARD-095` INV-087 owner decision (still deferred; its decision record
  verified genuine — see §3).

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-008/009/012/013,
  INV-018/020, INV-022, INV-067/068/070/071, INV-093, INV-099…INV-106, and the
  Enforcement reading ("tests must be corrected when they reward … actor proposals
  derived from raw physical state").
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` —
  acceptance tests must include adversarial truth present in authoritative state but
  absent from holder-known context (`ORD-HARD-105`).
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` and
  `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — a
  gate's evidence must be produced by the path under test (`ORD-HARD-102`/`111`).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-27 (acceptance-evidence
  reachability overstatement), R-28 (incomplete correction closure), R-29 (guard
  vacuity / decorative locks — added by 0021; this pass found four further instances
  and adds the meta-lock guardrails §5 prescribes).
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale — and a lock is real only if its negative case can actually fire on
  the behavior.

**External research consulted this pass** (extends the 0017/0018 lock-design corpus):
the pseudo-tested-methods / extreme-mutation literature (Vera-Pérez et al.,
Descartes), checked coverage (Schuler & Zeller, ICST 2011), rotten green tests
(Delplanque et al., ICSE 2019), vacuity-witness practice from model checking (a pass
is valid only with a witness that the antecedent fired), two-sided auto-tightening
ratchet practice (error-prone/NullAway, ESLint ratchet formatters), and
deterministic-simulation-testing oracle design (FoundationDB/TigerBeetle-style
replay-divergence oracles, metamorphic relations over event sequences). The §5
meta-locks (lock↔negative bijection census, nonzero-witness rule, two-sided ratchet)
are direct applications; all survive the zero-dependency constraint.

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not maximally locked.
Twenty-two findings follow, `ORD-HARD-099` … `ORD-HARD-120`: one high, twelve
medium, nine low, zero blockers. One finding is a product-behavior doctrine gap at
the letter of INV-009/012/013 (`ORD-HARD-107`, the unclosed `ORD-HARD-076` family
member); one is a high-severity violation of the lineage's own evidence contract
(`ORD-HARD-099`). No INV-099…INV-110 truth-firewall violation exists in production
cognition; no possession, telepathy, or prose-authority violation was found in
product behavior.

### Verified holding (no action; recorded so they are not re-litigated)

- **ORD-HARD-066 is substantive.** `TuiApp::bind_actor` clears `last_rejection`;
  `build_embodied_view_model` filters by
  `last_rejection.filter(|r| r.actor_id.as_ref() == Some(viewer_actor_id))`; both the
  rebind-after-rejection gate
  (`adversarial_gates_possession_rebind_does_not_transfer_rejection_why_not`) and the
  mismatched-actor unit test
  (`rejection_report_must_match_viewer_before_embodied_projection_renders_it`) fire on
  real behavior.
- **ORD-HARD-067/071/072 mechanics are substantive.** The scheduled job captures
  status (`set +e` / `mutants_status=$?` / `GITHUB_OUTPUT`), distinguishes exit 0/2
  from tool failure, and the `comm -23` ratchet step is reachable under
  `always() && … != 'skipped'`; `exclude_globs` is intersected per-entry against
  `MUTATION_PERIMETER_CANARY_PATHS`; filter checks anchored via `yaml_step_run_block`
  + `non_comment_lines`; the in-diff regex is evaluated per canary by
  `in_diff_filter_matches_path`; the rationale constant is split and the three large
  non-perimeter defs carry the honest non-perimeter rationale; the pinned count (143)
  and FNV-1a64 hash were independently recomputed and match. (The residue is
  `ORD-HARD-100`/`101`/`118`; the triage substance is `ORD-HARD-099`.)
- **ORD-HARD-068 is substantive in production.** `observe_visible_local` and both
  `build_actor_known_planning_state*` builders are deleted from production source;
  `from_observed_parts` is `pub(crate)` with compile-fail doctests; production
  cognition context is built solely via `NoHumanActorKnownSurfaceBuilder` over
  `classified_actor_known_records_for_context` with real `source_event_id`s;
  `workplace_assignment_active` is minted only from a real
  `RoleAssignmentNoticeRecorded` event; the fabricated `event_visible_local_planning_state`
  constant is retired and banned (`guard_0021_fabricated_visible_local_event_id_is_retired`).
  (The gate-side coverage regression is `ORD-HARD-105`.)
- **ORD-HARD-069/070 census mechanics are substantive.** `enclosing_apply_anchor`
  anchors inline sites per `EventKind::` arm and the two-arm synthetic genuinely
  fails; the write-shape scan is inverted with firing retain/rebind synthetics; the
  guard lists derive from `PHYSICAL_STATE_CHECKSUM_COVERAGE` ∪
  `AGENT_STATE_CHECKSUM_COVERAGE`. (The consumed-key derivation blind spot is
  `ORD-HARD-109`.)
- **ORD-HARD-074/075 product behavior is substantive.** `policy()` has production
  callers (`classified_actor_known_records_for_context`,
  `current_place_knowledge_context`, `consume_projection_record`);
  `CurrentPlaceLatestOnly` is deleted, replaced by consumed
  `ActorKnownProjectionEmbodiedScope` variants; `actor_knows_sleep_place` flows
  through `push_projection_fact(policy.freshness, …)`; sleep accessibility is
  `FromAnyPlace` with the proving mirror test
  (`sleep_record_from_other_place_surfaces_as_remembered_reachable_sleep_input`);
  supersession lives once. (The vacuous dispatch *lock* is `ORD-HARD-106`.)
- **ORD-HARD-076/077/085/088/090/091 kernel corrections are substantive.**
  `need_events::build_need_delta_and_threshold_events` is direction-agnostic
  (`delta.max(0)` gone, downward-crossing test real); sleep/work/wait/scheduler route
  through it; `validate_authoritative_intention` reads `active_intention_by_actor` +
  `intentions.status` with the parameter-omitted rejection test;
  `ApplyError::DuplicateNeedTickCharge`/`MalformedElapsedTicks` are typed with
  live-and-replay corrupt-history fixtures; the three payload defaults are
  `required(...)`; `credit_completion` keyed by `contains_tick` backs both crediting
  sites; typed decision diagnostics derive from trace outcome with absent hash typed
  `Option`. (The eat.rs perimeter gap is `ORD-HARD-107`; the residual `.expect`s are
  `ORD-HARD-108`.)
- **ORD-HARD-078/081/087/093/098(a) content corrections are substantive.**
  `is_visible_exit_target` gates on typed `VisibilityDefault` with the
  misleading-label proof
  (`visible_exit_perception_follows_typed_visibility_not_id_or_label_prose`); the
  loader fails closed on absent/unknown visibility; duplicate (actor, family)
  assignments and ambiguous suffixes are validator-rejected with the earliest-start
  active-intention rule enforced; the five fixture contracts name their seeded edge
  with firing denial/omission checks; wrapper discovery iterates to fixpoint with the
  clippy `disallowed-methods` entry; `stable_event_cause_id` uses typed prefixes, no
  Debug output. (The guard-scope residue is `ORD-HARD-110`; the diagnostic-vocabulary
  residue is `ORD-HARD-116`.)
- **ORD-HARD-079/080/094 state-perimeter corrections are substantive.** Both
  appliers fail closed (`ApplyError::UnhandledWorldEventKind`, `NonAgentEvent`);
  `set_need_model` is deleted; the divergence diff derives both checksummed families
  from `PHYSICAL_STATE_CHECKSUM_COVERAGE`. (The synthetic-quality and agent-stream
  asymmetry residue is `ORD-HARD-113`.)
- **ORD-HARD-082/083 embodied corrections are substantive at the producer layer.**
  Embodied provenance sources from `report.actor_visible_facts`; the
  `checked_facts` guard fires on real projections.rs; the two-sided sweep derives
  fields from `view_models.rs`, fails on unmatched names, handles the
  `ActionAvailability` enum, scopes by struct, and `typed_leads` is genuinely
  rendered (`render_notebook_prints_typed_lead_anatomy`);
  `VisibleDoor.endpoint_a/b` and `VisibleItem.source` are produced and consumed.
  (The render co-mingling is `ORD-HARD-103`; the constant-producer acceptance is
  `ORD-HARD-104`.)
- **ORD-HARD-084/086/092 generative corrections are substantive.** The fabricator
  ban scans both files with both synthetic violators firing; `advance_no_human`
  drains per-tick to production parity; the accepted continuity-reason set is trimmed
  to `actor_displaced`, which the work-completion path genuinely emits; per-kind
  tamper coverage asserts the exact four-kind set; the tautological `assert_ne!` is
  retired; the negative-wrap range guard precedes banding. Support constructs zero
  `EventEnvelope`s. (Minor lock-shape residue is `ORD-HARD-117`.)
- **Evidence documents verified mostly honest** — commit manifest exact (13 ticket
  commits `e1b94a1`–`a466d13`), EMERGE-OBS table reproduced byte-equivalent by
  re-running `emergence_ledger`, observer-only claim reproduced, all report-named test
  symbols exist, SPEC_LEDGER 0021 row accurate and per convention, R-29 risk entry
  landed with guardrail and escalation trigger, all nine §6 conformance-row actions
  diff-verified, the INV-087/ORD-HARD-095 deferral is a genuine recorded decision
  boundary. (The three honesty exceptions are `ORD-HARD-099`/`102`/`119`.)
- **The 0005 feature contract is substantially intact.** Needs/bands at exact §8.1
  boundaries; event-sourced decay/crossings replay-reconstructable; defeasible
  routines with validator-rejected "no failure modes"; HTN expansion from actor-known
  context only; `run_no_human_day` genuinely autonomous through the shared pipeline;
  canonical food-unavailable and routine-blocked proofs typed and refill-free;
  possession preserves intention/needs/routines; all five ordinary actions
  TUI-reachable with why-not; forbidden shortcut/teleport/quest fields rejected; no
  mechanic exists only in tests; no hardening lock narrowed product behavior below
  the 0005 contract. (The canonical-day evidence provenance gap is `ORD-HARD-111`;
  the recovery-variant gap is `ORD-HARD-120`.)

### Validated — no action (checked, found intentional or correctly scoped)

- `actor_elias` is absent from all fixtures — 0005 §10.4 marks him optional;
  compliant omission, not a gap.
- The self-referential synthetic beside the `checked_facts` guard is decorative but
  harmless — the load-bearing lock is the `assert_absent` against real
  projections.rs (folded into the `ORD-HARD-113`/§5 bijection treatment rather than
  carried as its own finding).
- Legacy 9/15-field decision-trace shapes still parse but reconstruct typed
  diagnostics from outcome and type the absent hash as `None` — no hidden-default
  leak.
- `guard_0021_hidden_truth_gates_use_event_log_provenance` and
  `guard_0021_actor_known_context_producers_are_projection_backed` carry working
  negative self-tests.
- The capstone commit `a466d13` made no production or test-oracle changes, exactly as
  the 0021 Outcome records; truthing commit `78c0973` improved report accuracy.
- `mutants.out` artifact handling, push-diff semantics (`HEAD^..HEAD`), ledger include
  path, and `normalized_mutant_misses` parsing all verified correct.

## 4. Findings and remediation requirements

Severity calibration notes: `ORD-HARD-099` is rated high where its direct precedent
(`ORD-HARD-073`, medium) was the same defect's first detection — the divergence is
the recurrence (the triage was promised by 0020, found absent by 0021, required by
0021, and skipped again) plus the acceptance-report masking, which puts it in the
R-27 harness-evidence family that the lineage rates high (`ORD-HARD-068` precedent).
Four findings are rated medium where the reporting audit slice proposed high:
`ORD-HARD-100` (precedent `ORD-HARD-097` low; elevated only to medium because the
hole intermittently vacates the now-live ratchet, not permanently — unlike
`ORD-HARD-067`); `ORD-HARD-103` (precedent `ORD-HARD-082`/`096`, both
structural-channel findings without live leak content); `ORD-HARD-106` and
`ORD-HARD-107` (precedents `ORD-HARD-074` and `ORD-HARD-076`, both medium; these are
their incomplete-closure residues, and the products' current behavior is correct).

### ORD-HARD-099 — The 0021 baseline triage did not happen: the required correction landed as a header reword, the acceptance report masks it, and the governance guard entrenches the deferral

**Severity:** high (calibration divergence from `ORD-HARD-073` named above).
**Verification:** operator-verified (git stats, ledger counts, prefix pin).

**Responsible layers:** `test_oracle`, CI, evidence honesty (substance and accuracy).

**Doctrine breached:** lock durability; R-28 (second recurrence of the identical
unperformed correction); R-29 (the hardened guard is itself the decorative shape it
polices); 0021 §8's own constraint that an unmet finding "is recorded as refuted in
the acceptance artifact, not silently dropped."

**Evidence:** commit `549e332` ("Complete 0021PHA3APOSREB-004") changed
`reports/0020_mutants_baseline_disposition.md` by 5 insertions / 3 deletions — header
only; zero of the 143 entry lines changed; `.cargo/mutants-baseline-misses.txt` is
untouched across the entire 0021 range; zero entries retired by writing tests. All
143 entries remain `justified-baseline`; 137 share the identical template "warrants a
future focused assertion before removing from the accepted baseline" (noun-phrase
variations yield ~76 distinct strings, max identical group 17, just under
`MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES = 20`). Zero `warrants-test` entries exist;
the later archived `archive/tickets/0021PHA3APOSREB-014.md` closes the PR-local
in-diff mutant gap but does not perform the baseline-ledger triage described here;
ticket-004 records no deviation and the acceptance report's "Deviations From Plan"
records none. Report §6
("Baseline Triage") asserts guard capabilities — "rejects … missing `warrants-test`
tickets" is vacuously true with zero such entries. Entrenchment:
`mutation_baseline_governance_errors` hard-pins
`ticket_id.starts_with("0021PHA3APOSREB-")`, so any future-series `warrants-test`
entry is structurally rejected. Entries that by their own wording warrant tests —
including `decide_proposal` comparison mutants and `controller_binding_check -> None`
in pipeline decision logic — sit under `justified-baseline`, escaping the ticket
requirement.

**Why existing gates miss it:** the repetition bound counts exact strings, so
noun-phrase-varied templates pass forever; ticket existence is checked only on
`warrants-test` tags, which have zero members; nothing inspects rationale semantics.

**Required correction:** perform the real triage once: entries in interruption
predicates and decision logic of guarded-layer files become
`warrants-test:<ticket-id>` with the follow-up test-debt tickets actually filed (new
series prefix), and focused tests retire entries from the baseline; the rest get
individually-true rationales. If, after honest triage, an entry class is genuinely
justified-baseline, its rationale must say why *that mutant* is acceptable — not that
a future assertion is warranted. Replace the series-prefix pin with `ticket_exists`
(any `tickets/` or `archive/tickets/` path). Amend the acceptance report (or its 0022
successor) to record the 0021 deviation explicitly.

**Structural lock:** a guard rule failing any `justified-baseline` rationale
containing deferral phrasing (`warrants a`, `future`, `follow-up`) — deferral intent
must carry a `warrants-test:<ticket-id>` tag; bound repetition on the rationale
*suffix* after stripping the leading noun phrase; a synthetic negative for each. Plus
the §5 two-sided ratchet: the baseline may only shrink or grow through a recorded
disposition delta, asserted by count comparison against the ledger's change log.

### ORD-HARD-100 — A push to main cancels an in-flight scheduled mutation baseline run

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble). **Verification:** operator-verified (ci.yml concurrency block).

**Responsible layers:** CI.

**Doctrine breached:** lock durability — the `ORD-HARD-097` correction covered the
scheduled run as the *incoming* event only; the `ORD-HARD-067` ratchet can be
operationally vacated by ordinary push traffic.

**Evidence:** `concurrency.group: ci-${{ github.workflow }}-${{ github.ref }}` is
shared between `push` to main and `schedule` (both `refs/heads/main`);
`cancel-in-progress: ${{ github.event_name != 'schedule' && github.event_name != 'workflow_dispatch' }}`
is evaluated on the *incoming* run, so an incoming push evaluates `true` and cancels
the in-progress multi-hour scheduled `mutants-lock-layer` run in the same group.

**Why existing gates miss it:** `mutation_perimeter_consistency_violations` checks
only whole-file textual presence of the exemption expression.

**Required correction:** include `github.event_name` in the concurrency group so
scheduled runs occupy their own group.

**Structural lock:** the perimeter guard parses the `concurrency:` block and asserts
`event_name` appears in `group`, with a synthetic negative removing it.

### ORD-HARD-101 — Mutation-guard bypass-channel residue: regex config keys unchecked, the glob matcher fails open, and capture/decoy checks remain text-shape-satisfiable

**Severity:** medium. **Verification:** operator-verified (absence of
`exclude_re`/`examine_re` handling, `simple_glob_matches` body, raw-file
`status_captures` count).

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability — the `ORD-HARD-071` exclusion-channel
correction covered `exclude_globs` only (R-28 instance/family gap, one config key
over).

**Evidence:** (a) cargo-mutants reads `.cargo/mutants.toml` in CI; adding
`exclude_re = [".*"]` (or a narrowing `examine_re`/`examine_globs`) empties the
perimeter with every guard green — `parse_exclude_globs` is the sole mutants.toml
inspection. (b) `simple_glob_matches` recognizes only exact, `/**`-suffix, and
`*`-suffix patterns and returns `false` otherwise — `"**/eat.rs"` excludes a canary
path while the intersection check sees no match (fails open). (c)
`status_captures = ci_yml.matches("mutants_status=$?").count()` counts the raw file
while invocations are counted over `non_comment_lines` — a comment containing the
capture pattern balances a gutted real step; the swallow rule flags only the literal
`|| true` (`cargo mutants … || echo ok` evades it); the in-diff filter check
evaluates the *first* matching line in the step block, shadowable by an uncommented
`echo` decoy carrying the canonical regex.

**Why existing gates miss it:** each check matches a text shape, not the executing
semantics; the synthetic negatives cover the shapes the checks already catch.

**Required correction:** fail closed on any unrecognized mutants.toml key from a
closed allowlist (`exclude_globs` only) and on any exclude glob containing `*` not in
the three recognized forms; count captures over `non_comment_lines` scoped to each
invocation's step block; flag any `||`/`&&` suffix on a `cargo mutants` line; anchor
the in-diff filter check to the line containing `git diff --name-only` (or assert
exactly one match).

**Structural lock:** synthetic negatives — an `exclude_re` key, a `**/eat.rs` glob, a
comment-only capture, an `|| echo ok` swallow, an uncommented decoy line — each must
fail the guard.

### ORD-HARD-102 — The acceptance report's §7-item-2 evidence is absent and its §2 heading wears that item's costume over unrelated content

**Severity:** medium. **Verification:** operator-verified (§2 heading vs. primary
ticket).

**Responsible layers:** evidence honesty.

**Doctrine breached:** R-27 (reachability/coverage overstatement);
docs/2-execution/10 (review artifacts must record what was actually run).

**Evidence:** 0021 §7 item 2 requires "the scheduled mutation job's first
green-with-live-ratchet run (or its honest red with newly caught misses)". No
scheduled-run output is recorded anywhere; the only run evidence is a local focused
`cargo mutants … eat.rs` invocation. Report §2 is titled "Scheduled Ratchet And Guard
Outputs" but its body covers ticket-002's hidden-truth provenance guards (its own
text: "The scheduled ratchet is represented by the widened … source guards").

**Why existing gates miss it:** nothing machine-checks the report's section claims
against the spec's §7 item list.

**Required correction:** record the first real scheduled-run result (or an explicit
"first run pending post-merge" deviation) and retitle §2.

**Structural lock:** an acceptance-artifact checklist guard mapping spec §7 item IDs
to required evidence strings in the report (extends to 0022's own artifact).

### ORD-HARD-103 — The embodied render co-mingles debug surfaces into the actor-facing string; the marker test locks the leak in rather than out

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble). **Verification:** operator-verified (`render_embodied_view` body).

**Responsible layers:** TUI.

**Doctrine breached:** INV-067/068 (debug must be visibly non-diegetic and never
confused with embodied knowledge — a substring label on a shared surface is a label,
not a quarantine); INV-008.

**Evidence:** `render_embodied_view` (`crates/tracewake-tui/src/render.rs`)
unconditionally appends `Debug: available={}` (`view.debug_available`), the
`debug_only_diagnostics()` join, and the
`Knowledge context: DEBUG NON-DIEGETIC … frontier=…` line (global event-log frontier)
into the same string an embodied actor reads. No mode gate exists. The 0021
`ORD-HARD-096` correction chose the marker option; the marker is present, but the
proving test (`renderer_marks_knowledge_context_frontier_non_diegetic`) asserts the
marked line is *present* — there is no test that a pure-embodied render *omits* the
debug tokens, so the co-mingling is structurally locked green.

**Why existing gates miss it:** the renderer gate family asserts marker presence and
why-not separation, never debug-token absence from an embodied-mode render.

**Required correction:** split the debug overlay out of `render_embodied_view` (an
explicit debug flag or a separate `render_debug_overlay` consumed only by the debug
path), so the embodied string carries no `Debug:`/`debug_diagnostics`/global-frontier
lines.

**Structural lock:** an embodied-render negative test `assert_absent`-ing the debug
tokens from the embodied-mode render, alongside the existing debug-mode presence
test.

### ORD-HARD-104 — `debug_available` is a hardcoded production literal with a misdescribing census rationale, and the sweep accepts constants and declaration snippets as "reachable producers"

**Severity:** medium. **Verification:** operator-verified (the `app.rs` literal and
`projections.rs` `false` default).

**Responsible layers:** TUI, `test_oracle`.

**Doctrine breached:** lock durability (R-29) — the `ORD-HARD-083` "derived or
deleted" requirement is satisfied by neither; INV-068 direction.

**Evidence:** `app.rs` sets `view.debug_available = true;` unconditionally
(production); `projections.rs` initializes it `false`; the
`EMBODIED_SURFACE_FIELD_PRODUCERS` rationale claims it is "derived from the presence
of debug panel wiring" — false. The sweep's
`source_has_non_default_struct_field_producer` accepts `= true` as a non-default
producer, so any constant satisfies the reachable-producer requirement; separately,
the `debug_only_diagnostics` deferral is registered with a `producer_snippet` that
matches the *field declaration* in `view_models.rs`, so the entry stays satisfied
even if the field is orphaned.

**Why existing gates miss it:** the sweep's producer predicate is syntactic
(non-default initializer text) and deferral entries alias declarations.

**Required correction:** derive `debug_available` from actual debug capability (or
delete the field and let the TUI own the mode decision); correct the rationale;
exclude constant-literal initializers from the producer predicate; give deferrals a
cite-only exemption path with no declaration-aliasing snippet.

**Structural lock:** a synthetic constant-producer negative that must fail the
sweep; the §5 nonzero-witness rule applied to the sweep's per-entry matches.

### ORD-HARD-105 — The rebuilt planner hidden-truth gates dropped their adversarial-truth dimension: two gates now prove "empty context rejects" instead of "present truth stays hidden"

**Severity:** medium. **Verification:** operator-verified (underscore-bound
`_known_edges`/`_known_food_sources` params).

**Responsible layers:** `test_oracle` (truth-firewall acceptance).

**Doctrine breached:** docs/1-architecture/03 ("acceptance tests must include
adversarial truth present in authoritative state but absent from holder-known
context"); R-27 family — the gates' names promise more than they assert; the exact
risk 0021 §9 flagged for this rebuild.

**Evidence:** `hidden_truth_gates.rs::context` declares parameters `_known_edges` and
`_known_food_sources` and ignores both — only workplace knowledge is seeded (via
`role_notice_event` + `apply_epistemic_event`). The planner gates
`hidden_food_closed_container_is_not_actor_known_food_source` and
`hidden_route_edge_absent_from_actor_known_edges_blocks_route_plan` therefore build
contexts with *no* food/route knowledge at all and assert the planner rejects —
trivially true with or without the firewall. The pre-rebuild harness (for all its
fabrication defects) exercised the discrimination between known and hidden entries.
The embodied gates retain real adversarial truth (hidden food in a closed container
present in `PhysicalState`, absent from the sealed context) — the regression is
planner-side only.

**Why existing gates miss it:** `guard_0021_hidden_truth_gates_use_event_log_provenance`
checks banned tokens and `apply_epistemic_event` presence; it cannot see that a
parameter went dead.

**Required correction:** seed real fixture food/route knowledge through event-log
provenance in `context()` (the pattern the embodied gates already use), so each
planner gate runs against authoritative truth that exists, is partially known, and
must stay hidden where unknown; delete the dead parameters if a gate genuinely needs
no seeded knowledge.

**Structural lock:** a harness self-check that every `context()` caller passing a
non-empty adversarial fixture has that fixture's hidden entries present in
`PhysicalState` and absent from the built context (witness rule: assert the
discrimination actually occurred, not just the rejection).

### ORD-HARD-106 — The policy-table dispatch lock is a self-echo: both sides of the assertion read the same table, and the workplace embodied-scope negative cannot fire from any test

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble). **Verification:** operator-verified (`policy()` delegates to
`actor_known_projection_kind_policy`, the same map the test compares against).

**Responsible layers:** `holder_known_context`, `test_oracle`.

**Doctrine breached:** lock durability (R-29) — 0021's `ORD-HARD-074` structural lock
required "a behavioral test per record kind driven *from* the table"; what landed
asserts the table against itself.

**Evidence:** `actor_known_projection_records_dispatch_to_declared_policy_table`
asserts `record.policy().classification() == policies[kind].classification()` — but
`ActorKnownProjectionRecord::policy()` is implemented as
`actor_known_projection_kind_policy(kind)`, which reads
`actor_known_projection_policy_kinds()`, the identical map: the assertion cannot fail
for any table contents. `actor_known_projection_policy_table_declares_every_record_kind`
likewise asserts hardcoded literals against the same accessor. The real behavioral
coverage is hand-written per scenario; nothing iterates the kinds asserting observed
behavior per declared policy on both surfaces. Concretely unfireable negative:
workplace's `ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly` gate — every
embodied workplace test co-locates the actor with the workplace, so flipping the
scope or removing the gate passes the suite.

**Why existing gates miss it:** the production-caller guard proves `.policy()` is
called, not that behavior is bound to the declaration.

**Required correction:** a table-driven behavioral test: for each kind in
`actor_known_projection_policy_kinds()`, build a projection exercising that kind and
assert the no-human and embodied surfaces reflect the declared
`classification`/`embodied_scope`/`accessibility_scope`; add the workplace
other-place negative (actor at B, workplace at A: embodied drops, no-human retains).

**Structural lock:** the parametric test itself, plus a synthetic negative mutating
one arm's behavior away from its declared policy that must fail.

### ORD-HARD-107 — Eating applies a band-changing hunger delta with no paired crossing event, and the shared-emitter guard's perimeter excludes eat.rs

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble). **Verification:** operator-verified (`hunger_delta_event` in
eat.rs; `GUARDED_PATHS` omits eat.rs).

**Responsible layers:** actions kernel, `events`, `test_oracle`.

**Doctrine breached:** INV-009/012/013 at the letter (a band crossing that affects
later reasoning leaves no event when caused by eating); R-28 — the `ORD-HARD-076`
correction's own contract ("no builder can apply a band-changing delta without the
paired crossing event") fails on the one surface left outside the perimeter.

**Evidence:** `build_eat_events` constructs `EventKind::NeedDeltaApplied` via its
private `hunger_delta_event`, not via
`need_events::build_need_delta_and_threshold_events`; a serving reduction (e.g. 120)
across a band boundary (510→390, Urgent→Rising) is a real crossing with no
`NeedThresholdCrossed`. The source guard's `GUARDED_PATHS` lists wait/sleep/work/
scheduler only — eat.rs is structurally exempt from the "single shared emitter" lock.
The eat unit test asserts exactly two events, baking the omission into the suite.

**Why existing gates miss it:** the guard's path list is the perimeter, and eat.rs
is not on it; replay determinism shares the omission.

**Required correction:** route eat's hunger delta through
`build_need_delta_and_threshold_events` (it has the actor's current hunger available
to the builder) and add eat.rs to `GUARDED_PATHS`; re-derive any golden logs the new
crossing events reprice honestly.

**Structural lock:** an eat-across-boundary test asserting the paired crossing both
directions; the guard derives its path list from the set of action defs that
construct `NeedDeltaApplied` (so a future def cannot be born outside the perimeter).

### ORD-HARD-108 — Residual `.expect`/`assert!` on event-log-derived data in scheduler completion paths; the `ORD-HARD-085` review-checklist guard was never implemented

**Severity:** medium. **Verification:** operator-verified (the
`open_body_exclusive_starts(...).expect(...)` site).

**Responsible layers:** scheduler, `replay`, `test_oracle`.

**Doctrine breached:** INV-020 (reject loudly: typed error, not process abort); R-28
— `ORD-HARD-085`'s correction converted the apply-side and action-def sites but its
structural-lock clause ("a review-checklist/guard rule against `assert!`/`expect` on
event-log-derived data in apply/completion paths") does not exist.

**Evidence:** `actor_has_open_body_exclusive_duration` (`scheduler.rs`) calls
`open_body_exclusive_starts(log, …).expect("duplicate duration terminals are rejected
before no-human scheduling")` — the callee returns `Err(DuplicateDurationTerminal)`
derived purely from log contents, and the invariant the message cites is enforced at
apply time, not before the scheduler reads a (possibly corrupt) log. Further
log-sourced `.expect`s sit on the stuck-diagnostic completion paths
(`apply_agent_event(...).expect(...)`). No guard polices the pattern.

**Why existing gates miss it:** all tests feed well-formed logs; the promised guard
is absent.

**Required correction:** propagate typed errors from the scheduler's log reads
(`DuplicateDurationTerminal`, `ApplyError`) instead of panicking; implement the
promised source guard over apply/completion/scheduler paths with an allowlist for
genuinely unreachable states (each with rationale).

**Structural lock:** the source guard with a synthetic violating case; corrupt-log
scheduler-path fixtures asserting typed rejection.

### ORD-HARD-109 — The consumed-payload-key derivation recurses only into helpers called as `(payload`, missing `&payload` and non-first-argument call shapes

**Severity:** medium. **Verification:** operator-verified (the `"(payload"` literal
scan).

**Responsible layers:** `test_oracle`, `replay`.

**Doctrine breached:** lock durability — the `ORD-HARD-070` exemption-truth
derivation is leaky by call shape (R-28 closure residue).

**Evidence:** `payload_helper_calls` finds callees via the literal substring
`(payload`; calls shaped `helper(state, event, payload, …)` (comma-preceded) or
`helper(&payload)` are invisible, so keys consumed by such helpers (e.g.
`assert_single_tick_delta_charge`'s reads under `apply_need_delta`) are not
attributed to the exemption's `typed_columns` subset check. No current false pass
exists (the relevant keys are covered by a separate anchor), but a future unlisted
key consumed through a comma-preceded helper slips silently.

**Why existing gates miss it:** the synthetic negative consumes its unlisted key via
a direct `required(payload, …)` in the anchor body, never through an oblique call
shape.

**Required correction:** resolve callees by parsing argument lists (split on
`(`/`,`, strip `&`) rather than a substring; recurse on any argument binding the
payload identifier.

**Structural lock:** a synthetic negative whose unlisted key is consumed by a helper
called as `helper(state, &payload)` — must fail.

### ORD-HARD-110 — The perception prose-branch guard scopes to one function, not the emission paths

**Severity:** medium. **Verification:** operator-verified
(`body_after_marker(&stripped, "fn is_visible_exit_target")`).

**Responsible layers:** `test_oracle`, perception channel.

**Doctrine breached:** lock durability — `ORD-HARD-078`'s structural lock promised
"no `display_label`/id-substring branching in any emission path"; the delivered
guard scans only `is_visible_exit_target`.

**Evidence:** `perception_visibility_prose_branch_violations` runs over
`body_after_marker(…, "fn is_visible_exit_target")`; `current_place_perception_events`,
`observation_event`, and the food/sleep gating loops are outside the window. Today
those gate on typed state (verified clean), so the gap is latent.

**Why existing gates miss it:** the synthetic negative feeds a fragment anchored at
the same marker, proving the gate fires for that one function only.

**Required correction:** widen the scan to the whole production module (allowlisting
typed-label payload *writes*, which legitimately carry display text into payloads).

**Structural lock:** a synthetic prose branch inside a different emission function —
must fail.

### ORD-HARD-111 — The canonical no-human-day completion evidence is hand-built: the runner is never required to close a work or sleep block on `no_human_day_001`

**Severity:** medium. **Verification:** operator-verified (the
`proposal_day_tomas_work` injection in
`crates/tracewake-content/tests/golden_fixtures_run.rs`).

**Responsible layers:** `test_oracle`, content fixtures.

**Doctrine breached:** docs/2-execution/09 (evidence produced by the path under
test) — the `ORD-HARD-060`/`086` family shape at the canonical-fixture layer; 0005
§10.4's "at least one actor completes or fails a work block" is proven on the
canonical day only after manual proposal injection.

**Evidence:** `no_human_day_fixture_has_roster_activity_and_metrics_envelope` runs
`run_no_human_day`, then injects hand-built proposals (`proposal_day_tomas_work`,
the Elena sleep analog) and calls the completion builders directly before asserting
`WorkBlockCompleted`/`SleepCompleted`/`FoodConsumed` presence — the assertions pass
whether or not the runner produced them autonomously. Autonomous duration completion
is proven only on synthetic worlds (`no_human_day_counts_duration_completion_as_window_progress`,
the capstone).

**Why existing gates miss it:** the merged-log assertion has no provenance
requirement (`SchedulePhase::NoHumanProcess` ancestry is never demanded for the
canonical day's completions).

**Required correction:** either author `no_human_day_001`'s windows/durations so the
runner closes at least one work and one sleep block unaided, with runner-only
assertions requiring `NoHumanProcess` ancestry before any manual injection; or
explicitly split the test into a runner-only proof and a separately-labeled
hand-driven payload-shape proof.

**Structural lock:** the runner-only ancestry assertion on the canonical fixture.

### Low findings

Each low finding retains the full remediation obligation; the compact format lists
evidence → correction → lock inline. All nine were operator-verified at their cited
symbols during the reassessment pass at `9ce820f`; `ORD-HARD-120`'s evidence was
reshaped there (one sub-claim refuted, the core gap confirmed).

**ORD-HARD-112 — The required-cause set is a hand-maintained census, not derived
from kind metadata** (`events` / low; INV-010 lock durability; operator-verified at
reassessment).
`EventKind::requires_cause` is a literal `matches!` list and
`action_emitted_event_kinds_have_cause_disposition` checks a hardcoded array without
asserting coverage of every action-emitting kind — a new kind defaults to
no-cause-required and passes. Correction: derive the required set from
`EventKindMetadata` (action-origin predicate) and iterate all variants. Lock: a
synthetic new action-emitted kind without a cause disposition must fail.

**ORD-HARD-113 — Apply-totality lock asymmetry and an artifact-shaped synthetic**
(`events`/`test_oracle` / low; operator-verified at reassessment). (a) The world-side totality guard's
synthetic negative asserts substrings of a hardcoded string literal instead of
routing the synthetic body through the guard's own scan — tautological. (b) No
agent-stream analogue exists: a new agent-stream kind without an apply arm routes to
the misleading `Err(NonAgentEvent)` rather than a registered arm or allowlisted
no-op. Correction: factor the world-arm scan into a function fed both real and
synthetic bodies; add the agent-stream totality guard derived from
`EventKind::all()`. Lock: per the §5 bijection rule — both synthetics must route
through the production scan path.

**ORD-HARD-114 — The actor-known constructor guard enforces less than its stated
contract** (`test_oracle` / low; operator-verified at reassessment). The guard scans three retired
builder names plus the literal `from_observed_parts(`; the *real* lock is
`from_observed_parts`'s `pub(crate)` visibility. A guard whose comment claims "no pub
producer outside the pair" but enforces a literal list invites false confidence.
Correction: reword the guard's contract to what it enforces, or add a return-type
scan for `-> ActorKnownPlanningContext` in `pub fn` signatures. Lock: synthetic pub
re-wrapper case if the broader contract is kept.

**ORD-HARD-115 — `SupersedeNewestBySubject` silently drops non-Workplace records**
(`holder_known_context` / low; operator-verified at reassessment). `includes_classified_record`'s
supersede arm `let …Workplace{..} = record else { return false }` — a future kind
declared supersede-by-subject in the table is dropped entirely rather than
superseded, the exact kind-added-without-behavior trap `ORD-HARD-074` targeted.
Correction: pair each supersede-policy kind with a subject extractor, or assert
in-code that only Workplace may carry the policy. Lock: folds into the
`ORD-HARD-106` parametric table-driven test.

**ORD-HARD-116 — Content typed-diagnostic and visibility hygiene group**
(`content_seeding` / low; operator-verified at reassessment). (a)
`RoutineStep::FailWithTypedDiagnostic { diagnostic: String }` accepts any free text
at load; the no-sleep gate exact-matches two spellings (`"no_sleep_affordance" |
"NoSleepAffordance"`) — better than the substring check `ORD-HARD-098` replaced, but
not a closed vocabulary (INV-105 spirit). Correction: a `RoutineDiagnosticKind`
parsed-and-rejected-at-load enum; drop the dual spelling. (b)
`PlaceState::new`/`Default` hard-default `visibility_default: Visible` (the
permissive direction) — loader-side authoring is fail-closed, but in-code
construction silently defaults. Correction: document the default as loader-only
convenience or remove the `Default` path for visibility. Lock: load-rejection
negative for an unregistered diagnostic kind.

**ORD-HARD-117 — Generative-tier minor lock shapes** (`test_oracle` / low;
operator-verified at reassessment). (a) The fabricator ban is token co-occurrence
(`EventEnvelope::new` + terminal-kind token), dormant on the real corpus; make
"support/generative.rs constructs zero `EventEnvelope`" an explicit asserted
invariant rather than incidental. (b) `assert_in_block_displacement_ordering`
derives the completion tick from generator arithmetic
(`work.start_tick.advance_by(work.duration_ticks)`), not from engine-emitted events
— the bound is structurally always true given the generator's `start+1` placement.
Correction: read the scheduled completion from the emitted event pair. Lock:
engine-sourced both-bounds assertion.

**ORD-HARD-118 — `mutants.toml` comment overclaims local/CI perimeter parity**
(CI / low; operator-verified at reassessment). `exclude_globs` omits the
non-perimeter defs (takeplace/wait/continue_routine/movement/etc.), so ad-hoc local
runs mutate files outside the reviewed perimeter while the file's comment claims
parity (`ORD-HARD-072` honesty class). Live corroboration: the post-audit `-014`
in-diff run mutated non-perimeter `wait.rs` precisely because the defs are not
excluded; `archive/tickets/0021PHA3APOSREB-014.md` records the leak and explicitly
defers the perimeter correction to this finding. Correction: derive the exclusion
set from the classification table or correct the comment. Lock: a parity check
between the exemption classification and `exclude_globs`.

**ORD-HARD-119 — Report §15 omits the R-29 risk-register addition that §7 item 15
required quoting** (evidence honesty / low; operator-verified at reassessment). The report quotes four
of nine conformance-row actions and never mentions R-29 — the §6 headline
deliverable (which itself landed correctly). Correction: amend §15 (or supersede in
the 0022 artifact). Lock: the `ORD-HARD-102` checklist guard covers this class.

**ORD-HARD-120 — The recommended recovery variant is never proven on the canonical
fixture** (content fixtures / low; operator-verified at reassessment — evidence
reshaped). 0005 §12's recommended success variant (Mara replans to reachable food
and recovers) is proven only in the capstone's hand-built world
(`capstone_world_and_agents` seeds `actor_bruno` + `food_stew_home_bruno`). On
`no_human_day_001` recovery is dynamically *reachable* — the fixture's legacy
`populate_known_food_sources_for_all_actors()` call (allowlisted via `#[expect]`)
makes `food_stew_home_tomas` Mara-known — but no test asserts an autonomous Mara
recovery on the canonical day (the roster test's `FoodConsumed` assertion fires only
after manual injection, `ORD-HARD-111`). Correction: assert an autonomous Mara
recovery on the canonical day, or record the fail-only canonical intent explicitly.
Lock: the autonomous-recovery assertion or the recorded intent.

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0021 layer. The meta-tier is new this pass and targets the
R-29 pattern generically, per the external research (§2).

1. **Meta-locks (new tier).**
   - **Lock↔negative bijection census:** a registry mapping every structural lock in
     `anti_regression_guards.rs` (and the gate files) to its synthetic negative; the
     census fails on a lock without a negative, and every negative must route through
     the same scan/assertion code path as the real check — an inline-literal
     synthetic (the `ORD-HARD-113` shape) fails the census.
   - **Nonzero-witness rule:** every census/scan lock asserts its iterator matched a
     nonzero (or pinned-minimum) number of real sites — a scan that silently matches
     nothing turns rotten-green into a hard failure (`ORD-HARD-104`/`110` class).
   - **Two-sided ratchets:** the mutation baseline and generative corpus floors fail
     on increase *and* on unrecorded decrease (auto-tightening), so a floor cannot
     silently weaken nor an improvement silently evaporate (`ORD-HARD-099` lock).
2. **Compile-time impossibility:** none owed this pass (the 0021 demotions verified
   substantive); keep `from_observed_parts` `pub(crate)` (`ORD-HARD-114` rewords the
   guard, not the visibility).
3. **Runtime gates:** typed scheduler-path errors replacing log-derived `.expect`s
   (`ORD-HARD-108`); the eat crossing emitter routing (`ORD-HARD-107`); the embodied
   render split (`ORD-HARD-103`); the derived `debug_available` (`ORD-HARD-104`).
4. **Test-oracle corrections:** adversarial-truth restoration in the planner gates
   with the discrimination witness (`ORD-HARD-105`); the table-driven behavioral
   policy test with the workplace other-place negative (`ORD-HARD-106`/`115`);
   runner-only ancestry assertions on the canonical day (`ORD-HARD-111`/`120`);
   call-shape-resolved consumed-key derivation (`ORD-HARD-109`); module-wide
   perception prose scan (`ORD-HARD-110`); engine-sourced generative bounds
   (`ORD-HARD-117`); derived cause-set and agent-stream totality guards
   (`ORD-HARD-112`/`113`).
5. **Mutation/CI posture:** the real baseline triage with deferral-phrase rejection,
   suffix-repetition bounds, prefix-pin removal, and filed test-debt tickets
   (`ORD-HARD-099`); concurrency-group event isolation (`ORD-HARD-100`); fail-closed
   mutants.toml key allowlist, fail-closed glob matcher, step-scoped capture
   counting, generalized swallow detection (`ORD-HARD-101`); classification-derived
   local exclusion parity (`ORD-HARD-118`).
6. **Evidence honesty:** the §7-checklist parity guard (`ORD-HARD-102`); 0021-report
   corrections or 0022-artifact supersession recording the 0021 triage deviation
   explicitly (`ORD-HARD-099`/`102`/`119`).

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: extend R-29 with the two shapes this
  pass surfaced — the *self-echo lock* (both sides of an assertion read the same
  source: `ORD-HARD-106`) and the *artifact-shaped synthetic* (a negative that does
  not route through the production scan: `ORD-HARD-113`); add the meta-lock
  guardrails (bijection census, nonzero-witness, two-sided ratchet) as the standing
  countermeasure. Add a Watch note under R-28: a correction whose required behavior
  is "perform work once" (a triage, a migration) needs its acceptance evidence to be
  the work product itself, not the governance machinery around it
  (`ORD-HARD-099`/`102` shape).
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  update the disposition-tag governance row once the real triage lands
  (`ORD-HARD-099`); update the scheduled-ratchet row for the concurrency isolation
  (`ORD-HARD-100`); update the per-kind policy dispatch row once the behavioral test
  is real (`ORD-HARD-106`); update the shared-emitter row for the eat.rs perimeter
  (`ORD-HARD-107`); add rows for the embodied debug-render split (`ORD-HARD-103`),
  the planner-gate adversarial restoration (`ORD-HARD-105`), and the meta-lock tier
  (§5.1).
- No doctrine amendment; INV-001…INV-110 are applied, not changed. The INV-087
  decision (`ORD-HARD-095`) remains deferred to its recorded owner-decision boundary;
  nothing in this spec pre-decides it.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0022_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The real baseline triage: per-entry dispositions with the deferral-phrase guard
   green, the filed test-debt tickets (new series), entries retired by focused
   tests, the baseline shrink recorded by the two-sided ratchet, and an explicit
   record of the 0021 deviation (`ORD-HARD-099`).
2. The concurrency isolation and the fail-closed config/glob/capture guards with
   their firing synthetics (`ORD-HARD-100`/`101`); the first scheduled-run result
   under the new posture, or its honest pending status (`ORD-HARD-102`).
3. The embodied render split: the debug-token absence test output and the
   `debug_available` derivation or deletion (`ORD-HARD-103`/`104`).
4. The planner-gate adversarial restoration: per-gate evidence that hidden truth
   exists in `PhysicalState`, partial knowledge is seeded by event, and the
   discrimination witness fires (`ORD-HARD-105`).
5. The table-driven policy behavioral test output across all kinds and both
   surfaces, including the workplace other-place negative and the supersede-subject
   pairing (`ORD-HARD-106`/`115`).
6. The eat crossing proof (both directions) and any honest golden repricing diff
   (`ORD-HARD-107`).
7. The scheduler typed-error conversions and the implemented expect/assert guard
   with its allowlist rationales (`ORD-HARD-108`).
8. The census call-shape fix, perception module-wide scan, derived cause-set, and
   agent-stream totality guard, each with firing synthetics
   (`ORD-HARD-109`/`110`/`112`/`113`).
9. The runner-only canonical-day proof (or the recorded fail-only intent) and the
   recovery-variant resolution (`ORD-HARD-111`/`120`).
10. The content/generative hygiene closures (`ORD-HARD-116`/`117`) and the
    mutants.toml parity resolution (`ORD-HARD-118`).
11. The meta-lock tier: the bijection census output, the nonzero-witness adoption
    list, and the two-sided ratchet proof (§5.1).
12. The 0021-report corrections (`ORD-HARD-102`/`119`) and the §7-checklist parity
    guard run against this artifact itself.
13. The risk-register and conformance-index diffs, quoted (§6).
14. An updated `EMERGE-OBS` ledger derivation over the corrected surface
    (measurement only, no thresholds).
15. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`;
    not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core;
  tui on core + content. No new dependencies, dev or production (the §5 meta-locks
  are hand-rolled in the existing integration-test style).
- All findings (`ORD-HARD-099`–`120`) are operator-verified at their load-bearing
  symbols — `099`–`111` during the audit at `ad39160`, `112`–`120` at the
  reassessment pass at `9ce820f` (zero refuted; `120`'s evidence reshaped) — so no
  re-verification step is owed; a finding whose premise nonetheless fails at
  implementation time is recorded as refuted in the acceptance artifact, not
  silently dropped.
- Recommended ticket ordering:
  1. `ORD-HARD-099` + `102` + `119` (triage integrity + evidence corrections — the
     pass's reason for existing; do it first and honestly).
  2. `ORD-HARD-100` + `101` + `118` (mutation-CI residue; one guard file + ci.yml).
  3. §5.1 meta-locks (bijection census, nonzero-witness, two-sided ratchet) — early,
     so subsequent tickets' new locks are born under the meta-rules.
  4. `ORD-HARD-105` + `114` (hidden-truth gate adversarial restoration).
  5. `ORD-HARD-106` + `115` (policy behavioral locks).
  6. `ORD-HARD-107` (eat crossing; batch any golden repricing once).
  7. `ORD-HARD-108` + `112` + `113` (scheduler/events reject-loudly and totality).
  8. `ORD-HARD-103` + `104` (embodied debug quarantine + sweep producer semantics).
  9. `ORD-HARD-109` + `110` (census/guard scan closure).
  10. `ORD-HARD-111` + `120` (canonical-day evidence provenance).
  11. `ORD-HARD-116` + `117` (content/generative hygiene remainder).
  Documentation corrections (§6) land with group 1. The acceptance artifact lands
  last, measuring the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-099`'s honest triage has real cost and real test debt.** The 0020 and
  0021 passes both deferred it; the third deferral would make the disposition ledger
  a permanent fiction. Budget the focused-test tickets genuinely; if the owner
  decides the cost is not worth paying now, the honest alternative is an explicit
  recorded owner deferral in the ledger header — not a re-costumed bulk accept.
- **`ORD-HARD-105`'s restoration may surface latent planner defects.** Re-arming the
  planner gates with real adversarial truth may fail gates that have been passing
  vacuously since the rebuild — per the Enforcement reading, treat any new failure
  as a potential product defect first.
- **`ORD-HARD-107`'s crossing events may reprice golden logs** (the `ORD-HARD-076`
  risk, again, for eat) — batch once, re-derive honestly.
- **The meta-lock tier must not itself be decorative.** The bijection census and
  nonzero-witness rules are scans over test source — apply them to themselves (the
  census lists itself with its own negative).
- **Next-iteration assessment (the recurring question):** findings were found, so a
  twelfth pass is warranted by the lineage's own rule. The honest trend: zero
  blockers for five consecutive passes; the kernel product core clean for five; the
  0005 feature contract verified intact end-to-end for the first time since 0014;
  and every high/medium this pass lives in the test-oracle, CI, or evidence layer
  except `ORD-HARD-107` (a residual family member, not a new class). The twelfth
  pass should be a **strictly scoped verification of 0022's deliverables** — above
  all whether the `ORD-HARD-099` triage actually happened this time (it is the
  lineage's first finding to recur twice) and whether the §5.1 meta-locks fire. If
  it returns clean or low-only, the lineage's standing recommendation activates:
  drop per-pass cadence and move to per-phase-entry audits (Phase 3B / Phase 4
  boundaries), keeping the meta-locks as the standing anti-contamination posture.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability / evidence-honesty) and names responsible layers from the execution
  diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding was independently operator-verified at its
  load-bearing symbols (the audit pass at `ad39160` plus the reassessment pass at
  `9ce820f`; zero refuted, one evidence sub-claim reshaped — `ORD-HARD-120`).
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression; the §5.1 meta-tier addresses the lock-vacuity class generically.
- [x] Verified-holding items from 0014–0021 are recorded to prevent re-litigation;
  the 0005 feature-contract verification is recorded; severity calibrations that
  diverge from lineage precedent or from the reporting audit slice are named in §4's
  preamble (`ORD-HARD-099`/`100`/`103`/`106`/`107`).
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved. The INV-087 decision remains deferred, untouched.
- [x] Scope stays within the Phase 3A ordinary-life surface and its lock/evidence
  layer.

## Outcome

Completed: 2026-06-12

Spec 0022 landed as twenty-three ticket commits, ending with the final
mutation-baseline closure commit `f005d30`. The implementation closed the
baseline-triage integrity, mutation-CI parity, meta-lock, hidden-truth,
policy-lock, eat-crossing, scheduler reject-loudly, embodied debug-render,
census-widening, runner-only no-human proof, diagnostic/generative, conformance,
acceptance-artifact, and focused mutation-test findings recorded by this spec.
The final archive truthing commit `57383f4` normalized completed ticket outcomes
to the repository archival format.

The final artifact is `reports/0022_ord_life_cert_scoped_acceptance.md`; it records
the implementation commit manifest, the §7 evidence map, the 0021 evidence
corrections, the conformance and risk-register closure, and the explicit
non-certification posture. The mutation-baseline disposition now records
normalized-count `0` with FNV-1a64 `cbf29ce484222325`. No doctrine amendment was
made.

Deviations from the original plan: the original thirteen-ticket implementation
opened focused mutation-test follow-ups `0022PHA3ABASTRI-015` through
`0022PHA3ABASTRI-023`; those follow-ups retired the remaining viable baseline
mutants instead of leaving ticketed test debt in the baseline ledger.

Verification passed on 2026-06-12:

```sh
cargo test -p tracewake-core --test anti_regression_guards
cargo test -p tracewake-core
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```
