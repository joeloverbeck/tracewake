# 0053 Foundational Conformance Fifth Hardening: Sealed Loaded-World Bootstrap Unrepresentability, Core-Owned Immutable Embodied Interval/Receipt Products, Token-Gated Debug/No-Human Command Authority, a Merge-Enforced Standing Conformance Barrier, a Forced `food_source` Survivor Family, and a Fail-Closed Acceptance Result Taxonomy Hardening Spec

> Section set follows the sibling hardening spec
> `archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_*` (the immediate
> predecessor on this seam), not the canonical `specs/` default. This is a
> proposal-time skeleton: it carries no `Outcome`, deviation log, enactment rows,
> or gate-run results — those are added by the acceptance artifact at closeout.

## 0. Baseline statement and source discipline

- **Driver.** `reports/0047-foundational-hardening-research-report-fifth-pass.md`, a
  recommendation-altitude independent post-`0052` static re-audit of the `0047`
  loaded-world / time-control / TUI-authority surface as hardened by `0048`, `0050`,
  `0051`, and `0052`. The report is the originating analysis; it is not itself doctrine
  and minted no spec, invariant, risk identifier, gate code, or glossary term. Its
  predecessors are the fourth-, third-, second-, and first-pass reports (drove `0052`,
  `0051`, `0050`, and `0048` respectively), used by the driver only as pre-remediation
  baselines and explicitly re-derived rather than carried forward.
- **Report target commit.** The report was conducted against
  `e9792dc8fd0a46cae0e40d1c7bc755fe86fdd24f` (`e9792dc`), the repository `HEAD` at
  report authoring time and the merge of the `0052FOUCONFOU` line. Every load-bearing
  code claim cited below was independently re-verified against the live working tree at
  authoring time of this spec:
  - F5-01: `crates/tracewake-core/src/runtime/session.rs` exposes
    `pub fn LoadedWorldBootstrap::from_loaded_state(...)` (taking raw `ActionRegistry`,
    `PhysicalState`, `AgentState`, `EventLog`, `EpistemicProjection`) beside
    `pub fn LoadedWorldRuntime::from_bootstrap(LoadedWorldBootstrap, SimTick)`;
    `crates/tracewake-core/src/state.rs` exposes `pub fn PhysicalState::from_seed_parts`
    and `pub fn AgentState::from_seed_parts`; `crates/tracewake-content/src/load.rs`
    `LoadedFixture::into_runtime_bootstrap` validates and then calls the same public
    `from_loaded_state`. The validated path is therefore a convention beside an equally
    public raw constructor, not a type boundary.
  - F5-02: `crates/tracewake-core/src/runtime/receipt.rs` `RuntimeReceiptKind` publicly
    carries `Continued(AdvanceUntilResult)` and `OneTickAdvanced(WorldAdvanceResult)`;
    `crates/tracewake-tui/src/app.rs` `advance_until` matches
    `RuntimeReceiptKind::Continued(result)` and calls
    `TypedActorKnownIntervalSummary::from_actor_known_delta(...)` client-side;
    `crates/tracewake-core/src/view_models.rs` exposes public
    `TypedActorKnownIntervalSummary::from_actor_known_delta` plus exact getters and the
    public mutators `set_actor_known_interval_summary`, `set_debug_available`, and
    `set_notebook` on the embodied product. The standalone delta constructor
    `ActorKnownIntervalDelta::from_verified` is correctly `pub(crate)`, so the leak is not
    standalone delta construction but the public path: the delta reaches external clients
    through the public `AdvanceUntilResult.actor_known_interval_delta` field carried by the
    public `Continued` receipt, and the delta's own getters and `from_actor_known_delta` are
    public. As with F5-01, the existing negative fixture
    (`tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary`) proves
    only that struct-literal construction is sealed (the summary fields are `pub(crate)`); it
    does **not** prove the receipt/getter/`from_actor_known_delta` path is closed, so that
    negative perimeter is vacuous for this exact attack.
  - F5-03: `crates/tracewake-core/src/runtime/command.rs` keeps `RuntimeCommandKind`
    `pub(crate)` (good), but `pub fn RuntimeCommand::run_no_human_day()` mints
    `DebugCapability::mint()` internally; the caller never presents a runtime-held
    debug/operator token. `DebugCapability::mint()` is itself `pub(crate)`, so an external
    crate cannot forge a capability directly and `RuntimeCommand::debug_view(capability)`
    correctly requires a presented token; the precise hole is therefore the *self-minting
    public constructors* — `RuntimeCommand::run_no_human_day()` and the public `Debug*View::new()`
    family in `view_models.rs` (`DebugControllerBindingView`, `DebugEventLogView`,
    `DebugItemLocationView`, `DebugActionRejectionView`, …) — which call the crate-private
    `mint()` internally rather than requiring a runtime-held token. The remediation seals those
    constructors, not `mint()`. The TUI (`crates/tracewake-tui/src/run.rs`) checks
    `debug_available()` before invoking — protection at the client layer, not the core
    public boundary.
  - F5-04: `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md`
    records a GitHub API result of `Branch not protected (HTTP 404)` for `main`; the
    named required-check job names exist in `.github/workflows/ci.yml` and are asserted by
    `crates/tracewake-core/tests/ci_workflow_guards.rs`, but no branch protection / ruleset
    makes them merge-required.
  - F5-05: `crates/tracewake-core/src/projections.rs` `food_source_fact_supersedes`
    (private fn) is the home of the seven routed-forward survivors recorded by the 0052
    line; the standing perimeter was honestly **not** called green.
  - F5-06: the 0047→0048→0050→0051→0052 acceptance corpus repeatedly rendered
    pass / "pass with disposition" / "scoped pass" wording while later audits found live
    foundational defects in the same seam; `crates/tracewake-core/tests/acceptance_artifact_wording.rs`
    and `ci_workflow_guards.rs` exist as the extension points for a machine-checked result
    taxonomy.
  The report relies on **named symbols, not line numbers**; its `citeturn…`/`#Lxxxx`-style
  anchors are research-tool artifacts and are not relied upon (per the project memory that
  externally researched specs fabricate line numbers).
