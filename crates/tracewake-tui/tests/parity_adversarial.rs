#![allow(dead_code)]

mod parity;

use std::panic::{catch_unwind, AssertUnwindSafe};

use parity::{
    registry,
    runner::{
        registered_action_coverage_failures, run_conformance,
        run_conformance_with_measurement_probe,
    },
    scenario::assert_actor_surface_does_not_leak,
    scenario::ScenarioMeasuredEvidence,
    EvidenceFlag,
};

#[test]
fn parity_adversarial_witness_drop_fails_runner() {
    let complete = registry();
    assert!(run_conformance(&complete).is_pass());

    let mut missing_witness = complete
        .iter()
        .find(|entry| !entry.actor_knowledge_witness.assertion.trim().is_empty())
        .expect("live registry must include actor-knowledge witnesses")
        .clone();
    missing_witness.key = "synthetic.adversarial.missing_actor_knowledge";
    missing_witness.actor_knowledge_witness.assertion = "";

    let report = run_conformance(&[missing_witness]);
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.code == "missing_actor_knowledge_witness"),
        "dropping a capability witness must fail closed: {:#?}",
        report.failures
    );
}

#[test]
fn parity_adversarial_uncovered_registered_action_fails_runner() {
    let complete = registry();
    let failures = registered_action_coverage_failures(
        &complete,
        ["wait", "synthetic_adversarial_registered_action"],
    );

    assert!(
        failures.iter().any(|failure| {
            failure.code == "registered_action_uncovered"
                && failure.key.as_deref() == Some("synthetic_adversarial_registered_action")
        }),
        "a registered action with no coverage must fail: {failures:#?}"
    );
}

#[test]
fn parity_adversarial_debug_hidden_truth_leak_fails_actor_surface_check() {
    let complete = registry();
    let leak_checked = complete
        .iter()
        .find(|entry| !entry.anti_leak_fixtures.is_empty())
        .expect("live registry must include anti-leak exemplars");

    let leaked_render = "Why-not: actor-safe placeholder\nDEBUG NON-DIEGETIC\nfood_hidden_pantry";
    let result = catch_unwind(AssertUnwindSafe(|| {
        assert_actor_surface_does_not_leak(leak_checked, leaked_render);
    }));

    assert!(
        result.is_err(),
        "debug or hidden truth injected into an embodied assertion must fail"
    );
}

#[test]
fn parity_adversarial_source_guard_target_removal_fails() {
    let render_source = include_str!("../src/render.rs");
    let view_model_source = include_str!("../../tracewake-core/src/view_models.rs");
    assert!(
        embodied_render_source_guard_errors(render_source, view_model_source).is_empty(),
        "live render source must satisfy the PAR-002 source guard"
    );

    let removed_deny = render_source.replace(
        "#[deny(unused_variables)]\npub fn render_embodied_view(",
        "pub fn render_embodied_view(",
    );
    assert!(
        embodied_render_source_guard_errors(&removed_deny, view_model_source)
            .iter()
            .any(|error| error.contains("unused_variables deny")),
        "removing the source guard target must fail"
    );

    let rest_pattern = render_source.replace(
        "debug_available: _debug_available,\n    } = view;",
        "debug_available: _debug_available,\n        ..\n    } = view;",
    );
    assert!(
        embodied_render_source_guard_errors(&rest_pattern, view_model_source)
            .iter()
            .any(|error| error.contains("rest pattern")),
        "adding a wildcard rest pattern must fail the source guard"
    );
}

#[test]
fn parity_adversarial_missing_actor_invocation_fails_typed_evidence() {
    let entry = registry_entry_with(|entry| !entry.typed_witness.assertion.trim().is_empty());
    let report = run_conformance_with_measurement_probe(std::slice::from_ref(&entry), |_| {
        Some(ScenarioMeasuredEvidence {
            actor_knowledge: true,
            rendered: true,
            replay_match: true,
            debug_or_embodied_disposition: true,
            ..ScenarioMeasuredEvidence::default()
        })
    });

    assert_failure(
        &report.failures,
        entry.key,
        "missing_measured_typed_evidence",
    );
}

