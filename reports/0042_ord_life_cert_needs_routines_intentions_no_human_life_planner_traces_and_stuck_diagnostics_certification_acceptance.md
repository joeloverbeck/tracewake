# 0042 ORD-LIFE-CERT certification acceptance artifact

## Exact commit under test

- Commit actually tested: `f7d8d666a8baa220b87d5e037e3eb50c8bf088c5`
- Target commit named by spec: `98dc0421211e6c9881d9c6679b9df74525e392bb`
- Target/tested commit equality: no. The tested commit is the local ticket-creation commit on `main`; it sits after the target commit and contains the active `0042ORDLIFCER` ticket/spec staging material.
- Branch or PR context only: `main`
- Repository: `joeloverbeck/tracewake`
- Work posture: Certification
- Phase label: `ORD-LIFE-CERT`, a composed phase-certification label consuming upstream gate artifacts; this report mints no new gate code, invariant ID, status enum, or obligation code.

## Worktree and source discipline

- Clean/dirty status at baseline execution: clean before this report was created.
- Evidence-only or implementation files changed for `0042ORDLIFCER-001`: this report file only.
- Source discipline: exact commit hashes and exact local paths are evidence; branch names, repository metadata, archive path existence, and historical prose are context only.
- Archived specs, tickets, reports, and research briefs are history unless this artifact explicitly consumes a predecessor acceptance artifact for admissibility.
- Predecessor artifacts consumed for scoped admissibility:
  - `archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md`: `P0-CERT passed` only for the scoped post-0008 baseline mutation remediation line stated by that artifact.
  - `archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`: `SPINE-CERT passed` only for the scoped 0039 mutation remediation line stated by that artifact.
  - `archive/reports/0041_epi_cert_mutation_remediation_replacement_certification_acceptance.md`: `EPI-CERT passed` only for the scoped 0041 mutation remediation line stated by that artifact.
- In-scope audit surface: needs, routines, intentions, no-human life, planner traces, stuck diagnostics, and the actor-known ordinary-life transaction.
- Excluded/downstream scope: FIRST-PROOF-CERT, PHASE-4-ENTRY, SECOND-PROOF-ENTRY, institutions, notices, travel, regional scale, LOD, LLM/speech surfaces, story-sifting, broad economy expansion, and production remediation of any failed in-scope seam.
- Placeholder aggregate verdict, owned by `0042ORDLIFCER-016`: `pending`.

## Environment

- Rust toolchain file: `rust-toolchain.toml`
- Rust toolchain channel: `1.93.0`
- Components: `rustfmt`, `clippy`
- Profile: `minimal`
- `rustc --version`: `rustc 1.93.0 (254b59607 2026-01-19)`
- `cargo --version`: `cargo 1.93.0 (083ac5135 2025-12-15)`
- OS: `Linux JOELOVERBECK 6.6.114.1-microsoft-standard-WSL2 #1 SMP PREEMPT_DYNAMIC Mon Dec 1 20:46:23 UTC 2025 x86_64 x86_64 x86_64 GNU/Linux`
- Local timezone used for command timestamps: `Europe/Madrid`

## Gates run

The first pass ran directly in the terminal against the clean tree. A second pass captured full command output into `/tmp` transcript files; those files are not committed artifacts, but their exact byte counts and SHA-256 digests are recorded below. There were no failures, flakes, timeouts, or selective failure-only retries.

| Command | Initial run window | Capture run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---:|---|---:|---|
| `cargo fmt --all --check` | `2026-06-20T11:48:46+02:00` to `2026-06-20T11:48:46+02:00` | `2026-06-20T11:49:25+02:00` to `2026-06-20T11:49:25+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-001-cargo-fmt.txt` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `2026-06-20T11:48:51+02:00` to `2026-06-20T11:48:51+02:00` | `2026-06-20T11:49:30+02:00` to `2026-06-20T11:49:31+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-001-cargo-clippy.txt` | 72 | `a13905646e2aed93ec6ea9ed6ac91490a4e01f91af0215c3965279a910612149` |
| `cargo build --workspace --all-targets --locked` | `2026-06-20T11:48:56+02:00` to `2026-06-20T11:48:56+02:00` | `2026-06-20T11:49:36+02:00` to `2026-06-20T11:49:36+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-001-cargo-build.txt` | 72 | `a13905646e2aed93ec6ea9ed6ac91490a4e01f91af0215c3965279a910612149` |
| `cargo test --workspace --locked` | `2026-06-20T11:48:59+02:00` to `2026-06-20T11:49:06+02:00` | `2026-06-20T11:49:41+02:00` to `2026-06-20T11:49:47+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-001-cargo-test-workspace.txt` | 79232 | `e5d632ffbb982dbc1460a081962ae9d08d198fb9af15662bf29327edc0f9fa5b` |
| `cargo test --locked -p tracewake-core --doc` | `2026-06-20T11:49:09+02:00` to `2026-06-20T11:49:10+02:00` | `2026-06-20T11:49:52+02:00` to `2026-06-20T11:49:52+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-001-cargo-test-core-doc.txt` | 1386 | `337b43984a2324a53b2ed6ddd9df99d173f6efb25c81f130012d41e2e60b34ff` |

### ORD-LIFE-01 command ledger

These commands were run for `0042ORDLIFCER-002` against commit `785d56758a00247284fb818ee72885405dc3760c` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T11:52:23+02:00` to `2026-06-20T11:52:23+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-002-no-human-capstone.txt` | 343 | `90c6564c5bf44199de8da4ce5e8e3cbc233ac1f699cabecfb7792de0ddfe3579` |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | `2026-06-20T11:52:28+02:00` to `2026-06-20T11:52:28+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-002-golden-scenarios.txt` | 1273 | `c72777732373335a7e5d8a1bf510489e122496f2aac5714570740139887f56ce` |
| `cargo test --locked -p tracewake-core --test generative_lock` | `2026-06-20T11:52:33+02:00` to `2026-06-20T11:52:33+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-002-generative-lock.txt` | 419 | `d84810847b58359b2c26655aec9b3cbb95798751cb93942ecf98587f700381d0` |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | `2026-06-20T11:52:38+02:00` to `2026-06-20T11:52:38+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-002-event-schema-replay-gates.txt` | 2692 | `0711d1073bb568ebf3fca4742aeca7bda4b792c7c4db5324e3ed0e0ab6e2573d` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T11:52:43+02:00` to `2026-06-20T11:52:44+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-002-golden-fixtures-run.txt` | 3008 | `58218bfdd0ffa2ca31234c5869f6fdb6fd37b2b1687d07c2f738c6e6957bd8b8` |

### ORD-LIFE-02 command ledger

These commands were run for `0042ORDLIFCER-003` against commit `689d712de31dd96b798bbb1b99ce079f95a207a3` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts. Candidate-generation unit tests in `crates/tracewake-core/src/agent/generation.rs` were covered by the green `cargo test --workspace --locked` scaffold baseline; the targeted commands below are the integration evidence required by this ticket.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | `2026-06-20T11:54:55+02:00` to `2026-06-20T11:54:55+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-003-hidden-truth-gates.txt` | 1434 | `c372bc821c22530daef14401c12b9599948f51eb84cd891c42d7e4416c3753ba` |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | `2026-06-20T11:55:00+02:00` to `2026-06-20T11:55:01+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-003-acceptance-gates.txt` | 1102 | `e4d1a58a9270b85b1207621ef1dede7a787bf27a9b56e6631feca3981d6fff0c` |
| `cargo test --locked -p tracewake-core --test generative_lock` | `2026-06-20T11:55:06+02:00` to `2026-06-20T11:55:06+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-003-generative-lock.txt` | 419 | `e71a584cc1ea58adaa1a6ed0467a220afad808e9c879920b326e47d2ee8d1ee3` |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T11:55:14+02:00` to `2026-06-20T11:55:14+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-003-no-human-capstone.txt` | 343 | `f7cca53a3a0cf9e654433e531ab7a7c0eceeedd62441e00e53f5b82f195f85b9` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T11:55:22+02:00` to `2026-06-20T11:55:23+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-003-golden-fixtures-run.txt` | 3008 | `f3597867089f948ea34487873520f8614604de0cc33587e65c0191be2a92a802` |

### ORD-LIFE-03 command ledger

These commands were run for `0042ORDLIFCER-004` against commit `608f16c0729963f051ac13c23368b3537228029d` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts. Embedded unit tests in `agent/intention.rs` and `agent/decision.rs` were covered by the green `cargo test --workspace --locked` scaffold baseline.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T11:58:37+02:00` to `2026-06-20T11:58:38+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-004-no-human-capstone.txt` | 343 | `ca089e3f43af4319627d4d8c2c57f1e9351a023ab0ef33d6f9ef3707e09bb9ea` |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | `2026-06-20T11:58:48+02:00` to `2026-06-20T11:58:48+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-004-acceptance-gates.txt` | 1102 | `ce7f52ca91de13128c4a8d358ecca613f53438e18ad0b1f6ebce37bbf1718e67` |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | `2026-06-20T11:58:58+02:00` to `2026-06-20T11:58:58+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-004-event-schema-replay-gates.txt` | 2692 | `0bba3da5453280b8fa4ea1a279f5bdf58f8124b12c6ad717f6d098b1955bd418` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T11:59:24+02:00` to `2026-06-20T11:59:25+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-004-golden-fixtures-run.txt` | 3008 | `8a5000b37c0e6610017a93ce7be926cc8837ddeb85f0db0e9444d708ac1e67a4` |
| `cargo test --locked -p tracewake-tui --test embodied_flow` | `2026-06-20T11:59:32+02:00` to `2026-06-20T11:59:32+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-004-embodied-flow.txt` | 682 | `7742004884d470e69c4179f3321e95a0fb345bbaffd6c00b336f35327164d910` |

### ORD-LIFE-04 command ledger

