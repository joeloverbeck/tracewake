# 0047 foundational hardening research report — seventh pass

## 1. Verdict

**Static verdict:** the spec-0047 loaded-world / time-control / TUI-authority surface at commit `2720167a0d1a60ac809ae1c670539a1846df031d` is **not foundationally conformant**. The sixth remediation line made real progress, but the authority class is not closed.

The decisive code reasons are:

1. **Bootstrap authority remains forgeable through a public validated-content alias.** `PhysicalState::from_validated_seed_parts` and `AgentState::from_validated_seed_parts` are now `pub(crate)`, but their `#[doc(hidden)] pub fn from_validated_content_parts(...)` wrappers remain externally callable. `ValidatedLoadedWorldBootstrap::from_validated_content(...)` is public, `LoadedWorldRuntime::from_bootstrap(...)` is public, and `tracewake-content::load::LoadedFixture` exposes public authoritative state fields plus a public `into_runtime_bootstrap(...)` handoff. This leaves an external-crate route to fabricate a “validated” loaded-world bootstrap without traversing the content validation gate.
2. **Debug authority is still inducible from the ordinary TUI command loop.** The direct core command vulnerability was repaired: `RuntimeCommand::bind_debug_controller(...)` now requires `DebugSessionAuthority`, and the token constructors are crate-private. But `LoadedWorldRuntime::local_operator_debug_authority()` publicly mints an authority without an externally supplied operator proof, `TuiApp::bind_debug_actor(...)` calls it, and `bind-debug <actor_id>` is parsed and executed from the normal command-loop input surface. That keeps the embodied/debug boundary inside ordinary play instead of at an explicit operator entrypoint.

**Acceptance-taxonomy verdict:** the fail-closed acceptance taxonomy is **not sound in the current post-0055 doctrine**. It does recompute `overall_result` from a machine-readable block rather than blindly trusting `overall_result`, which is progress. But that computation is still a shallow consistency check over self-authored scalar rows; it does not know the 0055 `solo-maintainer-compensating-control` posture, accepts only the pre-0055 `independent-review` / `last-push-required-reviewer` values, and rejects or forces falsification of the now-ratified solo-maintainer value. Its wording guard is a phrase denylist, not a closed grammar. CI does force changed acceptance artifacts through the parser for closure-path changes, but the parser being forced is not doctrine-complete.

**0055 no-weakening verdict:** the doctrine edits in `docs/1-architecture/13` and `docs/2-execution/10` **did not weaken the tier as written**. They explicitly say the solo-maintainer mode is not a weaker behavioral-evidence rule, preserve the invalid-pass conditions for open/pending/unbounded/historical/unproven evidence, preserve rejection of self-authored-only behavioral evidence, and scope the exception to the human-approval dimension. The weakness is implementation drift in the taxonomy code, not the 0055 doctrine text.

**Higher-tier amendment verdict:** I do **not** recommend a Tier-0 constitutional amendment. The constitution already governs product behavior, causal authority, replay, actor-known cognition, debug quarantine, and harsh acceptance at the product-feature altitude. “Independent acceptance for a solo-operated project” belongs in architecture/execution acceptance doctrine, not in constitutional invariants, unless a future governance decision attempts to let process convenience weaken behavioral evidence. No new invariant, gate code, risk ID, or glossary term should be minted. Lower-tier doctrine and the acceptance template need synchronization only after executable closure.

**Another iteration needed?** Yes. A seventh remediation iteration is needed because there are live code-authority defects and a live process-integrity defect. It should be narrowly scoped: close the validated-content alias, move debug authority creation outside ordinary embodied input, update the taxonomy to the settled 0055 posture, and force adversarial fixtures/mutation coverage over the new public-boundary proofs.

---

## 2. Disposition table

| Finding | Primary target | Classification | One-line basis |
|---|---|---|---|
| F7-01 | `crates/tracewake-core/src/state.rs`, `runtime/session.rs`, `crates/tracewake-content/src/load.rs`, `schema.rs`, `manifest.rs` | **Violation** | Validated seed constructors were sealed, but equivalent public validated-content constructors and public loaded-fixture fields still let an external crate fabricate authoritative seed aggregates and a `ValidatedLoadedWorldBootstrap`. |
| F7-02 | `crates/tracewake-core/src/runtime/session.rs`, `debug_capability.rs`, `crates/tracewake-tui/src/{app,input,run,debug_panels}.rs` | **Violation / hardening gap** | Direct core debug binding now requires a token, but ordinary TUI text input can still mint debug authority through `local_operator_debug_authority()` and `bind-debug`. |
| F7-03 | `crates/tracewake-core/tests/support/acceptance_status_manifest.rs` | **Process-integrity gap** | The manifest parser recomputes from scalar rows, but it does not implement the 0055 `solo-maintainer-compensating-control` posture and cannot prove the compensating-control set from the artifact. |
| F7-04 | `crates/tracewake-core/tests/acceptance_artifact_wording.rs` | **Process-integrity / evidence-honesty gap** | The wording guard is a denylist of overclaim phrases; it is not a closed verdict grammar and is paraphrase-bypassable. |
| F7-05 | `acceptance_status_manifest.rs`, `acceptance_artifact_wording.rs`, `.cargo/mutants.toml`, CI mutation lanes | **Mutation-coverage gap** | The taxonomy guards have synthetic tests, but this static pass found no independent mutation-sensitive proof that mutants of the parser/guard itself are killed. |
| F7-06 | `.github/workflows/ci.yml`, `ci_workflow_guards.rs`, acceptance template | **Forcing-function gap** | CI forces changed closure artifacts through the parser, but the forced parser is stale and shallow; the 0054 acceptance can still be self-consistent while live code defects persist. |
| F7-07 | `tests/negative-fixtures/external_crate_cannot_*`, `negative_fixture_runner.rs`, `public-boundary-conformance` | **Hardening gap** | The F6 negative fixtures attack retired/narrow symbols; they do not attack the live validated-content alias or ordinary TUI debug-authority path. |
| Present-P1 | `runtime/receipt.rs`, `view_models.rs`, direct debug token constructors, food-source projection/TUI tests | **Present property** | 0054 genuinely sealed the wait receipt, interval summary internals, direct debug token construction, and the public food-source replacement witness, subject to static-survey limits. |