- **Static-survey limitation inherited from the driver.** The report forbade execution, so
  its pass/fail statements are static readings of source/test intent, not command results.
  This spec therefore states code *structure* facts as verified and defers all green/red
  command claims to the implementing session (§8).
- **Source discipline.** A commit hash named in this spec is audit/provenance only. This
  spec recommends and scopes work; it does not declare latest-`main` certification or any
  phase entry. When executed, the implementation must name its own exact implementation
  commit, not assume `e9792dc` is latest `main`.
- **Ledger timing.** Per the sibling-spec convention, this package receives its
  `docs/4-specs/SPEC_LEDGER.md` entry at acceptance/closeout, not at proposal. This spec
  authors no ledger row now and makes no change to live `0001` or the ledger.
- **Archived-history discipline.** The archived `0047`/`0048`/`0050`/`0051`/`0052` specs,
  their acceptance artifacts, the `0049MUTWIT` / `0050FOUCONSEC` / `0051FOUCONTHI` /
  `0052FOUCONFOU` ticket lines, and any passed certification are immutable, commit-pinned
  historical records. This spec does not edit them and does not treat their claims as
  automatic current-state proof. In particular, the 0052 acceptance's honest non-green
  standing-mutation statement (seven routed-forward `food_source_fact_supersedes` survivors,
  zero timeouts) and its `Branch not protected (HTTP 404)` record are immutable; the correct
  response is this new remediation line plus a future acceptance artifact, never
  retrospective alteration.

## 1. Scope

### 1.1 In scope

The six divergences and warranted gaps the driver found at `e9792dc`, on the
`0047`/`0048`/`0050`/`0051`/`0052` seams and their named conformance collaborators. Finding
IDs are preserved from the driver (`F5-01`…`F5-06`) for lineage cross-reference. Three are
re-openings of authority classes a prior pass claimed closed by instance (F5-01↔F4-01,
F5-02↔F4-06, F5-03↔F4-02/F4-07); two are standing-barrier/governance residuals (F5-04↔F4-08
governance layer, F5-05↔F4-08 §5 survivor family); one is a new process-integrity mechanism
(F5-06).

1. **F5-01 — Production loaded-world bootstrap is not unrepresentable outside the validated
   authority path (critical; vacuity gap).** `LoadedWorldBootstrap::from_loaded_state` is a
   public raw constructor taking authoritative aggregates, and `PhysicalState::from_seed_parts`
   / `AgentState::from_seed_parts` are public seed-part constructors. The validated content
   loader reaches the runtime through this same public constructor, so "validated loaded-world
   bootstrap" is a convention rather than a type boundary; an external crate can fabricate a
   bootstrap from raw parts and pass it to `from_bootstrap`. The existing negative fixture
   proves scheduler-injection is non-public; it does **not** prove bootstrap fabrication is
   impossible, so the negative perimeter is vacuous for this exact attack.
2. **F5-02 — Embodied interval and receipt products remain partially client-assembled and
   exact (high; hardening gap).** `RuntimeReceiptKind::Continued(AdvanceUntilResult)` publicly
   carries the raw advance result; `TuiApp::advance_until` constructs
   `TypedActorKnownIntervalSummary::from_actor_known_delta` client-side; and
   `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` expose public exact getters (start/
   stop tick, start/stop frontier, stop reason, context ID/hash/source) and public mutators
   (`set_actor_known_interval_summary`, `set_debug_available`, `set_notebook`). The normal
   renderer hides exact values, but hiding-on-render is not sealing-at-the-boundary: external
   clients can read exact temporal fields and assemble the product, contradicting the 0052
   claim that the TUI consumes a core-owned read-only interval product.
3. **F5-03 — Debug/no-human runtime command authority is not token-gated at the core public
   API (hardening gap; authority gap).** `RuntimeCommandKind` is crate-private (good), but
   `RuntimeCommand::run_no_human_day()` is public and mints `DebugCapability` internally; debug
   view constructors similarly mint capability internally. The TUI checks `debug_available()`,
   so protection lives at the client, not at the public runtime boundary. The split is
   ambiguous and not fail-closed: if no-human day is debug/operator-only the capability is
   decorative; if it is ordinary play the debug labelling is overstated.
4. **F5-04 — The standing gate is not merge-enforced (governance gap; evidence-honesty gap).**
   `.github/workflows/ci.yml` defines `public-boundary-conformance`,
   `full-surface-mutation-trigger`, and `mutants-lock-layer-reconcile`, and
   `ci_workflow_guards.rs` asserts the workflow topology, but the 0052 acceptance records
   `Branch not protected (HTTP 404)` for `main`. A workflow job definition is not an enforced
   standing gate unless repository settings require it before merge; the recurring seam is
   specifically an enforcement failure, so this is a first-class anti-regression defect.
