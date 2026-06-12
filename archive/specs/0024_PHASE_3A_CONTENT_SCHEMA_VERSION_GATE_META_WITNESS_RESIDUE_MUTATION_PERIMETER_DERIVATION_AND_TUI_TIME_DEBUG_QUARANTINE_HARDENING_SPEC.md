# 0024 Phase 3A Content Schema-Version Gate, Meta-Witness Residue, Mutation-Perimeter Derivation, and TUI Time/Debug Quarantine Hardening Spec

**Status**: COMPLETED

**Target repository:** `joeloverbeck/tracewake`
**Target baseline:** local `main` at `4d62f61` (merge PR #31: all `0023PHA3AEMBLOC` tickets landed; acceptance artifact `reports/0023_ord_life_cert_scoped_acceptance.md`; post-capstone commit `a18c06b` "Fixed CI." verified test-only — two discriminating perception tests added to `crates/tracewake-core/src/agent/perception.rs`). All four gates (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) measured green at this baseline before the audit, and re-measured at reassessment on the same tree with per-gate unmasked sentinels — fmt, clippy, build, and test each reporting an individual OK.
**Gate posture:** scoped evidence toward `ORD-LIFE-CERT`; does not certify `ORD-LIFE-CERT`, full project, Phase 4, or later-proof readiness.
**Artifact type:** hardening / anti-contamination spec; replaces nothing; amends no doctrine.

This spec is the thirteenth Phase 3A alignment pass — the strictly scoped verification of
0023's deliverables that 0023 §9 itself prescribed (above all: did the `ORD-HARD-121`
embodied-locality migration genuinely close the truth read, and do the repaired
meta-locks fire with live witnesses?), plus the standing 0005 feature-contract
conformance check, plus the two blind sweeps 0023 §9 named of surfaces the lineage has
never scoped: the content loader end-to-end and the TUI input/command layer. Spec 0014
closed `ORD-HARD-001`–`007`; 0015 closed `008`–`013`; 0016 closed `014`–`025`; 0017
closed `026`–`034`; 0018 closed `035`–`043`; 0019 closed `044`–`052`; 0020 closed
`053`–`065`; 0021 closed `066`–`098`; 0022 closed `099`–`120`; 0023 closed `121`–`139`.
This audit re-derived the normative contract from `docs/0-foundation/*`,
`docs/1-architecture/*`, `docs/2-execution/*`, and `docs/3-reference/*`, and treated
every 0023 correction as unverified until proven in code at `4d62f61`, per the lineage
rule. Findings continue the `ORD-HARD` series at `140`.

All evidence below was verified against local sources at the target baseline. Citations
use named symbols, which are grep-stable; line numbers are omitted deliberately. The
audit was conducted as an eight-slice delegated review (embodied-locality migration
verification, meta-lock tier integrity, TUI debug surfaces, scan-evasion closures,
0005 conformance, content-loader blind sweep, TUI input-layer blind sweep, kernel
event posture + doc honesty) plus an external research refresh. Sixteen of the
twenty-six findings were independently operator-verified at their load-bearing cited
symbols during triage — including every high and every load-bearing medium; the ten
findings initially carried on agent evidence alone were operator-re-verified at
source during spec reassessment at the same baseline — nine confirmed (one with a
count refinement) and one (`ORD-HARD-160`) evidence-corrected from a same-file to a
cross-file alias evasion — so every finding is operator-verified and no
re-verification step is owed (a premise that nonetheless fails at implementation
time is recorded as refuted in the acceptance artifact, not silently dropped).

The headline answers to 0023 §9's questions:

1. **The `ORD-HARD-121` migration genuinely moved the display surface, but the
   promised compile-time lock did not land.** `visible_items`/`visible_doors`/
   `visible_containers`/local actors and their semantic actions are now derived from
   projected actor-known facts, with discriminating INV-093 absence negatives and a
   staleness positive — verified. But `build_embodied_view_model` still takes
   `state: &PhysicalState` as a live parameter (validator preflight and
   `from_sealed_context` truth reads), so the "cannot name raw `PhysicalState`,
   compile-time impossibility" claim is overstated: the actual lock is a two-token
   textual denylist (`ORD-HARD-143`/`154`).
2. **The repaired meta-locks fire with live witnesses for the explicitly-cased scans
   only.** Roughly fourteen registry entries route to genuinely measured counts; the
   remaining entries fall through to a `_ =>` arm whose "witness" is a 0/1
   presence check (`test_names.contains(lock_id) || source.contains(negative_id)`) —
   the decorative literal-witness shape relocated, not removed (`ORD-HARD-141`).
   The `ORD-HARD-124` deferral repair is self-satisfying for its only production
   entry (`ORD-HARD-142`).
3. **One product-behavior foundation violation was found, again outside the surfaces
   the lineage repeatedly audited, on the first-ever sweep of the content loader**
   (`ORD-HARD-140`): the top-level fixture `schema_version` is never validated
   against a supported set — any version string loads silently, the exact posture
   `docs/2-execution/08`'s schema-version rule and INV-020 forbid. The per-belief
   epistemic schema version *is* gated; the package-level gate simply never existed.
   This confirms 0023 §9's prediction verbatim: the residual risk lives between
   audit boundaries, not inside them.

## 1. Scope

### In scope

- The content loader end-to-end (`crates/tracewake-content/src/{schema.rs, load.rs,
  validate.rs, serialization.rs, manifest.rs}`): schema-version gating, the
  raw-line/prose-born-fact scan's production reachability, shortcut-marker coverage
  of ID-typed fields, silent need defaults, round-trip honesty.
- The meta-lock tier residue (`crates/tracewake-core/tests/anti_regression_guards.rs`:
  `meta_lock_live_witness_count`, `embodied_field_has_registered_producer`,
  `mutation_baseline_change_log_records`, `META_LOCK_CENSUS_EXEMPTIONS`).
- The embodied builder's remaining truth access (`projections.rs::
  build_embodied_view_model`, `EmbodiedProjectionSource::from_sealed_context`,
  `guard_014_embodied_projection_source_has_no_physical_state_field`).
- The post-seed mutation perimeter guard
  (`no_direct_apply_event_outside_event_replay_or_pipeline` vs the four
  `pub fn apply_*` mutators in `events/apply.rs`).
- The TUI input/command layer: debug-command gating (`run.rs::render_debug`),
  the `run no-human-day` operator command's quarantine and time-control posture
  (`input.rs::OperatorProofCommand`, `app.rs::run_no_human_day`), render-reachability
  guard scope.
- Projection-policy behavioral coverage residue (`no_human_surface.rs::
  add_workplace_knowledge`), epistemic discrimination witness
  (`hidden_truth_gates.rs::assert_context_excludes_unseeded_hidden_counterparts`),
  perception-scan provenance, and the remaining oracle-quality closures from 0023.
- 0005 contract coherence: candidate-priority ordering record, day-window
  disjointness, capstone event coverage.

### Out of scope

- Re-auditing Phase 1/1A/2A internals except where named findings touch them.
- Phase 3B speech/testimony, Phase 4 institutions, economy, travel, LOD.
- Re-fixing anything 0014–0023 fixed that this audit verified as holding (§3).
- The `ORD-HARD-095` INV-087 owner decision (still deferred; untouched).
- Building player-facing time acceleration with interruption stops and missed-event
  summaries (Phase 3B+ feature work): `ORD-HARD-152` requires the *quarantine and the
  recorded staging decision*, not the feature.
- Identity-uncertainty mechanics (INV-029) beyond preserving the existing seam.

## 2. Doctrine anchors

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — INV-020 (event/schema
  evolution mandatory: reject unsupported history rather than silently inventing
  repairs — `ORD-HARD-140`); INV-022 (raw prose is not authoritative state —
  `ORD-HARD-150`); INV-009/011 (mutation perimeter — `ORD-HARD-144`); INV-067/068/107
  (embodied/debug quarantine — `ORD-HARD-152`/`153`); INV-093; INV-097.
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` — the
  schema-version and migration rule ("silent migration is forbidden") breached by
  `ORD-HARD-140`.
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — time controls
  ("stop on salient perceived interruption", "actor-filtered missed-event summaries")
  are listed as *staged* features, so their absence is lawful only as a *recorded*
  staging decision; debug mode "may not provide normal-play world-affecting
  shortcuts" (`ORD-HARD-152`/`153`).
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — R-28 (one-sibling-over closure:
  the apply-token subset, the ID-newtype census blindness, the render-reachability
  single-file scope), R-29 (guard vacuity: the presence-check default witness arm,
  the self-satisfying deferral cite, the literal-conditioned discrimination witness).
