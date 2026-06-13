# 0025 Phase 3A Meta-Witness Execution Proof, Perception Kill-Set Provenance, Envelope Read-Path Fail-Closed, and Manifest Fingerprint Honesty Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `9e33d7a` (merge PR #32: all `0024PHA3ACONSCH` tickets landed; acceptance artifact `reports/0024_ord_life_cert_scoped_acceptance.md`). All four gates (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) measured green at this baseline before the audit, run as separate commands with per-gate unmasked OK sentinels.
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the fourteenth Phase 3A alignment pass — the strictly scoped verification of
0024's deliverables that 0024 §9 itself prescribed (above all: did the meta-witness
completion (`ORD-HARD-141`) eliminate the presence-check arm for real, and does the
schema-version gate sit on the production load path?), plus the standing 0005
feature-contract conformance check, plus the final blind sweeps 0024 §9 named of the
last never-scoped surfaces: the serialization/manifest layer end-to-end,
`controller.rs`/`debug_capability.rs`, and the CI workflow files as a product surface.
Spec 0014 closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`; 0016 closed
`014`–`025`; 0017 closed `026`–`034`; 0018 closed `035`–`043`; 0019 closed `044`–`052`;
0020 closed `053`–`065`; 0021 closed `066`–`098`; 0022 closed `099`–`120`; 0023 closed
`121`–`139`; 0024 closed `140`–`165`. This audit re-derived the normative contract from
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, and
`docs/3-reference/*`, and treated every 0024 correction as unverified until proven in
code at `9e33d7a`, per the lineage rule. Findings continue the `ORD-HARD` series at
`166`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The
audit was conducted as a nine-slice delegated review (meta-witness completion,
content-loader fixes, embodied truth-access completion, apply-perimeter + TUI
quarantine, policy/epistemic/oracle closures + 0005 coherence pins, 0005 conformance,
serialization/manifest blind sweep, controller/debug-capability blind sweep, CI +
evidence honesty) plus an external research refresh. Every high and medium finding,
and the majority of lows, were independently operator-verified at their load-bearing
cited symbols during triage; the remaining findings initially carried on agent
evidence alone (`ORD-HARD-178`, `185`, `189`, `190`, and sub-claims of `181`/`187`)
were operator-re-verified at source during spec reassessment at the same baseline —
all confirmed, none refuted — so every finding is operator-verified and no
re-verification step is owed (a premise that nonetheless fails at implementation
time is recorded as refuted in the acceptance artifact, not silently dropped).

The headline answers to 0024 §9's questions:

1. **The schema-version gate is real and production-pathed.** `validate_schema_version`
   (typed code `unsupported_fixture_schema_version`) runs inside
   `validate_fixture_errors`, reached by `validate_fixture_bytes` from
   `load_fixture_package`, with the negative proven end-to-end through the loader
   (`fixtures_load_unsupported_schema_version_rejected_001`) and a golden-version load.
   `ORD-HARD-140` is genuinely closed, as are `150`/`151`/`164` (operator-verified).
2. **The meta-witness completion is *not* complete: the presence-check shape relocated
   a third time.** The `_ =>` default arm is gone, but `behavior_assertion_live_witness_count`
   and `shared_scan_function_witness_count` — the routing for roughly forty
   `BehaviorAssertion` rows plus the un-cased SharedScan fall-through — return
   `body.matches("assert!").count()`-style token counts: textual presence of assertion
   macros, not executed negatives or inspected-site counts (`ORD-HARD-166`). Several
   cased perception witnesses still count anchor-function presence (`ORD-HARD-175`).
3. **For the third consecutive pass, the only product-behavior foundation defect came
   from a blind sweep of a never-scoped surface** — this time the kernel envelope
   read path: `EventEnvelope::deserialize_canonical` silently last-wins on duplicated
   field keys, accepting ambiguous serialized history instead of rejecting it loudly
   (`ORD-HARD-168`, the INV-020 reject-loudly posture at the letter). It is materially
   weaker than 0024's `ORD-HARD-140` (no in-repo producer can emit a duplicate key;
   the surrounding posture is fail-closed), and the supply of never-scoped surfaces is
   now exhausted: all three corners 0024 §9 named have been swept.
4. **The cognition firewall is clean for the eighth consecutive pass**, and the 0005
   feature contract is intact end-to-end with zero product regressions; no 0014–0024
   lock narrowed product behavior below the contract (one *recommended* demonstration
   remains unbuilt and its staging unrecorded — `ORD-HARD-187`).

## 1. Scope

### In scope

- The meta-lock witness tier residue (`crates/tracewake-core/tests/anti_regression_guards.rs`:
  `behavior_assertion_live_witness_count`, `shared_scan_function_witness_count`,
  `perception_visibility_other_emission_inspected_site_count`,
  `test_body_has_structural_scan_shape`, `META_LOCK_CENSUS_EXEMPTIONS`).
- The perception prose-branch scan's provenance machinery and kill set
  (`perception_sensitive_helper_params`, `perception_visibility_prose_branch_violations`,
  the `renamed_parameter_helper_synthetic`/`payload_value_relay_synthetic` pair).
- The kernel envelope read path (`events/envelope.rs::deserialize_canonical`,
  `checksum_after`, the decode-layer/log-layer version-gate split with
  `events/log.rs::append_deserialized`).
- The content manifest and fingerprint end-to-end (`manifest.rs::stable_fingerprint`,
  `load_fixture_package`'s `canonical_bytes` payload, fingerprint consumers, golden
  serialization bytes).
- The embodied truth-snapshot boundary's lock scope (`projections.rs::
  EmbodiedTruthSnapshot`, `EmbodiedPreflightSource`, `semantic_actions`,
  `phase3a_semantic_actions`, `guard_014_embodied_projection_source_has_no_physical_state_field`)
  and the observation-time capture `ORD-HARD-154` required.
- TUI gate depth (`app.rs::run_no_human_day` method surface,
  `debug_command_gating_errors`, TUI-local lock enrollment, the `ControllerMode::Debug`
  variant's role in `debug_available_for`).
- Census/oracle closures' residual hand-lists (`is_schema_id_field_type`,
  `Location`-embedded IDs, the `ActionEffect` positive-census filter, the
  `tests/support/*` ban list, the policy generic-loop oracle).
- Recorded-decision hygiene: the canonical recovery-resolution staging, CI↔doctrine
  reconciliation, the scheduled-mutation pending status, CI cache-key hygiene and a
  workflow-lint guard.

### Out of scope

- Re-auditing Phase 1/1A/2A internals except where named findings touch them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0024 fixed that this audit verified as holding (§3).
- The `ORD-HARD-095` INV-087 owner decision (bind-time perception events): still
  deferred, untouched; this pass's controller sweep re-surfaced it and the finding is
  recorded as mapped to the standing deferral, not re-minted.
- Building the player-facing recovery/success content beyond the recorded staging
  decision (`ORD-HARD-187` requires the decision record or the second resolution
  token, not new feature work).
- Adopting external CI tooling (actionlint, zizmor, pinact): rule catalogs are
  translated into in-repo grep guards only; no new dependencies.

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-020 (reject unsupported
  history rather than silently inventing repairs — `ORD-HARD-168`/`171`); INV-018/019
  (replay reconstruction inputs and ancestry — `ORD-HARD-169`/`170`); INV-067/093/099–101
  (embodied truth quarantine — `ORD-HARD-172`/`173`); INV-068/107 (debug quarantine —
  `ORD-HARD-176`/`186`); INV-097 (no-script scanning — `ORD-HARD-181`/`182`).
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — the
  reject-loudly replay posture and the content-manifest-ID replay input
  (`ORD-HARD-168`/`169`).
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — fixture
  manifests declare compatible expectations (`ORD-HARD-169`/`170`).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-28 (hand-lists beside derivable
  populations: `ORD-HARD-177`/`179`/`181`), R-29 (decorative witnesses: the
  assert-token-count shape `ORD-HARD-166`, the anchor-presence shape `ORD-HARD-175`,
  the prose-keyed taint `ORD-HARD-167`, the hollow integrity field `ORD-HARD-184`).
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale — and a lock is real only if its negative case can actually fire on
  the behavior.

**External research consulted this pass** (extends the 0017/0018/0022/0023/0024
corpus; all adopted techniques survive the zero-dependency constraint):

- *CI workflows as an integrity surface*: actionlint's checks catalog and zizmor's
  audit rules port as grep checklists (no tool adoption): full-SHA pinning for
  third-party actions, explicit `permissions:`, no expression interpolation into
  `run:` blocks. GitHub's default `run:` shell is `bash -e` **without** `pipefail` —
  a piped gate command can pass green on a failing first stage; the portable
  countermeasure is a workflow-lint `#[test]` over `.github/workflows/*` asserting
  the four documented gate commands appear verbatim and gate steps contain no pipes
  or `||` (`ORD-HARD-188`/`189`).
- *Who guards the guards* (Kintis et al., EMSE 2018; Petrović et al., IEEE TSE 2021):
  second-order assurance pays once per guard — prove it *can* fail with one committed
  known-bad fixture — and stops paying at deeper layers; the "productive mutant"
  criterion is the diminishing-returns floor. Adopted as the witness-repair posture
  for `ORD-HARD-166`: executed negatives, not a third meta-layer.
- *Stopping rules for inspection cadence* (capture-recapture in software inspections,
  JSS 2004; Briand et al., IEEE TSE 2000; reliability-growth release criteria):
  estimate residual defects from overlap between independent passes; re-inspect only
  while marginal yield exceeds cost. Feeds §9's cadence recommendation.
- *Content-addressable manifest integrity* (CAS pattern; TUF-style manifest layering):
  fingerprint raw committed bytes, never the parsed struct; pin one root digest so
  fixture updates force a reviewable constant change; assert the manifest's file list
  equals a directory walk so new unfingerprinted files fail closed. Adopted for
  `ORD-HARD-169`/`170`.
- *Possession-parity testing analogues* (AlphaStar interface-fairness; Riot
  headless-bot harnesses; EA game-testing agents): parity via forcing all controllers
  through one validation entry point — confirmed already implemented here
  (`human_after_authorization_matches_scheduler_validation`); no new adoption needed.
- Flagged not-applicable (so they are not re-researched): actionlint/zizmor/shellcheck/
  pinact/Dependabot/StepSecurity as adopted tools (dependencies); LLM-based mutant
  generation (LLM oracles out of scope); Merkle-DAG evidence chains (overkill vs one
  pinned digest); formal capture-recapture estimators (need ≥3 independent inspectors).

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not maximally locked, and one
kernel read-path defect breaches the reject-loudly posture at the letter. Twenty-five
findings follow, `ORD-HARD-166` … `ORD-HARD-190`: two high, nine medium, fourteen
low, zero blockers. One finding is a product-behavior foundation defect at the letter
of INV-020's reject-loudly posture (`ORD-HARD-168`); the two highs are decorative-guard
defects in the repaired meta-tier and the repaired perception scan — the third
consecutive pass in which a witness repair carried the witnessed defect's shape. No
INV-099…INV-106 truth-firewall violation exists in production cognition — the
cognition firewall verified clean for the **eighth consecutive pass**. The 0005
feature contract is intact end-to-end; no 0014–0024 lock narrowed product behavior
below the 0005 contract.

### Verified holding (no action; recorded so they are not re-litigated)

- **The fixture schema-version gate is substantive and production-pathed**
  (`ORD-HARD-140` closed): `validate_schema_version` emits
  `unsupported_fixture_schema_version` at `ParseSchema` inside `validate_fixture_errors`;
  `load_fixture_package` → `validate_fixture_bytes` reaches it on every load;
  negatives proven through the loader (`fixtures_load_unsupported_schema_version_rejected_001`),
  golden-version load present.
- **The raw-line scan runs on the production path** (`ORD-HARD-150` closed):
  `validate_fixture_bytes` calls `validate_raw_lines` before `deserialize_fixture`;
  prose-born-fact and forbidden-key negatives fire through `load_fixture_package`.
- **ID-typed fields are shortcut-scanned with a census** (`ORD-HARD-151` closed at its
  promised shape): `validate_id_shortcut_markers` routes every registered ID through
  the shared marker predicate; `schema_id_fields_are_classified_for_script_scanning`
  fails on an unregistered synthetic ID field. (Recognizer hand-list and
  `Location`-embedded IDs: `ORD-HARD-181`/`182`.)
- **Need seeds are required, the silent 100 default is gone** (`ORD-HARD-164` closed):
  `missing_actor_need_seed` typed rejection; `to_agent_state` materializes only seeded
  needs; census `all_fixtures_author_explicit_need_seeds_for_every_actor`.
- **Golden serialization bytes are decoder-independent** (`ORD-HARD-165` closed at its
  minimal shape): `fixture_serialization_golden_bytes_are_pinned_001` pins a frozen
  `EXPECTED` byte literal against `serialize_fixture` output. (Single-family coverage
  and the tautological perturbation assert: `ORD-HARD-183`.)
- **The apply-mutator perimeter derivation is real** (`ORD-HARD-144` closed):
  `apply_mutator_tokens_from` derives the forbidden set from `pub fn apply_*`
  signatures with a two-way parity census (`apply_story_event` synthetic fires);
  four rationale-bearing allowlisted seams; enrolled in `META_LOCK_REGISTRY` with a
  genuine measured witness (`apply_mutator_live_witness_count`, witness_min 4);
  non-allowlisted injection fires.
- **The deferral content witness is write/consume-shaped** (`ORD-HARD-142` closed):
  `EMBODIED_SURFACE_FIELD_PRODUCERS` cites `debug_only_diagnostics` to a real consume
  site in `render.rs`; `source_has_deferral_write_or_consume_site` required; the
  orphaned-definition synthetic exercises a reachable case.
- **The baseline chain is genesis-anchored** (`ORD-HARD-145` closed):
  `mutation_baseline_change_log_records` pins genesis `from_count`/`from_hash` to
  `MUTANTS_BASELINE_GENESIS_COUNT`/`MUTANTS_BASELINE_GENESIS_FNV1A64`; the
  fabricated-single-link and shortened-chain synthetics fire.
- **Census exemptions are validated at the promised shape** (`ORD-HARD-155` closed):
  scan-shaped exemptions must name a covering `lock_id` asserted present; the
  scan-shaped-exemption-without-cover synthetic fires. (Detector narrowness:
  `ORD-HARD-180`.)
- **The embodied struct quarantine and sealed snapshot landed** (`ORD-HARD-143`
  substantially closed): `EmbodiedProjectionSource` carries no `PhysicalState`;
  `from_sealed_context` takes an explicit `EmbodiedTruthSnapshot` instead of
  `&PhysicalState`; validator preflight flows through `EmbodiedPreflightSource` into
  `validate_proposal` lawfully under INV-099; the candidate set derives from sealed
  `actor_known_*` collections; the INV-093 absence negative and item staleness
  positive both discriminate post-refactor. (Lock-region scope: `ORD-HARD-172`;
  observation-time capture: `ORD-HARD-173`.)
- **The TUI quarantine decisions landed and are recorded** (`ORD-HARD-152`/`153`
  closed at the command surface): `run no-human-day` is a `debug `-prefixed command
  (`parse_debug_command`), bare form returns `UnknownCommand`; `render_debug`
  early-returns a typed refusal on `!app.debug_available()` before dispatching any
  `DebugCommand`; `debug_available_for` requires a bound, non-`Detached` binding;
  refusal test with no bound controller passes; `docs/2-execution/07` records the
  operator-proof classification and the Phase 3B+ staging of player time controls,
  matching code. (Method-surface gate depth: `ORD-HARD-176`; gating-guard census
  depth: `ORD-HARD-174`; `Debug` mode variant: `ORD-HARD-186`.)
- **Render reachability is widened** (`ORD-HARD-157` closed):
  `render_functions_have_production_callers_or_documented_allowlist` scans `render.rs`
  and `debug_panels.rs`; the orphaned-panel synthetic fires.
- **The policy/epistemic closures are substantive** (`ORD-HARD-147`/`148`/`158`
  closed): workplace accessibility gated on `FromAnyPlace` with the fourth row
  mutation firing; `assert_context_excludes_unseeded_hidden_counterparts` is
  closed-world over seeded sets with the self-contains assert deleted and the
  unseeded-injection negative firing; the tampered-log Mara negative proves the
  resolution arm fires on an injected autonomous `FoodConsumed`.
- **The 0005 coherence pins landed** (`ORD-HARD-161`/`162`/`163` closed):
  `goal_priority_selection_rank_snapshot_pins_decided_order` plus the behavioral
  urgent-vs-active pin, with the divergence-by-decision recorded in the conformance
  index; `default_day_windows` disjoint (`(0,3),(4,9),(10,17),(18,23),(24,32)`) with
  the disjoint-and-cover test; `EventKind::EatFailed` in
  `assert_required_acceptance_events`.
- **The 0005 feature contract is intact end-to-end at `9e33d7a`.** Need bands exact at
  §8.1 boundaries with both-direction evented crossings; typed intention lifecycle
  with possession-survival; all ten routine families authored with failure modes and
  interruption points; sleep/work duration-based with body-exclusive reservation and
  prorated costs; eat consumes modeled servings with typed refill-free failures;
  validation/outcome distinction (rejected vs started-then-failed); deterministic
  scheduler ordering through `ActorDecisionTransaction` → `run_pipeline` with no
  direct dispatch; `no_human_day_001` roster and required event coverage including
  `EatFailed`/`WorkBlockFailed`; blocker categories typed; capstone live==rebuild
  typed equality; no protagonist gravity (`player_conditioned_event_count == 0`).
- **The production cognition firewall is clean (eighth consecutive pass).** The
  possessed path and the scheduler path share one validation entry
  (`human_after_authorization_matches_scheduler_validation`); `controller_binding_check`
  only authorizes; binding application is a physical no-op; `debug_available` is
  derived at the TUI boundary, absent from authoritative state, written after
  `build_embodied_view_model`, and outside `holder_known_context_*` and
  `ProposalSourceContext`; `DebugCapability` is crate-mint-only with `compile_fail`
  proofs; no sacred-player residue in authoritative state.
- **CI gate fidelity holds at the command level.** The four documented gates appear
  with identical flags in `.github/workflows/ci.yml`; toolchain defers to
  `rust-toolchain.toml`; no `continue-on-error`, no pipes on gate commands, no
  exit-status masking on load-bearing steps; third-party actions are first-party
  (`actions/*`) tag-pinned; `cargo-mutants` version-pinned `--locked`; the
  mutants-in-diff ratchet compares against an empty committed baseline and fails on
  any new miss.
- **0024 evidence honesty holds.** `reports/0024_ord_life_cert_scoped_acceptance.md`
  carries the non-certification statement and the honest mutation-pending status; its
  load-bearing claims and the 0024 conformance-index rows cite symbols that resolve
  at `9e33d7a`.

### Validated — no action (checked, found intentional, correctly scoped, or already owned)

- **Bind-time perception events** (`TuiApp::bind_actor` →
  `record_current_place_perception_and_project`): re-surfaced by the controller sweep
  as an INV-087/108 tension; this is exactly `ORD-HARD-095` (0021), whose owner
  decision the lineage explicitly defers. Mapped to the standing deferral — not
  re-minted, no new finding ID. The deferral remains open and is restated in §9.
- **The fail-only Mara resolution pinning mechanism works as designed**: the
  `canonical_mara_recovery_resolution=fail_only_empty_food_source` token is authored
  fixture content; the guard enforces the recorded resolution and flipping it to an
  unsupported value fails loudly — decision-pinning, not contract narrowing. (The
  *unrecorded staging* of the recommended success variant is `ORD-HARD-187`.)
- `EpistemicApplyError` variant mapping, `from_observed_parts` underscore bindings,
  the `witness_min: 0` empty-baseline exemption, `parse_command` digit-prefix
  precedence: 0023/0024 dispositions re-confirmed, hold as scoped.
- Checksum honesty re-confirmed on the new sweep: `compute_physical_checksum`/
  `compute_agent_state_checksum` iterate ordered structures only; summary/controller
  prose excluded with discriminating negatives; no system-time/HashMap nondeterminism
  in any serialized output swept (`ORD-HARD-168`'s file included).
- Branch-protection/required-checks enforcement is GitHub-side and not auditable
  in-repo; recorded here so a maintainer confirms the five PR jobs are merge-blocking
  (informational; no code correction available).

## 4. Findings and remediation requirements

Severity calibration notes: `ORD-HARD-166` and `167` are rated high per the
`ORD-HARD-122`/`141`/`142` precedent — each is the anti-rotten-green repair itself
carrying a rotten-green shape, the third consecutive pass with this class.
`ORD-HARD-168` is rated medium where `ORD-HARD-140` (the closest INV-020 letter
precedent) was high: 140 was the entire absence of a mandated gate, fail-open for
every future revision; 168 is a single malformed-input acceptance on a path whose
sibling malformations all fail closed and which no in-repo producer can trigger —
the `ORD-HARD-107` single-site-residue calibration. `ORD-HARD-169`/`170` are rated
medium where the reporting slice proposed high: the fingerprint is not a
doctrine-mandated gate (no foundation/execution doc requires fingerprint
verification; replay's data-version input is satisfied by `content_version`), so the
defect class is hollow-evidence/R-29, not a missing constitutional gate.
`ORD-HARD-173` is rated medium where its precedent (`ORD-HARD-154`) was low: the
0024 repair claims closure while delivering a staleness positive aimed at the wrong
axis — the `ORD-HARD-142` delivered-repair-unfalsifiable elevation. `ORD-HARD-176`
is rated medium where the reporting slice proposed high: no production-reachable
bypass exists (precedent `ORD-HARD-104`/`153`, derived-gate-consulted-by-one-consumer).
`ORD-HARD-177` is rated low where the slice proposed medium (precedent
`ORD-HARD-159`, the same residue class at low). `ORD-HARD-188` is rated low where
the slice proposed medium: the slice's green-without-gates mechanism is partially
refuted (cargo re-fingerprints changed sources on a fresh checkout; cache writes
require an already-trusted run), leaving key hygiene and the absent workflow-lint
guard as the residue.

### ORD-HARD-166 — The meta-witness presence-check shape relocated a third time: BehaviorAssertion and fall-through SharedScan witnesses count `assert!` tokens in the test body, not executed negatives or inspected sites

**Severity:** high (precedent `ORD-HARD-141`/`122`).
**Verification:** operator-verified (`behavior_assertion_live_witness_count` computes
`assertion_count` as `body.matches("assert!").count() + body.matches("assert_eq!").count()
+ body.matches("assert_absent(").count()` over `function_body_if_present` and returns
`usize::from(assertion_count > 0)`; `shared_scan_function_witness_count` sums
`assert!`/`_errors(`/`_violations(` substring counts; roughly forty registry rows
route `MetaLockRouting::BehaviorAssertion`).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-29 (the witness measures textual presence, not behavior);
`ORD-HARD-141`'s required correction verbatim ("BehaviorAssertion rows get a distinct
routing whose witness is the negative's actual execution (run-and-must-fire), not name
presence"; "a scan that inspected zero sites is a failure").

**Evidence:** the 0024 repair deleted the `_ =>` arm and the two banned literal
strings (`test_names.contains(entry.lock_id)` / `negative_id` containment), but the
replacement helpers re-derive a 0/1 witness from assertion-macro *token counts* in the
test's source text. Renaming a BehaviorAssertion guard's internal anchor so its
violation set goes vacuously empty leaves its `assert!` macros textually present —
witness stays 1, suite green. The "formerly default-routed" synthetic feeds an empty
body to one cased entry, proving only that an absent body yields 0, not that a
present-but-vacuous detector drops below minimum. The textual ban on the two old
literals cannot see the relocated shape.

**Why thirteen passes missed it:** each repair is validated against the *previous*
presence shape's literal strings; no negative has ever exercised a
present-but-vacuous detector through the witness routing.

**Required correction:** per the adopted research posture (executed negatives, once
per guard — no third meta-layer): BehaviorAssertion rows' witness becomes the actual
execution of the row's negative (the negative test invoked as a function against its
known-bad fixture, asserted to produce a nonzero violation/failure signal at witness
time), or the row is re-routed to a SharedScan whose witness is the violation
predicate's inspected-site count returned from the scan body. `shared_scan_function_witness_count`
is deleted; every SharedScan routes to a measured count.

**Structural lock:** a synthetic that empties a BehaviorAssertion guard's violation
set while leaving its `assert!` macros textually intact must drop the witness below
`witness_min`; extend the witness-shape ban to forbid `matches("assert!")`-style
token counting anywhere in `meta_lock_live_witness_count`'s helpers, with the ban
itself carrying a firing negative.

### ORD-HARD-167 — The perception kill set is vacuous for the provenance machinery: both synthetics fire via pre-existing raw-token rules, and taint is keyed to prose tokens, so the 0024-cited evasion still escapes

**Severity:** high (precedent `ORD-HARD-142`/`122` — the repair is unfalsifiable for
the shape it claims to kill).
**Verification:** operator-verified (`renamed_parameter_helper_synthetic`'s asserted
violation line contains `place.display_label` — flagged by the raw `display_label`
rule; `payload_value_relay_synthetic`'s asserted line contains `.contains("hidden")` —
flagged by the raw hidden-prose rule; `perception_sensitive_helper_params` marks a
helper parameter sensitive only when the helper's own body line contains
`.contains("hidden")`/`.to_lowercase()`/`.to_ascii_lowercase()`).

**Responsible layers:** `test_oracle`, perception channel.

**Doctrine breached:** INV-022 lock durability; `ORD-HARD-149`'s required correction
("track bindings derived from `display_label`/`*_id` projections across `let` and
parameter boundaries"; kill-set rewrites "must both fail the scan" *through the
provenance path*).

**Evidence:** the provenance additions (`perception_tainted_let_alias`,
`branches_on_tainted_binding`, `line_calls_sensitive_helper_with_tainted_argument`)
exist but are never load-bearing: both kill-set synthetics' asserted violations fire
from the raw lexical rules that predate 0024. Taint is assigned by *helper-body prose
content*, not by *argument data provenance* — so the exact evasion `ORD-HARD-149`
cited (`fn gate(label: &str) -> bool { label.starts_with("vault") }` fed a relayed
display-label binding) carries no scanned token in any line: the helper is never
marked sensitive, the call site is not flagged, and the laundering hole the finding
named remains open while the kill set certifies it closed.

**Why existing gates miss it:** the kill-set assertions check that *a* violation is
reported for the synthetic, not *which rule* reported it; no synthetic isolates the
provenance path by carrying zero raw tokens.

**Required correction:** key taint to data provenance: a helper parameter is
sensitive when any call site passes an expression derived (via `let`-alias tracking,
already built) from `display_label`/`*_id` projections — regardless of the helper
body's tokens; the typed `PayloadField` write remains the only sink. Then add a
kill-set member with no raw token in any branching line (the `starts_with("vault")`
helper fed a relayed binding) asserted to fire **only** via
`line_calls_sensitive_helper_with_tainted_argument`.

**Structural lock:** the new kill-set synthetic must fail the scan with the raw
`display_label`/hidden-prose rules disabled for the synthetic source (proving the
provenance path is load-bearing); a guard asserting every kill-set member's asserted
violation names the rule family that produced it.

### ORD-HARD-168 — `EventEnvelope::deserialize_canonical` silently last-wins on duplicated field keys: ambiguous serialized history is accepted instead of rejected loudly

**Severity:** medium (calibration divergence from `ORD-HARD-140` named in the §4
preamble; product-behavior defect at the letter of INV-020's reject-loudly posture).
**Verification:** operator-verified (the decode loop `map.insert(key, value)` over a
`BTreeMap` with no occupancy check; sibling malformations fail closed —
missing `=` is `MalformedField`, unknown kind is `UnknownEventKind`).

**Responsible layers:** kernel event log, replay-evidence honesty.

**Doctrine breached:** INV-020 (reject unsupported history rather than silently
inventing repairs — a duplicated field is ambiguous history and last-wins is a silent
repair); the `docs/1-architecture/02` reject-loudly replay posture.

**Evidence:** a serialized envelope line set containing `event_type=…` twice (or a
duplicated `content_manifest_id`, `payload`, or `checksum_after`) deserializes with
the last occurrence silently winning. A tampered or corrupted log line can therefore
override authoritative fields without any diagnostic; downstream, state-affecting
tampering is caught only where a checksum comparison happens to run, and
provenance-only fields (`cause`, `content_manifest_id`) can be rewritten invisibly.
No in-repo writer can produce duplicate keys (`serialize_canonical` emits each field
once), which is why this is medium, not high — but the read path's contract is
fail-open for exactly the input class INV-020 exists to reject.

**Why existing gates miss it:** every round-trip test feeds self-produced
(deduplicated) bytes; no negative feeds a duplicated key.

**Required correction:** reject duplicate keys in `deserialize_canonical` — error
(`EventEnvelopeParseError::DuplicateField` or equivalent typed variant) when
`map.insert` returns `Some`.

**Structural lock:** a negative feeding a duplicated `content_manifest_id=` line must
fail with the typed code; enroll the envelope-decode negatives in the meta-lock
registry with a measured witness.

### ORD-HARD-169 — The content fingerprint is decorative: computed on every load, consumed by nothing, frozen nowhere

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble; hollow-evidence/R-29 class).
**Verification:** operator-verified (`content_fingerprint` is read only inside
`tracewake-content` — `load.rs`, `manifest.rs`, and two test files; zero consumers in
core or tui; replay reconstructs `ContentManifestId::new(context.content_version…)`
and never compares fingerprints; the only assertions are a `twf1-` prefix check and
first-load==second-load self-consistency; no frozen `twf1-` literal exists in the
repo).

**Responsible layers:** content manifest, replay-evidence honesty.

**Doctrine breached:** R-29 (an integrity-shaped artifact that witnesses nothing);
INV-018/019 direction (replay inputs include the content manifest — a fingerprint
that can drift to any value undetected gives a false impression that content identity
is pinned).

**Evidence:** `ContentManifest::new`/`stable_fingerprint` produce the fingerprint;
every consumer reads `manifest_id`/`fixture_id`/`content_version` only. A codec or
content change that reprices every fingerprint passes the whole suite untouched.

**Why existing gates miss it:** self-consistency assertions compare the value to
itself across loads; nothing compares it to a recorded expectation or to replay
state.

**Required correction:** freeze a per-fixture golden fingerprint table in
`golden_fixtures_run.rs` (fixture_id → `twf1-…` literal), updated only by explicit
reviewable edit; thread fingerprint verification into replay acceptance where the
manifest is already carried (the capstone asserts the loaded fixture's recomputed
fingerprint matches the recorded one).

**Structural lock:** the frozen table itself (any drift is a visible diff plus a red
test); a synthetic manifest with a mismatched fingerprint must fail the verification
seam.

### ORD-HARD-170 — The fingerprint payload is the re-serialized parsed struct plus path strings: raw-byte drift and secondary-file content are invisible to it

**Severity:** medium (sibling of `ORD-HARD-169`; research-corroborated CAS pattern —
fingerprint raw bytes, never the parsed struct).
**Verification:** operator-verified (`load_fixture_package` computes
`canonical_bytes = serialize_fixture(&fixture)` and fingerprints that; secondary
files contribute only their sorted path strings, never their bytes).

**Responsible layers:** content manifest.

**Doctrine breached:** INV-018/019 direction (data identity must cover what was
actually loaded); `docs/2-execution/08` (fixture manifests declare compatible
expectations).

**Evidence:** any raw-input detail the codec normalizes on round-trip is excluded
from the fingerprint by construction; editing a secondary file in a multi-file
package changes nothing in the fingerprint.

**Required correction:** fingerprint the raw validated `primary.bytes` plus every
secondary file's `(path, bytes)` pair; keep the struct-level canonical bytes as a
separate serialization-honesty check if desired.

**Structural lock:** a test mutating a secondary file's bytes must change the
fingerprint; a test asserting the fingerprint input includes the raw primary bytes
(synthetic raw-token difference with identical parsed struct must reprice).

### ORD-HARD-171 — Envelope decode is not version-gated at the decode layer: the schema-version gate lives only in `EventLog` append paths, and `deserialize_canonical` is `pub`

**Severity:** medium (precedent `ORD-HARD-144` — a perimeter enforced for a subset of
entry points).
**Verification:** operator-verified (`deserialize_canonical` parses
`event_schema_version` into `SchemaVersion` with no registry membership check;
`has_supported_schema_version` is consulted only by `events/log.rs::append`/
`append_deserialized`).

**Responsible layers:** kernel event log.

**Doctrine breached:** INV-020 direction (defense-in-depth on the read contract: a
future read path that constructs envelopes directly bypasses the gate).

**Evidence:** all current production read paths route through the gated log layer, so
no live false pass exists today; but the decode function is public, and a future
replay-prefix tool, debug import, or content-seeding path can decode a
`event_schema_v999` envelope without rejection.

**Required correction:** either gate `deserialize_canonical` itself against the
supported-version registry, or restrict it to `pub(crate)` and record the log layer
as the sole legal decode entry (owner decision; the spec recommends gating at decode —
cheaper than a visibility audit).

**Structural lock:** a negative decoding a `v999` envelope directly must fail; a
guard asserting every `deserialize_canonical` caller outside `log.rs` is test-only
(or that the function is `pub(crate)`).

### ORD-HARD-172 — guard_014's lock region is narrower than the relocated truth surface: `EmbodiedTruthSnapshot`, `EmbodiedPreflightSource`, and the sibling semantic-action builders are unscanned

**Severity:** medium (precedent `ORD-HARD-143` residue / `ORD-HARD-157` guard-scope
class).
**Verification:** operator-verified (`source_shape_errors` scans the
`EmbodiedProjectionSource` struct body, `from_sealed_context`'s body, and
`build_embodied_view_model`'s body only; `EmbodiedTruthSnapshot` and
`EmbodiedPreflightSource` have zero references in `anti_regression_guards.rs`;
`semantic_actions`/`phase3a_semantic_actions` are module-siblings of the private
`state` field).

**Responsible layers:** projections, `test_oracle`.

**Doctrine breached:** R-28 (the lock perimeter is a hand-picked subset of the truth
surface); 0024 §5 tier 1's intent (the embodied builder cluster cannot reach raw
truth).

**Evidence:** the 0024 refactor moved truth access into `EmbodiedTruthSnapshot::
from_physical_state` and `EmbodiedPreflightSource { state: &PhysicalState }` — both
outside the scanned regions. Because `semantic_actions` and `phase3a_semantic_actions`
live in the same module as the private `state` field, a future author can write
`preflight.state.items.values()…` inside `semantic_actions` (feeding the rendered
candidate set) and no banned substring fires in any scanned region. No truth read
exists there today (verified).

**Why existing gates miss it:** the synthetic re-adds a `state:` field to the struct
and injects `state.items` into the builder body — both inside scanned regions.

**Required correction:** extend the scanned region to the full embodied builder
cluster (`semantic_actions`, `phase3a_semantic_actions`, `with_validator_availability`)
and add a carrier census: `EmbodiedTruthSnapshot`/`EmbodiedPreflightSource`/
`SemanticActionPreflightContext` are the only symbols in `projections.rs` permitted
to name `PhysicalState`, asserted by parity so a new carrier fails enrollment.

**Structural lock:** a firing synthetic injecting `preflight.state.items` into
`semantic_actions`; the carrier-census parity negative.

### ORD-HARD-173 — `ORD-HARD-154`'s observation-time capture did not land: place label and carried-item attributes are read from live truth at every render, and the delivered staleness positive discriminates the wrong axis

**Severity:** medium (precedent `ORD-HARD-154`, low; elevated one band per the
`ORD-HARD-142` delivered-repair-unfalsifiable precedent — the repair ships a test
shaped like the required staleness positive that cannot catch the defect).
**Verification:** operator-verified (`TuiApp::current_view` calls
`EmbodiedTruthSnapshot::from_physical_state(&context, &self.state)` on each render,
re-reading the current place's `display_label` and each carried item's attributes
from live `PhysicalState`; `embodied_place_label_is_captured_before_truth_preflight`
mutates a *separate* preflight world, leaving the snapshot's source world unmutated).

**Responsible layers:** projections, TUI.

**Doctrine breached:** INV-067/093 direction (embodied surfaces derive from recorded
actor-known state, not live truth; a truth-changed-post-observation value renders
fresh); `ORD-HARD-154`'s required correction ("derive the label and item attributes
at observation/construction time into the sealed source").

**Evidence:** the snapshot struct made the truth read *explicit* but not
*observation-timed*: mutating `self.state`'s `display_label` (or a carried item's
`portable`) after the actor's last observation changes the next render. The delivered
test proves snapshot≠preflight divergence — an axis that cannot detect render-time
freshness.

**Required correction:** record the place label and carried-item `portable`/`source`
into actor-known facts at observation time (`record_current_place_perception`'s
boundary, the lawful truth→event seam) and derive the snapshot from those records;
`from_physical_state` then takes only body-state membership (carried-item IDs), which
is lawful body knowledge.

**Structural lock:** a true staleness positive: mutate `self.state.places[…].display_label`
and a carried item's `portable` *after* the recorded observation and assert the
render shows the recorded values; the `ORD-HARD-172` carrier census keeps
`from_physical_state`'s remaining reads enumerated.

### ORD-HARD-174 — The debug-gating guard is positional, not arm-complete: it asserts only that one substring precedes another

**Severity:** medium (precedent `ORD-HARD-153` residue).
**Verification:** operator-verified (`debug_command_gating_errors` finds
`if !app.debug_available()` and `match debug_command` and errors only when the first
position is not before the second; no variant enumeration, no early-return detection).

**Responsible layers:** TUI, `test_oracle`.

**Doctrine breached:** R-29 (the guard witnesses substring ordering, not the gating
property); INV-068/107 lock durability.

**Evidence:** a new world-advancing `DebugCommand` arm added to a *second* match in
`render_debug`, or an early `return` inserted between function entry and the gate,
passes the guard. The synthetic only deletes the gate string.

**Required correction:** derive the arm set from `enum DebugCommand` (parse variant
names from `input.rs`) and assert each appears exactly once, inside the single
`match debug_command`, strictly after the gate, with no `return` token between
function entry and the gate — the `apply_mutator_tokens_from` parity model already
proven in this codebase.

**Structural lock:** a parity synthetic adding an unrouted variant must fire; an
early-return synthetic must fire.

### ORD-HARD-175 — Cased perception witnesses still count anchor-function presence, not violation-detector-inspected sites

**Severity:** medium (precedent `ORD-HARD-146` — the direct residue, on the surviving
cased arms).
**Verification:** operator-verified
(`perception_visibility_other_emission_inspected_site_count` returns the count of
three named functions whose bodies are merely *present*
(`function_body_if_present(…).is_some()`); the violation detector is the per-line
lexical scan, which does not partition by those functions; the typed-place-visibility
witness counts `visibility_default` literal occurrences).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-29 (witness and violation predicate read different things).

**Evidence:** the lexical scan can go vacuous (e.g. its token table emptied or its
line feed broken) while the three anchor functions remain present; the witness holds
at 3.

**Required correction:** return the inspected-line count from
`perception_visibility_prose_branch_violations` itself (sites the per-line scan
actually examined) and use that as the witness; same for the marker-count witnesses.

**Structural lock:** a synthetic emptying the scan's inspected-line set while leaving
the anchor functions intact must drop the witness below minimum. Folds into the
`ORD-HARD-166` repair (one witness discipline for all entries).

### ORD-HARD-176 — `TuiApp::run_no_human_day` is a `pub` world-advancing method whose debug gate is a property of one call path

**Severity:** medium (calibration divergence from the reporting slice's high named in
the §4 preamble; precedent `ORD-HARD-104`/`153` — a derived gate consulted by one
consumer; no production-reachable bypass exists).
**Verification:** operator-verified (`pub fn run_no_human_day(&mut self) -> NoHumanDayReport`
on `TuiApp`; the only gated call site is the `DebugCommand::RunNoHumanDay` arm behind
`render_debug`'s gate; the parser maps only `debug run no-human-day` to it).

**Responsible layers:** TUI.

**Doctrine breached:** INV-068 boundary hygiene direction — the operator-proof
quarantine (`ORD-HARD-152`) is enforced at the command surface while the operation
itself remains an ungated public method (tests already call it ungated).

**Evidence:** the gate guards the dispatch path, not the operation; any future caller
(a new command arm, a harness, a panel) silently inherits no gate.

**Required correction:** make the gate intrinsic — `run_no_human_day` returns a typed
refusal (`Result<NoHumanDayReport, DebugUnavailable>`) when `!self.debug_available()`,
with test callers binding first (or using an explicit test-only constructor recorded
as such).

**Structural lock:** a refusal test invoking the *method* (not the command) unbound;
a guard enumerating world-advancing `pub fn` on `TuiApp` (those that mutate
`self.log`/`self.state`) and asserting each consults `debug_available` internally or
carries a rationale-bearing exemption.

### Low findings

Each low finding retains the full remediation obligation; the compact format lists
evidence → correction → lock inline. Verification status is tagged per finding.

**ORD-HARD-177 — The per-action positive census filters by a hand-picked five-variant
`ActionEffect` list** (test coverage / low; precedent `ORD-HARD-159`/`123`; the
reporting slice proposed medium — divergence named in the §4 preamble;
operator-verified at the `matches!` filter in `tui_acceptance.rs::
ordinary_action_positive_census_errors`). A future ordinary action with a new
`ActionEffect` variant is silently excluded and can ship with only negatives — the
exact defect the census exists to prevent. Correction: enumerate all effects with
rationale-bearing exemptions for the non-ordinary ones (or add an ordinary-life flag
to the registry definition). Lock: a synthetic ordinary action with an unlisted
effect and no positive must fail the census.

**ORD-HARD-178 — The policy generic-loop oracle re-derives the expected embodied flag
from `includes_in_embodied_context`, the method under test** (test oracle / low;
precedent `ORD-HARD-156`/`126`; operator-verified at reassessment —
`expected_embodied_presence` calls `includes_in_embodied_context` on the classified
output; the dedicated stale-path negative IS independent, so the row-mutation axis is
covered and only the generic per-kind check is tautological). Correction: derive the expected flag from a
literal truth table over `embodied_scope` × classifier flags. Lock: a synthetic
inverting `includes_in_embodied_context` must fail the generic loop.

**ORD-HARD-179 — The support-file `EventEnvelope` ban scans a hardcoded two-file
list** (test oracle / low; precedent `ORD-HARD-160`/`136`, R-28; operator-verified at
the `SUPPORT_GENERATIVE_RS`/`SUPPORT_MOD_RS` `include_str!` pair). A future
`tests/support/foo.rs` is unscanned. Correction: a parity census asserting the scan
list equals the `tests/support/` directory contents. Lock: the census fails when the
directory gains an unenrolled file.

**ORD-HARD-180 — The exemption scan-shape detector recognizes only `_errors(`/`_violations(`,
and a literal-rationale blanket bypass survives** (test oracle / low; precedent
`ORD-HARD-155`/`123`; operator-verified at `test_body_has_structural_scan_shape`).
An exempted test parking a real inline scan (`.filter(…).count()`, a `*_count()`
helper — the shape used by this file's own witnesses) escapes covering-lock
validation; any rationale containing the literal `"Historical acceptance-artifact
anchor audit"` skips validation entirely. Correction: broaden the detector to inline
violation-derivation shapes; replace the literal-rationale bypass with a structural
assertion. Lock: an inline-scan exemption without a covering lock must fail.

**ORD-HARD-181 — `is_schema_id_field_type` is a twelve-name hand-list beside a
derivable ID-newtype population** (content validation / low; precedent
`ORD-HARD-151`/`123`, R-28; operator-verified at the function; the twelve-name list contents
and the unlisted-newtype imports (`IntentionId`, `RoutineExecutionId`,
`CandidateGoalId`, `DecisionTraceId` in `schema.rs`) confirmed at reassessment). A schema field typed with an unlisted newtype (e.g.
`IntentionId`) is neither classified nor scan-demanded. Correction: assert the
recognizer equals the set of `*Id` newtypes referenced by `schema.rs` (census over
`ids` imports). Lock: the parity census fails on an unrecognized referenced newtype.

**ORD-HARD-182 — IDs embedded in `Location` values are never shortcut-marker-scanned**
(content validation / low; precedent `ORD-HARD-151`; operator-verified — zero
`location` references in `validate_id_shortcut_markers`). A marker-bearing ID solely
inside a `Location` referent reaches authored content unflagged (mitigated: a
dangling referent fails `bad_reference` first). Correction: scan the IDs yielded by
`Location` fields; teach the census to descend `Location`-typed fields. Lock: a
negative fixture with a marker ID only in a location referent.

**ORD-HARD-183 — The golden-bytes pin covers one hand-built Phase-1-shaped fixture,
and its perturbation assert is a tautology** (content serialization / low; precedent
`ORD-HARD-165`; operator-verified at the `perturbed[position] = b'I';
assert_ne!(perturbed, EXPECTED)` block — an assertion that cannot fail). Correction:
pin frozen golden bytes for a Phase-2A and Phase-3A representative (belief/routine/
workplace rows); delete the tautological assert. Lock: the committed per-scope golden
bytes.

**ORD-HARD-184 — `checksum_after` is a dead integrity field: serialized,
deserialized, never populated, never verified** (kernel event log / low; R-29 hollow
signal; operator-verified at the `checksum_after: None` producer default and the
decode path; the absence of any verifier corroborated by the slice sweep). Correction:
owner decision — populate at commit and verify per-event during replay (divergence
localization), or delete the field. Lock: if kept, replay asserts recomputed ==
recorded per world/agent event; if deleted, the round-trip goldens reprice once.

**ORD-HARD-185 — The TUI-local guards are enrolled in no lock registry** (test
oracle / low; precedent `ORD-HARD-144` enrollment discipline; operator-verified at
reassessment — zero lock-registry references anywhere in `tracewake-tui`). Deleting `debug_command_gating_errors`' test or the
render-reachability test is itself uncaught. Correction: a TUI-local mini-registry
(name census over `#[test]` fns in `app.rs` with required-member assertions),
paralleling the core registry. Lock: removing an enrolled TUI guard fails the census.

**ORD-HARD-186 — `ControllerMode::Debug` is non-discriminating: `debug_available_for`
grants debug to any non-`Detached` binding, so the `Debug` variant gates nothing**
(TUI / low; dead-variant/owner-decision class; operator-verified at the
`!matches!(… Detached)` predicate). The mode enum implies a discrimination the gate
does not honor; no invariant is breached (debug remains derived, non-diegetic,
boundary-only). Correction: owner decision — require `mode == Debug` for debug
availability (and have the TUI bind embodied/debug modes explicitly), or delete/
document the variant as reserved. Lock: a three-arm test pinning `debug_available_for`
against every `ControllerMode`, whichever way the decision goes.

**ORD-HARD-187 — The 0005 §12-recommended success-recovery variant is undemonstrated
and its staging is unrecorded: no supported recovery resolution token exists and no
test proves a successful FindFood replan→eat→continue chain** (content fixtures +
execution docs / low; precedents `ORD-HARD-152` (unrecorded staging) and
`ORD-HARD-159` (missing positive); the resolution-token mechanism itself
operator-verified — `canonical_mara_recovery_resolution=fail_only_empty_food_source`
authored in `no_human_day_001.rs` with the matching guard arm; the no-success-token
absence confirmed at reassessment — zero `recovered_via`/recovery-resolution matches
across the crates). The contract's minimum
("replans OR fails") is met and the fail-only pinning is a recorded fixture decision;
but the recommended success demonstration exists nowhere, the resolution vocabulary
admits no success value, and no execution doc records the deferral. Correction:
record the staging decision in `docs/2-execution/06` (success-recovery variant
staged, trigger named), or author the second resolution (`recovered_via_find_food`
token + guard arm + a fixture/test proving the chain). Lock: either the execution-doc
record (swept by the doc-honesty pass) or the new resolution arm's positive.

**ORD-HARD-188 — CI cache-key hygiene and the absent workflow-lint guard** (CI / low;
the reporting slice proposed medium with a green-without-gates poisoning mechanism
that is partially refuted — cargo re-fingerprints changed sources on fresh checkouts
and cache writes require an already-trusted run; divergence and refutation named in
the §4 preamble; cache block operator-verified — `actions/cache@v4` includes `target`
keyed only on `hashFiles('**/Cargo.lock')`). Residue: the key omits toolchain channel
and `Cargo.toml`, and no in-repo guard asserts workflow integrity properties at all.
Correction: add toolchain + `Cargo.toml` hashes to cache keys; add the
research-derived workflow-lint test over `.github/workflows/*` (four gate commands
verbatim; no pipes/`||` on gate steps; `permissions:` declared; third-party
non-`actions/*` uses SHA-pinned). Lock: the workflow-lint test itself, with a
synthetic-workflow negative.

