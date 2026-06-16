# 0037 — P0-CERT mutation remediation and replacement certification spec

**Staging path:** `specs/0037_P0_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`  
**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `c54caff24596f32104c4b83a1c4c6e0489636be6`  
**Spec series:** numbered implementation spec, staged under `specs/`, to be archived under `archive/specs/` on acceptance.  
**Status:** COMPLETED implementation specification; archived after acceptance.
**Work posture:** `Remediation`.  
**P0-CERT admissibility posture of this spec:** `P0-CERT scoped remediation`.  
**Certification-line effect:** successful execution of this spec must produce a replacement P0-CERT acceptance artifact that renders `P0-CERT passed` for the post-0008 baseline and explicitly supersedes `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`. Until that replacement artifact exists and passes, later specs may not cite `P0-CERT passed`.

This spec is subordinate to foundation, architecture, execution, reference, and live spec-layer doctrine. It operationalizes the scoped remediation demanded by the 0036 P0-CERT artifact. It does not amend invariants, redefine gate semantics, create a new gate, promote archived history into live certification, or decide any forward certification line.

This document is non-executable. It specifies what the implementing session must run, prove, record, and package. It does **not** assert that the current code passes, that the mutation survivor is killed, or that the replacement certification already exists.

---

## 1. Source discipline, freshness, and admissibility

The target of record is the user-supplied exact commit `c54caff24596f32104c4b83a1c4c6e0489636be6`. This spec does not verify that this commit is current `main`. A manifest is path inventory only. Branch names, default-branch lookups, repository metadata, connector namespace labels, code-search snippets, old chat memory, and historical commit strings inside prior specs are not proof of target-commit content.

`9f16222` appears in the 0036 spec and acceptance artifact as 0036 audit provenance. It is not the baseline of this remediation. This remediation is pinned to `c54caff24596f32104c4b83a1c4c6e0489636be6`.

The admissibility state is closed by the live ledger and 0036 acceptance artifact: the current line is `P0-CERT scoped remediation`, and the next admissible move is a scoped remediation for `0036-MUTATION-REMEDIATION-001`. This spec is that move. SPINE-CERT, EPI-CERT, the 0035 route-forward backlog, Phase-4 entry, second-proof entry, and all other forward moves remain blocked until the replacement P0-CERT artifact passes.

No new finding ID is coined here. `0036-MUTATION-REMEDIATION-001` remains the named finding. Any additional survivors discovered by the full configured mutation run are identified by cargo-mutants output and the triage register, not by new doctrine-level IDs.

---

## 2. Authority and dependency declarations

### 2.1 Controlling doctrine

The controlling authority order is:

1. `docs/0-foundation/`
2. `docs/1-architecture/`
3. `docs/2-execution/`
4. `docs/3-reference/`
5. `docs/4-specs/`
6. staged implementation specs under `specs/`

If this spec conflicts with a higher tier, the higher tier governs. If a convenient test proves only plausible behavior while bypassing holder-known provenance, the test is wrong. If code obtains a green result by treating archived rows as live proof, the certification is invalid.

### 2.2 Primary remediation sources

These sources define the failure and the next move:

- `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md` — failed P0-CERT acceptance artifact; verdict `P0-CERT scoped remediation`; mutation finding `0036-MUTATION-REMEDIATION-001`; admissibility lock; acceptance-artifact shape to supersede.
- `archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md` — predecessor audit spec; ten-point P0-CERT audit contract; gate composition; evidence commands; artifact section shape.
- `docs/4-specs/SPEC_LEDGER.md` — live ledger; source discipline; 0036 row; next known execution move.

The 0036 artifact is historical evidence and the direct object of remediation. It is not live proof that P0-CERT passed.

### 2.3 Higher-tier primary doctrine

Foundation controls the truth-firewall and acceptance properties this spec must preserve:

- `docs/README.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`

Architecture controls subsystem contracts:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

Execution controls gate vocabulary, P0-CERT, proof sequence, gate-failure handling, and artifact obligations:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

Live reference/spec-layer sources used for terminology, source discipline, and artifact shape:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`
- `docs/4-specs/README.md`
- `docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md`
- `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`

Historical structural references, not live certification:

- `archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md`
- `archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md`

Archived specs and tickets `0005` through `0036` are history only. They may explain why seams exist. They may not be used as current checkout certification.

### 2.4 Code and test seams

The remediation and replacement artifact must audit and, where necessary, change only the post-0008 baseline surfaces needed for P0-CERT:

- Primary mutation seam: `crates/tracewake-core/src/projections.rs::actor_known_local_actors_for_context`.
- Immediate actor-known/projection/view-model consumers: `crates/tracewake-core/src/epistemics/knowledge_context.rs`, `crates/tracewake-core/src/projections.rs`, `crates/tracewake-core/src/view_models.rs`.
- Actor-known/no-human decision surfaces to re-prove under P0-CERT: `crates/tracewake-core/src/agent/actor_known.rs`, `crates/tracewake-core/src/agent/no_human_surface.rs`, `crates/tracewake-core/src/scheduler.rs`, `crates/tracewake-core/src/actions/pipeline.rs`, and guarded action definitions `eat.rs`, `sleep.rs`, `work.rs`.
- Mutation posture configuration and CI command: `.cargo/mutants.toml`, `.github/workflows/ci.yml`, `.cargo/mutants-baseline-misses.txt`.
- Primary proof and regression tests: `crates/tracewake-core/tests/hidden_truth_gates.rs`, `no_human_capstone.rs`, `acceptance_gates.rs`, `event_schema_replay_gates.rs`, `anti_regression_guards.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, `crates/tracewake-tui/tests/adversarial_gates.rs`.
- Replay/provenance/diagnostic ancestry: `crates/tracewake-core/src/replay/`, `crates/tracewake-core/src/checksum.rs`, `crates/tracewake-core/src/agent/trace.rs`, `crates/tracewake-core/src/actions/report.rs`, `crates/tracewake-core/src/events/apply.rs`, `crates/tracewake-content/src/validate.rs`.

No backwards-compatibility shims, alias paths, or new files created merely for symmetry are allowed.

---

## 3. Problem statement

The 0036 P0-CERT baseline audit did not pass. Its capstone verdict is `P0-CERT scoped remediation`, not `P0-CERT passed`.

The failing evidence is the configured lock-layer mutation posture. The 0036 artifact records that the run:

- used the configured guarded-layer mutation command;
- passed the unmutated baseline;
- found 1128 mutants;
- emitted one untriaged missed mutant: `crates/tracewake-core/src/projections.rs`, `actor_known_local_actors_for_context -> Vec<ActorId>` replaced with `vec![]`;
- stopped before completion because the run was too large for that ticket turn.

That makes the mutation lock layer incomplete. The named finding is:

