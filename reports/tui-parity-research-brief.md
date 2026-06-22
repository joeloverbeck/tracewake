# Research brief — TUI/simulation feature-parity drift guard

*Paste this entire document into a fresh ChatGPT-Pro deep-research session, and upload the
manifest named in §1. You are the deep researcher (call you "Session 2"). Produce the
deliverable in §7 directly. **Do not interview, do not ask clarifying questions** — every
decision you need is fixed below.*

---

## 1. Context

The uploaded manifest (`reports/manifest_2026-06-22_1145e10.txt`) is the path inventory of the
`joeloverbeck/tracewake` repo — a causality-first living-world simulation in Rust: an
event-sourced kernel, subjective epistemics (agents act only from their own beliefs, which may
be stale, contradictory, or absent), fallible institutions, and a TUI-first playable surface.
The workspace is three crates with a strict one-way dependency direction: `tracewake-core`
(authoritative kernel, zero deps) → `tracewake-content` (fixtures, loading, schema) →
`tracewake-tui` (terminal client; possession, embodied/debug view models). Core must never
depend on content or tui.

Docs are layered authority — earlier tiers govern later ones:
`0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`. If execution
conflicts with architecture or foundation, execution is wrong; if implementation is more
convenient than the accepted gates, implementation is wrong. **Fetch every file from commit
`1145e109aa50721b37f7d343caef518b3be5fc7f` (short `1145e10`)** — the manifest reflects that
exact tree.

**This brief is the first research pass on this concern; there is no lineage predecessor.** It
descends from a short in-repo diagnostic note (`reports/tui-parity.md`) written in an earlier
session, which first surfaced the problem. That note is **untracked** (it is not in commit
`1145e10`, so it is not in the manifest) and is therefore **reproduced verbatim in Appendix A**
of this brief — read it there, do not look for it in the manifest.

**The motivating worry, in the user's words:** *"At any point I should be able to run the TUI
and find out that it runs the game with all the most recent features it should have. I don't
want to run the TUI and discover that the non-human players aren't actually thinking and acting
on their own, or that the epistemic layer doesn't actually work, etc."* The TUI today is
closer to a CLI than a finished game — no feature-forward spec has modernized it yet — and the
user expects future features (including, eventually, **domain-pack / "flavor-type" features**,
e.g. running the simulation as a post-apocalyptic world instead of a fantasy one) to keep
reaching the playable surface. The goal of your research: **figure out how to make it
structurally impossible for the TUI to silently fall out of step with the simulation's latest
features when it should have them.**

### Repo state you are reasoning against (verified at `1145e10`)

- The certification ladder is **exhausted and passed**:
  `P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT`, with
  `FIRST-PROOF-CERT passed` recorded for archived spec `0045` at its exact implementation
  commit. The live spec ledger (`docs/4-specs/SPEC_LEDGER.md`) declares **no** forward feature
  work; the genuine next moves are **Phase-4 entry** and **second-proof entry**, both
  *Expansion*-posture (not certification audit/remediation). Practical consequence for your
  research: the parity guarantee you design must be **standing** — it has to survive and govern
  *every* future feature spec, not be a one-off bolted onto the current baseline.
- The current confirmed defect (see Appendix A and the code seams in §2): the embodied
  renderer reads view-model fields **à la carte**, with no exhaustive destructure, no
  `#[non_exhaustive]`, no golden/snapshot acceptance test driven through the real pipeline, and
  no test that fails when a new core field or feature is added. A new `EmbodiedViewModel` field
  compiles and is silently ignored; a core feature that never reaches the view model is
  invisible with no field to forget. **The user's nightmare is live today.**

---

## 2. Read in full (authority order)

Read these before producing. The user explicitly asked that you read **all of**
`docs/0-foundation/*`, `docs/1-architecture/*`, `docs/2-execution/*`, `docs/3-reference/*`, and
`docs/4-specs/*`. Honor that — but to keep the high file count tractable, each tier below names
its **primary (load-bearing)** files with a reason; the remaining files in that tier are
**boundary-awareness reads** (read them to know what is *out* of scope and what doctrine lives
elsewhere, not as things to "correct").