**ORD-HARD-189 — CI is stricter than the documented gate contract, undocumented**
(CI + docs / low; precedent `ORD-HARD-152` record-the-decision class;
operator-verified at reassessment — workflow-level `RUSTFLAGS: "-D warnings"`,
`cargo test --workspace --locked`, and the twelve-invocation `lock-layer-gates` job
confirmed in `ci.yml`; zero mentions in doc 10).
CI adds `--locked` to `cargo test`, a global `RUSTFLAGS: -D warnings`, and a
twelve-invocation `lock-layer-gates` job — none named in `docs/2-execution/10` or the
four-gate contract. Safe-direction drift, but unreconciled authority. Correction: a
row in `docs/2-execution/10` enumerating the actual CI job set and flag posture.
Lock: fold into the `ORD-HARD-188` workflow-lint test (assert the documented superset
list matches the workflow jobs).

**ORD-HARD-190 — The scheduled mutation run's "pending" status never converts to a
required signal** (CI + evidence honesty / low; the 0022–0024 pending lineage;
operator-verified at reassessment — the 0024 report §13 records "still pending" and
no conversion rule exists in doc 10). The weekly `mutants-lock-layer` job fails on new misses and uploads
artifacts (healthy), but nothing requires a dated green run before `ORD-LIFE-CERT`
can clear "pending" — the status can persist indefinitely with no surfacing gate.
Correction: a phase-entry requirement consuming the latest scheduled-run result (a
dated green-run record in the acceptance artifact, asserted by the artifact parity
guard). Lock: the artifact wording guard fails if a report claims certification
readiness while the mutation row reads "pending".

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0024 layer.