5. **F5-05 — Seven `food_source_fact_supersedes` survivors remain routed-forward with no
   forcing function (mutation-survivor disposition).** The seven survivors (constant true,
   constant false, deletion of the two `Some`/`None` arms, and `<`→`==`/`>`/`<=` replacements)
   are cross-cutting and were honestly not laundered as equivalent, but a route-forward with no
   forcing function keeps the canonical standing perimeter non-green indefinitely. This is not
   an in-surface code-closure blocker for the loaded-world/TUI seam, but it blocks any
   unqualified "green standing perimeter" claim.
6. **F5-06 — The acceptance/verification process can still launder open defects into
   pass-shaped artifacts (process-integrity gap).** Across the 0047→0052 lineage a remediation
   line could define its own scope, run focused witnesses, truth docs to the implemented shape,
   record residuals as "scoped pass," and archive an acceptance artifact whose prose reads
   closed while the protected authority class survived. The doctrine already carries strong
   evidence-honesty rules; the missing piece is an **enforced, machine-checkable result
   taxonomy** and an operational required-check policy that make `pending`, `routed-forward`,
   `historical`, `sampled`, and `observer-only` evidence impossible to silently count as pass.

### 1.2 Out of scope

- Any constitutional-invariant amendment, gate-semantics change, new gate rung, new risk-ID, or
  new glossary term. The driver's tier determination is **no Tier-0 foundation amendment**;
  the warranted change is doctrine *strengthening below foundation* at architecture `13` and
  execution `10` (§6.1), at ordinary-owner approval altitude, plus reference `00`/`01`
  status/navigation.
- Any edit to archived specs, tickets, acceptance artifacts, reports, or passed certifications,
  or to existing `SPEC_LEDGER.md` archived rows.
- Re-litigating the properties the fourth/fifth passes found structurally **present and to be
  preserved, not rebuilt** (driver §4): the core-owned `LoadedWorldRuntime` with private
  aggregates and runtime-owned scheduler derivation; the validated content path through
  `LoadedFixture::into_runtime_bootstrap` → `from_bootstrap`; the crate-private
  `RuntimeCommandKind`; normal `continue` rendering that hides exact stop ticks; the
  qualitative `render_embodied_view`; and the named CI job topology (§4.7).
- Reclassifying the `food_source_fact_supersedes` family into a lower tier or asserting
  equivalence; it routes forward under a **bounded forcing function** (§5), not into a disposal
  bin, with no equivalence claim.
- Any new property-testing dependency (`proptest`/`quickcheck`); the existing deterministic
  corpus, generated lock corpus, integration seams, negative fixtures, and mutation campaigns
  are sufficient (driver §6, §10).
- Phase-4 entry, second-proof entry, latest-`main` certification, or feature expansion. No new
  gate code is minted; the barrier strengthens what counts as certifying evidence under the
  existing `DIAG-CERT` / acceptance machinery (driver §6).

## 2. Doctrine anchors

Authority order applied without inversion: `0-foundation` → `1-architecture` → `2-execution`
→ `3-reference`. Controlling invariants by finding (re-verified against the live constitution
`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`):

- **Causality / event sourcing / replay authority.** INV-018 (deterministic replay is
  foundational), INV-022 (raw prose is not authoritative state), INV-092 (deterministic replay
  is tested) — drive F5-01.
- **Truth firewall / belief-before-truth / records separation.** INV-002, INV-023, INV-030 —
  drive F5-01/F5-02.
- **TUI/embodied/debug boundary and non-diegesis.** INV-008 (UI assistance is not authority),
  INV-031 (human/debug notes are non-diegetic), INV-067 (embodied mode shows actor-known
  reality), INV-068 (debug mode is visibly non-diegetic) — drive F5-02/F5-03.
- **No-human authority and possession parity.** INV-004, INV-005, INV-006, INV-094 — drive
  F5-03 (no-human-day classification and possession-parity testing).
- **Harsh, path-under-test acceptance and validation.** INV-098-class evidence discipline plus
  the architecture/execution evidence-honesty contract — drive F5-04/F5-05/F5-06.

Architecture homes: `04` (action proposal / validation / scheduling / feedback — bootstrap
unforgeability evidence row), `10` (possession / TUI view models / debug / client boundaries —
sealed interval product and debug command split), `13` (validation / observability / acceptance
/ review artifacts — acceptance result-exclusivity, the **doctrine-strengthening home for
F5-06**). Execution homes: `05` (transaction scheduler / no direct dispatch — bootstrap
unforgeability + debug-command tokenization), `06` (no-human / ordinary-life proof — no-human-day
classification), `07` (epistemic view models / possession / debug proof — sealed interval product
proof), `10` (testing / observability / diagnostics / review artifacts — the **fail-closed
acceptance manifest, result taxonomy, wording guard, branch-protection evidence, and
routed-forward forcing-function procedure**, the doctrine-strengthening home for F5-04/F5-05/
F5-06). Reference homes: `00` (review checklist — reviewer pointer to fail-closed acceptance and
branch-protection proof), `01` (R-27/R-28/R-29 status/evidence only — mint no risk ID).

## 3. Determination

**Not conformant as a closed surface; remediation warranted; no Tier-0 foundation amendment
warranted; doctrine strengthening warranted below foundation at architecture `13` and execution
`10`.**