- The lineage's enforcement reading: a correction to a classified defect is complete
  only when every surface in the defect's class is corrected or explicitly exempted
  with rationale — and a lock is real only if its negative case can actually fire on
  the behavior.

**External research consulted this pass** (extends the 0017/0018/0022/0023 corpus;
all adopted techniques survive the zero-dependency constraint):

- *Rotten green tests* (Delplanque et al., ICSE 2019; Google Test study, ESEC/FSE
  2023): passing tests whose assertions never execute — directly the
  `ORD-HARD-141`/`142` shape; the portable countermeasure is counting assertion/scan
  executions and failing on zero, which the §5 witness repair adopts.
- *Metamorphic testing of static-analysis rules* (StaAgent, arXiv 2507.15892, LLM
  tooling not adopted): the portable idea is a per-scan **kill set** — semantically
  equivalent rewrites of known-bad code that must still trip the guard; adopted for
  the perception/laundering scans (`ORD-HARD-149`).
- *Vacuity detection*: every forbidden-pattern scan keeps at least one committed
  known-bad fixture that must fire (antecedent-failure detection); adopted as the
  live-witness floor semantics (`ORD-HARD-141`/`146`).
- *Schema-evolution practice* (event-versioning patterns, upcaster discipline;
  zero-dep translation): explicit version gate + committed golden fixtures per
  historical version with a test asserting the loader still accepts each archived
  version or rejects it loudly (`ORD-HARD-140`).
- *Round-trip property discipline* (UPenn PL Club; exact-printing biparsers, ACM
  2025): `parse(print(x)) == canonicalize(x)` is insufficient against symmetric
  encoder/decoder bugs; add frozen golden bytes (`ORD-HARD-165`).
- *Swarm testing* (Groce et al.): seeded random command-subset episodes for the
  command loop, hand-rolled (optional, recorded for Phase 3B+; not a finding).
- Flagged not-applicable (so they are not re-researched): insta/proptest/quickcheck/
  cargo-fuzz/radamsa (dependencies), ParVAL/traceable-RFC LLM oracles (tooling),
  checked coverage via dynamic slicing (impractical zero-dep; the rotten-green
  execution counter is the cheap proxy).

## 3. Determination

**Positive verdict.** The in-scope Phase 3A surface is not foundation-aligned and not
maximally locked. Twenty-six findings follow, `ORD-HARD-140` … `ORD-HARD-165`: three
high, eleven medium, twelve low, zero blockers. One finding is a product-behavior
foundation violation at the letter of INV-020 and the execution schema rule
(`ORD-HARD-140`); two are decorative-guard defects in the repaired meta-tier itself
(`ORD-HARD-141`/`142`). No INV-099…INV-106 truth-firewall violation exists in
production *cognition* — the cognition firewall verified clean for the **seventh
consecutive pass**. The 0005 feature contract is intact end-to-end; no 0014–0023 lock
narrowed product behavior below the 0005 contract.

### Verified holding (no action; recorded so they are not re-litigated)

- **The `ORD-HARD-121` display migration is substantive.** `build_embodied_view_model`
  derives `visible_items`/`visible_doors`/`visible_containers`/local actors from
  `source.actor_known_*` projected facts; semantic actions derive from those
  collections; concealment is decided at observation-recording time
  (`visible_item_payload` drops closed/not-visible container contents before the
  event is minted); `visible_exit_blocker_summary` reads projected doors. The INV-093
  absence negative (`embodied_projection_omits_unobserved_present_item_and_actor`)
  and staleness positive (`embodied_projection_renders_stale_projected_item_not_live_truth`)
  both discriminate. The INV-029 identity seam exists
  (`ActorKnownProjectionRecord::LocalActor { observed_actor_id, .. }`).
- **The post-capstone `a18c06b` tests discriminate.**
  `visible_item_perception_follows_current_place_match_not_id_prose` and
  `visible_item_perception_reveals_open_container_without_closed_visibility` assert
  cross-place exclusion and open-container revelation against engine-emitted events;
  neither echoes its source.
- **`ORD-HARD-123` (census closure) is real.** Membership derives from the full
  `#[test]` population (`anti_regression_test_names`) minus a rationale-bearing
  allowlist; the unprefixed-lock synthetic fires; the eleven locks 0023 named are
  enrolled. (Exemption-rationale validation residue: `ORD-HARD-155`.)
- **`ORD-HARD-129` (chained ledger) is real for interior links.** From→to count+hash
  chain with `windows(2)` adjacency and head pinned to consts; interior
  missing-predecessor and unrecorded-raise negatives fire. (Genesis anchoring residue:
  `ORD-HARD-145`.)
- **`ORD-HARD-125`/`135` are closed.** `render_debug_overlay` is production-wired
  (`render_debug` → `DebugCommand::Overlay` → `render_debug_embodied_overlay`), gated
  on derived `debug_available`; `DEBUG_TOKENS` is a shared const consumed by overlay
  and negatives, with the extension synthetic firing. (Guard-scope residue:
  `ORD-HARD-157`; sibling-command gating: `ORD-HARD-153`.)
- **`ORD-HARD-126` is closed for food/sleep and for classification/embodied axes.**
  The policy test builds real projections, real embodied contexts, and the real
  no-human surface; food and sleep `*_believed_accessible` facts are gated and
  row-mutation negatives fire. (Workplace accessibility residue: `ORD-HARD-147`;
  oracle hand-derivation residue: `ORD-HARD-156`.)
- **`ORD-HARD-127`/`128`/`130`/`132`/`134`/`136`/`137`/`138`/`139` closures are
  substantive at their promised shapes**: laundered let-binding and bare-`String`
  `starts_with` synthetics fire; YAML continuation lines are joined
  (`logical_shell_lines`) with the continuation-suffix synthetic firing;
  receiver/alias payload shapes are tracked (`payload_binding_aliases`) with both
  synthetics firing; `.unwrap(` is in the panic-scan token set with a firing
  synthetic; non-empty-baseline governance re-arms
  (`mutation_baseline_governance_errors`) with the unledgered-entry synthetic;
  the bare `EventEnvelope` token is banned in `support/generative.rs` with
  `default()`/struct-literal synthetics; the Mara resolution token is required and
  asserted against runner-only behavior; the embodied sleep positive exists
  (`accepted_sleep_started_event`); `cause_required` is an exhaustive `match` with no
  `_` arm. (Next-shape residues: `ORD-HARD-149`/`158`/`159`/`160`.)
- **`ORD-HARD-131` is structurally in-`context()`**: the discrimination call runs
  unconditionally at `context()` return. (Literal-conditioning residue:
  `ORD-HARD-148`.)
- **The 0005 feature contract is intact end-to-end at `4d62f61`.** Need bands exact
  at §8.1 boundaries (`NeedBand::for_value`, `u16`, clamped, both-direction evented
  crossings); intention lifecycle typed; sleep/work duration-based with
  body-exclusive reservation and prorated costs; eat consumes modeled servings with
  typed refill-free failures; `run_no_human_day` autonomous through
  `ActorDecisionTransaction` + `run_pipeline` with no direct dispatch; all ten
  routine families authored with failure modes; all blocker categories producible;
  the five-action embodied sweep is positive for all five actions; possession parity
  holds; capstone proves live==rebuild typed equality. No lock narrowed product
  behavior below the contract.
- **The production cognition firewall is clean (seventh consecutive pass).**
  Command parsing tokenizes only; embodied submission flows
  `submit_semantic_action` → sealed `KnowledgeContext` → `run_pipeline`; forged
  semantic IDs rejected without checksum change; possession rebind transfers no
  notebook/why-not/debug truth; debug renderers take `&self` and mutate nothing.
- **Kernel posture holds.** Replay rejects loudly with position+kind on unsupported
  schema/kind across world/agent/epistemic streams; agent-state checksum coverage
  enumerates all eleven authoritative maps; `scheduler_never_direct_dispatches_primitive_action`
  carries live witnesses; conformance-index rows 112–115 cite symbols that resolve;
  R-28/R-29 extensions promised by 0023 §6 landed; the 0023 acceptance artifact's
  §7 map cites real symbols, records the scheduled mutation run as honestly pending,
  and carries the non-certification statement; EMERGE-OBS derivation is
  measurement-only and replay-byte-identical.