1. **Witness discipline made executable:** no assertion-token-count witnesses —
   BehaviorAssertion rows witness by executed negatives, SharedScan rows by
   inspected-site counts returned from the scan body (`ORD-HARD-166`/`175`); the
   witness-shape ban itself carries a firing negative. Per the research corpus:
   second-order assurance is one committed known-bad fixture per guard, not a third
   meta-layer.
2. **Provenance-true kill sets:** taint keyed to argument data provenance, with a
   kill-set member that fires only through the provenance path (`ORD-HARD-167`).
3. **Kernel read path fail-closed:** duplicate-key rejection with a typed code
   (`ORD-HARD-168`); decode-layer version gating or `pub(crate)` restriction
   (`ORD-HARD-171`); the `checksum_after` decision (`ORD-HARD-184`).
4. **Manifest fingerprint honesty:** raw-bytes fingerprint payload including
   secondary files (`ORD-HARD-170`); frozen per-fixture golden fingerprints verified
   at replay acceptance (`ORD-HARD-169`); per-scope golden serialization bytes with
   the tautological assert deleted (`ORD-HARD-183`).
5. **Embodied truth-carrier census:** the scanned region extended to the full builder
   cluster with an enumerated-carrier parity census (`ORD-HARD-172`); observation-time
   capture for place label and carried-item attributes with true staleness positives
   (`ORD-HARD-173`).
