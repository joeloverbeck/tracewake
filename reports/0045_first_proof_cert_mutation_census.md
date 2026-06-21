# 0045 FIRST-PROOF-CERT mutation census

- Final evidence SHA: `9a071b6e32ebc5b6126645a9db257d453399c028`
- Clean status before generated evidence: not clean
- Cargo-mutants version: `cargo-mutants 27.1.0`
- Selected file count: 62
- Canonical mutant count: 2901
- Authoring-baseline delta: none; 0045 selected file count and mutant count match the 0044 baseline (62 files / 2,901 rows).
- Selected file list SHA-256: `5473142052dec947e4a238f286a1af659c603ebf678bf22634af997b8570e8e5`
- Canonical mutant list SHA-256: `64cd82ab86330e595f161ba374e59ac559b7b48e25032fd0a739f62e355b59ac`
- Completion canonical identity fingerprint: `c4bf4c6ae9d0184e6b3328db79218bf04e9c7ff1db0e7fd41a872c4627255425`
- Sharding: deterministic `--no-shuffle --shard K/8`, display-name identity mode, shard counts 363,363,363,363,363,363,363,360.

## Source Fingerprints

- `f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59  Cargo.lock`
- `dfd7c841cfa60c0fee22a94b42c4cb2d4ec638cf59363f5e00bcc5ea5f99df66  Cargo.toml`
- `027f4900829924c77f79205ed5c8d7eca01388c4737ea9c38bc509a62dab5edd  rust-toolchain.toml`
- `57154de07f3037ce76f91e5c67f5be136cb5cc631ad9b403730fd288137bc575  .cargo/mutants.toml`
- `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  .cargo/mutants-baseline-misses.txt`
- `848227cf867a30892a63bc554619e9ed694b995f9f4042c7caa5cba82390f63e  .github/workflows/ci.yml`
- `c3e09d01a5640e76bfdfad36befa5efe51f2877cfb7fee89eea26bf82a84eb03  tools/supervise-command.sh`
- `c5ce2ab8391c6390eccd234a986d1c2402db9d97c1d596bf2a80df70d8bc9ec4  tools/merge-mutation-shards.py`

## Zero-Mutant Selected Files

- `crates/tracewake-core/src/agent/mod.rs`
- `crates/tracewake-core/src/epistemics/mod.rs`
- `crates/tracewake-core/src/events/mod.rs`
- `crates/tracewake-core/src/replay/mod.rs`

## Required Carrier Presence

