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

- 0022PHA3ABASTRI-004 — baseline-delta: from-count=143 from-fnv1a64=bd1855a5ee82b428 -> to-count=143 to-fnv1a64=bd1855a5ee82b428; ratchet encoding only, with no accepted-entry growth or retirement.
- 0022PHA3ABASTRI-015 — baseline-delta: from-count=143 from-fnv1a64=bd1855a5ee82b428 -> to-count=137 to-fnv1a64=977cce46b241e47b; retired seven `actions/pipeline.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/actions/pipeline.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-016 — baseline-delta: from-count=137 from-fnv1a64=977cce46b241e47b -> to-count=130 to-fnv1a64=8285ca57bc708d55; retired seven `actions/defs/eat.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/actions/defs/eat.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-017 — baseline-delta: from-count=130 from-fnv1a64=8285ca57bc708d55 -> to-count=123 to-fnv1a64=de3b7670491e9a39; retired eight `agent/actor_known.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/actor_known.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-018 — baseline-delta: from-count=123 from-fnv1a64=de3b7670491e9a39 -> to-count=110 to-fnv1a64=3ca20aa3e40a267e; retired thirteen candidate/decision/generation focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-019 — baseline-delta: from-count=110 from-fnv1a64=3ca20aa3e40a267e -> to-count=83 to-fnv1a64=a336ed7ea5c0ed12; retired twenty-eight HTN/intention/methods/routine focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-020 — baseline-delta: from-count=83 from-fnv1a64=a336ed7ea5c0ed12 -> to-count=71 to-fnv1a64=007ff5276b9b36cd; retired twelve `agent/need.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/need.rs --no-shuffle` caught all viable mutants after replacing one equivalent comparison surface with explicit ordering.
- 0022PHA3ABASTRI-021 — baseline-delta: from-count=71 from-fnv1a64=007ff5276b9b36cd -> to-count=44 to-fnv1a64=28297e34c777adc7; retired twenty-seven no-human-surface/planner/scheduler focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
- 0022PHA3ABASTRI-022 — baseline-delta: from-count=44 from-fnv1a64=28297e34c777adc7 -> to-count=16 to-fnv1a64=e099eb55c87448c7; retired twenty-eight `agent/trace.rs` focused-test entries after `cargo mutants -f crates/tracewake-core/src/agent/trace.rs --no-shuffle` caught all viable mutants.
- 0022PHA3ABASTRI-023 — baseline-delta: from-count=16 from-fnv1a64=e099eb55c87448c7 -> to-count=0 to-fnv1a64=cbf29ce484222325; retired sixteen transaction/projection/perception focused-test entries after targeted `cargo mutants -f` runs caught all viable mutants.
