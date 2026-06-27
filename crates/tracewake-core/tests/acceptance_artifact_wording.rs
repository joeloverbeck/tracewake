mod support;

use support::acceptance_status_manifest::{validate_status_manifest, ComputedResult, STATUS_FENCE};

const TEMPLATE: &str = include_str!("../../../docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md");
const PHASE2A_ARTIFACT: &str =
    include_str!("../../../archive/reports/0006PHA2A_ACCEPTANCE_ARTIFACT.md");
const P0_CERT_0037_ARTIFACT: &str = include_str!(
    "../../../archive/reports/0037_p0_cert_mutation_remediation_replacement_certification_acceptance.md"
);
const SPINE_CERT_0039_ARTIFACT: &str = include_str!(
    "../../../archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md"
);
const FIRST_PROOF_CERT_0045_ARTIFACT: &str = include_str!(
    "../../../archive/reports/0045_first_proof_cert_mutation_remediation_replacement_certification_acceptance.md"
);

const REQUIRED_PHASE1_SCOPED_WORDING: &str =
    "Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit";
const REQUIRED_PHASE2A_SCOPED_WORDING: &str =
    "Phase 2A epistemic substrate hardening remediation accepted for exact commit";
const REQUIRED_P0_CERT_SCOPED_WORDING: &str =
    "P0-CERT post-0008 baseline mutation remediation accepted for exact commit";
const REQUIRED_SPINE_CERT_SCOPED_WORDING: &str =
    "SPINE-CERT mutation remediation accepted for exact commit";
const REQUIRED_FIRST_PROOF_CERT_SCOPED_WORDING: &str =
    "FIRST-PROOF-CERT mutation remediation and replacement certification accepted for exact commit";

const FORBIDDEN_WORDING_HEADING: &str = "Forbidden wording:";

const FORBIDDEN_RESULT_CLAIMS: &[&str] = &[
    "Tracewake is fully certified.",
    "Latest main was independently verified.",
    "Later Phase 2+ / Phase 3A+ systems are certified by this pass.",
    "Archived specs are live authority.",
    "Project is P0 certified.",
    "SPINE-CERT passed.",
];

const CONDITIONAL_CLOSURE_CLAIMS: &[&str] = &[
    "pass with",
    "scoped pass",
    "accepted",
    "green canonical perimeter",
    "canonical perimeter green",
];

#[test]
fn acceptance_artifact_template_requires_scoped_exact_commit_wording() {
    validate_acceptance_artifact_wording(TEMPLATE).expect("template wording is scoped");
    assert!(TEMPLATE.contains("Exact commit under test"));
    assert!(TEMPLATE.contains("Record the command, result, and concise output summary"));
    assert!(TEMPLATE.contains("Residual convention-only items"));
    assert!(TEMPLATE.contains(FORBIDDEN_WORDING_HEADING));
}

#[test]
fn phase2a_acceptance_artifact_uses_scoped_exact_commit_wording() {
    validate_acceptance_artifact_wording(PHASE2A_ARTIFACT)
        .expect("Phase-2A artifact wording is scoped");
    assert!(PHASE2A_ARTIFACT.contains("Exact commit under test"));
    assert!(PHASE2A_ARTIFACT.contains("9e0590d056b15d879ac02eb2556c855c080f27e4"));
}

#[test]
fn p0_cert_0037_acceptance_artifact_uses_scoped_exact_commit_wording() {
    validate_acceptance_artifact_wording(P0_CERT_0037_ARTIFACT)
        .expect("0037 P0-CERT artifact wording is scoped");
    assert!(P0_CERT_0037_ARTIFACT.contains("Exact implementation commit under test"));
    assert!(P0_CERT_0037_ARTIFACT.contains("a3b5e3c9e896d09ed9f5b6591fb6cd7a4edd0441"));
    assert!(P0_CERT_0037_ARTIFACT.contains("Verdict: P0-CERT passed"));
}