### Validated — no action (checked, found intentional or correctly scoped)

- `parse_command`'s digit-prefix menu-index precedence is load-bearing but currently
  collision-free; no panic path exists on user input (all ID constructions
  `map_err`-guarded). Documented-precedence note only; no lock owed.
- `EpistemicApplyError::MissingHolder`/`MissingSource` fire on present-but-malformed
  values; a wholly absent key routes to `MissingPayload`. Replay still rejects
  loudly; the variant naming is a mapping artifact, not a gap.
- `from_observed_parts` underscore-bound parameters, the `witness_min: 0`
  empty-baseline exemption, and `record_current_place_perception` as the lawful
  truth→event boundary: re-confirmed as correctly scoped (0023 dispositions hold).
- The capstone's manual-injection ordering (`ORD-HARD-120` fix) and the legacy
  blanket-seeding allowlist censuses: hold as locked.

## 4. Findings and remediation requirements

Severity calibration notes: `ORD-HARD-140` is rated high where its closest letter
violation precedent (`ORD-HARD-107`, medium) was a single-emission-site residue —
the divergence is that 140 is the *entire absence* of a constitutionally mandated
gate (INV-020 / execution §08), structurally fail-open for every future schema
revision, on a surface never before audited; like `ORD-HARD-121` it is not a blocker
because every in-repo fixture currently declares the sole real version.
`ORD-HARD-141` and `142` are rated high per the `ORD-HARD-122` precedent: each is
the anti-rotten-green repair itself carrying a rotten-green shape (a presence-check
fallback arm; a self-satisfying citation). `ORD-HARD-144` and `148` are rated medium
where their reporting slices proposed high: each is a derivation/scope gap with no
live false pass today (precedents `ORD-HARD-123`/`131`, medium/low). `ORD-HARD-158`
and `159` are rated low where the reporting slice proposed medium (precedents
`ORD-HARD-137`/`138`, low).

### ORD-HARD-140 — The top-level fixture `schema_version` is never validated against a supported set: any version string loads silently

**Severity:** high (calibration divergence from `ORD-HARD-107` named above).
**Verification:** operator-verified (`validate.rs` consumes `fixture.schema_version`
only as a duplicate-key seed in `validate_ids`; the epistemic contrast gate
`unsupported_epistemic_schema_version` exists at `validate_epistemic_seeds`; no
fixture-level membership check exists anywhere in `validate_fixture_errors`).

**Responsible layers:** content schema/validation, replay-evidence honesty.

**Doctrine breached:** INV-020 (schema evolution mandatory — reject unsupported
history/content rather than silently inventing repairs);
`docs/2-execution/08`'s schema-version and migration rule ("Replay must reject
mismatched content unless a named migration/upcast path exists… Silent migration is
forbidden").

**Evidence:** `FixtureSchema.schema_version` is consumed only by `validate_ids` (as
a duplicate-key seed) and copied verbatim into `ContentManifest::new` and the content
fingerprint by `load_fixture_package`. There is no check that
`schema_version == "schema_v1"` (or membership in any supported set) on any load or
validation path. By contrast, `validate_epistemic_seeds` gates each belief's
`schema_version` against `EPISTEMIC_RECORD_SCHEMA_V1` and emits
`unsupported_epistemic_schema_version` — the per-record gate was hardened in a prior
pass while the package-level gate was never created. A fixture declaring
`schema|schema_v999` deserializes, validates, loads, and propagates the bogus version
into the manifest with no diagnostic.

**Why twelve passes missed it:** the lineage audited the kernel's event schema gates
(INV-020 was verified at `apply_event`/replay) and the epistemic seed records; the
content crate's package-level version field sits between those boundaries and the
loader was never a scoped surface.

**Required correction:** add a fixture-level gate rejecting any
`fixture.schema_version` outside a named supported set (`FIXTURE_SCHEMA_V1` const),
phase `ParseSchema`, code `unsupported_fixture_schema_version`; thread the same gate
through `load_fixture_package` so no load path bypasses it; commit a golden fixture
per historical schema version (currently one) with a test asserting each archived
version still loads or is rejected loudly by name.

**Structural lock:** a negative test feeding `schema|schema_v999` through
`load_fixture_package` (not just `validate_fixture_bytes`) must fail with the typed
code; a `CONTENT_NEGATIVE_PROOFS`-style census row so deleting the rejection path
fails the census; the registry row for `schema_version` must be backed by the named
rejection test.

### ORD-HARD-141 — The repaired live-witness rule has a presence-check default arm: most registry entries' "live witness" is `test_names.contains(lock_id)`

**Severity:** high (precedent `ORD-HARD-122` — the same class, on the repair).
**Verification:** operator-verified (the `_ =>` arm of `meta_lock_live_witness_count`
returning `usize::from(test_names.contains(entry.lock_id) ||
anti_regression_source.contains(entry.negative_id))`).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-29 (literal-witness shape — an authored/derived constant
standing in for a measured count); 0023 §4 `ORD-HARD-122`'s required correction
("each enrolled scan returns its real matched-site count from the scan body").

**Evidence:** `meta_lock_live_witness_count` explicitly cases roughly fourteen
lock_ids onto genuinely measured counts (apply-arm scans, perception, generative
ratchet, mutant misses, etc.). Every other entry — all `BehaviorAssertion` routings
plus SharedScans such as `generative_support_constructs_zero_event_envelopes` and
`hidden_food_closed_container_is_not_actor_known_food_source` — falls through to the
default arm, where the "witness" is a 0/1 presence check on the test's *name* or its
negative's *string*, compared against `witness_min: 1`. That proves the test exists,
not that its scan matched real sites: renaming a default-arm scan's internal anchor
(so its violation set goes vacuously empty) leaves the witness at 1 and the suite
green. The single anchor-miss negative exercises only the one explicitly-cased apply
scan. Additionally (per the same shape), several *cased* witnesses measure anchor
presence rather than violation-detector liveness — see `ORD-HARD-146`.

**Why existing gates miss it:** the meta-test's one anchor-miss synthetic targets the
one scan whose witness re-derives from `EVENTS_APPLY_RS`; no negative exercises a
default-arm entry.

**Required correction:** eliminate the presence-check default arm: every SharedScan
entry routes to a real matched/inspected-site count derived from the scan body;
`BehaviorAssertion` rows get a distinct routing whose witness is the negative's
actual execution (run-and-must-fire), not name presence. Adopt the rotten-green
counter semantics from the research corpus: a scan that inspected zero sites is a
failure regardless of its violation set.

**Structural lock:** a synthetic that renames a default-arm scan's internal anchor
(vacuous violation set) must drop its witness below minimum; the registry census must
reject any new entry that routes to a presence-shaped witness.

### ORD-HARD-142 — The deferral content witness is self-satisfying: the cite points at the file that defines the field, so `source.contains(field_name)` can never fail

**Severity:** high (precedent `ORD-HARD-124`, medium; elevated because the delivered
repair is unfalsifiable for its only production entry — the `ORD-HARD-122`
rotten-green shape applied to the `ORD-HARD-124` repair).
**Verification:** operator-verified (`embodied_field_has_registered_producer`'s
empty-snippet branch requires only `source.contains(entry.field_name)`; the
`debug_only_diagnostics` deferral entry's `source_path` is `view_models.rs`, the file
that defines `ActionAvailability::Disabled { debug_only_diagnostics }` and
`pub fn debug_only_diagnostics()`).

**Responsible layers:** `test_oracle`, TUI.

**Doctrine breached:** R-29 (lock durability) — `ORD-HARD-124`'s required "content
witness" was delivered as a containment check satisfied by the witnessed field's own
type definition.

**Evidence:** the repaired predicate's cite-only path passes when the cited source
contains the field name — but the production `debug_only_diagnostics` entry cites
`view_models.rs`, where the field name appears in the enum variant declaration, the
constructor, and the accessor. The field could be orphaned in production (no producer
write, no consumer) and the sweep stays green. The
`synthetic_orphaned_deferral_embodied_surface_producer` negative only proves the
predicate fails against a *stripped stub* source omitting the variant — a case the
real self-referential file can never reach.