These commands were run for `0042ORDLIFCER-005` against commit `9ae209d8a20060ff54bf4b8ede6b2b2c2121442a` plus the uncommitted test-only template-census edit and report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts. The first command is an added targeted unit proof for the ticket's enumerated `phase3a_routine_templates` family criterion; the remaining five are the ticket-required commands rerun after that tracked edit.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core phase3a_template_census_has_defeasible_machinery_for_each_family` | `2026-06-20T12:04:13+02:00` to `2026-06-20T12:04:16+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-template-census.txt` | 3245 | `f874c6ce1288bdf3f540feb912fa8b9f68d90cfc69112a1e655b069fd7bc1551` |
| `cargo test --locked -p tracewake-core --test generative_lock` | `2026-06-20T12:04:34+02:00` to `2026-06-20T12:04:34+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-generative-lock-post.txt` | 419 | `e71a584cc1ea58adaa1a6ed0467a220afad808e9c879920b326e47d2ee8d1ee3` |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | `2026-06-20T12:04:40+02:00` to `2026-06-20T12:04:40+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-golden-scenarios-post.txt` | 1273 | `18e8246cf31dbf4258e04895ccba5e72473aabac6f298af31d1c8307e5912f52` |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T12:04:49+02:00` to `2026-06-20T12:04:50+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-no-human-capstone-post.txt` | 343 | `f7cca53a3a0cf9e654433e531ab7a7c0eceeedd62441e00e53f5b82f195f85b9` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T12:04:56+02:00` to `2026-06-20T12:04:57+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-golden-fixtures-run-post.txt` | 3111 | `69080062ca12c6b69c77f76e03d5b4a78a4115e2d68a8bd3b334d0af31116f8e` |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | `2026-06-20T12:05:10+02:00` to `2026-06-20T12:05:10+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-005-event-schema-replay-gates-post.txt` | 2692 | `4e76e82b304633445c6ce01d5a6d6eb8a28bced772173a052675f415211a8bc9` |

### ORD-LIFE-05 command ledger

These commands were run for `0042ORDLIFCER-006` against commit `7665965ed44fc653ffbf61f784bfcd0d2379f6b1` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | `2026-06-20T12:08:02+02:00` to `2026-06-20T12:08:02+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-006-hidden-truth-gates.txt` | 1434 | `18214bffdce6185b49eaa480e7d823d8a7c3c24aa71e9750f459496c325b2bff` |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T12:08:18+02:00` to `2026-06-20T12:08:18+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-006-no-human-capstone.txt` | 343 | `f7cca53a3a0cf9e654433e531ab7a7c0eceeedd62441e00e53f5b82f195f85b9` |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | `2026-06-20T12:08:25+02:00` to `2026-06-20T12:08:25+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-006-acceptance-gates.txt` | 1102 | `25f97575dbd37e58ce57fc8816d37b10b74d6c7f8882d5251260e5af5714018c` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T12:08:31+02:00` to `2026-06-20T12:08:32+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-006-golden-fixtures-run.txt` | 3008 | `45086854ba93315d8448fcf4880062cfaf268ab6d344b58aa929d8c82ba8688a` |
| `cargo test --locked -p tracewake-core --test event_schema_replay_gates` | `2026-06-20T12:08:38+02:00` to `2026-06-20T12:08:38+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-006-event-schema-replay-gates.txt` | 2692 | `6323a4382023ce3ca8285686bafd81670489a7dad96b9de015c21dcd676376c8` |

### ORD-LIFE-06 command ledger

These commands were run for `0042ORDLIFCER-007` against commit `4a4245ec4192ebac19b141628f035ed69927d180` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test generative_lock` | `2026-06-20T12:13:19+02:00` to `2026-06-20T12:13:19+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-007-generative-lock.txt` | 419 | `e71a584cc1ea58adaa1a6ed0467a220afad808e9c879920b326e47d2ee8d1ee3` |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | `2026-06-20T12:13:24+02:00` to `2026-06-20T12:13:24+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-007-hidden-truth-gates.txt` | 1434 | `4eff75d221f9935d8b61c8deea984d3719822fd634fb82153298ad2c79a545ed` |
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T12:13:30+02:00` to `2026-06-20T12:13:31+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-007-no-human-capstone.txt` | 343 | `f7cca53a3a0cf9e654433e531ab7a7c0eceeedd62441e00e53f5b82f195f85b9` |
| `cargo test --locked -p tracewake-core --test golden_scenarios` | `2026-06-20T12:13:40+02:00` to `2026-06-20T12:13:40+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-007-golden-scenarios.txt` | 1273 | `9af5b1eaae17a746974eba0de9b1809b2ebafdadd1a62bcae07021baefed78ba` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T12:13:48+02:00` to `2026-06-20T12:13:49+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-007-golden-fixtures-run.txt` | 3008 | `470937860a45a0c9aa612ee6e745557fec296107e54eca79a3070ee575ee557e` |

### ORD-LIFE-07 command ledger

These commands were run for `0042ORDLIFCER-008` against commit `a16d658e0808b6509ae4460611314c2e12cbd6d0` plus the uncommitted report edits created by that ticket. Transcript files are `/tmp` evidence files and are not committed artifacts.

| Command | Run window | Exit | Transcript fingerprint scope | Transcript bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `cargo test --locked -p tracewake-core --test no_human_capstone` | `2026-06-20T12:17:51+02:00` to `2026-06-20T12:17:51+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-no-human-capstone.txt` | 343 | `a8912d26b6db69d4a44c6b57a81965194660bc10c4e284745b6f63d0854f7594` |
| `cargo test --locked -p tracewake-core --test hidden_truth_gates` | `2026-06-20T12:17:58+02:00` to `2026-06-20T12:17:58+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-hidden-truth-gates.txt` | 1434 | `d0dbbcc09d38c1c76a8fadb834cfdf97f57a90cbd9c10a7dd8f5cfe4a720cc0a` |
| `cargo test --locked -p tracewake-core --test acceptance_gates` | `2026-06-20T12:18:03+02:00` to `2026-06-20T12:18:04+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-acceptance-gates.txt` | 1102 | `530d6c8ce301e63c47c6ff61e043c18c7251089c88ac84c8893fc072da500d01` |
| `cargo test --locked -p tracewake-content --test golden_fixtures_run` | `2026-06-20T12:18:10+02:00` to `2026-06-20T12:18:11+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-golden-fixtures-run.txt` | 3008 | `5688a13e70b723b27ae41c6ad9be5a1d80d983f5d647ea57ae2a79d9faebdc1e` |
| `cargo test --locked -p tracewake-tui --test adversarial_gates` | `2026-06-20T12:18:19+02:00` to `2026-06-20T12:18:19+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-tui-adversarial-gates.txt` | 1496 | `b473f921fb46bd678f9bcaf2f46836fce2170d77617eed8caa2413c900205c87` |
| `cargo test --locked -p tracewake-tui --test tui_seam_conformance` | `2026-06-20T12:18:32+02:00` to `2026-06-20T12:18:32+02:00` | 0 | full captured stdout/stderr bytes in `/tmp/0042-008-tui-seam-conformance.txt` | 535 | `c4d3521a1267887f5185b2c7494e96273d8cabd8b0986c88219b7ac49269dce6` |

## Per-requirement acceptance evidence

Rows are initialized now and must be completed by `0042ORDLIFCER-016`. Until then, every row remains `pending` and cannot be cited as a certifying pass.

### ORD-LIFE-01 through ORD-LIFE-12

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `ORD-LIFE-01` | needs/accounting/event ledger | `0042-ORD01-LEDGER`, `0042-ORD01-NEGATIVE`, `0042-ORD01-REPLAY` | `pass` |
| `ORD-LIFE-02` | actor-known candidate generation | `0042-ORD02-CANDIDATES`, `0042-ORD02-HIDDEN-TRUTH`, `0042-ORD02-PROVENANCE` | `pass` |
| `ORD-LIFE-03` | intention lifecycle | `0042-ORD03-LIFECYCLE`, `0042-ORD03-POSSESSION`, `0042-ORD03-REPLAY-NEGATIVES` | `pass` |
| `ORD-LIFE-04` | routines/HTN/fallback | `0042-ORD04-TEMPLATE-CENSUS`, `0042-ORD04-ROUTINE-BEHAVIOR`, `0042-ORD04-REPLAY-NEGATIVES` | `pass` |
| `ORD-LIFE-05` | routine temporal premises | `0042-ORD05-PREMISE-PROVENANCE`, `0042-ORD05-SCHEDULER-NEGATIVE`, `0042-ORD05-STALE-REPLAY` | `pass` |
| `ORD-LIFE-06` | method selection/local planner | `0042-ORD06-GOAL-CENSUS`, `0042-ORD06-BUDGET-PROVENANCE`, `0042-ORD06-FALLBACK-NEGATIVES` | `pass` |
| `ORD-LIFE-07` | planner and decision trace/debug | `0042-ORD07-TRACE-COMPLETE`, `0042-ORD07-DEBUG-QUARANTINE`, `0042-ORD07-FEEDBACK-NEGATIVES` | `pass` |
| `ORD-LIFE-08` | ordinary actions/movement/durations | `pending` | `pending` |
| `ORD-LIFE-09` | no-human orchestration/metrics | `pending` | `pending` |
| `ORD-LIFE-10` | stuck diagnostics/no-progress | `pending` | `pending` |
| `ORD-LIFE-11` | scheduler/proposal ancestry | `pending` | `pending` |
| `ORD-LIFE-12` | replay-derived projections/phase lock | `pending` | `pending` |

### Ten live pass conditions

| Requirement | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `ORD-LIFE-PASS-01` | ordinary-life replay/event ancestry | `pending` | `pending` |
| `ORD-LIFE-PASS-02` | actor-known cognition | `pending` | `pending` |
| `ORD-LIFE-PASS-03` | durable intentions | `pending` | `pending` |
| `ORD-LIFE-PASS-04` | routine machinery | `pending` | `pending` |
| `ORD-LIFE-PASS-05` | bounded local planning | `pending` | `pending` |
| `ORD-LIFE-PASS-06` | ordinary action affordances | `pending` | `pending` |
| `ORD-LIFE-PASS-07` | no-human ordinary life | `pending` | `pending` |
| `ORD-LIFE-PASS-08` | stuck diagnostics | `pending` | `pending` |
| `ORD-LIFE-PASS-09` | no-direct-dispatch/proposal ancestry | `pending` | `pending` |
| `ORD-LIFE-PASS-10` | replay-derived metrics/projections | `pending` | `pending` |

### Seven mandatory fixture families

| Fixture family | Responsible layer | Evidence item IDs | Result from certifying evidence |
|---|---|---|---|
| `need-accounting-and-duration-ledger` | needs/actions/replay | `pending` | `pending` |
| `actor-known-candidate-hidden-truth` | agent/actor-known | `pending` | `pending` |
| `intention-lifecycle-possession-neutrality` | agent/intention/controller | `pending` | `pending` |
| `routine-template-htn-fallback` | content/agent/routine | `pending` | `pending` |
| `ordinary-action-affordance-movement-duration` | actions/pipeline/state | `pending` | `pending` |
| `no-human-progress-stuck-metrics` | scheduler/no-human/diagnostics | `pending` | `pending` |
| `replay-provenance-phase-lock` | replay/projections/reports | `pending` | `pending` |

## Evidence item ledger

### `0042-BASELINE-001`

- Evidence item ID: `0042-BASELINE-001`
- Requirement IDs: `0042ORDLIFCER-001`, clean baseline, `INV-018`
- Evidence status: `pass`
- Fingerprint scope: command transcript
- Evidence summary: clean §4.1 baseline commands all exited 0; command windows, transcript byte counts, and SHA-256 values are recorded in `Gates run`.
- Path under test and behavior witness:
  - path under test: workspace baseline and existing deterministic replay/golden fixture suites;
  - command/event/trigger/emitter/scheduler entry: Cargo commands listed in `Gates run`;
  - responsible layer: workspace tooling, replay suites, content validation suites, core and TUI gates;
  - accepted/rejected action or validation stage witnessed: not applicable to scaffold ticket;
  - live negative, mutation-style failure, or reason no negative is applicable: no new behavior claim is made by this item;
  - checked facts or invariants: the unmutated baseline is green before downstream ORD-LIFE evidence interpretation.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: not applicable to scaffold ticket;
  - replay artifact or serialized-log reference: workspace tests include replay/golden fixture suites; this item records command status, not per-fixture ancestry;
  - seed, randomness, content version, or ruleset version: existing tests under the pinned toolchain;
  - extraction/projection version: current workspace at `f7d8d666a8baa220b87d5e037e3eb50c8bf088c5`;
  - source provenance: local Git commit and Cargo lockfile/toolchain.
- Sampling/exhaustiveness scope: full §4.1 baseline command list from the spec.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for the scaffold/baseline prerequisite only; not counted as pass evidence for ORD-LIFE-01 through ORD-LIFE-12.

### `0042-ORD01-LEDGER`

- Evidence item ID: `0042-ORD01-LEDGER`
- Requirement IDs: `ORD-LIFE-01`, `ORD-LIFE-PASS-01`, `ORD-LIFE-PASS-08`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. The suite includes `sleep_spanning_window_boundary_charges_each_tick_once`, `wait_then_window_passive_charges_each_tick_once`, `no_human_need_ledger_has_no_duplicate_regime_charges`, and `episode_tamper_proration_poisons_replay`. The helper `assert_no_duplicate_need_regime_charges` independently expands `NeedDeltaApplied` payloads into `(actor_id, need_kind, tick)` rows for both `tick_delta` and body-exclusive `action_effect` regimes, then fails if any row has more than one charge.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-content/tests/golden_fixtures_run.rs`, fixtures `sleep_spanning_window_boundary_charges_each_tick_once_001`, `wait_then_window_passive_charges_each_tick_once_001`, `sleep_interrupted_by_severe_need_prorates_recovery_001`, `work_block_failed_then_sleep_succeeds_001`, and `no_human_day_001`;
  - command/event/trigger/emitter/scheduler entry: `run_no_human_day` emits ordinary-life events through the scheduler/action pipeline and observes `SleepStarted`, `SleepCompleted`, `NeedDeltaApplied`, and no-human day metrics;
  - responsible layer: `scheduler`, `action_validation`, `event_append`, `event_application`, `projection`, and `replay`;
  - accepted/rejected action or validation stage witnessed: accepted sleep/wait/work/no-human ordinary-life paths with positive duration and passive charge windows;
  - live negative, mutation-style failure, or reason no negative is applicable: duplicate row detection in `assert_no_duplicate_need_regime_charges` is a live semantic negative over the produced event log;
  - checked facts or invariants: one owner and one charge per actor/need/tick, no passive-window overlap across already-open body-exclusive durations, and event-backed threshold/metric inputs.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `NeedDeltaApplied` rows with payload keys `need_kind`, `cause_kind`, and `elapsed_ticks`; body-exclusive charges require an `EventCause::Event` pointing to `SleepStarted` or `WorkBlockStarted`;
  - replay artifact or serialized-log reference: replay and checksum assertions in the same content suite plus the ORD-LIFE-01 replay command below;
  - seed, randomness, content version, or ruleset version: committed golden fixture definitions and current workspace content registry;
  - extraction/projection version: tested workspace at `785d56758a00247284fb818ee72885405dc3760c` plus this report edit;
  - source provenance: committed fixture constructors and event log payload fields, not display text.
