# 0048 Foundational Conformance Hardening Acceptance

Status: accepted for ticket evidence closeout on 2026-06-23.

This artifact records the evidence for `0048FOUCONHAR-008`, the capstone
report for spec `0048` after tickets `001` through `007` landed. It is an
evidence and review artifact only; it adds no doctrine, no production code, no
test-harness code, no new risk id, and no `.cargo/mutants.toml` perimeter
change.

## Commit Roles

- Implementation baseline commits:
  - `31889e8` `0048FOUCONHAR-001` replay temporal frontier
  - `3b76142` `0048FOUCONHAR-002` holder-known interval delta
  - `7610ed5` `0048FOUCONHAR-003` world-step transaction
  - `50d26d4` `0048FOUCONHAR-004` temporal authority cutover
  - `28e9fd2` `0048FOUCONHAR-005` typed interval stop
  - `dbd6ee6` `0048FOUCONHAR-006` behavioral locks
  - `3964f24` `0048FOUCONHAR-007` parity evidence
- Evidence/report commit: the commit adding this report and archiving
  `0048FOUCONHAR-008`.
- Archive/truthing commit: the later spec-closeout commit, if any, owns the
  final spec move and ledger repair.

## Gate Evidence

The full workspace gates named by `AGENTS.md` passed before this report and the
closeout doc edits were written:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

The earlier ticket commits also ran focused witnesses while their source/test
changes were current, including:

- `cargo test -p tracewake-core --test world_step_coordinator`
- `cargo test -p tracewake-core --test generative_lock --test reservation_body_exclusive_census`
- `cargo test -p tracewake-core`
- `cargo clippy -p tracewake-core --all-targets -- -D warnings`
- `cargo test -p tracewake-tui --test playable_capability_parity`
- `cargo test -p tracewake-tui`
- `cargo clippy -p tracewake-tui --all-targets -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`

Because ticket 008 only updates docs, the final ticket check after this report
is `git diff --check` plus grep proof over the conformance and risk surfaces.
The final spec closeout, if performed after ticket 008, must run the full
`AGENTS.md` gates again or record an explicit deviation.

## Gate To Witness Map

| Evidence class | Executable witnesses |
|---|---|
| Temporal replay and frontier authority | `crates/tracewake-core/tests/replay_temporal_frontier.rs`, `crates/tracewake-core/tests/world_step_coordinator.rs`, `project_temporal_frontier`, `validate_time_advanced`, and scheduler frontier privacy/restore paths. |
| Holder-known noninterference | `crates/tracewake-core/tests/holder_known_interval_projection.rs`, `crates/tracewake-core/tests/salient_stop_actor_known.rs`, `ActorKnownIntervalDelta`, and `VerifiedActorKnownIntervalNotice`. |
| Loaded-world actor/process differential | `crates/tracewake-core/tests/world_step_coordinator.rs`, especially the human-vs-scheduler/no-human loaded-world differential, duration lifecycle, accounting, and world-process counters. |
| Measured parity output and anti-vacuous locks | `crates/tracewake-tui/tests/playable_capability_parity.rs`, `crates/tracewake-core/tests/generative_lock.rs`, and `crates/tracewake-core/tests/reservation_body_exclusive_census.rs`. |

The parity evidence distinguishes registry declaration metadata from measured
output: the TUI parity rows require measured frontier/marker text, duration
terminal evidence, holder-known source evidence, typed stop reason evidence,
and autonomous no-human work/marker evidence before a load-bearing row counts.

## Focused Mutation Campaign

Tool version:

```sh
cargo mutants --version
# cargo-mutants 27.1.0
```

Denominator check:

```sh
cargo mutants --no-config --list \
  -f crates/tracewake-core/src/scheduler.rs \
  -f crates/tracewake-core/src/replay/temporal.rs \
  -f crates/tracewake-core/src/projections.rs \
  -F 'transact_world_one_tick|advance_until|build_time_advanced_event|project_temporal_frontier|validate_time_advanced|ActorKnownIntervalDelta|VerifiedActorKnownIntervalNotice|proposal_from_current_view_semantic_action|build_embodied_view_model'
```

The focused denominator was 61 mutants. `--no-config` was deliberate: the
checked-in mutation config already covers the standing perimeter and widened
the exploratory list beyond this ticket's focused post-remediation campaign.

Executed campaign:

```sh
cargo mutants --no-config \
  -f crates/tracewake-core/src/scheduler.rs \
  -f crates/tracewake-core/src/replay/temporal.rs \
  -f crates/tracewake-core/src/projections.rs \
  -F 'transact_world_one_tick|advance_until|build_time_advanced_event|project_temporal_frontier|validate_time_advanced|ActorKnownIntervalDelta|VerifiedActorKnownIntervalNotice|proposal_from_current_view_semantic_action|build_embodied_view_model' \
  --test-package tracewake-core \
  --timeout 120 \
  --minimum-test-timeout 20 \
  --no-times \
  --output target/mutants-0048-core
```

Result:

- 61 mutants tested.
- 40 caught.
- 8 missed.
- 13 unviable.

The stable result files were:

- `target/mutants-0048-core/mutants.out/caught.txt`
- `target/mutants-0048-core/mutants.out/missed.txt`
- `target/mutants-0048-core/mutants.out/unviable.txt`

