use std::io::{BufRead, Write};

use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::SemanticActionId;

use crate::app::TuiApp;
use crate::input::{parse_command, semantic_id_for_selection, DebugCommand, InputError, UiCommand};
use crate::render::render_notebook;

const PROMPT: &str = "tracewake>";
const WAIT_ACTION_ID: &str = "wait.1_tick";

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
        UiCommand::BindActor(actor_id) => {
            app_result(app.bind_actor(actor_id.clone()))?;
            writeln!(writer, "Bound actor: {}", actor_id.as_str())?;
            writeln!(writer, "{}", app_result(app.render_current_view())?)
        }
        UiCommand::SelectSemanticAction(semantic_action_id) => {
            submit_and_render(app, &semantic_action_id, writer)
        }
        UiCommand::SelectByMenuIndex(one_based_selection) => {
            let view = app_result(app.current_view())?;
            let semantic_action_id = semantic_id_for_selection(&view, one_based_selection)
                .map_err(|error| std::io::Error::other(describe_input_error(&error)))?;
            submit_and_render(app, &semantic_action_id, writer)
        }
        UiCommand::WaitOneTick => {
            let semantic_action_id = SemanticActionId::new(WAIT_ACTION_ID).expect("valid wait ID");
            submit_and_render(app, &semantic_action_id, writer)
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
    let result = app_result(app.submit_semantic_action(semantic_action_id))?;
    match result.report.status {
        ReportStatus::Accepted => writeln!(writer, "Accepted: {}", semantic_action_id.as_str())?,
        ReportStatus::Rejected => {
            writeln!(writer, "Why-not: {}", result.report.actor_visible_summary)?
        }
    }
    writeln!(writer, "{}", app_result(app.render_current_view())?)
}

fn render_debug<W: Write>(
    app: &TuiApp,
    debug_command: DebugCommand,
    writer: &mut W,
) -> std::io::Result<()> {
    match debug_command {
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
        DebugCommand::Beliefs(actor_id) => {
            let view = app_result(app.debug_beliefs_view(&actor_id))?;
            writeln!(
                writer,
                "{}",
                crate::debug_panels::render_debug_beliefs_panel(&view)
            )
        }
        DebugCommand::Observations(actor_id) => {
            let view = app_result(app.debug_observations_view(&actor_id))?;
            writeln!(
                writer,
                "{}",
                crate::debug_panels::render_debug_observations_panel(&view)
            )
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
    "Commands: help, view, notebook, bind <actor_id>, do <semantic_action_id>, <n>, wait, w, debug log, debug bindings, debug item <item_id>, debug rejection, debug projection, debug replay, debug epistemics, debug beliefs <actor_id>, debug observations <actor_id>, quit, q"
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::ids::ActorId;

    #[test]
    fn scripted_loop_dispatches_commands_and_exits_cleanly() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let script =
            b"view\nnotebook\ndo close.door.door_house_street\ndo move.to.street_lane\ndebug rejection\nwait\ndebug log\ndebug epistemics\ndebug beliefs actor_tomas\ndebug observations actor_tomas\nbogus\nquit\n";
        let mut output = Vec::new();

        run_command_loop(&mut app, &script[..], &mut output).unwrap();

        let rendered = String::from_utf8(output).unwrap();
        assert!(rendered.contains("Actor: actor_tomas | Tick: 0"));
        assert!(rendered.contains("Actions:"));
        assert!(rendered.contains("Accepted: close.door.door_house_street"));
        assert!(rendered.contains("Why-not:"));
        assert!(rendered.contains("Notebook: actor_tomas"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Action Rejection"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Event Log"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Epistemics"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Beliefs"));
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Observations"));
        assert!(rendered.contains("Error: unknown command: bogus"));
        assert!(rendered.contains(PROMPT));
    }

    #[test]
    fn help_lists_minimum_command_vocabulary() {
        let help = help_text();
        for command in [
            "help",
            "view",
            "notebook",
            "bind <actor_id>",
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
            "quit",
            "q",
        ] {
            assert!(help.contains(command));
        }
    }
}