The 0052 line delivered real and necessary gains — a core-owned `LoadedWorldRuntime` with
private aggregates and runtime-owned scheduler derivation, a validated content bootstrap path,
a crate-private `RuntimeCommandKind`, sealed-enough normal `continue` rendering, a qualitative
embodied renderer, and named CI conformance/mutation jobs. Those are recorded as present (§4.7)
and not re-reported as defects. They do **not** establish foundational conformance: 0052 again
**closed important instances without closing the authority class**.

The root cause is the same structural pattern the fourth pass diagnosed and the fifth pass
confirms (driver §6): a correct constructor *beside* an injectable/raw one; a closed command
token *beside* a public constructor that mints its own debug capability; private fields treated
as client de-authority while public getters/setters and raw receipt products still expose exact
temporal truth; workflow jobs that exist but are not merge-required; and focused mutation evidence
over a non-green standing perimeter with no forcing function. This is the **fifth** consecutive
pass to find divergence on this seam after a remediation claimed closure. The cycle-breaking
change is therefore not another per-finding test list but a **three-layer enforced standing
barrier** bound to a **fail-closed acceptance result taxonomy**: (1) compile-time
unrepresentability on the real public symbols (bootstrap, interval product, debug receipt/command);
(2) behavioral witnesses through the production boundary on a required public-boundary CI lane;
(3) governance enforcement via branch protection / rulesets that make the lane merge-required —
and an acceptance manifest under which `pass` is computed only from certifying current evidence.

The foundation already forbids every bad state (driver §8): blessing public raw bootstrap
constructors, debug-command self-minting, exact embodied interval access, unprotected standing
gates, or self-scoped pass artifacts would be a constitutional inversion. The defects are code
and process falling *below* the rules. The one warranted doctrinal change is to operationalize
the existing evidence-honesty rules into an executable result taxonomy at architecture `13` /
execution `10` — substance and home only, at ordinary-owner altitude, not a constitutional edit.

## 4. Findings and remediation requirements

Each finding names a **code/governance home**, a **remediation requirement**, and the
**strongest practical anti-regression guard** (compile-time/type ownership first, then
production-path behavior, then focused mutation, then — labeled — source/topology alarms).
Source-text / `include_str!` / workflow-YAML guards are permitted only as labeled topology
alarms and never as sole proof of unrepresentability, replay continuation, information-flow
noninterference, production reachability, mutation sensitivity, or merge enforcement (driver §6).
Every source guard for this surface must point to the compile-time or executable witness it
protects and include a synthetic negative proving the guard itself can fire. Subsections are
keyed by finding ID, not implementation order; §9 governs sequencing.

### 4.1 F5-01 — Sealed loaded-world bootstrap unrepresentability

Make "validated loaded-world bootstrap" **unrepresentable outside the authority path**. Because
`tracewake-content` is a separate crate, `pub(crate)` on `from_loaded_state` alone is
insufficient; choose one authority topology (§10.1): (a) move bootstrap construction + validation
into the same authority crate as the runtime, so content hands core authored/normalized validated
material and core mints the bootstrap internally; (b) introduce an unforgeable
`ValidatedLoadedWorldBootstrap` whose constructor is private to the runtime authority path, with
`from_bootstrap` accepting only that sealed product, not raw state/log/projection parts; or (c)
if topology forbids both, invert dependencies through a dedicated internal authority crate.
**Cargo features are not security** — external crates can enable public features. Delete the raw
`from_loaded_state` and the public `from_seed_parts` constructors (or make them crate-private to
the authority path); leave **no** backwards-compatible alias. Code home:
`crates/tracewake-core/src/runtime/session.rs`, `crates/tracewake-core/src/state.rs`,
`crates/tracewake-content/src/load.rs`, `tests/negative-fixtures/*`.

**Anti-regression guard.** Add an external negative fixture (compiled by
`negative_fixture_runner.rs`), e.g.
`external_crate_cannot_construct_loaded_world_bootstrap_from_seed_parts`, that tries to import
`LoadedWorldBootstrap`, call `from_loaded_state` or any successor raw constructor, build
`PhysicalState`/`AgentState` from seed parts, and pass the result to `from_bootstrap` — and must
**fail to compile** for privacy/constructor reasons (a runtime rejection is weaker than
unrepresentability), under default and all supported feature combinations, pinned to a
privacy/constructor diagnostic rather than a generic "cannot find function." Add a positive
public-boundary witness that the real content loader still produces a runtime through the sealed
path with no caller-injected scheduler actor/process registration.

### 4.2 F5-02 — Core-owned immutable embodied interval/receipt products

Seal the product boundary so external clients receive opaque, immutable, actor-known interval
data and cannot read exact temporal internals or assemble the product themselves. Required:
(1) replace public `RuntimeReceiptKind::Continued(AdvanceUntilResult)` exposure with a sealed
public receipt surface — normal callers receive an embodied receipt carrying only actor-known,
source-bearing qualitative interval data; exact tick/frontier/stop/replay detail reaches only
debug/operator callers through a separately token-gated debug receipt (ties to §4.3);
(2) move `TypedActorKnownIntervalSummary` construction fully into core — the TUI receives and
stores an already-sealed product (or an opaque renderable DTO) and never calls
`from_actor_known_delta`; (3) remove or restrict public exact getters on the embodied product —
exact ticks/frontiers/stop reasons/context hashes are debug/operator-only unless architecture
explicitly models a specific value as actor-known via a source; (4) remove public mutators
(`set_actor_known_interval_summary`, `set_debug_available`, `set_notebook`) — core uses
crate-private builders and public clients receive immutable products. Code home:
`crates/tracewake-core/src/runtime/receipt.rs`, `crates/tracewake-core/src/view_models.rs`,
`crates/tracewake-core/src/projections.rs`, `crates/tracewake-tui/src/app.rs`,
`crates/tracewake-tui/src/render.rs`.

