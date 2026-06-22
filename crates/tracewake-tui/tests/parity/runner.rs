use tracewake_content::fixtures;
use tracewake_core::ids::{ActionId, ActorId, SemanticActionId};
use tracewake_tui::app::TuiApp;

use super::{
    CapabilityClass, CapabilityEntry, EvidenceFlag, SetupOperation, SurfaceDisposition, WitnessKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoverageReport {
    pub rows: Vec<CoverageRow>,
    pub failures: Vec<ConformanceFailure>,
}

impl CoverageReport {
    pub fn is_pass(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn to_deterministic_text(&self) -> String {
        let mut lines = vec![format!("capability_count={}", self.rows.len())];
        for row in &self.rows {
            lines.push(format!(
                "capability key={} class={} fixtures={} typed={} rendered={} negative={} replay={} no_human={} status={}",
                row.key,
                row.class,
                row.fixture_ids.join(","),
                row.typed_witness,
                row.rendered_witness,
                row.negative_witness,
                row.replay_status,
                row.no_human_status,
                row.status
            ));
        }
        for failure in &self.failures {
            lines.push(format!(
                "failure key={} code={} message={}",
                failure.key.as_deref().unwrap_or("registry"),
                failure.code,
                failure.message
            ));
        }
        lines.join("\n")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoverageRow {
    pub key: String,
    pub class: &'static str,
    pub fixture_ids: Vec<String>,
    pub typed_witness: bool,
    pub rendered_witness: bool,
    pub negative_witness: bool,
    pub replay_status: &'static str,
    pub no_human_status: &'static str,
    pub status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConformanceFailure {
    pub key: Option<String>,
    pub code: &'static str,
    pub message: String,
}

pub fn run_conformance(entries: &[CapabilityEntry]) -> CoverageReport {
    run_conformance_with_render_probe(entries, real_embodied_render)
}

pub fn run_conformance_with_render_probe(
    entries: &[CapabilityEntry],
    render_probe: impl Fn(&CapabilityEntry) -> Option<String>,
) -> CoverageReport {
    let mut sorted_entries = entries.iter().collect::<Vec<_>>();
    sorted_entries.sort_by_key(|entry| entry.key);

    let mut failures = Vec::new();
    validate_keys(entries, &mut failures);

    let mut rows = Vec::new();
    for entry in sorted_entries {
        validate_entry(entry, &mut failures, &render_probe);
        rows.push(CoverageRow {
            key: entry.key.to_string(),
            class: class_label(&entry.capability_class),
            fixture_ids: entry
                .fixture_ids
                .iter()
                .map(|fixture_id| (*fixture_id).to_string())
                .collect(),
            typed_witness: !entry.typed_witness.assertion.trim().is_empty(),
            rendered_witness: entry
                .rendered_witness
                .as_ref()
                .is_some_and(|witness| !witness.assertion.trim().is_empty()),
            negative_witness: !entry.anti_leak_fixtures.is_empty(),
            replay_status: evidence_label(&entry.replay_evidence),
            no_human_status: evidence_label(&entry.no_human_evidence),
            status: if failures.iter().any(|failure| {
                failure
                    .key
                    .as_ref()
                    .is_some_and(|failure_key| failure_key == entry.key)
            }) {
                "fail"
            } else {
                "pass"
            },
        });
    }

    CoverageReport { rows, failures }
}

pub fn registered_action_coverage_failures(
    entries: &[CapabilityEntry],
    action_ids: impl IntoIterator<Item = impl AsRef<str>>,
) -> Vec<ConformanceFailure> {
    action_ids
        .into_iter()
        .filter_map(|action_id| {
            let action_id = action_id.as_ref();
            (!entries.iter().any(|entry| {
                entry.registry_action_id == Some(action_id) && !entry.fixture_ids.is_empty()
            }))
            .then(|| action_id.to_string())
        })
        .map(|action_id| ConformanceFailure {
            key: Some(action_id.clone()),
            code: "registered_action_uncovered",
            message: format!("registered action {action_id} has no capability disposition"),
        })
        .collect()
}

fn validate_keys(entries: &[CapabilityEntry], failures: &mut Vec<ConformanceFailure>) {
    let keys = entries.iter().map(|entry| entry.key).collect::<Vec<_>>();
    if keys.iter().any(|key| key.trim().is_empty()) {
        failures.push(registry_failure(
            "empty_key",
            "capability keys must not be empty",
        ));
    }

    let mut sorted = keys.clone();
    sorted.sort_unstable();
    if keys != sorted {
        failures.push(registry_failure(
            "keys_not_sorted",
            "capability keys must be deterministic",
        ));
    }

    for window in sorted.windows(2) {
        if let [left, right] = window {
            if left == right {
                failures.push(registry_failure(
                    "duplicate_key",
                    format!("duplicate capability key {left}"),
                ));
            }
        }
    }
}

fn validate_entry(
    entry: &CapabilityEntry,
    failures: &mut Vec<ConformanceFailure>,
    render_probe: &impl Fn(&CapabilityEntry) -> Option<String>,
) {
    let key = Some(entry.key.to_string());
    if entry.disposition_rationale.trim().is_empty() {
        failures.push(entry_failure(
            &key,
            "missing_rationale",
            "missing rationale",
        ));
    }
    if entry.fixture_ids.is_empty() {
        failures.push(entry_failure(&key, "missing_fixture", "missing fixture"));
    }
    for fixture_id in &entry.fixture_ids {
        if fixtures::by_id(fixture_id).is_none() {
            failures.push(entry_failure(
                &key,
                "unknown_fixture",
                format!("unknown fixture {fixture_id}"),
            ));
        }
    }
    for fixture_id in &entry.anti_leak_fixtures {
        if fixtures::by_id(fixture_id).is_none() {
            failures.push(entry_failure(
                &key,
                "unknown_anti_leak_fixture",
                format!("unknown anti-leak fixture {fixture_id}"),
            ));
        }
    }
    if entry.typed_witness.assertion.trim().is_empty() {
        failures.push(entry_failure(
            &key,
            "missing_typed_witness",
            "missing typed witness",
        ));
    }
    if entry.actor_knowledge_witness.assertion.trim().is_empty() {
        failures.push(entry_failure(
            &key,
            "missing_actor_knowledge_witness",
            "missing actor-knowledge witness",
        ));
    }
    if rendered_surface_requires_witness(&entry.surface_disposition)
        && entry
            .rendered_witness
            .as_ref()
            .is_none_or(|witness| witness.assertion.trim().is_empty())
    {
        failures.push(entry_failure(
            &key,
            "missing_rendered_witness",
            "missing rendered witness",
        ));
    }
    if anti_leak_required(entry) && entry.anti_leak_fixtures.is_empty() {
        failures.push(entry_failure(
            &key,
            "missing_anti_leak",
            "epistemic capability is missing anti-leak fixture coverage",
        ));
    }
    match entry.setup_operation {
        SetupOperation::SubmitSemanticAction { semantic_action_id } => {
            validate_semantic_action(entry, semantic_action_id, failures);
        }
        SetupOperation::SubmitRegistryAction { action_id } => {
            validate_registry_action(entry, action_id, failures);
        }
        SetupOperation::ObserveQueryOnly { action_id } => {
            validate_query_action(entry, action_id, failures);
        }
        SetupOperation::BindViewer
        | SetupOperation::AdvanceNoHuman
        | SetupOperation::RenderNotebook
        | SetupOperation::RenderDebugOverlay
        | SetupOperation::RunNoHumanDay => {}
    }
    if matches!(entry.surface_disposition, SurfaceDisposition::Embodied) {
        validate_embodied_render(entry, failures, render_probe);
    }
}

fn validate_registry_action(
    entry: &CapabilityEntry,
    action_id: &str,
    failures: &mut Vec<ConformanceFailure>,
) {
    let Some(fixture_id) = entry.fixture_ids.first() else {
        return;
    };
    let Some(golden) = fixtures::by_id(fixture_id) else {
        return;
    };
    let Ok(mut app) = TuiApp::from_golden(golden) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "fixture_load_failed",
            format!("fixture {fixture_id} failed to load"),
        ));
        return;
    };
    let Ok(actor_id) = ActorId::new(entry.viewer_actor) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "bad_viewer_actor",
            format!("bad viewer actor {}", entry.viewer_actor),
        ));
        return;
    };
    if app.bind_actor(actor_id).is_err() {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "viewer_bind_failed",
            format!("viewer actor {} could not bind", entry.viewer_actor),
        ));
        return;
    }
    let Ok(action_id) = ActionId::new(action_id) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "bad_registry_action_id",
            "registry action id is malformed",
        ));
        return;
    };
    let action_present = app.current_view().is_ok_and(|view| {
        view.semantic_actions
            .iter()
            .any(|action| action.action_id == action_id)
    });
    if !action_present {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "declared_registry_action_absent",
            "declared registry action is absent from the positive scenario",
        ));
    }
}

