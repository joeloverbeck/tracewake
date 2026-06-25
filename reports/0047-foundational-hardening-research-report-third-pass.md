# Spec-0047 foundational-conformance hardening — third-pass research report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `0429a8f7e7fed2329319b79818f9b891da91cba2`  
**Assessment type:** cross-cutting foundational-conformance and anti-regression delta re-audit  
**Freshness scope:** user-supplied target commit only; this report does **not** independently verify that the commit is the current `main`  
**Execution scope:** static source survey only; no build, test, replay, lint, CI, or mutation command was executed by this research session

## 1. Verdict

**Not conformant.** The spec-0047 surface at `0429a8f7e7fed2329319b79818f9b891da91cba2` contains real and important improvements over both the first-pass `cb3102e` baseline and the immediate second-pass `8d7c119` baseline, but it still does not satisfy the governing foundation, architecture, execution, and reference contracts.

The third-pass static reading confirms that several 0050 changes are substantive rather than decorative:

- the one-tick coordinator stages work on scratch physical state, agent state, event log, and epistemic projection and commits those aggregates only at the transaction cutover;
- `EventLog` rejects duplicate `EventId` values on both live append and deserialization;
- aggregate replay success now includes the temporal-divergence conjunct;
- actor-known interval notices are typed and constructed through private/verified projection machinery rather than raw-diff redaction;
- the post-acceptance `0050FOUCONSEC-014`, `-015`, and `-016` tickets add credible narrow witnesses for door/container source-key tie-breaking, bounded perception-event collision handling, and field-sensitive actor-known novelty.

Those holdings do not establish foundational conformance. The live production and API shape still leave the decisive authority boundaries open:

1. the production TUI/runtime creates an empty scheduler eligibility registry and never derives loaded actors or declared processes from loaded world state or content; the prominent “production reachability” witness manually seeds both registries;
2. scheduler eligibility and process declarations are ephemeral caller-authored state that disappear on replay restore, and an actor’s next-decision eligibility is not advanced or consumed by the scheduler after an opportunity;
3. a “declared process” currently commits a generic diagnostic marker that applies as a non-world no-op, while the due-work summary increments `world_processes_applied` as though a process transition occurred;
4. controlled proposals and autonomous due-actor transactions are scheduled independently, so a possessed actor can receive two decision opportunities in one tick whenever that actor is present in the due registry;
5. the actor transaction’s full `DecisionTrace`, `LocalPlan`, and complete proposal ancestry are still not committed or returned by the world-step choreography;
6. the TUI remains the owner of authoritative physical state, agent state, event log, epistemic projection, scheduler, and controller bindings; it directly runs the action pipeline, appends perception, and rebuilds state after a separate no-human path;
7. the ordinary embodied renderer exposes exact simulation ticks, exact interval bounds, raw source `EventId` values, and controller/operator stop reasons, while the public view-model structs remain externally forgeable;
8. the standing mutation perimeter remains non-green as evidence: post-acceptance focused tickets plausibly address 35 of the 48 historical misses plus the timeout, but no fresh standing run exists, and survivor families directly guarding restore and actor-known interval correctness remain in scope;
9. live conformance and risk-evidence rows overstate closure, even though the historical `SPEC_LEDGER.md` record itself now correctly records the 0049 line and the non-green 0050 mutation posture.

The prior reports’ findings were not carried forward as presumed truths. Every current finding below is grounded independently in the exact-commit code. Conversely, properties that are now structurally present—duplicate-ID rejection, temporal replay fail-closed aggregation, scratch-state atomic cutover, and typed positive interval construction—are recorded as present rather than re-reported as defects.

**No higher-tier doctrinal amendment is warranted.** The doctrine is coherent and already says what the implementation and evidence must do. The required response is to replace open authority-bearing API shapes, repair production reachability and replay restoration, strengthen actor/process transaction semantics, de-authorize the TUI, split embodied and debug temporal products, close the in-scope mutation families, and truth the live conformance/evidence rows. Foundation text must not be weakened to fit the present code.

All command outcomes quoted from tickets or acceptance artifacts are historical claims made by those artifacts. This session did not run `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, replay/golden lanes, CI, or `cargo-mutants`. Every statement about current test strength or mutation coverage is therefore a **preliminary static judgment about source shape and witness intent**, not an authoritative command result.

## 2. Disposition table

| ID | Finding | Primary target | Classification | One-line basis |
|---|---|---|---|---|
| F-01 | Production loaded-world discovery is absent and the replacement reachability witness manufactures eligibility | `crates/tracewake-core/src/scheduler.rs`; `crates/tracewake-tui/src/app.rs`; `crates/tracewake-core/tests/world_step_coordinator.rs` | **Violation — critical; vacuity gap** | INV-004, INV-005, INV-087, INV-088, INV-091, INV-094, INV-098, and INV-108 require one real loaded-world transition; production creates empty registries while tests call the seed methods themselves. |
| F-02 | Due-actor and due-process authority is ephemeral, caller-authored, and lost on replay restore | `scheduler.rs`; `replay/temporal.rs`; `replay/rebuild.rs`; TUI restore path | **Violation — critical** | INV-009, INV-010, INV-018, INV-088, INV-092, INV-098, and INV-112 require replay/recovery to reconstruct authoritative temporal work, not only the clock frontier. |
| F-03 | Declared-process “application” is a diagnostic no-op rather than a process-specific causal transition | `scheduler.rs`; `events/envelope.rs`; `events/apply.rs` | **Violation — critical; evidence overclaim** | INV-001, INV-009, INV-010, INV-018, INV-088, and INV-098 require declared causal processes and eventful effects; a generic marker and counter are not process behavior. |
| F-04 | A possessed actor can receive both a controlled and an autonomous opportunity in one tick | `scheduler.rs` world-step actor census | **Violation — critical structural gap** | INV-004, INV-005, INV-043, INV-087, INV-094, and INV-108 require ordinary possession and one shared transition, not two independent choreographies for the same actor/body. |
| F-05 | The actor decision transaction is still only partially consumed | `agent/transaction.rs`; `agent/trace.rs`; `scheduler.rs` | **Violation — high** | INV-018, INV-041, INV-098, INV-101, INV-102, and INV-105 require the selected method/plan, alternatives, blockers, ancestry, and resulting events to survive as authoritative trace data. |
| F-06 | The TUI remains an authoritative writer; its perception compile-fail witness is vacuous | `crates/tracewake-tui/src/app.rs`; `agent/perception.rs`; negative fixture and runner | **Violation — critical; vacuity gap** | INV-009, INV-018, INV-067, INV-069, INV-101, INV-102, and INV-112 plus architecture `10` deny event/state/projection ownership to the client. The fixture proves only that a nonexistent symbol is absent. |
| F-07 | Embodied output leaks exact replay/control metadata and its public products are forgeable | `view_models.rs`; `tracewake-tui/src/render.rs` | **Violation — high; hardening gap** | INV-067, INV-068, INV-093, INV-101, INV-102, and INV-112 reserve exact replay time and controller logistics for non-diegetic surfaces and require holder-known construction. |
| F-08 | The standing mutation floor still contains in-scope foundational survivor families | `0050FOUCONSEC-011…016`; `scheduler.rs`; `epistemics/projection.rs` | **Mutation-survivor disposition — high** | INV-018, INV-092, INV-093, INV-098, INV-101, INV-102, and INV-112 require measured evidence; focused post-acceptance runs do not establish a fresh standing result. |
| F-09 | Live conformance and risk evidence overstate production closure | architecture conformance `00`; execution `05`, `07`, `10`; reference R-27/R-28/R-29 | **Hardening/evidence-honesty gap — high** | Execution `10` and R-27/R-29 prohibit declaration-as-proof and decorative locks; live rows describe properties contradicted by the exact-commit production path. |

## 3. Method and provenance ledger

### 3.1 Authority order and delta posture

The audit applied the repository’s stated authority order without inversion:

```text
0-foundation
  -> 1-architecture
  -> 2-execution
  -> 3-reference
  -> 4-specs
  -> archived specs/tickets/acceptance as historical promises and evidence
  -> live code and tests as the implementation under review
```

The constitution remained the controlling contract, especially causality and event sourcing (INV-001, INV-009, INV-010, INV-018), no-human and possession neutrality (INV-004, INV-005, INV-006, INV-087, INV-091, INV-094, INV-108), declared processes (INV-088), replay testing and harsh acceptance (INV-092, INV-098), TUI/embodied boundaries (INV-067 through INV-069), cognition/trace and truth-firewall duties (INV-041, INV-099, INV-101 through INV-105), and temporal authority (INV-112).

The immediate predecessor report at `8d7c119` and the first-pass report at `cb3102e` were used only as **pre-remediation baselines**. Their findings were not assumed current. Each relevant code and evidence seam was re-read at `0429a8f7e7fed2329319b79818f9b891da91cba2`; where 0050 genuinely changed the code, this report assesses the new shape rather than repeating the old finding.

### 3.2 Exact-commit acquisition provenance

The uploaded manifest was used only as an inventory of paths authorized for the user-supplied commit. Every repository file used for target-state claims was fetched through a full exact-commit URL containing `joeloverbeck`, `tracewake`, the full SHA, and the exact manifest path. No repository clone, branch fetch, code search, snippet reconstruction, repository metadata lookup, default-branch lookup, or repository-scoped connector argument was used. Requested and resolved transport provenance remained on the target repository, commit, and path. Foreign repository names appearing inside validly fetched files were treated as ordinary file content.

```text
Requested repository: joeloverbeck/tracewake
Target commit: 0429a8f7e7fed2329319b79818f9b891da91cba2
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw URL); container.download after verification for retained plain-text files
Requested file count: 152
Successfully verified file count: 152
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

The complete append-only URL inventory is supplied beside this report as [`tracewake_0429a8f_exact_commit_acquisition_ledger.txt`](../tracewake_0429a8f_exact_commit_acquisition_ledger.txt).

### 3.3 Evidence posture and existing machinery

This survey accounted for the repository’s existing anti-regression infrastructure instead of proposing substitutes:

- `.cargo/mutants.toml` already covers the scheduler, replay, projection, pipeline, perception, and TUI/core seams under review;
- `anti_regression_guards.rs` already exists for narrow topology alarms;
- `negative_fixture_runner.rs` and the external-crate fixture pattern already exist for compile-time unrepresentability;
- `generative_lock.rs` already provides a deterministic generated corpus;
- dedicated coordinator, temporal replay, interval projection, salience, reservation, TUI parity, and adversarial tests already exist.

