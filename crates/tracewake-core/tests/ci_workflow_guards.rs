use std::collections::BTreeSet;

const CI_YML: &str = include_str!("../../../.github/workflows/ci.yml");
const MUTANTS_TOML: &str = include_str!("../../../.cargo/mutants.toml");
const DOC10: &str = include_str!(
    "../../../docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md"
);

const REQUIRED_GATE_COMMANDS: &[&str] = &[
    "cargo fmt --all --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo build --workspace --all-targets --locked",
    "cargo test --workspace --locked",
];

const STANDING_MUTATION_PERIMETER: &[&str] = &[
    "crates/tracewake-core/src/agent/**",
    "crates/tracewake-core/src/need_accounting.rs",
    "crates/tracewake-core/src/scheduler.rs",
    "crates/tracewake-core/src/time.rs",
    "crates/tracewake-core/src/projections.rs",
    "crates/tracewake-core/src/actions/pipeline.rs",
    "crates/tracewake-core/src/actions/registry.rs",
    "crates/tracewake-core/src/actions/defs/need_events.rs",
    "crates/tracewake-core/src/actions/defs/eat.rs",
    "crates/tracewake-core/src/actions/defs/sleep.rs",
    "crates/tracewake-core/src/actions/defs/work.rs",
    "crates/tracewake-core/src/actions/defs/wait.rs",
    "crates/tracewake-core/src/actions/defs/continue_routine.rs",
    "crates/tracewake-core/src/actions/defs/movement.rs",
    "crates/tracewake-core/src/actions/defs/checkcontainer.rs",
    "crates/tracewake-core/src/events/**",
    "crates/tracewake-core/src/replay/**",
    "crates/tracewake-core/src/checksum.rs",
    "crates/tracewake-core/src/state.rs",
    "crates/tracewake-core/src/actions/proposal.rs",
    "crates/tracewake-core/src/actions/report.rs",
    "crates/tracewake-core/src/view_models.rs",
    "crates/tracewake-core/src/debug_capability.rs",
    "crates/tracewake-core/src/controller.rs",
    "crates/tracewake-core/src/debug_reports.rs",
    "crates/tracewake-core/src/epistemics/**",
    "crates/tracewake-core/src/epistemics/knowledge_context.rs",
    "crates/tracewake-core/src/epistemics/projection.rs",
    "crates/tracewake-content/src/manifest.rs",
    "crates/tracewake-content/src/load.rs",
    "crates/tracewake-content/src/schema.rs",
    "crates/tracewake-content/src/serialization.rs",
    "crates/tracewake-content/src/validate.rs",
    "crates/tracewake-tui/src/app.rs",
    "crates/tracewake-tui/src/debug_panels.rs",
    "crates/tracewake-tui/src/render.rs",
    "crates/tracewake-tui/src/transcript.rs",
];

const STANDING_MUTATION_TRIGGER_FRAGMENTS: &[&str] = &[
    "crates/tracewake-core/src/agent/",
    "crates/tracewake-core/src/need_accounting\\.rs",
    "crates/tracewake-core/src/scheduler\\.rs",
    "crates/tracewake-core/src/time\\.rs",
    "crates/tracewake-core/src/projections\\.rs",
    "crates/tracewake-core/src/actions/pipeline\\.rs",
    "crates/tracewake-core/src/actions/registry\\.rs",
    "crates/tracewake-core/src/actions/defs/(need_events|eat|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs",
    "crates/tracewake-core/src/events/",
    "crates/tracewake-core/src/replay/",
    "crates/tracewake-core/src/checksum\\.rs",
    "crates/tracewake-core/src/state\\.rs",
    "crates/tracewake-core/src/actions/(proposal|report)\\.rs",
    "crates/tracewake-core/src/view_models\\.rs",
    "crates/tracewake-core/src/debug_capability\\.rs",
    "crates/tracewake-core/src/controller\\.rs",
    "crates/tracewake-core/src/debug_reports\\.rs",
    "crates/tracewake-core/src/epistemics/",
    "crates/tracewake-content/src/(manifest|load|schema|serialization|validate)\\.rs",
    "crates/tracewake-tui/src/(app|debug_panels|render|transcript)\\.rs",
];