```text
0036-MUTATION-REMEDIATION-001
Seam: projections.rs::actor_known_local_actors_for_context
Mutation: replace return Vec<ActorId> with vec![]
Responsible failing layer: lock layer / holder_known_context / projection / test_oracle
Required disposition: kill with behavior/provenance coverage, or prove and record equivalent/non-critical
Required follow-up: rerun configured mutation posture and replace the certification artifact
```

The failure is broader than a single named survivor. Because 0036 stopped early, its mutation evidence is sampled and incomplete. The implementing session must run the full configured posture to completion and disposition every survivor surfaced by that full run. A narrow test that kills only the already-known mutant is insufficient and would be metric-gaming, not certification.

The replacement artifact must also re-prove the rest of the ten-point P0-CERT contract live at `c54caff`. The 0036 pass rows are historical context from the 0036 audit provenance. They may inform the plan, but they are not live current-checkout proof.

---

## 4. Remediation approach

### 4.1 Required end state

Execution of this spec is complete only when all of the following are true:

1. The unmutated workspace passes the live P0-CERT command set at `c54caff` plus any implementation changes made by this remediation.
2. The full configured P0-CERT mutation posture runs to completion over the configured guarded seams.
3. Every `missed` or `timeout` survivor is triaged.
4. Every critical survivor is killed with behavior/provenance/replay coverage, or is proven equivalent/non-critical through a reviewable argument naming exact call sites and certified-behavior reachability.
5. The triage register is included or referenced by the replacement acceptance artifact.
6. `0036-MUTATION-REMEDIATION-001` is specifically dispositioned through Arm A or Arm B in this spec.
7. P0-01 through P0-10 are re-established live at the remediation checkout.
8. The replacement acceptance artifact conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, renders `P0-CERT passed`, and explicitly supersedes the 0036 artifact.

If any untriaged, critical, or uncertified mutation survivor remains, the replacement artifact must not render `P0-CERT passed`; it must render `P0-CERT scoped remediation` and name the remaining blocker under the responsible layer.

### 4.2 Live preflight and gate command set

The implementing session must run and record, at minimum:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --locked
```

It must also re-run the explicit lock-layer gate set used by CI:

```bash
cargo test --locked -p tracewake-core --test negative_fixture_runner
cargo test --locked -p tracewake-core --test doc_invariant_references
cargo test --locked -p tracewake-core --test spine_conformance
cargo test --locked -p tracewake-core --test acceptance_gates
cargo test --locked -p tracewake-core --test anti_regression_guards
cargo test --locked -p tracewake-core --test event_schema_replay_gates
cargo test --locked -p tracewake-core --test hidden_truth_gates
cargo test --locked -p tracewake-core --test acceptance_artifact_wording
cargo test --locked -p tracewake-content --test schema_conformance
cargo test --locked -p tracewake-content --test forbidden_content
cargo test --locked -p tracewake-content --test golden_fixtures_run
cargo test --locked -p tracewake-tui --test adversarial_gates
cargo test --locked -p tracewake-tui --test tui_seam_conformance
```

The replacement artifact may add more commands, but it may not omit these categories unless it records a higher-tier reason and keeps the verdict out of `P0-CERT passed` until the omission is resolved.

### 4.3 Full configured mutation posture

The full configured posture is the scheduled guarded-layer command from `.github/workflows/ci.yml`, with `.cargo/mutants.toml` in effect:

```bash
cargo install cargo-mutants --version 27.1.0 --locked

cargo mutants --workspace \
  -f 'crates/tracewake-core/src/agent/**' \
  -f 'crates/tracewake-core/src/scheduler*' \
  -f 'crates/tracewake-core/src/projections*' \
  -f 'crates/tracewake-core/src/actions/pipeline.rs' \
  -f 'crates/tracewake-core/src/actions/defs/eat.rs' \
  -f 'crates/tracewake-core/src/actions/defs/sleep.rs' \
  -f 'crates/tracewake-core/src/actions/defs/work.rs' \
  --no-shuffle
```

`.cargo/mutants.toml` adds `additional_cargo_args = ["--workspace", "--locked"]` and excludes non-perimeter crates/files so that the posture stays inside actor cognition, scheduler, projections, action pipeline boundary code, and guarded ordinary action definitions. `.cargo/mutants-baseline-misses.txt` is empty at the target commit; therefore this remediation must not hide a survivor behind an accepted baseline miss.

Cargo-mutants documentation supports using a checked-in config file at `.cargo/mutants.toml`, describes the baseline run as the check that the unmutated tree passes before mutations are applied, records output under `mutants.out` including `mutants.json`, `outcomes.json`, diffs, logs, and `caught.txt`/`missed.txt`/`timeout.txt`/`unviable.txt`, and treats `missed` mutants as likely coverage gaps or possible indistinguishable/equivalent mutants. It also permits sharding only when every shard uses identical arguments and the same denominator; inconsistent shards make the result meaningless.[^cargo-config][^cargo-baseline][^cargo-output][^cargo-results][^cargo-shards]

A full run may be sharded to control wall-clock cost, but only under these restrictions:

- All shards use the same source checkout, same cargo-mutants version, same config, same filters, same `--no-shuffle` posture, same environment-relevant test command, and same shard denominator `n`.
- Every shard from `0/n` through `(n-1)/n` completes.
- Each shard writes to a distinct output directory.
- The replacement artifact includes the shard commands, stdout/stderr summaries, `mutants.json`, `outcomes.json`, `missed.txt`, `timeout.txt`, `unviable.txt`, and logs or artifact references for each shard.
- The aggregate evidence states the total enumerated mutant count and proves no shard was skipped, failed silently, or run against a different filter set.
- `--baseline=skip` is forbidden unless the same artifact records a successful immediately preceding unmutated baseline and an explicit timeout strategy. If the baseline is skipped for sharded cost control, the replacement artifact must state why the result remains meaningful and must record the timeout value or multiplier used.

Timeouts are not green. Cargo-mutants uses timeouts to avoid hangs and reports timeout outcomes for investigation; this spec treats every timeout under a certified seam as requiring triage unless a reviewable equivalence/non-critical argument proves it cannot affect certified behavior.[^cargo-timeouts]

### 4.4 Survivor triage protocol

The implementing session must maintain a mutation-triage register. It may be a Markdown table, JSON artifact, or both, but the replacement artifact must link or embed it.

Minimum fields:

| Field | Requirement |
|---|---|
| Mutant identity | cargo-mutants mutant text, path, function, operator, and raw diff/log reference. |
| Outcome | `caught`, `missed`, `timeout`, `unviable`, or tool failure. |
| Responsible layer | One of the responsible implementation layers already used by diagnostics, using existing typed responsible layers such as `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, and `debug_quarantine`, plus existing non-agent diagnostic categories already emitted by replay/content/event validation code; do not invent gate/status vocabulary. |
| Certified reachability | Which P0-CERT proof requirement and canonical gate(s) the mutant can affect, or why none are reachable. |
| Call-site review | Exact consuming functions/modules used to decide behavior relevance. |
| Disposition | `killed`, `equivalent`, `non-critical`, `unviable`, or `blocked`; no untriaged survivors allowed in a passing artifact. |
| Behavior witness | For killed mutants: public/certified behavior that fails under the mutant; for equivalent/non-critical mutants: reason a behavior witness cannot exist or is outside the certified scope. |
| Provenance/replay witness | Source events, context hash/frontier, replay report, projection ancestry, or explicit reason not applicable. |
| Negative/adversarial witness | The forbidden shortcut that must remain failing or unavailable. |
| Review signoff | Implementer plus reviewer or equivalent repo closeout proof. |

