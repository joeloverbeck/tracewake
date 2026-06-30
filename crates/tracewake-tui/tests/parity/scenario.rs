use tracewake_content::fixtures;
use tracewake_core::actions::{ReasonCode, ReportStatus};
use tracewake_core::events::EventKind;
use tracewake_core::ids::{ActionId, ActorId, SemanticActionId};
use tracewake_tui::app::{AppError, TuiApp};
use tracewake_tui::render::render_notebook;

use super::{CapabilityEntry, SetupOperation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioWitnesses {
    pub key: String,
    pub ordered_witnesses: Vec<String>,
    pub measured_evidence: ScenarioMeasuredEvidence,
    pub submitted_status: Option<ReportStatus>,
    pub rendered: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScenarioMeasuredEvidence {
    pub typed: bool,
    pub actor_knowledge: bool,
    pub rendered: bool,
    pub replay_match: bool,
    pub frontier_advanced: bool,
    pub marker_counted: bool,
    pub autonomous_work: bool,
    pub duration_terminal: bool,
    pub holder_known_sources: bool,
    pub typed_stop_reason: bool,
    pub debug_or_embodied_disposition: bool,
}

pub fn run_real_pipeline(entry: &CapabilityEntry) -> Result<ScenarioWitnesses, ScenarioError> {
    let fixture_id = entry
        .fixture_ids
        .first()
        .ok_or(ScenarioError::MissingFixture)?;
    let golden = fixtures::by_id(fixture_id)
        .ok_or_else(|| ScenarioError::UnknownFixture((*fixture_id).to_string()))?;
    let actor_id = ActorId::new(entry.viewer_actor.to_string())
        .map_err(|_| ScenarioError::BadActor(entry.viewer_actor.to_string()))?;
    if matches!(
        entry.setup_operation,
        SetupOperation::RenderDebugOverlay | SetupOperation::RunNoHumanDay
    ) {
        let mut app = TuiApp::from_golden_operator_debug(golden).map_err(ScenarioError::from)?;
        app.bind_debug_actor(actor_id)
            .map_err(ScenarioError::from)?;
        run_real_pipeline_with_app(entry, app)
    } else {
        let mut app = TuiApp::from_golden(golden).map_err(ScenarioError::from)?;
        app.bind_actor(actor_id).map_err(ScenarioError::from)?;
        run_real_pipeline_with_app(entry, app)
    }
}

fn run_real_pipeline_with_app(
    entry: &CapabilityEntry,
    mut app: TuiApp,
) -> Result<ScenarioWitnesses, ScenarioError> {
    let view = app.current_view().map_err(ScenarioError::from)?;
    let mut measured = ScenarioMeasuredEvidence {
        actor_knowledge: view
            .holder_known_context_id()
            .clone()
            .as_str()
            .starts_with(&format!("hkc.{}.", entry.viewer_actor))
            && !view.holder_known_context_hash().as_str().is_empty(),
        rendered: false,
        debug_or_embodied_disposition: true,
        ..ScenarioMeasuredEvidence::default()
    };
    measured.frontier_advanced = view.holder_known_context_frontier() as usize <= app.event_count();
    measured.actor_knowledge &= view
        .holder_known_context_id()
        .clone()
        .as_str()
        .starts_with(&format!("hkc.{}.", entry.viewer_actor));

    let mut submitted_status = None;
    let initial_event_count = app.event_count();
    let mut rendered = app.render_current_view().map_err(ScenarioError::from)?;
    match entry.setup_operation {
        SetupOperation::SubmitSemanticAction { semantic_action_id } => {
            let semantic_action_id = SemanticActionId::new(semantic_action_id.to_string())
                .map_err(|_| ScenarioError::BadSemanticAction(semantic_action_id.to_string()))?;
            let action = view
                .semantic_actions
                .iter()
                .find(|action| action.semantic_action_id == semantic_action_id)
                .ok_or_else(|| {
                    ScenarioError::MissingSemanticAction(semantic_action_id.to_string())
                })?;
            assert_render_contains_action(entry, &rendered, semantic_action_id.as_str());

            let result = app
                .submit_semantic_action(&semantic_action_id)
                .map_err(ScenarioError::from)?;
            submitted_status = Some(result.report.status.clone());
            measured.typed = if action.action_id.as_str() == "continue_routine" {
                result.report.status == ReportStatus::Accepted
                    && result
                        .appended_events
                        .iter()
                        .any(|event| event.event_type == EventKind::ContinueRoutineProposed)
            } else {
                result.report.action_id == action.action_id
            };
            if !action.availability.is_available() {
                rendered = app.render_current_view().map_err(ScenarioError::from)?;
                assert_actor_safe_why_not(entry, &rendered);
            }
        }
        SetupOperation::HumanWaitOneTick => {
            let semantic_action_id = semantic_action_for_action(&view, "wait")?;
            assert_render_contains_action(entry, &rendered, semantic_action_id.as_str());
            let result = app
                .submit_semantic_action(&semantic_action_id)
                .map_err(ScenarioError::from)?;
            submitted_status = Some(result.report.status.clone());
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
            measured.typed = result.report.status == ReportStatus::Accepted;
            measured.frontier_advanced = app.event_count() > initial_event_count;
            measured.marker_counted = app.event_count() > initial_event_count;
        }
        SetupOperation::StartSleepThenAdvanceUntil { max_ticks } => {
            submit_semantic_action_by_id(&mut app, "sleep.here")?;
            let result = app.advance_until(max_ticks).map_err(ScenarioError::from)?;
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
            measure_advance_until(&mut measured, &app, &result);
        }
        SetupOperation::MoveWorkThenAdvanceUntil { max_ticks } => {
            submit_semantic_action_by_id(&mut app, "move.to.workshop_tomas")?;
            submit_semantic_action_by_id(&mut app, "work.block.workplace_tomas")?;
            let result = app.advance_until(max_ticks).map_err(ScenarioError::from)?;
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
            measure_advance_until(&mut measured, &app, &result);
        }
        SetupOperation::ContinueRoutineWorkday { max_ticks } => {
            let first_continue = semantic_action_for_action(
                &app.current_view().map_err(ScenarioError::from)?,
                "continue_routine",
            )?;
            assert_render_contains_action(entry, &rendered, first_continue.as_str());
            let moved = app
                .submit_semantic_action(&first_continue)
                .map_err(ScenarioError::from)?;

            let second_continue = semantic_action_for_action(
                &app.current_view().map_err(ScenarioError::from)?,
                "continue_routine",
            )?;
            let worked = app
                .submit_semantic_action(&second_continue)
                .map_err(ScenarioError::from)?;
            submitted_status = Some(worked.report.status.clone());

            let moved_by_continue = moved.report.status == ReportStatus::Accepted
                && moved
                    .appended_events
                    .iter()
                    .any(|event| event.event_type == EventKind::ContinueRoutineProposed)
                && moved
                    .appended_events
                    .iter()
                    .any(|event| event.event_type == EventKind::ActorMoved);
            let worked_by_continue = worked.report.status == ReportStatus::Accepted
                && worked
                    .appended_events
                    .iter()
                    .any(|event| event.event_type == EventKind::ContinueRoutineProposed)
                && worked
                    .appended_events
                    .iter()
                    .any(|event| event.event_type == EventKind::WorkBlockStarted);

            let result = app.advance_until(max_ticks).map_err(ScenarioError::from)?;
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
            measure_advance_until(&mut measured, &app, &result);
            measured.typed = moved_by_continue && worked_by_continue && measured.typed;
            measured.marker_counted = moved_by_continue && worked_by_continue;
            measured.autonomous_work = worked_by_continue;
        }
        SetupOperation::StartSleepThenWaitConflict => {
            submit_semantic_action_by_id(&mut app, "sleep.here")?;
            let view = app.current_view().map_err(ScenarioError::from)?;
            let semantic_action_id = semantic_action_for_action(&view, "wait")?;
            let result = app
                .submit_semantic_action(&semantic_action_id)
                .map_err(ScenarioError::from)?;
            submitted_status = Some(result.report.status.clone());
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
            measured.typed = result.report.status == ReportStatus::Rejected
                && result
                    .report
                    .reason_codes
                    .contains(&ReasonCode::ReservationConflict);
            assert_actor_safe_why_not(entry, &rendered);
        }
        SetupOperation::SubmitRegistryAction { action_id } => {
            let action_id = ActionId::new(action_id.to_string())
                .map_err(|_| ScenarioError::BadRegistryAction(action_id.to_string()))?;
            let action = view
                .semantic_actions
                .iter()
                .find(|action| action.action_id == action_id)
                .ok_or_else(|| ScenarioError::MissingRegistryAction(action_id.to_string()))?;
            let semantic_action_id = action.semantic_action_id.clone();
            assert_render_contains_action(entry, &rendered, semantic_action_id.as_str());

            let result = app
                .submit_semantic_action(&semantic_action_id)
                .map_err(ScenarioError::from)?;
            submitted_status = Some(result.report.status.clone());
            measured.typed = if action.action_id.as_str() == "continue_routine" {
                result.report.status == ReportStatus::Accepted
                    && result
                        .appended_events
                        .iter()
                        .any(|event| event.event_type == EventKind::ContinueRoutineProposed)
            } else {
                result.report.action_id == action.action_id
            };
            if !action.availability.is_available() {
                rendered = app.render_current_view().map_err(ScenarioError::from)?;
                assert_actor_safe_why_not(entry, &rendered);
            }
        }
        SetupOperation::ObserveQueryOnly { action_id } => {
            assert_eq!(
                entry.registry_action_id,
                Some(action_id),
                "{} query-only witness must name the registered action",
                entry.key
            );
            measured.typed = true;
        }
        SetupOperation::RenderNotebook => {
            rendered = render_notebook(&app.notebook_view().map_err(ScenarioError::from)?);
            measured.typed = true;
        }
        SetupOperation::RenderDebugOverlay => {
            rendered = app
                .render_debug_embodied_overlay()
                .map_err(ScenarioError::from)?
                .ok_or(ScenarioError::MissingDebugOverlay)?;
            measured.typed = rendered.contains("DEBUG NON-DIEGETIC");
            measured.debug_or_embodied_disposition = measured.typed;
        }
        SetupOperation::RunNoHumanDay => {
            let report = app.run_no_human_day().map_err(ScenarioError::from)?;
            measured.typed =
                report.final_tick > report.start_tick && report.scheduler_errors.is_empty();
            measured.marker_counted = !report.marker_event_ids.is_empty();
            measured.autonomous_work =
                !report.actor_decision_order.is_empty() || report.ordinary_pipeline_events > 0;
            rendered = app.render_current_view().map_err(ScenarioError::from)?;
        }
        SetupOperation::BindViewer | SetupOperation::AdvanceNoHuman => {
            measured.typed = true;
        }
    }

    if let Some(rendered_witness) = &entry.rendered_witness {
        assert!(
            !rendered_witness.assertion.trim().is_empty(),
            "{} rendered witness must be declared",
            entry.key
        );
    }

    measured.rendered = !rendered.trim().is_empty();
    measured.replay_match = app
        .render_debug_projection_rebuild_panel()
        .contains("diffs=0");

    Ok(ScenarioWitnesses {
        key: entry.key.to_string(),
        ordered_witnesses: vec![
            format!("typed_measured={}", measured.typed),
            format!("actor_knowledge_measured={}", measured.actor_knowledge),
            entry
                .rendered_witness
                .as_ref()
                .map(|_| format!("rendered_measured={}", measured.rendered))
                .unwrap_or_else(|| "rendered_measured=not_required".to_string()),
        ],
        measured_evidence: measured,
        submitted_status,
        rendered,
    })
}

fn measure_advance_until(
    measured: &mut ScenarioMeasuredEvidence,
    app: &TuiApp,
    result: &tracewake_core::runtime::ContinuedRuntimeReceipt,
) {
    measured.typed = result.advanced() && result.appended_event_count() > 0;
    measured.frontier_advanced = result.advanced();
    measured.marker_counted = result.appended_event_count() > 0;
    measured.duration_terminal = result.actor_known_interval_summary().is_some();
    measured.typed_stop_reason = result.actor_known_interval_summary().is_some();
    if let Ok(view) = app.current_view() {
        if let Some(summary) = view.actor_known_interval_summary() {
            measured.holder_known_sources =
                !summary.no_new_actor_known_information() || !summary.notices().is_empty();
        }
    }
}

fn submit_semantic_action_by_id(
    app: &mut TuiApp,
    semantic_action_id: &str,
) -> Result<ReportStatus, ScenarioError> {
    let semantic_action_id = SemanticActionId::new(semantic_action_id.to_string())
        .map_err(|_| ScenarioError::BadSemanticAction(semantic_action_id.to_string()))?;
    let result = app
        .submit_semantic_action(&semantic_action_id)
        .map_err(ScenarioError::from)?;
    Ok(result.report.status)
}

fn semantic_action_for_action(
    view: &tracewake_core::view_models::EmbodiedViewModel,
    action_id: &str,
) -> Result<SemanticActionId, ScenarioError> {
    let action_id = ActionId::new(action_id.to_string())
        .map_err(|_| ScenarioError::BadRegistryAction(action_id.to_string()))?;
    view.semantic_actions
        .iter()
        .find(|action| action.action_id == action_id)
        .map(|action| action.semantic_action_id.clone())
        .ok_or_else(|| ScenarioError::MissingRegistryAction(action_id.as_str().to_string()))
}

pub fn assert_matches_checked_in_golden(entry: &CapabilityEntry, rendered: &str) {
    let Some(golden_path) = entry.golden_path else {
        return;
    };
    let package_path = golden_path
        .strip_prefix("crates/tracewake-tui/")
        .unwrap_or(golden_path);
    #[expect(
        clippy::disallowed_methods,
        reason = "test-only parity harness compares rendered output against checked-in golden files"
    )]
    let expected = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(package_path),
    )
    .unwrap_or_else(|error| panic!("failed to read scenario golden {golden_path}: {error}"));
    let actual = format!("{rendered}\n");
    assert_eq!(
        expected, actual,
        "{} rendered witness must match checked-in golden {}",
        entry.key, golden_path
    );
}

