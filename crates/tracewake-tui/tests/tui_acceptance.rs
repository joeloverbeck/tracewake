use std::collections::BTreeSet;

use tracewake_content::fixtures;
use tracewake_core::actions::{ActionEffect, ActionRegistry, ActionScope, ReportStatus};
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_tui::app::TuiApp;
use tracewake_tui::input::semantic_id_for_selection;
use tracewake_tui::render::render_notebook;
use tracewake_tui::run::run_command_loop;
use tracewake_tui::transcript::{
    capture_representative_transcript, capture_representative_transcript_sections,
};

const ACTIONS_REGISTRY_RS: &str = include_str!("../../tracewake-core/src/actions/registry.rs");

#[derive(Debug)]
struct PositiveProofArtifact {
    responsible_layer: &'static str,
    scenario_id: &'static str,
    actor_id: String,
    context_id: String,
    action_id: Option<&'static str>,
    semantic_id: Option<String>,
    report_status: Option<ReportStatus>,
    event_ids: Vec<String>,
    typed_reason_codes: Vec<String>,
    provenance: Vec<String>,
    debug_capability_present: bool,
    surfaces_checked: Vec<&'static str>,
    checksum_result: &'static str,
}

impl PositiveProofArtifact {
    fn assert_review_fields(&self) {
        assert!(!self.responsible_layer.is_empty());
        assert!(!self.scenario_id.is_empty());
        assert!(!self.actor_id.is_empty());
        assert!(self.context_id.starts_with("hkc."));
        assert!(!self.surfaces_checked.is_empty());
        assert!(!self.checksum_result.is_empty());
    }
}

#[test]
fn embodied_view_lists_each_container_once_after_opening_in_same_tick() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let open = view
        .semantic_actions
        .iter()
        .find(|entry| entry.semantic_action_id.as_str() == "open.container.strongbox_tomas")
        .expect("strongbox is openable from the initial view")
        .semantic_action_id
        .clone();

    let result = app.submit_semantic_action(&open).unwrap();
    assert_eq!(result.report.status, ReportStatus::Accepted);

    let view = app.current_view().unwrap();
    let strongbox_entries: Vec<_> = view
        .visible_containers
        .iter()
        .filter(|container| container.container_id.as_str() == "strongbox_tomas")
        .collect();
    assert_eq!(
        strongbox_entries.len(),
        1,
        "a container observed twice in one tick must collapse to its latest known state, got {:?}",
        view.visible_containers
    );
    assert!(
        strongbox_entries[0].is_open,
        "the surviving container entry must reflect the latest perceived (open) state"
    );

    let check_actions = view
        .semantic_actions
        .iter()
        .filter(|entry| entry.semantic_action_id.as_str() == "check.container.strongbox_tomas")
        .count();
    assert_eq!(
        check_actions, 1,
        "duplicated container facts must not produce duplicated affordances, got {} check actions",
        check_actions
    );
}

#[test]
fn placing_a_carried_item_at_the_current_place_is_accepted_and_moves_the_item() {
    // The place affordance must carry its destination (the current place) so the
    // pipeline can bind a target; otherwise every place is rejected target_not_found.
    let mut app = TuiApp::from_golden(fixtures::place_carried_item_001()).unwrap();
    app.bind_actor(ActorId::new("actor_lina").unwrap()).unwrap();

    let view = app.current_view().unwrap();
    let place = view
        .semantic_actions
        .iter()
        .find(|entry| entry.semantic_action_id.as_str() == "place.item.sample_token_01.at.place")
        .expect("a place affordance is offered for the carried item")
        .semantic_action_id
        .clone();

    let result = app.submit_semantic_action(&place).unwrap();
    assert_eq!(
        result.report.status,
        ReportStatus::Accepted,
        "placing a carried item at the current place must succeed, got {:?}",
        result.report
    );

    let view = app.current_view().unwrap();
    assert!(
        view.carried_items
            .iter()
            .all(|item| item.item_id.as_str() != "sample_token_01"),
        "the placed item must leave the inventory"
    );
    assert!(
        view.visible_items
            .iter()
            .any(|item| item.item_id.as_str() == "sample_token_01"),
        "the placed item must appear among the place items, got {:?}",
        view.visible_items
    );
}

#[test]
fn inspecting_a_visible_item_is_accepted_not_forged() {
    // inspect_entity is a query-only affordance offered in the menu; selecting it
    // must be accepted (the re-rendered view is the query result), not rejected as
    // proposal_source_forged because its semantic token mismatches the action id.
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let open = SemanticActionId::new("open.container.strongbox_tomas").unwrap();
    assert_eq!(
        app.submit_semantic_action(&open).unwrap().report.status,
        ReportStatus::Accepted
    );

    let view = app.current_view().unwrap();
    let inspect = view
        .semantic_actions
        .iter()
        .find(|entry| {
            entry.action_id.as_str() == "inspect_entity"
                && entry
                    .target_ids
                    .iter()
                    .any(|target| target == "coin_stack_01")
        })
        .expect("an inspect affordance is offered for the now-visible coin")
        .semantic_action_id
        .clone();

    let result = app.submit_semantic_action(&inspect).unwrap();
    assert_eq!(
        result.report.status,
        ReportStatus::Accepted,
        "inspecting a visible item must be accepted, got {:?}",
        result.report
    );
}

