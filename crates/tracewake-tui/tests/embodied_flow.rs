use tracewake_content::fixtures;
use tracewake_core::actions::ReasonCode;
use tracewake_core::actions::ReportStatus;
use tracewake_core::events::EventKind;
use tracewake_core::ids::{ActorId, SemanticActionId};
use tracewake_core::view_models::ActionAvailabilityProvenanceKind;
use tracewake_tui::app::TuiApp;
use tracewake_tui::run::run_command_loop;

#[test]
fn bind_render_submit_rerender_and_show_why_not() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let first = app.render_current_view().unwrap();
    assert!(first.contains("Actor: actor_tomas"));
    assert!(first.contains("open.container.strongbox_tomas"));
    let first_view = app.current_view().unwrap();
    assert_eq!(
        first_view.holder_known_context_id.as_str(),
        "hkc.actor_tomas.0.7"
    );
    assert!(first_view
        .holder_known_context_hash
        .as_str()
        .starts_with("hkc1-"));
    assert!(!first.contains("Knowledge context"));
    assert!(!first.contains("DEBUG NON-DIEGETIC"));

    let accepted = app
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);

    let second = app.render_current_view().unwrap();
    assert!(second.contains("strongbox_tomas"));
    assert!(second.contains("coin_stack_01"));

    let mut door_app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    door_app
        .bind_actor(ActorId::new("actor_sena").unwrap())
        .unwrap();
    let pre_submit = door_app.render_current_view().unwrap();
    assert!(pre_submit.contains("move.to.back_room"));
    assert!(pre_submit.contains("disabled: The door is closed."));

    let rejected = door_app
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);

    let why_not = door_app.render_current_view().unwrap();
    assert!(why_not.contains("Why-not:"));
    assert!(why_not.contains(&rejected.report.actor_visible_summary));
    assert!(pre_submit.contains(&rejected.report.actor_visible_summary));
}

#[test]
fn phase3a_embodied_view_renders_needs_routine_affordances_without_hidden_truth() {
    let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let rendered = app.render_current_view().unwrap();

    assert!(rendered.contains("Needs:"));
    assert!(rendered.contains("- hunger:"));
    assert!(rendered.contains("band="));
    assert!(!rendered.contains("value="));
    assert!(rendered.contains("Intention:"));
    assert!(rendered.contains("Routine:"));
    assert!(rendered.contains("Eat food_stew_home_tomas"));
    assert!(!rendered.contains("Sleep here"));
    assert!(!rendered.contains("Work at workplace_tomas"));
    assert!(!rendered.contains("food_empty_pantry_mara"));
    assert!(!rendered.contains("actor_mara"));
}

#[test]
fn wait_command_advances_authoritative_world_one_tick() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_events = app.event_count();
    let mut output = Vec::new();

    run_command_loop(&mut app, b"wait\nquit\n".as_slice(), &mut output).unwrap();

    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("Accepted: wait.1_tick"));
    assert!(rendered.contains("Actor: actor_tomas"));
    assert!(!rendered.contains("Actor: actor_tomas | Tick: 1"));
    assert!(app.event_count() > before_events);
    assert_eq!(
        app.current_view().unwrap().holder_known_context_frontier(),
        app.event_count() as u64
    );
    let event_log = app.render_debug_event_log_panel();
    assert!(event_log.contains("actor_waited"));
    assert!(event_log.contains("time_advanced"));
    assert!(event_log.contains("declared_world_process_applied"));
    assert!(event_log.contains("decision_trace_recorded"));
    assert!(
        event_log.matches("actor_waited").count() >= 2,
        "production bootstrap must advance at least one loaded actor other than the possessed actor:\n{event_log}"
    );
    assert!(
        event_log.matches("declared_world_process_applied").count() >= 1,
        "production bootstrap must register declared world process due work:\n{event_log}"
    );
}