#[test]
fn ci_workflow_guards_cover_workflow_integrity() {
    let errors = ci_workflow_guard_errors(CI_YML, MUTANTS_TOML, DOC10);
    assert!(
        errors.is_empty(),
        "CI workflow guard failures:\n{}",
        errors.join("\n")
    );

    let masked_gate = CI_YML.replace(
        "run: cargo fmt --all --check",
        "run: cargo fmt --all --check || true",
    );
    assert!(
        ci_workflow_guard_errors(&masked_gate, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("masked gate command")),
        "synthetic masked gate step must fail"
    );

    let unpinned_third_party = format!("{CI_YML}\n      - uses: docker/login-action@v3\n");
    assert!(
        ci_workflow_guard_errors(&unpinned_third_party, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("non-actions use is not SHA-pinned")),
        "synthetic unpinned third-party action must fail"
    );

    let missing_permissions = CI_YML.replace("permissions:\n  contents: read\n\n", "");
    assert!(
        ci_workflow_guard_errors(&missing_permissions, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("missing top-level permissions")),
        "synthetic missing permissions must fail"
    );

    let hygiene_less_cache = CI_YML.replace(
        "hashFiles('rust-toolchain.toml', '**/Cargo.toml', '**/Cargo.lock')",
        "hashFiles('**/Cargo.lock')",
    );
    assert!(
        ci_workflow_guard_errors(&hygiene_less_cache, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("target cache key omits")),
        "synthetic target cache key without toolchain/manifests must fail"
    );

    let missing_gate_command = CI_YML.replace(
        "run: cargo build --workspace --all-targets --locked",
        "run: cargo build",
    );
    assert!(
        ci_workflow_guard_errors(&missing_gate_command, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("missing required gate command")),
        "synthetic missing verbatim gate command must fail"
    );

    let undocumented_job = format!(
        "{CI_YML}\n  synthetic-undocumented:\n    name: synthetic\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v5\n"
    );
    assert!(
        ci_workflow_guard_errors(&undocumented_job, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("workflow job synthetic-undocumented is not documented")),
        "synthetic undocumented workflow job must fail"
    );

    let divergent_scheduled_perimeter = CI_YML.replace(
        "cargo mutants --workspace --no-shuffle",
        "cargo mutants --workspace -f 'crates/tracewake-core/src/agent/**' --no-shuffle",
    );
    assert!(
        ci_workflow_guard_errors(&divergent_scheduled_perimeter, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("divergent scheduled mutation perimeter")),
        "synthetic scheduled mutation -f perimeter must fail"
    );

    let dropped_scheduled_shard = CI_YML.replace(
        "shard: [0, 1, 2, 3, 4, 5, 6, 7]",
        "shard: [0, 1, 2, 3, 4, 5]",
    );
    assert!(
        ci_workflow_guard_errors(&dropped_scheduled_shard, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("scheduled mutation matrix must enumerate shard indices")),
        "synthetic scheduled mutation matrix with a dropped shard must fail"
    );

    let fail_fast_scheduled_shards = CI_YML.replace("fail-fast: false", "fail-fast: true");
    assert!(
        ci_workflow_guard_errors(&fail_fast_scheduled_shards, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("scheduled mutation matrix must set fail-fast: false")),
        "synthetic scheduled mutation matrix with fail-fast true must fail"
    );

    let missing_reconcile_job = CI_YML.replace(
        "  mutants-lock-layer-reconcile:",
        "  mutants-lock-layer-reconcile-missing:",
    );
    assert!(
        ci_workflow_guard_errors(&missing_reconcile_job, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("scheduled mutation lane missing reconciliation job")),
        "synthetic scheduled mutation lane without reconciliation job must fail"
    );

    let missing_shard_upload = CI_YML.replace(
        "cargo-mutants-lock-layer-shard-${{ matrix.shard }}-of-8",
        "cargo-mutants-lock-layer-shard-missing",
    );
    assert!(
        ci_workflow_guard_errors(&missing_shard_upload, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("scheduled mutation lane missing shard artifact")),
        "synthetic scheduled mutation lane without shard upload must fail"
    );

    let missing_test_workspace = MUTANTS_TOML.replace("test_workspace = true", "");
    assert!(
        ci_workflow_guard_errors(CI_YML, &missing_test_workspace, DOC10)
            .iter()
            .any(|error| error.contains("missing mutation config posture")),
        "synthetic missing test_workspace must fail"
    );

    let missing_spine_file =
        MUTANTS_TOML.replace(r#"  "crates/tracewake-core/src/events/**","#, "");
    assert!(
        ci_workflow_guard_errors(CI_YML, &missing_spine_file, DOC10)
            .iter()
            .any(|error| error.contains("missing standing mutation perimeter path")),
        "synthetic missing SPINE perimeter path must fail"
    );

    let missing_perimeter_trigger =
        CI_YML.replace("crates/tracewake-core/src/need_accounting\\.rs|", "");
    assert!(
        ci_workflow_guard_errors(&missing_perimeter_trigger, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains("does not cover standing perimeter path")),
        "synthetic in-diff trigger missing a standing path must fail"
    );

    let missing_time_trigger = CI_YML.replace("crates/tracewake-core/src/time\\.rs|", "");
    assert!(
        ci_workflow_guard_errors(&missing_time_trigger, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains(
                "does not cover standing perimeter path crates/tracewake-core/src/time.rs"
            )),
        "synthetic in-diff trigger missing time.rs must fail"
    );

    let missing_checkcontainer_trigger = CI_YML.replace(
        "(need_events|eat|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs",
        "(need_events|eat|sleep|work|wait|continue_routine|movement)\\.rs",
    );
    assert!(
        ci_workflow_guard_errors(&missing_checkcontainer_trigger, MUTANTS_TOML, DOC10)
            .iter()
            .any(|error| error.contains(
                "does not cover standing perimeter path crates/tracewake-core/src/actions/defs/checkcontainer.rs"
            )),
        "synthetic in-diff trigger missing checkcontainer.rs must fail"
    );
}

fn ci_workflow_guard_errors(workflow: &str, mutants_config: &str, doc10: &str) -> Vec<String> {
    let mut errors = Vec::new();
    errors.extend(required_gate_command_errors(workflow, doc10));
    errors.extend(masked_gate_errors(workflow));
    errors.extend(permission_errors(workflow));
    errors.extend(action_pin_errors(workflow));
    errors.extend(cache_key_errors(workflow));
    errors.extend(doc_workflow_parity_errors(workflow, doc10));
    errors.extend(doc_flag_posture_errors(doc10));
    errors.extend(mutation_perimeter_errors(workflow, mutants_config));
    errors.extend(scheduled_mutation_lane_errors(workflow));
    errors
}

fn required_gate_command_errors(workflow: &str, doc10: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for command in REQUIRED_GATE_COMMANDS {
        if !workflow.contains(command) {
            errors.push(format!(
                "missing required gate command in workflow: {command}"
            ));
        }
        if !doc10.contains(command) {
            errors.push(format!(
                "missing required gate command in doc 10: {command}"
            ));
        }
    }
    errors
}

fn masked_gate_errors(workflow: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if workflow.contains("continue-on-error") {
        errors.push("workflow contains continue-on-error".to_string());
    }
    for line in workflow.lines().map(strip_comment) {
        if REQUIRED_GATE_COMMANDS
            .iter()
            .any(|command| line.contains(command))
            && (line.contains("||") || line.contains('|'))
        {
            errors.push(format!(
                "masked gate command or pipe in line: {}",
                line.trim()
            ));
        }
    }
    errors
}

fn permission_errors(workflow: &str) -> Vec<String> {
    if workflow.lines().any(|line| line == "permissions:")
        && workflow.lines().any(|line| line == "  contents: read")
    {
        Vec::new()
    } else {
        vec!["missing top-level permissions: contents: read".to_string()]
    }
}

fn action_pin_errors(workflow: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for line in workflow.lines().map(str::trim) {
        let action_ref = line
            .strip_prefix("uses: ")
            .or_else(|| line.strip_prefix("- uses: "));
        let Some(action_ref) = action_ref else {
            continue;
        };
        if action_ref.starts_with("actions/") {
            continue;
        }
        let Some((_, version)) = action_ref.rsplit_once('@') else {
            errors.push(format!("non-actions use is missing version: {action_ref}"));
            continue;
        };
        if version.len() != 40 || !version.chars().all(|ch| ch.is_ascii_hexdigit()) {
            errors.push(format!("non-actions use is not SHA-pinned: {action_ref}"));
        }
    }
    errors
}

fn cache_key_errors(workflow: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for block in workflow.split("uses: actions/cache@").skip(1) {
        let block = block
            .split("\n      - ")
            .next()
            .expect("split always yields first block");
        if !block.lines().any(|line| line.trim() == "target") {
            continue;
        }
        let key_line = block
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("key: "))
            .unwrap_or("");
        for required in ["rust-toolchain.toml", "**/Cargo.toml", "**/Cargo.lock"] {
            if !key_line.contains(required) {
                errors.push(format!("target cache key omits {required}: {key_line}"));
            }
        }
    }
    errors
}

