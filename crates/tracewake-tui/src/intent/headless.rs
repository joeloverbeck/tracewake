use crate::app::{AppError, TuiApp};
use crate::screen::{
    build_embodied_screen_model, render_embodied_screen_dump, RenderOptions, TerminalSize,
};
use crate::transcript::{
    render_scripted_transcript_step, render_transcript_sections, ScriptedInputSource,
    ScriptedTranscriptStep, TranscriptSection,
};

use super::key_script::{parse_key_script, KeyScriptError};
use super::reducer::{reduce, ReduceError, ReduceOutcome};
use super::semantic_script::{parse_semantic_script, SemanticScriptError};
use super::{PresentationState, UiIntent};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadlessRun {
    pub dumps: Vec<String>,
    pub transcript: String,
}

#[derive(Debug)]
pub enum HeadlessError {
    KeyScript(KeyScriptError),
    SemanticScript(SemanticScriptError),
    Reduce(Box<ReduceError>),
    App(Box<AppError>),
}

pub fn run_key_script(
    app: &mut TuiApp,
    script: &str,
    terminal_size: TerminalSize,
) -> Result<HeadlessRun, HeadlessError> {
    let intents = parse_key_script(script).map_err(HeadlessError::KeyScript)?;
    let inputs = script
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();
    run_intents(
        app,
        ScriptedInputSource::Key,
        inputs,
        intents,
        terminal_size,
    )
}

pub fn run_semantic_script(
    app: &mut TuiApp,
    script: &str,
    terminal_size: TerminalSize,
) -> Result<HeadlessRun, HeadlessError> {
    let intents = parse_semantic_script(script).map_err(HeadlessError::SemanticScript)?;
    let inputs = script
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    run_intents(
        app,
        ScriptedInputSource::Semantic,
        inputs,
        intents,
        terminal_size,
    )
}

fn run_intents(
    app: &mut TuiApp,
    input_source: ScriptedInputSource,
    inputs: Vec<String>,
    intents: Vec<UiIntent>,
    terminal_size: TerminalSize,
) -> Result<HeadlessRun, HeadlessError> {
    let mut state = PresentationState::default();
    let mut dumps = Vec::new();
    let mut sections = Vec::<TranscriptSection>::new();

    for (index, (input, intent)) in inputs.into_iter().zip(intents).enumerate() {
        let semantic_action_id = semantic_action_id_for_intent(&intent);
        let outcome = reduce(app, &mut state, intent.clone())
            .map_err(|error| HeadlessError::Reduce(Box::new(error)))?;
        let debug_output = match outcome {
            ReduceOutcome::DebugRendered(output) => Some(output),
            _ => None,
        };

        let view = app
            .current_view()
            .map_err(|error| HeadlessError::App(Box::new(error)))?;
        let screen_model = build_embodied_screen_model(
            &view,
            RenderOptions {
                terminal_size,
                focused_pane: state.focused_pane(),
                ..RenderOptions::default()
            },
        );
        let dump = render_embodied_screen_dump(&screen_model);
        let step = ScriptedTranscriptStep {
            step_index: index + 1,
            input_source,
            input,
            intent: format!("{intent:?}"),
            semantic_action_id,
            mode: format!("{:?}", view.mode()),
            bound_actor_id: view.viewer_actor_id().as_str().to_string(),
            terminal_size,
            focused_pane: state.focused_pane(),
            debug_marker_present: debug_output.is_some(),
            debug_output,
            rendered_pane_text: dump.clone(),
        };
        sections.push(render_scripted_transcript_step(&step));
        dumps.push(dump);
    }

    Ok(HeadlessRun {
        dumps,
        transcript: render_transcript_sections(&sections),
    })
}

fn semantic_action_id_for_intent(intent: &UiIntent) -> Option<String> {
    match intent {
        UiIntent::SubmitSemanticAction(semantic_action_id) => {
            Some(semantic_action_id.as_str().to_string())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_content::fixtures;
    use tracewake_core::ids::ActorId;

    #[test]
    fn headless_semantic_run_emits_dump_and_scripted_transcript() {
        let mut app = bound_app();
        let script = "\
focus actions
submit semantic_action_id=open.container.strongbox_tomas
wait_one_tick
continue_until max_ticks=1
notebook
";

        let run = run_semantic_script(&mut app, script, TerminalSize::new(80, 24)).unwrap();

        assert_eq!(run.dumps.len(), 5);
        assert!(run.dumps[0].contains("SCREEN mode=Embodied size=80x24"));
        assert!(run.transcript.contains("input_source: Semantic"));
        assert!(run
            .transcript
            .contains("input: submit semantic_action_id=open.container.strongbox_tomas"));
        assert!(run
            .transcript
            .contains("semantic_action_id: open.container.strongbox_tomas"));
        assert!(run.transcript.contains("mode: Embodied"));
        assert!(run.transcript.contains("bound_actor_id: actor_tomas"));
        assert!(run.transcript.contains("terminal_size: 80x24"));
        assert!(run.transcript.contains("focused_pane: Notebook"));
        assert!(run.transcript.contains("rendered_pane_text:"));
    }

    #[test]
    fn headless_key_run_is_byte_deterministic() {
        let first =
            run_key_script(&mut bound_app(), "Tab Down w n", TerminalSize::new(80, 24)).unwrap();
        let second =
            run_key_script(&mut bound_app(), "Tab Down w n", TerminalSize::new(80, 24)).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn headless_embodied_run_does_not_leak_debug_truth() {
        let mut app = bound_app();
        let run =
            run_semantic_script(&mut app, "wait_one_tick", TerminalSize::new(80, 24)).unwrap();

        assert!(!run.dumps.join("\n").contains("DEBUG NON-DIEGETIC"));
        assert!(!run.transcript.contains("DEBUG NON-DIEGETIC"));
    }

    fn bound_app() -> TuiApp {
        let mut app = TuiApp::from_golden(fixtures::strongbox_001()).unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        app
    }
}
