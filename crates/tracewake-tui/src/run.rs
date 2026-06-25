use std::io::{BufRead, Write};

use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::SemanticActionId;
use tracewake_core::scheduler::AdvanceUntilStopReason;
use tracewake_core::view_models::EmbodiedViewModel;

use crate::app::{AppError, TuiApp};
use crate::input::{parse_command, semantic_id_for_selection, DebugCommand, InputError, UiCommand};
use crate::render::render_notebook;

const PROMPT: &str = "tracewake>";

pub fn run_command_loop<R, W>(app: &mut TuiApp, reader: R, mut writer: W) -> std::io::Result<()>
where
    R: BufRead,
    W: Write,
{
    writeln!(writer, "{}", app_result(app.render_current_view())?)?;
    write_prompt(&mut writer)?;

    for line in reader.lines() {
        let line = line?;
        match parse_command(&line) {
            Ok(UiCommand::Quit) => return Ok(()),
            Ok(command) => dispatch_command(app, command, &mut writer)?,
            Err(error) => writeln!(writer, "Error: {}", describe_input_error(&error))?,
        }
        write_prompt(&mut writer)?;
    }

    Ok(())
}

fn dispatch_command<W: Write>(
    app: &mut TuiApp,
    command: UiCommand,
    writer: &mut W,
) -> std::io::Result<()> {
    match command {
        UiCommand::Help => writeln!(writer, "{}", help_text()),
        UiCommand::View => writeln!(writer, "{}", app_result(app.render_current_view())?),
        UiCommand::Notebook => {
            let notebook = app_result(app.notebook_view())?;
            writeln!(writer, "{}", render_notebook(&notebook))
        }
        UiCommand::BindActor(actor_id) => match app.bind_actor(actor_id.clone()) {
            Ok(()) => {
                writeln!(writer, "Bound actor: {}", actor_id.as_str())?;
                writeln!(writer, "{}", app_result(app.render_current_view())?)
            }
            Err(error) => writeln!(writer, "Error: {}", describe_app_error(&error)),
        },
        UiCommand::BindDebugActor(actor_id) => match app.bind_debug_actor(actor_id.clone()) {
            Ok(()) => {
                writeln!(writer, "Bound debug actor: {}", actor_id.as_str())?;
                writeln!(writer, "{}", app_result(app.render_current_view())?)
            }
            Err(error) => writeln!(writer, "Error: {}", describe_app_error(&error)),
        },
        UiCommand::SelectSemanticAction(semantic_action_id) => {
            submit_and_render(app, &semantic_action_id, writer)
        }
        UiCommand::SelectByMenuIndex(one_based_selection) => {
            let view = app_result(app.current_view())?;
            // An out-of-range index parses fine but cannot resolve here; report it as
            // an input error and keep the loop alive rather than aborting the TUI.
            match semantic_id_for_selection(&view, one_based_selection) {
                Ok(semantic_action_id) => submit_and_render(app, &semantic_action_id, writer),
                Err(error) => writeln!(writer, "Error: {}", describe_input_error(&error)),
            }
        }
        UiCommand::WaitOneTick => {
            let view = app_result(app.current_view())?;
            let Some(semantic_action_id) = semantic_id_for_wait_alias(&view) else {
                return writeln!(writer, "Error: no such current action: wait");
            };
            submit_and_render(app, &semantic_action_id, writer)
        }
        UiCommand::ContinueUntil { max_ticks } => {
            let result = app_result(app.advance_until(max_ticks))?;
            writeln!(
                writer,
                "Advanced until: reason={} ticks={} stop_tick={}",
                describe_advance_until_stop(result.stop_reason),
                result.ticks_advanced,
                result.stop_tick.value()
            )?;
            writeln!(writer, "{}", app_result(app.render_current_view())?)
        }
        UiCommand::Debug(debug_command) => render_debug(app, debug_command, writer),
        UiCommand::Quit => Ok(()),
    }
}

