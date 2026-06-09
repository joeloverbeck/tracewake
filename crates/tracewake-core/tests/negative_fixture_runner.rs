use std::path::PathBuf;
use std::process::Command;

const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");

struct NegativeFixture {
    name: &'static str,
    expected_stderr: &'static str,
}

const FIXTURES: &[NegativeFixture] = &[
    NegativeFixture {
        name: "banned_hashmap_direct_path",
        expected_stderr: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_hashmap_import_alias",
        expected_stderr: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_hashset_reexport",
        expected_stderr: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_systemtime_alias",
        expected_stderr: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_instant_alias",
        expected_stderr: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_thread_spawn_direct",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_thread_spawn_reexport",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_fs_read_and_file_open",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_tcp_udp_network_calls",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_process_command_new",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_macro_expands_to_spawn_or_fs",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "external_crate_cannot_call_seed_mutators_after_load",
        expected_stderr: "no method named `seed_items_mut`",
    },
    NegativeFixture {
        name: "external_crate_cannot_mutate_agent_state_seed_maps",
        expected_stderr: "no method named `seed_intentions_mut`",
    },
    NegativeFixture {
        name: "external_crate_cannot_forge_mutation_capability",
        expected_stderr: "module `mutation` is private",
    },
    NegativeFixture {
        name: "external_crate_cannot_construct_debug_report",
        expected_stderr: "private",
    },
];

fn fixture_root(fixture: &NegativeFixture) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/negative-fixtures")
        .join(fixture.name)
}

fn body_after_marker<'a>(source: &'a str, marker: &str) -> &'a str {
    let after_marker = source
        .split(marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{marker} is present"));
    let start = after_marker
        .find('{')
        .unwrap_or_else(|| panic!("{marker} body starts with an opening brace"));
    let mut depth = 0_i32;
    let mut end = None;
    for (offset, byte) in after_marker[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    &after_marker[start..end.unwrap_or_else(|| panic!("{marker} body closes"))]
}

fn phase1_loader_later_phase_registration_violations(source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let load_body = body_after_marker(source, "pub fn load_fixture_package");
    let phase1_body = body_after_marker(source, "FixtureScope::Phase1 =>");
    for token in [
        "register_phase2a_",
        "register_phase3a_",
        "register_phase2a_epistemics",
        "register_phase3a_sleep",
        "register_phase3a_eat",
        "register_phase3a_work",
        "register_phase3a_continue_routine",
    ] {
        if load_body.contains(token) {
            violations.push(format!("load_fixture_package directly calls {token}"));
        }
        if phase1_body.contains(token) {
            violations.push(format!("FixtureScope::Phase1 arm calls {token}"));
        }
    }
    violations
}

#[test]
fn phase1_loader_registration_guard_negative_fixture_fires_on_mutation() {
    let mut mutated = CONTENT_LOAD_RS.to_string();
    mutated = mutated.replace(
        "FixtureScope::Phase1 => {}",
        "FixtureScope::Phase1 => { registry.register_phase3a_sleep(); }",
    );

    let violations = phase1_loader_later_phase_registration_violations(&mutated);
    assert!(
        violations
            .iter()
            .any(|violation| violation.contains("FixtureScope::Phase1 arm calls")),
        "source guard must catch a deliberate Phase 1 loader later-phase registration mutation"
    );
}

#[allow(
    clippy::disallowed_methods,
    reason = "negative fixture runner must spawn cargo to assert isolated fixture failures"
)]
fn assert_negative_fixture_fails(fixture: &NegativeFixture) {
    let root = fixture_root(fixture);
    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--quiet")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir(&root)
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run cargo clippy for fixture {} at {}: {error}",
                fixture.name,
                root.display()
            )
        });

    assert!(
        !output.status.success(),
        "negative fixture {} unexpectedly passed clippy",
        fixture.name
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(fixture.expected_stderr),
        "negative fixture {} failed without expected stderr fragment {}.\nstderr:\n{}",
        fixture.name,
        fixture.expected_stderr,
        stderr
    );
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "negative fixture runner must spawn cargo to assert pinned clippy policy failures"
)]
fn banned_api_negative_fixtures_fail_with_expected_lints() {
    for fixture in FIXTURES {
        assert_negative_fixture_fails(fixture);
    }
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "negative fixture runner must spawn cargo to assert the external debug-report fixture fails"
)]
fn debug_report_construction_without_capability_compile_fails() {
    let fixture = FIXTURES
        .iter()
        .find(|fixture| fixture.name == "external_crate_cannot_construct_debug_report")
        .expect("debug report negative fixture is registered");

    assert_negative_fixture_fails(fixture);
}