- Sampling/exhaustiveness scope: finite named fixture matrix required by spec §5 ORD-LIFE-01 plus integrated no-human corpus coverage in the content golden fixture suite.
- Pending or historical handling: none for this row.
- Certification use: counted as certifying pass for `ORD-LIFE-01`; capstone rows for live pass conditions and fixture-family aggregate coverage remain pending until `0042ORDLIFCER-016`.

### `0042-ORD01-NEGATIVE`

- Evidence item ID: `0042-ORD01-NEGATIVE`
- Requirement IDs: `ORD-LIFE-01`, `ORD-LIFE-PASS-08`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passed. The suite includes `duplicate_need_tick_charge_rejects_live_and_replay_001` and `duplicate_duration_terminal_poisons_rebuild_001`. The production paths also expose `DuplicateDurationTerminal` through `need_accounting.rs`, `events/apply.rs`, `replay/rebuild.rs`, and scheduler error mapping.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-core/tests/event_schema_replay_gates.rs`, `crates/tracewake-core/src/events/apply.rs`, `crates/tracewake-core/src/need_accounting.rs`, `crates/tracewake-core/src/replay/rebuild.rs`;
  - command/event/trigger/emitter/scheduler entry: tampered or duplicated `NeedDeltaApplied` and duration-terminal events;
  - responsible layer: `event_application` and `replay`, with scheduler reporting duplicate duration terminals before no-human scheduling;
  - accepted/rejected action or validation stage witnessed: duplicate charge/terminal payloads are rejected or poison rebuild rather than silently normalizing;
  - live negative, mutation-style failure, or reason no negative is applicable: duplicate need-tick charge and duplicate/conflicting duration-terminal negatives;
  - checked facts or invariants: no duplicate actor/need/tick charge, no duplicate duration terminal hiding behind a different proposal ID or closing cause, and no replay normalization of accounting defects.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: test-local duplicate `NeedDeltaApplied`, `WorkBlockStarted`, `WorkCompleted`, and `WorkFailed` events;
  - replay artifact or serialized-log reference: replay rebuild mismatch and typed replay/application errors;
  - seed, randomness, content version, or ruleset version: not applicable;
  - extraction/projection version: current event schema/replay implementation;
  - source provenance: event payload fields and typed event IDs.
- Sampling/exhaustiveness scope: finite adversarial cases required by ORD-LIFE-01, including duplicate need charge and duplicate terminal.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-01`; not an aggregate mutation substitute.

### `0042-ORD01-REPLAY`

- Evidence item ID: `0042-ORD01-REPLAY`
- Requirement IDs: `ORD-LIFE-01`, `ORD-LIFE-PASS-01`
- Evidence status: `pass`
- Fingerprint scope: command transcript.
- Evidence summary: `cargo test --locked -p tracewake-core --test no_human_capstone`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-core --test generative_lock` all passed. Together they cover no-human typed ancestry/replay, baseline replay/golden scenarios, and generated replay/metamorphic locks that include duplicate-charge guards.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-core/tests/no_human_capstone.rs`, `crates/tracewake-core/tests/golden_scenarios.rs`, `crates/tracewake-core/tests/generative_lock.rs`;
  - command/event/trigger/emitter/scheduler entry: no-human day run, replay rebuilds, generated ordinary-life sequences;
  - responsible layer: `scheduler`, `event_append`, `event_application`, `projection`, `replay`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: ordinary-life event sequences replay to matching state/metrics and generated sequences reject duplicate accounting keys;
  - live negative, mutation-style failure, or reason no negative is applicable: generated lock asserts duplicate need charge keys fail the test oracle;
  - checked facts or invariants: `INV-018` replay derivation and ORD-LIFE-01 event-before-derived-state.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: no-human day and generated event sequences;
  - replay artifact or serialized-log reference: clean replay checks in the named suites;
  - seed, randomness, content version, or ruleset version: generated lock seed coverage as implemented by the test suite;
  - extraction/projection version: current replay/projection code;
  - source provenance: committed test and fixture code.
- Sampling/exhaustiveness scope: named no-human/golden scenarios plus the committed generated sample set in `generative_lock`.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-01`; broader generated/metamorphic package remains pending until `0042ORDLIFCER-014`.

### `0042-ORD02-CANDIDATES`

- Evidence item ID: `0042-ORD02-CANDIDATES`
- Requirement IDs: `ORD-LIFE-02`, `ORD-LIFE-PASS-02`, `ORD-LIFE-PASS-07`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-core --test no_human_capstone`, and the `0042-BASELINE-001` workspace run passed. The content suite includes `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, which generates candidate goals from actor-known facts, records selected/rejected candidates, asserts selected `GoalKind::Eat`, and verifies hidden-truth audit results remain actor-known-only. The workspace baseline covers generation unit tests including `candidate_generation_is_deterministic`, `generated_candidates_carry_actor_known_fact_notes`, and `rising_hunger_adds_eat_candidate_without_erasing_active_intention`.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-core/src/agent/generation.rs`, `crates/tracewake-core/src/agent/decision.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`;
  - command/event/trigger/emitter/scheduler entry: `generate_candidate_goals`, `select_goal_and_trace`, no-human decision trace recording;
  - responsible layer: `candidate_generation`, `holder_known_context`, `intention_lifecycle`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: selected and rejected candidate goals are recorded in decision traces with actor-known inputs;
  - live negative, mutation-style failure, or reason no negative is applicable: generation unit tests prove hidden true food is not used without actor belief, while `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit` records an unproven source as not actor-known-only;
  - checked facts or invariants: deterministic candidate ordering, selected/rejected candidate traceability, and actor-known source notes.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: planner-trace fixture uses `event_planner_trace_food`; no-human capstone records decision trace actor-known inputs and context hashes;
  - replay artifact or serialized-log reference: no-human capstone and generated lock commands passed;
  - seed, randomness, content version, or ruleset version: committed fixture registry and content version in the content test harness;
  - extraction/projection version: current candidate generation and decision trace code;
  - source provenance: `ActorKnownFact::observed_now`, `SourceEventIds::checked`, and decision trace `actor_known_inputs`.