**Universal (load-bearing for every finding):**
- `docs/README.md` — the authority order and the layering rule you must respect when routing each recommendation to a tier.
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — `INV-001…INV-112`; every recommendation must satisfy these, and you must not invent new `INV-###` identifiers.

**`docs/0-foundation/` — primary:**
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — the constitutional home of "kernel and TUI advance together," the embodied/debug split, the per-feature *Review rule*, and the *Automated acceptance* obligations. This is the most load-bearing doc for the whole concern.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — defines first-playable scope and the "feature is not done unless…" acceptance gates the parity guard must operationalize.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — the truth-firewall constitution; a parity guard must never create a path that lets the TUI read hidden truth to "prove" a feature is present.
- (Remaining `0-foundation/*` — boundary-awareness: read to run the tier-fit test of where each recommendation belongs and what the constitution must *not* absorb from a lower tier.)

**`docs/1-architecture/` — primary:**
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — the architecture contract that owns view-model shape, semantic-action entries, test-harness obligations, and the *Required diagnostics / replay / TUI hooks* section. Primary amendment candidate.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — owns validation/observability/acceptance-artifact architecture; a parity mechanism's evidence has to fit here.
- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — the conformance index a new structural obligation would be registered in.
- (Remaining `1-architecture/*` — boundary-awareness.)

**`docs/2-execution/` — primary:**
- `07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — owns execution acceptance for actor-known view models, possession parity, embodied affordances, why-not, debug quarantine; defines `EPI-CERT`. The seed note suggests parity guards could ride here.
- `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — owns per-surface rendering diagnostics, testing posture, and review artifacts; a mechanical parity guard's test obligations land here.
- `03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` — the canonical gate order; tells you whether a *standing* parity obligation fits the existing ladder or needs a new home.
- `02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` — the current baseline/acceptance contract you are extending, not replacing.
- `11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` — Phase-4 entry doctrine; shows the kind of future feature work the parity guard must survive.
- `12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md` — the deferred feature surfaces (notices, travel, LOD) that are exactly the "features that should reach the TUI later" the user worries about.
- `00_EXECUTION_INDEX_AND_AUTHORITY.md` — the execution authority/index and canonical gate-code list (so you do not mint a gate code that conflicts).
- (Remaining `2-execution/*` — boundary-awareness.)

**`docs/3-reference/` (small tier — read all three):**
- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — the review checklist a parity question could be added to.
- `01_DESIGN_RISK_REGISTER.md` — the risk register; "TUI silently lags the simulation" is a candidate relapse-risk to record (you recommend the substance; you do **not** mint a new `R-##` identifier).
- `02_GLOSSARY.md` — canonical terminology; if you name a mechanism, check the glossary for fit.

**`docs/4-specs/` (read all):**
- `SPEC_LEDGER.md` — the active spec set, the archived-spec verdicts, and the "Next known execution move" (read it for *what is already certified and what is explicitly not declared*; note that it declares no forward feature work).
- `README.md` — specs-tier authority posture and what a spec may/may not do.
- `0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` — the live first-proof ontology/fixture contract a parity scenario would exercise.
- `0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` — the live acceptance-artifact template; the shape any future parity evidence package would follow.

**Structural context (archived — read for evidence-package shape, not as live authority):**
- `archive/specs/0009_TUI_PROOF_SURFACE_HARDENING_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md` — the prior TUI proof-surface/debug-quarantine hardening; closest precedent for how this repo hardens the TUI seam.
- `archive/specs/0044_FIRST_PROOF_CERT_MISSING_PROPERTY_COHERENT_GATE_SET_TEMPORAL_BUNDLE_AND_INTEGRATED_ACCEPTANCE_CERTIFICATION_SPEC.md` and `archive/specs/0045_FIRST_PROOF_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md` — the terminal capstone gate; shows how the repo composes evidence packages a standing parity gate might extend.