## Mutation Survivor Triage

Missed mutants:

1. `crates/tracewake-core/src/projections.rs:776:9` replaced
   `VerifiedActorKnownIntervalNotice::source_key` with `""`.
2. `crates/tracewake-core/src/projections.rs:776:9` replaced
   `VerifiedActorKnownIntervalNotice::source_key` with `"xyzzy"`.
3. `crates/tracewake-core/src/scheduler.rs:516:49` replaced `==` with `!=` in
   `DeterministicScheduler::transact_world_one_tick`.
4. `crates/tracewake-core/src/scheduler.rs:625:32` replaced `==` with `!=` in
   `DeterministicScheduler::transact_world_one_tick`.
5. `crates/tracewake-core/src/replay/temporal.rs:75:47` replaced `&&` with
   `||` in `project_temporal_frontier`.
6. `crates/tracewake-core/src/replay/temporal.rs:75:65` replaced `>` with
   `>=` in `project_temporal_frontier`.
7. `crates/tracewake-core/src/replay/temporal.rs:112:23` replaced `<` with
   `<=` in `validate_time_advanced`.
8. `crates/tracewake-core/src/replay/temporal.rs:119:38` replaced `||` with
   `&&` in `validate_time_advanced`.

Disposition:

- The two `source_key()` accessor mutants are non-blocking display/accessor
  survivors for this focused capstone. Existing witnesses validate source event
  id, notice kind, frontier, and rendered interval summary evidence, but do not
  assert the exact accessor string as a stable API.
- The two scheduler rejection-condition mutants are scoped campaign survivors.
  The focused campaign ran `--test-package tracewake-core`; they show the core
  package witnesses do not independently kill both controlled-proposal
  rollback predicate inversions. TUI parity and transaction tests remain the
  relevant composed-path evidence, but a future mutation pass should either run
  the workspace/TUI seam or add a narrower core witness.
- The four temporal projection/validation mutants are witness gaps in the
  focused core campaign. Existing tests catch marker omission, ordinary future
  world events, payload/cause corruption, and many arithmetic divergences, but
  do not kill every boundary mutation for non-world future events, same-tick
  ordinary world events, zero-duration markers, and split duplicate/frontier
  conditions. These are not classified equivalent.

Unviable mutants:

1. `crates/tracewake-core/src/projections.rs:573:5` replaced
   `build_embodied_view_model` with `Ok(Default::default())`.
2. `crates/tracewake-core/src/projections.rs:760:9` replaced
   `VerifiedActorKnownIntervalNotice::from_verified` with `Default::default()`.
3. `crates/tracewake-core/src/projections.rs:768:9` replaced
   `VerifiedActorKnownIntervalNotice::notice_kind` with `Default::default()`.
4. `crates/tracewake-core/src/projections.rs:772:9` replaced
   `VerifiedActorKnownIntervalNotice::source_event_id` with a leaked default.
5. `crates/tracewake-core/src/projections.rs:802:9` replaced
   `ActorKnownIntervalDelta::from_verified` with `Default::default()`.
6. `crates/tracewake-core/src/projections.rs:816:9` replaced
   `ActorKnownIntervalDelta::viewer_actor_id` with a leaked default.
7. `crates/tracewake-core/src/projections.rs:836:9` replaced
   `ActorKnownIntervalDelta::stop_reason` with `Default::default()`.
8. `crates/tracewake-core/src/projections.rs:840:9` replaced
   `ActorKnownIntervalDelta::notices` with a leaked default vector.
9. `crates/tracewake-core/src/projections.rs:1543:5` replaced
   `proposal_from_current_view_semantic_action` with `Default::default()`.
10. `crates/tracewake-core/src/scheduler.rs:447:9` replaced
    `DeterministicScheduler::transact_world_one_tick` with
    `Ok(Default::default())`.
11. `crates/tracewake-core/src/scheduler.rs:831:9` replaced
    `DeterministicScheduler::advance_until` with `Ok(Default::default())`.
12. `crates/tracewake-core/src/scheduler.rs:1483:5` replaced
    `build_time_advanced_event` with `Ok(Default::default())`.
13. `crates/tracewake-core/src/replay/temporal.rs:63:5` replaced
    `project_temporal_frontier` with `Default::default()`.

The unviable set is compiler/type-shape fallout from replacing typed builders
or accessors with default values that do not satisfy required trait or lifetime
constraints.

## Conformance And Risk Register Updates

This capstone updates:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`

The risk-register update uses only existing IDs: R-08, R-09, R-10, R-11,
R-13, R-15, R-16, R-27, R-28, and R-29. No `R-NN` id is minted.

## Limits

- The focused mutation campaign was core-package only and used `--no-config` to
  keep the selected denominator at the rewritten surfaces named above. It does
  not replace the standing configured perimeter or a future scheduled mutation
  ratchet.
- Eight surviving focused mutants remain recorded above. The capstone accepts
  the report with those survivors triaged; it does not claim mutation-perfect
  closure.
- Full workspace gates passed before the report/doc edits. The report/doc edits
  are doc-only; the final ticket check after edits is whitespace/truth grep.