- Sampling/exhaustiveness scope: finite named fixture and unit-test set required for ORD-LIFE-02; broader generated/metamorphic package remains pending until `0042ORDLIFCER-014`.
- Pending or historical handling: none for this row.
- Certification use: counted as certifying pass for `ORD-LIFE-02`; capstone aggregate rows remain pending.

### `0042-ORD02-HIDDEN-TRUTH`

- Evidence item ID: `0042-ORD02-HIDDEN-TRUTH`
- Requirement IDs: `ORD-LIFE-02`, `ORD-LIFE-PASS-02`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. The content suite includes `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`, which loads `no_hidden_truth_planning_001`, verifies hidden physical food exists in authoritative state, generates hunger candidates with no actor-known food facts, selects `GoalKind::FindFood`, verifies planner inputs omit `food_hidden_pantry`, and rejects a direct plan to eat the hidden pantry food with `food source is not actor-known`.
- Path under test and behavior witness:
  - path under test: hidden-truth fixtures and actor-known context projection;
  - command/event/trigger/emitter/scheduler entry: candidate generation and local planning from a sealed actor-known context;
  - responsible layer: `holder_known_context` and `candidate_generation`;
  - accepted/rejected action or validation stage witnessed: hidden-food direct target is rejected before it can become a selected goal/proposal/event;
  - live negative, mutation-style failure, or reason no negative is applicable: hidden authoritative food and route/workplace truth are not admitted until legal source evidence exists;
  - checked facts or invariants: no telepathy, no validation-truth planning, no fixture-prose target synthesis.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: actor-known context proof sources and source-event-backed facts where present;
  - replay artifact or serialized-log reference: hidden-truth gates and content fixture replay paths passed;
  - seed, randomness, content version, or ruleset version: committed hidden-truth fixtures;
  - extraction/projection version: current actor-known projection/context builders;
  - source provenance: actor-known context facts, not authoritative hidden state.
- Sampling/exhaustiveness scope: finite hidden-truth fixture set named by the spec/ticket, including `no_hidden_truth_planning_001`, `hidden_food_unknown_route_001`, and related hidden food/workplace provenance cases covered by the content and hidden-truth suites.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-02`.

### `0042-ORD02-PROVENANCE`

- Evidence item ID: `0042-ORD02-PROVENANCE`
- Requirement IDs: `ORD-LIFE-02`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript.
- Evidence summary: `cargo test --locked -p tracewake-core --test acceptance_gates`, `cargo test --locked -p tracewake-core --test generative_lock`, and `cargo test --locked -p tracewake-core --test no_human_capstone` passed. The report also relies on the `0042-BASELINE-001` workspace run for embedded unit tests that source classes and candidate generation are deterministic.
- Path under test and behavior witness:
  - path under test: `ActorKnownInputRef`, `ActorKnownInputSourceClass`, decision trace serialization, generated ordinary-life sequences;
  - command/event/trigger/emitter/scheduler entry: acceptance gates, no-human decision trace events, and generated sequence replay;
  - responsible layer: `holder_known_context`, `candidate_generation`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: source context and actor-known input references are retained in traces; forbidden inputs remain excluded or marked non-actor-known-only;
  - live negative, mutation-style failure, or reason no negative is applicable: unproven source refs in planner trace produce a failed actor-known-only audit note rather than an admitted source;
  - checked facts or invariants: source class/source-event discipline, fail-closed provenance handling, same-input determinism.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: decision trace source-event IDs and no-human capstone decision trace records;
  - replay artifact or serialized-log reference: no-human capstone and generated lock passed;
  - seed, randomness, content version, or ruleset version: current test fixtures and generated lock seeds;
  - extraction/projection version: current decision trace and actor-known context hashing code;
  - source provenance: actor-known input refs and checked source-event IDs.
- Sampling/exhaustiveness scope: finite command set plus generation unit tests from the workspace baseline.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-02`; not counted as a substitute for the later mutation or capstone packages.

### `0042-ORD03-LIFECYCLE`

- Evidence item ID: `0042-ORD03-LIFECYCLE`
- Requirement IDs: `ORD-LIFE-03`, `ORD-LIFE-PASS-01`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test no_human_capstone`, and the `0042-BASELINE-001` workspace run passed. The workspace baseline covers `agent/intention.rs` unit tests for reason-bearing suspended/completed/failed/abandoned/interrupted transitions, illegal reactivation from completed state, suspended reactivation with reason, one active intention, and stable IDs for all variants. It also covers `agent/decision.rs` tests proving mild hunger continues an active intention while urgent/severe hunger interrupts it and adopts a new need-pressure intention with trace ancestry.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-core/src/agent/intention.rs`, `crates/tracewake-core/src/agent/decision.rs`, `crates/tracewake-core/tests/event_schema_replay_gates.rs`, `crates/tracewake-core/tests/no_human_capstone.rs`;
  - command/event/trigger/emitter/scheduler entry: intention-started, intention-transition, intention-continued, and no-human decision trace events;
  - responsible layer: `intention_lifecycle`, `event_append`, `event_application`, `replay`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: reason-bearing transitions apply only from legal source states; illegal statuses reject before state change;
  - live negative, mutation-style failure, or reason no negative is applicable: illegal transition tests and `intention_transition_wrong_status_rejects_before_state_change` reject wrong lifecycle paths;
  - checked facts or invariants: typed reasons, predecessor/successor ancestry, active/continued/interrupted/adopted lifecycle effects, and durable active-intention continuity under mild pressure.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: no-human capstone `assert_intention_and_routine_ancestry` checks intention/routine ancestry events; event-schema tests apply and replay intention payload matrices;
  - replay artifact or serialized-log reference: `event_schema_replay_gates` and `no_human_capstone` commands passed;
  - seed, randomness, content version, or ruleset version: current committed test fixtures and no-human corpus;
  - extraction/projection version: current agent lifecycle and replay code;
  - source provenance: typed event payload fields and decision trace IDs.
- Sampling/exhaustiveness scope: lifecycle enum variants and transition matrix covered by unit/workspace baseline plus named integration replay tests.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-03`; capstone aggregate rows remain pending.

### `0042-ORD03-POSSESSION`

- Evidence item ID: `0042-ORD03-POSSESSION`
- Requirement IDs: `ORD-LIFE-03`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-tui --test embodied_flow` passed. The content suite includes `possession_fixture_preserves_intention_needs_and_can_continue`, which loads `possession_does_not_reset_intention_001`, attaches and detaches `controller_human`, verifies `agent_state` is unchanged, sees `ControllerAttached` and `ControllerDetached`, then successfully proposes `continue_routine` with `active_intention_id=intention_mara_work`, `intention_status=active`, and `routine_execution_id=routine_exec_mara_work`.
- Path under test and behavior witness:
  - path under test: `ControllerBindings`, possession fixtures, content golden fixture runner, and TUI embodied flow;
  - command/event/trigger/emitter/scheduler entry: controller attach/detach plus possessed `continue_routine`;
  - responsible layer: `view_model`, `intention_lifecycle`, `event_application`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: possession changes controller binding and command authority but does not mutate agent needs, intentions, routine executions, memory, or actor-known state;
  - live negative, mutation-style failure, or reason no negative is applicable: equality assertions compare before/bind/detach/continue state; any reset or rewrite would fail the fixture;
  - checked facts or invariants: possession neutrality under `INV-006` and ordinary-action parity through the TUI boundary.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `ControllerAttached`, `ControllerDetached`, `ContinueRoutineProposed`, and continue-routine acceptance/rejection events;
  - replay artifact or serialized-log reference: possession replay tamper tests described in `0042-ORD03-REPLAY-NEGATIVES`;
  - seed, randomness, content version, or ruleset version: `possession_does_not_reset_intention_001` and `possession_parity_001`;
  - extraction/projection version: current controller, content, and TUI code;
  - source provenance: typed proposal parameters and controller binding events.
- Sampling/exhaustiveness scope: finite possession fixtures named by the spec plus TUI embodied-flow suite. Full EPI possession certification is consumed from predecessor artifacts and not re-audited here.
- Pending or historical handling: predecessor EPI possession guarantee is historical/scoped context only; this row counts only the live ORD-LIFE possession/intention evidence.
- Certification use: counted as certifying pass for `ORD-LIFE-03`.

### `0042-ORD03-REPLAY-NEGATIVES`

- Evidence item ID: `0042-ORD03-REPLAY-NEGATIVES`
- Requirement IDs: `ORD-LIFE-03`, `ORD-LIFE-PASS-01`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passed. The content suite includes `continue_routine_tamper_kind_flip_poisons_replay` and `continue_routine_tamper_reason_rewrite_poisons_replay`, both derived from `possession_continue_routine_replay_fixture`; tampering the continue-routine event kind or reason causes replay mismatch and agent checksum mismatch. Event schema tests include missing current-step rejection, transition status matrices, resume sequencing, and wrong-status rejection.
- Path under test and behavior witness:
  - path under test: continue-routine event replay, intention transition apply/rebuild, and agent checksum comparison;
  - command/event/trigger/emitter/scheduler entry: tampered `ContinueRoutine` event kind/reason and malformed intention transition payloads;
  - responsible layer: `event_application`, `replay`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: illegal or tampered lifecycle ancestry does not reconstruct as a plausible legal intention;
  - live negative, mutation-style failure, or reason no negative is applicable: tampered replay fails `matches_expected` and `agent_checksum_matches`; wrong-status transitions reject before state change;
  - checked facts or invariants: final-state equality cannot substitute for legal transition history.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `possession_continue_routine_replay_fixture` log, intention transition matrix events, and continue-routine arbitration records;
  - replay artifact or serialized-log reference: live vs tampered replay reports and agent checksum comparisons;
  - seed, randomness, content version, or ruleset version: committed fixture/test data;
  - extraction/projection version: current replay/rebuild implementation;
  - source provenance: typed event payloads, transition reasons, and event IDs.