6. **Gate depth and enrollment:** intrinsic gating on the world-advancing method
   (`ORD-HARD-176`); arm-complete derived debug-dispatch census (`ORD-HARD-174`);
   TUI-local lock registry (`ORD-HARD-185`); the `ControllerMode` decision pinned
   either way (`ORD-HARD-186`).
7. **Census derivation closures:** effect-complete positive census (`ORD-HARD-177`);
   truth-table policy oracle (`ORD-HARD-178`); directory-parity support-file ban
   (`ORD-HARD-179`); broadened exemption detector (`ORD-HARD-180`); ID-recognizer
   parity census and `Location` descent (`ORD-HARD-181`/`182`).
8. **CI and evidence honesty:** the workflow-lint guard with doctrine reconciliation
   (`ORD-HARD-188`/`189`); the mutation phase-entry conversion rule (`ORD-HARD-190`);
   the recovery-staging record (`ORD-HARD-187`); the 0025 acceptance artifact runs
   the checklist parity guard against itself; a finding premise that fails at
   implementation time is recorded as refuted.

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: extend R-29 with the two shapes this
  pass surfaced — the *assertion-token-count witness* (a meta-witness that counts
  `assert!` substrings: `ORD-HARD-166`) and the *prose-keyed taint* (provenance
  machinery whose sensitivity test reads the helper's tokens, not the argument's
  provenance: `ORD-HARD-167`). Extend R-28's Watch note with the *fingerprint-payload
  pitfall* (hashing the parsed struct instead of the raw bytes: `ORD-HARD-170`) and
  the *positional guard* (substring-order assertions standing in for arm-complete
  censuses: `ORD-HARD-174`).
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`: record
  the canonical-fixture recovery-resolution decision and the staging status of the
  §12-recommended success variant (`ORD-HARD-187`).
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`:
  enumerate the actual CI job set and flag posture; record the mutation phase-entry
  conversion rule (`ORD-HARD-189`/`190`).
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  add/update rows for the executable meta-witness discipline (`ORD-HARD-166`), the
  envelope duplicate-key rejection (`ORD-HARD-168`), the fingerprint verification
  seam (`ORD-HARD-169`/`170`), and the embodied carrier census (`ORD-HARD-172`) —
  landing with the capstone artifact, citing only landed symbols (the 0023/0024
  precedent).