**Code seams — read directly at `1145e10` (do not paste them into the report; cite file + symbol):**
- `crates/tracewake-core/src/view_models.rs` — defines `EmbodiedViewModel` (≈22 `pub` fields), `DebugViewModel` (enum, 10 variants behind a sealed `DebugCapability`), and the kind enums (`SemanticActionEntry`, `ActionAvailability`, `WhyNotFailureKind`, `VisibleItemSource`, notebook/lead views). This is the surface a new feature adds a field/variant to.
- `crates/tracewake-core/src/projections.rs` — `build_embodied_view_model(...)` and `build_notebook_view(...)`: where core simulation state (actors, beliefs, needs, affordances, holder-known context) is projected into the view model. **Hop 1** lives here: a core feature that this projection never reads is invisible with no field to forget.
- `crates/tracewake-tui/src/render.rs` — `render_embodied_view(&EmbodiedViewModel) -> String` (pure function, no terminal needed) plus `render_debug_overlay`, `render_notebook`, `render_rejection`. **Confirmed: reads fields à la carte, no exhaustive `let EmbodiedViewModel { .. } = view` destructure.** This is **Hop 2**.
- `crates/tracewake-tui/src/app.rs`, `crates/tracewake-tui/src/debug_panels.rs` — the app/render glue and debug panel construction.
- `crates/tracewake-tui/tests/tui_seam_conformance.rs`, `embodied_flow.rs`, `tui_acceptance.rs`, `adversarial_gates.rs`, `transcript_snapshot.rs` — the existing parity/conformance guards. Note especially: snapshot tests use **byte-for-byte assertions, not `insta`**; static source guards forbid the TUI calling `apply_event` etc.; **no test fails when a new view-model field is added.**
- `crates/tracewake-content/src/fixtures/` — Rust-sourced `GoldenFixture` scenarios (e.g. `view_model_local_actions_001`, `no_human_day_001`, `sleep_eat_work_001`, `embodied_workplace_believed_open_truth_closed_commit_fails_001`) that drive the TUI tests; the substrate any "kitchen-sink" parity scenario would build on.

---

## 3. Settled intentions (final — do not re-open these)

These were resolved with the repo owner before this brief was written. Treat them as fixed
requirements, not options.

1. **The deliverable is a single recommendation report, at substance-and-home altitude.** It
   diagnoses the problem, presents the external research, designs the mechanism, names the
   `docs/**` amendment targets (for each finding: *what doctrine the target doc must own*, in
   your own prose at the right altitude for that tier, and *which file* it lands in), and
   **sketches/recommends** a future implementation spec. It does **not** produce final
   paste-ready doc wording, does **not** produce a numbered `specs/NNNN_*.md` spec, and does
   **not** invent new identifiers (`INV-###`, gate codes, `R-##` risk IDs). Authoring final
   doc text and the spec remain the repo's own reassess/amend and spec processes — your job is
   to tell them precisely *what* to change and *where*, with evidence.
2. **Domain packs / flavor-types are forward-compatibility only.** Design the mechanism for the
   current base simulation. It must **not preclude** later domain-pack / flavor-type feature
   sets (e.g. post-apocalyptic vs. fantasy), and you should state how the design generalizes to
   per-pack feature coverage — but building out per-pack parity now is **not** required.
   Per-pack parity is a noted future extension.
3. **TUI modernization is out of scope.** Do **not** roadmap turning the CLI-ish surface into a
   richer game. Your mechanism must *survive and govern* that future feature-forward TUI work,
   but the modernization itself is a separate future spec you are not designing.
4. **The enforcement posture is your call to recommend, not pre-decided.** Research prior art
   and recommend how the standing guarantee should be enforced — a new standing TUI-parity
   gate that every future feature spec must satisfy, **vs.** folding the obligation into the
   existing `EPI-CERT` / `FIRST-PROOF-CERT` evidence packages, **vs.** a layered mechanism —
   and justify the choice against this repo's gate/authority structure. Do not pre-commit to
   one in your framing; let the evidence decide.