**Anti-regression guard.** External negative fixtures proving an external crate cannot call
`from_actor_known_delta`, cannot call exact tick/frontier/stop getters, cannot call the embodied
setters, and cannot pattern-match a public `RuntimeReceiptKind` to extract raw `AdvanceUntilResult`
exact fields. Positive tests through `TuiApp::advance_until` and command-loop `continue` proving
the normal path receives an actor-known update and cannot render exact internals; keep render
tests but do not let them alone certify sealing. Mutation sensitivity on public product accessors:
every surviving "return default tick/frontier/stop reason" mutant must be killed through a public
behavior witness or removed from the public embodied API.

### 4.3 F5-03 — Token-gated debug/no-human command authority (with classification)

First **classify** `run_no_human_day` (§10.2): ordinary play acceleration vs. debug/operator
execution. If **ordinary**: remove the debug marker from the command vocabulary and receipt path,
prove it crosses the same core world-step boundary as ordinary play, and ensure it exposes no
debug metrics to embodied output. If **debug/operator**: require an unforgeable
`DebugSessionAuthority` (or equivalent) token supplied by the runtime/controller binding state;
the public constructor must **not** mint the token internally, and debug receipt/view constructors
must require the same token or be crate-private builders fed by runtime-owned debug APIs. Code
home: `crates/tracewake-core/src/runtime/command.rs`,
`crates/tracewake-core/src/runtime/receipt.rs`, `crates/tracewake-core/src/debug_capability.rs`,
`crates/tracewake-tui/src/app.rs`, `crates/tracewake-tui/src/run.rs`,
`crates/tracewake-tui/src/input.rs`.

**Anti-regression guard.** An external negative fixture importing `tracewake_core` directly and
failing there, proving an external crate cannot construct or submit a debug/operator command
without a runtime-minted token (a fixture that only asserts the TUI checks debug mode is vacuous —
the protected claim is the public runtime boundary). A behavioral TUI test showing the command
loop rejects `debug run no-human-day` when not bound in debug mode and succeeds only via the token
path.

### 4.4 F5-04 — Merge-enforced standing barrier (governance)

Make the named CI jobs **required status checks** for merge to `main`. Governance home: GitHub
branch protection or repository ruleset for `main`. Required settings: protect `main` (or an
equivalent ruleset); require PRs for changes to protected code/docs paths; require the workspace
gates and the `public-boundary conformance` check; require the lock-layer mutation
trigger/reconciliation status appropriate to changed paths; disallow bypass for normal
maintainers (any admin bypass produces an explicit governance residual and cannot be called pass);
require the branch to be up to date with base or use a merge queue to rerun required checks on the
merge result. Code/doc home: `.github/workflows/ci.yml`,
`crates/tracewake-core/tests/ci_workflow_guards.rs`, and the §6 doctrine homes.

**Anti-regression guard.** A CI/governance audit job that queries the branch-protection/ruleset
API and fails if the required status checks are not configured on the exact repository/branch; if
API permissions are unavailable (e.g. forked PRs) it must report `pending/unverified`, never pass.
Keep `ci_workflow_guards.rs` as a labeled topology alarm for job names and path filters — not as
merge-enforcement proof. Acceptance evidence is an **API transcript** plus a PR merge-box / ruleset
proof that the named checks are required; a screenshot is weaker than an API transcript, and a YAML
grep is not enough.

### 4.5 F5-05 — Forced `food_source_fact_supersedes` survivor family

Resolve the seven survivors or bind them under a forcing function (§5). Code home:
`crates/tracewake-core/src/projections.rs` plus public behavior tests that exercise food-source
belief replacement via actor-known/projection paths (not private-function-only tests).
Mutation/config home: `.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt` (or a successor
survivor registry). Note `.cargo/mutants-baseline-misses.txt` is currently empty — the seven
survivors are recorded narratively in the 0052 acceptance, not in that file — so the forcing-function
encoding (§5) must **populate** a registry, not assume one already lists them. Decide the intended ordering/replacement rule for source-bearing vs.
source-less food-source facts and source-key tie-breaking (§10.4), then encode it in public tests
that kill constant true/false and ordering mutants. Any genuinely equivalent mutant gets a narrow
`#[mutants::skip]` / registry entry **with a semantic proof** — "the suite did not kill it" is not
equivalence evidence.

**Anti-regression guard.** Public projection/actor-known tests that introduce two food-source
facts with controlled source presence and source keys through modeled observation/record paths,
then assert the resulting actor-known belief and rendered/decision consequences, killing the
constant and ordering mutants.

### 4.6 F5-06 — Fail-closed acceptance result taxonomy (process-integrity keystone)

Install a machine-checked acceptance status mechanism that extends existing machinery (no new test
framework, gate code, invariant, risk ID, or glossary term). Required:
1. **Machine-readable acceptance status manifest** accompanying every future remediation
   acceptance artifact for this seam, checked by existing Rust tests, carrying: exact commit under
   test; source acquisition method; findings/residuals by existing finding labels (not new
   canonical IDs); per-finding status from a closed set
   (`closed` / `open` / `routed-forward` / `pending-governance` / `historical-only` /
   `not-in-scope`); a certifying evidence item per `closed`; a live negative/public-boundary proof
   per protected shortcut; mutation evidence status + survivor list; branch-protection/ruleset
   enforcement status; and an overall result **computed** from statuses.
