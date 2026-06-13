use std::io::Write;
use std::process::{Command, Stdio};

const README: &str = include_str!("../../../README.md");

#[test]
fn readme_documented_commands_are_live_against_binary() {
    for documented in [
        "help",
        "view",
        "bind <actor_id>",
        "bind-debug <actor_id>",
        "<n>",
        "do <semantic_action_id>",
        "notebook",
        "wait",
        "w",
        "debug log",
        "debug bindings",
        "debug item <item_id>",
        "debug rejection",
        "debug projection",
        "debug replay",
        "debug epistemics",
        "debug beliefs <actor_id>",
        "debug observations <actor_id>",
        "debug run no-human-day",
        "quit",
        "q",
    ] {
        assert!(
            README.contains(documented),
            "README command block should document {documented}"
        );
    }

    let output = run_session(
        "help\n\
         view\n\
         bind actor_elena\n\
         bind actor_tomas\n\
         do open.container.strongbox_tomas\n\
         do check.container.strongbox_tomas\n\
         notebook\n\
         do wait.1_tick\n\
         wait\n\
         w\n\
         bind-debug actor_tomas\n\
         debug log\n\
         debug bindings\n\
         debug item coin_stack_01\n\
         debug rejection\n\
         debug projection\n\
         debug replay\n\
         debug epistemics\n\
         debug beliefs actor_tomas\n\
         debug observations actor_tomas\n\
         q\n",
    );

    assert!(!output.contains("Error:"), "{output}");
    assert!(output.contains("tracewake-tui ready"));
    assert!(output.contains("Commands: help, view"));
    assert!(output.contains("Bound actor: actor_elena"));
    assert!(output.contains("Bound actor: actor_tomas"));
    assert!(output.contains("Bound debug actor: actor_tomas"));
    assert!(output.contains("Accepted: open.container.strongbox_tomas"));
    assert!(output.contains("Accepted: check.container.strongbox_tomas"));
    assert!(output.contains("Notebook: actor_tomas"));
    assert!(output.contains("Accepted: wait.1_tick"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Event Log"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Controller Binding"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Item Location"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Action Rejection"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Projection Rebuild"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(output.contains("DEBUG NON-DIEGETIC: Observations"));
}

#[allow(
    clippy::disallowed_methods,
    reason = "TUI subprocess smoke test launches the binary; this is not simulation outcome code"
)]
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
