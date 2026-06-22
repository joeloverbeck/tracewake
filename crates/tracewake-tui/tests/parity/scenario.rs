use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActionId, ActorId, SemanticActionId};
use tracewake_tui::app::{AppError, TuiApp};
use tracewake_tui::render::render_notebook;

use super::{CapabilityEntry, SetupOperation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioWitnesses {
    pub key: String,
    pub ordered_witnesses: Vec<&'static str>,
    pub submitted_status: Option<ReportStatus>,
    pub rendered: String,
}

pub fn run_real_pipeline(entry: &CapabilityEntry) -> Result<ScenarioWitnesses, ScenarioError> {
    let fixture_id = entry
        .fixture_ids
        .first()
        .ok_or(ScenarioError::MissingFixture)?;
    let golden = fixtures::by_id(fixture_id)
        .ok_or_else(|| ScenarioError::UnknownFixture((*fixture_id).to_string()))?;
    let mut app = TuiApp::from_golden(golden).map_err(ScenarioError::App)?;
    let actor_id = ActorId::new(entry.viewer_actor.to_string())
        .map_err(|_| ScenarioError::BadActor(entry.viewer_actor.to_string()))?;
    if matches!(
        entry.setup_operation,
        SetupOperation::RenderDebugOverlay | SetupOperation::RunNoHumanDay
    ) {
        app.bind_debug_actor(actor_id).map_err(ScenarioError::App)?;
    } else {
        app.bind_actor(actor_id).map_err(ScenarioError::App)?;
    }

    let view = app.current_view().map_err(ScenarioError::App)?;
    assert!(
        view.holder_known_context_id
            .as_str()
            .starts_with(&format!("hkc.{}.", entry.viewer_actor)),
        "{} actor-knowledge witness must come from the bound actor context",
        entry.key
    );
    assert!(
        !entry.typed_witness.assertion.trim().is_empty(),
        "{} typed witness must be declared",
        entry.key
    );
    assert!(
        !entry.actor_knowledge_witness.assertion.trim().is_empty(),
        "{} actor-knowledge witness must be declared",
        entry.key
    );

    let mut submitted_status = None;
    let mut rendered = app.render_current_view().map_err(ScenarioError::App)?;
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
                .map_err(ScenarioError::App)?;
            submitted_status = Some(result.report.status);
            if !action.availability.is_available() {
                rendered = app.render_current_view().map_err(ScenarioError::App)?;
                assert_actor_safe_why_not(entry, &rendered);
            }
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
                .map_err(ScenarioError::App)?;
            submitted_status = Some(result.report.status);
            if !action.availability.is_available() {
                rendered = app.render_current_view().map_err(ScenarioError::App)?;
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
        }
        SetupOperation::RenderNotebook => {
            rendered = render_notebook(&app.notebook_view().map_err(ScenarioError::App)?);
        }
        SetupOperation::RenderDebugOverlay => {
            rendered = app
                .render_debug_embodied_overlay()
                .map_err(ScenarioError::App)?
                .ok_or(ScenarioError::MissingDebugOverlay)?;
        }
        SetupOperation::RunNoHumanDay => {
            app.run_no_human_day().map_err(ScenarioError::App)?;
            rendered = app.render_current_view().map_err(ScenarioError::App)?;
        }
        SetupOperation::BindViewer | SetupOperation::AdvanceNoHuman => {}
    }

    if let Some(rendered_witness) = &entry.rendered_witness {
        assert!(
            !rendered_witness.assertion.trim().is_empty(),
            "{} rendered witness must be declared",
            entry.key
        );
    }

    Ok(ScenarioWitnesses {
        key: entry.key.to_string(),
        ordered_witnesses: vec![
            entry.typed_witness.assertion,
            entry.actor_knowledge_witness.assertion,
            entry
                .rendered_witness
                .as_ref()
                .map(|witness| witness.assertion)
                .unwrap_or("no rendered witness required"),
        ],
        submitted_status,
        rendered,
    })
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
    App(AppError),
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