**Why existing gates miss it:** the negative exercises absence-of-field, which the
definition site makes impossible; no negative exercises
present-as-definition-but-orphaned.

**Required correction:** the cite-only path must require a write-shaped or
consumer-shaped occurrence (constructor field init, `.field_name(` call, or an
app-layer consumer citation) outside the field's own definition span — or carry an
explicit consumer-existence proof (a named non-test caller asserted present).

**Structural lock:** a synthetic deferral entry whose cited source contains the field
only as a `struct`/`enum`/`fn` definition (no write or consume site) must fail;
enroll the per-entry match in the repaired live-witness rule (`ORD-HARD-141`).

### ORD-HARD-143 — The promised compile-time truth-access lock did not land: `build_embodied_view_model` still takes `state: &PhysicalState`, enforced only by a two-token denylist

**Severity:** medium (precedent `ORD-HARD-121` residue; the display surface itself is
verified migrated, so no truth renders today).
**Verification:** operator-verified (the `state: &PhysicalState` parameter on
`build_embodied_view_model` and its flow into `SemanticActionPreflightContext`;
`guard_014_embodied_projection_source_has_no_physical_state_field` checks only the
struct body for `PhysicalState`/`state:` fields and the builder body for the two
substrings `source.state` and `visible_locality`).

**Responsible layers:** projections, `test_oracle`.

**Doctrine breached:** 0023 §4 `ORD-HARD-121`'s structural-lock clause ("the embodied
builder *cannot* name raw `PhysicalState` — compile-time impossibility, the lineage's
strongest tier") and §5 tier 1; R-29 (the delivered tier is a token denylist whose
uncovered shapes are exactly the dangerous ones).

**Evidence:** `EmbodiedProjectionSource` carries no `state` field (the struct
quarantine landed), but the builder function signature still receives
`&PhysicalState` for validator preflight, and
`EmbodiedProjectionSource::from_sealed_context` also takes `&PhysicalState`, reading
`state.items`/`state.places`/`state.actors` for carried-items/place-label/
current-place derivation. A future author can read `state.doors` or `state.items`
directly inside `build_embodied_view_model` — neither banned substring fires, no type
error occurs, and a truth-derived surface returns.

**Why existing gates miss it:** `guard_014` bans `source.state` (the old access path)
and `visible_locality` (the old helper), not the live `state` parameter or its field
reads.

**Required correction:** preferred: remove `&PhysicalState` from the embodied builder
entirely — move validator preflight behind a sealed boundary (the preflight needs
truth lawfully, but the *builder* should receive a preflight-result or a
validation-capability object, not the raw state), and move `from_sealed_context`'s
body-state derivation to the construction site (see `ORD-HARD-154`). Fallback if
preflight must keep truth access: wrap it in a `ValidatorOnlyState` newtype exposing
no map accessors, so the builder cannot iterate truth.

**Structural lock:** compile-time: `build_embodied_view_model` and
`from_sealed_context` no longer name `PhysicalState` (then extend `guard_014` to ban
the type token in the whole embodied builder region as the backstop, with a firing
synthetic for a bare `state.items` read — the current synthetic only re-adds the
struct field).

### ORD-HARD-144 — The post-seed mutation guard scans two of the four public apply mutators: `apply_agent_event`/`apply_epistemic_event` are outside the perimeter

**Severity:** medium (precedent `ORD-HARD-123`, derivation-scope gap with no live
false pass; the reporting slice proposed high — divergence named).
**Verification:** operator-verified (`no_direct_apply_event_outside_event_replay_or_pipeline`
forbids only the literals `apply_event(`/`apply_event_stream(`; `events/apply.rs`
exposes `pub fn apply_event_stream`, `apply_event`, `apply_epistemic_event`,
`apply_agent_event`; `scheduler.rs` carries production `apply_agent_event`/
`apply_epistemic_event` call sites while `agent/no_human_surface.rs`'s apply helper
is test-only, and the live production tokens are not scanned).

**Responsible layers:** `test_oracle`, kernel mutation perimeter.