fn doc_workflow_parity_errors(workflow: &str, doc10: &str) -> Vec<String> {
    let workflow_jobs = workflow_job_ids(workflow);
    let doc_jobs = doc_ci_job_ids(doc10);
    let mut errors = Vec::new();
    for job in &workflow_jobs {
        if !doc_jobs.contains(job) {
            errors.push(format!("workflow job {job} is not documented in doc 10"));
        }
    }
    for job in &doc_jobs {
        if !workflow_jobs.contains(job) {
            errors.push(format!("doc 10 lists CI job {job} not present in workflow"));
        }
    }
    errors
}

fn doc_flag_posture_errors(doc10: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for required in [
        r#"RUSTFLAGS: "-D warnings""#,
        "cargo test --workspace --locked",
        "lock-layer-gates",
        "mutants-lock-layer",
        "dated green scheduled mutation run",
    ] {
        if !doc10.contains(required) {
            errors.push(format!("doc 10 missing CI posture text: {required}"));
        }
    }
    errors
}

fn mutation_perimeter_errors(workflow: &str, mutants_config: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for required in [
        r#"additional_cargo_args = ["--locked"]"#,
        "test_workspace = true",
        "examine_globs = [",
    ] {
        if !mutants_config.contains(required) {
            errors.push(format!("missing mutation config posture: {required}"));
        }
    }
    if mutants_config.contains("exclude_globs") {
        errors.push("mutation config reintroduces exclude_globs".to_string());
    }
    for path in STANDING_MUTATION_PERIMETER {
        if !mutants_config.contains(&format!(r#""{path}""#)) {
            errors.push(format!("missing standing mutation perimeter path: {path}"));
        }
        let trigger = in_diff_trigger_fragment_for_perimeter_path(path);
        if !workflow.contains(trigger) {
            errors.push(format!(
                "in-diff mutation trigger does not cover standing perimeter path {path} with fragment: {trigger}"
            ));
        }
    }
    for trigger in STANDING_MUTATION_TRIGGER_FRAGMENTS {
        if !workflow.contains(trigger) {
            errors.push(format!(
                "in-diff mutation trigger omits standing perimeter fragment: {trigger}"
            ));
        }
    }
    if workflow.contains("cargo mutants --workspace -f")
        || workflow.contains("cargo mutants --workspace \\\n            -f")
    {
        errors.push("divergent scheduled mutation perimeter uses -f filters".to_string());
    }
    for forbidden in ["--no-config"] {
        if workflow.contains(forbidden) {
            errors.push(format!(
                "mutation workflow uses forbidden option: {forbidden}"
            ));
        }
    }
    for required in [
        "cargo install cargo-mutants --version 27.1.0 --locked",
        "cargo mutants --workspace --no-shuffle",
        ".cargo/mutants-baseline-misses.txt",
        "comm -23",
        "mutants.out/timeout.txt",
        "actions/upload-artifact@",
    ] {
        if !workflow.contains(required) {
            errors.push(format!(
                "mutation workflow missing enforcement text: {required}"
            ));
        }
    }
    errors
}

fn scheduled_mutation_lane_errors(workflow: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for required in [
        "mutants-lock-layer-baseline:",
        "mutants-lock-layer:",
        "mutants-lock-layer-reconcile:",
        "needs: mutants-lock-layer-baseline",
        "timeout-minutes: 130",
        "fail-fast: false",
        "shard: [0, 1, 2, 3, 4, 5, 6, 7]",
        r#"MUTANTS_JOBS: "2""#,
        r#"MUTANTS_SHARDS: "8""#,
        r#"MUTANTS_WALL_SECONDS: "7200""#,
        r#"MUTANTS_GRACE_SECONDS: "120""#,
        r#"MUTANTS_TEST_TIMEOUT: "183""#,
        "cargo mutants --workspace --no-shuffle --list-files",
        "cargo mutants --workspace --no-shuffle --list",
        "tools/supervise-command.sh",
        r#"--shard "${shard_id}/${MUTANTS_SHARDS}""#,
        r#"--jobs "$MUTANTS_JOBS""#,
        "--baseline=skip",
        r#"--timeout "$MUTANTS_TEST_TIMEOUT""#,
        "assigned-mutants.json",
        r#"cp -R "$out_dir/mutants.out" "$shard_dir/mutants.out""#,
        "actions/download-artifact@",
        "pattern: cargo-mutants-lock-layer-*",
        "python3 tools/merge-mutation-shards.py",
        "--canonical-list",
        "--expected-shards 8",
        "--out-md mutation-lane/mutation_completion_manifest.md",
        "--out-json mutation-lane/mutation_completion_manifest.json",
    ] {
        if !workflow.contains(required) {
            errors.push(format!(
                "scheduled mutation lane missing required text: {required}"
            ));
        }
    }
    if !workflow.contains("cargo-mutants-lock-layer-shard-${{ matrix.shard }}-of-8") {
        errors.push("scheduled mutation lane missing shard artifact upload".to_string());
    }
    if !workflow.contains("mutants-lock-layer-reconcile:") {
        errors.push("scheduled mutation lane missing reconciliation job".to_string());
    }
    if !workflow.contains("shard: [0, 1, 2, 3, 4, 5, 6, 7]") {
        errors
            .push("scheduled mutation matrix must enumerate shard indices 0 through 7".to_string());
    }
    if workflow.contains("fail-fast: true") {
        errors.push("scheduled mutation matrix must set fail-fast: false".to_string());
    }
    errors
}

fn in_diff_trigger_fragment_for_perimeter_path(path: &str) -> &'static str {
    match path {
        "crates/tracewake-core/src/agent/**" => "crates/tracewake-core/src/agent/",
        "crates/tracewake-core/src/need_accounting.rs" => {
            "crates/tracewake-core/src/need_accounting\\.rs"
        }
        "crates/tracewake-core/src/scheduler.rs" => {
            "crates/tracewake-core/src/scheduler\\.rs"
        }
        "crates/tracewake-core/src/time.rs" => "crates/tracewake-core/src/time\\.rs",
        "crates/tracewake-core/src/projections.rs" => {
            "crates/tracewake-core/src/projections\\.rs"
        }
        "crates/tracewake-core/src/actions/pipeline.rs" => {
            "crates/tracewake-core/src/actions/pipeline\\.rs"
        }
        "crates/tracewake-core/src/actions/registry.rs" => {
            "crates/tracewake-core/src/actions/registry\\.rs"
        }
        "crates/tracewake-core/src/actions/defs/need_events.rs"
        | "crates/tracewake-core/src/actions/defs/eat.rs"
        | "crates/tracewake-core/src/actions/defs/sleep.rs"
        | "crates/tracewake-core/src/actions/defs/work.rs"
        | "crates/tracewake-core/src/actions/defs/wait.rs"
        | "crates/tracewake-core/src/actions/defs/continue_routine.rs"
        | "crates/tracewake-core/src/actions/defs/movement.rs"
        | "crates/tracewake-core/src/actions/defs/checkcontainer.rs" => {
            "crates/tracewake-core/src/actions/defs/(need_events|eat|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs"
        }
        "crates/tracewake-core/src/events/**" => "crates/tracewake-core/src/events/",
        "crates/tracewake-core/src/replay/**" => "crates/tracewake-core/src/replay/",
        "crates/tracewake-core/src/checksum.rs" => {
            "crates/tracewake-core/src/checksum\\.rs"
        }
        "crates/tracewake-core/src/state.rs" => "crates/tracewake-core/src/state\\.rs",
        "crates/tracewake-core/src/actions/proposal.rs"
        | "crates/tracewake-core/src/actions/report.rs" => {
            "crates/tracewake-core/src/actions/(proposal|report)\\.rs"
        }
        "crates/tracewake-core/src/view_models.rs" => {
            "crates/tracewake-core/src/view_models\\.rs"
        }
        "crates/tracewake-core/src/debug_capability.rs" => {
            "crates/tracewake-core/src/debug_capability\\.rs"
        }
        "crates/tracewake-core/src/controller.rs" => {
            "crates/tracewake-core/src/controller\\.rs"
        }
        "crates/tracewake-core/src/debug_reports.rs" => {
            "crates/tracewake-core/src/debug_reports\\.rs"
        }
        "crates/tracewake-core/src/epistemics/**"
        | "crates/tracewake-core/src/epistemics/knowledge_context.rs"
        | "crates/tracewake-core/src/epistemics/projection.rs" => {
            "crates/tracewake-core/src/epistemics/"
        }
        "crates/tracewake-content/src/manifest.rs"
        | "crates/tracewake-content/src/load.rs"
        | "crates/tracewake-content/src/schema.rs"
        | "crates/tracewake-content/src/serialization.rs"
        | "crates/tracewake-content/src/validate.rs" => {
            "crates/tracewake-content/src/(manifest|load|schema|serialization|validate)\\.rs"
        }
        "crates/tracewake-tui/src/app.rs"
        | "crates/tracewake-tui/src/debug_panels.rs"
        | "crates/tracewake-tui/src/render.rs"
        | "crates/tracewake-tui/src/transcript.rs" => {
            "crates/tracewake-tui/src/(app|debug_panels|render|transcript)\\.rs"
        }
        _ => panic!("standing mutation perimeter has no in-diff trigger mapping: {path}"),
    }
}

