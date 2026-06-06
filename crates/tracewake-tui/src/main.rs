fn main() {
    let mut app = tracewake_tui::app::TuiApp::load_default().expect("default fixture loads");
    app.bind_actor("actor_tomas".parse().expect("default actor ID"))
        .expect("default actor exists");
    println!("{}", tracewake_tui::startup_message());
    println!(
        "{}",
        app.render_current_view().expect("default view renders")
    );
}
