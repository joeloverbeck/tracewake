use std::path::PathBuf;
use std::process::Command;

struct NegativeFixture {
    name: &'static str,
    lint_category: &'static str,
}

const FIXTURES: &[NegativeFixture] = &[
    NegativeFixture {
        name: "banned_hashmap_direct_path",
        lint_category: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_hashmap_import_alias",
        lint_category: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_hashset_reexport",
        lint_category: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_systemtime_alias",
        lint_category: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_instant_alias",
        lint_category: "disallowed_types",
    },
    NegativeFixture {
        name: "banned_thread_spawn_direct",
        lint_category: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_thread_spawn_reexport",
        lint_category: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_fs_read_and_file_open",
        lint_category: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_tcp_udp_network_calls",
        lint_category: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_process_command_new",
        lint_category: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_macro_expands_to_spawn_or_fs",
        lint_category: "disallowed_methods",
    },
];

fn fixture_root(fixture: &NegativeFixture) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/negative-fixtures")
        .join(fixture.name)
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "negative fixture runner must spawn cargo to assert pinned clippy policy failures"
)]
fn banned_api_negative_fixtures_fail_with_expected_lints() {
    for fixture in FIXTURES {
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
            stderr.contains(fixture.lint_category),
            "negative fixture {} failed without expected lint category {}.\nstderr:\n{}",
            fixture.name,
            fixture.lint_category,
            stderr
        );
    }
}