fn workflow_job_ids(workflow: &str) -> BTreeSet<String> {
    let mut jobs = BTreeSet::new();
    let mut in_jobs = false;
    for line in workflow.lines() {
        if line == "jobs:" {
            in_jobs = true;
            continue;
        }
        if !in_jobs {
            continue;
        }
        if line.starts_with(char::is_alphabetic) && !line.starts_with("jobs:") {
            break;
        }
        if line.starts_with("  ") && !line.starts_with("    ") {
            if let Some(job) = line.trim().strip_suffix(':') {
                jobs.insert(job.to_string());
            }
        }
    }
    jobs
}

fn doc_ci_job_ids(doc10: &str) -> BTreeSet<String> {
    let mut jobs = BTreeSet::new();
    let mut in_section = false;
    for line in doc10.lines() {
        if line == "## Current CI Job Set" {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with("## ") {
            break;
        }
        if !in_section || !line.starts_with('|') {
            continue;
        }
        let Some(first_tick) = line.find('`') else {
            continue;
        };
        let Some(second_tick) = line[first_tick + 1..].find('`') else {
            continue;
        };
        jobs.insert(line[first_tick + 1..first_tick + 1 + second_tick].to_string());
    }
    jobs
}

fn strip_comment(line: &str) -> String {
    line.split('#').next().unwrap_or("").to_string()
}