- Sampling/exhaustiveness scope: finite lifecycle/adversarial cases named by ORD-LIFE-03.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-03`.

### `0042-ORD04-TEMPLATE-CENSUS`

- Evidence item ID: `0042-ORD04-TEMPLATE-CENSUS`
- Requirement IDs: `ORD-LIFE-04`, `ORD-LIFE-PASS-03`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: Added and ran `phase3a_template_census_has_defeasible_machinery_for_each_family`. It enumerates every current `phase3a_routine_templates()` family and fails if any family is missing, duplicated by count, lacks proposal steps, lacks interruptor checkpoints, lacks explicit failure modes, lacks fallback rules, or lacks a family trace/debug label.
- Path under test and behavior witness:
  - path under test: `crates/tracewake-core/src/agent/methods.rs`;
  - command/event/trigger/emitter/scheduler entry: static phase-3A template registry returned by `phase3a_routine_templates`;
  - responsible layer: `method_selection`, `content_schema`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: template registry structure, not a committed action;
  - live negative, mutation-style failure, or reason no negative is applicable: removing a required family or clearing steps/interruptors/failure modes/fallback/debug labels fails the unit census;
  - checked facts or invariants: routines expose defeasible machinery instead of only a label or happy-path step.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: not applicable to static template census;
  - replay artifact or serialized-log reference: behavior/replay rows below cover event execution;
  - seed, randomness, content version, or ruleset version: current in-code template registry;
  - extraction/projection version: current `methods.rs`;
  - source provenance: direct registry entries.
- Sampling/exhaustiveness scope: exhaustive over the ten current `phase3a_routine_templates()` members:
  - `routine_morning_wake` / `MorningWake`
  - `routine_eat_meal` / `EatMeal`
  - `routine_go_to_work` / `GoToWork`
  - `routine_work_block` / `WorkBlock`
  - `routine_return_home` / `ReturnHome`
  - `routine_sleep_night` / `SleepNight`
  - `routine_find_food` / `FindFood`
  - `routine_leave_unsafe_place` / `LeaveUnsafePlace`
  - `routine_continue_current_intention` / `ContinueCurrentIntention`
  - `routine_wait_idle` / `Wait`
- Pending or historical handling: none.
- Certification use: counted as certifying pass for the ORD-LIFE-04 template-census criterion; live reachability comes from `0042-ORD04-ROUTINE-BEHAVIOR`.

### `0042-ORD04-ROUTINE-BEHAVIOR`

- Evidence item ID: `0042-ORD04-ROUTINE-BEHAVIOR`
- Requirement IDs: `ORD-LIFE-04`, `ORD-LIFE-PASS-01`, `ORD-LIFE-PASS-03`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-core --test golden_scenarios`, and `cargo test --locked -p tracewake-core --test no_human_capstone` passed. The content suite includes `routine_blocked_fixture_records_access_failure_without_silent_loop`, `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, `routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry`, `severe_safety_with_known_exit_produces_move_and_replays`, and `work_block_failed_then_sleep_succeeds_fixture_closes_reservation`.
- Path under test and behavior witness:
  - path under test: routine/method selection, no-human day routines, content fixtures, action validation;
  - command/event/trigger/emitter/scheduler entry: work-block, move, wait, continue-routine, method selection, no-human scheduler;
  - responsible layer: `method_selection`, `local_planning`, `intention_lifecycle`, `event_application`, `fixture_contract`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: blocked work records `WorkBlockFailed` with `blocker_kind=access` instead of silent looping; no-teleport remote work records `WorkBlockFailed` without `ActorMoved` or `WorkBlockStarted`; severe safety with known exit commits an `ActorMoved` event and replays; method selection records selected/rejected method traces;
  - live negative, mutation-style failure, or reason no negative is applicable: routine blocked/no-teleport fixtures are live negatives for script dispatch and movement/affordance skipping;
  - checked facts or invariants: routine machinery includes applicability, selected/rejected methods, interruptor/failure/fallback outcomes, and no label-based primitive dispatch.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `WorkBlockFailed`, `ActorMoved`, `DecisionTraceRecorded`, routine step events, no-human routine events;
  - replay artifact or serialized-log reference: severe-safety fixture replays; no-human capstone compares final routine executions after replay;
  - seed, randomness, content version, or ruleset version: committed content fixtures and current action registry;
  - extraction/projection version: current routine/method and replay code;
  - source provenance: typed actor-known context and fixture contract fields.
- Sampling/exhaustiveness scope: finite behavior fixture set required by ORD-LIFE-04 plus integrated no-human routine corpus. The template census is exhaustive over families; behavior fixtures exercise representative positive/negative routine paths across work, movement, safety, method selection, fallback, and no-human routine execution.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-04`.

### `0042-ORD04-REPLAY-NEGATIVES`

- Evidence item ID: `0042-ORD04-REPLAY-NEGATIVES`
- Requirement IDs: `ORD-LIFE-04`, `ORD-LIFE-PASS-01`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test event_schema_replay_gates`, `cargo test --locked -p tracewake-core --test generative_lock`, and the content validation unit tests from `0042-BASELINE-001` passed. Event schema tests cover missing routine step progress tick live/replay rejection, routine execution status/fallback-attempt matrices, continue-routine arbitration records, checksum distinction for routine status, and routine-effect need cause handling. Content validation tests reject routine templates with missing failure modes, unknown fallback rules, direct state/script operations, bad tick order, and shortcut markers.
- Path under test and behavior witness:
  - path under test: routine event apply/replay, content routine validation, generated sequence replay locks;
  - command/event/trigger/emitter/scheduler entry: `RoutineStepStarted`, `RoutineStepCompleted`, `RoutineStepFailed`, `ContinueRoutineAccepted`, content validation;
  - responsible layer: `event_application`, `replay`, `content_validation`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: malformed routine progress and content shortcut attempts fail instead of producing plausible routine state;
  - live negative, mutation-style failure, or reason no negative is applicable: `phase3a_routine_structure_failures_are_rejected`, `phase3a_routine_typed_modes_windows_and_direct_ops_are_rejected`, and `phase3a_routine_shortcut_effects_are_rejected` were covered by the scaffold baseline;
  - checked facts or invariants: source shape alone is not certifying, and illegal routine histories/content cannot be normalized.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: routine step and continue-routine event matrices;
  - replay artifact or serialized-log reference: event schema replay gates and generated replay locks;
  - seed, randomness, content version, or ruleset version: current test fixtures and content validator;
  - extraction/projection version: current replay and content validation code;
  - source provenance: typed routine template, assignment, and event payload fields.
- Sampling/exhaustiveness scope: finite adversarial cases named by ORD-LIFE-04 plus content validation negative families.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-04`; not counted as a substitute for later mutation proof.

### `0042-ORD05-PREMISE-PROVENANCE`

- Evidence item ID: `0042-ORD05-PREMISE-PROVENANCE`
- Requirement IDs: `ORD-LIFE-05`, `ORD-LIFE-PASS-02`, `ORD-LIFE-PASS-03`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run`, `cargo test --locked -p tracewake-core --test hidden_truth_gates`, and `cargo test --locked -p tracewake-core --test no_human_capstone` passed. The live surfaces prove raw workplace assignment is not actor-known without a modeled notice, workplace facts become actor-known only from `RoleAssignmentNoticeRecorded` source events, no-human workplace knowledge requires a notice channel, and no-human decision traces cite source event IDs with rebuilt context hashes.
- Path under test and behavior witness:
  - path under test: `NoHumanActorKnownSurfaceBuilder`, actor decision transaction, hidden-truth gates, no-human decision trace records;
  - command/event/trigger/emitter/scheduler entry: role assignment notices, no-human day decision path, holder-known context projection;
  - responsible layer: `holder_known_context`, `candidate_generation`, `method_selection`, `scheduler`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: work/sleep routine premises are admitted only through source-backed facts; missing premise yields wait/stuck/failure rather than work planning;
  - live negative, mutation-style failure, or reason no negative is applicable: `raw_workplace_assignment_is_not_actor_known_without_notice` and `no_human_workplace_knowledge_requires_notice_channel` reject raw schedule/assignment truth as cognition authority;
  - checked facts or invariants: temporal premise provenance, source-event citation, and context-hash replay.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `RoleAssignmentNoticeRecorded`, decision trace actor-known inputs, source-event IDs;
  - replay artifact or serialized-log reference: no-human capstone and event schema replay gates passed;
  - seed, randomness, content version, or ruleset version: committed content fixtures and current epistemic projection code;
  - extraction/projection version: current no-human surface and actor decision transaction code;
  - source provenance: holder-known facts and source-event IDs, not scheduler windows.
- Sampling/exhaustiveness scope: finite workplace/sleep provenance fixtures and hidden-truth gates named by ORD-LIFE-05.
- Pending or historical handling: consolidated cross-seam temporal bundle is explicitly out of scope and routed to FIRST-PROOF-CERT; this row certifies only the ORD-LIFE routine-premise mechanism.
- Certification use: counted as certifying pass for `ORD-LIFE-05`.

### `0042-ORD05-SCHEDULER-NEGATIVE`

- Evidence item ID: `0042-ORD05-SCHEDULER-NEGATIVE`
- Requirement IDs: `ORD-LIFE-05`, `ORD-LIFE-PASS-02`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-core --test acceptance_gates` passed. The content suite includes `no_human_unseen_workplace_assignment_does_not_plan_work_001`, `no_human_workplace_knowledge_requires_notice_event_001`, `scheduler_cannot_rewrite_wait_reason_after_transaction_001`, and hidden-truth planning fixtures. Scheduler day-window metadata appears only as scheduler metadata; the actor-visible wait reason remains actor-decision provenance.
- Path under test and behavior witness:
  - path under test: scheduler to actor decision transaction boundary;
  - command/event/trigger/emitter/scheduler entry: no-human day windows, wait proposals, routine-window family input;
  - responsible layer: `scheduler`, `holder_known_context`, `candidate_generation`, `method_selection`;
  - accepted/rejected action or validation stage witnessed: objectively work-appropriate schedule position cannot create a work plan without actor-known notice/assignment evidence;
  - live negative, mutation-style failure, or reason no negative is applicable: actor waits or records a holder-known-context stuck diagnostic instead of planning work when modeled premise sources are absent;
  - checked facts or invariants: scheduler wake/day-window/elapsed tick is not temporal-premise provenance.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `ActorWaited`, `StuckDiagnosticRecorded`, scheduler no-human windows;
  - replay artifact or serialized-log reference: no-human capstone and event schema replay gates passed;
  - seed, randomness, content version, or ruleset version: committed no-human fixtures;
  - extraction/projection version: current scheduler and actor transaction code;
  - source provenance: actor-known context facts and absence thereof.