Recommendations therefore extend those mechanisms. Source-text guards are treated only as topology alarms. They are never proposed as proof of atomicity, replay, information-flow noninterference, production reachability, or process semantics. No `proptest` or `quickcheck` dependency is recommended: the required properties can be expressed by the existing deterministic fixtures, generated corpus, integration seams, negative fixtures, and mutation campaigns.

### 3.4 Static-survey limitation

This report can establish source shape, API authority, data flow, test intent, and whether a witness appears structurally capable of observing a regression. It cannot certify current command outcomes. Every executable check named below belongs to the implementing session, which must begin from a clean baseline and retain the exact output artifacts required by execution `10`.

## 4. Findings

## F-01 — Production loaded-world discovery is absent, and the replacement reachability witness manufactures eligibility

### Foundational driver

INV-004 requires the authoritative world to run coherently without a human. INV-005 and INV-087 restrict possession to input binding rather than a privileged simulation path. INV-088 requires world processes to be declared causal processes. INV-091 and INV-094 require no-human and possession-parity evidence, while INV-098 requires the feature to survive harsh, production-shaped acceptance. Foundation `08` defines waiting as one input slot in which other loaded actors, world processes, and due consequences advance in the same transition. Architecture `04` and execution `05` assign discovery and execution of due actors/processes to one core-owned world-step coordinator.

### Current `0429a8f` code state

`DeterministicScheduler::new` initializes both `loaded_actor_next_decision_tick` and `declared_world_processes` as empty maps. `due_loaded_actor_ids` derives due actors only from the former map; `due_process_invocations` derives due processes only from the latter. The only writers are public methods on the scheduler: `schedule_loaded_actor_decision` and `register_cadenced_world_process` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/scheduler.rs), symbols `DeterministicScheduler::new`, `due_loaded_actor_ids`, `due_process_invocations`, `schedule_loaded_actor_decision`, and `register_cadenced_world_process`).

The production TUI fixture bootstrap constructs `DeterministicScheduler::new(SimTick::ZERO)` and does not derive or register any actor or process eligibility ([`app.rs` lines 98–132](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L98-L132)). The launch and run layers do not fill those registries either. Consequently, the normal TUI’s wait/advance path can execute duration/accounting and controlled work, but it cannot autonomously discover another loaded actor or any declared process from the loaded fixture.

The prominent replacement differential witness does not cross that production boundary. Its helper explicitly calls `schedule_loaded_actor_decision` and `register_cadenced_world_process` before invoking the coordinator ([`world_step_coordinator.rs` helper around lines 1059–1119](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/tests/world_step_coordinator.rs#L1059-L1119)). That test is useful for coordinator composition *after eligibility has been manufactured*, but it does not prove that production loading produces eligibility. Calling the result “authoritative loaded-world differential” or “production reachability” therefore overstates what is exercised.

This is not the second-pass F-01 carried forward unchanged. The old raw request collections were removed and the maps are now scheduler-owned. The live defect is the next layer: no authoritative production bootstrap or replay-derived source populates those maps, while tests retain direct seeding authority.

### Conformance verdict

**Critical violation and vacuity gap.** The core can process due work only after a caller supplies the very scheduling facts whose production derivation the doctrine assigns to core. The normal TUI path therefore does not realize “waiting runs the loaded world,” and the named replacement witness proves composition rather than production reachability.

### Required remediation

**Code home:** introduce a core-owned loaded-world runtime/session boundary adjacent to `scheduler.rs`, fixture/content loading, replay rebuild, and controller binding. It must derive the initial actor census and declared process registry from authoritative loaded content/state and maintain eligibility as part of the core runtime. TUI construction should receive that already-initialized owner rather than construct a blank scheduler.

External crates and clients must not be able to seed loaded actors or process declarations directly. Remove or restrict `schedule_loaded_actor_decision` and `register_cadenced_world_process`; do not leave compatibility aliases. If tests need deterministic setup, expose a test-only builder inside the core crate or construct a real loaded package whose declarations produce the expected runtime state.

**Documentation home:** architecture conformance `00`, architecture `04`, execution `05`, execution `06`, execution `10`, and the evidence/status paragraphs under existing R-27/R-29. No foundation amendment is needed.

### Strongest practical anti-regression guard

1. **Compile-time:** external-crate negative fixtures must fail when attempting to seed actor eligibility or process registrations. The failure must be privacy/constructor authority on the actual production symbols, not absence of an invented symbol.
2. **Production behavior:** load a two-actor fixture plus one declared process through the same bootstrap used by `TuiApp::from_golden`; bind one actor; submit one wait; assert the unpossessed actor transaction and process effect appear without any test call to scheduler registration methods.
3. **Differential:** run the same loaded package with and without a controller binding and compare the non-controller world/event/projection result.
4. **Mutation:** kill empty actor discovery, empty process discovery, boundary inversions, and omission of loader-to-runtime registration through the public runtime entry point.

### Evidence-honesty check

The closure witness must fail if all explicit scheduler registration calls are deleted from the test because there must be no such calls. A helper that directly inserts actor IDs, process IDs, cadence, or trigger evidence is setup for a coordinator unit test, not proof of production discovery.

## F-02 — Due-work authority is ephemeral, caller-authored, and lost on replay restore

### Foundational driver

INV-009 and INV-010 require meaningful scheduling consequences to remain eventful and causally explained. INV-018 and INV-092 require deterministic replay. INV-088 makes processes declared causal entities rather than anonymous callbacks. INV-112 requires every accepted step—including an empty one—to carry ancestry sufficient to reconstruct the temporal frontier. Architecture `02` extends that obligation through replay/save/restore; execution `05` says due work belongs to the core world-step boundary.

### Current `0429a8f` code state

The scheduler’s due-work registries are ordinary in-memory maps. `restore_from_temporal_projection` and `restore_from_rebuild_report` each return `Self::new(reconstructed_final_frontier)`, reconstructing only the clock and discarding loaded-actor eligibility, process declarations, cadence, trigger provenance, and proposal sequence state ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/scheduler.rs), restore symbols).

The separate TUI debug/no-human path invokes replay rebuild and then replaces the scheduler with that clock-only restoration ([`app.rs` lines 375–414](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L375-L414)). Any scheduler registrations that existed before the run are therefore lost.

The public registration methods also admit arbitrary caller-authored actor IDs, process IDs, cadence, source event IDs, content manifest identity, and random provenance. The fields are private, but the authority is not: the public methods are equivalent write channels.

Finally, the scheduler does not consume or advance `loaded_actor_next_decision_tick` after it runs an actor transaction. Within the scheduler implementation, the only mutation of that map is the public seeding method. Once an actor has `next_decision_tick <= target_tick`, the actor remains due on every later tick unless some outside caller rewrites the map. That is not a self-contained decision cadence and cannot be recovered from the event log.

Historical standing survivor #10 is directly diagnostic: replacing `restore_from_temporal_projection` with `None` survived the recorded standing campaign. That historical result cannot certify current behavior, but it identifies the exact restore boundary that still lacks a production continuation witness.

### Conformance verdict

**Critical violation.** The current replay boundary can rebuild the temporal frontier while silently dropping the authoritative work needed to continue the same loaded simulation. A replay that reconstructs state and clock but resumes with different actor/process eligibility is not deterministic continuation.

### Required remediation

**Code home:** make loaded-actor eligibility and process declarations one of the following, with one chosen design used consistently:

- event-sourced runtime declarations and next-opportunity updates projected during rebuild;
- deterministic derivations from canonical loaded state/content plus event history;
- a dedicated replay-derived runtime projection included in save/restore and checksums.

The result must include process cadence/trigger provenance and actor next-decision state. Proposal ordering state must either be reconstructed or replaced by ordering derived entirely from event frontier and stable identities. After an actor opportunity, core must deterministically record or derive the actor’s next opportunity; no outside owner should be required to reschedule it.

`restore_from_temporal_projection` should not claim complete scheduler restoration if its input cannot reconstruct all runtime authority. Either accept the complete runtime projection or narrow/rename it and prevent production use as a full restore.

**Documentation home:** architecture `02`, architecture `04`, execution `05`, execution `09`/`10` evidence rows, architecture conformance `00`, and existing R-08/R-09/R-27/R-29 status evidence.

### Strongest practical anti-regression guard

1. **Type/ownership:** a core-owned runtime with private due-work state and no external registration methods.
2. **Continuation equivalence:** load a world with at least two actors and a cadenced process; run `N` ticks, serialize/rebuild, run `K` more; compare the uninterrupted and restored physical state, agent state, event log suffix, epistemic projection, temporal frontier, actor-opportunity sequence, process invocations, and returned summaries.
3. **Cadence boundary:** prove an actor is not perpetually due after one opportunity unless the modeled next-decision rule says so.
4. **Mutation:** kill restore-to-`None`, restoration that drops either registry, off-by-one next-decision updates, and proposal-order reconstruction changes.

### Evidence-honesty check

A replay checksum that excludes due-work state cannot close this finding. The witness must continue execution after restore and compare future behavior. A test that manually re-registers actors/processes after restore proves caller repair, not replay reconstruction.

## F-03 — Declared-process “application” is a diagnostic no-op, not a process-specific causal transition

### Foundational driver

INV-001 requires events to arise from modeled causes; INV-009 and INV-010 require meaningful changes and explainable causes; INV-018 requires replayable consequences; INV-088 requires regional/world processes to be declared causal processes; INV-098 rejects marker-only acceptance. Architecture `04` and execution `05` require due processes to enter the ordinary proposal/validation/event/projection boundary rather than bypassing it or substituting a declaration marker for behavior.

### Current `0429a8f` code state

The scheduler’s private `DueProcessInvocation` carries useful trigger metadata, but `build_declared_world_process_event` turns it into a generic `NoHumanAdvanceStarted` event. `EventKind::NoHumanAdvanceStarted` is classified on the diagnostic stream, and `apply_event_stream` treats diagnostic, controller, and replay-debug events as `NonWorldNoOp` ([`events/envelope.rs` around the event-stream mapping](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/events/envelope.rs); [`events/apply.rs` lines 44–70](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/events/apply.rs#L44-L70)).

`transact_world_one_tick` appends and “applies” that diagnostic event, then increments `world_processes_applied` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/scheduler.rs), process loop inside `transact_world_one_tick`). The coordinator therefore reports an applied world process even though the physical, agent, and epistemic application layer deliberately performs no world transition for the event.

