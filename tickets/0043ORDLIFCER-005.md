# 0043ORDLIFCER-005: Live ORD-LIFE-01…12 re-proof capstone and replacement acceptance artifact superseding 0042

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — acceptance-only capstone; re-runs existing suites/fixtures at the frozen commit and authors the replacement acceptance artifact. No new production logic.
**Deps**: 0043ORDLIFCER-004, 0043ORDLIFCER-006, 0043ORDLIFCER-007

## Problem

After remediation is frozen and the configured campaign has completed and reconciled, the ORD-LIFE-CERT line still reads `scoped remediation` until a complete replacement acceptance artifact re-proves the full 0042 seam contract at one exact final commit and explicitly supersedes the 0042 artifact. This capstone renders that verdict: it re-proves `ORD-LIFE-01` through `ORD-LIFE-12`, the ten live pass conditions, the seven mandatory fixture families, and the cross-cutting families at the frozen commit; instantiates the acceptance-artifact template fields; carries the completed-mutation pass row; keeps `EMERGE-OBS` observer-only; and supersedes the 0042 acceptance artifact only if every aggregate condition passes.

(spec §1.1, §7, §8, §8.10, §8.11)

## Assumption Reassessment (2026-06-20)

1. The upstream evidence exists via Deps: the corrected CI posture (-001), the three seed kills (-002), the completion mechanism (-003), the completed campaign + triage register + completion manifest (-004), and the completed-run survivor kill tickets (-006 movement, -007 wait). The acceptance-artifact template `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md` exists and defines the evidence-item fields (evidence-status, fingerprint-scope, behavior-witness, replay/provenance, sampling/exhaustiveness, pending/historical, certification-use, staged-abstraction). The 0042 artifact `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` and its triage register exist and are the supersession targets. Verified by `test -f` this session.
2. `ORD-LIFE-01` through `ORD-LIFE-12` are all defined in `archive/specs/0042_ORD_LIFE_CERT_NEEDS_ROUTINES_INTENTIONS_NO_HUMAN_LIFE_PLANNER_TRACES_AND_STUCK_DIAGNOSTICS_CERTIFICATION_SPEC.md` (verified by grep this session); the 0042 spec controls the full audit details and this capstone is the remediation re-proof routing summary, not a replacement of those requirements. The seven mandatory fixture families and the cross-cutting families (spec §7.4, §7.5) all resolve to fixtures under `crates/tracewake-content/src/fixtures/` (sampled and verified this session, e.g. `no_human_day_001`, `food_unavailable_replan_001`, `possession_does_not_reset_intention_001`, `planner_trace_001`, `routine_blocked_diagnostic_001`, `sleep_spanning_window_boundary_charges_each_tick_once_001`).
3. Cross-artifact boundary under audit: the replacement acceptance artifact's per-seam verdict matrix (positive / adversarial-negative / replay-provenance / mutation evidence per ORD-LIFE row), the completed-mutation pass row (denominator + transport + outcome counts + disposition counts + member evidence), the `EMERGE-OBS` observer-only member, the staged-abstraction declaration, and the supersession + ledger-handling contract (spec §8.5–§8.11). The artifact renders `ORD-LIFE-CERT passed` only if every §8.10 condition holds, else remains `ORD-LIFE-CERT scoped remediation` with named responsible layer and follow-up.
4. Invariant/rule motivation: INV-098 (a runnable feature is done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, regression-tested); INV-004/091 (no-human run), INV-018/092 (deterministic replay), INV-093 (no actor-knowledge leakage), INV-095 (TUI/view-model acceptance), INV-111 (living-world acceptance requires observer-only emergence evidence that feeds no behavior, sets no objective). The capstone amends no invariant and re-proves all (spec §10, reviewing every final delta against INV-001…INV-112).
5. Enforcement surface (evidence-consumer basis): the capstone re-proves — does not modify — the deterministic-replay, single-charge-accounting, actor-knowledge/truth-firewall, no-direct-dispatch, possession-continuity, and debug-quarantine surfaces. It must keep `EMERGE-OBS` strictly observer-only (Evidence status: observer-only; no pass/fail, no mutation disposition, no score/objective), keep debug rows non-diegetic, and label pending/sampled/observer-only/historical evidence honestly so none aggregates as pass. SPINE/EPI seams appear only as ancestry, not re-audited.