---

## 3. Method & provenance ledger

### Authority order applied

I applied the repository’s documented authority order: foundation governs architecture; architecture governs execution; execution governs reference/spec history and implementation. The live conformance target was the repository state at exact commit `2720167a0d1a60ac809ae1c670539a1846df031d`. The uploaded manifest was used only as a path inventory. It was not treated as file-content evidence.

The brief states that `crates/`, `.cargo/`, and `tests/` are byte-identical between spec 0054’s implementation commit under test and this baseline. I treated that as user-supplied control material, not as an independently rerun diff. The consequence for this pass is that the 0054 acceptance describes the source shape under audit, but its `closed` rows, `overall_result: pass`, and `mutation_survivors: none` remain unverified claims until re-derived from code.

### Static-survey limitation

No `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, replay/golden lane, `cargo-mutants`, GitHub API, or live ruleset command was run. Current test strength, mutation status, governance enforcement, and pass/fail status are preliminary static judgments about source shape, public API reachability, data flow, and CI/workflow text. Any command result or mutation count quoted from archived artifacts is a historical artifact claim only.

### Mandatory exact-commit acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 2720167a0d1a60ac809ae1c670539a1846df031d
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.open exact raw.githubusercontent.com URLs; container.download was used only for local line-reading copies after the same exact raw URL form was established
Requested file count: 210
Successfully verified file count: 210
Fetched repository files: complete exact-URL list in Appendix A
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

No target-repository claim below depends on repository search snippets, branch fetches, default-branch metadata, prior chat memory, or another repository. Ordinary text inside a validly fetched file was treated as file content, not as provenance evidence.

---

## 4. Re-verified present properties from 0054

0054 did not fail wholesale. Several properties are now present and should be preserved rather than re-commissioned.

### Present-01 — the exact F6 seed-part constructors are no longer public

`PhysicalState::from_validated_seed_parts(...)` is `pub(crate)` in `crates/tracewake-core/src/state.rs`, and `AgentState::from_validated_seed_parts(...)` is also `pub(crate)`. The F6 negative fixture that tries to call those exact symbols from an external crate now targets a real private boundary.

This present property is narrow. F7-01 exists because equivalent public validated-content wrappers remain.

### Present-02 — the wait receipt no longer exposes `WorldAdvanceResult` internals

`RuntimeReceiptKind::OneTickAdvanced` carries `OneTickRuntimeReceipt`, not `WorldAdvanceResult`. `OneTickRuntimeReceipt` has private fields and limited public accessors: `advanced()`, `appended_event_count()`, and `actor_known_interval_summary()`. The conversion from `WorldAdvanceResult` is internal. The external negative fixture that tries to destructure raw tick/frontier/due-work internals attacks the live public boundary and should fail for the right reason.

### Present-03 — direct debug-token construction and direct core debug binding are sealed

`DebugCapability::mint()` and `DebugSessionAuthority::mint()` are crate-private; public `for_test()` is feature/test gated. `RuntimeCommand::bind_debug_controller(...)`, `run_no_human_day(...)`, and `debug_view(...)` require a `DebugSessionAuthority`. That closes the direct F6-03 core API path. F7-02 is the residual operator/TUI path, not a resurrection of the same direct constructor bug.

### Present-04 — actor-known interval summaries are sealed as products

`TypedActorKnownIntervalSummary` has private fields, a crate-private constructor, and production-visible actor-safe accessors. Exact tick/frontier/stop-reason accessors are test/support gated. The external fixture that attempts a struct literal, direct constructor, and temporal accessors attacks the correct public boundary.

### Present-05 — embodied temporal/debug fields are no longer direct client state

The relevant temporal/debug authority fields in `EmbodiedViewModel` are private or crate-owned rather than public mutable client state. The TUI stores/renders the product rather than constructing actor-known interval summaries directly.

### Present-06 — the food-source replacement witness is public-facing

`actor_known_food_sources_for_context(...)` collapses multiple actor-known food-source facts per supply and applies the documented supersession rule: serving knowledge supersedes source-less knowledge, source-less knowledge does not replace source-bearing knowledge, and equal serving-knowledge classes use source-key ordering. The core test `food_source_projection.rs` covers source-bearing empty food superseding source-less availability and source-less food not replacing known empty knowledge through embodied action availability. The TUI acceptance file adds public actor-known witnesses using `TuiApp::from_golden(...)` and public semantic action menus. That is the right kind of witness: public embodied behavior, not a private helper assertion.

### Present-07 — CI has a real topology for the standing surface gate

The workflow has `lock-layer-gates`, `public-boundary-conformance`, `full-surface-mutation-trigger`, `mutation in-diff (lock layer)`, and scheduled `mutants-lock-layer` / `mutants-lock-layer-reconcile` jobs. It also has an “Ingest changed acceptance artifacts” step that rejects report/spec closure changes without a changed acceptance artifact and passes each changed artifact through `TRACEWAKE_ACCEPTANCE_ARTIFACT` into the manifest parser. This is a real forcing topology. The defect is that the forced taxonomy is stale and incomplete.

---

## 5. Per-finding sections — code surface

### F7-01 — Public validated-content constructors leave bootstrap authority forgeable

#### Foundational driver

The controlling doctrine is the causal/replay and TUI/client authority stack:

- `INV-009` through `INV-020`: meaningful state changes require events, causal ancestry, and deterministic replay.
- `INV-061` through `INV-063`: authored seed material may create initial machinery and prehistory only with structured provenance.
- `INV-069`: the TUI/client must not own simulation rules or mutate world state through player-only paths.
- `INV-098`: runnable features are complete only when caused, replay-safe, and regression-tested.
- Architecture `04` and execution `05`: the loaded-world handoff must flow through validation and the core-owned world-step boundary, not caller-injected authoritative aggregate lists.

#### Current code state

The repaired part is real:

- `PhysicalState::from_validated_seed_parts(...)` is `pub(crate)`.
- `AgentState::from_validated_seed_parts(...)` is `pub(crate)`.

The residual public path is still live:

- `PhysicalState::from_validated_content_parts(...)` is `#[doc(hidden)] pub fn` and delegates straight to `from_validated_seed_parts(...)`.
- `AgentState::from_validated_content_parts(...)` is `#[doc(hidden)] pub fn` and delegates straight to `from_validated_seed_parts(...)`.
- `ValidatedLoadedWorldBootstrap::from_validated_content(...)` is public and accepts already-materialized `PhysicalState`, `AgentState`, `EventLog`, and `EpistemicProjection`.
- `LoadedWorldRuntime::from_bootstrap(...)` is public.
- `LoadedFixture` in `tracewake-content` is public, has public fields for canonical world, agent state, projection, and seed event log, and offers public `into_runtime_bootstrap(...)`.
- `ContentManifest` has public identity/fingerprint fields and a public `new(...)`, which makes fabricated `LoadedFixture` construction easier.