fn submit_and_render<W: Write>(
    app: &mut TuiApp,
    semantic_action_id: &SemanticActionId,
    writer: &mut W,
) -> std::io::Result<()> {
    let result = match app.submit_semantic_action(semantic_action_id) {
        Ok(result) => result,
        Err(AppError::SemanticActionNotFound(action_id)) => {
            return writeln!(writer, "Error: no such current action: {action_id}");
        }
        Err(error) => return Err(std::io::Error::other(format!("{error:?}"))),
    };
    match result.report.status {
        ReportStatus::Accepted => writeln!(writer, "Accepted: {}", semantic_action_id.as_str())?,
        ReportStatus::Rejected => {
            writeln!(writer, "Why-not: {}", result.report.actor_visible_summary)?
        }
    }
    writeln!(writer, "{}", app_result(app.render_current_view())?)
}

fn semantic_id_for_wait_alias(view: &EmbodiedViewModel) -> Option<SemanticActionId> {
    let mut matching_wait_actions = view
        .semantic_actions
        .iter()
        .filter(|entry| entry.action_id.as_str() == "wait");
    let semantic_action_id = matching_wait_actions.next()?.semantic_action_id.clone();
    if matching_wait_actions.next().is_some() {
        return None;
    }
    Some(semantic_action_id)
}

fn describe_advance_until_stop(reason: AdvanceUntilStopReason) -> &'static str {
    match reason {
        AdvanceUntilStopReason::PossessedDurationTerminal => "possessed_duration_terminal",
        AdvanceUntilStopReason::ActorKnownSalientObservation => "actor_known_salient_observation",
        AdvanceUntilStopReason::UserPausedBeforeNextTick => "user_paused_before_next_tick",
        AdvanceUntilStopReason::ControllerSafetyBound => "controller_safety_bound",
    }
}

fn render_debug<W: Write>(
    app: &mut TuiApp,
    debug_command: DebugCommand,
    writer: &mut W,
) -> std::io::Result<()> {
    if !app.debug_available() {
        return writeln!(writer, "Error: debug unavailable");
    }
    match debug_command {
        DebugCommand::Overlay => {
            let rendered = app_result(app.render_debug_embodied_overlay())?
                .expect("debug availability gate permits overlay rendering");
            writeln!(writer, "{rendered}")
        }
        DebugCommand::EventLog => writeln!(writer, "{}", app.render_debug_event_log_panel()),
        DebugCommand::ControllerBindings => {
            writeln!(writer, "{}", app.render_debug_controller_binding_panel())
        }
        DebugCommand::ItemLocation(item_id) => {
            writeln!(writer, "{}", app.render_debug_item_location_panel(&item_id))
        }
        DebugCommand::Rejection => writeln!(
            writer,
            "{}",
            app.render_debug_action_rejection_panel()
                .unwrap_or_else(|| "DEBUG NON-DIEGETIC: Action Rejection\nnone".to_string())
        ),
        DebugCommand::ProjectionRebuild => {
            writeln!(writer, "{}", app.render_debug_projection_rebuild_panel())
        }
        DebugCommand::Replay => writeln!(writer, "{}", app.render_debug_replay_panel()),
        DebugCommand::Epistemics => {
            writeln!(
                writer,
                "{}",
                crate::debug_panels::render_debug_epistemics_panel(&app.debug_epistemics_view())
            )
        }
        DebugCommand::Beliefs(actor_id) => match app.debug_beliefs_view(&actor_id) {
            Ok(view) => writeln!(
                writer,
                "{}",
                crate::debug_panels::render_debug_beliefs_panel(&view)
            ),
            Err(error) => writeln!(writer, "Error: {}", describe_app_error(&error)),
        },
        DebugCommand::Observations(actor_id) => match app.debug_observations_view(&actor_id) {
            Ok(view) => writeln!(
                writer,
                "{}",
                crate::debug_panels::render_debug_observations_panel(&view)
            ),
            Err(error) => writeln!(writer, "Error: {}", describe_app_error(&error)),
        },
        DebugCommand::Needs => writeln!(writer, "{}", app.render_debug_needs_panel()),
        DebugCommand::Routines => writeln!(writer, "{}", app.render_debug_routines_panel()),
        DebugCommand::Planner(actor_id) => {
            writeln!(writer, "{}", app.render_debug_planner_panel(&actor_id))
        }
        DebugCommand::Stuck => writeln!(writer, "{}", app.render_debug_stuck_panel()),
        DebugCommand::NoHumanDay => writeln!(writer, "{}", app.render_debug_no_human_day_panel()),
        DebugCommand::RunNoHumanDay => {
            app_result(app.run_no_human_day())?;
            writeln!(writer, "{}", app.render_debug_no_human_day_panel())?;
            writeln!(writer, "{}", app_result(app.render_current_view())?)
        }
        DebugCommand::Actor(actor_id) => {
            writeln!(writer, "{}", app.render_debug_actor_panel(&actor_id))
        }
    }
}

