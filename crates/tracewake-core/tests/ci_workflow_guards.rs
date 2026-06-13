use std::collections::BTreeSet;

const CI_YML: &str = include_str!("../../../.github/workflows/ci.yml");
const DOC10: &str = include_str!(
    "../../../docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md"
);

const REQUIRED_GATE_COMMANDS: &[&str] = &[
    "cargo fmt --all --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo build --workspace --all-targets --locked",
    "cargo test --workspace --locked",
];

#[test]
fn ci_workflow_guards_cover_workflow_integrity() {
    let errors = ci_workflow_guard_errors(CI_YML, DOC10);
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
        ci_workflow_guard_errors(&masked_gate, DOC10)
            .iter()
            .any(|error| error.contains("masked gate command")),
        "synthetic masked gate step must fail"
    );

    let unpinned_third_party = CI_YML.replace(
        "- uses: actions/checkout@v4",
        "- uses: docker/login-action@v3",
    );
    assert!(
        ci_workflow_guard_errors(&unpinned_third_party, DOC10)
            .iter()
            .any(|error| error.contains("non-actions use is not SHA-pinned")),
        "synthetic unpinned third-party action must fail"
    );

    let missing_permissions = CI_YML.replace("permissions:\n  contents: read\n\n", "");
    assert!(
        ci_workflow_guard_errors(&missing_permissions, DOC10)
            .iter()
            .any(|error| error.contains("missing top-level permissions")),
        "synthetic missing permissions must fail"
    );

    let hygiene_less_cache = CI_YML.replace(
        "hashFiles('rust-toolchain.toml', '**/Cargo.toml', '**/Cargo.lock')",
        "hashFiles('**/Cargo.lock')",
    );
    assert!(
        ci_workflow_guard_errors(&hygiene_less_cache, DOC10)
            .iter()
            .any(|error| error.contains("target cache key omits")),
        "synthetic target cache key without toolchain/manifests must fail"
    );

    let missing_gate_command = CI_YML.replace(
        "run: cargo build --workspace --all-targets --locked",
        "run: cargo build",
    );
    assert!(
        ci_workflow_guard_errors(&missing_gate_command, DOC10)
            .iter()
            .any(|error| error.contains("missing required gate command")),
        "synthetic missing verbatim gate command must fail"
    );

    let undocumented_job = format!(
        "{CI_YML}\n  synthetic-undocumented:\n    name: synthetic\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n"
    );
    assert!(
        ci_workflow_guard_errors(&undocumented_job, DOC10)
            .iter()
            .any(|error| error.contains("workflow job synthetic-undocumented is not documented")),
        "synthetic undocumented workflow job must fail"
    );
}

fn ci_workflow_guard_errors(workflow: &str, doc10: &str) -> Vec<String> {
    let mut errors = Vec::new();
    errors.extend(required_gate_command_errors(workflow, doc10));
    errors.extend(masked_gate_errors(workflow));
    errors.extend(permission_errors(workflow));
    errors.extend(action_pin_errors(workflow));
    errors.extend(cache_key_errors(workflow));
    errors.extend(doc_workflow_parity_errors(workflow, doc10));
    errors.extend(doc_flag_posture_errors(doc10));
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
    for block in workflow.split("uses: actions/cache@v4").skip(1) {
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