The named cadence test checks the marker’s identity/provenance and the applied counter but not any process-specific state or projection consequence ([`world_step_coordinator.rs` process test around lines 699–763](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/tests/world_step_coordinator.rs#L699-L763)). It is non-vacuous for the private cadence predicate and marker ancestry; it is vacuous as proof that a declared process actually runs.

### Conformance verdict

**Critical causal-process violation and evidence overclaim.** The registry now owns trigger selection, which is an improvement over raw process envelopes, but the transition terminates in a diagnostic marker. Counting that marker as a process application makes observability stronger than behavior.

### Required remediation

**Code home:** define a closed process declaration/dispatch registry inside core. A due invocation should identify a process kind and trigger witness; the process transaction should build an ordinary typed proposal or a closed process-transition command; validation should produce process-specific world/agent/epistemic events; those events should be appended and applied through the same atomic cutover as actor work. Diagnostic marker events may accompany the transition, but they cannot be the transition.

The process declaration must be loaded from canonical content or runtime state and reconstructed on replay as required by F-02. `world_processes_applied` should count committed process transactions, not diagnostic envelopes.

**Documentation home:** architecture `04`, execution `05`, architecture conformance `00`, execution `10`, and existing risk evidence for R-08/R-27/R-28/R-29.

### Strongest practical anti-regression guard

1. **Type-level:** private `DueProcessInvocation` plus a closed process-kind dispatch that cannot accept a completed `EventEnvelope` from a caller.
2. **Behavior:** load a declared process whose due transition changes a small authoritative state/projection fact; run one tick immediately before cadence and one at cadence; assert no effect before, one effect at cadence, correct ancestry, and identical replay.
3. **Negative:** prove external crates cannot construct due invocations or inject raw process events.
4. **Mutation:** kill process-builder bypass, diagnostic-only substitution, wrong process ID/source event, cadence inversions, and increment-without-commit.

### Evidence-honesty check

A witness that asserts only `NoHumanAdvanceStarted`, an invocation count, or a diagnostic payload does not prove process behavior. Closure requires a process-specific authoritative consequence and replay of that consequence.

## F-04 — Controlled and autonomous actor opportunities are not mutually exclusive within a tick

### Foundational driver

INV-004 and INV-005 require possession-neutral simulation and input-binding-only control. INV-043 requires ordinary-agent validation for world-affecting actions. INV-087 and INV-094 require held-equal human/NPC behavior. INV-108 requires one authoritative world-step coordinator. Execution `05` permits the scheduler to select opportunities but forbids a separate possessed-actor behavior path.

### Current `0429a8f` code state

`transact_world_one_tick` processes controlled proposals first, then derives `due_actor_ids` from the scheduler registry, and then runs `ActorDecisionTransaction` for every due actor. There is no exclusion for an actor already represented in `controlled_proposals`, no controller-binding check in the due-actor census, and no closed per-tick actor disposition table. Autonomous transactions explicitly call the pipeline with `controller_bindings: None` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/scheduler.rs), controlled-proposal phase and due-actor loop inside `transact_world_one_tick`).

Today’s normal TUI avoids the collision only accidentally because it never populates loaded-actor eligibility. The core API nevertheless permits a caller or test to schedule the possessed actor as due and submit that actor’s controlled proposal in the same request. Once F-01 is repaired by deriving all loaded actors, this latent split becomes a direct production double-opportunity unless the census is redesigned first.

The existing body-exclusive reservation census guards simultaneous duration ownership, not duplicate cognition/action opportunities for an actor who is not currently reserved.

### Conformance verdict

**Critical structural violation.** The one-tick coordinator does not define exactly one disposition per loaded actor. The same ordinary actor can be both human-controlled and autonomously planned in one tick, producing two proposals/attempts and potentially two committed actions for one body.

### Required remediation

**Code home:** before executing proposals, build one closed, exhaustive per-tick actor census. A suitable internal enum would represent dispositions such as:

- controlled attempt for the bound actor;
- autonomous decision opportunity;
- body-exclusive duration continuation/reservation;
- deferred/not-due;
- stuck/blocked outcome.

Every loaded actor must receive exactly one disposition. The possessed actor is not special in rules, but a valid controlled input consumes that tick’s ordinary actor opportunity. Duration reservation must take precedence consistently for both human and autonomous actors. The enum and census constructors should remain private to core; external callers submit commands, not dispositions.

**Documentation home:** architecture `04`/`05`, execution `05`, execution `06`, architecture conformance `00`, and existing R-10/R-27/R-28 evidence.

### Strongest practical anti-regression guard

1. **Compile-time/exhaustiveness:** a closed internal `ActorDisposition`/`ActorOpportunity` enum whose variants must be handled before commit.
2. **Behavior:** one loaded tick with a controlled possessed actor, a second unpossessed due actor, and a third actor under body-exclusive duration; assert exactly one disposition per actor, at most one ordinary proposal per actor, and no controller-specific validation rule.
3. **Permutation/differential:** reorder loaded actors and controlled inputs and prove canonical output ordering and possession parity.
4. **Mutation:** kill possessed-actor exclusion, duplicate-disposition checks, duration precedence, and one-opportunity cardinality.

### Evidence-honesty check

Separate tests for controlled proposals and autonomous actors do not prove mutual exclusion. The same tick must contain both classes, and the possessed actor must also be present in the loaded census so the negative assertion is reachable.

## F-05 — The actor decision transaction is still only partially consumed

### Foundational driver

INV-041 requires debug traces to preserve beliefs used, needs/duties considered, active intention, selected method/plan, rejected alternatives, blocked preconditions, random draws, interruptions, and resulting events. INV-099 prevents hidden truth from becoming planning authority. INV-101 and INV-102 seal the actor-known context and require source-bearing cognition inputs. INV-105 makes decision traces, stuck diagnostics, lifecycle changes, and planner outputs authoritative diagnostic data. Architecture `05`, foundation `14`, and execution `05` define one canonical cognition transaction whose typed result is consumed by the scheduler rather than selectively reinterpreted.

### Current `0429a8f` code state