fn write_prompt<W: Write>(writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "{PROMPT}")?;
    writer.flush()
}

fn app_result<T>(result: Result<T, crate::app::AppError>) -> std::io::Result<T> {
    result.map_err(|error| std::io::Error::other(format!("{error:?}")))
}

fn describe_app_error(error: &AppError) -> String {
    match error {
        AppError::ActorNotFound(actor_id) => format!("no such actor: {}", actor_id.as_str()),
        AppError::ActorNotBound => "no actor is bound".to_string(),
        AppError::DebugUnavailable => "debug unavailable".to_string(),
        AppError::SemanticActionNotFound(action_id) => {
            format!("no such current action: {action_id}")
        }
        other => format!("{other:?}"),
    }
}

fn describe_input_error(error: &InputError) -> String {
    match error {
        InputError::Empty => "empty command".to_string(),
        InputError::UnknownCommand(command) => format!("unknown command: {command}"),
        InputError::BadActorId(actor_id) => format!("bad actor id: {actor_id}"),
        InputError::BadSemanticActionId(action_id) => {
            format!("bad semantic action id: {action_id}")
        }
        InputError::BadMenuIndex(index) => format!("bad menu selection: {index}"),
        InputError::BadDebugCommand(command) => format!("bad debug command: {command}"),
        InputError::BadItemId(item_id) => format!("bad item id: {item_id}"),
        InputError::SelectionOutOfRange(selection) => {
            format!("menu selection out of range: {selection}")
        }
    }
}

