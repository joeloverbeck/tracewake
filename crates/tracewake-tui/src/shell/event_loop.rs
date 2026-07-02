use std::{
    io::{self, stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyEvent},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    app::{AppError, TuiApp},
    intent::{
        reducer::{reduce, ReduceError, ReduceOutcome},
        PresentationState,
    },
    screen::{build_embodied_screen_model, render_embodied_to_buffer, RenderOptions, TerminalSize},
};

use super::key_map::key_event_to_intent;

const DEFAULT_POLL_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellState {
    presentation: PresentationState,
    terminal_size: TerminalSize,
}

impl ShellState {
    pub fn new(terminal_size: TerminalSize) -> Self {
        Self {
            presentation: PresentationState::default(),
            terminal_size,
        }
    }

    pub fn presentation(&self) -> &PresentationState {
        &self.presentation
    }

    pub fn presentation_mut(&mut self) -> &mut PresentationState {
        &mut self.presentation
    }

    pub fn terminal_size(&self) -> TerminalSize {
        self.terminal_size
    }

    pub fn set_terminal_size(&mut self, terminal_size: TerminalSize) {
        self.terminal_size = terminal_size;
    }
}

#[derive(Debug)]
pub enum ShellError {
    App(Box<AppError>),
    Io(io::Error),
    Reduce(ReduceError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellStep {
    Continue,
    Quit,
    Ignored,
}

impl From<AppError> for ShellError {
    fn from(value: AppError) -> Self {
        Self::App(Box::new(value))
    }
}

impl From<io::Error> for ShellError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ReduceError> for ShellError {
    fn from(value: ReduceError) -> Self {
        Self::Reduce(value)
    }
}

pub fn run_fullscreen_shell(app: &mut TuiApp) -> Result<(), ShellError> {
    let (columns, rows) = terminal::size()?;
    let mut state = ShellState::new(TerminalSize::new(columns, rows));
    let mut output = stdout();
    redraw(app, &state, &mut output)?;

    loop {
        if !event::poll(DEFAULT_POLL_TIMEOUT)? {
            continue;
        }

        match event::read()? {
            Event::Key(key_event) => {
                let step = apply_key_event(app, &mut state, key_event)?;
                redraw(app, &state, &mut output)?;
                if matches!(step, ShellStep::Quit) {
                    break;
                }
            }
            Event::Resize(columns, rows) => {
                state.set_terminal_size(TerminalSize::new(columns, rows));
                redraw(app, &state, &mut output)?;
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn apply_key_event(
    app: &mut TuiApp,
    state: &mut ShellState,
    key_event: KeyEvent,
) -> Result<ShellStep, ShellError> {
    let Some(intent) = key_event_to_intent(key_event) else {
        return Ok(ShellStep::Ignored);
    };
    let outcome = reduce(app, state.presentation_mut(), intent)?;
    if matches!(outcome, ReduceOutcome::Quit) {
        Ok(ShellStep::Quit)
    } else {
        Ok(ShellStep::Continue)
    }
}

pub fn redraw_buffer(app: &TuiApp, state: &ShellState) -> Result<Buffer, ShellError> {
    let view = app.current_view()?;
    let options = RenderOptions {
        terminal_size: state.terminal_size(),
        focused_pane: state.presentation().focused_pane(),
        ..RenderOptions::default()
    };
    let model = build_embodied_screen_model(&view, options);
    let area = Rect::new(
        0,
        0,
        state.terminal_size().columns,
        state.terminal_size().rows,
    );
    Ok(render_embodied_to_buffer(&model, area))
}

pub fn redraw<W: Write>(
    app: &TuiApp,
    state: &ShellState,
    output: &mut W,
) -> Result<(), ShellError> {
    let buffer = redraw_buffer(app, state)?;
    draw_buffer(output, &buffer)?;
    Ok(())
}

pub fn draw_buffer<W: Write>(output: &mut W, buffer: &Buffer) -> io::Result<()> {
    execute!(output, Clear(ClearType::All))?;
    let area = buffer.area;
    let width = usize::from(area.width);

    for row in 0..area.height {
        let start = usize::from(row) * width;
        let end = start + width;
        let line = buffer.content[start..end]
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        execute!(output, MoveTo(area.x, area.y + row), Print(line.trim_end()))?;
    }
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::buffer_to_plain_text;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use tracewake_content::fixtures;
    use tracewake_core::ids::ActorId;

    #[test]
    fn redraw_matches_direct_screen_buffer_rendering() {
        let app = bound_app();
        let state = ShellState::new(TerminalSize::new(96, 30));

        let shell_buffer = redraw_buffer(&app, &state).expect("shell redraw succeeds");

        let view = app.current_view().unwrap();
        let direct_model = build_embodied_screen_model(
            &view,
            RenderOptions {
                terminal_size: TerminalSize::new(96, 30),
                focused_pane: state.presentation().focused_pane(),
                ..RenderOptions::default()
            },
        );
        let direct_buffer = render_embodied_to_buffer(&direct_model, Rect::new(0, 0, 96, 30));

        assert_eq!(
            buffer_to_plain_text(&shell_buffer),
            buffer_to_plain_text(&direct_buffer)
        );
    }

    #[test]
    fn draw_buffer_writes_plain_buffer_without_tty() {
        let app = bound_app();
        let state = ShellState::new(TerminalSize::new(80, 24));
        let buffer = redraw_buffer(&app, &state).expect("shell redraw succeeds");
        let mut output = Vec::new();

        draw_buffer(&mut output, &buffer).expect("buffer writes to memory");

        assert!(!output.is_empty());
        let rendered = String::from_utf8_lossy(&output);
        assert!(rendered.contains("[Header / Mode]"));
    }

    #[test]
    fn quit_intent_terminates_shell_key_step() {
        let mut app = bound_app();
        let mut state = ShellState::new(TerminalSize::new(80, 24));

        let step = apply_key_event(&mut app, &mut state, key(KeyCode::Char('q')))
            .expect("quit key reduces");

        assert_eq!(step, ShellStep::Quit);
    }

    fn bound_app() -> TuiApp {
        let mut app = TuiApp::from_golden(fixtures::strongbox_001()).unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        app
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }
}