`ActorDecisionTransactionOutcome::Proposed` carries five load-bearing products: a sealed proposal, full `DecisionTrace`, reduced `DecisionTraceRecord`, lifecycle effects, and `LocalPlan` ([`agent/transaction.rs` lines 28–43](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/agent/transaction.rs#L28-L43)). `SelectedGoalBundle` also computes a `local_plan_id` and a proposal ancestry list containing the trace, candidate, and method identities ([lines 278–305](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/agent/transaction.rs#L278-L305)).

The final proposal persists the decision-trace ID, candidate-goal ID, routine template ID, and routine execution ID, but it does not persist `local_plan_id` or the bundle’s complete ancestry vector ([lines 344–367](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/agent/transaction.rs#L344-L367)).

The scheduler’s proposed branch clones `decision_trace_record` and lifecycle effects, consumes the proposal, runs the pipeline, appends lifecycle events, and appends a trace event built from the reduced record. It never consumes `proposed.decision_trace` or `proposed.local_plan` ([`scheduler.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/scheduler.rs), `ActorDecisionTransactionOutcome::Proposed` branch).

`DecisionTraceRecord` records IDs, window bounds, outcome, candidate count, a context hash/input list, hidden-truth audit, and a compact typed diagnostic ([`agent/trace.rs` lines 266–323](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/agent/trace.rs#L266-L323)). It does not preserve the full selected method/plan, all rejected alternatives, blocked preconditions, planner structure, or the direct linkage from local plan to resulting event set required by INV-041.

The 0050 change is therefore real but incomplete. `Stuck` is no longer silently discarded, lifecycle effects are committed, and pipeline status appears in `ActorStepSummary`. The remaining defect is that the canonical transaction returns more authoritative trace/planning data than the coordinator commits or exposes.

### Conformance verdict

**High-severity foundational violation.** The scheduler invokes the correct cognition boundary but still reduces its typed result. The reduction makes the full plan/trace products decorative and prevents replay/debug from reconstructing the complete “why this concrete action” chain.

### Required remediation

**Code home:** replace the loose public-field outcome with a closed, commit-ready internal actor-step transition. Its private fields must include the proposal, full trace, trace record/render projection, local-plan identity and plan steps required by diagnostics, lifecycle effects, and explicit ancestry links. The scheduler must exhaustively consume that transition and return a typed committed summary linking:

```text
actor-known context
  -> candidate selection
  -> selected method
  -> local plan
  -> proposal
  -> validation/pipeline result
  -> ordinary event(s)
  -> lifecycle/trace/stuck artifacts
```

Do not solve this by serializing opaque debug prose. Persist typed IDs/fields sufficient to rebuild the chain. If the full plan is intentionally too large for every event, persist a canonical plan record/projection with stable identity and event ancestry.

**Documentation home:** foundation `03`/`14` clarification by conformance status only, architecture `04`/`05`, execution `05`/`06`/`10`, architecture conformance `00`, and existing R-11/R-27/R-28/R-29 evidence. No doctrine wording change is required.

### Strongest practical anti-regression guard

1. **Compile-time:** keep the outcome closed and non-reducible; external callers must not construct it, and internal scheduler handling must be exhaustive. Extend the existing `external_crate_cannot_reduce_actor_step_outcome_to_option` pattern to guard actual private fields/constructors, while recognizing that this alone does not prove consumption.
2. **Behavior:** through `transact_world_one_tick`, produce (a) a successful planned action, (b) a `Stuck` result, and (c) a rejected proposal. Assert typed ancestry from context/trace/method/plan through resulting events and lifecycle/diagnostic artifacts.
3. **Replay:** rebuild the diagnostic/actor-step projection and compare canonical trace/plan identities and links.
4. **Mutation:** kill omission of each artifact class, missing local-plan ancestry, trace reduction, `Stuck` suppression, and ignored pipeline status.

### Evidence-honesty check

A unit test that calls `ActorDecisionTransaction::run` proves that the transaction can produce the data, not that the world-step commits it. Closure must inspect the log and returned core result after the real coordinator path, and a deliberate deletion of plan/trace consumption must fail the test.

## F-06 — The TUI remains an authoritative writer, and the perception compile-fail witness is vacuous

### Foundational driver

INV-009 and INV-018 require eventful, replay-safe mutation. INV-067 confines embodied output to actor-known reality. INV-069 states that the TUI must not implement simulation rules. INV-101 and INV-102 protect the actor-known boundary and provenance. INV-112 separates authoritative replay time from holder-known interpretation. Architecture `10` is explicit: the TUI asks core for a typed world step, does not apply events, mutate world state, own durations, or hold a local clock. Execution `05` likewise prohibits any TUI path from mutating authoritative state directly.

### Current `0429a8f` code state

`TuiApp` owns the authoritative action registry, initial and current physical state, initial and current agent state, `EventLog`, `ControllerBindings`, `DeterministicScheduler`, and `EpistemicProjection` ([`app.rs` lines 74–91](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L74-L91)). This is not a read-only client cache; methods pass mutable references directly into core mutation functions.

Specific write paths remain:

- bind attaches the controller directly to the log and invokes scheduler-owned perception append/projection ([lines 143–167](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L143-L167));
- non-wait actions construct `PipelineContext` in the TUI and call `run_pipeline` directly with mutable state/log/projection ([lines 260–325](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L260-L325));
- after accepted actions, the TUI appends/project current-place perception ([lines 326–340](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L326-L340));
- `advance_until` passes all mutable aggregates into the scheduler and then performs a public `From<ActorKnownIntervalDelta>` conversion itself ([lines 348–372](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L348-L372));
- `run_no_human_day` executes a separate scheduler path, rebuilds projections, replaces authoritative state/agent/projection/scheduler, and then appends perception ([lines 375–414](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/app.rs#L375-L414)).

The underlying writer is public in core: `record_current_place_perception_and_project` accepts mutable log, physical state, agent state, and projection ([`agent/perception.rs` lines 71 onward](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/agent/perception.rs#L71-L85)). The scheduler exposes another public forwarding writer.

The advertised compile-fail witness does not guard either real symbol. Its entire body is:

```rust
pub fn call_tui_perception_append_helper() {
    tracewake_tui::record_current_place_perception_and_project();
}
```

([negative fixture](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/src/lib.rs)). The negative runner expects only the generic fragment `cannot find function` ([`negative_fixture_runner.rs`](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/tests/negative_fixture_runner.rs)). The fixture therefore proves that a nonexistent, no-argument function is not exported by the TUI crate. It says nothing about the real public core/scheduler writer APIs, and it would continue to fail even if those APIs remained fully callable—as they do.

### Conformance verdict

**Critical TUI-authority violation and clear vacuity gap.** The TUI is still the aggregate owner and mutation orchestrator. The compile-fail evidence is decorative because it does not attempt the prohibited production capability.

### Required remediation

**Code home:** introduce one core-owned `SimulationSession`/`LoadedWorldRuntime` that owns all authoritative mutable aggregates: physical state, agent state, event log, epistemic projection, scheduler/runtime eligibility, process registry, controller bindings, and replay/save state. The TUI should own only client/session handle, selected presentation state, command parsing state, and immutable typed results.

The public core boundary should accept typed commands such as bind/unbind, submit semantic action, one-tick wait, duration continuation/advance-until, and debug-authorized no-human execution. Every world-affecting semantic action—including non-wait actions—must go through that boundary. Bind-time perception, if retained, must occur inside core as a modeled channel. Remove public event-appending perception helpers and public scheduler forwarding methods from external reach.

The separate no-human path should become a command on the same runtime, not a TUI-managed rebuild/replacement choreography. Debug authorization may select a non-diegetic output product; it must not transfer aggregate ownership to the client.

**Documentation home:** architecture `10`, execution `05`, execution `07`, architecture conformance `00`, reference checklist `00`, and R-27/R-29 evidence. Foundation doctrine is already sufficient.

### Strongest practical anti-regression guard

1. **Compile-time:** external-crate negative fixtures must attempt the actual former APIs with realistic arguments and fail because the mutation function, constructor, or fields are private. Add fixtures proving the TUI crate cannot construct `PipelineContext` over authoritative aggregates, append perception, replace replay state, or access mutable runtime fields.
2. **Architecture dependency:** expose no `&mut PhysicalState`, `&mut AgentState`, `&mut EventLog`, or `&mut EpistemicProjection` across the TUI/core public boundary.
3. **Behavior:** TUI command-loop tests use only typed session commands, then compare returned view/results and replayed state. A source guard may alarm on imports of `run_pipeline`, `PipelineContext`, perception append helpers, or replay rebuild inside `tracewake-tui`, but it is secondary.
4. **Mutation:** kill command-routing bypasses and any client-side conversion that alters authoritative state.

### Evidence-honesty check

The replacement negative fixture must name the real production symbol and fail for the intended privacy reason. An expected error such as “cannot find function” is acceptable only when the function previously existed and was deliberately made unreachable; an invented name or wrong arity is not proof.

## F-07 — Embodied output leaks exact replay/control metadata, and its public products are forgeable

### Foundational driver

INV-067 says embodied mode shows actor-known reality; INV-068 requires debug mode to be visibly non-diegetic; INV-093 makes actor-knowledge leakage high severity. INV-101 and INV-102 require sealed, source-bearing holder-known construction. INV-112 separates authoritative temporal ordering from actor-known temporal interpretation. Architecture `10` permits exact event/replay time, due queues, and hidden temporal comparisons only in debug/operator panels; actor-facing time and interval summaries must derive from modeled observations, records, cues, or holder-known projections.

### Current `0429a8f` code state

The default embodied renderer prints authoritative simulation time directly:

```text
Actor: <id> | Tick: <exact SimTick>
```

([`render.rs` lines 45–49](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/render.rs#L45-L49)). It also prints exact interval start/stop ticks, the internal stop-reason identifier, and each notice’s raw source `EventId` ([lines 91–106](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/render.rs#L91-L106)). A renderer test explicitly locks actor-facing `controller_safety_bound` output ([lines 384–413](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-tui/src/render.rs#L384-L413)). `UserPausedBeforeNextTick` and `ControllerSafetyBound` are controller/operator facts, not facts the actor acquired through a modeled channel.

The type boundary does not prevent fabrication. Every field of `EmbodiedViewModel` is public, including `sim_tick`, holder-known frontier/hash/source summary, interval summary, and debug availability. Every field of `TypedActorKnownIntervalSummary` is public, including exact frontiers and stop reason; a public `From<ActorKnownIntervalDelta>` copies the internal product wholesale ([`view_models.rs` lines 18–67](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/crates/tracewake-core/src/view_models.rs#L18-L67)). External code can therefore construct an apparently embodied summary with controller/debug values without passing through the verified holder-known constructor.

The interval delta’s internal verified notices are a real improvement. The violation is the next conversion/render layer: it conflates internal temporal receipt, actor-known change, and operator stop logistics into one public actor-facing type.

### Conformance verdict

**High-severity truth-firewall violation and structural hardening gap.** Normal embodied output leaks exact replay/control metadata, and the public type shape allows downstream callers to forge the product the truth firewall is supposed to guarantee.

### Required remediation

**Code home:** split the current product into at least two sealed types:

- an internal/debug `WorldAdvanceReceipt` containing exact ticks/frontiers, controller stop reason, appended event IDs, and diagnostic details;
- an actor-facing `EmbodiedIntervalSummary` containing only positively constructed actor-legible notices and temporal language supported by modeled sources.

`UserPausedBeforeNextTick` is client UI state and need not appear as world knowledge. `ControllerSafetyBound` belongs to an operator/debug receipt. Exact ticks or clock labels may appear in embodied mode only when a modeled actor-known clock/calendar/record supports them; otherwise render qualitative or source-derived interval language. Raw event IDs can remain internal provenance handles but should not be printed as diegetic content.

Make fields private and constructors crate-private. Expose read-only getters appropriate to each surface. Do not implement debug-to-embodied conversions. The TUI should receive the already-separated product from core.

**Documentation home:** architecture `03`/`10`, execution `04`/`07`, architecture conformance `00`, reference checklist, and R-09/R-13/R-15/R-27/R-29 evidence.

### Strongest practical anti-regression guard

1. **Compile-time:** negative fixtures prove external crates cannot construct embodied summaries, mutate their source/frontier fields, or convert debug receipts to embodied products.
2. **Behavior:** default embodied transcripts assert absence of exact replay tick/frontier, raw `EventId`, `controller_safety_bound`, and pause logistics; debug overlay tests assert their presence under explicit debug capability.
3. **Information-flow differential:** feed identical holder-known contexts with different hidden current ticks/debug receipts and prove embodied output is unchanged unless an actor-known temporal source differs.
4. **Mutation:** kill source omission, debug/embodied conversion, stop-reason routing, and actor-known temporal-source checks.

### Evidence-honesty check

A renderer snapshot that merely contains a typed interval value does not prove holder-known safety when tests can instantiate the public struct directly. The witness must obtain the embodied product from the core production constructor and must include hidden-time/debug perturbation negatives.

## F-08 — The standing mutation floor remains unresolved for the current commit

### Foundational driver

INV-018 and INV-092 require deterministic replay and its tests. INV-093, INV-101, and INV-102 require strong actor-known/provenance enforcement. INV-098 rejects unmeasured acceptance. INV-112 requires temporal authority and restoration to fail closed. Execution `10` treats missed mutants and timeouts as actionable evidence until classified and requires tests at the real public behavior boundary.

### Current evidence state

`0050FOUCONSEC-011` records a historical standing campaign at the `-013` acceptance point: 3,182 selected, 2,458 caught, 675 unviable, 48 missed, and 1 timeout ([ticket lines 115–178](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/archive/tickets/0050FOUCONSEC-011.md#L115-L178)). The acceptance artifact correctly refuses to call that perimeter green.

Three tickets landed after the acceptance:

- `-014` reports a focused 16-mutant run for door/container tie-breaking: 12 caught, 4 unviable, 0 missed ([outcome](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/archive/tickets/0050FOUCONSEC-014.md#L84-L95));
- `-015` reports a corrected focused 8-mutant run for bounded perception collision/rename: 7 caught, 1 unviable, 0 missed, after explicitly rejecting an initial stale-diff command as evidence ([outcome](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/archive/tickets/0050FOUCONSEC-015.md#L100-L112));
- `-016` reports a focused 57-mutant novelty run: 56 caught, 1 unviable, 0 missed ([outcome](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/archive/tickets/0050FOUCONSEC-016.md#L83-L94)).

By static reading, those witnesses are credible for the narrow functions they target. They plausibly address historical survivors 8–9, 11–14, 18–46, and the perception timeout. They do **not** establish a current full standing denominator or result because no post-`-016` standing campaign is recorded.

### Survivor-by-survivor/family disposition

| Historical survivor(s) | Property | Third-pass disposition | Basis and required action |
|---|---|---|---|
| 1–7: `food_source_fact_supersedes` constants/arms/comparison | food-source belief precedence | **Route forward — cross-cutting, out of this surface** | Foundational epistemic correctness, but not load-bearing to the spec-0047 loaded-world/time-control transition. Owner/reassess should place it in a future cross-cutting mutation-remediation spec, not a lower tier. No equivalence claim is made. |
| 8–9: door/container `>` → `>=` | deterministic actor-known dedup tie-break | **Provisionally addressed by `-014`; rerun required** | The new equal-key and greater-key witnesses appear mutant-sensitive. Rerun the exact focused command and then the standing perimeter at `0429a8f`. |
| 10: `restore_from_temporal_projection` → `None` | replay restoration of scheduler/runtime authority | **In scope — must close** | Directly guards F-02 and INV-018/INV-112. Add continuation-equivalence evidence through the real runtime, then run a focused scheduler/replay mutation command. |
| 11–14 plus timeout: perception collision counter and rename fields | deterministic perception-event identity/provenance and termination | **Provisionally addressed by `-015`; rerun required** | The bounded-loop/double-collision witness appears capable of killing the timeout and arithmetic/rename mutants. Confirm with the exact corrected command and standing rerun. |
| 15–17: `EpistemicProjection::insert_observation` retain condition | replacement of stale same-source/same-subject actor-known records | **In scope — must close** | This predicate directly feeds holder-known interval construction and salient-stop behavior. Add production-path same-source/same-subject vs different-source/different-subject witnesses and focused mutation kills. |
| 18–46: `actor_known_record_is_novel_to_context` | field-sensitive novelty across actor-known records | **Provisionally addressed by `-016`; rerun required** | The per-variant all-fields-match and single-field-differs table is statically strong. Confirm at the target commit and in the standing campaign. |
| 47–48: `embodied_subject_key` → constant | subject identity used to collapse same-tick actor-known records | **In scope — must close** | A constant key can merge unrelated records or fail same-subject replacement, corrupting interval novelty/salience. Add direct and production-path subject-separation witnesses and focused kills. |

No survivor is classified equivalent by this report. The mutation literature recognizes equivalent-mutant classification as a real problem, but equivalence cannot be inferred from a surviving test result alone; it requires a defensible semantic argument and, where practical, executable confirmation.

### Conformance verdict

**High-severity mutation-evidence gap with in-scope survivors.** The historical number `48 missed + 1 timeout` is no longer a reliable current floor because later tickets changed tests and one production loop. Equally, the later focused runs do not justify calling the standing perimeter green. At minimum, survivor #10, #15–17, and #47–48 guard foundational correctness of this exact surface and belong in the remediation generated from this report.

### Required remediation

**Code/test home:** extend existing scheduler/replay, epistemic projection, interval, salience, and generative tests. Do not alter `.cargo/mutants.toml` merely to restate coverage that already exists. Do not pad `.cargo/mutants-baseline-misses.txt` for killable foundational mutants.

The implementing session should:

1. run a clean baseline;
2. rerun the exact focused commands preserved by 0049 and 0050 `-014`/`-015`/`-016`;
3. add and run focused campaigns for restore continuation, `insert_observation`, and `embodied_subject_key`;
4. run `cargo mutants --timeout 183` with the standing config after all code/test work;
5. publish the selected denominator and complete caught/missed/unviable/timeout disposition.

Any remaining out-of-surface or semantically equivalent mutants must be routed to an owner/reassess decision and a future **cross-cutting** implementation spec. They do not route to a lower tier because mutation evidence spans foundation through reference and executable code.

### Strongest practical anti-regression guard

Prefer production-path behavior at the runtime boundary, then focused mutation kills. The existing deterministic corpus is sufficient; a new property-testing framework is not justified. For identity/dedup predicates, table-driven deterministic cases should vary exactly one field and also pass through interval/salience construction so the public behavior—not merely the private predicate—is protected.

### Evidence-honesty check

A focused 0-miss result is evidence only for its selected functions/mutants and exact source tree. It cannot be substituted for the standing campaign. Conversely, an old standing miss must not be reported as live after a later witness without rerunning it. The final artifact must state both scopes precisely.

## F-09 — Live conformance and risk evidence overstate closure

### Foundational driver

Architecture `13`, execution `10`, and reference R-27/R-28/R-29 require evidence to identify the real path under test, distinguish declarations from behavior, and avoid decorative locks. A historical acceptance artifact may accurately record what was claimed at its evidence commit; live conformance documents must not continue to present contradicted properties as current executable fact.

### Current `0429a8f` documentation state

The architecture conformance index states that 0050 live witnesses establish core-owned loaded actor/process discovery, closed actor outcome consumption, TUI read-only interval consumption, and replacement production-path evidence ([`00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`, loaded-world row](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md)). Execution `05` says loaded actors are discovered by `due_loaded_actor_ids`, declared processes by `due_process_invocations`, and `transact_world_one_tick` consumes the closed outcome without caller-injected actor lists or a parallel no-human choreography ([lines 85–116](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md#L85-L116)). Execution `07` says the TUI stores and renders the complete core interval product read-only and does not call event-appending perception helpers or rebuild holder-known context ([lines 95–105](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md#L95-L105)). The live code contradicts those reachability and authority claims.

The 0050 acceptance artifact’s F-01…F-08 closure table is immutable historical evidence and must not be edited. Its explicit non-green mutation statement is honest. The correct response is a new remediation line and a future acceptance artifact, not retrospective alteration.

`SPEC_LEDGER.md` is now accurate for the historical source/navigation question: it records the 0050 archived row, the 48-missed/1-timeout limit, and the 0049MUTWIT ticket-only source record ([0050 row and 0049 record](https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/docs/4-specs/SPEC_LEDGER.md#L92-L116)). The second-pass F-08 source-discipline defect is therefore genuinely closed and is not carried forward.

### Conformance verdict

**High-severity evidence-honesty gap.** The live conformance and risk evidence describes intended/claimed closure as if it were production-proven. That weakens future reviews because later sessions can cite the row instead of tracing the real ownership and bootstrap path.

### Required remediation

**Documentation home after code closure:**

- update architecture conformance `00` to name the actual core runtime entry point and real production witness;
- update execution `05`, `06`, `07`, and `10` to align status/evidence references with the repaired runtime and separated products without changing governing doctrine;
- update the reference checklist’s live-evidence pointers;
- update only the status/evidence under existing R-27, R-28, and R-29—mint no new risk ID;
- add the future remediation spec to `SPEC_LEDGER.md` through the normal process; preserve all archived rows and artifacts.

### Strongest practical anti-regression guard

Wire a standing conformance test lane in CI that starts from production loading and uses only the public core runtime/TUI command boundary. The architecture row should cite that executable lane and the compile-fail fixtures that protect actual symbols. Narrow `include_str!` guards may verify that forbidden imports or methods do not reappear in the TUI, but must be labeled topology alarms.

### Evidence-honesty check

Every evidence map should answer: “What production constructor created the runtime? Which public command crossed the client boundary? What state/event/projection effect was observed? Which deliberate mutation or negative compile attempt proves sensitivity?” A row that points only to function names, private unit helpers, or source strings is not closure evidence.

## 5. Prior-remediation witness re-verification and vacuity assessment

The table below distinguishes three different claims that the historical evidence sometimes compresses into one:

1. a helper or local predicate has the intended semantics;
2. the coordinator composes correctly when a harness injects the right inputs;
3. the **production loading and client path** actually supplies those inputs and cannot reacquire authority.

A witness may be strong for the first or second claim while remaining vacuous for the third.

| Historical witness or evidence line | Current static reading at `0429a8f` | Vacuity determination | Executable check required from the implementing session |
|---|---|---|---|
| 0050 loaded-world differential in `world_step_coordinator.rs` | It calls the real one-tick coordinator and can distinguish a manually scheduled second actor from a private possessed-actor-only step. The harness itself invokes `schedule_loaded_actor_decision`, however, while the normal TUI/runtime never derives that eligibility. | **Non-vacuous for coordinator composition after manual seeding; vacuous for production loaded-world discovery.** | Start through the production content/runtime constructor, make no scheduler registration calls in the test, submit one possessed input, and prove another loaded actor advances. Deliberately remove the production derivation and show the test fails. |
| 0050 declared-process cadence/provenance witness | It exercises the private cadence calculation and checks a typed trigger/source marker. The resulting envelope is diagnostic and applies no world effect. | **Non-vacuous for cadence and marker ancestry; vacuous for declared-process behavior.** | Use a canonical loaded process whose due invocation changes authoritative state or projection exactly once; prove before/at/after cadence behavior and replay equivalence. A counter or marker-only assertion is insufficient. |
| 0050 actor-transaction integration witness | It enters `ActorDecisionTransaction` and proves that proposal, reduced trace record, and some lifecycle effects cross the coordinator. It does not require full `DecisionTrace`, `LocalPlan`, complete ancestry, or one mutually exclusive actor disposition. | **Partially non-vacuous; incomplete for the doctrinal transaction.** | Make the public world-step result expose or persist the full selected-method/plan ancestry, rejected alternatives/blockers, and lifecycle effects; delete each consumption arm in turn and require the integration test or focused mutant to fail. |
| 0050 TUI read-only interval-product witness | The live TUI still owns and mutates authoritative aggregates, directly invokes the pipeline, appends perception, and performs a separate rebuild/replace path. | **Contradicted by the production path, not merely weak.** | Compile the TUI against a core-owned session whose authoritative fields are private; drive wait, duration continuation, ordinary action, and no-human progression only through typed commands; prove the TUI cannot name any mutating aggregate API. |
| `external_crate_cannot_call_tui_perception_append_helper` | The fixture calls `tracewake_tui::record_current_place_perception_and_project()` with no arguments. No such symbol exists. It does not attempt to call the real public core or scheduler perception writers used by the TUI. | **Vacuous.** It proves only that an invented symbol is absent. | Replace it with fixtures that import the exact current symbols/capabilities that must become private. Pin the expected diagnostic to privacy/constructor unavailability, not generic “cannot find function.” Also add a positive in-crate test proving the single core owner can still perform the operation. |
| Duplicate-`EventId` fail-closed tests | `EventLog` checks duplicates during live append and deserialization, and the tests target those real paths. | **Statically non-vacuous.** | Run the focused tests and mutate/remove both checks independently. Also pass a collision through the production coordinator so the transaction fails without partial aggregate commit. |
| Replay aggregate `matches_expected` temporal conjunct | `run_replay` computes temporal divergence and aggregate success includes that result. The source shape makes removal observable to the named replay tests. | **Statically non-vacuous for aggregate reporting.** | Run the temporal replay tests and a focused mutant deleting/inverting the conjunct. Retain a fixture where physical/projection checks match while the temporal frontier diverges. |
| Empty-tick temporal-frontier witness | The coordinator emits ancestry for an otherwise empty accepted tick, and replay temporal reconstruction consumes it. | **Statically non-vacuous, subject to execution.** | Run an empty-tick append/rebuild/continue equivalence case and mutate away the empty-tick ancestry event or its rebuild arm. |
| Salient-stop quiet/novel/hidden/replay policy | The tests exercise typed actor-known deltas and distinguish quiet, novel, hidden, and replay cases. The post-acceptance novelty witnesses improve field sensitivity. Public product construction and exact-time leakage still weaken the overall truth-firewall claim. | **Non-vacuous for the internal salience predicate; incomplete for the sealed production product.** | Obtain the interval only from the core runtime, perturb hidden truth/debug receipts without changing holder-known sources, and prove stop behavior/output remains unchanged. Rerun the `-014` and `-016` focused mutation selections. |
| `0050FOUCONSEC-014` door/container source tie witnesses | Equal-key and greater-key cases appear to distinguish `>` from `>=` in the exact target functions. | **Credible narrow non-vacuity; historical result only.** | Reproduce the ticket’s exact focused selection against a clean `0429a8f` tree, then rerun the standing perimeter. |
| `0050FOUCONSEC-015` bounded perception collision witness | The double-collision case reaches the bounded retry loop and observes exact deterministic IDs; it appears capable of killing the timeout and arithmetic/rename mutants. | **Credible narrow non-vacuity; historical result only.** | Run the corrected command using a diff that actually contains the target source. Confirm no timeout and no miss, then run the standing perimeter. |
| `0050FOUCONSEC-016` actor-known novelty table | The table varies each discriminating field by record variant and appears capable of killing conjunction/equality/outer-negation changes. | **Credible narrow non-vacuity; historical result only.** | Reproduce the exact focused run and add at least one end-to-end interval/salience assertion so private-predicate correctness is also observed at the public behavior boundary. |
| `generative_lock.rs` corpus | It explores coordinator behavior under deterministic generated seeds and is valuable for ordering, rollback, and invariants once the runtime is already constructed. It does not itself prove that production loading derives actors/processes or that the TUI lacks write authority. | **Non-vacuous for the modeled coordinator corpus; not production-bootstrap proof.** | Add a production-constructor mode to the existing deterministic corpus rather than introducing a second generator framework. Include actor census, process cadence/effect, restore continuation, and hidden-context perturbations. |
| TUI parity/adversarial suites | They compare command families and rendered behavior, but a client that owns both sides of an authority boundary can remain internally consistent while still violating the boundary. | **Useful output parity; insufficient as ownership proof.** | Combine them with external-crate compile-fail fixtures and a core-owned session. A parity test should receive opaque core receipts, not construct or mutate the compared products. |

### 5.1 Preserved 0049MUTWIT focused witnesses

The three 0049 ticket lines are **mostly non-vacuous for the narrow helper semantics they name**. Static reading finds real assertions capable of distinguishing the cited scheduler, interval-source, temporal-rebuild, and duplicate-suppression mutations. They remain historical command claims and do not prove production loading, process effects, TUI de-authority, or full actor-transaction consumption.

The implementing session should reproduce the preserved commands exactly before changing the affected code:

```text
cargo mutants --no-config -f crates/tracewake-core/src/scheduler.rs -F 'latest_current_place_perception_event_id|latest_need_event_id|actor_has_open_body_exclusive_at|append_decision_trace_after_proposal|transact_world_one_tick' --test-package tracewake-core --timeout 183
```

```text
cargo mutants --no-config -f crates/tracewake-core/src/epistemics/projection.rs -f crates/tracewake-core/src/projections.rs -F 'actor_known_interval_delta|VerifiedActorKnownIntervalNotice::source_key' --test-package tracewake-core --timeout 183
```

```text
cargo mutants --no-config -f crates/tracewake-core/src/replay/temporal.rs -f crates/tracewake-core/src/replay/rebuild.rs -f crates/tracewake-core/src/actions/pipeline.rs -F 'validate_time_advanced|rebuild_projection|is_duplicate_need_tick_candidate' --test-package tracewake-core --timeout 183
```

A zero-miss rerun would re-establish those **selected** per-function claims only. It would not supersede the standing campaign or close F-01 through F-07.

## 6. Comprehensive anti-regression layer

The following table covers the load-bearing spec-0047 properties, including properties that appear correct in the current source. “Present” means present by static reading, not command-certified.

| Property | Current static state at `0429a8f` | Existing guard | Required delta | Strongest practical mechanism |
|---|---|---|---|---|
| One shared core world-tick transaction | `transact_world_one_tick` exists and orders controlled proposals, duration work, due actors, processes, time advance, perception/projection, and commit. The TUI still bypasses it for ordinary actions and owns a second no-human choreography. | `world_step_coordinator.rs`, TUI seam/parity tests, source guards | Make one core-owned session command the only externally usable transition boundary; delete direct client pipeline/rebuild paths. | **Compile-time ownership first**, then production integration. |
| Atomic scratch-state cutover | Physical state, agent state, event log, and epistemic projection are cloned/staged and assigned only on accepted completion. | Coordinator rollback tests, generative lock, mutation witnesses | Extend failure injection to each actor/process/perception phase and assert no partial commit, including duplicate-ID failure. | Behavior + focused mutation; no source-text proof. |
| Production loaded-actor discovery | Absent; scheduler registry starts empty and tests seed it. | Manually seeded differential test | Derive all loaded actor opportunities from canonical runtime state/content and current event frontier. | Private core runtime + production-bootstrap differential. |
| Production declared-process discovery | Absent; declarations are caller registrations and normal TUI registers none. | Cadence/marker test | Load process declarations through canonical content/runtime construction and restore them deterministically. | Private declaration registry + compile-fail external construction. |
| Process-specific causal effects | Absent; due process emits diagnostic no-op and increments an “applied” count. | Marker provenance/cadence assertions | Closed typed process dispatch producing ordinary authoritative events and replayable effects. | Type-level process command + state/projection/replay behavior + mutation. |
| Due-work ordering and one opportunity per actor | Partially ordered, but controlled and autonomous paths can both select one actor. | Ordering tests; body-reservation census | Build a closed exhaustive per-tick actor disposition census before execution. | Internal closed enum/table with duplicate-insertion impossible or rejected. |
| Actor next-opportunity cadence | Caller-authored and not consumed/advanced after opportunity. | No complete production witness | Record or derive next opportunity inside the actor transaction and replay projection. | Event-/projection-derived schedule + boundary/off-by-one mutants. |
| Replay/save restoration of runtime authority | Temporal frontier restored; actor/process registries and ordering state are dropped. | Temporal rebuild tests; historical restore survivor exposed gap | Include all authoritative runtime eligibility in deterministic rebuild and continuation checks. | Uninterrupted-vs-restored continuation equivalence. |
| Controlled/autonomous possession parity | Ordinary proposal validation exists, but double opportunity and client-side direct pipeline remain possible. | Parity tests; no-human scenarios | One actor census and one command route for possessed and unpossessed actors. | Compile-time single entry point + held-equal production scenario. |
| Full actor-decision transaction consumption | Proposal, reduced record, and lifecycle effects partly consumed; full trace/local plan/ancestry are dropped. | Transaction unit tests and integration assertions | Return/commit a closed authoritative outcome containing all required ancestry and blockers. | Closed private result type + exhaustive match + field-deletion mutants. |
| Duration continuation and log-derived completion | Appears present: continuation/completion is driven through coordinator state and event/log ancestry rather than a TUI timer. | Coordinator duration tests, replay tests, 0049 scheduler mutations | Re-express under the core-owned session and add restore-mid-duration continuation equivalence. | Production behavior + replay continuation + mutation. |
| Single-charge need accounting | Appears present for wait/duration boundary cases, with dedicated accounting helpers and tests. | Need-accounting tests, sleep/window fixtures, 0049 duplicate-need witness | Add a mixed actor/process tick through production runtime and restore boundary to prove each tick charges once. | Deterministic scenario + focused mutation. |
| Body-exclusive reservation | Appears present; census rejects overlapping body-exclusive work. | `reservation_body_exclusive_census.rs`, scheduler helper witness | Integrate with one-actor disposition and restored runtime so reservations cannot be bypassed by the alternate choreography. | Closed reservation/actor-disposition types + behavior. |
| Accepted empty-tick ancestry | Appears present and replay-visible. | `replay_temporal_frontier.rs`, temporal replay tests | Keep as a required production-runtime case after session refactor. | Behavior + deletion mutant. |
| Replay aggregate fail-closed on temporal divergence | Present: aggregate success includes temporal match. | Replay temporal tests and 0049 focused mutation | Preserve through runtime refactor and add a physical-equal/temporal-different case if not already explicit. | Behavior + focused mutation. |
| Duplicate `EventId` rejection | Present on append and deserialization. | Event-log tests, perception collision witness | Drive a collision through the world-step transaction and prove atomic rollback. | Behavior + compile-time/private ID minting where practical + mutation. |
| Positive holder-known interval construction | Substantially present: typed verified notices are built from holder-known projection deltas, not raw world diffs. | `holder_known_interval_projection.rs`, 0049/0050 projection mutations | Seal public constructors/fields and obtain products only from core session. | Private fields/constructors + external negative fixtures. |
| Field-sensitive interval novelty | Improved by `-014`/`-016`; historical standing status is stale. `insert_observation` and subject-key families remain unwitnessed. | Novelty table, tie tests, salience tests | Close #15–17 and #47–48 at private and production behavior boundaries; rerun focused and standing mutation. | Table-driven behavior + end-to-end salience + mutation. |
| Salient-stop holder-known policy | Internal predicate distinguishes quiet/novel/hidden cases; production product remains forgeable and renderer leaks controller detail. | `salient_stop_actor_known.rs`, novelty witnesses | Feed a sealed embodied summary from core and separate controller/debug receipt. | Information-flow differential + private types + mutation. |
| TUI read-only client boundary | Violated: TUI owns and mutates authoritative aggregates and directly appends perception/runs pipeline/rebuilds. | Source guards, parity tests, vacuous compile-fail fixture | Move ownership into core session; expose typed commands and immutable receipts only; replace fixture with exact-symbol privacy tests. | Compile-time unrepresentability. |
| Embodied/debug product separation | Violated: exact ticks, interval bounds, event IDs, and controller stop reasons render in normal mode; public fields are forgeable. | Snapshots and debug tests, but some lock the leak | Split sealed products; default output only from actor-known sources; explicit capability-gated debug receipt. | Private types + hidden-context differential + transcript negatives. |
| Human wait/no-human shared world progression | Coordinator can express shared work after manual seeding, but normal loading leaves the world private/empty and no-human path rebuilds separately. | No-human fixtures, coordinator differential, TUI command sessions | Start both modes from same production runtime, with identical loaded actors/processes, and compare event/projection semantics. | Held-equal production scenario + mutation. |
| Standing mutation evidence | Historical standing result non-green; later focused tickets narrow several families but no current standing run exists. | `.cargo/mutants.toml`, empty baseline, CI in-diff lane, focused tickets | Kill in-scope restore/observation/subject-key mutants; classify remaining survivors honestly; run full standing campaign. | Focused kills plus fresh standing report. |
| Live evidence/document truth | Several rows overclaim production closure; archived history and `SPEC_LEDGER` source record are otherwise honest. | Architecture/execution/reference review artifacts | Update only after executable closure and cite real production constructor/path. | Evidence map tied to tests and negative fixtures, not declarations. |

### 6.1 Compile-time and ownership guard disposition

The highest-leverage changes are not additional assertions around the current public API. They are removal of the authority-bearing API:

- move physical state, agent state, event log, epistemic projection, scheduler/runtime eligibility, and controller bindings behind one core-owned session/runtime;
- make due-actor, due-process, interval-source, perception-write, replay-restore, and actor-outcome constructors private or `pub(crate)` as appropriate;
- expose typed commands and immutable, sealed receipts;
- use private fields and crate-owned constructors for embodied products;
- make external-crate negative fixtures name the **real forbidden capability** and pin a privacy/constructor diagnostic.

Rust’s privacy rules are suitable for this boundary: private items cannot be used outside their allowed module scope, and restricted visibility can confine helpers to the crate or parent module. `#[non_exhaustive]` may assist API evolution, but it is not by itself an authority seal—public enum variants remain constructible outside the crate—so private constructors/fields and unexported authority tokens remain the controlling mechanism.

### 6.2 Behavioral and deterministic-generative disposition

Extend the existing deterministic corpus rather than adding `proptest` or `quickcheck`. The required state space is structured and can be covered with explicit dimensions:

- zero/one/multiple loaded actors;
- possessed actor due/not due/reserved;
- process absent/before cadence/at cadence/multiple due;
- ordinary action accepted/rejected/partial phase failure;
- duration active/completing/interrupted/restored mid-duration;
- empty tick and event-ID collision;
- actor-known delta quiet/novel/hidden/same-source replacement/different-subject separation;
- uninterrupted vs restored continuation.

Each generated case should enter through the same production constructor and typed command boundary as the TUI. A harness that injects a private due list may remain as a unit test, but it must not be cited as production reachability.

### 6.3 Mutation disposition

The standing perimeter already names the correct broad files. The delta is behavioral sensitivity, not another configuration layer. Focused campaigns should be used during implementation for fast feedback, followed by the configured standing campaign. The artifact must separately report caught, missed, timeout, and unviable counts and must not convert a timeout into an accepted baseline merely to obtain green status.

No historical survivor should be called equivalent without a semantic argument. Equivalent-mutant detection is generally undecidable, so classification necessarily uses justified heuristics and review rather than “the test suite did not kill it” as proof. Out-of-surface survivors route to owner/reassess and a future **cross-cutting** implementation spec because the mutation perimeter spans doctrine, evidence, and executable seams; they do not route to a lower documentation tier.

### 6.4 Source-guard disposition

`include_str!` and syntax scans remain useful alarms for obvious topology regressions such as reintroducing a forbidden TUI import or public mutator. They must never be the sole evidence for:

- atomic rollback;
- replay continuation;
- process effects;
- one opportunity per actor;
- holder-known noninterference;
- production loading;
- mutation sensitivity.

Those properties require type boundaries and executable behavior.

## 7. Recurrence and structural-guard analysis

### 7.1 Why this seam keeps recurring

This is the third consecutive audit pass on the same loaded-world/time-control/TUI-boundary seam to find critical foundational divergence after a remediation line claimed closure. The recurrence is not primarily a failure to add enough individual tests. It is a failure to close the authority class.

**The scheduler is treated as a callable utility rather than the owner of a loaded simulation.** Private internal maps look safe, but public registration methods let any caller author actor/process eligibility. Restore reconstructs only the temporal frontier because the object has no canonical ownership relationship to loaded content or runtime state. Each remediation can patch one route while the caller remains able to rebuild or reseed authority.

**The TUI repeatedly reacquires write authority.** It owns all aggregates, can invoke the pipeline, append perception, replace rebuilt state, and create scheduler state. That design makes “read-only client” an assertion about programmer discipline, not a property enforced by the crate boundary. A later feature naturally reaches for the already available mutable field and reopens the seam.

**Tests prove composition with manufactured inputs and then label it reachability.** Manually populating the due registry is a legitimate coordinator unit test. It is not evidence that production loading discovers those actors/processes. The same pattern appears in the process marker: a typed trigger and an incremented counter prove wiring, while the authoritative world effect is absent.

**Public data products conflate internal receipt, debug evidence, and actor-facing knowledge.** Once one struct carries exact frontier ticks, stop logistics, source IDs, and embodied notices, the renderer has no type-level reason not to print them. Positive holder-known construction inside the projection cannot compensate for a forgeable/public wrapper or a debug-to-embodied path.

**Acceptance was cut before the mutation follow-ups were complete.** The `-013` acceptance honestly recorded a non-green standing campaign; `-014` through `-016` then changed witnesses and one production loop without a fresh standing run. This sequencing does not invalidate the focused work, but it leaves future reviewers with multiple evidence epochs and encourages overbroad summaries.

**Negative evidence targets names rather than capabilities.** A compile-fail test that calls an invented symbol can stay green forever while the real write capability remains public. Similarly, source guards can detect a spelling or import but cannot establish information-flow or transaction behavior.

### 7.2 Higher-leverage structural guard

The cycle-breaking change is a **single core-owned loaded-world runtime/session boundary**, not another wrapper around `DeterministicScheduler`.

The runtime should own, with private fields:

- canonical physical and agent state;
- event log and temporal/replay frontier;
- epistemic projection;
- loaded actor census and next-opportunity projection;
- declared process registry/cadence/trigger state;
- controller bindings and possession mapping;
- duration/reservation state;
- deterministic ID/order minting.

Its public surface should accept a small closed set of typed commands—controlled proposal/input slot, wait/advance, continue duration, pause policy, save/rebuild—and return immutable typed receipts. Internally, one transaction should first derive a closed actor/process disposition census, then execute all selected work on scratch aggregates, produce full actor/process ancestry, construct separate debug and embodied projection products, and commit once.

The TUI should own only presentation/input state and opaque runtime/session handle access. It should be unable to name mutable simulation aggregates or a raw perception/event/process injection API. Replay restore should create the same runtime authority from canonical event/content inputs, not a clock-only scheduler that clients repair.

The structural guard belongs primarily in:

- `crates/tracewake-core`: runtime/session owner, private constructors, closed commands/results, actor/process census, replay-derived eligibility;
- `crates/tracewake-tui`: removal of aggregate ownership and direct pipeline/perception/rebuild paths;
- existing external-crate negative fixtures: exact forbidden operations;
- existing production coordinator/parity/generative tests: one public entry path;
- CI: a standing production-conformance lane plus focused mutation and the existing standing mutation campaign;
- architecture `01`/`02`/`04`/`05`/`10`, execution `05`/`06`/`07`/`10`, and existing reference risks as evidence/status homes after implementation.

A public `#[non_exhaustive]` enum alone is not enough: downstream code can still construct its existing public variants. The authority-bearing commands may be public, but their trusted receipts and internal due-work/notice constructors need private fields, crate-private constructors, or unexported tokens. This is the practical type-level unrepresentability the current seam lacks.

### 7.3 External-research sharpening — separate evidence lane

The following sources inform the guard design; none is used to assert what exists in `tracewake`:

- The [Rust Reference on visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html) confirms that private and restricted-visibility items cannot be used outside their permitted scope. This supports making authority constructors and mutable aggregates unavailable to the TUI/external crates rather than relying on convention.
- The [Rust Reference on `non_exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) shows that non-exhaustive structs cannot be directly constructed outside their defining crate, but existing public enum variants can still be constructed. That distinction supports sealed structs/private constructors for trusted receipts and cautions against treating `non_exhaustive` enums as authority tokens.
- Schneider’s state-machine survey, [*Implementing Fault-Tolerant Services Using the State Machine Approach*](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf), grounds deterministic state-machine behavior in replicas starting from the same state and processing the same requests in the same order. The relevant design lesson here is that due-work eligibility and ordering are part of continuation semantics, not disposable scheduler decoration.
- Temporal’s [Workflow Execution and replay documentation](https://docs.temporal.io/workflow-execution) describes replay as regenerating commands and checking them against event history, resuming from the latest recorded event. The transferable lesson is that recovery must reconstruct the information needed to continue deterministically, not merely recover a scalar clock.
- Sabelfeld and Myers, [*Language-Based Information-Flow Security*](https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf), explain that access control cannot substitute for information-flow control once data has been read. Applied here, merely restricting one constructor is insufficient if a mixed debug/embodied product still carries hidden or controller-only information to the renderer; separate positive construction paths are needed.
- The cargo-mutants guidance on [using results](https://mutants.rs/using-results.html) treats a missed mutant as either a test gap or a semantically indistinguishable change requiring analysis, and its [timeout guidance](https://mutants.rs/timeouts.html) treats hangs as actionable rather than automatically acceptable. This supports the report’s per-family disposition and refusal to baseline a killable timeout.
- Papadakis et al.’s [mutation-testing survey](https://mutationtesting.uni.lu/survey.pdf) describes equivalent-mutant detection as undecidable in general and therefore dependent on heuristics. That supports explicit semantic classification rather than silently labeling survivors equivalent.

## 8. Foundation and documentation determination

### 8.1 Higher-tier amendment verdict

**No amendment to `0-foundation`, `1-architecture`, `2-execution`, or `3-reference` doctrine is warranted.** The live doctrine already states all load-bearing requirements needed to decide this audit:

- one event-sourced causal transition and deterministic replay;
- loaded-world actor/process progression rather than a private possessed-actor tick;
- ordinary proposal/validation parity;
- core ownership of due work and time;
- full cognition-transaction ancestry;
- positive holder-known construction and debug quarantine;
- a read-only client boundary;
- real-path, mutation-sensitive, non-vacuous evidence.

The defects arise because the code and evidence fall below those rules, not because the rules are missing or contradictory. Weakening doctrine to bless caller-seeded registries, diagnostic process markers, client-owned aggregates, or exact hidden time would be a constitutional inversion.

### 8.2 Documentation work after implementation

No archived spec, ticket, acceptance artifact, or passed certification should be edited. Required live-document work is limited to status, navigation, and conformance evidence after the code and executable witnesses exist:

| Home | Required substance | Constraint |
|---|---|---|
| `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | Replace the overbroad 0050 reachability/ownership row with the actual runtime entry point, production constructor, negative fixtures, continuation witness, and current mutation artifact. | Do not claim green before the standing campaign. |
| Architecture `01`, `02`, `04`, `05`, `10`, `13` | Align implementation-status/evidence references with the single core runtime, restored eligibility, process transaction, actor census, and split products. | Governing doctrine need not be rewritten unless the reassess process identifies a genuine contradiction; none was found here. |
| Execution `05`, `06`, `07`, `10` | Replace stale “already closed” descriptions and evidence pointers with the repaired production path and honest command artifacts. | Typed behavior before rendered/source evidence. |
| `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` | Point reviewers to the real production bootstrap, continuation equivalence, compile-fail capability checks, hidden-context differential, and standing mutation artifact. | Preserve authority order. |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | Update evidence/status under existing R-27, R-28, and R-29; use other existing relevant risks as appropriate. | Mint no new risk ID. |
| `docs/4-specs/SPEC_LEDGER.md` | Add the future numbered remediation spec through the normal process and eventually its immutable acceptance record. | Preserve the correct 0049 source record and all archived history. |

The prior second-pass source-discipline finding concerning `0049MUTWIT` is closed: the live ledger contains the source/navigation record. It must not be resurrected as a current defect.

## 9. Recommended closure order

The order matters. Writing more tests around the present public ownership shape before replacing it would harden the wrong API.

1. **Introduce the core-owned loaded-world runtime/session.** Move authoritative aggregates and ID/order authority behind private fields. Expose typed commands and opaque/sealed receipts. Keep a temporary internal adapter only long enough to migrate core tests; do not add a backwards-compatibility public alias.
2. **Move production bootstrap and replay restore into that owner.** Derive/load actor eligibility and process declarations from canonical inputs; reconstruct next-opportunity/cadence/order state; add uninterrupted-vs-restored continuation equivalence before altering higher-level behavior.
3. **Replace diagnostic process markers with real typed process transactions.** Preserve diagnostic ancestry as accompaniment, not effect. Make external raw-envelope/due-invocation injection unrepresentable.
4. **Create the exhaustive per-tick actor/process disposition census.** Resolve possession, controlled input, reservation, due status, and process work exactly once before executing any phase. This prevents tests from being written around a double-opportunity choreography.
5. **Close the actor decision outcome.** Carry the full decision trace, local plan/method ancestry, alternatives/blockers, lifecycle effects, and committed events through one exhaustive result consumed by the coordinator.
6. **Remove TUI write paths.** Delete direct pipeline invocation, perception append, aggregate replacement/rebuild, local scheduler authority, and any client clock/duration ownership. Migrate wait, ordinary action, duration continuation, and no-human controls to the same typed session command.
7. **Split internal/debug and embodied interval products.** Make fields/private constructors enforce origin; remove exact replay/controller metadata from default embodied rendering; add hidden-context noninterference and debug-capability tests.
8. **Replace vacuous and composition-only closure witnesses.** Rewrite the perception compile-fail fixture against actual capabilities; add production-bootstrap actor/process differential, one-opportunity census, process effect, full actor outcome, and TUI de-authority tests. Extend the existing deterministic corpus.
9. **Close in-scope mutation families.** Kill restore #10, observation replacement #15–17, and subject-key #47–48; reproduce 0049 and `-014`/`-015`/`-016`; then run the full standing campaign. Route remaining out-of-surface/equivalent candidates through owner/reassess into a future cross-cutting spec with explicit reasons.
10. **Truth the live documentation and produce acceptance evidence.** Update conformance/risk rows only after clean command artifacts exist. Preserve all archived material unchanged.

## 10. Open maintainer decisions

These are implementation choices inside settled doctrine, not questions that block the determination:

| Decision | Acceptable choice space | Non-negotiable constraint |
|---|---|---|
| Runtime owner name/module | `SimulationSession`, `LoadedWorldRuntime`, or equivalent core type | It, not the TUI/caller, owns all authoritative mutable aggregates and due-work authority. |
| Eligibility representation | Event-sourced declarations/updates, deterministic canonical-state derivation, or dedicated replay projection | Save/replay must reconstruct identical future actor/process selection without caller repair. |
| Process dispatch representation | Closed enum plus registry, trait objects registered only inside core/content bootstrap, or generated closed table | Callers cannot inject completed raw envelopes; a due process produces authoritative effects. |
| Actor next-decision rule | Fixed cadence, planner-derived typed delay, action-completion-derived tick, or explicit event | Exactly one deterministic rule is committed/derived and replay-restored. |
| Actor disposition data shape | Closed enum per actor, indexed census table, or typestate builder | One and only one disposition per loaded actor per tick; controlled and autonomous are mutually exclusive. |
| Public receipt ergonomics | Read-only getters, opaque iterators, serialization DTOs produced by core | External code cannot forge trusted embodied/debug provenance or mutate internal frontier/source data. |
| Actor-facing temporal language | Qualitative interval, modeled clock/calendar reading, remembered source, or explicit “time passed” notice | Exact simulation/replay ticks appear only when an actor-known modeled source supports them; controller stop logistics remain debug/operator data. |
| Save artifact boundary | Event log plus canonical content fingerprint and derived runtime projection, or another replay-complete representation | Continuation equivalence covers actor/process cadence, ordering, durations, reservations, and projections—not only physical state and tick. |
| Mutation survivor ownership | Current remediation for surface-relevant families; future cross-cutting line for others | No silent drop, no unjustified equivalence, no lower-tier routing for this cross-cutting perimeter. |

## 11. Self-check

- [x] The verdict is explicit, and because violations, vacuity gaps, hardening gaps, and in-scope mutation survivors exist, the report-file branch is warranted.
- [x] Every repository file used for target-state claims was fetched from the full exact commit; every required path selected from the brief was present in the uploaded manifest and verified. The complete URL ledger is linked in §3.2.
- [x] The `8d7c119` second-pass and `cb3102e` first-pass findings were treated only as pre-remediation baselines and were re-verified against live `0429a8f` code.
- [x] Every finding identifies controlling invariants/doctrine and names code plus `docs/**` remediation homes.
- [x] The 0050 production-path witnesses and preserved 0049MUTWIT witnesses are assessed for vacuity; executable checks are named wherever static reading cannot certify behavior.
- [x] The historical 48-missed/1-timeout floor is disposed survivor-by-survivor or family-by-family, accounting for post-acceptance tickets `-014`, `-015`, and `-016`.
- [x] The recurrence/structural-guard section identifies why the same authority class reopens and proposes a single core-owned runtime/session plus CI evidence lane to break the cycle.
- [x] Recommendations extend `.cargo/mutants.toml`, `anti_regression_guards.rs`, the negative-fixture pattern, and `generative_lock.rs`; they do not duplicate those mechanisms or add a property-testing dependency without need.
- [x] No archived artifact is proposed for editing; no invariant, gate, risk, or glossary identifier is minted; no ratified doctrine wording is authored.
- [x] The higher-tier amendment determination is explicit and rejects changing doctrine to fit incorrect code.
- [x] Repository evidence and external research are separated; external sources are used only to sharpen design recommendations.
- [x] Static-survey limitations are explicit; no current command is assumed green or red.

## 12. References

### 12.1 User-supplied control material

- `manifest_2026-06-24_0429a8f.txt` — path inventory for the user-supplied target commit; not file-content evidence.
- `0047-foundational-hardening-research-brief-third-pass.md` — locked scope, authority, lineage, and deliverable requirements.
- [`tracewake_0429a8f_exact_commit_acquisition_ledger.txt`](../tracewake_0429a8f_exact_commit_acquisition_ledger.txt) — complete append-only requested URL ledger.

### 12.2 Target-repository evidence

All target-repository links in this report are immutable exact-commit URLs under:

```text
https://github.com/joeloverbeck/tracewake/blob/0429a8f7e7fed2329319b79818f9b891da91cba2/
```

Load-bearing repository sources include the foundation/architecture/execution/reference doctrine identified in the brief; the 0047/0048/0050 specs and acceptance lineage; `0050FOUCONSEC-001…016`; `0049MUTWIT-001…003`; `.cargo/mutants.toml`; scheduler, replay, event-log, projection, actor-transaction, perception, pipeline, view-model, and TUI seams; and the coordinator/replay/interval/salience/reservation/generative/negative-fixture/parity tests cited above. The acquisition ledger is the authoritative complete URL list.

### 12.3 External research — not target-repository evidence

1. Rust Project, [“Visibility and privacy,” *The Rust Reference*](https://doc.rust-lang.org/reference/visibility-and-privacy.html).
2. Rust Project, [“The `non_exhaustive` attribute,” *The Rust Reference*](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
3. Fred B. Schneider, [“Implementing Fault-Tolerant Services Using the State Machine Approach: A Tutorial,” *ACM Computing Surveys* 22(4), 1990](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf).
4. Temporal Technologies, [“Temporal Workflow Execution overview”](https://docs.temporal.io/workflow-execution).
5. Andrei Sabelfeld and Andrew C. Myers, [“Language-Based Information-Flow Security,” *IEEE Journal on Selected Areas in Communications* 21(1), 2003](https://www.cs.cornell.edu/andru/papers/jsac/sm-jsac03.pdf).
6. cargo-mutants, [“Using the results”](https://mutants.rs/using-results.html) and [“Hangs and timeouts”](https://mutants.rs/timeouts.html).
7. Mike Papadakis et al., [“Mutation Testing Advances: An Analysis and Survey,” *Advances in Computers* 112, 2019](https://mutationtesting.uni.lu/survey.pdf).