fn validate_query_action(
    entry: &CapabilityEntry,
    action_id: &str,
    failures: &mut Vec<ConformanceFailure>,
) {
    if entry.registry_action_id != Some(action_id) {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "query_action_mismatch",
            "query-only setup action must match registry action id",
        ));
    }
}

fn validate_semantic_action(
    entry: &CapabilityEntry,
    semantic_action_id: &str,
    failures: &mut Vec<ConformanceFailure>,
) {
    let Some(fixture_id) = entry.fixture_ids.first() else {
        return;
    };
    let Some(golden) = fixtures::by_id(fixture_id) else {
        return;
    };
    let Ok(mut app) = TuiApp::from_golden(golden) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "fixture_load_failed",
            format!("fixture {fixture_id} failed to load"),
        ));
        return;
    };
    let Ok(actor_id) = ActorId::new(entry.viewer_actor) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "bad_viewer_actor",
            format!("bad viewer actor {}", entry.viewer_actor),
        ));
        return;
    };
    if app.bind_actor(actor_id).is_err() {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "viewer_bind_failed",
            format!("viewer actor {} could not bind", entry.viewer_actor),
        ));
        return;
    }
    let Ok(semantic_action_id) = SemanticActionId::new(semantic_action_id) else {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "bad_semantic_action_id",
            "semantic action id is malformed",
        ));
        return;
    };
    let action_present = app.current_view().is_ok_and(|view| {
        view.semantic_actions
            .iter()
            .any(|action| action.semantic_action_id == semantic_action_id)
    });
    if !action_present {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "declared_action_absent",
            "declared semantic action is absent from the positive scenario",
        ));
    }
}