- Sampling/exhaustiveness scope: finite no-human schedule-negative fixtures named by ORD-LIFE-05.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-05`.

### `0042-ORD05-STALE-REPLAY`

- Evidence item ID: `0042-ORD05-STALE-REPLAY`
- Requirement IDs: `ORD-LIFE-05`, `ORD-LIFE-PASS-01`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed tests.
- Evidence summary: `cargo test --locked -p tracewake-content --test golden_fixtures_run` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passed. The content loader test `stale_workplace_notice_superseded_by_newer_001` and no-human surface tests prove newer notices supersede stale workplace notices before actor-known facts are minted. `provenance_class_mismatch_diagnostic` in the actor transaction emits a typed diagnostic when an `observed_now` fact is stale for the decision tick.
- Path under test and behavior witness:
  - path under test: epistemic projection, no-human surface, actor transaction, event schema replay;
  - command/event/trigger/emitter/scheduler entry: workplace notice supersession, stale fact classification, replay of source-event-backed decision context;
  - responsible layer: `holder_known_context`, `action_validation`, `replay`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: stale-premise classification remains distinct from validation truth, and stale `observed_now` provenance yields a typed diagnostic;
  - live negative, mutation-style failure, or reason no negative is applicable: stale facts are not repaired from ground truth; tampered decision trace source evidence causes replay/context-hash failure;
  - checked facts or invariants: stale is not false, supersession is modeled, and clean replay rebuilds premise source selection.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: workplace notice events, decision trace actor-known input source events, stale-after tick payloads;
  - replay artifact or serialized-log reference: `belief_stale_frontier_and_witness_links_survive_projection_debug_and_replay` and decision context hash tests in the event schema suite;
  - seed, randomness, content version, or ruleset version: committed fixtures and content version;
  - extraction/projection version: current epistemic projection and replay code;
  - source provenance: source-event IDs and freshness fields.
- Sampling/exhaustiveness scope: finite stale/supersession and replay provenance cases named by ORD-LIFE-05.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-05`.

### `0042-ORD06-GOAL-CENSUS`

- Evidence item ID: `0042-ORD06-GOAL-CENSUS`
- Requirement IDs: `ORD-LIFE-06`, `ORD-LIFE-PASS-03`, `ORD-LIFE-PASS-05`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed planner and fixture tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test generative_lock`, `cargo test --locked -p tracewake-core --test no_human_capstone`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. The supported `PlannerGoal` member list is `ReachPlace`, `CheckContainer`, `EatKnownFood`, `StartSleep`, `StartWorkBlock`, `LeaveUnsafePlace`, and `WaitWithReason`. The planner tests and content fixtures cover route/open/move proposals, known food proposals, sleep-at-current-known-place proposals, work-block-at-known-workplace proposals, unsafe-place leave via known route, and modeled waits; `CheckContainer` is a supported planner branch with an explicit `check_container` proposal path when a known container target is requested.
- Path under test and behavior witness:
  - path under test: `PlannerGoal`, `LocalPlanRequest`, `plan_local_actions`, `PlannedProposal`, `LocalPlanTrace`, actor-known context construction;
  - command/event/trigger/emitter/scheduler entry: generated candidates, selected method goal, local planner request, proposal construction;
  - responsible layer: `method_selection`, `local_planning`, `proposal_construction`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: planned route/eat/sleep/work/wait proposals remain proposal-based and reach ordinary validation rather than direct state mutation;
  - live negative, mutation-style failure, or reason no negative is applicable: unsupported or non-actor-known targets fail closed through typed planner failure rather than hidden search;
  - checked facts or invariants: every planner goal member is enumerated and local-plan output is derived from actor-known inputs.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: actor-known input refs, selected method trace, proposal source context, local-plan trace;
  - replay artifact or serialized-log reference: golden scenarios and no-human capstone passed;
  - seed, randomness, content version, or ruleset version: committed core tests and content fixtures;
  - extraction/projection version: current planner, decision, method-selection, and actor-known context code;
  - source provenance: actor-known fact provenance, proof sources, and selected method refs.
- Sampling/exhaustiveness scope: exhaustive live `PlannerGoal` enum member census, with finite fixture coverage for ordinary goal families.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-06`.

### `0042-ORD06-BUDGET-PROVENANCE`

- Evidence item ID: `0042-ORD06-BUDGET-PROVENANCE`
- Requirement IDs: `ORD-LIFE-06`, `ORD-LIFE-PASS-03`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed planner, hidden-truth, and transaction tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test generative_lock`, `cargo test --locked -p tracewake-core --test hidden_truth_gates`, and `cargo test --locked -p tracewake-core --test no_human_capstone` passed. `DEFAULT_PLANNER_BUDGET` is `8`; explicit budget tests use `1` and small local-planner boundaries to produce deterministic success or `PlannerBudgetExhausted`. `budget_exhaustion_reports_candidates_tried` verifies a route plan with budget `1` fails with `PlannerBudgetExhausted`, non-empty `candidates_tried`, and the same blocker recorded in the trace. `hidden_truth_audit_is_derived_from_fact_provenance` proves unproven actor-known facts mark the trace audit as not actor-known-only rather than being laundered into provenance.
- Path under test and behavior witness:
  - path under test: route planner expansion, budget guard, hidden-truth audit, `dangling_provenance_diagnostic`, stuck diagnostics;
  - command/event/trigger/emitter/scheduler entry: planner request budget, actor-known fact list, transaction failure-to-stuck path;
  - responsible layer: `local_planning`, `proposal_construction`, `action_validation`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: source-free or dangling provenance fails before proposal acceptance, and budget exhaustion produces a typed blocker;
  - live negative, mutation-style failure, or reason no negative is applicable: hidden food, hidden route edge, true workplace without assignment, and unproven wait provenance do not alter planning until modeled evidence changes;
  - checked facts or invariants: hard budget bound, actor-known-only provenance, source-event citation discipline.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `DecisionTraceRecorded`, `StuckDiagnosticRecorded`, local-plan trace blocker fields;
  - replay artifact or serialized-log reference: no-human capstone and golden scenario gates passed;
  - seed, randomness, content version, or ruleset version: committed planner and hidden-truth fixtures;
  - extraction/projection version: current planner, transaction, hidden-truth audit, and trace code;
  - source provenance: source-event-backed actor-known facts, rejected unproven facts, and blocker provenance.
- Sampling/exhaustiveness scope: finite budget-boundary tests plus hidden-truth negative families named by ORD-LIFE-06.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-06`.

### `0042-ORD06-FALLBACK-NEGATIVES`

- Evidence item ID: `0042-ORD06-FALLBACK-NEGATIVES`
- Requirement IDs: `ORD-LIFE-06`, `ORD-LIFE-PASS-05`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed content fixtures and hidden-truth tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. `food_unavailable_replan_001` records `EatFailed` with blocker kind `resource`, no `FoodConsumed`, and no `NeedDeltaApplied`; `routine_blocked_diagnostic_001` records `WorkBlockFailed` with blocker kind `access` and no `WorkBlockStarted`; `planner_trace_001` exposes selected method, rejected methods, actor-known proof sources, and an unproven-source negative; `method_fallback_requires_new_trace_or_stuck_001` is included in the golden fixture manifest and fixture run.
- Path under test and behavior witness:
  - path under test: method selection, local-plan failure, action validation, fixture runner, decision trace;
  - command/event/trigger/emitter/scheduler entry: food-unavailable, routine-blocked, planner-trace, and method-fallback fixtures;
  - responsible layer: `method_selection`, `local_planning`, `action_validation`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: failed primary methods yield typed failure/stuck/fallback evidence rather than silent success or loop;
  - live negative, mutation-style failure, or reason no negative is applicable: empty/no-applicable/unsupported/failing branches are represented by trace or typed failure status;
  - checked facts or invariants: coherent fallback, rejected-method trace, no hidden truth in proposal construction.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `EatFailed`, `WorkBlockFailed`, selected/rejected method traces, stuck diagnostic records;
  - replay artifact or serialized-log reference: golden scenarios and content golden fixtures passed;
  - seed, randomness, content version, or ruleset version: committed fixture manifests and `ContentVersion::new("content_v1")` fixture loads where applicable;
  - extraction/projection version: current content loader, method selection, planner, and action validation code;
  - source provenance: actor-known state proof sources and fixture manifest IDs.
