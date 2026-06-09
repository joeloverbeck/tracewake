const TEMPLATE: &str = include_str!("../../../docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md");

const REQUIRED_SCOPED_WORDING: &str =
    "Phase 1 / Phase 1A third hardening and lock-layer remediation accepted for exact commit";

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
fn acceptance_artifact_forbidden_overclaim_phrase_fails() {
    let artifact =
        format!("{REQUIRED_SCOPED_WORDING} `<commit>`.\n\nTracewake is fully certified.");

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
    if !text.contains(REQUIRED_SCOPED_WORDING) {
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