#[test]
fn wait_command_during_sleep_is_reservation_conflict_without_world_advance() {
    let mut app = TuiApp::from_golden(fixtures::sleep_eat_work_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let sleep_started = app
        .submit_semantic_action(&SemanticActionId::new("sleep.here").unwrap())
        .unwrap();
    assert_eq!(sleep_started.report.status, ReportStatus::Accepted);
    let after_sleep_view = app.render_current_view().unwrap();

    let rejected_wait = app
        .submit_semantic_action(&SemanticActionId::new("wait.1_tick").unwrap())
        .unwrap();

    assert_eq!(rejected_wait.report.status, ReportStatus::Rejected);
    assert!(rejected_wait
        .report
        .reason_codes
        .contains(&ReasonCode::ReservationConflict));
    assert!(app.render_current_view().unwrap().contains(
        after_sleep_view
            .lines()
            .next()
            .expect("rendered view starts with actor and tick")
    ));
    let event_log = app.render_debug_event_log_panel();
    assert!(event_log.contains("sleep_started"));
    assert!(!event_log.contains("actor_waited"));
    assert!(!event_log.contains("time_advanced"));
}

#[test]
fn human_sleep_completion_real_pipeline_witness() {
    let mut app = TuiApp::from_golden(fixtures::sleep_eat_work_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let mut output = Vec::new();

    run_command_loop(
        &mut app,
        b"do sleep.here\nwait\ncontinue\nbind-debug actor_tomas\ndebug replay\nquit\n".as_slice(),
        &mut output,
    )
    .unwrap();

    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("Accepted: sleep.here"));
    assert!(rendered.contains("Why-not:"));
    assert!(rendered.contains("reasons=reservation_conflict"));
    assert!(rendered.contains("Advanced until: actor-known interval updated"));
    assert!(!rendered.contains("possessed_duration_terminal"));
    assert!(!rendered.contains("stop_tick="));
    assert!(rendered.contains("Recent interval: actor-known update"));
    assert!(rendered.contains("- perception"));
    assert!(!rendered.contains("event.perception.actor_tomas.4."));
    assert!(rendered.contains("Bound debug actor: actor_tomas"));
    assert!(rendered.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(rendered.contains("matches_expected=false"));
    assert!(rendered.contains("agent_checksum_matches=true"));
    assert!(!rendered.contains("RunNoHumanDay"));
    assert!(!rendered.contains("No Human Day"));

    let needs = app.render_debug_needs_panel();
    assert!(needs.contains("actor=actor_tomas need=fatigue"));
    assert!(needs.contains("actor=actor_tomas need=hunger"));
    assert!(needs.contains("cause=action_effect:sleep"));
    let replay = app.render_debug_replay_panel();
    assert!(replay.contains("matches_expected=false"));
    assert!(replay.contains("agent_checksum_matches=true"));
}

#[test]
fn human_work_completion_real_pipeline_witness() {
    let mut app = TuiApp::from_golden(fixtures::ordinary_workday_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let mut output = Vec::new();

    run_command_loop(
        &mut app,
        b"do move.to.workshop_tomas\ndo work.block.workplace_tomas\nwait\ncontinue\nbind-debug actor_tomas\ndebug replay\nquit\n".as_slice(),
        &mut output,
    )
    .unwrap();

    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("Accepted: move.to.workshop_tomas"));
    assert!(rendered.contains("Accepted: work.block.workplace_tomas"));
    assert!(rendered.contains("Why-not:"));
    assert!(rendered.contains("reasons=reservation_conflict"));
    assert!(rendered.contains("Advanced until: actor-known interval updated"));
    assert!(!rendered.contains("possessed_duration_terminal"));
    assert!(!rendered.contains("stop_tick="));
    assert!(rendered.contains("Recent interval: actor-known update"));
    assert!(rendered.contains("- perception"));
    assert!(!rendered.contains("event.perception.actor_tomas.4."));
    assert!(rendered.contains("Bound debug actor: actor_tomas"));
    assert!(rendered.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(rendered.contains("matches_expected=false"));
    assert!(rendered.contains("agent_checksum_matches=true"));
    assert!(!rendered.contains("RunNoHumanDay"));
    assert!(!rendered.contains("No Human Day"));

    let needs = app.render_debug_needs_panel();
    assert!(needs.contains("actor=actor_tomas need=fatigue"));
    assert!(needs.contains("actor=actor_tomas need=hunger"));
    let event_log = app.render_debug_event_log_panel();
    assert!(event_log.contains("work_block_completed"));
    assert!(event_log.contains("need_delta_applied"));
    let replay = app.render_debug_replay_panel();
    assert!(replay.contains("matches_expected=false"));
    assert!(replay.contains("agent_checksum_matches=true"));
}

#[test]
fn embodied_view_omits_raw_workplace_assignment_without_context() {
    let mut app =
        TuiApp::from_golden(fixtures::embodied_view_omits_raw_assignment_without_context_001())
            .unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let rendered = app.render_current_view().unwrap();

    assert!(rendered.contains("move.to.workshop_tomas"));
    assert!(!rendered.contains("Work at workplace_tomas"));
    assert!(!view.semantic_actions.iter().any(|entry| {
        entry.action_id.as_str() == "work_block"
            || entry
                .target_ids
                .iter()
                .any(|target| target == "workplace_tomas")
    }));
}

#[test]
fn embodied_workplace_availability_reflects_belief_not_truth() {
    let mut app = TuiApp::from_golden(
        fixtures::embodied_workplace_availability_reflects_belief_not_truth_001(),
    )
    .unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let rendered = app.render_current_view().unwrap();

    let entry = view
        .semantic_actions
        .iter()
        .find(|entry| {
            entry.action_id.as_str() == "work_block"
                && entry
                    .target_ids
                    .iter()
                    .any(|target| target == "workplace_tomas")
        })
        .expect("workplace action is present from actor-known workplace fact");

    assert!(rendered.contains("Work at workplace_tomas"));
    assert!(rendered.contains("disabled: You know that workplace access is closed."));
    assert!(!entry.availability.is_available());
    assert_eq!(
        entry.availability.reason_codes(),
        &[ReasonCode::KnowledgePreconditionNotMet]
    );
    assert!(entry
        .availability
        .provenance_refs()
        .iter()
        .any(|reference| {
            reference.kind == ActionAvailabilityProvenanceKind::SourceEvent
                && reference.reference
                    == "event_seed_role_assignment_notice_actor_tomas_workplace_tomas"
        }));
}

#[test]
fn embodied_workplace_believed_open_truth_closed() {
    let mut app = TuiApp::from_golden(
        fixtures::embodied_workplace_believed_open_truth_closed_commit_fails_001(),
    )
    .unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let view = app.current_view().unwrap();
    let rendered = app.render_current_view().unwrap();
    let entry = view
        .semantic_actions
        .iter()
        .find(|entry| {
            entry.semantic_action_id.as_str() == "work.block.workplace_tomas"
                && entry.action_id.as_str() == "work_block"
        })
        .expect("workplace action is present from actor-known open-access fact");

    assert!(rendered.contains("Work at workplace_tomas"));
    assert!(!rendered.contains("disabled: You know that workplace access is closed."));
    assert!(entry.availability.is_available());

    let result = app
        .submit_semantic_action(&SemanticActionId::new("work.block.workplace_tomas").unwrap())
        .unwrap();
    assert_eq!(result.report.status, ReportStatus::Accepted);
    let failure = result
        .appended_events
        .iter()
        .find(|event| event.event_type == EventKind::WorkBlockFailed)
        .expect("closed physical workplace records a work failure event");
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "blocker_kind" && field.value == "access"));
    assert!(failure
        .payload
        .iter()
        .any(|field| field.key == "reason" && field.value == "workplace access closed"));
}

#[test]
fn source_scan_smoke_tui_does_not_call_event_applier() {
    // Smoke-only guard: the adversarial gate suite proves the command boundary
    // through typed current-view semantic actions and checksum behavior.
    let app_source = include_str!("../src/app.rs");
    let render_source = include_str!("../src/render.rs");
    let input_source = include_str!("../src/input.rs");
    let main_source = include_str!("../src/main.rs");
    let run_source = include_str!("../src/run.rs");

    assert!(!app_source.contains("apply_event"));
    assert!(!render_source.contains("apply_event"));
    assert!(!input_source.contains("apply_event"));
    assert!(!main_source.contains("apply_event"));
    assert!(!run_source.contains("apply_event"));
}