#[test]
fn contradicted_belief_links_its_contradiction_in_the_notebook() {
    // After tomas checks a strongbox he expected to hold a coin, the absence is a
    // typed contradiction. The newly recorded "item missing" belief must carry that
    // contradiction id so the notebook shows the link instead of contradictions=none.
    let mut app =
        TuiApp::from_golden_operator_debug(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    for semantic in [
        "open.container.strongbox_tomas",
        "check.container.strongbox_tomas",
    ] {
        let id = SemanticActionId::new(semantic).unwrap();
        assert_eq!(
            app.submit_semantic_action(&id).unwrap().report.status,
            ReportStatus::Accepted,
            "{semantic} must be accepted"
        );
    }

    let notebook = app.notebook_view().unwrap();
    assert!(
        !notebook.known_contradictions.is_empty(),
        "checking the empty strongbox must record a typed contradiction"
    );
    let contradiction_id = notebook.known_contradictions[0].contradiction_id.clone();
    let missing_belief = notebook
        .source_bound_beliefs
        .iter()
        .find(|belief| belief.belief_id.starts_with("belief.missing."))
        .expect("an item-missing belief is recorded after the contradiction");
    assert!(
        missing_belief.contradiction_ids.contains(&contradiction_id),
        "the contradicted belief must link its contradiction id, got {:?}",
        missing_belief.contradiction_ids
    );
}

#[test]
fn carried_item_is_not_also_listed_among_place_items_in_same_tick() {
    // Taking an item moves it from the container to the actor's inventory. Within
    // the take tick the place still carries a stale "item in container" perception;
    // the embodied view must not show the same item both in the place and in the
    // inventory (an item has exactly one location).
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    for semantic in [
        "open.container.strongbox_tomas",
        "take.item.coin_stack_01.from.strongbox_tomas",
    ] {
        let id = SemanticActionId::new(semantic).unwrap();
        let result = app.submit_semantic_action(&id).unwrap();
        assert_eq!(
            result.report.status,
            ReportStatus::Accepted,
            "{semantic} must be accepted"
        );
    }

    let view = app.current_view().unwrap();
    assert!(
        view.carried_items
            .iter()
            .any(|item| item.item_id.as_str() == "coin_stack_01"),
        "the coin must be in the inventory after being taken"
    );
    assert!(
        view.visible_items
            .iter()
            .all(|item| item.item_id.as_str() != "coin_stack_01"),
        "a carried item must not also appear in the place item list, got {:?}",
        view.visible_items
    );
}

#[test]
fn embodied_menu_offers_each_food_source_once_when_known_via_belief_and_perception() {
    // actor_tomas both holds a seeded starting belief about food_breakfast_tomas
    // (servings unknown) and directly perceives the same supply (servings known).
    // The embodied menu must offer a single Eat affordance for that one food.
    let mut app =
        TuiApp::from_golden(fixtures::embodied_menu_lags_truth_change_without_perception_001())
            .unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let eat_entries = view
        .semantic_actions
        .iter()
        .filter(|entry| entry.semantic_action_id.as_str() == "eat.food.food_breakfast_tomas")
        .count();

    assert_eq!(
        eat_entries, 1,
        "one food source known via belief and perception must surface a single Eat affordance, got {} in {:?}",
        eat_entries,
        view.semantic_actions
            .iter()
            .map(|entry| entry.semantic_action_id.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn embodied_menu_disables_empty_food_when_seeded_food_source_competes_with_observation() {
    let mut app = TuiApp::from_golden(fixtures::competing_food_source_facts_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let eat_actions = view
        .semantic_actions
        .iter()
        .filter(|entry| entry.semantic_action_id.as_str() == "eat.food.food_empty_bowl_tomas")
        .collect::<Vec<_>>();
    assert_eq!(
        eat_actions.len(),
        1,
        "competing food-source facts must surface exactly one public Eat affordance"
    );
    let eat_action = eat_actions[0];
    assert!(
        !eat_action.enabled,
        "source-bearing observed zero servings must supersede source-less seeded food knowledge"
    );
    assert_eq!(
        eat_action.availability.actor_safe_summary(),
        Some("You know that food source is empty.")
    );
    assert!(
        eat_action.availability.debug_only_diagnostics().is_empty(),
        "public TUI action availability must not expose hidden provenance diagnostics"
    );

    let rendered = app.render_current_view().unwrap();
    assert!(
        rendered.contains(
            "Eat food_empty_bowl_tomas [eat.food.food_empty_bowl_tomas] disabled: You know that food source is empty. reasons=knowledge_precondition_not_met"
        ),
        "rendered TUI menu must carry the public disabled consequence, got:\n{rendered}"
    );
    assert!(
        !rendered.contains("source:"),
        "public TUI render must not leak source keys or hidden provenance, got:\n{rendered}"
    );
}

#[test]
fn tui_selects_semantic_action_id_not_menu_index() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = app.current_view().unwrap();

    let selected = semantic_id_for_selection(&view, 1).unwrap();

    assert_ne!(selected.as_str(), "1");
    assert!(view
        .semantic_actions
        .iter()
        .any(|action| action.semantic_action_id == selected));
}

#[test]
fn tui_transcript_is_deterministic() {
    let first = capture_representative_transcript().unwrap();
    let second = capture_representative_transcript().unwrap();

    assert_eq!(first, second);
    assert!(first.contains("DEBUG NON-DIEGETIC"));
}

#[test]
fn tui_sources_do_not_call_event_application_directly() {
    let sources = [
        ("app.rs", include_str!("../src/app.rs")),
        ("debug_panels.rs", include_str!("../src/debug_panels.rs")),
        ("input.rs", include_str!("../src/input.rs")),
        ("launch.rs", include_str!("../src/launch.rs")),
        ("lib.rs", include_str!("../src/lib.rs")),
        ("main.rs", include_str!("../src/main.rs")),
        ("render.rs", include_str!("../src/render.rs")),
        ("run.rs", include_str!("../src/run.rs")),
        ("transcript.rs", include_str!("../src/transcript.rs")),
    ];
    let mut violations = Vec::new();
    for (path, source) in sources {
        for forbidden in [
            "apply_event(",
            "apply_event_stream(",
            "use tracewake_core::events::apply",
            "tracewake_core::events::apply::",
        ] {
            if source.contains(forbidden) {
                violations.push(format!("{path} contains {forbidden}"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "TUI must mutate only through runtime commands:\n{}",
        violations.join("\n")
    );
    assert!(
        include_str!("../src/app.rs")
            .contains(".submit_command(RuntimeCommand::submit_semantic_action("),
        "TUI submit path must route mutation through the runtime command boundary"
    );
}

#[test]
fn tui_semantic_submissions_use_runtime_allocated_ordering_and_time_policy() {
    let mut app =
        TuiApp::from_golden_operator_debug(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let initial_tick = app.current_view().unwrap().sim_tick();
    let opened = app
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(opened.report.status, ReportStatus::Accepted);
    assert_eq!(
        app.current_view().unwrap().sim_tick(),
        initial_tick,
        "accepted non-wait semantic actions must not advance core time"
    );

    let checked = app
        .submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(checked.report.status, ReportStatus::Accepted);
    assert_eq!(opened.report.proposal_id.as_str(), "proposal_runtime_0");
    assert_eq!(
        checked.report.proposal_id.as_str(),
        "proposal_runtime_2",
        "non-wait commands must use runtime-minted proposal ids while scheduler ordering remains internal"
    );

    let wait_action = semantic_action_for_action_id(&app, "wait");
    let waited = app.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    assert!(
        app.current_view().unwrap().sim_tick() > initial_tick,
        "wait semantic actions must follow core time-advance policy"
    );
}

#[test]
fn debug_truth_does_not_enter_embodied_view() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before = app.render_current_view().unwrap();
    let checksum_before = app.physical_checksum();

    let debug = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
    let after = app.render_current_view().unwrap();

    assert!(debug.contains("container:strongbox_tomas"));
    assert!(!before.contains("coin_stack_01"));
    assert_eq!(before, after);
    assert_eq!(checksum_before, app.physical_checksum());
}

#[test]
fn positive_proof_fixtures_emit_typed_artifacts_first() {
    let mut artifacts = Vec::new();

    let mut embodied = TuiApp::from_golden(fixtures::view_model_local_actions_001()).unwrap();
    embodied
        .bind_actor(ActorId::new("actor_lina").unwrap())
        .unwrap();
    let before_checksum = embodied.physical_checksum();
    let view = embodied.current_view().unwrap();
    let semantic_id = semantic_action_for_action_id(&embodied, "open");
    let accepted = embodied.submit_semantic_action(&semantic_id).unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);
    assert!(!accepted.report.event_ids.is_empty());
    assert_ne!(embodied.physical_checksum(), before_checksum);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "view_model_local_actions_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("open"),
        semantic_id: Some(semantic_id.as_str().to_string()),
        report_status: Some(accepted.report.status),
        event_ids: accepted
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_hash().as_str().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "proposal_report", "event_ids", "checksum"],
        checksum_result: "changed_after_accepted_world_event",
    });
    let view = embodied.current_view().unwrap();
    let close_action = semantic_action_for_action_id(&embodied, "close");
    let closed = embodied.submit_semantic_action(&close_action).unwrap();
    assert_eq!(closed.report.status, ReportStatus::Accepted);
    assert!(!closed.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "view_model_local_actions_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("close"),
        semantic_id: Some(close_action.as_str().to_string()),
        report_status: Some(closed.report.status),
        event_ids: closed
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_hash().as_str().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "close_semantic_action", "event_ids"],
        checksum_result: "accepted_close_event",
    });

    let mut sleep = TuiApp::from_golden(fixtures::sleep_eat_work_001()).unwrap();
    sleep
        .bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let rendered_sleep = sleep.render_current_view().unwrap();
    assert!(
        rendered_sleep.contains("Sleep here"),
        "actor-known sleep affordance must render an embodied sleep action: {rendered_sleep}"
    );
    let view = sleep.current_view().unwrap();
    let sleep_action = semantic_action_for_action_id(&sleep, "sleep");
    let slept = sleep.submit_semantic_action(&sleep_action).unwrap();
    assert_eq!(slept.report.status, ReportStatus::Accepted);
    assert!(!slept.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "sleep_eat_work_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("sleep"),
        semantic_id: Some(sleep_action.as_str().to_string()),
        report_status: Some(slept.report.status),
        event_ids: slept
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "sleep_semantic_action", "event_ids"],
        checksum_result: "accepted_sleep_started_event",
    });

    let mut eat = TuiApp::from_golden_operator_debug(fixtures::no_human_day_001()).unwrap();
    eat.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = eat.current_view().unwrap();
    let eat_action = semantic_action_for_action_id(&eat, "eat");
    let eaten = eat.submit_semantic_action(&eat_action).unwrap();
    assert_eq!(eaten.report.status, ReportStatus::Accepted);
    assert!(!eaten.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "no_human_day_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("eat"),
        semantic_id: Some(eat_action.as_str().to_string()),
        report_status: Some(eaten.report.status),
        event_ids: eaten
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "eat_semantic_action", "event_ids"],
        checksum_result: "accepted_food_consumed_event",
    });

    let mut work = TuiApp::from_golden(
        fixtures::embodied_workplace_believed_open_truth_closed_commit_fails_001(),
    )
    .unwrap();
    work.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = work.current_view().unwrap();
    let work_action = semantic_action_for_action_id(&work, "work_block");
    let worked = work.submit_semantic_action(&work_action).unwrap();
    assert_eq!(worked.report.status, ReportStatus::Accepted);
    assert!(!worked.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "embodied_workplace_believed_open_truth_closed_commit_fails_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("work_block"),
        semantic_id: Some(work_action.as_str().to_string()),
        report_status: Some(worked.report.status),
        event_ids: worked
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "work_semantic_action", "event_ids"],
        checksum_result: "accepted_work_block_failure_event",
    });

    let mut wait = TuiApp::load_default().unwrap();
    wait.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = wait.current_view().unwrap();
    let wait_action = semantic_action_for_action_id(&wait, "wait");
    let waited = wait.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    assert!(!waited.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "strongbox_tomas_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("wait"),
        semantic_id: Some(wait_action.as_str().to_string()),
        report_status: Some(waited.report.status),
        event_ids: waited
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["embodied_view", "wait_semantic_action", "event_ids"],
        checksum_result: "accepted_wait_event",
    });

    let mut continuation =
        TuiApp::from_golden(fixtures::possession_does_not_reset_intention_001()).unwrap();
    continuation
        .bind_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let view = continuation.current_view().unwrap();
    let continue_action = semantic_action_for_action_id(&continuation, "continue_routine");
    let continued = continuation
        .submit_semantic_action(&continue_action)
        .unwrap();
    assert_eq!(continued.report.status, ReportStatus::Accepted);
    assert!(!continued.report.event_ids.is_empty());
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "possession_does_not_reset_intention_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("continue_routine"),
        semantic_id: Some(continue_action.as_str().to_string()),
        report_status: Some(continued.report.status),
        event_ids: continued
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec![
            "embodied_view",
            "continue_routine_semantic_action",
            "event_ids",
        ],
        checksum_result: "accepted_continue_routine_event",
    });

    let mut why_not = TuiApp::from_golden_operator_debug(fixtures::door_access_001()).unwrap();
    why_not
        .bind_actor(ActorId::new("actor_sena").unwrap())
        .unwrap();
    let before_checksum = why_not.physical_checksum();
    let view = why_not.current_view().unwrap();
    let rejected = why_not
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert_eq!(why_not.physical_checksum(), before_checksum);
    assert!(rejected.report.actor_visible_summary.contains("door"));
    assert!(!rejected.report.actor_visible_summary.contains("debug"));
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "door_access_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("move"),
        semantic_id: Some("move.to.back_room".to_string()),
        report_status: Some(rejected.report.status),
        event_ids: rejected
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: rejected
            .report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id().to_string())
            .collect(),
        provenance: rejected
            .report
            .actor_visible_facts
            .iter()
            .map(tracewake_core::actions::CheckedFact::render_pair)
            .collect(),
        debug_capability_present: false,
        surfaces_checked: vec!["why_not", "typed_reason_codes", "actor_visible_facts"],
        checksum_result: "unchanged_after_rejection",
    });

    let mut notebook =
        TuiApp::from_golden_operator_debug(fixtures::expectation_contradiction_001()).unwrap();
    notebook
        .bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let opened = notebook
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(opened.report.status, ReportStatus::Accepted);
    let checked = notebook
        .submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(checked.report.status, ReportStatus::Accepted);
    let view = notebook.current_view().unwrap();
    let notebook_view = notebook.notebook_view().unwrap();
    assert_eq!(notebook_view.typed_leads.len(), 1);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "expectation_contradiction_001",
        actor_id: notebook_view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("check_container"),
        semantic_id: Some("check.container.strongbox_tomas".to_string()),
        report_status: Some(checked.report.status),
        event_ids: checked
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: notebook_view
            .typed_leads
            .iter()
            .map(|lead| format!("{}:{}", lead.source_kind, lead.source_summary))
            .collect(),
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["notebook", "typed_leads", "source_refs"],
        checksum_result: "observation_event_recorded",
    });

    let mut debug =
        TuiApp::from_golden_operator_debug(fixtures::debug_omniscience_excluded_001()).unwrap();
    debug
        .bind_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let before_view = debug.render_current_view().unwrap();
    let before_checksum = debug.physical_checksum();
    let before_events = debug.event_count();
    let debug_panel = debug.render_debug_item_location_panel(
        &tracewake_core::ids::ItemId::new("food_hidden_pantry").unwrap(),
    );
    assert!(debug_panel.contains("DEBUG NON-DIEGETIC"));
    assert_eq!(debug.render_current_view().unwrap(), before_view);
    assert_eq!(debug.physical_checksum(), before_checksum);
    assert_eq!(debug.event_count(), before_events);
    let view = debug.current_view().unwrap();
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "debug_omniscience_excluded_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: None,
        semantic_id: None,
        report_status: None,
        event_ids: Vec::new(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_hash().as_str().to_string()],
        debug_capability_present: true,
        surfaces_checked: vec!["debug_panel", "checksum", "event_count", "embodied_view"],
        checksum_result: "unchanged_after_debug_panel",
    });

    let mut possession = TuiApp::from_golden(fixtures::possession_parity_001()).unwrap();
    possession
        .bind_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let take_action = semantic_action_for_action_id(&possession, "take");
    let before_checksum = possession.physical_checksum();
    let taken = possession.submit_semantic_action(&take_action).unwrap();
    assert_eq!(taken.report.status, ReportStatus::Accepted);
    assert!(!taken.report.event_ids.is_empty());
    assert_ne!(possession.physical_checksum(), before_checksum);
    let view = possession.current_view().unwrap();
    assert!(view
        .semantic_actions
        .iter()
        .any(|action| action.action_id.as_str() == "place"));
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "possession_parity_001",
        actor_id: view.viewer_actor_id().as_str().to_string(),
        context_id: view.holder_known_context_id().as_str().to_string(),
        action_id: Some("take"),
        semantic_id: view
            .semantic_actions
            .first()
            .map(|action| action.semantic_action_id.as_str().to_string()),
        report_status: Some(taken.report.status),
        event_ids: taken
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary().to_string()],
        debug_capability_present: view.debug_available(),
        surfaces_checked: vec!["possession_view", "semantic_actions"],
        checksum_result: "changed_after_ordinary_take",
    });
    let first_sections = capture_representative_transcript_sections().unwrap();
    let second_sections = capture_representative_transcript_sections().unwrap();
    assert_eq!(first_sections, second_sections);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "replay_item_location_001",
        actor_id: "actor_sena".to_string(),
        context_id: "hkc.actor_sena.0.1".to_string(),
        action_id: None,
        semantic_id: None,
        report_status: None,
        event_ids: Vec::new(),
        typed_reason_codes: Vec::new(),
        provenance: first_sections
            .iter()
            .map(|section| section.name.clone())
            .collect(),
        debug_capability_present: first_sections
            .iter()
            .any(|section| section.body.contains("DEBUG NON-DIEGETIC")),
        surfaces_checked: vec!["transcript_sections", "replay_panel"],
        checksum_result: "byte_identical_sections",
    });

    assert_eq!(artifacts.len(), 12);
    for artifact in &artifacts {
        artifact.assert_review_fields();
    }
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.event_ids.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.typed_reason_codes.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.debug_capability_present));
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.provenance.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.report_status.is_some()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.semantic_id.is_some()));
    let submitted_positive_action_ids = artifacts
        .iter()
        .filter(|artifact| {
            artifact.report_status == Some(ReportStatus::Accepted) && !artifact.event_ids.is_empty()
        })
        .filter_map(|artifact| artifact.action_id.map(str::to_string))
        .collect::<BTreeSet<_>>();
    let census_errors = ordinary_action_positive_census_errors(&submitted_positive_action_ids);
    assert!(
        census_errors.is_empty(),
        "ordinary action submitted-positive census failed: {census_errors:#?}"
    );
    let mut sleep_removed = submitted_positive_action_ids.clone();
    sleep_removed.remove("sleep");
    assert!(
        ordinary_action_positive_census_errors(&sleep_removed)
            .iter()
            .any(|error| error.contains("sleep")),
        "synthetic sleep-positive removal must fail the ordinary action census"
    );
    let synthetic_unhandled = synthetic_registry_rows_with_unhandled_effect();
    assert!(
        ordinary_action_positive_census_errors_for_rows(
            &submitted_positive_action_ids,
            &synthetic_unhandled,
        )
        .iter()
        .any(|error| error.contains("synthetic_new_action")),
        "synthetic unhandled ordinary action effect must fail the census"
    );
}