`#[doc(hidden)]` is not a Rust visibility boundary. It can hide documentation; it does not make a `pub fn` inaccessible to another crate. Rust’s actual access rule is item visibility, not documentation visibility.

#### Conformance verdict

**Violation.** The implementation re-sealed the exact seed-part symbol names that F6 attacked, but it did not seal the authority class. External code can still construct arbitrary authoritative aggregate state, wrap it as “validated content,” and hand it to the runtime constructor.

This violates the loaded-world bootstrap boundary because the proof object is no longer proof of validation; it is just a public function name.

#### Required remediation

Code home:

1. Remove public reachability of `PhysicalState::from_validated_content_parts(...)` and `AgentState::from_validated_content_parts(...)`. They must become crate-private, feature-gated test-only, or eliminated.
2. Do not replace them with another public raw-aggregate constructor. No backwards-compatible shim or alias.
3. Replace the cross-crate handoff with a sealed artifact that external crates cannot forge. Plausible implementation choices:
   - move content validation/materialization into `tracewake-core` so the authoritative state constructors are crate-private; or
   - make `tracewake-content` return a core-owned sealed `ValidatedContentArtifact` created only by a core validation entrypoint; or
   - change `tracewake-content` to pass raw authored data/events to a core validation/materialization function that owns the final aggregate construction.
4. Make `LoadedFixture` fields private if `LoadedFixture` remains the proof vehicle. Construction must be possible only through the validating loader.
5. Keep `LoadedWorldRuntime::from_bootstrap(...)` public only if the bootstrap type is genuinely unforgeable by external crates.

Doc home after executable closure:

- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`: clarify that “validated loaded-world bootstrap” means a proof object minted by the validation/materialization boundary, not any public function named `from_validated_*`.
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`: state that external crates must not be able to seed runtime actor/process eligibility or authoritative aggregates by public constructors.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`: record that clients consume runtime products and typed action attempts only.
- Reference `00`/`01` after closure only: update R-27/R-28/R-29 status/navigation, minting no new risk ID.

#### Strongest practical anti-regression guard

Add an external-crate negative fixture that attacks the live symbols, not retired ones:

- attempt to call `PhysicalState::from_validated_content_parts(...)`;
- attempt to call `AgentState::from_validated_content_parts(...)`;
- attempt to fabricate `ContentManifest` and `LoadedFixture { canonical_world, canonical_agent_state, epistemic_projection, seed_event_log, ... }` and call `into_runtime_bootstrap(...)`;
- attempt to pass that bootstrap to `LoadedWorldRuntime::from_bootstrap(...)`.

The expected failure must be a privacy/unconstructability error on the authority proof, not a stale method-name error. The fixture belongs under the existing `tests/negative-fixtures/external_crate_cannot_*` pattern and must be run by `negative_fixture_runner.rs` and `public-boundary-conformance`.

Add behavior tests proving the real loader path still succeeds and derives due-work from core-owned scheduler discovery. Add mutation/in-diff coverage for `state.rs`, `runtime/session.rs`, `tracewake-content/src/{load,schema,manifest,validate}.rs`, and any new sealed proof type. Source-text guards may verify the fixture is wired, but they are topology alarms only.

#### Evidence-honesty check

Closure is not proven by hiding functions with `#[doc(hidden)]`, by renaming `from_validated_seed_parts`, or by a negative fixture that attacks only the old name. Closure is proven only when an external crate cannot create the runtime bootstrap except through the validating loader, and when a public positive path still loads real content.

