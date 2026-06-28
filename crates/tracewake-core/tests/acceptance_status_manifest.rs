mod support;

use support::acceptance_status_manifest::{validate_status_manifest, ComputedResult};

#[test]
#[allow(clippy::disallowed_methods)]
fn actual_acceptance_artifact_from_ci_env_is_pass_eligible() {
    // CI passes the changed artifact path through an environment variable so this
    // test can ingest the real PR artifact with the same parser used by the
    // synthetic cases. This is deliberately test-boundary IO, not simulation IO.
    let Ok(path) = std::env::var("TRACEWAKE_ACCEPTANCE_ARTIFACT") else {
        return;
    };
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    let manifest = validate_status_manifest(&text)
        .unwrap_or_else(|error| panic!("{path} is not a valid acceptance artifact: {error}"));

    assert_eq!(
        manifest.computed_result,
        ComputedResult::Pass,
        "{path} must be pass-eligible for CI ingestion"
    );
    assert!(
        !manifest.has_blocking_status,
        "{path} has blocking acceptance status"
    );
    assert!(
        !manifest.has_mutation_survivors,
        "{path} has mutation survivors and cannot be a green closure artifact"
    );
}

#[test]
fn all_closed_manifest_computes_pass() {
    let manifest = validate_status_manifest(&synthetic_manifest(
        "pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .expect("all-closed manifest is valid");

    assert_eq!(manifest.computed_result, ComputedResult::Pass);
    assert!(!manifest.has_blocking_status);
    assert!(!manifest.has_mutation_survivors);
}

#[test]
fn open_or_pending_status_computes_non_pass() {
    let mut findings = closed_findings();
    findings[1] = "finding: F6-02 | open";
    findings[3] = "finding: F6-04 | pending-governance";

    let manifest = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "pending/unverified",
        "open",
        "none",
        &findings,
        &[],
    ))
    .expect("open and pending statuses are valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);
    assert!(manifest.has_blocking_status);
}

#[test]
fn stated_pass_with_open_status_fails_closed() {
    let mut findings = closed_findings();
    findings[0] = "finding: F6-01 | open";

    let error = validate_status_manifest(&synthetic_manifest(
        "pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &findings,
        &[],
    ))
    .unwrap_err();

    assert!(error.contains("does not match computed result"));
}

#[test]
fn routed_forward_requires_bounded_forcing_fields() {
    let mut findings = closed_findings();
    findings[4] = "finding: F6-05 | routed-forward | owner=projection | reason=semantic decision deferred | next_move=projection remediation | max_epochs=1 | failing_check=cargo mutants -f crates/tracewake-core/src/projections.rs";

    let manifest = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &findings,
        &[],
    ))
    .expect("bounded routed-forward finding is valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);

    findings[4] = "finding: F6-05 | routed-forward | owner=projection";
    let error = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &findings,
        &[],
    ))
    .unwrap_err();
    assert!(error.contains("missing forcing-function field"));
}

#[test]
fn mutation_survivors_require_bounded_forcing_or_block_pass() {
    let survivors = ["survivor: food_source_fact_supersedes | routed-forward | owner=projection | reason=cross-cutting semantic closure | next_move=next projection remediation | max_epochs=1 | failing_check=cargo mutants -f crates/tracewake-core/src/projections.rs"];
    let manifest = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "ruleset-transcript-current",
        "non-blocking-bounded-forcing",
        "food_source_fact_supersedes",
        &closed_findings(),
        &survivors,
    ))
    .expect("bounded survivor forcing is valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);
    assert!(manifest.has_mutation_survivors);

    let error = validate_status_manifest(&synthetic_manifest(
        "pass",
        "enforced",
        "open",
        "food_source_fact_supersedes",
        &closed_findings(),
        &survivors,
    ))
    .unwrap_err();
    assert!(error.contains("does not match computed result"));
}

