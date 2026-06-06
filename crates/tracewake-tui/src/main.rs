fn main() {
    let mut app = tracewake_tui::app::TuiApp::load_default().expect("default fixture loads");
    app.bind_actor("actor_tomas".parse().expect("default actor ID"))
        .expect("default actor exists");
    println!("{}", tracewake_tui::startup_message());
    tracewake_tui::run::run_command_loop(
        &mut app,
        std::io::stdin().lock(),
        std::io::stdout().lock(),
    )
    .expect("command loop runs");
}
