# 0020 Mutation Baseline Disposition Ledger

Completed: 2026-06-11

This ledger records the accepted normalized entries in
`.cargo/mutants-baseline-misses.txt` after the 0020 `-001` and `-002` tests landed.
The guard normalizes file+mutation+function by stripping line and column numbers,
matching CI ratchet semantics. The 0022 triage pass assigned every remaining entry
to a concrete focused-test-debt ticket; no baseline entry was retired by
`0022PHA3ABASTRI-001` because no targeted mutation-killing run was performed for an
individual entry in that ticket.

Disposition keys are closed by `mutation_baseline_misses_are_pinned_and_ledgered`:
`justified-baseline` means the mutant remains accepted for the current lock-layer
baseline with a rationale specific enough to survive the repetition bound and no
deferred-test language; `warrants-test:<ticket-id>` is reserved for entries already
assigned to a real follow-up ticket before they can be removed.

## Baseline Change Log

- 0022PHA3ABASTRI-004 — baseline-delta: normalized-count=143 fnv1a64=bd1855a5ee82b428; ratchet encoding only, with no accepted-entry growth or retirement.
- 0022PHA3ABASTRI-015 — baseline-delta: normalized-count=137 fnv1a64=977cce46b241e47b; retired seven `actions/pipeline.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/actions/pipeline.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-016 — baseline-delta: normalized-count=130 fnv1a64=8285ca57bc708d55; retired seven `actions/defs/eat.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/actions/defs/eat.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-017 — baseline-delta: normalized-count=123 fnv1a64=de3b7670491e9a39; retired eight `agent/actor_known.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/actor_known.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-018 — baseline-delta: normalized-count=110 fnv1a64=3ca20aa3e40a267e; retired thirteen candidate/decision/generation focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-019 — baseline-delta: normalized-count=83 fnv1a64=a336ed7ea5c0ed12; retired twenty-eight HTN/intention/methods/routine focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-020 — baseline-delta: normalized-count=71 fnv1a64=007ff5276b9b36cd; retired twelve `agent/need.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/need.rs --no-shuffle` caught all viable mutants after replacing one equivalent comparison surface with explicit ordering.
- 0022PHA3ABASTRI-021 — baseline-delta: normalized-count=44 fnv1a64=28297e34c777adc7; retired twenty-seven no-human-surface/planner/scheduler focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-022 — baseline-delta: normalized-count=16 fnv1a64=e099eb55c87448c7; retired twenty-eight `agent/trace.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/trace.rs --no-shuffle` caught all viable mutants.
- `crates/tracewake-core/src/agent/perception.rs: replace && with || in is_visible_exit_target` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace && with || in is_visible_exit_target`.
- `crates/tracewake-core/src/agent/transaction.rs: delete match arm "active_intention_present" | "next_step_available" in witness_kind_allowed` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm "active_intention_present" | "next_step_available" in witness_kind_allowed`.
- `crates/tracewake-core/src/agent/transaction.rs: delete match arm "actor_belief_projection_limitation" | "modeled_wait_reason" | "reevaluation_window_known" in witness_kind_allowed` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm "actor_belief_projection_limitation" | "modeled_wait_reason" | "reevaluation_window_known" in witness_kind_allowed`.
- `crates/tracewake-core/src/agent/transaction.rs: delete match arm "agent_needs_present" in witness_kind_allowed` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm "agent_needs_present" in witness_kind_allowed`.
- `crates/tracewake-core/src/agent/transaction.rs: delete match arm "empty local plan" in stuck_diagnostic` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm "empty local plan" in stuck_diagnostic`.
- `crates/tracewake-core/src/agent/transaction.rs: delete match arm "no applicable candidate" in stuck_diagnostic` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm "no applicable candidate" in stuck_diagnostic`.
- `crates/tracewake-core/src/agent/transaction.rs: replace active_intention_for_actor -> Option<crate::agent::Intention> with None` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace active_intention_for_actor -> Option<crate::agent::Intention> with None`.
- `crates/tracewake-core/src/projections.rs: delete match arm Some(0) in phase3a_semantic_actions` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `delete match arm Some(0) in phase3a_semantic_actions`.
- `crates/tracewake-core/src/projections.rs: replace && with || in no_human_day_metrics` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace && with || in no_human_day_metrics`.
- `crates/tracewake-core/src/projections.rs: replace == with != in proposal_from_semantic_action_entry` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace == with != in proposal_from_semantic_action_entry`.
- `crates/tracewake-core/src/projections.rs: replace actor_known_sleep_affordances_for_context -> Vec<SleepAffordanceId> with vec![]` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace actor_known_sleep_affordances_for_context -> Vec<SleepAffordanceId> with vec![]`.
- `crates/tracewake-core/src/projections.rs: replace typed_blocker_code -> Option<BlockerCode> with None` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace typed_blocker_code -> Option<BlockerCode> with None`.
- `crates/tracewake-core/src/projections.rs: replace typed_responsible_layer -> Option<ResponsibleLayer> with None` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace typed_responsible_layer -> Option<ResponsibleLayer> with None`.
- `crates/tracewake-core/src/projections.rs: replace visible_open_sleep_affordance -> Option<SleepAffordanceId> with None` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace visible_open_sleep_affordance -> Option<SleepAffordanceId> with None`.
- `crates/tracewake-core/src/projections.rs: replace || with && in contains_player_conditioned_fact` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace || with && in contains_player_conditioned_fact`.
- `crates/tracewake-core/src/projections.rs: replace || with && in is_typed_planner_failure_event` — warrants-test:0022PHA3ABASTRI-023: Focused mutation coverage remains assigned for `replace || with && in is_typed_planner_failure_event`.