- Record the `ORD-HARD-186` ControllerMode decision and the `ORD-HARD-171` decode-entry
  decision where the TUI/kernel conformance rows live, whichever way each resolves.
- No doctrine amendment; INV-001…INV-110 are applied, not changed. The INV-087
  decision (`ORD-HARD-095`) remains deferred; nothing in this spec pre-decides it.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0025_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The executable witness discipline: no assertion-token-count or anchor-presence
   witness anywhere in the meta-tier; per-row executed negatives or inspected-site
   counts, with the present-but-vacuous synthetic firing (`ORD-HARD-166`/`175`).
2. The provenance-true kill set with its raw-token-free member firing only through
   the provenance path (`ORD-HARD-167`).
3. The envelope duplicate-key rejection negative through `deserialize_canonical`, the
   decode-layer version decision and its negative, and the `checksum_after` decision
   (`ORD-HARD-168`/`171`/`184`).
4. The raw-bytes fingerprint payload with the secondary-file repricing test, the
   frozen per-fixture fingerprint table, the replay-acceptance verification seam, and
   the per-scope golden bytes (`ORD-HARD-169`/`170`/`183`).
5. The widened embodied lock region with the `semantic_actions` injection synthetic
   firing, the carrier census, and the true staleness positives for place label and
   carried-item attributes (`ORD-HARD-172`/`173`).