#[test]
fn spine_cert_0039_acceptance_artifact_uses_scoped_exact_commit_wording() {
    validate_acceptance_artifact_wording(SPINE_CERT_0039_ARTIFACT)
        .expect("0039 SPINE-CERT artifact wording is scoped");
    assert!(SPINE_CERT_0039_ARTIFACT.contains("Exact implementation commit under test"));
    assert!(SPINE_CERT_0039_ARTIFACT.contains("92ba47f14998e0ea2fc95502bc3b76c5909478ca"));
    assert!(SPINE_CERT_0039_ARTIFACT.contains("Verdict: SPINE-CERT passed"));
}

#[test]
fn first_proof_cert_0045_acceptance_artifact_uses_scoped_exact_commit_wording() {
    validate_acceptance_artifact_wording(FIRST_PROOF_CERT_0045_ARTIFACT)
        .expect("0045 FIRST-PROOF-CERT artifact wording is scoped");
    assert!(
        FIRST_PROOF_CERT_0045_ARTIFACT.contains("Exact implementation/evidence commit under test")
    );
    assert!(FIRST_PROOF_CERT_0045_ARTIFACT.contains("9a071b6e32ebc5b6126645a9db257d453399c028"));
    assert!(FIRST_PROOF_CERT_0045_ARTIFACT.contains("Verdict: FIRST-PROOF-CERT passed"));
    assert!(FIRST_PROOF_CERT_0045_ARTIFACT
        .contains("Certification use: not counted as certifying evidence"));
}

#[test]
fn acceptance_artifact_forbidden_overclaim_phrase_fails() {
    let artifact =
        format!("{REQUIRED_PHASE1_SCOPED_WORDING} `<commit>`.\n\nTracewake is fully certified.");

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("forbidden overclaim"));
}

#[test]
fn acceptance_artifact_missing_scoped_phrase_fails() {
    let artifact = "Phase 1 acceptance passed for this branch.";

    assert!(validate_acceptance_artifact_wording(artifact)
        .unwrap_err()
        .contains("missing scoped exact-commit wording"));
}

#[test]
fn status_manifest_blocks_pass_shaped_wording_over_open_items() {
    let artifact = format!(
        "{} `<commit>`.\n\nVerdict: scoped pass accepted.\n\n{}",
        REQUIRED_PHASE1_SCOPED_WORDING,
        synthetic_status_manifest(
            "non-pass",
            "pending/unverified",
            "open",
            "none",
            &open_findings(),
            &[],
        )
    );

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("closure wording over non-pass status manifest"));
}

#[test]
fn status_manifest_blocks_green_perimeter_with_survivors() {
    let artifact = format!(
        "{} `<commit>`.\n\nThe canonical perimeter green claim is ready.\n\n{}",
        REQUIRED_PHASE1_SCOPED_WORDING,
        synthetic_status_manifest(
            "pass",
            "enforced",
            "non-blocking-bounded-forcing",
            "food_source_fact_supersedes",
            &closed_findings(),
            &["survivor: food_source_fact_supersedes | routed-forward | owner=projection | reason=cross-cutting semantic closure | next_move=next projection remediation | max_epochs=1 | failing_check=cargo mutants -f crates/tracewake-core/src/projections.rs"],
        )
    );

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("green perimeter wording with mutation survivors"));
}

#[test]
fn status_manifest_requires_branch_protection_transcript_for_enforced_claims() {
    let artifact = format!(
        "{} `<commit>`.\n\nBranch protection is enforced for main.\n\n{}",
        REQUIRED_PHASE1_SCOPED_WORDING,
        synthetic_status_manifest(
            "pass",
            "enforced",
            "killed",
            "none",
            &closed_findings(),
            &[],
        )
    );

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("branch-protection enforcement claim lacks API transcript"));
}

#[test]
fn status_manifest_blocks_historical_results_as_current_certification() {
    let artifact = format!(
        "{} `<commit>`.\n\nHistorical command results certify current status.\n\nBranch-protection API transcript: gh api repos/:owner/:repo/branches/main/protection.\n\n{}",
        REQUIRED_PHASE1_SCOPED_WORDING,
        synthetic_status_manifest(
            "pass",
            "enforced",
            "killed",
            "none",
            &closed_findings(),
            &[],
        )
    );

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("historical command results as current certification"));
}