#[test]
fn expected_finding_manifest_accepts_current_f6_labels_without_source_edit() {
    let manifest = validate_status_manifest(&synthetic_manifest(
        "pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .expect("current F6 finding set is manifest-driven");

    assert_eq!(manifest.computed_result, ComputedResult::Pass);

    let mut findings = closed_findings();
    findings[5] = "finding: F6-07 | closed | evidence_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | evidence_test=all_closed_manifest_computes_pass | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=stated_pass_with_open_status_fails_closed | negative_scope=current";
    let error = validate_status_manifest(&synthetic_manifest(
        "pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &findings,
        &[],
    ))
    .unwrap_err();
    assert!(error.contains("missing required finding: F6-06"));
}

#[test]
fn scalar_branch_protection_without_ruleset_transcript_is_non_pass() {
    let manifest = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "enforced",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .expect("legacy scalar branch protection is parseable but blocking");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);
    assert!(manifest.has_blocking_status);
}

#[test]
fn zero_approval_governance_transcript_computes_non_pass() {
    let manifest = validate_status_manifest(&synthetic_manifest_with_governance(
        "non-pass",
        "ruleset-transcript-current",
        "zero-approval",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .expect("zero-approval transcript is valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);
    assert!(manifest.has_blocking_status);
}

#[test]
fn status_checks_only_transcript_is_not_independent_acceptance() {
    let manifest = validate_status_manifest(&synthetic_manifest_with_governance(
        "non-pass",
        "ruleset-transcript-current",
        "status-checks-only",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .expect("status-checks-only transcript is valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);
    assert!(manifest.has_blocking_status);

    let error = validate_status_manifest(&synthetic_manifest_with_governance(
        "pass",
        "ruleset-transcript-current",
        "status-checks-only",
        "killed",
        "none",
        &closed_findings(),
        &[],
    ))
    .unwrap_err();
    assert!(error.contains("does not match computed result"));
}

#[test]
fn killed_mutation_status_requires_current_counted_evidence() {
    for (field, value, expected) in [
        (
            "mutation_evidence",
            "scheduled-trigger-only",
            "requires current mutation evidence",
        ),
        (
            "mutation_missed",
            "1",
            "cannot include missed or timeout mutants",
        ),
        (
            "mutation_timeout",
            "1",
            "cannot include missed or timeout mutants",
        ),
        (
            "mutation_baseline_reconciliation",
            "stale-baseline",
            "baseline reconciliation must be current-reconciled",
        ),
    ] {
        let manifest = synthetic_manifest_with_mutation_override(field, value);
        let error = validate_status_manifest(&manifest).unwrap_err();
        assert!(
            error.contains(expected),
            "expected {expected:?} for {field}={value}, got {error}"
        );
    }
}

#[test]
fn trigger_alarm_is_not_mutation_proof_for_pass() {
    let error = validate_status_manifest(&synthetic_manifest_with_mutation_override(
        "mutation_evidence",
        "trigger-alarm",
    ))
    .unwrap_err();

    assert!(error.contains("requires current mutation evidence"));
}

#[test]
fn non_current_evidence_inputs_fail_closed() {
    for (field_value, expected) in [
        ("stale_method_name", "non-current or non-behavior evidence"),
        (
            "self-authored-status-row",
            "non-current or non-behavior evidence",
        ),
        (
            "historical-only-command",
            "non-current or non-behavior evidence",
        ),
        (
            "display-only-render-string",
            "non-current or non-behavior evidence",
        ),
    ] {
        let mut findings = closed_findings();
        findings[0] = Box::leak(format!("finding: F6-01 | closed | evidence_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | evidence_test={field_value} | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=stated_pass_with_open_status_fails_closed | negative_scope=current").into_boxed_str());
        let error = validate_status_manifest(&synthetic_manifest(
            "pass",
            "ruleset-transcript-current",
            "killed",
            "none",
            &findings,
            &[],
        ))
        .unwrap_err();
        assert!(
            error.contains(expected),
            "expected {expected:?} for {field_value:?}, got {error}"
        );
    }
}

#[test]
fn missing_or_unparseable_status_block_fails_closed() {
    assert!(validate_status_manifest("no status block")
        .unwrap_err()
        .contains("missing tracewake acceptance status block"));
    assert!(
        validate_status_manifest("```tracewake-acceptance-status\nnot a kv line\n```")
            .unwrap_err()
            .contains("missing key/value separator")
    );
}

fn closed_findings() -> [&'static str; 6] {
    [
        "finding: F6-01 | closed | evidence_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | evidence_test=all_closed_manifest_computes_pass | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=stated_pass_with_open_status_fails_closed | negative_scope=current",
        "finding: F6-02 | closed | evidence_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | evidence_test=open_or_pending_status_computes_non_pass | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=missing_or_unparseable_status_block_fails_closed | negative_scope=current",
        "finding: F6-03 | closed | evidence_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | evidence_test=routed_forward_requires_bounded_forcing_fields | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=mutation_survivors_require_bounded_forcing_or_block_pass | negative_scope=current",
        "finding: F6-04 | closed | evidence_file=crates/tracewake-core/tests/acceptance_artifact_wording.rs | evidence_test=status_manifest_requires_branch_protection_transcript_for_enforced_claims | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=scalar_branch_protection_without_ruleset_transcript_is_non_pass | negative_scope=current",
        "finding: F6-05 | closed | evidence_file=crates/tracewake-core/tests/acceptance_artifact_wording.rs | evidence_test=status_manifest_blocks_display_string_as_sole_behavior_evidence | evidence_scope=current | negative_file=crates/tracewake-core/tests/acceptance_status_manifest.rs | negative_test=non_current_evidence_inputs_fail_closed | negative_scope=current",
        "finding: F6-06 | closed | evidence_file=crates/tracewake-core/tests/ci_workflow_guards.rs | evidence_test=ci_workflow_guards_cover_workflow_integrity | evidence_scope=current | negative_file=crates/tracewake-core/tests/ci_workflow_guards.rs | negative_test=acceptance_artifact_ingestion_guard_rejects_missing_job | negative_scope=current",
    ]
}

fn synthetic_manifest(
    overall_result: &str,
    branch_protection: &str,
    mutation_status: &str,
    mutation_survivors: &str,
    findings: &[&str],
    survivors: &[&str],
) -> String {
    synthetic_manifest_with_governance(
        overall_result,
        branch_protection,
        "independent-review",
        mutation_status,
        mutation_survivors,
        findings,
        survivors,
    )
}

fn synthetic_manifest_with_governance(
    overall_result: &str,
    branch_protection: &str,
    governance_independence: &str,
    mutation_status: &str,
    mutation_survivors: &str,
    findings: &[&str],
    survivors: &[&str],
) -> String {
    let mut lines = vec![
        "# Synthetic 0053 acceptance artifact".to_string(),
        "```tracewake-acceptance-status".to_string(),
        format!("overall_result: {overall_result}"),
        "commit_under_test: 0123456789abcdef0123456789abcdef01234567".to_string(),
        "source_acquisition: clean local checkout".to_string(),
        "expected_findings: F6-01,F6-02,F6-03,F6-04,F6-05,F6-06".to_string(),
        format!("branch_protection: {branch_protection}"),
        format!("governance_independence: {governance_independence}"),
        "mutation_evidence: current-in-diff".to_string(),
        "mutation_denominator: 2".to_string(),
        "mutation_caught: 2".to_string(),
        "mutation_unviable: 0".to_string(),
        "mutation_missed: 0".to_string(),
        "mutation_timeout: 0".to_string(),
        "mutation_baseline_reconciliation: current-reconciled".to_string(),
        format!("mutation_status: {mutation_status}"),
        format!("mutation_survivors: {mutation_survivors}"),
    ];
    lines.extend(findings.iter().map(|line| (*line).to_string()));
    lines.extend(survivors.iter().map(|line| (*line).to_string()));
    lines.push("```".to_string());
    lines.join("\n")
}

fn synthetic_manifest_with_mutation_override(field: &str, value: &str) -> String {
    let manifest = synthetic_manifest(
        "pass",
        "ruleset-transcript-current",
        "killed",
        "none",
        &closed_findings(),
        &[],
    );
    let manifest = if matches!(field, "mutation_missed" | "mutation_timeout") {
        manifest.replace("mutation_denominator: 2", "mutation_denominator: 3")
    } else {
        manifest
    };
    manifest.replace(
        &format!("{field}: {}", default_mutation_value(field)),
        &format!("{field}: {value}"),
    )
}

fn default_mutation_value(field: &str) -> &'static str {
    match field {
        "mutation_evidence" => "current-in-diff",
        "mutation_denominator" => "2",
        "mutation_caught" => "2",
        "mutation_unviable" => "0",
        "mutation_missed" => "0",
        "mutation_timeout" => "0",
        "mutation_baseline_reconciliation" => "current-reconciled",
        _ => panic!("unknown mutation field: {field}"),
    }
}