#[test]
fn parity_adversarial_missing_world_process_fails_no_human_evidence() {
    let entry = registry_entry_with(|entry| entry.key == "base.family.no_human_autonomy");
    let report = run_conformance_with_measurement_probe(std::slice::from_ref(&entry), |_| {
        Some(ScenarioMeasuredEvidence {
            typed: true,
            actor_knowledge: true,
            rendered: true,
            replay_match: true,
            debug_or_embodied_disposition: true,
            ..ScenarioMeasuredEvidence::default()
        })
    });

    assert_failure(
        &report.failures,
        entry.key,
        "missing_measured_no_human_work",
    );
}

#[test]
fn parity_adversarial_hidden_same_actor_id_source_fails_holder_known_evidence() {
    let entry =
        registry_entry_with(|entry| !entry.actor_knowledge_witness.assertion.trim().is_empty());
    let report = run_conformance_with_measurement_probe(std::slice::from_ref(&entry), |_| {
        Some(ScenarioMeasuredEvidence {
            typed: true,
            rendered: true,
            replay_match: true,
            debug_or_embodied_disposition: true,
            ..ScenarioMeasuredEvidence::default()
        })
    });

    assert_failure(
        &report.failures,
        entry.key,
        "missing_measured_actor_knowledge",
    );
}

#[test]
fn parity_adversarial_removed_temporal_marker_fails_replay_evidence() {
    let entry =
        registry_entry_with(|entry| matches!(entry.replay_evidence, EvidenceFlag::Required));
    let report = run_conformance_with_measurement_probe(std::slice::from_ref(&entry), |_| {
        Some(ScenarioMeasuredEvidence {
            typed: true,
            actor_knowledge: true,
            rendered: true,
            debug_or_embodied_disposition: true,
            ..ScenarioMeasuredEvidence::default()
        })
    });

    assert_failure(&report.failures, entry.key, "missing_measured_replay");
}

fn registry_entry_with(
    predicate: impl Fn(&parity::CapabilityEntry) -> bool,
) -> parity::CapabilityEntry {
    registry()
        .into_iter()
        .find(predicate)
        .expect("registry contains a matching adversarial entry")
}

fn assert_failure(failures: &[parity::runner::ConformanceFailure], key: &str, code: &'static str) {
    assert!(
        failures
            .iter()
            .any(|failure| failure.key.as_deref() == Some(key) && failure.code == code),
        "expected failure {code} for {key}, got {failures:#?}"
    );
}

fn embodied_render_source_guard_errors(
    render_source: &str,
    view_model_source: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    let function_body = source_after(render_source, "pub fn render_embodied_view(");
    let destructure = source_between(function_body, "let EmbodiedViewModel {", "} = view;");

    if destructure.contains("..") {
        errors.push("render_embodied_view contains a rest pattern".to_string());
    }
    if destructure
        .lines()
        .any(|line| line.trim().trim_end_matches(',') == "_")
    {
        errors.push("render_embodied_view contains a bare wildcard".to_string());
    }
    if !render_source.contains("#[deny(unused_variables)]\npub fn render_embodied_view(") {
        errors.push("render_embodied_view is missing the local unused_variables deny".to_string());
    }

    for field_name in embodied_view_model_field_names(view_model_source) {
        if !destructure.contains(&format!("{field_name}:"))
            && !destructure
                .lines()
                .any(|line| line.trim().starts_with(&format!("{field_name},")))
        {
            errors.push(format!(
                "render_embodied_view destructure is missing EmbodiedViewModel field {field_name}"
            ));
        }
    }

    errors
}

fn source_after<'a>(source: &'a str, needle: &str) -> &'a str {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("missing source marker {needle}"));
    &source[start..]
}

fn source_between<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("missing source marker {start_marker}"));
    let after_start = start + start_marker.len();
    let end = source[after_start..]
        .find(end_marker)
        .unwrap_or_else(|| panic!("missing source marker {end_marker}"))
        + after_start;
    &source[after_start..end]
}

fn embodied_view_model_field_names(source: &str) -> Vec<&str> {
    let struct_body = source_between(source, "pub struct EmbodiedViewModel {", "}\n\n#[derive");
    struct_body
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("pub ")
                .and_then(|field| field.split_once(':'))
                .map(|(field_name, _)| field_name.trim())
        })
        .collect()
}