A killed mutant requires a test that fails because the public/certified behavior changed. Cargo-mutants guidance is explicit that the right fix for a private-function miss is often not a private-function assertion, but a test of the public behavior that would break if the private function were wrong.[^cargo-results] For Tracewake, that behavior witness must also be source-backed: actor-known facts, provenance, holder-known context IDs/hashes/frontiers, event/replay ancestry, and debug quarantine must be inspectable.

A proven-equivalent or proven-non-critical mutant is allowed only with a reviewable argument. Academic mutation-testing literature treats equivalent and redundant mutant identification as a hard problem; the distribution of mutants is unpredictable, equivalence/redundancy identification is undecidable in general, and human review is often needed.[^papadakis-survey][^nguyen-madeyski] Therefore this repo must not mark a survivor equivalent because no existing test failed. The argument must name the exact call sites and explain why no P0-certified behavior can observe a semantic difference.

### 4.5 Re-proof of the rest of P0-CERT

The replacement artifact must re-establish P0-01 through P0-10 live at the remediation checkout. It may cite the 0036 spec and report as historical context and as shape references, but not as current proof.

Re-proof must be proportionate but real:

- Re-run the current command set.
- Re-run the full guarded mutation posture to completion.
- Re-collect event/replay/projection evidence for the first-proof fixture families exercised by this line.
- Re-collect actor-known provenance and hidden-truth negative evidence.
- Re-collect possession/debug quarantine evidence.
- Re-collect diagnostics by responsible layer.
- Record all changed files and any unchanged seam audits that remain relevant.

Staged abstractions are allowed only if the replacement artifact declares them under the acceptance-template field added by the live specs tier. They cannot mask a failed P0 obligation. A staged abstraction may bound future institutions, LOD, regional scale, or LLM speech surfaces; it may not defer actor-known provenance, mutation survivor disposition, replay, debug quarantine, or the ten P0-CERT requirements.

---

## 5. Preliminary static seam survey — informative, not certification

This section is a static reading of exact-commit source. It is not a pass/fail finding and cannot substitute for cargo tests, cargo-mutants, replay, or acceptance evidence.

### 5.1 Observed source shape

At `c54caff`, `crates/tracewake-core/src/projections.rs::actor_known_local_actors_for_context` is a private helper:

```rust
fn actor_known_local_actors_for_context(context: &KnowledgeContext) -> Vec<ActorId> {
    let mut actors = context
        .actor_known_local_actors()
        .iter()
        .map(|fact: &ActorKnownLocalActorFact| fact.actor_id().clone())
        .collect::<Vec<_>>();
    actors.sort();
    actors.dedup();
    actors
}
```

The 0036 survivor replaces this return with `vec![]`.

`EmbodiedProjectionSource::from_sealed_context` calls this helper while constructing the actor-known projection source. `build_embodied_view_model` then maps `source.actor_known_local_actors` into `VisibleActor` entries and stores them as `EmbodiedViewModel.local_actors`. `EmbodiedViewModel` is an actor-filtered view model with holder-known context ID, context hash, frontier, source summary, visible exits/doors/containers/items, carried items, local actors, semantic actions, and debug-availability state.

`KnowledgeContext` stores `actor_known_local_actors: Vec<ActorKnownLocalActorFact>`. `ActorKnownLocalActorFact` carries an `ActorId` and `source_key`, has a canonical key containing both, and is sorted/deduplicated by context sealing. The context hash includes the actor-known local actor vector. The projection helper therefore participates in carrying sealed actor-known facts from context into the embodied view model.

A static text search of the exact fetched `agent/actor_known.rs` and `agent/no_human_surface.rs` did not reveal direct references to `ActorKnownLocalActorFact` or `actor_known_local_actors`. That does not prove non-reachability. It only suggests that the current directly observed consuming path is holder-known context → `EmbodiedProjectionSource` → `build_embodied_view_model` → `EmbodiedViewModel.local_actors`. The implementing session must verify call sites under the final checkout and cargo-mutants output.

The named core test files searched statically did not contain direct `local_actors`/`VisibleActor` assertions. That makes the survivor unsurprising: existing tests may exercise the projection path without asserting the actor-known local actor consequence.

### 5.2 Preliminary interpretation

The survivor is likely killable, not equivalent, because replacing the helper with `vec![]` removes source-backed local actors from a certified actor-filtered embodied view-model field. The correct witness should not assert only “the vector is non-empty.” It must show that a modeled, source-backed actor-known local actor fact reaches embodied play, while an unmodeled or hidden-truth local actor does not.

A good killing test belongs closest to the lock layer, probably in `crates/tracewake-core/tests/hidden_truth_gates.rs` or `crates/tracewake-core/tests/acceptance_gates.rs`, with optional fixture support from content if the repo prefers fixture-backed view-model proofs. The test should be core-visible enough to fail directly when `actor_known_local_actors_for_context` returns `vec![]`, and integrated enough to carry holder-known context ID/hash/frontier and source-key ancestry.

---

## 6. Per-finding deliverable: `0036-MUTATION-REMEDIATION-001`

### 6.1 Finding block

```text
Finding: 0036-MUTATION-REMEDIATION-001
Mutation: actor_known_local_actors_for_context -> vec![]
Primary path: crates/tracewake-core/src/projections.rs
Primary function: actor_known_local_actors_for_context(context: &KnowledgeContext) -> Vec<ActorId>
Responsible failing layer: lock layer / holder_known_context / projection / test_oracle
Composed gates: P0-CERT, TFW, POS-PARITY, NO-HUMAN, REPLAY, FIXTURE, DIAG
Required disposition: Arm A kill or Arm B prove equivalent/non-critical
```

### 6.2 Required call-site inventory

The implementing session must record the exact call-site inventory at the remediation checkout. At minimum, it must confirm or update these observed seams:

- `KnowledgeContext::embodied_at_frontier_with_all_facts` and `KnowledgeContext::embodied_at_frontier_with_all_facts_and_observations` accept `ActorKnownLocalActorFact` vectors.
- `KnowledgeContext::seal` sorts/deduplicates `actor_known_local_actors` and includes them in the holder-known context hash input.
- `EmbodiedProjectionSource::from_sealed_context` calls `actor_known_local_actors_for_context`. Its known top-level call site is `crates/tracewake-core/src/controller.rs` (the possession/TUI projection path), plus the internal projection helpers in `crates/tracewake-core/src/projections.rs`; note that `controller.rs` is excluded from the mutation perimeter by `.cargo/mutants.toml`, so the killing witness must drive the projection/view-model path directly rather than relying on a mutated `controller.rs`.
- `build_embodied_view_model` maps `source.actor_known_local_actors` into `VisibleActor` values and assigns `EmbodiedViewModel.local_actors`.
- Existing no-human, actor-known, TUI, or fixture consumers that indirectly observe `EmbodiedViewModel.local_actors` are identified or explicitly recorded as absent at the final checkout.

### 6.3 Arm A — kill with behavior/provenance coverage

Arm A is the preferred disposition unless the final checkout proves that the field is unreachable by certified behavior.

The killing evidence must include a test or fixture-backed integration proof with all of these properties:

1. **Modeled source-backed setup.** Construct or load a scenario in which the viewer actor has a legitimate actor-known local actor fact for another actor, with a `source_key` tied to modeled perception, observation, contact, fixture/prehistory source, or other accepted provenance route. Do not use raw physical co-location alone as the source of actor knowledge.
2. **Positive behavior witness.** Build the sealed holder-known context, projection source, and embodied view model; assert that the source-backed actor appears in `EmbodiedViewModel.local_actors` as a `VisibleActor` entry for the expected `ActorId`.
3. **Negative contamination witness.** In the same test or a paired test, keep another actor physically present or otherwise true in authoritative state but absent from holder-known local actor facts. Assert that this actor does not appear in `local_actors`. This prevents a truth-backed repair that would kill the mutation while violating the firewall.
4. **Provenance witness.** Assert the holder-known context ID, event frontier, context hash, or source summary ties the view to the sealed context that contains the local actor fact. Where possible, include source-event ancestry resolving to the event log.
5. **Replay or deterministic ancestry.** Either run through a fixture/replay path or record a deterministic context/replay artifact showing that the same source-backed local actor projection is reproducible from event/provenance ancestry.
6. **Debug quarantine.** Prove that debug availability or debug comparison does not create the local actor entry in embodied mode.
7. **Mutation kill proof.** Re-run the targeted mutant and then the full configured mutation posture; record that the `actor_known_local_actors_for_context -> vec![]` mutation is caught.

A sufficient test name would be descriptive, for example:

```text
actor_known_local_actor_reaches_embodied_view_model_with_context_provenance
```

The exact name is not doctrinal. The behavior/provenance obligations are.

Rejected test shapes:

- Asserting only that `actor_known_local_actors_for_context(context)` returns a non-empty vector.
- Asserting only that a private helper sorts/deduplicates.
- Seeding a hidden truth actor and asserting it appears because it is physically nearby.
- Parsing debug strings or fixture names as actor-visible proof.
- Adding a compile-time shim, alias path, or test-only branch that bypasses the real projection/view-model path.

### 6.4 Arm B — prove equivalent/non-critical

Arm B is admissible only if the implementing session proves that the mutation cannot change any P0-certified behavior. The proof must be recorded in the triage register and replacement artifact.

Minimum burden:

1. List every direct and indirect consumer of `actor_known_local_actors_for_context`, `EmbodiedProjectionSource.actor_known_local_actors`, and `EmbodiedViewModel.local_actors` at the final checkout.
2. Show that no P0-CERT, TFW, POS-PARITY, NO-HUMAN, REPLAY, FIXTURE, or DIAG evidence depends on the presence of local actor entries.
3. Show that erasing the vector cannot change actor-visible view models, semantic actions, no-human cognition, possession parity, debug quarantine, replay/provenance artifacts, or failure diagnostics within the post-0008 baseline.
4. Explain why retaining the field is either dead certified surface, non-critical display-only surface, or outside P0-CERT scope without contradicting `INV-067`, `INV-095`, or the execution view-model gates.
5. Include reviewer signoff. A single implementer’s statement is not sufficient for an equivalent-mutant decision on a truth-firewall seam.

Given the preliminary source survey, Arm B is expected to be difficult: the field is present in `EmbodiedViewModel`, which is a certified actor-filtered surface. If Arm B is chosen, the replacement artifact must be especially explicit about why that surface is outside certified behavior or why no user/no-human/TUI path observes it.

---

## 7. Gate evidence requirements and P0-CERT re-proof matrix

This section carries every “Gate evidence requirements” element from `docs/2-execution/03`: exact files/seams audited; foundation and architecture dependencies; artifact dependencies including observer-only `EMERGE-OBS` where the corpus exercises first-proof living-world acceptance; positive and negative fixtures; event/replay/projection evidence; actor-known provenance evidence; debug-quarantine evidence; failure diagnostics by responsible layer; historical-only use of archived specs/tickets; and tolerated deferrals tied to named gates.

### 7.1 Cross-cutting evidence package

The replacement artifact must include:

- **Exact files/seams audited:** the paths in §2.4 plus every file changed by the remediation.
- **Foundation dependencies:** `INV-001`, `INV-002`, `INV-004`, `INV-006`, `INV-008`, `INV-009`–`INV-026`, `INV-031`, `INV-032`–`INV-045`, `INV-065`–`INV-071`, `INV-091`–`INV-098`, `INV-099`–`INV-108`, `INV-111`, `INV-112` as applicable in §9.
- **Architecture dependencies:** event/replay/projection, holder-known context, proposal/validation pipeline, decision transaction, possession/TUI boundaries, validation observability.
- **Artifact dependencies:** 0036 artifact as failed historical source; 0003 template as acceptance shape; `EMERGE-OBS` section as observer-only evidence when the first-proof ordinary-life corpus is exercised.
- **Positive fixtures:** no-human day, ordinary workday, source-backed food/sleep/workplace/local view, possession parity, view-model local actions, replay/golden fixtures.
- **Negative/adversarial fixtures:** hidden food, hidden route, raw workplace assignment without context, unobserved food at open place, debug omniscience excluded, forbidden provenance input, no-direct-dispatch, external compile-fail fixture family, forbidden content/prose-born facts.
- **Event/replay/projection evidence:** event logs, replay rebuild reports, projection versions, context hashes/frontiers, checksum evidence, golden serialized outputs where relevant.
- **Actor-known provenance evidence:** sealed context IDs, source events, provenance entries, forbidden-source audit, actor-known facts used by proposals and view models.
- **Debug quarantine evidence:** debug capability boundaries, debug-only report markers, TUI adversarial gates proving debug truth cannot feed embodied views or actor decisions.
- **Diagnostics by responsible layer:** `candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, `debug_quarantine`, or existing non-agent diagnostic categories already emitted by replay/content/event validation code.
- **Historical-only statement:** archived specs/tickets `0005`–`0036` are context, not live proof.
- **Deferrals:** only those in §10, tied to named gates; no P0-CERT obligation may be deferred.

### 7.2 Ten-point P0-CERT matrix

| P0 proof requirement | Gates composed | Live re-proof required at remediation checkout |
|---|---|---|
| **P0-01 — Event/proposal/validation/event append/application/projection/replay boundaries.** | `P0-CERT`, `PIPE`, `NO-DIRECT`, `REPLAY`, `FIXTURE`, `DIAG` | Audit `actions/pipeline.rs`, `events/apply.rs`, `events/log.rs`, `projections.rs`, `replay/`, and `checksum.rs`. Positive proof: proposals commit only through validation and events; replay rebuilds significant outcomes and projections. Negative proof: no raw state mutation, no projection-only authority, no silently accepted unsupported history. Diagnostics must use existing typed layers where applicable, especially `action_validation`, `projection`, or existing event/replay diagnostic categories. |
| **P0-02 — Autonomous actor decisions use sealed actor-known contexts with provenance.** | `P0-CERT`, `TFW`, `NO-HUMAN`, `REPLAY`, `DIAG` | Audit `agent/actor_known.rs`, `agent/no_human_surface.rs`, `epistemics/knowledge_context.rs`, `epistemics/projection.rs`, `scheduler.rs`, and `projections.rs`. Positive proof: no-human decisions cite sealed context IDs, source events, and actor-known facts. Negative proof: hidden food, hidden route, raw workplace assignment, debug truth, and fixture-only truth do not enter cognition. The primary local-actor remediation belongs here and under P0-07/POS-PARITY. Diagnostics name existing typed layers such as `candidate_generation`, `method_selection`, `local_planning`, `projection`, or `test_oracle`. |
| **P0-03 — Human commands bind to ordinary actors and use the same action rules.** | `P0-CERT`, `PIPE`, `NO-DIRECT`, `POS-PARITY`, `DIAG` | Re-run TUI/core adversarial tests proving controller binding changes input/viewpoint only. Positive proof: possessed actor submits semantic actions through the shared pipeline. Negative proof: no player-only command path, no TUI mutation of world state, no direct event dispatch from input. Diagnostics name existing typed layers such as `proposal_construction` or `action_validation`, or existing TUI seam diagnostics without minting a new layer label. |
| **P0-04 — Possession never resets needs, intentions, memories, routines, or access.** | `P0-CERT`, `POS-PARITY`, `NO-HUMAN`, `REPLAY`, `DIAG` | Re-run possession parity fixtures and TUI adversarial gates. Positive proof: possession rebinding preserves actor state and intentions. Negative proof: possession does not transfer hidden knowledge or reset need/routine state. Replay must preserve possession-relevant ancestry. Diagnostics name existing typed layers such as `view_model`, `candidate_generation`, or `test_oracle`, plus existing possession-parity report fields without minting new layer labels. |
| **P0-05 — Scheduler paths cannot emit primitive actions from raw truth/routine labels/need thresholds/fixture tables.** | `P0-CERT`, `NO-DIRECT`, `NO-HUMAN`, `PIPE`, `TFW`, `DIAG` | Audit `scheduler.rs`, `agent/no_human_surface.rs`, `agent/actor_known.rs`, `actions/pipeline.rs`, and guarded action definitions. Positive proof: scheduler invokes decision transactions and due effects but not primitive action dispatch. Negative proof: hunger/fatigue/routine label cannot choose true food/work/bed/route without actor-known context. Diagnostics name existing typed layers such as `scheduler`, `candidate_generation`, `method_selection`, `local_planning`, or `proposal_construction`. |
| **P0-06 — Validation truth may accept/reject/mutate through events; it may not propose fallback or hidden actor-visible facts.** | `P0-CERT`, `PIPE`, `TFW`, `REPLAY`, `DIAG` | Audit `actions/pipeline.rs`, `actions/report.rs`, `events/apply.rs`, and proposal consumers. Positive proof: validator uses truth only to accept/reject/resolve and emits modeled feedback or events. Negative proof: rejection text and replanning cannot reveal hidden target details. Diagnostics name existing typed layers such as `action_validation`, `projection`, or `debug_quarantine`, plus existing validation report fields. |
| **P0-07 — Debug non-diegetic output cannot feed embodied affordances, decisions, records, institutions, or tests masquerading as player knowledge.** | `P0-CERT`, `TFW`, `POS-PARITY`, `DIAG` | Audit debug capability/report seams, `view_models.rs`, TUI adversarial gates, and compile-fail fixtures that prevent external construction of debug contexts/reports. Positive proof: debug panels are visibly non-diegetic. Negative proof: debug truth cannot populate `EmbodiedViewModel`, semantic actions, actor-known context, records, or tests claiming embodied knowledge. Diagnostics name `debug_quarantine`. |
| **P0-08 — Golden fixtures include adversarial hidden-truth, no-human, possession, replay, view-model, content-validation, and direct-dispatch rejection cases.** | `P0-CERT`, `FIXTURE`, `TFW`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `DIAG` | Re-run content golden fixture tests and negative fixture runners. Positive fixture families must cover ordinary no-human life, source-backed actor-known behavior, possession parity, view-model local actions, replay determinism, and content validation. Negative families must include hidden truth, prose-born facts, forbidden provenance, raw assignment without context, unobserved food, direct dispatch, and debug omniscience. Diagnostics name responsible layer per fixture failure. |
| **P0-09 — Failure diagnostics name responsible layer.** | `P0-CERT`, `DIAG` with all above gates | Audit `agent/trace.rs`, `actions/report.rs`, `events/apply.rs`, `content/src/validate.rs`, no-human metrics, and test output assertions. Positive proof: failures carry typed or structural responsible-layer fields. Negative proof: display strings alone cannot satisfy diagnostics; debug strings cannot be parsed as proof. Diagnostics must use existing typed layers and report fields to distinguish actor-known/candidate/method/local-planning blockers, action-validation blockers, scheduler blockers, projection/view-model/test-oracle blockers, replay blockers, and content-validation blockers. |
| **P0-10 — Archived specs/tickets are historical evidence only.** | `P0-CERT`, `DIAG`, source-discipline cross-reference | The replacement artifact must cite 0036 and earlier specs/tickets only as history. Every pass row must cite live commands/evidence from the remediation checkout. It must record target commit, changed files, manifest/path-inventory discipline, exact mutation command(s), full-run completion, survivor dispositions, and explicit supersession of the 0036 artifact. Any reliance on 0036 pass rows as live proof invalidates `P0-CERT passed`. |

---

## 8. Replacement acceptance artifact specification

### 8.1 Artifact identity and verdict rule

The implementing session must produce a replacement P0-CERT acceptance artifact, expected under `archive/reports/` on closeout. A clear path is:

```text
archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md
```

The exact filename may follow repository closeout convention, but the artifact must state:

```text
Supersedes: archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md
Supersession condition: this artifact renders P0-CERT passed after completing mutation remediation and live P0-01..P0-10 re-proof.
```

If all acceptance criteria pass, the verdict is:

```text
P0-CERT passed
```

If any required live proof, mutation disposition, or replacement field is missing, the verdict remains:

```text
P0-CERT scoped remediation
```

### 8.2 Required template fields

The replacement artifact must conform to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`. At minimum, it must contain the following fields for each material evidence item:

- **Evidence status:** `pass`, `fail`, `blocked`, or equivalent template-approved status. The completed mutation posture row must be `pass` only when the full posture completes and every survivor is dispositioned.
- **Fingerprint scope:** exact target commit, changed files, audited seams, command lines, cargo-mutants version, cargo/rust toolchain, mutation output artifact names, and manifest/path inventory statement.
- **Behavior witness:** public/certified behavior demonstrated, including `0036-MUTATION-REMEDIATION-001` Arm A behavior if killed or Arm B rationale if proven equivalent/non-critical.
- **Replay/provenance ancestry:** event log references, replay reports, holder-known context IDs/hashes/frontiers, source-event IDs, projection versions, checksum evidence, or explicit reason not applicable.
- **Sampling/exhaustiveness:** explicit distinction between full-run proof and any sampled/informative survey. For mutation, this field must state that the configured posture ran to completion over all configured seams, or list shard completion proof. It must include survivor disposition counts.
- **Pending/historical:** 0036 artifact and tickets listed as historical. No pending P0-CERT obligation may remain in a passed artifact.
- **Certification use:** exact statement that later specs may cite `P0-CERT passed` only after this artifact exists and passes; if failed, later specs remain blocked.
- **Staged-abstraction declaration:** none, or explicit bounded declarations. Staged abstractions cannot cover mutation survivor disposition, actor-known provenance, debug quarantine, replay, or P0-01..P0-10 live re-proof.

### 8.3 Mandatory mutation pass row

The replacement artifact must include a row equivalent to:

```markdown
| Evidence item | Status | Fingerprint scope | Behavior witness | Replay/provenance ancestry | Sampling/exhaustiveness | Pending/historical | Certification use |
|---|---|---|---|---|---|---|---|
| Full configured P0-CERT mutation posture | pass | `cargo mutants --workspace -f ... --no-shuffle` with `.cargo/mutants.toml`, cargo-mutants 27.1.0 or recorded equivalent, target checkout, all shard outputs if sharded | Every critical survivor killed by behavior/provenance coverage or proven equivalent/non-critical; `0036-MUTATION-REMEDIATION-001` disposition recorded | `mutants.out` artifacts, logs/diffs, targeted kill reproduction, holder-known/replay evidence for behavior-killing tests | Full configured posture completed; total mutants enumerated; all `missed`/`timeout` survivors dispositioned; no untriaged survivors | 0036 mutation run treated as incomplete historical context | Supports `P0-CERT passed` only with P0-01..P0-10 pass rows |
```

### 8.4 `EMERGE-OBS` handling

Where the replacement run exercises first-proof living-world acceptance through no-human or normal-controller ordinary-life corpus, the artifact must include `EMERGE-OBS` as observer-only evidence. It may record replayable ordinary-life phenomena, event-log ancestry, and retrospective explanation. It must not treat `EMERGE-OBS` as a phase gate, pass/fail threshold, mutation score, or simulation input.

---

## 9. Invariants alignment

This spec preserves and operationalizes existing invariants only. It coins no invariants.