2. **The computed rule:** overall `pass` is legal only when every required finding is `closed`,
   every required governance control is enforced, and every standing mutation residual is killed
   or explicitly non-blocking with a bounded forcing function. `pending-governance`, `open`, and
   unbounded `routed-forward` make the overall result non-pass.
3. **Acceptance wording guard:** extend `crates/tracewake-core/tests/acceptance_artifact_wording.rs`
   (or a sibling under the same pattern) so CI rejects acceptance artifacts that use "pass with,"
   "scoped pass," "accepted," or equivalent closure language while the status block has
   open/pending/routed-forward items; call the canonical perimeter green while the survivor list is
   non-empty; cite branch-protection enforcement without an API transcript/ruleset evidence; cite
   historical command results as current certification; or cite display strings / artifact
   existence / checksums / source guards as sole evidence for a behavior claim architecture
   requires typed path-under-test evidence for.
4. **Routed-forward forcing function:** any routed-forward residual must name owning surface, why
   it is not closed by the current line, the next known execution move in `SPEC_LEDGER.md`, a
   maximum number of remediation epochs or a concrete trigger after which it becomes blocking, and
   the exact CI/mutation test that fails if it remains.
5. **Independent/adversarial posture:** a foundational certification claim must be re-checked by a
   separate adversarial pass or by CI-enforced evidence independent of the implementation author's
   wording. An implementation session may say "implementation evidence collected" / "scoped
   remediation evidence," but not "foundational pass" unless the manifest computes pass.

Code/doc home: `crates/tracewake-core/tests/acceptance_artifact_wording.rs`,
`crates/tracewake-core/tests/ci_workflow_guards.rs`, `.github/workflows/ci.yml`, and the §6
doctrine homes. The mechanism belongs under existing acceptance/diagnostic evidence doctrine and
the central `DIAG-CERT` machinery; do **not** mint a new gate code.

### 4.7 Preserved properties (regression-guard, not rebuild)

Recorded present in the live tree and to be preserved while fixing the above: the core-owned
`LoadedWorldRuntime` with private aggregates and `from_bootstrap` scheduler derivation; the
validated content path `LoadedFixture::into_runtime_bootstrap` → `from_bootstrap`; the
crate-private `RuntimeCommandKind` consumed by `submit_command`; normal `continue` rendering with
qualitative "actor-known interval updated" wording that avoids exact stop ticks; the qualitative
`render_embodied_view` and non-diegetic debug-panel markers; and the named CI jobs
(`public-boundary-conformance`, `full-surface-mutation-trigger`, `mutants-lock-layer-reconcile`)
asserted by `ci_workflow_guards.rs`. These are not re-commissioned as absent.

## 5. Standing-mutation survivor disposition

The seven `food_source_fact_supersedes` survivors (constant true; constant false; deletion of the
`Some` arm; deletion of the `None` arm; `<`→`==`; `<`→`>`; `<`→`<=`) route forward as a
cross-cutting mutation residual **under a bounded forcing function**, with no equivalence claim and
no lower-tier routing:

- **Owning surface:** `crates/tracewake-core/src/projections.rs` `food_source_fact_supersedes`
  and the actor-known/projection paths that consume it.
- **Why not closed by this line:** the family is cross-cutting (food-source belief semantics), not
  the loaded-world/TUI authority seam this pass primarily seals; closing it requires deciding the
  intended replacement/tie-break semantics (§10.4).
- **Forcing function:** the next remediation line that touches projection/food-source/freshness
  **must** kill the seven mutants or record a defensible per-mutant semantic-equivalence rationale;
  the CI mutation reconciliation must fail if this family survives after the forcing-function
  deadline without an explicit maintainer decision; and **no artifact may call the canonical
  standing perimeter green while this family survives**. The next known execution move is recorded
  in `SPEC_LEDGER.md` at closeout (§6.2).
- **Maximum epochs / trigger:** blocking at the next projection/food-source-touching remediation
  line, or after this `0053` line's acceptance, whichever comes first — whichever the maintainer
  records in the manifest forcing-function field (§4.6.4).

The fifth-pass line **may** elect to close this family within `0053` (preferred) by encoding the
semantics decision (§10.4); if it does, the manifest records the survivors as `closed`, not
routed-forward.

## 6. Doctrine strengthening and live-doc truthing

Unlike the 0052 line (which warranted no amendment at any tier), this line **does** warrant
doctrine strengthening below foundation — substance and home only, at ordinary-owner approval
altitude, attaching to the eventual tier edit, not to this spec's write. It mints no invariant,
gate code, risk ID, or glossary term, and authors no ratified wording in this spec.

### 6.1 Below-foundation doctrine strengthening (F5-04/F5-05/F5-06 home; part of this line)

The existing architecture/execution evidence-honesty docs already reject display-string-only,
fixture-only, and historical proof; the missing piece is operationalizing those rules so an
acceptance artifact cannot overclaim. Add, as part of this remediation line (these are *new
doctrine substance*, not post-closure conformance-row edits):

- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` —
  **acceptance result exclusivity:** an artifact may not render `pass` while any flagged
  foundational violation, required governance control, or standing mutation residual remains
  open/pending/unbounded; acceptance must include typed path-under-test evidence and live
  negatives for every protected authority claim; production-constructor evidence must include a
  negative fixture for raw bootstrap fabrication; interval products must be core-constructed and
  immutable from the public client boundary.
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — the
  **fail-closed acceptance manifest, result taxonomy, wording guard, branch-protection/ruleset
  evidence requirement, and routed-forward forcing-function procedure** (§4.6): `pass` is computed
  only from certifying current evidence; `pending`, `routed-forward`, `historical`, `sampled`,
  `observer-only`, and "pass with disposition" are not pass.
- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — reviewer pointer to fail-closed
  acceptance evidence and branch-protection enforcement proof (navigation only).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — update existing **R-27/R-28/R-29** status/evidence
  rows only; mint no new risk ID, retire no general risk.

The doctrine substance is enacted by the existing owner-approval process and lands together with
the F5-04/F5-06 code (manifest + wording guard + governance audit) so the rule is executable, not
merely written.

### 6.2 Post-implementation live-doc truthing (after executable closure only)

Do not edit conformance rows first. After F5-01…F5-06 are implemented and executed at one exact
commit: `docs/1-architecture/04` updates the current evidence row to name the sealed bootstrap
constructor and its negative proof (not a public raw constructor);
`docs/1-architecture/10` updates the embodied/debug split evidence to name the sealed interval
product and distinguish public embodied vs. debug/operator exact fields and the resolved
no-human-day classification; `docs/2-execution/05` adds bootstrap unforgeability and debug/operator
command tokenization to the no-direct-dispatch evidence; `docs/2-execution/06` clarifies the
no-human-day classification per the §10.2 decision; `docs/2-execution/07` adds the sealed interval
product and exact-debug-only evidence with external-client compile-fail checks; reference `00`/`01`
update navigation/status; `SPEC_LEDGER.md` routes this remediation spec and closeout through the
normal ledger/archive process and records the `food_source` forcing function in "Next known
execution move." Every updated evidence row must answer: which sealed constructor created the
runtime, which public command/token crossed the client boundary, what state/event/projection effect
was observed, and which deliberate mutation or negative compile attempt proves sensitivity. No
archived spec/ticket/report/acceptance/certification is edited.

## 7. Required fixtures, tests, and CI

Extend existing machinery (driver §6); do **not** duplicate it or add a property-testing
dependency:

- **External-crate negative fixtures** (`tests/negative-fixtures/*`, `negative_fixture_runner.rs`):
  add fixtures naming the **real** forbidden capabilities — raw bootstrap construction
  (`from_loaded_state` / `from_seed_parts` / successor), embodied interval product construction
  (`from_actor_known_delta`) and mutation (the embodied setters), exact temporal-field getters,
  raw `RuntimeReceiptKind` exact-field extraction, and debug/operator command/receipt construction
  without a runtime-minted token — under **default and all supported feature combinations**, each
  pinned to a privacy/constructor diagnostic; plus positive in-crate tests proving the single core
  owner can still perform each operation. The interval-product fixture **extends** the existing
  `tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary` (which today
  proves only that struct-literal construction is sealed) to cover the live attack — the public
  `Continued` receipt / `AdvanceUntilResult.actor_known_interval_delta` field, the delta getters,
  and `from_actor_known_delta` — rather than adding a duplicate fixture (driver §6: extend, do not
  duplicate).
- **Production-bootstrap conformance lane** (the required `public-boundary conformance` CI job):
  deterministic tests through `TuiApp::from_golden`, public runtime commands/tokens, the world-step
  coordinator, replay frontier, the TUI command loop, parity/adversarial surfaces, and the negative
  fixtures.
- **Sealed-interval / embodied-debug separation** (`view_models.rs`/`receipt.rs` + TUI tests):
  default-transcript exact-leak-absence assertions, debug-capability/token presence assertions, and
  a hidden-context/debug-receipt information-flow differential.
- **Food-source projection behavior** (`projections.rs` + actor-known tests): public observation/
  record-driven replacement tests that kill the §5 survivors (if closed this line).
- **Governance audit job**: the branch-protection/ruleset API check (§4.4).
- **Acceptance manifest + wording guard**: the machine-readable status block and the extended
  `acceptance_artifact_wording.rs` / `ci_workflow_guards.rs` checks (§4.6).
- **Mutation**: focused campaigns during implementation for fast feedback, then the configured
  standing campaign after all code/test work; keep in-diff fast feedback plus the required
  full-surface trigger breadth; the canonical perimeter may not be called green until the §5 family
  is resolved or reported under its forcing function.
- `include_str!` / workflow-YAML import/method guards remain labeled topology alarms only.

## 8. Acceptance artifact and evidence

On implementation the session must: begin from a clean baseline; name its own exact implementation
commit (not this proposal's baseline `e9792dc`); run `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo build --workspace --all-targets --locked`, and `cargo test --workspace`; run focused
mutation campaigns for each in-surface change and then the full standing campaign **after** all
code/test work; and publish the selected denominator with the complete caught/missed/unviable/
timeout disposition. The acceptance artifact follows
`docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`, **carries the §4.6 machine-readable status
manifest**, records per-finding closure with real production-path evidence (sealed constructor,
public command/token, observed effect, sensitivity/negative-compile proof), records the
required-check names **and a branch-protection/ruleset API transcript**, and must not render `pass`
unless the manifest computes pass — i.e. unless every required finding is `closed`, governance is
enforced, and the §5 family is killed or under a bounded forcing function. Doc-truthing (§6.2)
lands only after the executable witnesses exist; the §6.1 doctrine substance lands with the F5-06
code. Every executable claim is the implementing session's to produce — this spec asserts no
green/red command result.

## 9. Implementation constraints

- Follow the driver's recommended **closure order** (report §10): (1) install fail-closed process
  enforcement first — acceptance status manifest, wording guard, routed-forward forcing function,
  branch-protection/ruleset API audit (F5-06/F5-04 scaffolding); (2) enable branch protection /
  ruleset on `main` and record API proof (F5-04); (3) seal production bootstrap authority and add
  the negative fixture (F5-01); (4) seal embodied interval/receipt products and split exact debug
  data behind token-gated receipts (F5-02); (5) token-gate debug/no-human commands or reclassify
  no-human day (F5-03); (6) resolve or force the `food_source` survivor family (F5-05); (7) run the
  authoritative gates from a clean baseline; (8) only then truth live conformance docs (§6.2).
  Installing process enforcement first is deliberate: without it the next code remediation can
  repeat the same acceptance failure.
- No backwards-compatibility shim or public alias path in new work; a temporary internal adapter
  may exist only to migrate core tests and must be removed before closeout.
- Core must not depend on content or tui; the sealed bootstrap authority and interval product live
  in `tracewake-core` (or a dedicated internal authority crate per §10.1, preserving the one-way
  dependency direction).
- Worktree discipline: if implemented in a worktree, all paths resolve against the worktree root.
- Decompose into one ticket per reviewable diff (the closure-order steps are the natural ticket
  boundaries); the reassess/decomposition step determines the ticket prefix continuing the sibling
  convention (`0048FOUCONHAR` → `0050FOUCONSEC` → `0051FOUCONTHI` → `0052FOUCONFOU` → a
  `0053`-keyed successor such as `0053FOUCONFIF`).

## 10. Risks and open questions

These are implementation choices inside settled doctrine; none blocks the determination (driver
§11 open maintainer decisions):

1. **Bootstrap authority topology** — bootstrap minting moves into core, into a dedicated internal
   authority crate, or another topology that makes raw runtime bootstraps unrepresentable outside
   the trusted path. Non-negotiable: no external crate can fabricate a validated bootstrap; Cargo
   features are not security.
2. **No-human-day classification** — ordinary play acceleration vs. debug/operator execution. The
   code and docs must stop straddling both; the chosen path is stated in the acceptance artifact
   and §6.2 doc-truthing.
3. **Exact interval public API** — which temporal fields, if any, are legitimately actor-known
   through modeled sources. Everything else is debug/operator-only.
4. **Food-source semantics** — the intended ordering/replacement rule for source-bearing vs.
   source-less facts and source-key tie-breaking, encoded in public behavior tests or per-mutant
   semantic dispositions (§5).
5. **Governance owner** — the maintainer/admin responsible for branch-protection/ruleset
   configuration and API-proof capture; the recording location is operational evidence, not a new
   gate identifier.
6. **Manifest format/home** — where the machine-readable status manifest lives (inline in the
   acceptance artifact vs. a sibling file) and which existing Rust test consumes it; non-negotiable:
   the overall result is computed from statuses, not asserted in prose.

Open recurrence risk: this is the **fifth** consecutive pass to find divergence on this seam after
a remediation line claimed closure. 0052 proved that a validated production path and a closed
command *token* are not the same as authority closure while a raw constructor and a self-minting
debug capability sit beside them. The §3 three-layer barrier bound to the §4.6 fail-closed result
taxonomy is the structural bet that breaks the cycle; if the implementation leaves any public raw
bootstrap path, forgeable/exact embodied product, self-minted debug command, advisory (non-merge-
required) standing gate, or unbounded routed-forward residual, the seam will reopen for a sixth
pass.

## 11. Invariants alignment

| Invariant(s) | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-018, INV-022, INV-092, INV-002, INV-023 | aligns | §4.1 makes the validated loaded-world bootstrap unrepresentable outside the authority path (sealed core constructor + compile-fail negative fixture), so authoritative state cannot be fabricated from raw parts beside the validated path @ runtime/content boundary. |
| INV-008, INV-030, INV-031, INV-067, INV-068 | aligns | §4.2 seals the embodied interval/receipt product as a core-owned immutable actor-known product and confines exact tick/frontier/stop/replay detail to token-gated debug receipts, so the TUI renders but cannot assemble or read hidden temporal truth @ view-model/TUI boundary. |
| INV-004, INV-005, INV-006, INV-031, INV-068, INV-094 | aligns | §4.3 token-gates debug/no-human command authority at the core public API (or reclassifies no-human day as ordinary play), so debug capability is caller-held and non-diegetic rather than self-minted, preserving possession parity @ runtime command boundary. |
| INV-098-class acceptance discipline | aligns | §4.4/§4.6 make the public-boundary lane merge-required and compute acceptance `pass` only from certifying current evidence (manifest + wording guard + branch-protection API proof), ending pass-shaped artifacts over open defects @ CI/governance + acceptance surface. |
| INV-092 | aligns | §4.5/§5 bind the `food_source` survivor family under a bounded forcing function and forbid a green canonical-perimeter claim while it survives @ projection + mutation surface. |
| (all above) | N/A — no Tier-0 amendment | §3/§6: foundation already forbids every bad state; the only doctrinal change is operationalizing existing evidence-honesty rules at architecture `13` / execution `10` below foundation; no invariant is weakened, minted, or redefined. |
