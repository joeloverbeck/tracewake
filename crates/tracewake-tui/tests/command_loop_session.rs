use std::io::Write;
use std::process::{Command, Stdio};

fn run_session(script: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tracewake-tui"))
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
        "notebook\n1\ndo move.to.street_lane\ndebug log\ndebug epistemics\ndebug beliefs actor_tomas\ndebug observations actor_tomas\nquit\n",
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
    let output = run_session("1\nquit\n");
    let selected_id = first_menu_id(&output, 1);

    assert_ne!(selected_id, "1");
    assert!(output.contains(&format!("Accepted: {selected_id}")));
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

fn checksum_values(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("checksum=").map(ToString::to_string))
        .collect()
}
