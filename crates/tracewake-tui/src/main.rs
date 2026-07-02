fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let launch = match tracewake_tui::launch::resolve(&args) {
        Ok(launch) => launch,
        Err(error) => {
            eprintln!("{}", tracewake_tui::launch::render_error(&error));
            std::process::exit(2);
        }
    };

    let tracewake_tui::launch::Launch::Run {
        golden,
        actor_id,
        mode,
        fullscreen,
    } = launch
    else {
        match launch {
            tracewake_tui::launch::Launch::List => {
                println!("{}", tracewake_tui::launch::render_catalog());
            }
            tracewake_tui::launch::Launch::Help => {
                println!("{}", tracewake_tui::launch::usage());
            }
            tracewake_tui::launch::Launch::Run { .. } => unreachable!(),
        }
        return;
    };

    let mut app = match mode {
        tracewake_tui::launch::LaunchMode::Embodied => {
            let mut app = tracewake_tui::app::TuiApp::from_golden(*golden).expect("fixture loads");
            app.bind_actor(actor_id).expect("selected actor exists");
            app
        }
        tracewake_tui::launch::LaunchMode::OperatorDebug => {
            let mut app = tracewake_tui::app::TuiApp::from_golden_operator_debug(*golden)
                .expect("fixture loads");
            app.bind_debug_actor(actor_id)
                .expect("selected actor exists");
            app
        }
    };
    if fullscreen {
        let _terminal_guard =
            tracewake_tui::shell::TerminalGuard::enter_crossterm().expect("terminal enters");
        tracewake_tui::shell::event_loop::run_fullscreen_shell(&mut app).expect("shell runs");
    } else {
        println!("{}", tracewake_tui::startup_message());
        tracewake_tui::run::run_command_loop(
            &mut app,
            std::io::stdin().lock(),
            std::io::stdout().lock(),
        )
        .expect("command loop runs");
    }
}