---

### F7-02 — Debug authority is still inducible by ordinary TUI input

#### Foundational driver

The controlling doctrine is the embodied/debug split:

- `INV-067`: embodied mode shows actor-known reality.
- `INV-068`: debug mode is visibly non-diegetic and must not be confused with embodied knowledge.
- `INV-069`: the TUI must not implement simulation rules or bypass validators.
- `INV-093`: actor-knowledge leakage is high severity.
- `INV-107`: debug omniscience is quarantined and cannot create actor knowledge or future plans unless explicitly marked non-ordinary.
- Architecture `10` and execution `07`: debug products are operator/debug surfaces; embodied play stores/renders core products read-only.

#### Current code state

The fixed part:

- `DebugCapability::mint()` and `DebugSessionAuthority::mint()` are crate-private.
- `RuntimeCommand::bind_debug_controller(...)` requires `DebugSessionAuthority`.
- The negative fixture for direct public bind without token attacks that closed path.

The residual path:

- `LoadedWorldRuntime::local_operator_debug_authority(&self) -> DebugSessionAuthority` is public and mints a token without a separate operator proof.
- `TuiApp::bind_debug_actor(...)` calls `self.runtime.local_operator_debug_authority()` and then binds debug mode.
- `parse_input(...)` accepts `bind-debug <actor_id>` as a normal `UiCommand::BindDebugActor`.
- `run_command_loop(...)` dispatches that command from ordinary text input.
- Debug panels and debug commands become available once the command-loop bind has succeeded.

#### Conformance verdict

**Violation / hardening gap.** Direct core command authority is sealed, but debug authority is still inducible by an unprivileged ordinary command-loop input. If `bind-debug` is intended as an operator action, the code needs an explicit operator entrypoint outside embodied input, not a command available through the same parser as `wait`, `look`, or semantic actions.

#### Required remediation

Code home:

1. Remove `LoadedWorldRuntime::local_operator_debug_authority()` from the public ordinary runtime API, or make it require an unforgeable operator-mode capability that cannot be produced by the command parser.
2. Split TUI launch modes:
   - ordinary embodied TUI: no `bind-debug` command, no debug authority mint, no debug panels;
   - explicit operator/debug launch: carries an operator capability created at launch/session setup, visibly non-diegetic, and never confused with embodied play.
3. Ensure the parser cannot turn ordinary text into debug authority. If a debug command is typed in ordinary mode, it must fail actor-safely and must not mutate controller bindings.
4. Keep `RuntimeCommand::bind_debug_controller(...)` token-gated.

Doc home after executable closure:

- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`: no amendment needed, but cite it as controlling doctrine.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`: clarify the operator launch boundary if the code introduces one.
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`: record the negative ordinary-mode proof and positive operator-mode proof.

#### Strongest practical anti-regression guard

Add a public TUI test that runs a normal command-loop script containing `bind-debug actor_tomas` followed by `debug ...` and proves debug is not enabled and no debug view is produced. Add a positive test for explicit operator launch, proving debug remains reachable only through that launch path and visibly non-diegetic.

Add an external-crate fixture that attempts to call any public runtime/TUI API capable of minting `DebugSessionAuthority` without an operator proof. Add mutation coverage over `debug_capability.rs`, `runtime/session.rs`, `runtime/command.rs`, `tui/src/{app,input,run,debug_panels,launch}.rs`.

#### Evidence-honesty check

Closure is not proven by token-gating `RuntimeCommand::bind_debug_controller(...)` alone. It is proven only when the ordinary TUI parser cannot induce the token or any equivalent debug binding.

---

## 6. Residual disposition — `food_source_fact_supersedes`

The F6-07 food-source residual is **static-present**.

Repository evidence shows the replacement rule is implemented in the embodied projection surface, not only in a private unit helper. The `actor_known_food_sources_for_context(...)` path selects one fact per food supply and calls `food_source_fact_supersedes(...)`; the rule prefers source-bearing serving knowledge over source-less knowledge and then uses source-key ordering for deterministic selection. Core tests exercise action availability through `build_embodied_view_model(...)`; the public TUI acceptance exercises `TuiApp::from_golden(...)`, `bind_actor(...)`, and the semantic action menu. That is the right public witness because the observed behavior is actor-known embodied availability, not direct mutation of private projection internals.

I make no current equivalence claim about any historical mutant. The 0054 acceptance’s “2679 caught / 766 unviable / 0 missed / 0 timeout” line is historical artifact evidence only. The current claim here is static: the source now has a public witness that would be able to catch the routed-forward pattern if actually run.

Recommended hardening:

- Keep the public TUI witness and core actor-known witness.
- Add an explicit mutation-target note in `.cargo/mutants.toml` / baseline registry only if a future actual run shows a survivor or unviable mutant.
- Do not allow future reports to treat the historical 0054 mutation count as current green evidence.

---

## 7. Structural anti-regression / enforced standing gate — code surface

The seam reopened because the remediation line closed named holes rather than the authority class.

F6-01’s fixture attacks `from_validated_seed_parts`; the live public alias is `from_validated_content_parts` plus public `LoadedFixture` construction. F6-03’s fixture attacks direct `DebugSessionAuthority::mint()` and direct `RuntimeCommand::bind_debug_controller(...)`; the live inducible path is the TUI command loop. These are classic negative-fixture drift failures: the fixture name says the right thing, but the fixture body no longer attacks the strongest reachable public route.

The standing gate should be hardened as follows, without minting a new gate code:

1. **Public-boundary conformance must compile-fail the live authority class.** Add negative fixtures against the current public symbols and route them through the existing `negative_fixture_runner.rs` and `public-boundary-conformance` job.
2. **Behavioral TUI tests must prove ordinary-mode debug noninducibility.** This cannot be a source-text guard; it must submit commands through the public loop and prove debug stays unavailable.
3. **Mutation perimeter must include the corrected seam.** The in-diff and standing perimeter already cover most of the needed files. Preserve that coverage and add any new sealed-proof/operator-mode files to `.cargo/mutants.toml` and the CI guard list.
4. **Source-text guards stay as topology alarms.** `ci_workflow_guards.rs` can prove jobs and path globs are wired. It cannot prove bootstrap unforgeability, debug quarantine, replay semantics, or mutation closure.
5. **CI acceptance artifact ingestion is useful but must be doctrine-complete.** A forced parser that cannot represent the current doctrine is not a fail-closed authority.

Placement relative to the certification ladder: put this under the existing lock-layer / public-boundary / mutation lane posture described in `docs/2-execution/03` and `docs/2-execution/10`. Do not mint a new gate code.

---

## 8. Fail-closed acceptance taxonomy audit

### 8.1 Recompute vs. validate

The parser does recompute a result from the status block. `validate_status_manifest(...)` extracts the block, parses it, computes a result, and rejects if `overall_result` disagrees. That is materially better than trusting prose or trusting `overall_result` alone.

But the computation is still **self-authored scalar recomputation**, not evidence recomputation. It checks fields such as `evidence_file`, `evidence_test`, `negative_file`, `negative_test`, mutation counts, and governance posture strings. It does not verify that the cited tests exist, that the cited test actually covers the claim, that a named negative fixture attacks the current public API, that mutation output files match the counts, or that governance evidence proves the exact posture.

That shallow recomputation is exactly how the 0054 acceptance can be internally “pass”-shaped while this pass finds live code defects.

### 8.2 0055 solo-maintainer posture enforcement

The parser is stale. Its accepted governance values are:

- `independent-review`
- `last-push-required-reviewer`

It treats `pending-governance`, `status-checks-only`, and `zero-approval` as non-pass, and any other value as an unknown error. It does **not** know `solo-maintainer-compensating-control`, even though `docs/2-execution/10` now says the posture vocabulary includes that value when the architecture-tier compensating-control set is proven.

Consequence:

- a truthful post-0055 solo-maintainer artifact using `governance_independence: solo-maintainer-compensating-control` would be rejected as unknown; or
- an artifact author would have to mislabel the current posture as `independent-review` or `last-push-required-reviewer` to get a pass.

That is a fail-closed implementation failure. The doctrine is settled; the parser must implement it.

Required remediation:

1. Extend the manifest schema with `governance_independence: solo-maintainer-compensating-control`.
2. Require machine-readable compensating-control fields for that value: required checks present, active enforcement, no bypass actors, `current_user_can_bypass: never`, non-fast-forward protection, deletion protection, strict/up-to-date enforcement.
3. Fail closed if any compensating-control field is absent, unproven, stale, or prose-only.
4. Preserve the rejection of bare `zero-approval` and `status-checks-only`.

This does not re-litigate whether solo-maintainer mode is legitimate. It enforces the doctrine that 0055 ratified.

### 8.3 Closed grammar vs. phrase denylist

`acceptance_artifact_wording.rs` is not a closed grammar. It has:

- a list of exact forbidden overclaims;
- a list of conditional closure substrings such as `pass with`, `scoped pass`, `accepted`, `green canonical perimeter`, and `canonical perimeter green`;
- ad hoc checks for branch-protection claims, historical-results claims, display-string evidence claims, and green-perimeter claims.

A paraphrase can still bypass it: “approved,” “validated,” “ready to merge,” “successful conclusion,” “no blockers remain,” “closure recognized,” or “all required evidence is satisfied” are not structurally parsed as verdict claims unless they happen to contain one of the banned substrings.

Required remediation:

- Make the template require a closed verdict grammar, e.g. a single computed verdict line generated from the manifest: `Computed result: pass` or `Computed result: non-pass`.
- Forbid freeform prose from containing any verdict-bearing sentence outside named sections, or make the guard parse the allowed sections and reject other closure claims.
- Keep the existing denylist as a secondary alarm, not the primary grammar.

### 8.4 Mutation coverage of the taxonomy itself

The taxonomy has synthetic tests for open findings, routed-forward fields, survivors, branch-protection scalar failure, zero-approval/status-checks-only failure, current mutation evidence, non-current evidence strings, missing blocks, and wording overclaims. That is useful.

This static pass did not find current mutation evidence showing mutants of `compute_result(...)`, `governance_is_independent(...)`, `validate_green_mutation_evidence(...)`, or the wording guard are killed. A guard whose own mutants survive is decorative. The next remediation must add the parser/guard files to the mutation perimeter or provide current in-diff mutation evidence for changes to those files.

### 8.5 Forcing function before archival/merge

There is a real forcing function: `.github/workflows/ci.yml` has an “Ingest changed acceptance artifacts” step. It detects changed `reports/` or `archive/reports/` acceptance artifacts, rejects report/spec closure changes without an acceptance artifact, requires the status fence, and runs `acceptance_status_manifest actual_acceptance_artifact_from_ci_env_is_pass_eligible` with `TRACEWAKE_ACCEPTANCE_ARTIFACT` set to the changed artifact.

That is the right topology. It is insufficient because:

- the parser is stale relative to 0055;
- the parser recomputes from self-authored scalar rows, not independent evidence ingestion;
- the wording guard is not a closed grammar;
- the parser can accept a cited negative test that attacks a retired symbol.

The durable forcing function is not a new gate. It is the existing CI ingestion step plus a doctrine-complete parser, closed verdict grammar, current-symbol negative fixtures, and mutation coverage of the taxonomy code itself.

### 8.6 Adversarial application to the 0054 acceptance

The 0054 acceptance can be internally self-consistent under the 0054-evolved taxonomy, but this pass finds live defects in the source it accepted. That means the taxonomy would not reject an artifact merely because its `closed` rows cite tests that are too narrow or stale. It validates that the block is shaped like a pass; it does not independently re-prove the authority property.

Concrete laundering vectors:

- `F6-01` can cite the seed-parts negative fixture while the validated-content alias remains open.
- `F6-03` can cite direct debug token/bind fixtures while ordinary TUI input still induces debug authority.
- `mutation_survivors: none` can be stated with current-looking counts from an artifact, while this static pass cannot certify any current run.
- `governance_independence` can be made self-consistent by choosing one of the parser’s accepted scalar values even when the current doctrine value is different.

The remedy is adversarial fixtures and parser semantics that attach each closure row to the live public API route it claims to close.

---

## 9. Foundation & documentation determination

### 9.1 0055 no-weakening check

The 0055 doctrine edits did **not** silently weaken architecture/execution doctrine.

Architecture `13` says solo-maintainer acceptance mode is “not a weaker behavioral-evidence rule,” requires the full compensating-control set, preserves second-human review as default for multi-maintainer operation, and states that open/pending/unbounded/historical/unproven/not-ingested governance controls still make `pass` invalid. It also preserves the rejection of self-authored-only behavioral evidence for behavior, mutation, and typed path-under-test claims.

Execution `10` says the manifest vocabulary includes `solo-maintainer-compensating-control` only when the compensating controls are proven by machine-readable ruleset evidence; a zero-approval configuration is not an independence gap only with that proof. It explicitly says zero-approval or status-checks-only gaps, self-authored-only evidence, missing actual-artifact ingestion, unbounded routed-forward items, missing governance proof, survivors, timeouts, stale mutation evidence, and “pass with disposition” are not pass.

Therefore the doctrine text is sound at its tier. The implementation is stale.

### 9.2 Tier-0 amendment verdict: no amendment

No Tier-0 amendment is warranted.

Reasoning:

- The foundation already owns product truths: causality, replay, actor-known cognition, debug quarantine, UI deauthority, no-human parity, and harsh feature acceptance.
- It does not need to define the social governance mechanics of a solo-operated repository. That belongs to architecture/execution acceptance doctrine.
- Spec 0055’s solo-maintainer posture does not ask the product constitution to accept weaker behavior evidence. It substitutes a compensating-control set for human approval on routine merges while keeping behavioral evidence independent.
- Elevating solo-maintainer acceptance into Tier 0 would overfit the constitution to one repository operation mode and risk confusing product invariants with project governance mechanics.

Conditional caveat: if a future remediation tries to let `solo-maintainer-compensating-control` bless self-authored-only behavioral evidence, stale mutation evidence, or a pass while a protected authority defect is open, that would be a lower-tier violation first. Amend Tier 0 only if lower tiers explicitly contradict product invariants.

### 9.3 Post-implementation live-doc work table

No archived spec, ticket, acceptance artifact, or passed certification should be edited.

| Home | Post-closure update |
|---|---|
| `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Update loaded-world / TUI authority rows after executable proof exists. |
| `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` | Cross-link the unforgeable validated bootstrap as replay seed authority. |
| `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` | Clarify sealed content bootstrap proof object and core-owned handoff. |
| `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | No substantive change unless remediation changes transaction entrypoints. |
| `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Describe explicit operator/debug launch boundary after implementation. |
| `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Sync with parser grammar only after parser implementation exists. |
| `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` | Add proof requirement for external-crate inability to forge validated content handoff. |
| `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` | No change unless bootstrap remediation alters no-human loading surface. |
| `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Add ordinary-mode debug noninducibility proof and operator-mode positive proof. |
| `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Align taxonomy implementation details with 0055 value and closed verdict grammar. |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Update reviewer pointers after executable closure. |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Update R-27/R-28/R-29 status only; mint no new risk ID. |
| `docs/4-specs/SPEC_LEDGER.md` | Normal future remediation ledger process only; this report does not assign a spec number. |
| `.cargo/mutants.toml` / `.cargo/mutants-baseline-misses.txt` | Extend existing perimeter/baseline registry only if executable mutation evidence requires it. |

---

## 10. Recommended closure order

1. **Seal the validated-content bootstrap class.** Remove public raw-aggregate constructors and public loaded-fixture fabrication first; otherwise later tests can certify a false boundary.
2. **Separate debug operator authority from ordinary TUI input.** Make ordinary-mode debug noninducibility a public behavior proof.
3. **Add current-symbol negative fixtures.** Attack the exact public APIs found here, not historical names.
4. **Update taxonomy parser for 0055.** Add `solo-maintainer-compensating-control` with required proof fields and keep zero-approval/status-checks-only failing without proof.
5. **Convert wording guard to closed grammar.** Keep the denylist as secondary defense.
6. **Mutation-cover the guard and seam.** Parser/guard changes and code authority changes must have current mutation evidence or honest non-pass disposition.
7. **Run the full standing gate from a clean baseline.** Only the implementing session can certify actual test/mutation/CI status.
8. **Then update live docs and reference rows.** No archive edits and no new invariant/gate/risk/glossary identifiers.

---

## 11. Open maintainer decisions inside settled doctrine

- Choose the content/core handoff design: move materialization into core, introduce an unforgeable core proof object, or make content deliver raw authored data to a core validator. The requirement is unforgeability, not a specific architecture.
- Decide the exact operator/debug launch UX. It may remain convenient for local development, but it must be outside ordinary embodied command input and visibly non-diegetic.
- Decide whether foundational-conformance acceptance artifacts in solo-maintainer mode require an artifact-specific independent acceptor, as architecture `13` leaves recommended but process-ratifiable.
- Decide whether the parser reads governance proof fields from the artifact block or a checked-in machine-readable transcript file. Either is acceptable if fail-closed and current.

---

## 12. Self-check

- Verdict covers code conformance, taxonomy soundness, 0055 no-weakening, higher-tier amendment, and another-iteration judgment.
- Target-state claims are tied to exact-commit repository files from `joeloverbeck/tracewake` at `2720167a0d1a60ac809ae1c670539a1846df031d`.
- 0054 present properties are recorded as present where they hold; live residual defects are reported separately.
- The 0055 solo-maintainer governance mechanism is not re-litigated; only taxonomy enforcement and no-weakening are audited.
- Recommendations extend existing fixtures, CI, mutation, and taxonomy machinery; no new property-testing framework, backwards-compat shim, invariant, gate code, risk ID, or glossary term is introduced.
- Static-survey limits are explicit.
- Repository evidence and external research lanes are separated.

---

## 13. References

### Repository evidence lane

The complete exact-commit URL ledger is Appendix A. Key repository evidence used above includes:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `crates/tracewake-core/src/state.rs`
- `crates/tracewake-core/src/runtime/session.rs`
- `crates/tracewake-core/src/runtime/command.rs`
- `crates/tracewake-core/src/runtime/receipt.rs`
- `crates/tracewake-core/src/debug_capability.rs`
- `crates/tracewake-core/src/view_models.rs`
- `crates/tracewake-core/src/projections.rs`
- `crates/tracewake-content/src/load.rs`
- `crates/tracewake-content/src/schema.rs`
- `crates/tracewake-content/src/manifest.rs`
- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/src/debug_panels.rs`
- `crates/tracewake-core/tests/support/acceptance_status_manifest.rs`
- `crates/tracewake-core/tests/acceptance_status_manifest.rs`
- `crates/tracewake-core/tests/acceptance_artifact_wording.rs`
- `.github/workflows/ci.yml`
- `archive/reports/0054_foundational_conformance_sixth_hardening_acceptance.md`
- `archive/specs/0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md`

