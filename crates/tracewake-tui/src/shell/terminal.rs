use std::{
    io::{self, stdout},
    panic,
    sync::Once,
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

static PANIC_HOOK: Once = Once::new();

pub trait TerminalLifecycle {
    fn enter(&mut self) -> io::Result<()>;
    fn restore(&mut self) -> io::Result<()>;
}

#[derive(Debug, Default)]
pub struct CrosstermTerminal;

impl TerminalLifecycle for CrosstermTerminal {
    fn enter(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    fn restore(&mut self) -> io::Result<()> {
        let raw_result = disable_raw_mode();
        let screen_result = execute!(stdout(), LeaveAlternateScreen);
        raw_result.and(screen_result)
    }
}

#[derive(Debug)]
pub struct TerminalGuard<T: TerminalLifecycle = CrosstermTerminal> {
    lifecycle: T,
    active: bool,
}

impl TerminalGuard<CrosstermTerminal> {
    pub fn enter_crossterm() -> io::Result<Self> {
        install_panic_restore_hook();
        Self::enter_with(CrosstermTerminal)
    }
}

impl<T: TerminalLifecycle> TerminalGuard<T> {
    pub fn enter_with(mut lifecycle: T) -> io::Result<Self> {
        lifecycle.enter()?;
        Ok(Self {
            lifecycle,
            active: true,
        })
    }

    #[cfg(test)]
    fn lifecycle(&self) -> &T {
        &self.lifecycle
    }

    fn restore(&mut self) -> io::Result<()> {
        if !self.active {
            return Ok(());
        }
        self.active = false;
        self.lifecycle.restore()
    }
}

impl<T: TerminalLifecycle> Drop for TerminalGuard<T> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

fn install_panic_restore_hook() {
    PANIC_HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let mut terminal = CrosstermTerminal;
            let _ = terminal.restore();
            previous(info);
        }));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::RefCell,
        panic::{catch_unwind, AssertUnwindSafe},
        rc::Rc,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum TerminalStep {
        EnterRawMode,
        EnterAlternateScreen,
        DisableRawMode,
        LeaveAlternateScreen,
    }

    #[derive(Clone, Debug)]
    struct RecordingTerminal {
        steps: Rc<RefCell<Vec<TerminalStep>>>,
    }

    impl RecordingTerminal {
        fn new() -> Self {
            Self {
                steps: Rc::new(RefCell::new(Vec::new())),
            }
        }

        fn steps(&self) -> Vec<TerminalStep> {
            self.steps.borrow().clone()
        }
    }

    impl TerminalLifecycle for RecordingTerminal {
        fn enter(&mut self) -> io::Result<()> {
            let mut steps = self.steps.borrow_mut();
            steps.push(TerminalStep::EnterRawMode);
            steps.push(TerminalStep::EnterAlternateScreen);
            Ok(())
        }

        fn restore(&mut self) -> io::Result<()> {
            let mut steps = self.steps.borrow_mut();
            steps.push(TerminalStep::DisableRawMode);
            steps.push(TerminalStep::LeaveAlternateScreen);
            Ok(())
        }
    }

    #[test]
    fn terminal_guard_restores_on_normal_drop() {
        let recorder = RecordingTerminal::new();
        let observed = recorder.clone();

        {
            let guard = TerminalGuard::enter_with(recorder).unwrap();
            assert_eq!(
                guard.lifecycle().steps(),
                vec![
                    TerminalStep::EnterRawMode,
                    TerminalStep::EnterAlternateScreen,
                ]
            );
        }

        assert_eq!(
            observed.steps(),
            vec![
                TerminalStep::EnterRawMode,
                TerminalStep::EnterAlternateScreen,
                TerminalStep::DisableRawMode,
                TerminalStep::LeaveAlternateScreen,
            ]
        );
    }

    #[test]
    fn terminal_guard_restores_during_panic_unwind() {
        let recorder = RecordingTerminal::new();
        let observed = recorder.clone();

        let panic_result = catch_unwind(AssertUnwindSafe(|| {
            let _guard = TerminalGuard::enter_with(recorder).unwrap();
            panic!("forced terminal guard panic");
        }));

        assert!(panic_result.is_err());
        assert_eq!(
            observed.steps(),
            vec![
                TerminalStep::EnterRawMode,
                TerminalStep::EnterAlternateScreen,
                TerminalStep::DisableRawMode,
                TerminalStep::LeaveAlternateScreen,
            ]
        );
    }
}
