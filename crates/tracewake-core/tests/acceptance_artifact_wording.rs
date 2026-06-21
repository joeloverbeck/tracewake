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

    Ok(())
}

fn text_before_forbidden_wording_section(text: &str) -> &str {
    text.split_once(FORBIDDEN_WORDING_HEADING)
        .map_or(text, |(result_claim_text, _)| result_claim_text)
}
