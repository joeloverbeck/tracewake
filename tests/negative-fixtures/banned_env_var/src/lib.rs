pub fn ambient_environment() -> Option<String> {
    std::env::var("TRACEWAKE_FIXTURE").ok()
}
