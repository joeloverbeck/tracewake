fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let launch = match tracewake_tui::launch::resolve(&args) {
        Ok(launch) => launch,
        Err(error) => {
            eprintln!("{}", tracewake_tui::launch::render_error(&error));
            std::process::exit(2);
        }
    };

    let tracewake_tui::launch::Launch::Run { golden, actor_id } = launch else {
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

    let mut app = tracewake_tui::app::TuiApp::from_golden(*golden).expect("fixture loads");
    app.bind_actor(actor_id).expect("selected actor exists");
    println!("{}", tracewake_tui::startup_message());
    tracewake_tui::run::run_command_loop(
        &mut app,
        std::io::stdin().lock(),
        std::io::stdout().lock(),
    )
    .expect("command loop runs");
}
