mod support;

use support::acceptance_status_manifest::{validate_status_manifest, ComputedResult};

#[test]
fn all_closed_manifest_computes_pass() {
    let manifest = validate_status_manifest(&synthetic_manifest(
        "pass",
        "enforced",
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
    findings[1] = "finding: F5-02 | open";
    findings[3] = "finding: F5-04 | pending-governance";

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
    findings[0] = "finding: F5-01 | open";

    let error = validate_status_manifest(&synthetic_manifest(
        "pass",
        "enforced",
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
    findings[4] = "finding: F5-05 | routed-forward | owner=projection | reason=semantic decision deferred | next_move=projection remediation | max_epochs=1 | failing_check=cargo mutants -f crates/tracewake-core/src/projections.rs";

    let manifest = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "enforced",
        "killed",
        "none",
        &findings,
        &[],
    ))
    .expect("bounded routed-forward finding is valid but non-pass");

    assert_eq!(manifest.computed_result, ComputedResult::NonPass);

    findings[4] = "finding: F5-05 | routed-forward | owner=projection";
    let error = validate_status_manifest(&synthetic_manifest(
        "non-pass",
        "enforced",
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
        "pass",
        "enforced",
        "non-blocking-bounded-forcing",
        "food_source_fact_supersedes",
        &closed_findings(),
        &survivors,
    ))
    .expect("bounded survivor forcing can compute pass");

    assert_eq!(manifest.computed_result, ComputedResult::Pass);
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
        "finding: F5-01 | closed | evidence=sealed bootstrap constructor | negative=external bootstrap negative fixture",
        "finding: F5-02 | closed | evidence=sealed interval product | negative=external interval negative fixture",
        "finding: F5-03 | closed | evidence=debug session authority token | negative=external debug command negative fixture",
        "finding: F5-04 | closed | evidence=branch protection API transcript | negative=governance audit pending failure witness",
        "finding: F5-05 | closed | evidence=food source projection behavior tests | negative=focused mutation survivor kills",
        "finding: F5-06 | closed | evidence=manifest consumer and wording guard | negative=synthetic contradictory manifest",
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
    let mut lines = vec![
        "# Synthetic 0053 acceptance artifact".to_string(),
        "```tracewake-acceptance-status".to_string(),
        format!("overall_result: {overall_result}"),
        "commit_under_test: 0123456789abcdef0123456789abcdef01234567".to_string(),
        "source_acquisition: clean local checkout".to_string(),
        format!("branch_protection: {branch_protection}"),
        format!("mutation_status: {mutation_status}"),
        format!("mutation_survivors: {mutation_survivors}"),
    ];
    lines.extend(findings.iter().map(|line| (*line).to_string()));
    lines.extend(survivors.iter().map(|line| (*line).to_string()));
    lines.push("```".to_string());
    lines.join("\n")
}