fn assert_render_contains_action(
    entry: &CapabilityEntry,
    rendered: &str,
    semantic_action_id: &str,
) {
    assert!(
        rendered.contains(semantic_action_id),
        "{} rendered witness must include {semantic_action_id} before submission",
        entry.key
    );
}

fn assert_actor_safe_why_not(entry: &CapabilityEntry, rendered: &str) {
    assert!(
        rendered.contains("Why-not:"),
        "{} disabled action must render actor-safe why-not after rejection",
        entry.key
    );
}

pub fn assert_actor_surface_does_not_leak(entry: &CapabilityEntry, rendered: &str) {
    if entry.anti_leak_fixtures.is_empty() {
        return;
    }
    for forbidden in [
        "DEBUG NON-DIEGETIC",
        "Knowledge context",
        "holder_known_context",
        "debug_diagnostics",
        "culprit",
        "food_hidden_pantry",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "{} anti-leak rendered witness exposed {forbidden}",
            entry.key
        );
    }
    if matches!(
        entry.setup_operation,
        SetupOperation::SubmitSemanticAction { .. } | SetupOperation::SubmitRegistryAction { .. }
    ) {
        assert!(
            rendered.contains("Why-not:"),
            "{} anti-leak exemplar must keep an actor-safe why-not reason",
            entry.key
        );
    }
}

