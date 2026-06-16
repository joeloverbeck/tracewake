# 0037P0CERTMUTREM-002: Run full configured mutation posture to completion + survivor triage register

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — runs the existing configured mutation posture and produces an evidence/triage artifact; any code fix for a newly-discovered critical survivor is routed to a reserved follow-up ticket.
**Deps**: 0037P0CERTMUTREM-001

## Problem

0036 stopped its mutation run after emitting one miss, so its mutation evidence is sampled and incomplete and the lock layer is unproven. P0-CERT cannot pass on a partial run — killing only the named survivor would be Goodharting. This ticket runs the full configured guarded-layer posture to completion and dispositions every survivor it surfaces.

(spec §3, §4.3, §4.4)

## Assumption Reassessment (2026-06-16)

1. The configured posture is the scheduled `mutants-lock-layer` job in `.github/workflows/ci.yml:188-196` (`cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f '…/scheduler*' -f '…/projections*' -f '…/actions/pipeline.rs' -f eat/sleep/work.rs --no-shuffle`), cargo-mutants 27.1.0, with `.cargo/mutants.toml` adding `additional_cargo_args = ["--workspace", "--locked"]` plus the `exclude_globs` perimeter. Verified verbatim this session — the spec §4.3 command matches CI exactly.
2. Spec §4.3 (sharding restrictions, `--baseline=skip` rules, timeouts-are-not-green) and §4.4 (triage-register minimum fields) govern. `.cargo/mutants-baseline-misses.txt` is empty at `c54caff` (verified) — no survivor may be hidden behind a baseline miss.
3. Cross-artifact boundary under audit: the configured mutation perimeter (actor cognition / scheduler / projections / action-pipeline boundary / guarded action defs) — the lock layer protecting the truth-firewall and cognition-authority invariants.
4. Invariant motivation: the guarded seams enforce INV-099–108 (truth-firewall / cognition-authority set), INV-024, INV-006. The run must complete and every survivor be triaged; a timeout under a certified seam requires triage, not a green pass (spec §4.3).
5. Replay / actor-knowledge surfaces are the mutation targets; triage must use existing typed responsible layers (`candidate_generation`, `intention_lifecycle`, `method_selection`, `local_planning`, `proposal_construction`, `scheduler`, `action_validation`, `projection`, `view_model`, `test_oracle`, `debug_quarantine`, plus existing non-agent diagnostic categories) — no invented gate/status vocabulary — and must not treat any `missed`/`timeout` as green without a reviewable equivalence/non-critical argument naming exact call sites and certified-behavior reachability.

## Architecture Check

1. Running the spec/CI-identical command (not an ad-hoc filter) guarantees the enumerated-mutant denominator and perimeter match the certified posture; any sharding must hold identical args + denominator (spec §4.3) so the aggregate stays meaningful.
2. No shims; the triage register records dispositions rather than suppressing survivors via the baseline file (forbidden, spec §10).

## Verification Layers

1. Mutation completeness → cargo-mutants output: `mutants.out` (`mutants.json`, `outcomes.json`, `missed.txt`, `timeout.txt`, `unviable.txt`, logs) proves total enumerated count and that the full posture (or all consistent shards) completed with no silent skip.
2. Survivor disposition → triage register: every `missed`/`timeout` carries mutant identity, outcome, responsible layer, certified reachability, call-site review, disposition (`killed` / `equivalent` / `non-critical` / `unviable` / `blocked`), behavior + provenance/replay + negative/adversarial witnesses, and review signoff.
3. Truth-firewall integrity → invariants alignment check: no disposition weakens INV-099–108 / INV-024 / INV-006 even if it would kill a mutant.

## What to Change

### 1. Run the full configured posture to completion

Install `cargo-mutants 27.1.0 --locked`; run the §4.3 command (optionally sharded per the §4.3 restrictions: identical checkout/version/config/filters/`--no-shuffle`/denominator, every shard `0/n`..`(n-1)/n` completes, distinct output dirs, no silent skip, `--baseline=skip` only with a recorded immediately-preceding unmutated baseline + explicit timeout strategy). After -001 lands, confirm `actor_known_local_actors_for_context -> vec![]` is `caught`.

### 2. Triage every survivor

Maintain the mutation-triage register (Markdown and/or JSON) with the spec §4.4 minimum fields. Kill every critical survivor with behavior/provenance/replay coverage, or prove equivalent/non-critical with a reviewable argument naming exact call sites and certified-behavior reachability. Reviewer signoff is required for any equivalent/non-critical disposition on a truth-firewall seam.

### 3. Route discovered survivors out

Any newly-discovered critical survivor that needs a code fix becomes its own remediation ticket — **reserve `0037P0CERTMUTREM-004` onward** (count implementation-discovered; the next batch in this namespace must check the actual high-water mark before claiming numbers).

## Files to Touch

- `reports/0037_mutation_triage_register.md` (new) — or a JSON sibling; the register that the replacement artifact (-003) embeds or links.
- `tickets/0037P0CERTMUTREM-004.md` onward (new — reserved, implementation-discovered; only authored if critical survivors surface)

## Out of Scope

- The killing test for the already-named survivor (→ -001; this ticket consumes its result).
- P0-01..P0-10 re-proof and the replacement acceptance artifact (→ -003).
- Editing the `.cargo/mutants.toml` perimeter or `.cargo/mutants-baseline-misses.txt` (the posture is fixed; baseline-file suppression of survivors is forbidden).

## Acceptance Criteria

### Tests That Must Pass

1. The full configured posture completes (or all consistent shards complete); `mutants.out` artifacts exist and the total enumerated mutant count is recorded.
2. Every `missed`/`timeout` survivor has a triage-register row with a recorded disposition; no untriaged survivors remain.
3. `actor_known_local_actors_for_context -> vec![]` is recorded `caught` (post -001).

### Invariants

1. No survivor is waived merely because no existing test failed; equivalence/non-critical dispositions name exact call sites and carry reviewer signoff.
2. No timeout under a certified seam is treated as green without a reviewable equivalence/non-critical argument.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the deliverable is the completed mutation run + triage register. Verification is the cargo-mutants posture command and the register's recorded dispositions; existing pipeline coverage is unchanged.`

### Commands

1. `cargo install cargo-mutants --version 27.1.0 --locked`
2. `cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' -f 'crates/tracewake-core/src/scheduler*' -f 'crates/tracewake-core/src/projections*' -f 'crates/tracewake-core/src/actions/pipeline.rs' -f 'crates/tracewake-core/src/actions/defs/eat.rs' -f 'crates/tracewake-core/src/actions/defs/sleep.rs' -f 'crates/tracewake-core/src/actions/defs/work.rs' --no-shuffle` (with `.cargo/mutants.toml` in effect)
3. Inspect `mutants.out/missed.txt`, `mutants.out/timeout.txt`, `mutants.out/outcomes.json` — every entry maps to a triage-register disposition; no entry is left untriaged.