### External research lane

External sources informed the remediation posture but were not used to assert target-repository state.

- Rust Reference, “Visibility and privacy” — public items are externally accessible when their module path is accessible; privacy is a language rule, not a documentation convention. https://doc.rust-lang.org/reference/visibility-and-privacy.html
- Rustdoc Book, `#[doc]` / `#[doc(hidden)]` — documentation attributes affect documentation output and API presentation, not Rust privacy. https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html and https://doc.rust-lang.org/rustdoc/write-documentation/what-to-include.html
- Fred B. Schneider, “Implementing Fault-Tolerant Services Using the State Machine Approach: A Tutorial” — state-machine/replay discipline depends on ordered commands/events and deterministic transition authority. https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf
- Temporal documentation, Workflow / Workflow Definition / Event History — replay reconstructs execution from event history and requires deterministic workflow code. https://docs.temporal.io/workflows and https://docs.temporal.io/workflow-definition
- Sabelfeld & Myers, “Language-Based Information-Flow Security” — access control alone is not equivalent to information-flow control; embodied/debug separation needs product-level flow discipline. https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf
- cargo-mutants documentation, timeouts and baseline behavior — mutation gates need explicit handling of misses, timeouts, baselines, and current result artifacts. https://mutants.rs/timeouts.html and https://mutants.rs/baseline.html
- DeepMind, “Specification gaming: the flip side of AI ingenuity” — a process can satisfy a literal spec while missing the intended outcome; this frames the taxonomy laundering risk. https://deepmind.google/blog/specification-gaming-the-flip-side-of-ai-ingenuity/
- NIST SP 800-171 3.1.4 — separation of duties reduces abuse of authorized privileges; small-team compensating controls should be explicit rather than implicit. https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171r2.pdf
- Goodhart/Strathern literature on metrics becoming targets — supports keeping acceptance taxonomy closed and evidence-bound rather than pass-wording-bound. https://pmc.ncbi.nlm.nih.gov/articles/PMC7901608/