pub enum ScenarioError {
    MissingFixture,
    UnknownFixture(String),
    BadActor(String),
    BadRegistryAction(String),
    BadSemanticAction(String),
    MissingRegistryAction(String),
    MissingSemanticAction(String),
    MissingDebugOverlay,
    App(Box<AppError>),
}

impl From<AppError> for ScenarioError {
    fn from(error: AppError) -> Self {
        Self::App(Box::new(error))
    }
}

impl std::fmt::Debug for ScenarioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, formatter)
    }
}

impl std::fmt::Display for ScenarioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFixture => write!(formatter, "missing fixture"),
            Self::UnknownFixture(fixture_id) => write!(formatter, "unknown fixture {fixture_id}"),
            Self::BadActor(actor_id) => write!(formatter, "bad actor id {actor_id}"),
            Self::BadRegistryAction(action_id) => {
                write!(formatter, "bad registry action id {action_id}")
            }
            Self::BadSemanticAction(action_id) => {
                write!(formatter, "bad semantic action id {action_id}")
            }
            Self::MissingRegistryAction(action_id) => {
                write!(formatter, "missing registry action {action_id}")
            }
            Self::MissingSemanticAction(action_id) => {
                write!(formatter, "missing semantic action {action_id}")
            }
            Self::MissingDebugOverlay => write!(formatter, "missing debug overlay"),
            Self::App(error) => write!(formatter, "app error: {error:?}"),
        }
    }
}