- Sampling/exhaustiveness scope: mandatory ORD-LIFE-06 finite fixture families: Food unavailable, Hidden-truth planning, Planner trace, Routine blocker.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-06`.

### `0042-ORD07-TRACE-COMPLETE`

- Evidence item ID: `0042-ORD07-TRACE-COMPLETE`
- Requirement IDs: `ORD-LIFE-07`, `ORD-LIFE-PASS-07`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed planner-trace and no-human tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test no_human_capstone`, `cargo test --locked -p tracewake-core --test acceptance_gates`, and `cargo test --locked -p tracewake-content --test golden_fixtures_run` passed. `planner_trace_001` verifies generated candidate count, selected `GoalKind::Eat`, rejected goals, selected method ID, rejected methods, hidden-truth audit actor-known-only state, and actor-known proof sources. No-human decision traces record actor-known inputs and hidden-truth audit fields.
- Path under test and behavior witness:
  - path under test: `DecisionTrace`, `LocalPlanTrace`, method-selection trace, decision trace records, no-human trace event payloads;
  - command/event/trigger/emitter/scheduler entry: candidate generation, selected/rejected goals and methods, planner request, proposal or stuck result;
  - responsible layer: `candidate_generation`, `local_planning`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: a plausible final action is not treated as sufficient unless rejected alternatives, budget/blocker fields, and actor-known inputs are present;
  - live negative, mutation-style failure, or reason no negative is applicable: unproven actor-known planner input flips the hidden-truth audit away from actor-known-only;
  - checked facts or invariants: trace completeness, actor-known input refs, selected/rejected alternatives, budget/failure fields.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: `DecisionTraceRecorded`, no-human trace payloads, planner trace records;
  - replay artifact or serialized-log reference: no-human capstone and acceptance gates passed;
  - seed, randomness, content version, or ruleset version: committed `planner_trace_001` and no-human fixtures;
  - extraction/projection version: current decision, method, planner, and trace code;
  - source provenance: actor-known input refs and source-event-backed facts.