---

## Appendix A — complete exact-commit acquisition URL ledger

Every URL below was constructed mechanically from:

`https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/` + manifest path.

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/00_FOUNDATION_INDEX.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/01_PROJECT_CHARTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/4-specs/SPEC_LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report-sixth-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-brief-sixth-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report-fifth-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report-second-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report-third-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0047-foundational-hardening-research-report-fourth-pass.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/reports/0048_foundational_conformance_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0047_tui_authoritative_world_advance_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0050_foundational_conformance_second_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0053_foundational_conformance_fifth_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/reports/0054_foundational_conformance_sixth_hardening_acceptance.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0048_FOUNDATIONAL_CONFORMANCE_HARDENING_LOADED_WORLD_TICK_TEMPORAL_AUTHORITY_HOLDER_KNOWN_INTERVALS_AND_REPLAY_FRONTIER_RECONSTRUCTION_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0050_FOUNDATIONAL_CONFORMANCE_SECOND_HARDENING_LOADED_WORLD_DISCOVERY_ACTOR_TRANSACTION_UNIFICATION_TUI_DEAUTHORITY_AND_REPLAY_FAIL_CLOSED_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0051_FOUNDATIONAL_CONFORMANCE_THIRD_HARDENING_CORE_OWNED_LOADED_WORLD_RUNTIME_PRODUCTION_REACHABILITY_PROCESS_TRANSACTIONS_ACTOR_CENSUS_AND_TUI_DEAUTHORITY_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_PRODUCTION_BOOTSTRAP_CLOSED_COMMAND_BOUNDARY_REPLAY_AUTHORITY_REAL_PROCESSES_ACTOR_CENSUS_EMBODIED_DEBUG_SPLIT_AND_STANDING_BARRIER_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0053_FOUNDATIONAL_CONFORMANCE_FIFTH_HARDENING_SEALED_BOOTSTRAP_INTERVAL_PRODUCT_TOKENIZED_DEBUG_AUTHORITY_MERGE_ENFORCED_BARRIER_AND_FAIL_CLOSED_ACCEPTANCE_TAXONOMY_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0054_FOUNDATIONAL_CONFORMANCE_SIXTH_HARDENING_RESEALED_BOOTSTRAP_SEALED_WAIT_RECEIPT_NON_INDUCIBLE_DEBUG_AUTHORITY_INDEPENDENT_ACCEPTANCE_AND_FAIL_CLOSED_TAXONOMY_HARDENING_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/archive/specs/0055_SOLO_MAINTAINER_ACCEPTANCE_GOVERNANCE_AMENDMENT_PROPOSAL_SPEC.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/.github/workflows/ci.yml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/.cargo/mutants.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/.cargo/mutants-baseline-misses.txt`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/runtime/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/runtime/session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/runtime/command.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/runtime/receipt.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/debug_capability.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/debug_reports.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/scheduler.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/events/envelope.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/events/apply.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/events/log.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/events/mod.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/agent/transaction.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/agent/trace.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/agent/perception.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/agent/actor_known.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/epistemics/projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/projections.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/view_models.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/replay/temporal.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/replay/rebuild.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/replay/report.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/src/actions/pipeline.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/src/load.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/src/schema.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/src/validate.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/app.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/run.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/transcript.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/render.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/input.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/launch.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/debug_panels.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/acceptance_status_manifest.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/acceptance_artifact_wording.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/support/acceptance_status_manifest.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/world_step_coordinator.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/negative_fixture_runner.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/anti_regression_guards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/generative_lock.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/ci_workflow_guards.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/replay_temporal_frontier.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/holder_known_interval_projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/salient_stop_actor_known.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/reservation_body_exclusive_census.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/mutation_completion_merge.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/food_source_projection.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/golden_scenarios.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/no_human_capstone.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/spine_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/event_schema_replay_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-core/tests/hidden_truth_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/tui_acceptance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/tui_seam_conformance.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/command_loop_session.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/playable_capability_parity.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/embodied_flow.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/parity_adversarial.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/transcript_snapshot.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-tui/tests/adversarial_gates.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_assign_scheduler_frontier/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_assign_scheduler_frontier/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_build_debug_knowledge_context/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_build_debug_projection_view_without_core_debug_api/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_core_perception_append_helper/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_core_perception_append_helper/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_scheduler_perception_writer/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_scheduler_perception_writer/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_seed_mutators_after_load/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_seed_mutators_after_load/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_belief_literal/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_belief_literal/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_debug_report/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_debug_report/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_observation_without_source/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_pipeline_context_with_runtime_aggregates/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_construct_pipeline_context_with_runtime_aggregates/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_convert_debug_report_to_interval_summary/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_convert_debug_report_to_interval_summary/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_forge_interval_notice/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_forge_interval_notice/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_forge_mutation_capability/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_insert_raw_epistemic_records/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_agent_state_seed_maps/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_agent_state_seed_maps/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_belief_source_or_scope/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_contradiction_links/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_embodied_temporal_fields/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_embodied_temporal_fields/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_mode/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_knowledge_context_viewer/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_loaded_runtime_fields/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_mutate_loaded_runtime_fields/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_name_due_process_invocation/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_name_due_process_invocation/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_read_one_tick_wait_receipt_internals/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_read_one_tick_wait_receipt_internals/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_read_raw_epistemic_projection_maps/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_reduce_actor_step_outcome_to_option/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_reduce_actor_step_outcome_to_option/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_set_world_step_due_actor_ids/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_set_world_step_due_actor_ids/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_set_world_step_process_events/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_set_world_step_process_events/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_submit_debug_command_without_token/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/tests/negative-fixtures/external_crate_cannot_submit_debug_command_without_token/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/2720167a0d1a60ac809ae1c670539a1846df031d/crates/tracewake-content/src/manifest.rs`