#[test]
fn possession_rebind_preserves_next_routine_step() {
    let mut app = TuiApp::from_golden(fixtures::possession_does_not_reset_intention_001()).unwrap();
    let actor_mara = ActorId::new("actor_mara").unwrap();
    app.bind_actor(actor_mara.clone()).unwrap();
    let before_view = app.current_view().unwrap();
    let before_continue = before_view
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == "continue_routine")
        .expect("actor_mara starts with a continue-routine step")
        .clone();
    let before_status = before_view.phase3a_status.clone();
    let before_checksum = app.physical_checksum();

    app.bind_actor(actor_mara).unwrap();

    let after_view = app.current_view().unwrap();
    let after_continue = after_view
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == "continue_routine")
        .expect("rebound actor_mara still surfaces the continue-routine step")
        .clone();
    assert_eq!(after_continue, before_continue);
    assert_eq!(after_view.phase3a_status, before_status);
    assert_eq!(app.physical_checksum(), before_checksum);
}

#[test]
fn phase3a_debug_surfaces_render_deterministically_and_read_only() {
    let mut app = TuiApp::from_golden_operator_debug(fixtures::no_human_day_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_view = app.render_current_view().unwrap();
    let before_checksum = app.physical_checksum();
    let before_event_count = app.event_count();

    let needs_first = app.render_debug_needs_panel();
    let needs_second = app.render_debug_needs_panel();
    let routines = app.render_debug_routines_panel();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let stuck = app.render_debug_stuck_panel();
    let no_human_day = app.render_debug_no_human_day_panel();
    let actor = app.render_debug_actor_panel(&ActorId::new("actor_tomas").unwrap());

    assert_eq!(needs_first, needs_second);
    for panel in [
        &needs_first,
        &routines,
        &planner,
        &stuck,
        &no_human_day,
        &actor,
    ] {
        assert!(panel.contains("DEBUG NON-DIEGETIC"));
    }
    assert!(needs_first.contains("actor_tomas"));
    assert!(needs_first.contains("need=hunger"));
    assert!(needs_first.contains("value=520 band=urgent cause=fixture_initial"));
    assert!(routines.contains("Routines"));
    assert!(routines.contains("active actor=actor_tomas intention=intention_tomas_go_work"));
    assert!(routines.contains("routine_exec_tomas_go_work"));
    assert!(planner.contains("actor=actor_mara"));
    assert!(planner.contains("candidate_goals"));
    assert!(planner.contains("selected_method"));
    assert!(planner.contains("rejected_reasons"));
    assert!(planner.contains("blocked_preconditions"));
    assert!(planner.contains("hidden_truth_audit"));
    assert!(stuck.contains("stuck_diagnostic_count="));
    assert!(no_human_day.contains("No Human Day"));
    assert!(no_human_day.contains("no_human_day_metrics_v1"));
    assert!(no_human_day.contains("routine_interruptions=0"));
    assert!(actor.contains("actor=actor_tomas"));
    assert!(actor.contains("active_intention=intention_tomas_go_work"));
    assert_eq!(app.render_current_view().unwrap(), before_view);
    assert_eq!(app.physical_checksum(), before_checksum);
    assert_eq!(app.event_count(), before_event_count);

    let debug_panels_source = include_str!("../src/debug_panels.rs");
    for forbidden in ["apply_event", "insert_belief", "insert_observation"] {
        assert!(
            !debug_panels_source.contains(forbidden),
            "debug panel rendering must not call {forbidden}"
        );
    }
}

#[test]
fn tui_runs_no_human_day_and_inspects_real_post_run_panels() {
    let mut app = TuiApp::from_golden_operator_debug(fixtures::no_human_day_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_events = app.event_count();

    let report = app.run_no_human_day().unwrap();
    let after_run_events = app.event_count();
    let embodied_view = app.current_view().unwrap();
    assert_eq!(
        embodied_view.holder_known_context_frontier(),
        after_run_events as u64
    );
    let embodied = tracewake_tui::render::render_embodied_view(&embodied_view);
    let before_debug_checksum = app.physical_checksum();
    let metrics = app.render_debug_no_human_day_panel();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let stuck = app.render_debug_stuck_panel();
    let after_debug_events = app.event_count();

    assert!(report.ordinary_pipeline_events > 0);
    assert!(after_run_events > before_events);
    assert!(embodied.contains("Needs:"));
    assert!(embodied.contains("- hunger: band=rising cause=tick_delta"));
    assert!(!embodied.contains("value=410"));
    assert!(embodied.contains("Intention:"));
    assert!(embodied.contains("active:routine_tomas_go_work:wait"));
    assert!(!embodied.contains("Knowledge context"));
    assert!(!embodied.contains("DEBUG NON-DIEGETIC"));
    assert!(!embodied.contains("routine_events="));
    assert!(!embodied.contains("work_failed="));
    assert!(!embodied.contains("food_hidden_pantry"));
    assert!(metrics.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(metrics.contains("no_human_day_metrics_v1"));
    assert!(metrics.contains("routine_events=8"));
    assert!(metrics.contains("work_failed=1"));
    assert!(metrics.contains("need_crossings=2"));
    assert!(metrics.contains("routine_interruptions=2"));
    assert!(metrics.contains("replay_failures=0"));
    let events_line = metrics
        .lines()
        .find(|line| line.starts_with("events="))
        .unwrap();
    assert_ne!(events_line, "events=0");
    assert!(planner.contains("DEBUG NON-DIEGETIC: Planner"));
    assert!(planner.contains("candidate_goals"));
    assert!(stuck.contains("DEBUG NON-DIEGETIC: Stuck"));
    assert!(stuck.contains("stuck_diagnostic_count="));
    assert!(stuck.contains("stuck="));
    assert!(stuck.contains("debug_detail=no-human day stuck detection"));
    let routines = app.render_debug_routines_panel();
    assert!(routines.contains("routine_exec_mara_eat"));
    assert!(routines.contains("status=Failed"));
    assert!(routines.contains("routine_exec_tomas_work"));
    assert!(routines.contains("status=Completed"));
    assert_eq!(app.event_count(), after_debug_events);
    assert_eq!(app.physical_checksum(), before_debug_checksum);

    let mut command_app = TuiApp::from_golden_operator_debug(fixtures::no_human_day_001()).unwrap();
    command_app
        .bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let script = b"debug run no-human-day\ndebug no-human-day\nview\nquit\n";
    let mut output = Vec::new();

    run_command_loop(&mut command_app, &script[..], &mut output).unwrap();

    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(!rendered.contains("Ran no-human day:"));
    assert!(!rendered.contains("ordinary_events="));
    assert!(rendered.contains("work_failed=1"));
    assert!(rendered.contains("routine_interruptions=2"));
    assert!(rendered.contains("- hunger: band=rising cause=tick_delta"));
    assert!(!rendered.contains("value=410"));
    assert!(rendered.contains("Actor: actor_tomas"));
    assert!(!rendered.contains("food_hidden_pantry"));
}

#[test]
fn phase3a_possess_continue_and_debug_transcript_is_deterministic() {
    let first = phase3a_possess_continue_debug_transcript();
    let second = phase3a_possess_continue_debug_transcript();

    assert_eq!(first, second);
    assert!(first.contains("Actor: actor_mara"));
    assert!(first.contains("Needs:"));
    assert!(first.contains("Intention:"));
    assert!(first.contains("Accepted: wait.1_tick"));
    assert!(first.contains("continue_routine"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Needs"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Planner"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Stuck"));
    assert!(first.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Actor"));
    assert!(!first.contains("food_hidden_pantry"));
}

#[test]
fn leakage_debug_truth_does_not_enter_embodied_view() {
    debug_truth_does_not_enter_embodied_view();
}

#[test]
fn tui_playability_reaches_action_rejection_wait_and_debug() {
    let mut app = TuiApp::from_golden_operator_debug(fixtures::door_access_001()).unwrap();
    app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();
    assert!(app
        .render_current_view()
        .unwrap()
        .contains("move.to.back_room"));

    let rejected = app
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert!(app.render_current_view().unwrap().contains("Why-not:"));
    assert!(app
        .render_debug_action_rejection_panel()
        .unwrap_or_default()
        .contains("Action Rejection"));

    let wait_action = semantic_action_for_action_id(&app, "wait");
    let waited = app.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    assert!(app
        .render_debug_event_log_panel()
        .contains("DEBUG NON-DIEGETIC"));
    assert!(app
        .render_debug_projection_rebuild_panel()
        .contains("Projection Rebuild"));
    assert!(app.render_debug_replay_panel().contains("Replay"));
}

#[test]
fn phase2a_tui_transcript_discovers_absence_without_culprit_leak() {
    let mut app =
        TuiApp::from_golden_operator_debug(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let before_view = app.render_current_view().unwrap();
    let before_notebook = render_notebook(&app.notebook_view().unwrap());

    assert!(before_view.contains("check.container.strongbox_tomas"));
    assert_no_embodied_culprit_leak(&before_view);
    assert_no_embodied_culprit_leak(&before_notebook);

    let opened = app
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(opened.report.status, ReportStatus::Accepted);

    let checked = app
        .submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(checked.report.status, ReportStatus::Accepted);

    let after_view = app.render_current_view().unwrap();
    let after_notebook = render_notebook(&app.notebook_view().unwrap());
    let debug_epistemics =
        tracewake_tui::debug_panels::render_debug_epistemics_panel(&app.debug_epistemics_view());
    let debug_beliefs = tracewake_tui::debug_panels::render_debug_beliefs_panel(
        &app.debug_beliefs_view(&ActorId::new("actor_tomas").unwrap())
            .unwrap(),
    );
    let debug_observations = tracewake_tui::debug_panels::render_debug_observations_panel(
        &app.debug_observations_view(&ActorId::new("actor_tomas").unwrap())
            .unwrap(),
    );
    let debug_replay = app.render_debug_replay_panel();

    assert!(!after_view.contains("Knowledge context:"));
    assert!(after_notebook.contains("missing"));
    assert!(after_notebook.contains("source=event:"));
    assert!(after_notebook.contains("Contradictions:"));
    assert_no_embodied_culprit_leak(&after_view);
    assert_no_embodied_culprit_leak(&after_notebook);
    assert!(debug_epistemics.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(debug_epistemics.contains("contradiction_count=1"));
    assert!(debug_beliefs.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(debug_observations.contains("DEBUG NON-DIEGETIC: Observations"));
    assert!(debug_replay.contains("matches_expected=true"));
    assert!(debug_replay.contains("agent_checksum_matches=true"));

    let debug_truth = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
    assert!(debug_truth.contains("actor_mara"));
    assert!(!after_view.contains("actor_mara"));
    assert!(!after_notebook.contains("actor_mara"));
}

fn assert_no_embodied_culprit_leak(rendered: &str) {
    for forbidden in ["actor_mara", "Mara", "culprit", "stole", "theft"] {
        assert!(
            !rendered.contains(forbidden),
            "embodied surface leaked {forbidden}: {rendered}"
        );
    }
}

fn semantic_action_for_action_id(app: &TuiApp, action_id: &str) -> SemanticActionId {
    app.current_view()
        .unwrap()
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == action_id)
        .map(|action| action.semantic_action_id.clone())
        .expect("current view surfaces requested action")
}

fn ordinary_action_positive_census_errors(submitted_action_ids: &BTreeSet<String>) -> Vec<String> {
    let mut registry = ActionRegistry::new_with_active_scopes([
        ActionScope::Phase1,
        ActionScope::Phase3AHistorical,
    ]);
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    let rows = registry
        .definitions()
        .map(|definition| {
            (
                definition.action_id.as_str().to_string(),
                action_effect_variant(&definition.effect),
            )
        })
        .collect::<Vec<_>>();
    ordinary_action_positive_census_errors_for_rows(submitted_action_ids, &rows)
}

fn ordinary_action_positive_census_errors_for_rows(
    submitted_action_ids: &BTreeSet<String>,
    rows: &[(String, String)],
) -> Vec<String> {
    let mut errors = Vec::new();
    let expected_effects = action_effect_variants_from_source(ACTIONS_REGISTRY_RS);
    for variant in expected_effects {
        if action_effect_positive_decision(&variant).is_none() {
            errors.push(format!("ActionEffect::{variant} has no census decision"));
        }
    }
    for (action_id, variant) in rows {
        match action_effect_positive_decision(variant) {
            None => errors.push(format!(
                "ordinary action {action_id} uses unhandled ActionEffect::{variant}"
            )),
            Some(ActionEffectPositiveDecision::RequiresPositive) => {
                if !submitted_action_ids.contains(action_id) {
                    errors.push(format!(
                        "ordinary action {action_id} ({variant}) lacks a submitted positive"
                    ));
                }
            }
            Some(ActionEffectPositiveDecision::Exempt { rationale }) => {
                if rationale.trim().is_empty() {
                    errors.push(format!("ActionEffect::{variant} exemption lacks rationale"));
                }
            }
        }
    }
    errors
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionEffectPositiveDecision {
    RequiresPositive,
    Exempt { rationale: &'static str },
}

fn action_effect_positive_decision(variant: &str) -> Option<ActionEffectPositiveDecision> {
    match variant {
        "QueryOnly" => Some(ActionEffectPositiveDecision::Exempt {
            rationale: "query-only inspection does not emit accepted world events by design",
        }),
        "Place" => Some(ActionEffectPositiveDecision::Exempt {
            rationale: "current TUI fixtures surface place only as a disabled negative until an authored enabled target exists",
        }),
        "Move" | "Open" | "Close" | "Take" | "Wait" | "CheckContainer" | "Sleep" | "Eat"
        | "Work" | "ContinueRoutine" => {
            Some(ActionEffectPositiveDecision::RequiresPositive)
        }
        _ => None,
    }
}

fn action_effect_variant(effect: &ActionEffect) -> String {
    format!("{effect:?}")
}

fn action_effect_variants_from_source(source: &str) -> BTreeSet<String> {
    let enum_body = source
        .split("pub enum ActionEffect")
        .nth(1)
        .and_then(|tail| tail.split_once('{'))
        .and_then(|(_, tail)| tail.split_once('}'))
        .map(|(body, _)| body)
        .unwrap_or("");
    enum_body
        .lines()
        .filter_map(|line| {
            let name = line
                .trim()
                .trim_end_matches(',')
                .split_whitespace()
                .next()
                .unwrap_or("");
            (!name.is_empty()).then(|| name.to_string())
        })
        .collect()
}

fn synthetic_registry_rows_with_unhandled_effect() -> Vec<(String, String)> {
    let mut rows = ActionRegistry::new_with_active_scopes([
        ActionScope::Phase1,
        ActionScope::Phase3AHistorical,
    ]);
    rows.register_phase1_inspect_wait();
    rows.register_phase3a_sleep();
    rows.register_phase3a_eat();
    rows.register_phase3a_work();
    rows.register_phase3a_continue_routine();
    let mut rows = rows
        .definitions()
        .map(|definition| {
            (
                definition.action_id.as_str().to_string(),
                action_effect_variant(&definition.effect),
            )
        })
        .collect::<Vec<_>>();
    rows.push((
        "synthetic_new_action".to_string(),
        "SyntheticNewEffect".to_string(),
    ));
    rows
}

fn phase3a_possess_continue_debug_transcript() -> String {
    let mut app = TuiApp::from_golden(fixtures::possession_does_not_reset_intention_001()).unwrap();
    app.bind_actor(ActorId::new("actor_mara").unwrap()).unwrap();
    let view = app.current_view().unwrap();
    let continue_action = view
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == "continue_routine")
        .map(|action| action.semantic_action_id.clone());

    let mut transcript = vec![app.render_current_view().unwrap()];
    let wait_action = semantic_action_for_action_id(&app, "wait");
    let waited = app.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    transcript.push(format!("Accepted: {}", wait_action.as_str()));
    if let Some(continue_action) = continue_action {
        let continued = app.submit_semantic_action(&continue_action).unwrap();
        assert_eq!(continued.report.status, ReportStatus::Accepted);
        transcript.push(format!("Accepted: {}", continue_action.as_str()));
    } else {
        transcript.push("continue_routine unavailable: no active intention".to_string());
    }
    transcript.push(app.render_current_view().unwrap());
    transcript.push(app.render_debug_needs_panel());
    transcript.push(app.render_debug_routines_panel());
    transcript.push(app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap()));
    transcript.push(app.render_debug_stuck_panel());
    transcript.push(app.render_debug_no_human_day_panel());
    transcript.push(app.render_debug_actor_panel(&ActorId::new("actor_mara").unwrap()));
    transcript.join("\n---\n")
}
