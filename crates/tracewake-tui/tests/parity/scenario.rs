use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActorId, SemanticActionId};
use tracewake_tui::app::{AppError, TuiApp};

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
    app.bind_actor(actor_id).map_err(ScenarioError::App)?;

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
    if let SetupOperation::SubmitSemanticAction { semantic_action_id } = entry.setup_operation {
        let semantic_action_id = SemanticActionId::new(semantic_action_id.to_string())
            .map_err(|_| ScenarioError::BadSemanticAction(semantic_action_id.to_string()))?;
        let action = view
            .semantic_actions
            .iter()
            .find(|action| action.semantic_action_id == semantic_action_id)
            .ok_or_else(|| ScenarioError::MissingSemanticAction(semantic_action_id.to_string()))?;
        assert!(
            rendered.contains(semantic_action_id.as_str()),
            "{} rendered witness must include {} before submission",
            entry.key,
            semantic_action_id.as_str()
        );

        let result = app
            .submit_semantic_action(&semantic_action_id)
            .map_err(ScenarioError::App)?;
        submitted_status = Some(result.report.status);
        if !action.availability.is_available() {
            rendered = app.render_current_view().map_err(ScenarioError::App)?;
            assert!(
                rendered.contains("Why-not:"),
                "{} disabled action must render actor-safe why-not after rejection",
                entry.key
            );
        }
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
    let expected = match golden_path {
        "crates/tracewake-tui/tests/goldens/base_epistemic_why_not_door_closed.txt" => {
            include_str!("../goldens/base_epistemic_why_not_door_closed.txt")
        }
        "crates/tracewake-tui/tests/goldens/base_semantic_action_wait.txt" => {
            include_str!("../goldens/base_semantic_action_wait.txt")
        }
        other => panic!("unregistered scenario golden path: {other}"),
    };
    let actual = format!("{rendered}\n");
    assert_eq!(
        expected, actual,
        "{} rendered witness must match checked-in golden {}",
        entry.key, golden_path
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
        "actor_mara",
        "food_hidden_pantry",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "{} anti-leak rendered witness exposed {forbidden}",
            entry.key
        );
    }
    assert!(
        rendered.contains("Why-not:") && rendered.contains("door_closed_blocks_movement"),
        "{} anti-leak exemplar must keep an actor-safe why-not reason",
        entry.key
    );
}

pub enum ScenarioError {
    MissingFixture,
    UnknownFixture(String),
    BadActor(String),
    BadSemanticAction(String),
    MissingSemanticAction(String),
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
            Self::BadSemanticAction(action_id) => {
                write!(formatter, "bad semantic action id {action_id}")
            }
            Self::MissingSemanticAction(action_id) => {
                write!(formatter, "missing semantic action {action_id}")
            }
            Self::App(error) => write!(formatter, "app error: {error:?}"),
        }
    }
}