| Surface | Invariants preserved |
|---|---|
| Primary local-actor projection seam | `INV-002`, `INV-024`, `INV-026`, `INV-067`, `INV-093`, `INV-095`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-107`, `INV-108`. Local actors may appear in embodied view only through actor-known fact/provenance ancestry, not hidden truth or possession/debug privilege. |
| P0-01 event/replay/projection boundaries | `INV-001`, `INV-009`, `INV-010`, `INV-011`, `INV-013`, `INV-017`, `INV-018`, `INV-019`, `INV-020`, `INV-092`, `INV-098`. |
| P0-02 sealed actor-known autonomous decisions | `INV-002`, `INV-021`, `INV-024`, `INV-026`, `INV-032`, `INV-033`, `INV-041`, `INV-099`, `INV-100`, `INV-101`, `INV-102`, `INV-103`, `INV-104`, `INV-105`, `INV-106`. |
| P0-03 human commands use ordinary rules | `INV-004`, `INV-005`, `INV-006`, `INV-007`, `INV-043`, `INV-069`, `INV-087`, `INV-108`. |
| P0-04 possession non-reset | `INV-005`, `INV-006`, `INV-031`, `INV-067`, `INV-094`, `INV-108`. |
| P0-05 scheduler/no-direct-dispatch | `INV-035`, `INV-036`, `INV-039`, `INV-040`, `INV-043`, `INV-099`, `INV-100`, `INV-103`, `INV-104`, `INV-112`. |
| P0-06 validation truth split | `INV-015`, `INV-030`, `INV-043`, `INV-099`, `INV-100`, `INV-106`, `INV-112`. |
| P0-07 debug quarantine | `INV-008`, `INV-031`, `INV-068`, `INV-069`, `INV-070`, `INV-093`, `INV-095`, `INV-107`. |
| P0-08 golden/negative fixtures | `INV-091`, `INV-092`, `INV-093`, `INV-094`, `INV-095`, `INV-097`, `INV-098`, plus the relevant gate-specific invariants above. |
| P0-09 typed diagnostics | `INV-041`, `INV-070`, `INV-105`, `INV-107`. |
| P0-10 source discipline and historical-only archive use | `INV-098` and the live spec-ledger source-discipline rule. This is not a new invariant; it is evidence-honesty required to avoid certifying stale history. |
| `EMERGE-OBS` observer-only evidence | `INV-111`; it remains retrospective, replayable, and unable to feed simulation behavior or define dramatic objectives. |

Any attempted remediation that weakens these invariants is invalid even if it kills the named mutant.

---

## 10. Out of scope and tolerated deferrals

This spec certifies only the post-0008 baseline line: spine, epistemic substrate, ordinary life, possession/view-model surfaces, replay/provenance, diagnostics, and the P0-CERT mutation lock layer. It advances no forward gate.

| Deferred surface | Gate/entry lock | Treatment in this spec |
|---|---|---|
| Institutions, records, wrong suspicion, courts, bounties, sanctions, institutional fallibility beyond P0 seams | `PHASE-4-ENTRY` | Out of scope except as boundary-awareness and future proof. P0 debug/actor-known rules must not be weakened in anticipation of institutions. |
| Notices, travel, regional scale, LOD promotion, second-proof living-world expansion | `SECOND-PROOF-ENTRY` and relevant first-proof/second-proof locks | Out of scope. LOD appears only as doctrine-bound future surface preserving holder-known ancestry. |
| LLM dialogue, freeform speech, speech parsing/rendering | Future LLM/speech surfaces; LLM-disabled operation remains a P0 expectation | Out of scope except that P0 evidence must not require LLM authority. |
| Story-sifting, leads as future gameplay expansion, wrong-suspicion incident systems | Later scoped specs after P0-CERT pass | Not advanced; no story/quest outcome chain may be introduced. |
| SPINE-CERT, EPI-CERT, ORD-LIFE-CERT expansion, 0035 route-forward backlog | Blocked behind replacement `P0-CERT passed` artifact for this line | Not surveyed as alternatives and not authorized by this spec. |
| `EMERGE-OBS` quantitative thresholds | Observer-only obligation, not a gate | May be recorded retrospectively but cannot determine pass/fail or feed simulation. |

No P0-CERT obligation is deferred. The only tolerated deferrals are future-phase surfaces explicitly tied to named entry gates.

---

## 11. Risks and open questions

### 11.1 Risks

- **Tautological killing test.** A test that directly asserts a helper’s non-empty vector may kill the mutant while proving little about certified behavior. The required witness is actor-known local actor → sealed context/provenance → embodied view-model behavior, with a negative hidden-truth control.
- **Equivalent-mutant overclaim.** Equivalent mutant detection is hard and undecidable in general. A survivor must not be waived merely because existing tests did not fail.
- **Goodharting the named mutant.** Killing only `0036-MUTATION-REMEDIATION-001` leaves the 0036 incompleteness unresolved. The full configured run must complete and every survivor must be triaged.
- **Mutation run cost.** 0036 saw 1128 mutants before stopping. Sharding and timeout control may be needed, but inconsistent shards, skipped baselines without recorded proof, or missing `mutants.out` artifacts invalidate the result.
- **Timeout ambiguity.** A timeout can indicate an infinite loop, performance-sensitive semantic change, flaky test, or tool/runtime issue. Treating timeouts as green would weaken the lock layer.
- **Baseline-file abuse.** `.cargo/mutants-baseline-misses.txt` is empty at `c54caff`; adding survivors to a baseline file without behavior/equivalence triage would fail this spec.
- **Source-discipline relapse.** Reusing 0036 pass rows, branch-name claims, or repository metadata as current proof violates P0-10.
- **Debug/embodied cross-contamination.** A quick fix could populate local actors from truth or debug to satisfy a view assertion. The negative witness must prove that does not happen.
- **Line-number drift.** 0036 records `projections.rs:336`, while the exact commit’s readable file places the function elsewhere. The implementing session must use function/path identity and exact cargo-mutants output, not stale line numbers.
- **Artifact inflation.** A replacement artifact that is too broad but shallow can obscure missing proof. Every pass row must tie to concrete commands, artifacts, and responsible layers.

### 11.2 Open owner decisions

These decisions are left to the implementing session or repo owner because this spec is non-executable:

- The shard count and timeout policy for the full configured mutation run.
- Whether the primary killing witness is a core test only, a fixture-backed test, or both.
- The exact replacement acceptance-artifact filename under `archive/reports/`.
- The retention format for mutation artifacts: raw `mutants.out` upload, compressed archive, selected logs plus machine-readable summaries, or all of the above.
- Review/signoff procedure for any equivalent/non-critical disposition.
- Whether scheduled CI remains ratcheted only or is expanded after this remediation to run the full posture more frequently.

These are owner decisions about execution mechanics, not doctrine gaps. They cannot justify skipping full mutation completion or P0-01..P0-10 live re-proof.

---

## 12. Completion checklist for the implementing session

- [ ] `0036-MUTATION-REMEDIATION-001` is dispositioned by Arm A or Arm B.
- [ ] Arm A, if used, proves a real actor-known/provenance/view-model behavior consequence and includes a hidden-truth negative control.
- [ ] Arm B, if used, lists exact call sites and proves no P0-certified behavior can observe the mutation.
- [ ] `cargo fmt`, `cargo clippy`, `cargo build`, workspace tests, and lock-layer gate tests are rerun live.
- [ ] The full configured guarded-layer mutation posture runs to completion, either unsharded or with complete consistent shards.
- [ ] Every `missed` and `timeout` survivor is triaged; no untriaged survivors remain in a passed artifact.
- [ ] Every critical survivor is killed or proven equivalent/non-critical with recorded rationale.
- [ ] The triage register includes behavior witness, provenance/replay ancestry, responsible layer, and review signoff.
- [ ] P0-01 through P0-10 are re-proven live at the remediation checkout.
- [ ] Positive and negative fixture evidence is collected.
- [ ] Event/replay/projection evidence is collected.
- [ ] Actor-known provenance evidence is collected.
- [ ] Debug-quarantine evidence is collected.
- [ ] Diagnostics name responsible layers.
- [ ] Archived specs/tickets are labeled historical only.
- [ ] `EMERGE-OBS`, if present, is observer-only and not a threshold or phase gate.
- [ ] Deferrals are limited to named future gates in §10.
- [ ] Replacement artifact conforms to `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`.
- [ ] Replacement artifact explicitly supersedes 0036 and renders `P0-CERT passed` only if all criteria above are met.

---

## Appendix A — Exact-commit source inventory used while authoring

The authoring workflow used exact URLs mechanically constructed from:

```text
https://raw.githubusercontent.com/joeloverbeck/tracewake/c54caff24596f32104c4b83a1c4c6e0489636be6/<path>
```

GitHub blob URLs at the same owner/repo/commit/path were also used for readable line inspection of selected source files. The uploaded manifest was used only as path inventory. Repository metadata, default-branch lookup, code search, branch fetches, and clone were not used.

Primary raw paths fetched:

```text
archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md
archive/specs/0036_P0_CERT_POST_0008_BASELINE_CERTIFICATION_AUDIT_SPEC.md
docs/4-specs/SPEC_LEDGER.md
docs/README.md
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
docs/3-reference/01_DESIGN_RISK_REGISTER.md
docs/3-reference/02_GLOSSARY.md
docs/4-specs/README.md
docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
archive/specs/0014_PHASE_3A_ORDINARY_LIFE_NEEDS_ROUTINES_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md
archive/specs/0025_PHASE_3A_META_WITNESS_EXECUTION_PROOF_PERCEPTION_KILL_SET_PROVENANCE_ENVELOPE_READ_PATH_FAIL_CLOSED_AND_MANIFEST_FINGERPRINT_HONESTY_HARDENING_SPEC.md
crates/tracewake-core/src/projections.rs
crates/tracewake-core/src/epistemics/knowledge_context.rs
crates/tracewake-core/src/epistemics/projection.rs
crates/tracewake-core/src/view_models.rs
crates/tracewake-core/src/agent/actor_known.rs
crates/tracewake-core/src/agent/no_human_surface.rs
crates/tracewake-core/src/agent/trace.rs
crates/tracewake-core/src/scheduler.rs
crates/tracewake-core/src/actions/pipeline.rs
crates/tracewake-core/src/actions/report.rs
crates/tracewake-core/src/actions/defs/eat.rs
crates/tracewake-core/src/actions/defs/sleep.rs
crates/tracewake-core/src/actions/defs/work.rs
crates/tracewake-core/src/events/apply.rs
crates/tracewake-core/src/replay/mod.rs
crates/tracewake-core/src/replay/rebuild.rs
crates/tracewake-core/src/replay/report.rs
crates/tracewake-core/src/checksum.rs
crates/tracewake-content/src/validate.rs
crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs
crates/tracewake-content/src/fixtures/view_filtering_001.rs
crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs
crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs
crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs
crates/tracewake-core/tests/hidden_truth_gates.rs
crates/tracewake-core/tests/no_human_capstone.rs
crates/tracewake-core/tests/acceptance_gates.rs
crates/tracewake-core/tests/event_schema_replay_gates.rs
crates/tracewake-core/tests/anti_regression_guards.rs
crates/tracewake-content/tests/golden_fixtures_run.rs
crates/tracewake-tui/tests/adversarial_gates.rs
.cargo/mutants.toml
.cargo/mutants-baseline-misses.txt
.github/workflows/ci.yml
Cargo.toml
crates/tracewake-core/Cargo.toml
crates/tracewake-content/Cargo.toml
crates/tracewake-tui/Cargo.toml
```

---

## Appendix B — External method sources

These sources shaped the mutation-run and triage method. They are not Tracewake doctrine.

[^cargo-config]: cargo-mutants documentation, “Config file,” notes that `.cargo/mutants.toml` is the default source-tree config and command-line/list options merge with config values. <https://mutants.rs/config-file.html>

[^cargo-baseline]: cargo-mutants documentation, “Baseline tests,” explains that cargo-mutants normally builds and runs the unmodified tree before applying mutations, and warns that skipping baseline requires proof tests are passing. <https://mutants.rs/baseline.html>

[^cargo-timeouts]: cargo-mutants documentation, “Hangs and timeouts,” explains that cargo-mutants kills build/test after timeouts, derives default timeouts from baseline, and supports explicit timeout controls. <https://mutants.rs/timeouts.html>

[^cargo-shards]: cargo-mutants documentation, “Sharding,” defines shard `k/n` behavior and warns all shards must run with the same arguments and denominator. <https://mutants.rs/shards.html>

[^cargo-output]: cargo-mutants documentation, “The mutants.out directory,” lists `mutants.json`, `outcomes.json`, diffs, logs, and outcome text files. <https://mutants.rs/mutants-out.html>

[^cargo-results]: cargo-mutants documentation, “Using the results,” states that a missed mutant may indicate a coverage gap or an indistinguishable mutant, and recommends adding tests for public behavior affected by a private-function bug. <https://mutants.rs/using-results.html>

[^papadakis-survey]: Papadakis et al., “Mutation Testing Advances: An Analysis and Survey,” notes that equivalent/redundant mutants inflate mutation-score interpretation and that identifying equivalence/redundancy is undecidable in general. <https://mutationtesting.uni.lu/survey.pdf>

[^nguyen-madeyski]: Nguyen and Madeyski, “Problems of Mutation Testing and Higher Order Mutation Testing,” summarizes mutation-testing cost, equivalent mutants with same behavior as the original program, and the need for human effort in equivalence assessment. <https://madeyski.e-informatyka.pl/download/NguyenMadeyski14.pdf>

## Outcome

Completed: 2026-06-16

The `0037P0CERTMUTREM` ticket series completed the scoped remediation demanded
by the 0036 P0-CERT artifact.

Implemented and verified:

- `0037P0CERTMUTREM-001` added
  `hidden_truth_gates::actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`,
  a public behavior/provenance witness proving a source-backed
  actor-known local actor reaches `EmbodiedViewModel.local_actors` while a
  physically co-located but not actor-known actor stays absent. The targeted
  projection mutation posture caught
  `actor_known_local_actors_for_context -> vec![]`.
- `0037P0CERTMUTREM-002` ran the full configured guarded-layer mutation posture
  unsharded: 1128 mutants tested in 66m, 896 caught, 232 unviable, 0 missed,
  0 timeout. The triage register is
  `reports/0037_mutation_triage_register.md`.
- `0037P0CERTMUTREM-003` produced
  `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`,
  which supersedes
  `archive/reports/0036_p0_cert_post_0008_baseline_certification_acceptance.md`
  and renders `P0-CERT passed` for the scoped post-0008 baseline mutation
  remediation line.

Verification run during the series included:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace --locked`
- `cargo test --locked -p tracewake-core --test negative_fixture_runner`
- `cargo test --locked -p tracewake-core --test doc_invariant_references`
- `cargo test --locked -p tracewake-core --test spine_conformance`
- `cargo test --locked -p tracewake-core --test acceptance_gates`
- `cargo test --locked -p tracewake-core --test anti_regression_guards`
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
- `cargo test --locked -p tracewake-core --test hidden_truth_gates`
- `cargo test --locked -p tracewake-core --test acceptance_artifact_wording`
- `cargo test --locked -p tracewake-content --test schema_conformance`
- `cargo test --locked -p tracewake-content --test forbidden_content`
- `cargo test --locked -p tracewake-content --test golden_fixtures_run`
- `cargo test --locked -p tracewake-tui --test adversarial_gates`
- `cargo test --locked -p tracewake-tui --test tui_seam_conformance`
- `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle`

No P0-CERT obligation in this spec was deferred. Future certification or
feature work remains governed by the live foundation, architecture, execution,
reference, and spec ledgers; this spec does not advance Phase-4 entry, second
proof, institutions, notices, travel, LOD, LLM/speech surfaces, SPINE-CERT,
EPI-CERT, or ORD-LIFE-CERT.