6. The intrinsic method gate with its unbound-method refusal test, the arm-complete
   debug-dispatch census, the TUI-local registry, and the recorded `ControllerMode`
   decision (`ORD-HARD-176`/`174`/`185`/`186`).
7. The census closures: effect-complete positive census, truth-table policy oracle,
   support-directory parity, broadened exemption detector, ID-recognizer parity,
   `Location` descent (`ORD-HARD-177`–`182`).
8. The workflow-lint guard with the doctrine-reconciliation row, the cache-key
   hygiene diff, the recovery-staging record, and the mutation phase-entry rule
   (`ORD-HARD-187`–`190`).
9. Confirmation that every finding's premise still held at implementation time
   (all twenty-five findings were operator-verified pre-implementation — nineteen
   fully at audit triage; `ORD-HARD-178`, `185`, `189`, `190` and the tagged
   sub-claims of `181`/`187` re-verified at source during spec reassessment at the
   same baseline); a premise that nonetheless fails is recorded as refuted, not
   silently dropped.
10. The risk-register, execution-doc, and conformance-index diffs, quoted (§6).
11. An updated `EMERGE-OBS` ledger derivation over the corrected surface
    (measurement only, no thresholds).
12. The scheduled mutation run's latest dated result, or its still-pending status
    restated honestly — plus the new phase-entry conversion rule's first evaluation.
13. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`;
    not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core;
  tui on core + content. No new dependencies, dev or production.
- Verification posture: every finding is operator-verified at its load-bearing
  symbols at `9e33d7a` — nineteen fully at audit triage; `ORD-HARD-178`, `185`,
  `189`, `190` and the tagged sub-claims of `181`/`187` re-verified at source during
  spec reassessment at the same baseline (all confirmed, none refuted; the 0024
  precedent) — so no re-verification step is owed; a finding whose premise
  nonetheless fails at implementation time is recorded as refuted in the acceptance
  artifact, not silently dropped.
- `ORD-HARD-173` ordering note: land the observation-time capture before extending
  the `ORD-HARD-172` carrier census, so the census enumerates the already-shrunk
  truth reads; reprice any goldens once, honestly, in the same ticket group.
- Four findings require owner decisions (`ORD-HARD-171` decode-entry posture;
  `ORD-HARD-184` populate-or-delete; `ORD-HARD-186` ControllerMode semantics;
  `ORD-HARD-187` staging-or-build): make each decision in its ticket, record it per
  §6, and implement accordingly; do not silently choose.
- Recommended ticket ordering:
  1. `ORD-HARD-166` + `175` + `180` (executable witness discipline — first, so every
     subsequent ticket's new locks are born under repaired meta-rules) + §6
     risk-register corrections.
  2. `ORD-HARD-167` (perception kill-set provenance).
  3. `ORD-HARD-168` + `171` + `184` (kernel envelope read path fail-closed).
  4. `ORD-HARD-169` + `170` + `183` (manifest fingerprint honesty + golden bytes;
     batch repricing once).
  5. `ORD-HARD-172` + `173` (embodied carrier census + observation-time capture; see
     ordering note).
  6. `ORD-HARD-174` + `176` + `185` + `186` (TUI gate depth, enrollment, mode
     decision).
  7. `ORD-HARD-177` + `178` + `179` + `181` + `182` (census/oracle derivation
     closures).
  8. `ORD-HARD-187` + `188` + `189` + `190` (recorded decisions: recovery staging,
     CI hygiene + workflow-lint guard, doctrine reconciliation, mutation phase-entry
     rule) + the §6 execution-doc records.
  9. The acceptance artifact lands last, measuring the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-166`'s executed-negative routing may be expensive.** Forty
  BehaviorAssertion rows executing their negatives at witness time could slow the
  guard suite; the honest fallback is converting rows to SharedScans with measured
  inspected-site counts where execution is impractical — never back to token
  presence. If a row's negative cannot be executed in-process, that row gets a
  rationale-bearing exemption validated by the repaired `ORD-HARD-180` detector.
