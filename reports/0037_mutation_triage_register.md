# 0037 P0-CERT mutation triage register

**Status**: COMPLETE
**Ticket**: `0037P0CERTMUTREM-002`
**Date**: 2026-06-16
**Posture**: Full configured guarded-layer mutation posture, unsharded.
**Cargo-mutants version**: `cargo-mutants 27.1.0`
**Baseline misses file**: `.cargo/mutants-baseline-misses.txt` was empty.

## Command

```bash
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

The command used `.cargo/mutants.toml`, including
`additional_cargo_args = ["--workspace", "--locked"]`.

## Result

```text
Found 1128 mutants to test
ok       Unmutated baseline in 9s build + 29s test
INFO     Auto-set test timeout to 150s
1128 mutants tested in 66m: 896 caught, 232 unviable
```

Final output files:

| File | Count | Disposition |
|---|---:|---|
| `mutants.out/caught.txt` | 896 | Killed by the configured gate set. |
| `mutants.out/missed.txt` | 0 | No missed survivors. |
| `mutants.out/timeout.txt` | 0 | No timeout survivors. |
| `mutants.out/unviable.txt` | 232 | Tool-classified unviable; not a missed/timeout survivor. |
| `mutants.out/outcomes.json` | 1128 outcomes | Machine-readable result source. |

## 0036-MUTATION-REMEDIATION-001 disposition

| Field | Value |
|---|---|
| Mutant identity | `crates/tracewake-core/src/projections.rs:336:5: replace actor_known_local_actors_for_context -> Vec<ActorId> with vec![]` |
| Outcome | `caught` in `mutants.out/caught.txt`; no corresponding miss or timeout. |
| Responsible layer | `projection`, `view_model`, `test_oracle` |
| Certified reachability | P0-02 sealed actor-known contexts and P0-07 embodied/debug quarantine via holder-known context -> `EmbodiedProjectionSource` -> `EmbodiedViewModel.local_actors`. |
| Call-site review | `EmbodiedProjectionSource::from_sealed_context` calls `actor_known_local_actors_for_context`; `build_embodied_view_model` maps the resulting IDs into `VisibleActor` entries. |
| Disposition | `killed` by `crates/tracewake-core/tests/hidden_truth_gates.rs::actor_known_local_actor_reaches_embodied_view_model_with_context_provenance`. |
| Behavior witness | A source-backed actor-known local actor appears in `EmbodiedViewModel.local_actors`. |
| Provenance/replay witness | The test asserts the sealed holder-known context ID, hash, and frontier are carried into the view model, and uses source-keyed current-place and local-actor facts. |
| Negative/adversarial witness | A physically co-located actor absent from actor-known local actor facts stays absent from `local_actors`; debug availability remains false. |
| Review signoff | Implementer closeout evidence: `0037P0CERTMUTREM-001` commit `2abfce9`, this full configured run, and this register. |

## Missed/timeout survivor register

There were no `missed` or `timeout` survivors in the completed configured run.
No `0037P0CERTMUTREM-004` follow-up ticket was required.

| Mutant identity | Outcome | Responsible layer | Certified reachability | Call-site review | Disposition | Behavior witness | Provenance/replay witness | Negative/adversarial witness | Review signoff |
|---|---|---|---|---|---|---|---|---|---|
| None | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a |

## Notes for replacement P0-CERT artifact

- The full run was unsharded, so no shard-denominator reconciliation is needed.
- The unmutated baseline passed before mutation execution.
- `mutants.out/missed.txt` and `mutants.out/timeout.txt` were empty after completion.
- Archived 0036 mutation evidence remains historical context only; this register is the live 0037 mutation evidence for the replacement artifact.