fn validate_embodied_render(
    entry: &CapabilityEntry,
    failures: &mut Vec<ConformanceFailure>,
    render_probe: &impl Fn(&CapabilityEntry) -> Option<String>,
) {
    if render_probe(entry).is_some_and(|rendered| rendered.trim().is_empty()) {
        failures.push(entry_failure(
            &Some(entry.key.to_string()),
            "empty_embodied_render",
            "embodied capability rendered empty output",
        ));
    }
}

fn real_embodied_render(entry: &CapabilityEntry) -> Option<String> {
    let fixture_id = entry.fixture_ids.first()?;
    let golden = fixtures::by_id(fixture_id)?;
    let Ok(mut app) = TuiApp::from_golden(golden) else {
        return None;
    };
    let Ok(actor_id) = ActorId::new(entry.viewer_actor) else {
        return None;
    };
    if app.bind_actor(actor_id).is_err() {
        return None;
    }
    app.render_current_view().ok()
}

fn class_label(class: &CapabilityClass) -> &'static str {
    match class {
        CapabilityClass::SemanticAction => "semantic_action",
        CapabilityClass::ActorObservableState => "actor_observable_state",
        CapabilityClass::ActorObservableConsequence => "actor_observable_consequence",
        CapabilityClass::NotebookRecordSurface => "notebook_record_surface",
        CapabilityClass::DebugOnlyInfrastructure => "debug_only_infrastructure",
        CapabilityClass::HeadlessInfrastructure => "headless_infrastructure",
    }
}

fn evidence_label(flag: &EvidenceFlag) -> &'static str {
    match flag {
        EvidenceFlag::Required => "required",
        EvidenceFlag::NotApplicable { .. } => "not_applicable",
    }
}

fn rendered_surface_requires_witness(disposition: &SurfaceDisposition) -> bool {
    match disposition {
        SurfaceDisposition::Embodied => true,
        SurfaceDisposition::Notebook => true,
        SurfaceDisposition::DebugOnly => true,
        SurfaceDisposition::Headless => false,
    }
}

fn anti_leak_required(entry: &CapabilityEntry) -> bool {
    matches!(
        entry.capability_class,
        CapabilityClass::NotebookRecordSurface
    ) || matches!(
        entry.typed_witness.kind,
        WitnessKind::ActorKnowledge | WitnessKind::AntiLeakNegative
    ) || entry.key.contains("epistemic")
        || entry.key.contains("belief")
        || entry.key.contains("notebook")
}

fn registry_failure(code: &'static str, message: impl Into<String>) -> ConformanceFailure {
    ConformanceFailure {
        key: None,
        code,
        message: message.into(),
    }
}

fn entry_failure(
    key: &Option<String>,
    code: &'static str,
    message: impl Into<String>,
) -> ConformanceFailure {
    ConformanceFailure {
        key: key.clone(),
        code,
        message: message.into(),
    }
}