#[test]
fn status_manifest_blocks_display_string_as_sole_behavior_evidence() {
    let artifact = format!(
        "{} `<commit>`.\n\nBranch-protection API transcript: gh api repos/:owner/:repo/branches/main/protection.\n\nEvidence row: display string as sole evidence for behavior claim.\n\n{}",
        REQUIRED_PHASE1_SCOPED_WORDING,
        synthetic_status_manifest(
            "pass",
            "enforced",
            "killed",
            "none",
            &closed_findings(),
            &[],
        )
    );

    assert!(validate_acceptance_artifact_wording(&artifact)
        .unwrap_err()
        .contains("display string as sole behavior evidence"));
}

fn validate_acceptance_artifact_wording(text: &str) -> Result<(), String> {
    if !text.contains(REQUIRED_PHASE1_SCOPED_WORDING)
        && !text.contains(REQUIRED_PHASE2A_SCOPED_WORDING)
        && !text.contains(REQUIRED_P0_CERT_SCOPED_WORDING)
        && !text.contains(REQUIRED_SPINE_CERT_SCOPED_WORDING)
        && !text.contains(REQUIRED_FIRST_PROOF_CERT_SCOPED_WORDING)
    {
        return Err("missing scoped exact-commit wording".to_string());
    }

    let result_claim_text = text_before_forbidden_wording_section(text);
    for forbidden in FORBIDDEN_RESULT_CLAIMS {
        if result_claim_text.contains(forbidden) {
            return Err(format!("forbidden overclaim wording: {forbidden}"));
        }
    }

    if text.contains(STATUS_FENCE) {
        let manifest = validate_status_manifest(text)?;
        let lower_claim_text = result_claim_text.to_ascii_lowercase();
        if manifest.computed_result == ComputedResult::NonPass
            && CONDITIONAL_CLOSURE_CLAIMS
                .iter()
                .any(|claim| lower_claim_text.contains(claim))
        {
            return Err("closure wording over non-pass status manifest".to_string());
        }
        if manifest.has_mutation_survivors
            && (lower_claim_text.contains("green canonical perimeter")
                || lower_claim_text.contains("canonical perimeter green"))
        {
            return Err("green perimeter wording with mutation survivors".to_string());
        }
        if text.contains("Branch protection is enforced")
            && !text.contains("Branch-protection API transcript:")
            && !text.contains("Ruleset API transcript:")
        {
            return Err("branch-protection enforcement claim lacks API transcript".to_string());
        }
        if lower_claim_text.contains("historical command results certify current") {
            return Err("historical command results as current certification".to_string());
        }
        if lower_claim_text.contains("display string as sole evidence")
            || lower_claim_text.contains("artifact existence as sole evidence")
            || lower_claim_text.contains("checksum as sole evidence")
            || lower_claim_text.contains("source guard as sole evidence")
        {
            return Err("display string as sole behavior evidence".to_string());
        }
    }

    Ok(())
}

fn text_before_forbidden_wording_section(text: &str) -> &str {
    text.split_once(FORBIDDEN_WORDING_HEADING)
        .map_or(text, |(result_claim_text, _)| result_claim_text)
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

fn open_findings() -> [&'static str; 6] {
    [
        "finding: F5-01 | closed | evidence=sealed bootstrap constructor | negative=external bootstrap negative fixture",
        "finding: F5-02 | open",
        "finding: F5-03 | closed | evidence=debug session authority token | negative=external debug command negative fixture",
        "finding: F5-04 | pending-governance",
        "finding: F5-05 | closed | evidence=food source projection behavior tests | negative=focused mutation survivor kills",
        "finding: F5-06 | closed | evidence=manifest consumer and wording guard | negative=synthetic contradictory manifest",
    ]
}

fn synthetic_status_manifest(
    overall_result: &str,
    branch_protection: &str,
    mutation_status: &str,
    mutation_survivors: &str,
    findings: &[&str],
    survivors: &[&str],
) -> String {
    let mut lines = vec![
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