- `crates/tracewake-core/src/time.rs`: present, 9 mutants
- `crates/tracewake-core/src/events/apply.rs`: present, 206 mutants
- `crates/tracewake-core/src/replay/rebuild.rs`: present, 58 mutants
- `crates/tracewake-core/src/epistemics/contradiction.rs`: present, 18 mutants
- `crates/tracewake-core/src/epistemics/projection.rs`: present, 143 mutants
- `crates/tracewake-core/src/agent/perception.rs`: present, 34 mutants
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs`: present, 14 mutants

## Shard Completion Summary

| Shard | Assigned | Outcomes | Caught | Missed | Timeout | Unviable | Supervisor |
|---|---:|---:|---:|---:|---:|---:|---|
| 0/8 | 363 | 363 | 322 | 0 | 0 | 41 | child_exit_0 |
| 1/8 | 363 | 363 | 291 | 0 | 0 | 72 | child_exit_0 |
| 2/8 | 363 | 363 | 277 | 0 | 0 | 86 | child_exit_0 |
| 3/8 | 363 | 363 | 277 | 0 | 0 | 86 | child_exit_0 |
| 4/8 | 363 | 363 | 290 | 0 | 0 | 73 | child_exit_0 |
| 5/8 | 363 | 363 | 250 | 0 | 0 | 113 | child_exit_0 |
| 6/8 | 363 | 363 | 284 | 0 | 0 | 79 | child_exit_0 |
| 7/8 | 360 | 360 | 286 | 0 | 0 | 74 | child_exit_0 |

## Per-File Counts

| File | Mutants |
|---|---:|
| `crates/tracewake-content/src/load.rs` | 25 |
| `crates/tracewake-content/src/manifest.rs` | 7 |
| `crates/tracewake-content/src/schema.rs` | 27 |
| `crates/tracewake-content/src/serialization.rs` | 128 |
| `crates/tracewake-content/src/validate.rs` | 246 |
| `crates/tracewake-core/src/agent/mod.rs` | 0 |
| `crates/tracewake-core/src/checksum.rs` | 25 |
| `crates/tracewake-core/src/controller.rs` | 13 |
| `crates/tracewake-core/src/debug_capability.rs` | 6 |
| `crates/tracewake-core/src/debug_reports.rs` | 53 |
| `crates/tracewake-core/src/epistemics/mod.rs` | 0 |
| `crates/tracewake-core/src/events/mod.rs` | 0 |
| `crates/tracewake-core/src/need_accounting.rs` | 42 |
| `crates/tracewake-core/src/projections.rs` | 123 |
| `crates/tracewake-core/src/replay/mod.rs` | 0 |
| `crates/tracewake-core/src/scheduler.rs` | 203 |
| `crates/tracewake-core/src/state.rs` | 58 |
| `crates/tracewake-core/src/time.rs` | 9 |
| `crates/tracewake-core/src/view_models.rs` | 50 |
| `crates/tracewake-core/src/actions/pipeline.rs` | 68 |
| `crates/tracewake-core/src/actions/proposal.rs` | 4 |
| `crates/tracewake-core/src/actions/registry.rs` | 19 |
| `crates/tracewake-core/src/actions/report.rs` | 31 |
| `crates/tracewake-core/src/agent/actor_known.rs` | 79 |
| `crates/tracewake-core/src/agent/candidate.rs` | 13 |
| `crates/tracewake-core/src/agent/decision.rs` | 26 |
| `crates/tracewake-core/src/agent/generation.rs` | 20 |
| `crates/tracewake-core/src/agent/htn.rs` | 26 |
| `crates/tracewake-core/src/agent/intention.rs` | 19 |
| `crates/tracewake-core/src/agent/methods.rs` | 18 |
| `crates/tracewake-core/src/agent/need.rs` | 64 |
| `crates/tracewake-core/src/agent/no_human_surface.rs` | 17 |
| `crates/tracewake-core/src/agent/perception.rs` | 34 |
| `crates/tracewake-core/src/agent/planner.rs` | 22 |
| `crates/tracewake-core/src/agent/routine.rs` | 71 |
| `crates/tracewake-core/src/agent/trace.rs` | 146 |
| `crates/tracewake-core/src/agent/transaction.rs` | 70 |
| `crates/tracewake-core/src/epistemics/belief.rs` | 34 |
| `crates/tracewake-core/src/epistemics/contradiction.rs` | 18 |
| `crates/tracewake-core/src/epistemics/knowledge_basis.rs` | 4 |
| `crates/tracewake-core/src/epistemics/knowledge_context.rs` | 169 |
| `crates/tracewake-core/src/epistemics/observation.rs` | 29 |
| `crates/tracewake-core/src/epistemics/projection.rs` | 143 |
| `crates/tracewake-core/src/epistemics/proposition.rs` | 52 |
| `crates/tracewake-core/src/events/apply.rs` | 206 |
| `crates/tracewake-core/src/events/envelope.rs` | 112 |
| `crates/tracewake-core/src/events/log.rs` | 21 |
| `crates/tracewake-core/src/events/mutation.rs` | 2 |
| `crates/tracewake-core/src/replay/rebuild.rs` | 58 |
| `crates/tracewake-core/src/replay/report.rs` | 16 |
| `crates/tracewake-core/src/actions/defs/checkcontainer.rs` | 14 |
| `crates/tracewake-core/src/actions/defs/continue_routine.rs` | 9 |
| `crates/tracewake-core/src/actions/defs/eat.rs` | 24 |
| `crates/tracewake-core/src/actions/defs/movement.rs` | 13 |
| `crates/tracewake-core/src/actions/defs/need_events.rs` | 6 |
| `crates/tracewake-core/src/actions/defs/sleep.rs` | 35 |
| `crates/tracewake-core/src/actions/defs/wait.rs` | 15 |
| `crates/tracewake-core/src/actions/defs/work.rs` | 50 |
| `crates/tracewake-tui/src/app.rs` | 63 |
| `crates/tracewake-tui/src/debug_panels.rs` | 24 |
| `crates/tracewake-tui/src/render.rs` | 14 |
| `crates/tracewake-tui/src/transcript.rs` | 8 |