5. **The seed's two-hop decomposition is the analytical spine** (see Appendix A). *Hop 2*
   (view model → rendered text) is cheaply enforceable **at compile time** — exhaustive
   destructure with no `..` rest pattern, plus exhaustive `match` (no `_` arm) on the
   action/affordance kind enums — so a new field or variant becomes a build error until
   consciously handled. *Hop 1* (core feature → view model) cannot be caught by a field guard,
   because a blind projection has no field to forget; it needs **scenario-driven
   golden/snapshot acceptance tests** that run a fixture through the *real* pipeline and assert
   the feature is visible in the rendered surface. Treat both hops as first-class; do not
   collapse the problem to just the compile-time half.
6. **The guarantee must be standing.** Because the cert ladder is passed and the next moves are
   Expansion-posture (Phase-4 / second-proof entry), the mechanism's value is that it binds
   *future* feature specs. Frame every recommendation for permanence, not for the current
   baseline alone.
7. `assumption:` the report's filename is `reports/tui-parity-research-report.md` (see §7). If
   the owner prefers another slug, that is a trivial rename — proceed with this name.

---

## 4. The task

This is a **hardening / anti-contamination** research task with a **doc-overhaul** secondary.
Produce an evidence-based recommendation report that answers one question: **how does Tracewake
make it structurally impossible for the TUI surface to silently lag the simulation's
features?** Diagnose the current two-hop drift exposure against the actual code and doctrine;
research how comparable systems (and the relevant CS/PL literature and engineering practice)
prevent UI-vs-model drift; design concrete, repo-idiomatic mechanisms (compile-time
exhaustiveness for Hop 2; pipeline-driven golden/feature-coverage acceptance tests for Hop 1;
and whatever registry/conformance layer ties "new core capability" to "rendering + test
obligation"); and route every recommendation to its doctrine home and to a sketched future
implementation spec — without writing final doc text or the spec itself.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — follow the
view-model/projection/render seam wherever it leads, and inspect the existing tests closely so
your mechanism composes with them rather than duplicating them.

Research online as deeply as needed — similar implementations, research papers, prior art,
engineering practice — wherever it sharpens the deliverable. Useful directions (not
exhaustive): Rust exhaustiveness patterns (non-`..` struct destructuring, non-`_` enum matches,
`#[non_exhaustive]` semantics across crate boundaries, `#[deny(unused_variables)]`/clippy as a
forced-decision lever); golden/snapshot testing for view layers (e.g. `insta` and equivalents)
and the trade-offs vs. byte-stable assertions; **capability/feature registries** and
data-driven UI generation that make "a feature exists ⇒ it has a surface + a test" a checkable
invariant; ECS/MVU/Elm-style "single source of truth → view" architectures and how they detect
unrendered state; consumer-driven contract testing and conformance suites; coverage techniques
that tie enum/variant growth to a rendering obligation; and how moddable/data-driven games keep
flavor packs' content reachable in their UI. **Cite every external source that shapes a
recommendation.**

---

## 6. Doctrine & constraints

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution: every recommendation
  must satisfy it; a genuine divergence would require amending an invariant *first*, never
  designing against it silently. You recommend amendments at substance altitude — you do not
  mint `INV-###` numbers.
- **Authority order is downward.** Route each recommendation to the *highest* tier that
  legitimately owns it and no higher: the constitution states principle, architecture states
  contract, execution states proof mechanics and gates, reference states terminology/risk,
  specs operationalize. If a guard is a test mechanic, its home is execution `10`, not the
  constitution.
- **No backwards-compatibility shims or alias paths** in any recommended new work.
- **Anti-contamination is non-negotiable.** No simulation fact may be born from prose; preserve
  event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible
  institutions, and validation/replay. Critically: a parity guard must **never** create a path
  where the TUI reads hidden/ground truth (or debug truth) to demonstrate a feature is
  "present." A golden parity scenario proves a feature reaches the *actor-filtered* surface
  through the real pipeline — it must not become a wallhack. The embodied/debug split in
  foundation `08` and execution `07` is a hard boundary your mechanism must respect.
- The TUI may be friendly; it may not be metaphysically privileged. It must not apply events,
  implement action rules, or mutate state through player-only paths.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **`reports/tui-parity-research-report.md`** — **new** (not a replacement; confirm no live
  `reports/tui-parity-research-report.md` exists — the untracked `reports/tui-parity.md` is the
  *seed note*, reproduced in Appendix A, a different file).

This is an **analysis / recommendation report**, *not* a numbered spec — so the spec
numbering/ledger/epoch rules do **not** apply, and you author **substance + home, not ratified
text**. Use this structure (the shape this repo's cascade/recommendation reports reuse):

1. **Executive summary** — the drift exposure in two sentences, the two-hop framing, and your
   single headline recommendation set.
2. **Disposition table** — one row per finding: *finding → target doc (or "code/test only,"
   or "future spec") → verdict (amend / add / record / route-forward / no-change) → one-line
   basis*. This is the at-a-glance map.
3. **Method & provenance ledger** — what you read in-repo, what you searched online, and how
   you verified each external claim.
4. **Diagnosis (current state)** — Hop 1 and Hop 2 exposure proven against the actual seams in
   §2 (cite `file:symbol`). Include what the existing tests *do* and *do not* guard, so you do
   not re-recommend coverage that already exists.
5. **External research synthesis** — the prior art and literature, with citations, and what
   each implies for Tracewake.
6. **Mechanism design** — the concrete, repo-idiomatic guards, layered cheapest-first:
   (a) Hop-2 compile-time exhaustiveness (destructure + enum matches); (b) Hop-1
   pipeline-driven golden/feature-coverage acceptance tests; (c) any registry/conformance layer
   binding "new core capability ⇒ rendering + test obligation"; and (d) how the design
   generalizes to domain packs as a *forward-compat* property (per §3.2). For each guard:
   what it catches, what it cannot, its failure mode, and how it composes with the existing
   tests.
7. **Enforcement-posture recommendation** — standing parity gate vs. folding into existing
   evidence packages vs. layered, **with your reasoned choice** and how it slots into the gate
   order and authority structure (per §3.4).
8. **Per-finding sections** — for each finding: *driver → current coverage → tier-fit verdict
   → recommendation* (what doctrine the target doc must own, in your prose at that tier's
   altitude, and which file). No paste-ready wording; no invented identifiers.
9. **Sketched future implementation spec** — a recommendation-level outline of the spec that
   would implement the guards (scope, the guards it lands, the acceptance evidence it would
   carry, where it sits relative to Phase-4/second-proof entry) — an outline, **not** a
   numbered spec file.
10. **Forward-routing appendix** — because this is a cross-cutting hardening target rather than
    a single-tier cascade, route items that belong to owner/reassess decisions or to future
    implementation specs (e.g. the per-pack parity extension, the possession-bind perception
    owner decision if it intersects). State plainly that no finding routes to a *lower doc
    tier* than already covered — the legitimate out-routes are owner/reassess decisions and
    future implementation specs.
11. **Open questions** — anything genuinely undecidable from the repo + research.
12. **References** — every external source cited.

**Locked / no-questions instruction:**

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do
> not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it in the deliverable and proceed with the most
> faithful interpretation.

---

## 8. Self-check (run before returning)

- The deliverable is exactly one new file, `reports/tui-parity-research-report.md`, at
  substance-and-home altitude — no paste-ready doc wording, no numbered spec, no invented
  `INV-###` / gate / `R-##` identifiers.
- Both hops are addressed as first-class; the report does not collapse to the compile-time half.
- Every recommendation is routed to the *highest tier that legitimately owns it and no higher*,
  consistent with the downward authority order.
- No recommended guard creates a path for the TUI to read hidden/debug/ground truth to "prove"
  a feature — the embodied/debug firewall (foundation `08`, `14`; execution `07`) is intact.
- Domain-pack coverage is treated as forward-compat, not built out; TUI modernization is not
  roadmapped.
- The enforcement-posture choice is made and justified, not left open.
- Every external claim that shapes a recommendation is cited.
- Every file named in §2 exists at commit `1145e10` (the manifest reflects that tree); the seed
  note used is the one reproduced in Appendix A.

---

## Appendix A — seed note (`reports/tui-parity.md`, reproduced verbatim; untracked, not in the manifest)

> What I found
>
> The seam is clean: tracewake-core builds an EmbodiedViewModel, and tracewake-tui's render_embodied_view(view: &EmbodiedViewModel) -> String
> turns it into text. The renderer is a pure function returning a String — no terminal needed to test it. That's a big asset; parity can be
> asserted as plain string checks.
>
> But render_embodied_view reads fields à la carte (view.visible_exits, view.carried_items, view.semantic_actions, …). It never destructures
> the struct. So:
>
> ▎ If you add a field to EmbodiedViewModel, the renderer compiles cleanly and silently ignores it. That is exactly your nightmare, and it's
> ▎ live today.
>
> The render tests reinforce this: they're hand-written, one per field (renderer_prints_semantic_action_ids,
> renderer_prints_door_endpoints_and_item_sources, …). Good coverage of what exists, but a new field gets a test only if someone remembers.
> Nothing forces it.
>
> There are actually two hops to keep in parity
>
> Your worry decomposes, and the two halves need different guards:
>
> Hop 2 — view model → rendered text. This one is cheaply enforceable at compile time. Make the renderer destructure exhaustively with no ..
> rest pattern:
>
> pub fn render_embodied_view(view: &EmbodiedViewModel) -> String {
>     let EmbodiedViewModel {
>         view_model_id, mode, viewer_actor_id, sim_tick, place_id, place_label,
>         visible_exits, visible_doors, visible_containers, visible_items,
>         carried_items, local_actors, semantic_actions, /* … */ debug_available,
>     } = view;
>     // …render using the locals…
> }
>
> Now the moment a new field lands, -D warnings (you already run clippy that way in CI) fires unused_variables and the build goes red until
> someone consciously either renders it or binds it to _field with a comment saying why it's intentionally not shown. It doesn't prove the
> field renders well — but it converts "forgot the surface exists" into a forced, reviewable decision at the exact moment of divergence. This
> is the single highest-leverage change, and it's small.
>
> Pair it with exhaustive match (no _ arm) on the action/affordance enums (SemanticActionEntry kinds, WhyNotFailureKind, etc.). Then every new
> action variant the simulation can produce is a compile error in the renderer until it's handled. Same mechanism, applied to "new kinds"
> rather than "new fields."
>
> Hop 1 — core feature → view model. This is the deeper version of your fear, and it's the one that'll bite when Phase 4 lands. A field-level
> guard can't catch "institutions exist in core but the projection that builds EmbodiedViewModel never looks at them" — the view model is
> simply blind, so there's no field to forget. The guard here is a scenario-driven golden/snapshot test: run a fixture through the real
> pipeline that exercises the feature, render the embodied surface, and assert the feature is visible in the output. You already have the
> ingredients — embodied_flow.rs, app.render_current_view(), and a view_model_local_actions_001 fixture. A snapshot tool like insta over a
> "kitchen-sink" scenario makes the diff impossible to ignore: when a feature changes the world but not the surface, the reviewer sees it.
>
> Recommendation
>
> Layer them, cheapest first:
>
> 1. Exhaustive destructure + exhaustive match in the renderer — compile-time parity for hop 2. Do this now; it's nearly free and permanent.
> 2. One golden snapshot acceptance test per major feature scenario, driven through the real pipeline — covers hop 1, where Phase
> 4/notices/institutions would otherwise vanish.
> 3. Optionally, a tiny coverage test that enumerates the affordance/feature enum and asserts each renders non-empty — ties "new variant" to
> "rendering obligation" explicitly.
>
> This also isn't ad-hoc relative to your doctrine — it operationalizes the existing VIEW-DEBUG-SPLIT / EMBODIED-TEMPORAL view-model gates
> (execution docs 07 and 10). So it could even ride in as a small Certification-posture addition to the ORD-LIFE/FIRST-PROOF evidence package
> rather than a one-off, which keeps it consistent with how the repo governs everything else.
