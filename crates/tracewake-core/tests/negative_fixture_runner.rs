use std::path::PathBuf;
use std::process::Command;

const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");
const CLIPPY_TOML: &str = include_str!("../../../clippy.toml");

struct NegativeFixture {
    name: &'static str,
    expected_stderr: &'static str,
}

const FIXTURES: &[NegativeFixture] = &[
    NegativeFixture {
        name: "banned_env_var",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_float_confidence_types",
        expected_stderr: "disallowed_types",
    },
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
        name: "banned_fs_write_open_options",
        expected_stderr: "disallowed_methods",
    },
    NegativeFixture {
        name: "banned_rand_entry_points",
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
        name: "banned_known_food_blanket_helper",
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
    NegativeFixture {
        name: "external_crate_cannot_mutate_knowledge_context_mode",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_mutate_knowledge_context_viewer",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_build_debug_knowledge_context",
        expected_stderr: "this function takes 3 arguments but 2 arguments were supplied",
    },
    NegativeFixture {
        name: "external_crate_cannot_read_raw_epistemic_projection_maps",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_insert_raw_epistemic_records",
        expected_stderr: "private method",
    },
    NegativeFixture {
        name: "external_crate_cannot_build_debug_projection_view_without_core_debug_api",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_construct_belief_literal",
        expected_stderr: "are private",
    },
    NegativeFixture {
        name: "external_crate_cannot_mutate_belief_source_or_scope",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_construct_observation_without_source",
        expected_stderr: "other private field `source`",
    },
    NegativeFixture {
        name: "external_crate_cannot_mutate_contradiction_links",
        expected_stderr: "private field",
    },
    NegativeFixture {
        name: "external_crate_cannot_forge_interval_notice",
        expected_stderr: "private",
    },
    NegativeFixture {
        name: "external_crate_cannot_convert_debug_report_to_interval_summary",
        expected_stderr: "the trait bound",
    },
];

struct ClippyBanProof {
    clippy_path: &'static str,
    proving_fixture: &'static str,
}

const CLIPPY_BAN_PROOFS: &[ClippyBanProof] = &[
    ClippyBanProof {
        clippy_path: "f32",
        proving_fixture: "banned_float_confidence_types",
    },
    ClippyBanProof {
        clippy_path: "f64",
        proving_fixture: "banned_float_confidence_types",
    },
    ClippyBanProof {
        clippy_path: "rand::random",
        proving_fixture: "banned_rand_entry_points",
    },
    ClippyBanProof {
        clippy_path: "rand::rng",
        proving_fixture: "banned_rand_entry_points",
    },
    ClippyBanProof {
        clippy_path: "std::collections::HashMap",
        proving_fixture: "banned_hashmap_direct_path",
    },
    ClippyBanProof {
        clippy_path: "std::collections::HashSet",
        proving_fixture: "banned_hashset_reexport",
    },
    ClippyBanProof {
        clippy_path: "std::env::var",
        proving_fixture: "banned_env_var",
    },
    ClippyBanProof {
        clippy_path: "std::fs::File::open",
        proving_fixture: "banned_fs_read_and_file_open",
    },
    ClippyBanProof {
        clippy_path: "std::fs::OpenOptions",
        proving_fixture: "banned_fs_write_open_options",
    },
    ClippyBanProof {
        clippy_path: "std::fs::read",
        proving_fixture: "banned_fs_read_and_file_open",
    },
    ClippyBanProof {
        clippy_path: "std::fs::read_to_string",
        proving_fixture: "banned_fs_read_and_file_open",
    },
    ClippyBanProof {
        clippy_path: "std::fs::write",
        proving_fixture: "banned_fs_write_open_options",
    },
    ClippyBanProof {
        clippy_path: "std::net::TcpStream::connect",
        proving_fixture: "banned_tcp_udp_network_calls",
    },
    ClippyBanProof {
        clippy_path: "std::net::UdpSocket::bind",
        proving_fixture: "banned_tcp_udp_network_calls",
    },
    ClippyBanProof {
        clippy_path: "std::process::Command::new",
        proving_fixture: "banned_process_command_new",
    },
    ClippyBanProof {
        clippy_path: "std::thread::spawn",
        proving_fixture: "banned_thread_spawn_direct",
    },
    ClippyBanProof {
        clippy_path: "std::time::Instant",
        proving_fixture: "banned_instant_alias",
    },
    ClippyBanProof {
        clippy_path: "std::time::SystemTime",
        proving_fixture: "banned_systemtime_alias",
    },
    ClippyBanProof {
        clippy_path:
            "tracewake_content::schema::FixtureSchema::populate_known_food_sources_for_all_actors",
        proving_fixture: "banned_known_food_blanket_helper",
    },
];

fn fixture_root(fixture: &NegativeFixture) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/negative-fixtures")
        .join(fixture.name)
}

fn negative_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/negative-fixtures")
}

fn registered_fixture_names() -> Vec<String> {
    let mut names = FIXTURES
        .iter()
        .map(|fixture| fixture.name.to_string())
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[allow(
    clippy::disallowed_methods,
    reason = "negative fixture census must inspect fixture directories outside simulation outcome code"
)]
fn fixture_directory_names() -> Vec<String> {
    let mut names = std::fs::read_dir(negative_fixture_dir())
        .expect("negative fixture directory is readable")
        .map(|entry| {
            let entry = entry.expect("negative fixture directory entry is readable");
            let path = entry.path();
            assert!(
                path.is_dir(),
                "{} is not a fixture directory",
                path.display()
            );
            entry
                .file_name()
                .into_string()
                .expect("fixture directory name is valid UTF-8")
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn clippy_ban_paths() -> Vec<String> {
    let mut paths = CLIPPY_TOML
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let after_path = trimmed.strip_prefix("{ path = \"")?;
            Some(
                after_path
                    .split('"')
                    .next()
                    .expect("clippy ban path is closed")
                    .to_string(),
            )
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn clippy_ban_proof_paths() -> Vec<String> {
    let mut paths = CLIPPY_BAN_PROOFS
        .iter()
        .map(|proof| proof.clippy_path.to_string())
        .collect::<Vec<_>>();
    paths.sort();
    paths
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
fn registered_negative_fixtures_match_directory_census() {
    assert_eq!(
        registered_fixture_names(),
        fixture_directory_names(),
        "tests/negative-fixtures directories and FIXTURES registry must match exactly"
    );
}

#[test]
fn clippy_ban_entries_have_proving_negative_fixtures() {
    assert_eq!(
        clippy_ban_paths(),
        clippy_ban_proof_paths(),
        "every clippy.toml disallowed type/method entry must have a proving negative fixture"
    );

    let registered = registered_fixture_names();
    for proof in CLIPPY_BAN_PROOFS {
        assert!(
            registered
                .binary_search(&proof.proving_fixture.to_string())
                .is_ok(),
            "{} is proved by unregistered fixture {}",
            proof.clippy_path,
            proof.proving_fixture
        );
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