**Doctrine breached:** R-28 (derive membership instead of trusting review memory —
the forbidden-token set is a hand-picked subset of the real mutator population);
INV-009/011 direction (the guard's claim "event application is the only post-seed
mutation path" is enforced for half the entry points).

**Evidence:** the current production call sites are legitimate (scheduler and
pipeline route agent/epistemic application lawfully), but the guard cannot tell: a new
`apply_agent_event` call in a view model, content loader, or TUI path would mutate
authoritative agent state with zero guard firing. The companion ordering check
(`accepted_action_appends_before_authoritative_apply`) checks a string pair only.
The scan also has no nonzero-witness floor: it is not enrolled in
`META_LOCK_REGISTRY`, so a refactor that renames the allowlisted seams' calls leaves
it green with zero real sites (the `ORD-HARD-122` vacuous-empty shape).

**Why existing gates miss it:** the token list predates the agent/epistemic apply
functions; nothing derives it from `apply.rs`'s public surface.

**Required correction:** derive the forbidden-token set from the `pub fn apply_*`
signatures in `events/apply.rs` (a parity census so adding a fifth mutator
auto-extends the scan); allowlist the live production seams
`scheduler.rs`/`actions/pipeline.rs`/`replay/rebuild.rs` with cited rationales;
enroll the scan in `META_LOCK_REGISTRY` with a live witness per allowlisted seam.

**Structural lock:** a synthetic injecting `apply_agent_event(` into a
non-allowlisted source must fire; the parity census must fail when `apply.rs` gains a
`pub fn apply_*` absent from the scan's token set.

### ORD-HARD-145 — The baseline change-log chain is unanchored at genesis: a fabricated single-link ledger with a matching head passes

**Severity:** medium (precedent `ORD-HARD-129`).
**Verification:** operator-verified (`mutation_baseline_change_log_records` walks
`windows(2)` adjacency and pins only the head `to_count`/`to_hash` to the consts; no
assertion on the first delta's `from` values).

**Responsible layers:** `test_oracle`, CI.

**Doctrine breached:** lock durability (R-29) — the `ORD-HARD-129` chain records
transitions but not the chain's origin, so wholesale ledger replacement is
indistinguishable from history.

**Evidence:** replacing the entire delta list with one line whose `from` equals its
`to` equals the pinned head (`0`/offset-basis hash) yields zero `windows(2)` pairs
and a matching head — passing both checks while erasing the recorded 143→0
retirement history.

**Why existing gates miss it:** the missing-predecessor negative perturbs an interior
link only.

**Required correction:** pin the genesis `from` (143 / its recorded hash) as consts
and assert `deltas.first()` matches; assert chain length is monotonically
non-decreasing (or pin a full-list content hash updated by the same two-sided ratchet
discipline).

**Structural lock:** a ledger whose first delta's `from` differs from the pinned
genesis must fail; a shortened chain must fail.

### ORD-HARD-146 — Several explicitly-cased witness counts measure anchor presence, not violation-detector liveness

**Severity:** medium (precedent `ORD-HARD-122` residue).
**Verification:** operator-verified at reassessment (all three cited arms confirmed
verbatim in `meta_lock_live_witness_count`: exemption-entry count,
`run_no_human_day` line count, three literal markers).

**Responsible layers:** `test_oracle` (meta-lock tier).

**Doctrine breached:** R-29 — the witness and the violation predicate read different
things, so a defeated detector leaves the witness pinned.

**Evidence:** for these entries the measured count tracks a fixture/anchor population
(exemption entries, marker literals) rather than the set of sites the violation
predicate inspected; the detector can go vacuous while the witness holds.

**Required correction:** per emptiness-asserting scan, the witness is the count of
sites the violation predicate actually inspected (lines examined, files matched), not
a neighboring anchor population.

**Structural lock:** per repaired entry, a synthetic emptying the inspected-site set
(while leaving anchors intact) must drop the witness below minimum.

### ORD-HARD-147 — `workplace_believed_accessible` is minted unconditionally: the workplace `accessibility_scope` policy column is behaviorally inert and its mutation is uncaught

**Severity:** medium (precedent `ORD-HARD-126` — the direct residue).
**Verification:** operator-verified (`add_workplace_knowledge` in
`agent/no_human_surface.rs` pushes `workplace_believed_accessible` with no
`accessibility_scope` gate; the food and sleep emitters both gate on
`ActorKnownProjectionAccessibilityScope::FromAnyPlace`).

**Responsible layers:** `holder_known_context`, `test_oracle`.

**Doctrine breached:** lock durability (R-29 self-echo shape) — the policy table's
workplace accessibility row asserts a value the surface ignores.

**Evidence:** the policy behavioral test's workplace accessibility axis compares
expected-from-table against a fact that is always present, so it cannot fail for any
table contents; the row-mutation negative
(`actor_known_projection_policy_table_detects_synthetic_row_mutations`) mutates only
`food_source.accessibility_scope`, never workplace's.

**Required correction:** gate the workplace fact on
`policy.accessibility_scope == FromAnyPlace` (parity with food/sleep), or amend the
table row to declare the fact unconditional with rationale.

**Structural lock:** a fourth synthetic row mutation flipping
`workplace.accessibility_scope` must produce a detected mismatch.

### ORD-HARD-148 — The in-`context()` discrimination witness is keyed to hardcoded counterpart literals, and its standalone "leak" witness is a self-contains vacuity assert

**Severity:** medium (precedent `ORD-HARD-131`, low; elevated one band because the
standalone witness is the R-29 decorative shape — an assertion that cannot fail; the
reporting slice proposed high — divergence named).
**Verification:** operator-verified (the
`if requested_food_sources.contains("food_visible_table")` gate inside
`assert_context_excludes_unseeded_hidden_counterparts`; the
`synthetic_context_leak = "pub viewer_actor_id: ActorId"` /
`assert!(synthetic_context_leak.contains("pub viewer_actor_id"))` pair in
`hidden_truth_gates.rs`).

**Responsible layers:** `test_oracle`, epistemics gates.

**Doctrine breached:** R-29; `ORD-HARD-131`'s required correction ("hidden
counterparts absent from the returned context, asserted unconditionally").

**Evidence:** the discrimination assertions run only when the caller requests the
exact literals `food_visible_table`/the known-market edge; a future `context()`
caller with differently-named fixtures receives zero hidden-exclusion checks, and a
leak under a new name passes. The separate `synthetic_context_leak` assertion is a
string asserted to contain itself — it can never fail and witnesses nothing.

**Required correction:** closed-world discrimination: derive the seeded set from the
epistemic events actually applied and assert `context()`'s known
food-sources/edges/workplaces are a subset of the seeded set, independent of names;
delete the self-contains assert.

**Structural lock:** a synthetic seeding the projection with one extra unseeded
hidden food/route must make `context()` fail; the deleted vacuity assert is replaced
by that firing negative.

### ORD-HARD-149 — The perception prose-branch scan has no binding/parameter provenance: a renamed `&str` helper parameter launders a non-"hidden" discriminator

**Severity:** medium (precedent `ORD-HARD-127` — the next laundering shape over).
**Verification:** operator-verified at reassessment
(`perception_visibility_prose_branch_violations` confirmed per-line lexical with the
same-line `PayloadField` exemption and no binding provenance; a helper
`fn gate(label: &str) -> bool { label.starts_with("vault") }` body carries none of
the scanned tokens).

**Responsible layers:** `test_oracle`, perception channel.

**Doctrine breached:** INV-022 lock durability — the `ORD-HARD-127` widening is
lexical per-line; provenance through a parameter rename is invisible.

**Evidence:** the call site `gate(&place.display_label)` is flagged, but routing the
label through an exempted typed-payload write first, then branching on the payload
value in a helper whose parameter is renamed, escapes every token.

**Required correction:** port the `payload_binding_aliases` provenance approach
(already built for `ORD-HARD-130`) to the perception scan: track bindings derived
from `display_label`/`*_id` projections across `let` and parameter boundaries within
the file; the typed `PayloadField` write is the only sink.

**Structural lock:** a kill-set per the research corpus: two semantically equivalent
rewrites of the known-bad branch (renamed-parameter helper; payload-value relay) must
both fail the scan.

### ORD-HARD-150 — The raw-line validation layer (prose-born-fact, unknown-field, forbidden top-level keys) never runs on the production load path

**Severity:** medium (precedent `ORD-HARD-125` — a delivered surface unreachable from
production).
**Verification:** operator-verified (`load_fixture_package` calls
`deserialize_fixture` → `validate_fixture`; `validate_raw_lines` is invoked only by
`validate_fixture_bytes`).

**Responsible layers:** content validation.

**Doctrine breached:** INV-022 (the `prose_born_fact` rejection is structurally
unreachable in production — `note|`/`notes|`/`description|` tags fail earlier as
`BadLine`, so the typed INV-022 diagnostic can never fire on a real load); INV-097
(the top-level forbidden-key scan similarly bypassed, mitigated by fail-closed
`BadLine` on unknown sections).

**Evidence:** the INV-022 proof (`content_prose_born_fact_rejected`,
`prose_born_fact_rejected_001`) drives `validate_fixture_bytes` directly with
hand-fed bytes, masking that no production path reaches the check.

**Why existing gates miss it:** the negative proves the predicate, not the path.

**Required correction:** route `load_fixture_package` through
`validate_fixture_bytes` (it already holds `primary.bytes`), so the raw-line scan
runs on every load; keep the struct-path validation as the second phase.

**Structural lock:** a test loading a prose-marker fixture through
`load_fixture_package` (not `validate_fixture_bytes`) must fail with the
`prose_born_fact` code; same for one forbidden top-level key.

### ORD-HARD-151 — The shortcut/script-marker scan is blind to ID-typed fields: `appear_at_workshop` passes as an item id

**Severity:** medium (precedent `ORD-HARD-123` shape — census scoped to a type
convention).
**Verification:** operator-verified (`reject_reserved_or_display` is a separate,
smaller hand-list than `PHASE3A_SHORTCUT_MARKERS`; `SCANNED_STRING_FIELDS` registers
`String`-typed fields only; the conformance census `discover_schema_fields` covers
`String`/`Option<String>`/`Vec<String>` and cannot see ID newtypes).

**Responsible layers:** content validation, `test_oracle`.

**Doctrine breached:** INV-097/0005 §16.5 direction — the forbidden-marker family is
enforced per-type-convention, not per-authored-value.

**Evidence:** stable-ID newtype fields (`ItemId`, `PlaceId`, etc.) are checked only
against the seven reserved display words; a fixture authoring
`item|appear_at_workshop|…` or `set_need_hunger` as an id carries a script marker
into authored content unflagged.

**Required correction:** route every stable-ID validation through the same
`is_phase3a_shortcut_marker`/authored-outcome predicate used for strings (substring
or token match on the id).

**Structural lock:** extend the `discover_schema_fields` census to enumerate ID-typed
fields and assert each is shortcut-scanned; a negative fixture with a marker-bearing
id must fail.

### ORD-HARD-152 — `run no-human-day` is an ungated top-level command that autonomously fast-forwards the whole world — including the possessed actor — with no interruption stop, no missed-event summary, and no recorded staging decision

**Severity:** medium (foundation lists interruption-stop/missed-event summaries as
*staged* TUI features, so this is an unrecorded-staging + quarantine defect, not a
truth-firewall violation; merges the input-layer slice's two convergent findings).
**Verification:** operator-verified (`parse_command` classifies the command as
`UiCommand::OperatorProof`, a sibling of `view`/`wait`, dispatched in `run.rs` with
no `debug_available` check; `TuiApp::run_no_human_day` passes
`actor_ids: Vec::new()`; the scheduler's empty-list fill to all actors verified at
`config.actor_ids.is_empty()` in `scheduler.rs`; no execution doc records the
staging — swept).

**Responsible layers:** TUI, scheduler boundary, execution docs.

**Doctrine breached:** `docs/0-foundation/08` debug rule ("debug mode may not provide
normal-play world-affecting shortcuts" — the command prints a `DEBUG NON-DIEGETIC`
panel yet sits on the normal-play command surface, ungated); the staged time-control
contract ("stop on salient perceived interruption", "actor-filtered missed-event
summaries") with no execution-doc record of the staging decision — the same
unrecorded-deferral defect class as `ORD-HARD-121`'s "no execution doc records the
locality surface as an intended deferral"; INV-068 boundary hygiene.

**Evidence:** a bound player can type `run no-human-day` as an ordinary command; the
possessed actor is advanced by autonomous routines for a full day; the view re-renders
at `final_tick` with no stop opportunity and no summary of what the actor perceived
meanwhile. The adversarial gate
(`adversarial_gates_no_human_operator_output_stays_debug_only`) quarantines the
*output*, blessing the command surface itself.

**Required correction (scoped — this spec does not build the player feature):**
classify the command as an operator/debug surface: move it under the `debug ` prefix
(or an explicit operator mode) and gate it on derived `debug_available`; record in
`docs/2-execution/07` (a) the operator-proof classification and (b) the staging
status of player-facing time acceleration with interruption stops and missed-event
summaries (Phase 3B+), so the deferral is execution-doc-recorded per the lineage
rule. If maintainers instead elect to keep it as a player verb, the interruption-stop
contract becomes mandatory — record the owner decision either way.

**Structural lock:** a command-loop test asserting world-advancing operator commands
are rejected without debug availability; a parser census asserting the embodied verb
set contains no world-advancing operator commands outside the gated prefix.

### ORD-HARD-153 — Every debug panel except the overlay is dispatched without a `debug_available` check: the derived gate is decorative for nine of ten debug surfaces

**Severity:** medium (precedent `ORD-HARD-104` family — a derived gate consulted by
one consumer).
**Verification:** operator-verified (`run.rs::render_debug`: only
`DebugCommand::Overlay` routes through `render_debug_embodied_overlay`'s gating;
`EventLog`/`Beliefs`/`Observations`/`Planner`/`Epistemics`/`Replay`/
`ProjectionRebuild`/`NoHumanDay`/`Needs`/`Routines`/`Stuck`/`Actor` arms call their
render methods directly).

**Responsible layers:** TUI.

**Doctrine breached:** INV-107/068 direction — debug omniscience is quarantined and
visibly non-diegetic; the 0022-derived `debug_available` predicate
(`debug_available_for`) exists precisely to gate debug access, and nine surfaces
ignore it. (The panels are read-only and marked non-diegetic, so this is boundary
hygiene, not leakage into cognition.)

**Evidence:** `debug beliefs <actor>` dumps any actor's belief store regardless of
binding state; tests always bind first, so the ungated path is never exercised.

**Required correction:** route every `DebugCommand` arm through one gating helper
that consults the same derived availability the overlay uses, returning a typed
"debug unavailable" message otherwise (with the gating policy recorded — if
maintainers intend debug panels to be unconditionally available in the local TUI,
record that as the owner decision and gate only the embodied-adjacent surfaces).

**Structural lock:** a command-loop test issuing a debug command with no bound
controller and asserting refusal; a guard asserting every `DebugCommand` match arm
routes through the gating helper.

### Low findings

Each low finding retains the full remediation obligation; the compact format lists
evidence → correction → lock inline. Verification status is tagged per finding;
every initially agent-reported low finding was operator-re-verified at source during
spec reassessment (`ORD-HARD-160` evidence-corrected in place).

**ORD-HARD-154 — `from_sealed_context` reads live truth for `place_label` and
per-item fields** (projections / low; precedent `ORD-HARD-121` sanctioned-residue;
operator-verified at reassessment). Carried-item *membership* is lawful body-state, but per-item
`portable`/`source` fields and the place's `display_label` are read from live
`PhysicalState` at view-build time, not from recorded observations — a
truth-changed-post-observation value renders fresh. Correction: derive the label and
item attributes at observation/construction time into the sealed source (folds into
the `ORD-HARD-143` parameter removal). Lock: the compile-time removal plus a
staleness positive for the place label.

**ORD-HARD-155 — Census exemption rationales are unvalidated prose** (`test_oracle` /
low; precedent `ORD-HARD-123`; operator-verified at reassessment).
`META_LOCK_CENSUS_EXEMPTIONS` accepts
any non-empty rationale; a future structural lock can park itself there with a
plausible string and escape enrollment. Correction: each exemption either names a
covering registry `lock_id` (asserted present) or is asserted to contain no
violation-set scan shape. Lock: a synthetic exemption wrapping a real scan must fail.

**ORD-HARD-156 — The policy oracle hand-derives the latest-record flag against a
hardcoded place literal** (`test_oracle` / low; precedent `ORD-HARD-126`;
operator-verified at reassessment). `expected_embodied_presence` computes `latest_current_place_record`
via `== "home_tomas"` and re-calls the production `includes_in_embodied_context`; a
regression in the real latest-record derivation is masked for the generic loop (the
dedicated stale-path test mitigates). Correction: derive the flag from the real
classifier output. Lock: a stale-record row mutation through the generic loop.

**ORD-HARD-157 — The render-reachability guard scans `render.rs` only; eleven
`pub fn render_*_panel` functions in `debug_panels.rs` are outside it** (TUI / low;
precedent `ORD-HARD-125`; operator-verified at reassessment). The exact
`ORD-HARD-125` failure mode
(orphaned renderer) is unguarded for the panel file. Correction: add
`debug_panels.rs` to `render_reachability_errors`' producer sources. Lock: a
synthetic uncalled `*_panel` must fail the widened guard.

**ORD-HARD-158 — The Mara resolution negative exercises token recognition, not log
discrimination** (content fixtures / low; precedent `ORD-HARD-137`; the reporting
slice proposed medium — divergence named; operator-verified). The `flipped_resolution`
synthetic flips the token to an unsupported value, firing the `other =>` arm; no
negative proves an injected autonomous `FoodConsumed` for Mara fails the
`fail_only_empty_food_source` arm. Correction + lock: a tampered-log synthetic
(`synthetic_mara_autonomous_food_consumed_fails_resolution`) asserting the
resolution-violation arm fires.

**ORD-HARD-159 — No per-ordinary-action positive-coverage census exists** (TUI /
low; precedent `ORD-HARD-138`; the reporting slice proposed medium — divergence
named; operator-verified via zero census matches in `tui_acceptance.rs`). The sleep
positive landed as an instance; eat/work/continue/wait positives exist ad hoc; a new
ordinary action can ship with only negatives. Correction: a guard enumerating
ordinary semantic action ids from the registry and asserting each has a submitted
positive in the acceptance suite, with rationale-bearing exemptions. Lock: removing
the sleep positive must fail the census.

**ORD-HARD-160 — The support `EventEnvelope` ban scans `support/generative.rs`
only: a cross-file type alias defined in `tests/support/mod.rs` evades it**
(`test_oracle` / low; precedent `ORD-HARD-136`; operator-verified at reassessment —
evidence corrected: the audit slice's same-file `as Env` claim was wrong, since the
total token ban catches the alias import line itself). The ban is a total
`EventEnvelope` token ban scoped to the single path `support/generative.rs`;
defining `pub type Env = EventEnvelope;` in `tests/support/mod.rs` (unscanned) and
constructing via `Env::…` in `generative.rs` leaves no banned token in the scanned
file. Correction: extend the token ban to every `tests/support/*` source (`mod.rs`
included), or resolve cross-file aliases before scanning. Lock: a synthetic with the
alias defined in `mod.rs` and consumed in `generative.rs` must fail.

**ORD-HARD-161 — `selection_rank` diverges from 0005 §8.3's recommended priority
order, unpinned and unrecorded** (agent cognition / low; no clean precedent —
doc-code coherence; operator-verified). `UrgentHungerOrFatigue` (3) and
`RoutineWindowDuty` (4) outrank `ActiveIntentionContinuation` (5), where 0005 §8.3
recommends active-intention continuation above urgent need and routine duty. The §8.3
hard floor (active intention beats *mild* pressure) holds, and
`docs/0-foundation/05` sanctions hunger/fatigue interruptions, so behavior is
constitution-defensible — but no test pins active-vs-urgent ordering and the
divergence from the spec's recommendation is recorded nowhere. Correction: an owner
decision — restore §8.3's order or record the chosen order (in the conformance index
or a 0005 outcome note). Lock: a `selection_rank` table snapshot test plus an
`urgent_need_vs_active_intention_follows_documented_order` behavioral test, so any
future re-rank is a visible intentional edit.

**ORD-HARD-162 — `default_day_windows` ranges share boundary ticks** (scheduler /
low; no precedent — latent footgun; operator-verified at reassessment).
`(0,4),(4,10),…` with
inclusive-inclusive `contains_tick` puts ticks 4/10/18/24 in two windows; today's
consumers are idempotent so no double-charge occurs. Correction: half-open or
offset-by-one windows. Lock: a `default_day_windows_are_disjoint_and_cover` unit
test.

**ORD-HARD-163 — `EatFailed` is absent from the capstone required-event list**
(test coverage / low; operator-verified at reassessment). The canonical
food-unavailable proof lives in
`food_unavailable_replan_001`, satisfying the contract, but the capstone that bills
itself as the end-to-end gate does not witness the chain 0005 §10.4 places inside
`no_human_day_001`. Correction + lock: add `EventKind::EatFailed` to
`assert_required_acceptance_events`.

**ORD-HARD-164 — Unspecified actor needs silently default to 100 with
`FixtureInitial` provenance** (content schema / low; precedent: the lineage's
no-silent-schema-defaults rule; operator-verified at `NeedState::initial(kind, 100,
NeedChangeCause::FixtureInitial)` in `schema.rs::to_agent_state`). A
decision-relevant cognition input is born from a magic literal carrying authored
provenance no author wrote. Correction: require all three needs per actor (typed
rejection `missing_actor_need_seed`) or source the default from an authored
`need_model` baseline field. Lock: a validation negative for the missing-need
fixture.

**ORD-HARD-165 — The serialization round-trip check is a self-check: symmetric
encoder/decoder bugs round-trip cleanly** (content serialization / low;
operator-verified at reassessment).
`deserialize(serialize(x)) == canonicalize(x)` uses the same codec both ways.
Correction: frozen golden-bytes snapshots per fixture family compared against
`serialize_fixture` output, independent of the decoder. Lock: the committed golden
bytes themselves (a codec change that reprices bytes is a visible diff).

## 5. Anti-contamination lock layer (consolidated)

Tiers extend the 0016–0023 layer.

1. **Compile-time impossibility:** `build_embodied_view_model` and
   `from_sealed_context` no longer name `PhysicalState` (`ORD-HARD-143`/`154`).
2. **Meta-lock repairs (the witness rule made real everywhere):** no presence-check
   default arm — measured counts or executed negatives per entry (`ORD-HARD-141`);
   inspected-site witnesses for emptiness-asserting scans (`ORD-HARD-146`);
   write/consume-shaped deferral cites (`ORD-HARD-142`); genesis-anchored baseline
   chain (`ORD-HARD-145`); validated census exemptions (`ORD-HARD-155`). Per the
   rotten-green research: every repaired meta-check gains a fixture negative that
   must fire through the production scan path.
3. **Runtime/product gates:** the fixture schema-version gate on the production load
   path (`ORD-HARD-140`); the raw-line scan in `load_fixture_package`
   (`ORD-HARD-150`); ID-field shortcut scanning (`ORD-HARD-151`); required need
   seeds or authored defaults (`ORD-HARD-164`); gated debug command surface and the
   operator-command quarantine with the staging decision recorded
   (`ORD-HARD-152`/`153`).
4. **Test-oracle corrections:** derived apply-perimeter token census with registry
   enrollment (`ORD-HARD-144`); closed-world in-`context()` discrimination
   (`ORD-HARD-148`); provenance-aware perception scan with a kill set
   (`ORD-HARD-149`); workplace accessibility gating + fourth row mutation
   (`ORD-HARD-147`); real-classifier policy oracle (`ORD-HARD-156`); widened render
   reachability (`ORD-HARD-157`); log-discrimination Mara negative (`ORD-HARD-158`);
   per-action positive census (`ORD-HARD-159`); alias-aware envelope ban
   (`ORD-HARD-160`); selection-rank pin (`ORD-HARD-161`); disjoint windows
   (`ORD-HARD-162`); capstone `EatFailed` (`ORD-HARD-163`).
5. **Evidence honesty:** golden-bytes serialization snapshots (`ORD-HARD-165`); the
   0024 acceptance artifact runs the checklist parity guard against itself; a
   finding premise that fails at implementation time is recorded as refuted.

## 6. Documentation corrections (housekeeping, same package)

- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`: extend R-29 with the two shapes this
  pass surfaced — the *presence-check fallback arm* (an enumerated witness repair
  whose default arm degrades to name-presence: `ORD-HARD-141`) and the
  *self-satisfying citation* (a content witness pointed at the definition site of the
  thing it witnesses: `ORD-HARD-142`). Extend R-28 with a Watch note: type-convention
  censuses (`String`-typed field scans, single-file `include_str!` perimeters,
  hand-picked token subsets of a public API) are hand-maintained for everything
  outside the convention (`ORD-HARD-144`/`151`/`157`).
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`: record
  the operator-proof classification of `run no-human-day`, the debug-gating policy
  decision, and the staging status of player-facing time acceleration with
  interruption stops and missed-event summaries (`ORD-HARD-152`/`153`).
- Conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`):
  add/update rows for the fixture schema-version gate (`ORD-HARD-140`), the
  completed embodied truth-access removal (`ORD-HARD-143`), the derived apply
  perimeter (`ORD-HARD-144`), and the meta-witness completion (`ORD-HARD-141`) —
  landing with the capstone artifact, citing only landed symbols (the 0023
  precedent).
- If `ORD-HARD-161` resolves as "keep the current order": record the decided
  candidate-priority order where the conformance row for agent cognition lives, so
  the 0005 §8.3 divergence is an owned decision rather than drift.
- No doctrine amendment; INV-001…INV-110 are applied, not changed. The INV-087
  decision (`ORD-HARD-095`) remains deferred; nothing in this spec pre-decides it.

## 7. Acceptance artifact

A new report under `reports/` (e.g. `reports/0024_ord_life_cert_scoped_acceptance.md`)
recording, for the implementation commits:

1. The fixture schema-version gate with its firing negative through
   `load_fixture_package`, and the per-version golden fixture (`ORD-HARD-140`).
2. The meta-witness completion: no presence-check default arm, per-entry measured
   witnesses or executed negatives, with the anchor-miss synthetic firing against a
   formerly-default-arm scan (`ORD-HARD-141`/`146`); the write-shaped deferral cite
   with its orphaned-definition synthetic (`ORD-HARD-142`); the genesis-anchored
   chain with its fabricated-ledger synthetic (`ORD-HARD-145`); validated census
   exemptions (`ORD-HARD-155`).
3. The compile-time truth-access removal diff and the staleness positive for the
   place label (`ORD-HARD-143`/`154`).
4. The derived apply-perimeter census, its registry enrollment, and the
   non-allowlisted-injection synthetic (`ORD-HARD-144`).
5. The production raw-line scan wiring and the prose-born-fact load negative
   (`ORD-HARD-150`); ID-field shortcut scanning with its negative fixture
   (`ORD-HARD-151`); the need-seed decision and its negative (`ORD-HARD-164`);
   golden serialization bytes (`ORD-HARD-165`).
6. The TUI quarantine decisions as recorded in `docs/2-execution/07`, the gated
   command surfaces, and their refusal tests (`ORD-HARD-152`/`153`).
7. The policy/projection closures with firing row mutations
   (`ORD-HARD-147`/`156`), the closed-world discrimination negative
   (`ORD-HARD-148`), and the perception-scan kill set (`ORD-HARD-149`).
8. The oracle closures: widened render reachability, Mara log-discrimination,
   per-action positive census, alias-aware envelope ban
   (`ORD-HARD-157`/`158`/`159`/`160`).
9. The 0005 coherence decisions: the recorded candidate-priority order and its pin
   (`ORD-HARD-161`), disjoint windows (`ORD-HARD-162`), capstone `EatFailed`
   (`ORD-HARD-163`).
10. Confirmation that every finding's premise still held at implementation time
    (all twenty-six findings were operator-verified pre-implementation — sixteen at
    audit, ten re-verified at source during spec reassessment at the same baseline);
    a premise that nonetheless fails is recorded as refuted, not silently dropped.
11. The risk-register, execution-doc, and conformance-index diffs, quoted (§6).
12. An updated `EMERGE-OBS` ledger derivation over the corrected surface
    (measurement only, no thresholds).
13. The scheduled mutation run's result under the post-0022 posture if it has fired
    by then (closing the 0022/0023 "pending" status), or its still-pending status
    restated honestly.
14. Explicit non-certification statement: scoped evidence toward `ORD-LIFE-CERT`;
    not full-project certification, not Phase 4 entry, not `FIRST-PROOF-CERT`.

## 8. Implementation constraints

- No backwards-compatibility shims or alias paths; no silent schema defaults.
- No doctrine amendment.
- Crate direction preserved: core depends on nothing at runtime; content on core;
  tui on core + content. No new dependencies, dev or production.
- Verification posture: all twenty-six findings are operator-verified at their
  load-bearing symbols — `ORD-HARD-140`–`145`, `147`, `148`, `150`–`153`, `158`,
  `159`, `161`, and `164` during the audit; `ORD-HARD-146`, `149`, `154`–`157`,
  `160`, `162`, `163`, and `165` re-verified at source during spec reassessment at
  the same baseline (nine confirmed, `ORD-HARD-160` evidence-corrected to the
  cross-file alias shape) — so no re-verification step is owed; a finding whose
  premise nonetheless fails at implementation time is recorded as refuted in the
  acceptance artifact, not silently dropped.
- `ORD-HARD-143` ordering note: land the preflight/sealed-source refactor before
  removing the `&PhysicalState` parameter, so the compile-time lock lands against an
  already-correct builder; re-derive any repriced goldens once, honestly, in the same
  ticket group.
- `ORD-HARD-152`/`153` require recorded owner decisions (operator-vs-player
  classification; debug-gating policy) — make those decisions in the ticket, record
  them in `docs/2-execution/07`, and implement accordingly; do not silently choose.
- Recommended ticket ordering:
  1. `ORD-HARD-141` + `146` + `142` + `145` + `155` (meta-lock witness completion —
     first, so every subsequent ticket's new locks are born under repaired
     meta-rules) + §6 risk-register corrections.
  2. `ORD-HARD-140` + `150` + `151` + `164` + `165` (content-loader group; the
     foundation violation and its siblings; batch fixture/golden repricing once).
  3. `ORD-HARD-143` + `154` (embodied truth-access completion; compile-time lock).
  4. `ORD-HARD-144` (apply-perimeter derivation + enrollment).
  5. `ORD-HARD-152` + `153` + `157` (TUI quarantine decisions + render-guard
     widening) + the §6 execution-doc record.
  6. `ORD-HARD-147` + `156` (policy surface closure).
  7. `ORD-HARD-148` + `149` (epistemic discrimination + perception provenance).
  8. `ORD-HARD-158` + `159` + `160` (oracle closures).
  9. `ORD-HARD-161` + `162` + `163` (0005 coherence decisions and pins).
  10. The acceptance artifact lands last, measuring the corrected surface.
- All four gates pass before any ticket is marked complete:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

## 9. Risks and open questions

- **`ORD-HARD-143`'s preflight refactor touches the validator boundary.** The
  preflight lawfully needs truth (INV-099: truth may validate); the correction must
  relocate, not weaken, that check. If the sealed-boundary refactor proves to need a
  Phase-3B-scale redesign, the honest fallback is the `ValidatorOnlyState` newtype
  quarantine plus the widened `guard_014`, with the full parameter removal ticketed
  against an execution-doc entry — not a silent re-scope.
- **`ORD-HARD-141`'s completion may surface latent vacuous scans.** Once every
  registry entry carries a measured witness, scans that have been matching zero
  sites will fail honestly — treat each as a potential masked defect first, per the
  Enforcement reading (the 0023 §9 warning, still live).
- **`ORD-HARD-140`'s gate may reveal version drift in committed fixtures.** If any
  in-repo fixture declares a version other than the canonical one, the gate will
  fail honestly at first run; reprice the fixture, never widen the set silently.
- **Two findings require owner decisions** (`ORD-HARD-152`/`153` classification and
  gating policy; `ORD-HARD-161` priority order). The spec specifies the recommended
  resolution for each but they are decisions, not mechanical fixes.
- **Next-iteration assessment (the recurring question):** a product-behavior
  foundation violation was found (`ORD-HARD-140`), so a fourteenth pass is warranted
  by the lineage's own rule, and it should be a **strictly scoped verification of
  0024's deliverables** — above all whether the meta-witness completion
  (`ORD-HARD-141`) eliminated the presence-check arm for real, and whether the
  schema-version gate sits on the production load path. The honest trend, stated
  plainly: the cognition firewall is clean seven passes running; the 0005 feature
  contract held with zero product regressions; 0023's product fix (the locality
  migration) verified substantive at the display surface on first inspection; and —
  for the second consecutive pass — the only product foundation violation came from
  a *blind sweep of a never-scoped surface*, exactly as 0023 §9 predicted. Both
  named never-scoped surfaces have now been swept once. Per the standing research
  corpus (defect-discovery curves, event-triggered audit practice, the Tricorder
  graduation criterion), if the fourteenth pass verifies 0024 clean-or-low-only,
  the defensible cadence is the one 0023 already recommended: drop per-pass audits,
  move to phase-entry audits (Phase 3B / Phase 4 boundaries), keep the four gates +
  mutation CI + completed meta-locks as the continuous layer, and schedule
  guard-consolidation hygiene at the same boundaries. The fourteenth pass should
  include one final blind sweep of the remaining never-scoped corners (the
  serialization/manifest layer end-to-end, `controller.rs`/`debug_capability.rs`,
  and the CI workflow files as a product surface) — the two passes of evidence now
  say the residual risk lives between audit boundaries, and the supply of unaudited
  boundaries is nearly exhausted.

## 10. Self-check

- [x] Determination is positive; spec authored under the produce-only-if-positive
  rule.
- [x] Every finding cites INV numbers from the local
  `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` (or is explicitly
  lock-durability / evidence-honesty / doc-code coherence) and names responsible
  layers from the execution diagnostic vocabulary.
- [x] Every finding cites local sources by named symbol (grep-stable; line numbers
  deliberately omitted); every finding is operator-verified — sixteen at audit, ten
  re-verified at source during spec reassessment (`ORD-HARD-160` evidence-corrected)
  — with the consolidated status in §8.
- [x] Every finding includes a structural lock that fails the build or test suite on
  regression; `ORD-HARD-143` carries a compile-time impossibility lock.
- [x] Verified-holding items from 0014–0023 are recorded to prevent re-litigation;
  the 0005 feature-contract verification is recorded; severity calibrations that
  diverge from lineage precedent or from the reporting audit slice are named in §4's
  preamble (`ORD-HARD-140`/`141`/`142`/`144`/`148`/`158`/`159`).
- [x] No doctrine amendment; no compatibility shims; no new dependencies; crate
  direction preserved. The INV-087 decision remains deferred, untouched.
- [x] Scope stays within the Phase 3A ordinary-life surface, its content-loader and
  TUI input boundaries (first-ever sweeps, prescribed by 0023 §9), and its
  lock/evidence layer.

## Outcome

Completed: 2026-06-12

- Implemented and archived the full `0024PHA3ACONSCH-*` ticket series, closing
  `ORD-HARD-140` through `ORD-HARD-165` across fixture schema-version gating,
  meta-lock live witnesses, embodied truth-access quarantine, derived apply
  perimeter coverage, content-loader validation, TUI debug quarantine,
  projection-policy locks, oracle closures, and 0005 coherence pins.
- Added `reports/0024_ord_life_cert_scoped_acceptance.md`, including all fourteen
  §7 evidence anchors, the `emerge_obs_v1` derivation, the honest pending
  mutation-run status, and the explicit non-certification statement.
- Added conformance-index rows for the 0024 schema-version gate, meta-witness
  completion, embodied truth-access removal, derived apply perimeter, and the
  recorded `ORD-HARD-161` cognition-priority decision.
- Added the 0024 acceptance-artifact parity guard and registered it in the
  meta-lock registry with a firing missing-anchor synthetic.

Deviations:

- The scheduled mutation run remains pending in the local evidence surface, as
  permitted by §7 item 13. No mutation result is claimed.
- This closeout does not certify `ORD-LIFE-CERT`, full-project readiness, Phase 4
  entry, or `FIRST-PROOF-CERT`.

Verification:

1. `cargo test -p tracewake-core --test emergence_ledger -- --nocapture`
2. `cargo test -p tracewake-core --test anti_regression_guards --test emergence_ledger`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