- Sampling/exhaustiveness scope: finite planner-trace and no-human trace fixture families named by ORD-LIFE-07.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-07`.

### `0042-ORD07-DEBUG-QUARANTINE`

- Evidence item ID: `0042-ORD07-DEBUG-QUARANTINE`
- Requirement IDs: `ORD-LIFE-07`, `ORD-LIFE-PASS-07`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed TUI adversarial tests.
- Evidence summary: `cargo test --locked -p tracewake-tui --test adversarial_gates` and `cargo test --locked -p tracewake-tui --test tui_seam_conformance` passed. `adversarial_gates_debug_truth_does_not_enter_actor_surfaces` renders debug item location, projection, epistemics, planner, and replay panels, verifies `DEBUG NON-DIEGETIC` markers, verifies the physical checksum is unchanged, and proves actor surfaces omit hidden pantry/debug strings. `debug_panel_does_not_change_embodied_affordances` verifies semantic actions, holder-known context ID/hash/frontier/source summary, and physical checksum are unchanged after debug panels are rendered.
- Path under test and behavior witness:
  - path under test: TUI debug panels, core debug views/reports, embodied view rendering, transcript rendering;
  - command/event/trigger/emitter/scheduler entry: debug render/read path and current embodied view path;
  - responsible layer: `debug_quarantine`, `view_model`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: debug rendering is read-only and cannot become an action/proposal/event;
  - live negative, mutation-style failure, or reason no negative is applicable: debug truth never appears in semantic actions, notebook, actor view, holder-known context source summary, or physical checksum;
  - checked facts or invariants: debug-on/read/non-read non-interference for actor surfaces and state fingerprints.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: no accepted event is emitted by debug reads; event count/checksum remains unchanged where tested;
  - replay artifact or serialized-log reference: TUI seam conformance maps runtime negatives and replay determinism tests to named evidence;
  - seed, randomness, content version, or ruleset version: `debug_omniscience_excluded_001` and representative transcript fixtures;
  - extraction/projection version: current TUI app, debug panels, render, transcript, and core debug view code;
  - source provenance: authorized debug capability path and holder-known context metadata.
- Sampling/exhaustiveness scope: finite TUI debug quarantine tests named by ORD-LIFE-07.
- Pending or historical handling: EPI-CERT debug-capability authority is consumed, not re-audited here.
- Certification use: counted as certifying pass for `ORD-LIFE-07`.

### `0042-ORD07-FEEDBACK-NEGATIVES`

- Evidence item ID: `0042-ORD07-FEEDBACK-NEGATIVES`
- Requirement IDs: `ORD-LIFE-07`, `ORD-LIFE-PASS-07`, `ORD-LIFE-PASS-09`
- Evidence status: `pass`
- Fingerprint scope: command transcript plus parsed semantic content from committed hidden-truth, content, and TUI tests.
- Evidence summary: `cargo test --locked -p tracewake-core --test hidden_truth_gates`, `cargo test --locked -p tracewake-content --test golden_fixtures_run`, and the two TUI gates passed. `debug_omniscience_facts_are_excluded_from_planner_context` proves `debug_omniscience` is absent from proof sources and unproven debug-hidden-food input fails the actor-known audit. `debug_command_strings_are_not_embodied_commands` renders a debug item command, rejects `do debug.item.food_hidden_pantry` as no current action, and verifies semantic actions, context ID/hash/frontier, physical checksum, and event count are unchanged. `tui_epistemic_debug_uses_core_builder_not_raw_projection_storage` forbids direct raw projection storage reads and writes in the TUI debug path.
- Path under test and behavior witness:
  - path under test: actor-known proof-source audit, command loop, semantic action submission, TUI seam conformance;
  - command/event/trigger/emitter/scheduler entry: debug command string, hidden-truth audit, unproven actor-known fact, TUI debug epistemic view;
  - responsible layer: `debug_quarantine`, `candidate_generation`, `local_planning`, `test_oracle`;
  - accepted/rejected action or validation stage witnessed: prior trace/debug/rendered strings and hidden-truth debug tokens are rejected or kept outside current embodied commands;
  - live negative, mutation-style failure, or reason no negative is applicable: debug-only strings cannot enter actor-known packet, planning, proposal, metrics, embodied view, or future memory;
  - checked facts or invariants: no trace-to-cognition feedback and typed/API boundaries fail closed.
- Replay/provenance ancestry:
  - event-log segment or event identifiers: rejected semantic action path with unchanged event count/checksum;
  - replay artifact or serialized-log reference: TUI seam conformance and content fixture gates passed;
  - seed, randomness, content version, or ruleset version: `debug_omniscience_excluded_001`, `debug_attach_001`, and `hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001` are in the fixture registry and golden fixture run;
  - extraction/projection version: current actor-known audit, TUI command loop, and debug seam code;
  - source provenance: absence from holder-known proof sources plus explicit debug-only markers.
- Sampling/exhaustiveness scope: mandatory ORD-LIFE-07 fixture families: Planner trace and Hidden-truth planning, plus TUI debug negative suite.
- Pending or historical handling: none.
- Certification use: counted as certifying pass for `ORD-LIFE-07`.

## ORD-LIFE-01: bounded event-sourced needs, single-owner accounting, and single-charge ledgers

Result: `pass` for the ORD-LIFE-01 local audit point.

Positive witnesses:

- `sleep_spanning_window_boundary_charges_each_tick_once_001`: no-human execution starts and completes sleep, then `assert_no_duplicate_need_regime_charges` proves no duplicate `(actor, need, tick)` charge rows. The test also rejects a morning passive `tick_delta` charge across the already-open sleep window, proving the passive window does not second-charge body-exclusive duration ticks.
- `wait_then_window_passive_charges_each_tick_once_001`: modeled wait plus later passive window expands `tick_delta` charges with positive elapsed ticks and proves no duplicate rows.
- `sleep_interrupted_by_severe_need_prorates_recovery_001`: interruption/proration is replay-sensitive; tampering `SleepInterrupted` recovery payload breaks replay/agent checksum matching.
- `work_block_failed_then_sleep_succeeds_001`: failed work closes reservation before later sleep, proving failure/completion paths do not leave overlapping body-exclusive accounting ownership.
- Integrated `no_human_day_001`: multi-actor no-human run passes `assert_no_duplicate_need_regime_charges` across Anna, Elena, Mara, and Tomas.

Adversarial and responsible-layer witnesses:

- Duplicate need-tick charges are rejected live and in replay by `duplicate_need_tick_charge_rejects_live_and_replay_001`; responsible layer: `event_application` / `replay`.
- Duplicate or conflicting duration terminals poison rebuild through `duplicate_duration_terminal_poisons_rebuild_001` and the typed `DuplicateDurationTerminal` path; responsible layer: `event_application`, `replay`, and scheduler preflight reporting.
- Generated ordinary-life sequences in `generative_lock` include duplicate need charge key assertions, so generated event sequences cannot hide a second charge behind golden normalization.

Ownership/delegation boundary:

- The action pipeline and ordinary-life event application own accepted need deltas and duration terminals. Scheduler code may construct passive need-event specs and report duplicate-terminal preflight errors, but the certifying tests prove downstream scheduler/planner/projection/replay/golden paths consume event-backed rows rather than independently reconciling or synthesizing charges. Stable golden bytes are not used as the pass surface; semantic event payload expansion is.

## ORD-LIFE-02: actor-known candidate generation, deterministic priority, and hidden-target exclusion

Result: `pass` for the ORD-LIFE-02 local audit point.

Positive witnesses:

- Candidate generation and selection are traced from actor-known facts in `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`; selected and rejected candidates are present, selection chooses `GoalKind::Eat` only with actor-known food-source evidence, and the hidden-truth audit remains actor-known-only.
- The scaffold baseline's full workspace test run covered `agent/generation.rs` unit tests for deterministic generation, candidate source notes, hunger pressure without erasing active intention, and hidden true food exclusion without actor belief.
- No-human capstone evidence records actor-known context and candidate arbitration fields in the no-human decision trace path.

Adversarial and fail-closed witnesses:

- `no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs` proves authoritative hidden food exists while the selected goal remains `FindFood`, planner inputs omit `food_hidden_pantry`, actor-known food sources exclude it, and a direct plan to eat it fails with `food source is not actor-known`.
- Hidden-truth gate commands passed for actor-known context unforgeability, debug truth exclusion, hidden food/route exclusion, context injection rejection, and workplace provenance requirements.
- Provenance negatives with unproven actor-known facts are surfaced as failed actor-known-only audit notes, not admitted as valid source refs.

Projection/replay note:

- Candidate generation itself has no dedicated projection field that certifies target selection by path existence alone. The certifying surface is the typed candidate/decision trace output from identical actor-known/event inputs plus no-human/replay suites that preserve the trace and context hashes. This avoids replacing relational hidden-truth proof with string or golden-byte scans.

## ORD-LIFE-03: durable intention lifecycle, typed ancestry, replacement semantics, and possession neutrality

Result: `pass` for the ORD-LIFE-03 local audit point.

Lifecycle witnesses:

- Unit and workspace baseline coverage prove all `IntentionStatus` variants have stable IDs and terminal semantics, every non-active transition records a reason, suspended intentions can reactivate with a reason, completed intentions cannot reactivate or fail without fresh adoption, and `ActorIntentions` permits at most one active intention.
- Decision lifecycle tests prove mild hunger continues an active work intention, while urgent/severe hunger interrupts it with a typed reason and adopts a new need-pressure intention whose `trace_ancestry` points at the selected decision trace.
- Event-schema replay gates apply intention started/transition/continued matrices, reject missing current-step continuation live and in replay, and reject wrong-status transitions before state change.
- No-human capstone checks intention and routine ancestry and proves replay preserves final agent intentions.

Possession witnesses:

- `possession_fixture_preserves_intention_needs_and_can_continue` proves attach/detach events occur without changing agent state, intentions, routine executions, or needs, and that `continue_routine` remains valid afterward with the same active intention and routine execution.
- `possession_parity_001` and TUI `embodied_flow` preserve ordinary embodied action behavior across the controller/view boundary. This row consumes, but does not re-audit, predecessor EPI possession certification.

Replay/adversarial witnesses:

- `continue_routine_tamper_kind_flip_poisons_replay` and `continue_routine_tamper_reason_rewrite_poisons_replay` prove replay is sensitive to lifecycle event kind and reason ancestry.
- Event-schema lifecycle negatives prove illegal status transitions and malformed lifecycle references fail loudly rather than being inferred from the final intention state.

## ORD-LIFE-04: defeasible routine templates, HTN method families, interruptors, failure modes, and fallback

Result: `pass` for the ORD-LIFE-04 local audit point.

Template census:

- Current `phase3a_routine_templates()` members are `MorningWake`, `EatMeal`, `GoToWork`, `WorkBlock`, `ReturnHome`, `SleepNight`, `FindFood`, `LeaveUnsafePlace`, `ContinueCurrentIntention`, and `Wait`.
- The added census test proves each member has at least one proposal step, a non-empty interruptor checkpoint list, explicit failure modes, fallback rules, and a `phase3a_<family>` debug/trace label.
- The existing `all_required_families_have_proposal_steps`, `find_food_method_is_actor_known_only`, `leave_unsafe_place_uses_flight_family_and_move_step`, and proposal-step validation tests continue to cover proposal shape and actor-known/no-hidden-truth method constraints.

Behavior witnesses:

- `routine_blocked_fixture_records_access_failure_without_silent_loop` proves blocked workplace access records `WorkBlockFailed` with typed `blocker_kind=access` and no silent loop/start.
- `routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry` proves a remote work routine cannot skip movement ancestry or affordance validation.
- `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit` proves selected and rejected routine methods are traced, actor-known-only, and fail closed on unproven food-source facts.
- `severe_safety_with_known_exit_produces_move_and_replays` proves the severe-safety interruptor/flight family commits a causal move and replays.
- `work_block_failed_then_sleep_succeeds_fixture_closes_reservation` proves failure closes the work reservation so a later routine can proceed.

Replay/adversarial witnesses:

- Event schema replay gates cover routine step apply/replay matrices, fallback attempts, routine-effect cause handling, continue-routine arbitration, and missing progress tick rejection.
- Content validation negatives reject missing failure modes, unknown fallback rules, direct state/script operations, bad tick order, shortcut markers, and invalid sleep routine surface/diagnostic shape.

## ORD-LIFE-05: routine temporal premises and modeled adaptation without ground-truth schedule cognition

Result: `pass` for the ORD-LIFE-05 local audit point.

Temporal-premise witnesses:

- Raw workplace assignment is not actor-known without modeled notice; workplace knowledge is minted only from role/notice source events.
- `no_human_workplace_knowledge_requires_notice_channel` proves a no-human actor with missing modeled workplace premise performs no work event and records a holder-known-context stuck diagnostic.
- `ordinary_workday_001`, `workplace_assignment_provenance_001`, and `no_human_known_workplace_requires_provenance_001` are included in the content golden fixture suite and remain scoped to premise/source provenance rather than schedule truth.

Scheduler-boundary negatives:

- `no_human_unseen_workplace_assignment_does_not_plan_work_001` and related hidden-truth fixtures prove an objectively appropriate schedule tick/window cannot make an unobserved workplace into a candidate or method premise.
- `scheduler_cannot_rewrite_wait_reason_after_transaction_001` proves the scheduler window ID remains metadata and cannot rewrite the sealed actor decision transaction's wait reason.
- `routine_window_family` may supply a candidate hint only when actor-known facts support the corresponding premise; scheduler input is not recorded as actor-known provenance.

Staleness/replay witnesses:

- `stale_workplace_notice_superseded_by_newer_001` and no-human surface tests prove newer workplace notices supersede stale notices before actor-known facts are minted.
- Stale `observed_now` facts at a later decision tick produce a typed `ProvenanceClassMismatch` diagnostic with `input_source=actor_known_context`.
- Event-schema replay gates preserve stale frontier/witness links and fail decision context hashes when source evidence is tampered.

## ORD-LIFE-06: actor-known method selection, bounded local planning, planner-budget discipline, and coherent fallback

Result: `pass` for the ORD-LIFE-06 local audit point.

PlannerGoal member census:

- The live supported `PlannerGoal` members are `ReachPlace`, `CheckContainer`, `EatKnownFood`, `StartSleep`, `StartWorkBlock`, `LeaveUnsafePlace`, and `WaitWithReason`.
- Route planning emits local open/move proposals over actor-known edges and records candidates tried; known food, sleep, work, and wait goals emit proposal records only from actor-known targets or modeled reasons.
- `CheckContainer` remains a supported local planner branch for opening a known container target; unsupported or unknown targets fail as local-plan failures rather than falling through to world search.

Budget and provenance witnesses:

- `DEFAULT_PLANNER_BUDGET` is `8`; explicit budget-boundary tests produce deterministic success or `PlannerBudgetExhausted`.
- `budget_exhaustion_reports_candidates_tried` verifies budget `1` on a route plan records non-empty candidates tried and the `PlannerBudgetExhausted` blocker in the trace.
- Hidden food, hidden route edges, unknown workplace targets, remote sleep targets, and unproven actor-known facts do not become proposal provenance; they fail with typed knowledge/resource blockers or a hidden-truth audit failure before proposal acceptance.

Fallback and validation witnesses:

- `food_unavailable_replan_001` records `EatFailed` with resource blocker, no `FoodConsumed`, and no `NeedDeltaApplied`.
- `routine_blocked_diagnostic_001` records `WorkBlockFailed` with access blocker and no `WorkBlockStarted`.
- `planner_trace_001` records selected and rejected methods plus actor-known proof sources; `method_fallback_requires_new_trace_or_stuck_001` is in the golden fixture manifest and passed in the content fixture run.

## ORD-LIFE-07: planner and decision trace honesty, rejected alternatives, and debug quarantine

Result: `pass` for the ORD-LIFE-07 local audit point.

Trace-completeness witnesses:

- `planner_trace_001` records candidate count, selected and rejected goals, selected and rejected methods, actor-known proof sources, and hidden-truth audit state.
- No-human capstone trace records carry actor-known inputs and hidden-truth audit payload fields.
- Unproven planner input produces an actor-known audit failure rather than a trace that looks complete by final action alone.

Debug quarantine witnesses:

- TUI adversarial gates render debug item-location, projection, epistemics, planner, replay, and transcript sections with `DEBUG NON-DIEGETIC` markers.
- Reading debug panels leaves semantic actions, holder-known context ID/hash/frontier/source summary, physical checksum, and event count unchanged.
- Actor-facing embodied view, notebook, why-not, semantic actions, and transcript embodied sections omit debug-only hidden truth.

Feedback-negative witnesses:

- `debug_omniscience` is absent from actor-known proof sources, and unproven debug-hidden-food facts fail actor-known audit.
- `do debug.item.food_hidden_pantry` is rejected as no current action and does not change semantic actions, context, checksum, or event count.
- TUI seam conformance forbids raw projection storage access in the debug path and maps debug non-interference to named runtime/static evidence.

## ORD-LIFE-08: ordinary action affordances, causal movement, durations, terminals, and no-teleport behavior

Pending; owned by `0042ORDLIFCER-009`.

## ORD-LIFE-09: no-human orchestration, canonical recovery, meaningful progress, and metric honesty

Pending; owned by `0042ORDLIFCER-010`.

## ORD-LIFE-10: typed stuck diagnostics, blocker taxonomy, and cross-tick no-progress detection

Pending; owned by `0042ORDLIFCER-011`.

## ORD-LIFE-11: scheduler no-direct-dispatch, sealed proposal ancestry, and forged/stale validation rejection

Pending; owned by `0042ORDLIFCER-012`.

## ORD-LIFE-12: deterministic replay-derived ordinary-life projections, metrics, diagnostics, and phase-entry lock

Pending; owned by `0042ORDLIFCER-013`.

## Generated and metamorphic evidence package

Pending; owned by `0042ORDLIFCER-014`.

## Mutation package

Pending; owned by `0042ORDLIFCER-015`.

## Staged-abstraction declaration

Pending; owned by `0042ORDLIFCER-016`.

## Residual convention-only items

Pending; owned by `0042ORDLIFCER-016`.

## EMERGE-OBS handling

Pending; owned by `0042ORDLIFCER-016`. Any observer-only material included here must remain non-gating and cannot become a phase gate, pass/fail threshold, scheduler objective, scenario goal, mutation substitute, or code-quality score.

## Aggregate verdict

Pending; owned by `0042ORDLIFCER-016`. `ORD-LIFE-CERT passed` may be rendered only if every condition in spec §9.11 is satisfied by certifying evidence; otherwise the result must be `ORD-LIFE-CERT scoped remediation` naming the failed rows, evidence gaps, mutation survivors, and remediation route.

## Scoped certification wording

Current wording: `ORD-LIFE-CERT pending for exact tested commit f7d8d666a8baa220b87d5e037e3eb50c8bf088c5; this scaffold only records the clean baseline and report structure. It does not certify ORD-LIFE-CERT, latest main, FIRST-PROOF-CERT, PHASE-4-ENTRY, SECOND-PROOF-ENTRY, or the full project.`
