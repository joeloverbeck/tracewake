use std::io::Write;
use std::process::{Command, Stdio};

fn run_session(script: &str) -> String {
    run_session_with_args(&[], script)
}

#[allow(
    clippy::disallowed_methods,
    reason = "TUI subprocess smoke test launches the binary; this is not simulation outcome code"
)]
fn run_session_with_args(args: &[&str], script: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tracewake-tui"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("tracewake-tui binary starts");

    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(script.as_bytes())
        .expect("script writes to stdin");

    let output = child.wait_with_output().expect("tracewake-tui exits");
    assert!(
        output.status.success(),
        "status={:?}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout is UTF-8")
}

#[test]
fn scripted_session_exercises_actual_binary_loop() {
    let output = run_session(
        "notebook\ndo close.door.door_house_street\ndo move.to.street_lane\ndebug log\ndebug epistemics\ndebug beliefs actor_tomas\ndebug observations actor_tomas\nquit\n",
    );

    assert!(output.contains("tracewake-tui ready"));
    assert!(output.contains("Actor: actor_tomas | Tick: 0"));
    assert!(output.contains("Actions:"));
    assert!(output.contains("Accepted: close.door.door_house_street"));
    assert!(output.contains("Why-not: The door is closed."));
    assert!(output.contains("Notebook: actor_tomas"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Event Log"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Observations"));
    assert!(output.contains("tracewake>"));
}

#[test]
fn malformed_debug_actor_id_is_typed_error() {
    let output = run_session("debug beliefs BAD\nquit\n");

    assert!(output.contains("Error: bad actor id: BAD"));
}

#[test]
fn numeric_selection_executes_stable_semantic_action_id() {
    let output = run_session("2\nquit\n");
    let selected_id = first_menu_id(&output, 2);

    assert_ne!(selected_id, "1");
    assert!(output.contains(&format!("Accepted: {selected_id}")));
}

#[test]
fn wait_command_executes_current_view_wait_action() {
    let output = run_session("wait\nquit\n");
    let wait_action_id = first_menu_id_with_label(&output, "Wait");

    assert!(output.contains(&format!("Accepted: {wait_action_id}")));
}

#[test]
fn debug_item_does_not_leak_to_following_view_or_change_checksum() {
    let output = run_session("debug item coin_stack_01\ndebug projection\nview\nquit\n");

    assert!(output.contains("DEBUG NON-DIEGETIC: Item Location"));
    assert!(output.contains("current_location=container:strongbox_tomas -> place:house_tomas"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Projection Rebuild"));

    let checksums = checksum_values(&output);
    assert!(
        checksums.len() >= 2,
        "expected item and projection checksums in output:\n{output}"
    );
    assert_eq!(checksums[0], checksums[1]);

    let following_view = output
        .rsplit_once("Actor: actor_tomas | Tick: 0")
        .map(|(_, view)| view)
        .expect("following embodied view renders");
    assert!(!following_view.contains("coin_stack_01"));
    assert!(!following_view.contains("container:strongbox_tomas"));
}

#[test]
fn no_human_day_command_loop_renders_phase3a_behavior_rows() {
    let output = run_session_with_args(
        &["no_human_day_001", "actor_tomas"],
        "run no-human-day\ndebug routines\ndebug replay\nquit\n",
    );

    assert!(output.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(!output.contains("Ran no-human day:"));
    assert!(!output.contains("ordinary_events="));
    assert!(output.contains("routine_events=8"));
    assert!(output.contains("work_failed=2"));
    assert!(output.contains("routine_interruptions=2"));
    assert!(output.contains("routine_exec_mara_eat"));
    assert!(output.contains("status=Failed"));
    assert!(output.contains("routine_exec_tomas_work"));
    assert!(output.contains("status=Completed"));
    assert!(output.contains("- hunger: band=rising cause=tick_delta"));
    assert!(!output.contains("value=410"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(output.contains("fixture=no_human_day_001"));
    assert!(!output.contains("food_hidden_pantry"));
}

fn first_menu_id(output: &str, one_based_index: usize) -> String {
    let prefix = format!("{one_based_index}. ");
    let line = output
        .lines()
        .find(|line| line.starts_with(&prefix))
        .expect("menu line exists");
    let (_, id_with_suffix) = line.rsplit_once('[').expect("menu line has stable ID");
    let (id, _) = id_with_suffix
        .split_once(']')
        .expect("menu line closes stable ID");
    id.to_string()
}

fn first_menu_id_with_label(output: &str, label: &str) -> String {
    let line = output
        .lines()
        .find(|line| line.contains(label) && line.contains('['))
        .expect("menu line with label exists");
    let (_, id_with_suffix) = line.rsplit_once('[').expect("menu line has stable ID");
    let (id, _) = id_with_suffix
        .split_once(']')
        .expect("menu line closes stable ID");
    id.to_string()
}

fn checksum_values(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("checksum=").map(ToString::to_string))
        .collect()
}