fn help_text() -> &'static str {
    "Commands: help, view, notebook, bind <actor_id>, bind-debug <actor_id>, do <semantic_action_id>, <n>, wait, w, continue, c, continue <max_ticks>, debug overlay, debug log, debug bindings, debug item <item_id>, debug rejection, debug projection, debug replay, debug epistemics, debug beliefs <actor_id>, debug observations <actor_id>, debug needs, debug routines, debug planner <actor_id>, debug stuck, debug no-human-day, debug run no-human-day, debug actor <actor_id>, quit, q"
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::ids::ActorId;

    #[test]
    fn scripted_loop_dispatches_commands_and_exits_cleanly() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let script =
            b"view\nnotebook\ndo close.door.door_house_street\ndo move.to.street_lane\ndebug rejection\nwait\ndebug overlay\ndebug log\ndebug epistemics\ndebug beliefs actor_tomas\ndebug observations actor_tomas\nbogus\nquit\n";
        let mut output = Vec::new();

        run_command_loop(&mut app, &script[..], &mut output).unwrap();

        let rendered = String::from_utf8(output).unwrap();
        assert!(rendered.contains("Actor: actor_tomas"));
        assert!(!rendered.contains("Actor: actor_tomas | Tick: 0"));
        assert!(rendered.contains("Actions:"));
        assert!(rendered.contains("Accepted: close.door.door_house_street"));
        assert!(rendered.contains("Why-not:"));
        assert!(rendered.contains("Notebook: actor_tomas"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Action Rejection"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Embodied Overlay"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Event Log"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Epistemics"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Beliefs"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Observations"));
        assert!(rendered.contains("Error: unknown command: bogus"));
        assert!(rendered.contains(PROMPT));
    }

    #[test]
    fn out_of_range_menu_selection_is_reported_without_crashing() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        // 999 parses as a valid index but exceeds the menu; it must be reported as
        // an input error and the loop must keep running, not abort the whole TUI.
        let script = b"999\nwait\nquit\n";
        let mut output = Vec::new();

        run_command_loop(&mut app, &script[..], &mut output).unwrap();

        let rendered = String::from_utf8(output).unwrap();
        assert!(
            rendered.contains("Error: menu selection out of range: 999"),
            "out-of-range selection must be reported as an input error: {rendered}"
        );
        assert!(
            rendered.contains("Accepted: wait.1_tick"),
            "the command loop must keep running after a bad selection: {rendered}"
        );
    }

    #[test]
    fn unknown_actor_arguments_are_reported_without_crashing() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        // Each of these names a syntactically valid but absent actor; every one must
        // be reported as an input error and leave the existing binding intact so the
        // loop keeps running.
        let script = b"bind actor_ghost\nbind-debug actor_ghost\ndebug beliefs actor_ghost\ndebug observations actor_ghost\nwait\nquit\n";
        let mut output = Vec::new();

        run_command_loop(&mut app, &script[..], &mut output).unwrap();

        let rendered = String::from_utf8(output).unwrap();
        assert_eq!(
            rendered
                .matches("Error: no such actor: actor_ghost")
                .count(),
            4,
            "every unknown-actor command must be reported as an input error: {rendered}"
        );
        assert!(
            rendered.contains("Accepted: wait.1_tick"),
            "the existing binding must survive so the loop keeps running: {rendered}"
        );
    }

    #[test]
    fn debug_commands_refuse_without_debug_availability() {
        let mut app = TuiApp::load_default().unwrap();
        let before_events = app.event_count();
        let mut output = Vec::new();

        render_debug(
            &mut app,
            DebugCommand::Beliefs(ActorId::new("actor_tomas").unwrap()),
            &mut output,
        )
        .unwrap();
        render_debug(&mut app, DebugCommand::RunNoHumanDay, &mut output).unwrap();

        let rendered = String::from_utf8(output).unwrap();
        assert_eq!(rendered.matches("Error: debug unavailable").count(), 2);
        assert_eq!(app.event_count(), before_events);
    }

    #[test]
    fn wait_alias_resolves_only_from_current_view_actions() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let view = app.current_view().unwrap();
        let expected_wait_action = view
            .semantic_actions
            .iter()
            .find(|entry| entry.action_id.as_str() == "wait")
            .expect("fixture surfaces wait action")
            .semantic_action_id
            .clone();

        assert_eq!(
            semantic_id_for_wait_alias(&view),
            Some(expected_wait_action)
        );

        let mut without_wait = view.clone();
        without_wait
            .semantic_actions
            .retain(|entry| entry.action_id.as_str() != "wait");

        assert_eq!(semantic_id_for_wait_alias(&without_wait), None);
    }

    #[test]
    fn help_lists_minimum_command_vocabulary() {
        let help = help_text();
        for command in [
            "help",
            "view",
            "notebook",
            "bind <actor_id>",
            "bind-debug <actor_id>",
            "do <semantic_action_id>",
            "<n>",
            "wait",
            "w",
            "debug log",
            "debug bindings",
            "debug item <item_id>",
            "debug rejection",
            "debug projection",
            "debug replay",
            "debug epistemics",
            "debug beliefs <actor_id>",
            "debug observations <actor_id>",
            "debug needs",
            "debug routines",
            "debug planner <actor_id>",
            "debug stuck",
            "debug no-human-day",
            "debug run no-human-day",
            "debug actor <actor_id>",
            "quit",
            "q",
        ] {
            assert!(help.contains(command));
        }
    }
}