## Architecture Check

1. Folding the §7 multi-seam re-proof into one acceptance-only capstone (per the mutation-remediation capstone shape) is cleaner than scattering verdict fragments across the implementation tickets: the verdict is gated on the complete evidence set existing coherently at one frozen commit, which only a trailing capstone can assert. Counts are re-enumerated from fixtures/the completion manifest at evaluation time, not hardcoded.
2. No backwards-compatibility aliasing/shims and no new production logic — the capstone exercises and records; it does not modify the surfaces it re-proves. It does not promote historical/sampled/observer-only/partial evidence into a live pass, and does not re-audit SPINE/EPI.

## Verification Layers

1. ORD-LIFE-01…12 live re-proof (INV-098) → replay/golden-fixture + core/content/tui gate checks: each seam row carries positive, adversarial/negative, replay/provenance, and mutation evidence at the frozen commit (spec §7.2, §8.5).
2. Ten pass conditions + seven fixture families + cross-cutting families (INV-004/091, INV-093, INV-095) → fixture execution: each family is registry-reachable and executes live; no-human day advances, hidden-truth non-interference holds, TUI gates pass, fail-only-empty-food recovery preserved.
3. Deterministic replay (INV-018/092) → replay check: empty-projection rebuild of accounting/metrics/diagnostics equals live; checksums/fingerprints recorded.
4. Completed-mutation pass row → register/manifest reconciliation: denominator + transport + outcome counts + disposition counts reconcile to -004's campaign; zero blocked/untriaged, zero unsigned exceptions, zero unresolved timeouts/tool failures.
5. EMERGE-OBS observer-only (INV-111) + evidence honesty → manual review: `EMERGE-OBS` member carries `Evidence status: observer-only` and contributes to no pass/fail; pending/sampled/historical labels never aggregate as pass.

## What to Change

### 1. Author the replacement acceptance artifact

Create `reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md` instantiating every `docs/4-specs/0003` field (spec §8.2–§8.9): header/environment block (authoring baseline + exact final SHA, latest-main-not-verified statement, clean worktree, tool versions, fingerprints, predecessor artifacts consumed); command ledger; evidence-item ledger; per-seam verdict matrix (one row per `ORD-LIFE-01`…`12` plus configured-mutation-posture and artifact/evidence-honesty rows); completed-mutation pass row; ordinary-life behavior/replay/provenance package; `EMERGE-OBS` observer-only member; staged-abstraction declaration (`none` or per-abstraction). Link the stable, fingerprinted evidence paths (the -003 diagnostic, the -004 triage register / final-missed / list outputs / full.out / transcripts, and an `EMERGE-OBS` companion `reports/0043_ord_life_cert_emerge_obs.md`).

### 2. Live ORD-LIFE re-proof at the frozen commit

Re-run (spec §7.1) the clean baseline, the named gate binaries, and the seam evidence at the exact final commit; record each `ORD-LIFE-01`…`12` row with the minimum remediation-specific evidence (spec §7.2 routing matrix), the ten pass conditions (§7.3), the seven mandatory fixture families (§7.4), the cross-cutting families (§7.5), and the generated/metamorphic package (§7.6). SPINE/EPI shared seams are exercised as ancestry only, not re-audited.

### 3. Render the verdict and supersede 0042

