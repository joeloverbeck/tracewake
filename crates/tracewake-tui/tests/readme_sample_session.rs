use std::io::Write;
use std::process::{Command, Stdio};

const README: &str = include_str!("../../../README.md");

#[test]
fn readme_documented_commands_are_live_against_binary() {
    for documented in [
        "help",
        "view",
        "bind <actor_id>",
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

    let ordinary_output = run_session_with_args(
        &[],
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
         q\n",
    );

    assert!(!ordinary_output.contains("Error:"), "{ordinary_output}");
    assert!(ordinary_output.contains("tracewake-tui ready"));
    assert!(ordinary_output.contains("Commands: help, view"));
    assert!(ordinary_output.contains("Bound actor: actor_elena"));
    assert!(ordinary_output.contains("Bound actor: actor_tomas"));
    assert!(ordinary_output.contains("Accepted: open.container.strongbox_tomas"));
    assert!(ordinary_output.contains("Accepted: check.container.strongbox_tomas"));
    assert!(ordinary_output.contains("Notebook: actor_tomas"));
    assert!(ordinary_output.contains("Accepted: wait.1_tick"));

    let debug_output = run_session_with_args(
        &["--operator-debug", "strongbox_001", "actor_tomas"],
        "do open.container.strongbox_tomas\n\
         do check.container.strongbox_tomas\n\
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

    assert!(!debug_output.contains("Error:"), "{debug_output}");
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Event Log"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Controller Binding"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Item Location"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Action Rejection"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Projection Rebuild"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(debug_output.contains("DEBUG NON-DIEGETIC: Observations"));
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