- **`ORD-HARD-173`'s observation-time capture touches the perception seam.**
  `record_current_place_perception` is the lawful truth→event boundary; the capture
  must extend what is *recorded there*, not add a new truth read elsewhere. If the
  capture proves to need a Phase-3B-scale observation model, the honest fallback is
  the recorded-staleness-positive-only posture with an execution-doc entry — not a
  silent re-scope.
- **`ORD-HARD-169`'s frozen fingerprints will reprice on any codec or fixture edit.**
  That is the point; batch repricing with ticket group 4 and never widen via wildcard.
- **Four owner decisions** (`ORD-HARD-171`/`184`/`186`/`187`) — recommended
  resolutions are specified but they are decisions, not mechanical fixes.
- **The `ORD-HARD-095` INV-087 deferral remains open** (bind-time perception). This
  pass's controller sweep independently rediscovered it, which suggests the deferral
  should acquire a decision deadline (Phase 3B entry) rather than rolling forward
  indefinitely; recorded here as an observation, not a finding.
- **Next-iteration assessment (the recurring question):** a product-behavior
  foundation defect was found at the letter (`ORD-HARD-168`), so a fifteenth pass is
  warranted by the lineage's own rule, and it should be a **strictly scoped
  verification of 0025's deliverables** — above all whether the executable witness
  discipline (`ORD-HARD-166`) eliminated token-count witnesses for real, and whether
  the provenance kill-set member genuinely fires through the provenance path
  (`ORD-HARD-167`). The honest trend, stated plainly: the cognition firewall is clean
  eight passes running; the 0005 contract has held with zero product regressions for
  the entire lineage; product-behavior findings are weakening monotonically (0023: a
  display-surface truth read; 0024: an absent mandated gate, high; 0025: a
  tamper-edge acceptance gap no in-repo writer can trigger, medium); and the supply
  of never-scoped surfaces is now **exhausted** — all three corners 0024 §9 named
  have been swept once, and every production crate surface has now been audited at
  least once. Per the standing research corpus (capture-recapture overlap, marginal
  defect yield, the Tricorder graduation criterion), if the fifteenth pass verifies
  0025 clean-or-low-only, the defensible cadence is the one 0023 and 0024 already
  recommended: drop per-pass audits, move to phase-entry audits (Phase 3B / Phase 4
  boundaries), keep the four gates + mutation CI + the completed meta-locks as the
  continuous layer, and schedule guard-consolidation hygiene at the same boundaries.
  The recurring counter-signal to weigh honestly: three consecutive passes have found
  the witness-repair-carries-the-defect shape (`122` → `141`/`142` → `166`/`167`),
  so the fifteenth pass's verification of ticket group 1 and 2 must be adversarial,
  not confirmatory — if a fourth instance appears, the meta-tier design itself (not
  its instances) is the defect, and consolidation into fewer, executed-negative-only
  guards should precede any cadence relaxation.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive
  rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability / evidence-honesty / doc-code coherence) and names responsible
  layers from the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding is operator-verified — nineteen at audit
  triage, six re-verified at source during spec reassessment — with the consolidated
  posture in §8.
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression.
- [x] Verified-holding items from 0014–0024 are recorded to prevent re-litigation;
  the 0005 feature-contract verification is recorded; severity calibrations that
  diverge from lineage precedent or from the reporting audit slice are named in §4's
  preamble (`ORD-HARD-166`–`170`, `173`, `176`, `177`, `188`).
- [x] The `ORD-HARD-095` rediscovery is mapped to the standing deferral, not
  re-minted.
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved.
- [x] Scope stays within the Phase 3A ordinary-life surface, its kernel
  envelope/serialization, manifest, controller, and CI boundaries (final blind
  sweeps, prescribed by 0024 §9), and its lock/evidence layer.

## Outcome

Completed: 2026-06-13

Implemented the full `0025PHA3AMETWIT` ticket series and archived tickets
`0025PHA3AMETWIT-001` through `0025PHA3AMETWIT-011`. The series repaired the
meta-witness execution discipline, provenance-keyed perception taint, event
envelope duplicate/version fail-closed behavior, manifest raw-byte fingerprint
honesty, embodied observation capture, TUI debug-gate depth, census/oracle
closures, recovery-staging record, CI workflow hygiene, and mutation phase-entry
evidence rule. The capstone added
`reports/0025_ord_life_cert_scoped_acceptance.md`, 0025 conformance-index rows,
and the `acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors`
parity guard with pending-mutation certification-wording enforcement.

Implementation commits:

- `617a5b1` `Complete 0025PHA3AMETWIT-001 meta-witness discipline`
- `593658b` `Complete 0025PHA3AMETWIT-002 perception provenance guard`
- `b75641e` `Complete 0025PHA3AMETWIT-003 envelope fail-closed read path`
- `5841a1f` `Complete 0025PHA3AMETWIT-004 manifest fingerprint honesty`
- `c61243d` `Complete 0025PHA3AMETWIT-005 embodied observation capture`
- `214a42a` `Complete 0025PHA3AMETWIT-006 TUI debug gate depth`
- `045653c` `Complete 0025PHA3AMETWIT-007 census oracle closures`
- `5a709bd` `Complete 0025PHA3AMETWIT-008 content ID scan closure`
- `f7b9d05` `Complete 0025PHA3AMETWIT-009 recovery staging record`
- `133d2b1` `Complete 0025PHA3AMETWIT-010 CI workflow guards`
- `0d834bf` `Complete 0025PHA3AMETWIT-011 capstone artifact`

Verification:

- `cargo test -p tracewake-core --test ci_workflow_guards`
- `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`

This remains scoped evidence toward `ORD-LIFE-CERT`; it is not full-project
certification, not Phase 4 entry, and not `FIRST-PROOF-CERT`. The scheduled
mutation baseline remains honestly recorded as pending in the local evidence
surface; a dated green scheduled mutation run is still required before any
readiness claim that depends on that row.