Render `ORD-LIFE-CERT passed` only if every §8.10 condition holds (all 12 rows pass; ten pass conditions + seven fixture families pass; complete configured posture run + reconciled; all historical + new survivors dispositioned; zero blocked/untriaged, zero unsigned exceptions, zero unresolved timeouts/incomplete shards; no-shrink/no-laundering/CI-trigger/single-charge/replay-provenance/truth-firewall/no-direct-dispatch/possession-continuity/debug-quarantine controls pass; all template fields complete; no upstream doctrine conflict). The passing artifact states it supersedes `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` for current ORD-LIFE-CERT use. If any condition is unmet, it remains `ORD-LIFE-CERT scoped remediation` with named failing requirement, first responsible layer, retained evidence, and a separately numbered follow-up — never a partial/percentage/"tool flaky but probably clean" pass.

## Files to Touch

- `reports/0043_ord_life_cert_mutation_remediation_replacement_certification_acceptance.md` (new)
- `reports/0043_ord_life_cert_emerge_obs.md` (new — observer-only companion, or an equivalent observer-only section linked from the artifact)

## Out of Scope

- All implementation work it re-proves: the CI convergence (-001), seed kills (-002), lane-completion mechanism (-003), and campaign run + triage register (-004). The capstone exercises these; it does not modify their surfaces.
- The `docs/4-specs/SPEC_LEDGER.md` row update and the `specs/` → `archive/specs/` move — deferred to spec acceptance/archival per `docs/archival-workflow.md` (recorded as a cross-spec follow-up, not ticketed here; spec §8.11, §1.5).
- Beginning any later gate (`FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`) — they remain locked until this artifact passes (spec §1.5).
- Re-auditing SPINE-CERT / EPI-CERT (consumed as settled predecessors; ancestry only — spec §7.7, §11.1).

## Acceptance Criteria

### Tests That Must Pass

1. Clean baseline + named core/content/tui gate binaries pass at the frozen final commit (re-run, not copied from -004).
2. Each `ORD-LIFE-01`…`12` row, the ten pass conditions, the seven mandatory fixture families, and the cross-cutting families have live positive + adversarial/negative + replay/provenance evidence at the frozen commit.
3. The completed-mutation pass row reconciles to -004's campaign (denominator, transport, outcome counts, disposition counts, member evidence); zero blocked/untriaged identities, zero unsigned exceptions, zero unresolved timeouts/tool failures/incomplete shards.
4. The artifact instantiates every `docs/4-specs/0003` field; `EMERGE-OBS` is observer-only and contributes to no pass/fail; pending/sampled/observer-only/historical evidence is labeled and never aggregated as pass.
5. The verdict is `ORD-LIFE-CERT passed` with an explicit supersession of the 0042 artifact ONLY if every §8.10 condition holds; otherwise it remains `ORD-LIFE-CERT scoped remediation` with named responsible layer + follow-up.

### Invariants

1. INV-098 / INV-004 / INV-018 / INV-093 / INV-095: the ORD-LIFE feature set is re-proven caused, no-human-runnable, replay-safe, leak-free, and TUI-playable at one frozen commit; no historical/partial/observer-only evidence is promoted into a live pass.
2. INV-111: `EMERGE-OBS` remains observer-only — replayable, retrospective, feeding no behavior, defining no score/objective.

## Test Plan

### New/Modified Tests

1. `None — acceptance-only capstone; verification re-runs existing gate suites/fixtures at the frozen commit and the deliverable is the replacement acceptance artifact with its per-seam verdict matrix and completed-mutation pass row.`

### Commands

1. Re-proof baseline + gates: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked && cargo test --locked -p tracewake-core --doc`, then the named core/content/tui gate binaries (spec §4.7) at the frozen commit.
2. Replay/no-leak seam evidence: run the golden-fixture / replay and hidden-truth gates that back the ORD-LIFE rows (e.g. `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test golden_scenarios --test hidden_truth_gates --test no_human_capstone`; `cargo test --locked -p tracewake-content --test golden_fixtures_run`), capturing the per-seam witnesses.
3. Verdict reconciliation: confirm the completed-mutation pass row matches `reports/0043_ord_life_cert_mutation_triage_register.md` + `reports/0043_ord_life_cert_mutation_final_missed.txt` from -004, and that the artifact renders the verdict per §8.10 with the 0042 supersession statement.
