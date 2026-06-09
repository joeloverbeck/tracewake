use std::path::PathBuf;
use std::process::Command;

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
