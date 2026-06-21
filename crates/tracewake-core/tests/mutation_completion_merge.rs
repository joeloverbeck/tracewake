#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "tooling integration test builds temporary merger fixtures and invokes the repository tool; it does not feed simulation outcomes"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const COMMIT: &str = "synthetic_commit_0045";
const CONFIG_FP: &str = "synthetic_config_fp";
const TOOLCHAIN_FP: &str = "synthetic_toolchain_fp";

#[test]
fn mutation_completion_merger_reconciles_sets_and_fails_closed() {
    let root = repo_root();
    let case = SyntheticCase::create("complete");
    let output = run_merger(&root, &case.dir, &case.shards);
    assert_success(&output, "complete synthetic shard set must reconcile");
    assert!(
        case.dir.join("completion.md").exists() && case.dir.join("completion.json").exists(),
        "successful merge must emit both manifest shapes"
    );

    let list_case = SyntheticCase::create("canonical_list");
    let list_output = run_merger_with_canonical_list(&root, &list_case.dir, &list_case.shards);
    assert_success(
        &list_output,
        "canonical list mode used by CI must reconcile display names",
    );

    let line_drift = SyntheticCase::create("line_column_drift").with_line_column_drift();
    assert_success(
        &run_merger(&root, &line_drift.dir, &line_drift.shards),
        "line/column-only drift must reconcile by normalized identity",
    );

    for (name, case, expected) in [
        (
            "missing shard",
            SyntheticCase::create("missing_shard").without_last_shard(),
            "missing shard indices",
        ),
        (
            "duplicate shard index",
            SyntheticCase::create("duplicate_index").with_duplicate_index(),
            "duplicate shard index",
        ),
        (
            "overlapping identity",
            SyntheticCase::create("overlap").with_overlap(),
            "overlapping identity",
        ),
        (
            "mismatched commit",
            SyntheticCase::create("bad_commit").with_env_value(0, "commit", "other"),
            "mismatched commit",
        ),
        (
            "mismatched config",
            SyntheticCase::create("bad_config").with_env_value(0, "config_fingerprint", "other"),
            "mismatched config_fingerprint",
        ),
        (
            "mismatched toolchain",
            SyntheticCase::create("bad_toolchain").with_env_value(
                0,
                "toolchain_fingerprint",
                "other",
            ),
            "mismatched toolchain_fingerprint",
        ),
        (
            "truncated JSON",
            SyntheticCase::create("truncated_json").with_truncated_outcomes(0),
            "malformed JSON",
        ),
        (
            "non-normal supervisor",
            SyntheticCase::create("bad_supervisor").with_status_value(
                0,
                "supervisor_result",
                "wrapper_wall_timeout",
            ),
            "non-normal supervisor_result",
        ),
        (
            "text JSON disagreement",
            SyntheticCase::create("text_json_disagreement").with_text_json_disagreement(0),
            "text/JSON outcome-set disagreement",
        ),
        (
            "survivor floor",
            SyntheticCase::create("survivor_floor").with_summary(0, 0, "MissedMutant"),
            "survivor floor present",
        ),
    ] {
        let output = run_merger(&root, &case.dir, &case.shards);
        assert!(
            !output.status.success(),
            "{name} must fail, stdout={}, stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{name} stderr must contain {expected:?}, got {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("core crate lives under crates/tracewake-core")
        .to_path_buf()
}

fn run_merger(root: &Path, case_dir: &Path, shards: &[PathBuf]) -> Output {
    run_merger_with_canonical_arg(
        root,
        case_dir,
        shards,
        "--canonical",
        case_dir.join("canonical-mutants.json"),
    )
}

fn run_merger_with_canonical_list(root: &Path, case_dir: &Path, shards: &[PathBuf]) -> Output {
    run_merger_with_canonical_arg(
        root,
        case_dir,
        shards,
        "--canonical-list",
        case_dir.join("canonical-list.txt"),
    )
}

fn run_merger_with_canonical_arg(
    root: &Path,
    case_dir: &Path,
    shards: &[PathBuf],
    canonical_arg: &str,
    canonical_path: PathBuf,
) -> Output {
    let mut command = Command::new("python3");
    command
        .current_dir(root)
        .arg(root.join("tools/merge-mutation-shards.py"))
        .arg(canonical_arg)
        .arg(canonical_path)
        .arg("--expected-shards")
        .arg("2")
        .arg("--commit")
        .arg(COMMIT)
        .arg("--config-fingerprint")
        .arg(CONFIG_FP)
        .arg("--toolchain-fingerprint")
        .arg(TOOLCHAIN_FP)
        .arg("--out-md")
        .arg(case_dir.join("completion.md"))
        .arg("--out-json")
        .arg(case_dir.join("completion.json"));
    for shard in shards {
        command.arg("--shard").arg(shard);
    }
    command.output().expect("merge tool should run")
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}: stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

struct SyntheticCase {
    dir: PathBuf,
    shards: Vec<PathBuf>,
}

impl SyntheticCase {
    fn create(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time is monotonic enough for test dirs")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("tracewake-{name}-{unique}"));
        fs::create_dir_all(&dir).expect("create synthetic case dir");
        let mutants = [
            mutant("alpha", 10),
            mutant("bravo", 20),
            mutant("charlie", 30),
        ];
        fs::write(
            dir.join("canonical-mutants.json"),
            format!("[{}]\n", mutants.join(",")),
        )
        .expect("write canonical mutants");
        let canonical_names = mutants
            .iter()
            .map(|mutant| extract_name(mutant))
            .collect::<Vec<_>>();
        write_lines(&dir.join("canonical-list.txt"), &canonical_names);

        let shard0 = dir.join("shard-0");
        let shard1 = dir.join("shard-1");
        write_shard(
            &shard0,
            0,
            &[mutants[0].clone(), mutants[1].clone()],
            &["CaughtMutant", "Unviable"],
        );
        write_shard(&shard1, 1, &[mutants[2].clone()], &["CaughtMutant"]);
        Self {
            dir,
            shards: vec![shard0, shard1],
        }
    }

    fn without_last_shard(mut self) -> Self {
        self.shards.pop();
        self
    }

    fn with_duplicate_index(self) -> Self {
        write_env_value(&self.shards[1].join("shard.env"), "shard_index", "0");
        self
    }

    fn with_overlap(self) -> Self {
        let alpha = mutant("alpha", 10);
        write_shard(
            &self.shards[1],
            1,
            &[alpha, mutant("charlie", 30)],
            &["CaughtMutant", "CaughtMutant"],
        );
        self
    }

    fn with_env_value(self, shard: usize, key: &str, value: &str) -> Self {
        write_env_value(&self.shards[shard].join("shard.env"), key, value);
        self
    }

    fn with_status_value(self, shard: usize, key: &str, value: &str) -> Self {
        write_env_value(&self.shards[shard].join("status.env"), key, value);
        self
    }

    fn with_truncated_outcomes(self, shard: usize) -> Self {
        fs::write(
            self.shards[shard].join("mutants.out/outcomes.json"),
            "{\"outcomes\": [",
        )
        .expect("write truncated outcomes");
        self
    }

    fn with_text_json_disagreement(self, shard: usize) -> Self {
        fs::write(
            self.shards[shard].join("mutants.out/caught.txt"),
            "not a shard JSON outcome\n",
        )
        .expect("write mismatched text outcome");
        self
    }

    fn with_summary(self, shard: usize, mutant_index: usize, summary: &str) -> Self {
        let assigned = if shard == 0 {
            vec![mutant("alpha", 10), mutant("bravo", 20)]
        } else {
            vec![mutant("charlie", 30)]
        };
        let mut summaries = vec!["CaughtMutant"; assigned.len()];
        summaries[mutant_index] = summary;
        write_shard(&self.shards[shard], shard, &assigned, &summaries);
        self
    }

    fn with_line_column_drift(self) -> Self {
        let drifted_alpha = mutant("alpha", 99);
        write_shard(
            &self.shards[0],
            0,
            &[drifted_alpha, mutant("bravo", 20)],
            &["CaughtMutant", "Unviable"],
        );
        self
    }
}

fn mutant(id: &str, line: u32) -> String {
    format!(
        r#"{{
  "diff": "--- crates/tracewake-core/src/{id}.rs\n+++ replace {id}\n@@ stable semantic diff for {id}",
  "file": "crates/tracewake-core/src/{id}.rs",
  "function": {{
    "function_name": "{id}_function",
    "return_type": "-> bool",
    "span": {{"start": {{"line": 1, "column": 1}}, "end": {{"line": 3, "column": 2}}}}
  }},
  "genre": "FnValue",
  "name": "crates/tracewake-core/src/{id}.rs:{line}:5: replace {id}_function -> bool with false",
  "package": "tracewake-core",
  "replacement": "false",
  "span": {{"start": {{"line": {line}, "column": 5}}, "end": {{"line": {line}, "column": 9}}}}
}}"#
    )
}

fn write_shard(path: &Path, index: usize, assigned: &[String], summaries: &[&str]) {
    fs::create_dir_all(path.join("mutants.out")).expect("create shard dirs");
    fs::write(
        path.join("assigned-mutants.json"),
        format!("[{}]\n", assigned.join(",")),
    )
    .expect("write assigned mutants");
    fs::write(
        path.join("shard.env"),
        format!(
            "shard_index={index}\nshard_total=2\ncommit={COMMIT}\nconfig_fingerprint={CONFIG_FP}\ntoolchain_fingerprint={TOOLCHAIN_FP}\n"
        ),
    )
    .expect("write shard env");
    fs::write(
        path.join("status.env"),
        "exit_status=0\nsupervisor_result=child_exit_0\n",
    )
    .expect("write status env");

    let mut caught = Vec::new();
    let mut missed = Vec::new();
    let mut timeout = Vec::new();
    let mut unviable = Vec::new();
    let outcomes = assigned
        .iter()
        .zip(summaries)
        .map(|(mutant, summary)| {
            let name = extract_name(mutant);
            match *summary {
                "CaughtMutant" => caught.push(name),
                "MissedMutant" => missed.push(name),
                "Timeout" => timeout.push(name),
                "Unviable" => unviable.push(name),
                other => panic!("unsupported synthetic summary {other}"),
            }
            format!(
                r#"{{
  "scenario": {{"Mutant": {mutant}}},
  "summary": "{summary}",
  "log_path": "log.txt",
  "diff_path": "diff.txt",
  "phase_results": []
}}"#
            )
        })
        .collect::<Vec<_>>();
    let out = path.join("mutants.out");
    fs::write(
        out.join("outcomes.json"),
        format!(
            r#"{{
  "cargo_mutants_version": "cargo-mutants 27.1.0",
  "caught": {},
  "end_time": "synthetic",
  "missed": {},
  "outcomes": [{}],
  "start_time": "synthetic",
  "success": true,
  "timeout": {},
  "total_mutants": {},
  "unviable": {}
}}"#,
            caught.len(),
            missed.len(),
            outcomes.join(","),
            timeout.len(),
            assigned.len(),
            unviable.len()
        ),
    )
    .expect("write outcomes");
    write_lines(&out.join("caught.txt"), &caught);
    write_lines(&out.join("missed.txt"), &missed);
    write_lines(&out.join("timeout.txt"), &timeout);
    write_lines(&out.join("unviable.txt"), &unviable);
}

fn extract_name(mutant_json: &str) -> String {
    let marker = "\"name\": \"";
    let start = mutant_json.find(marker).expect("synthetic mutant has name") + marker.len();
    let rest = &mutant_json[start..];
    let end = rest.find('"').expect("synthetic name terminates");
    rest[..end].to_string()
}

fn write_lines(path: &Path, lines: &[String]) {
    let mut text = lines.join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    fs::write(path, text).expect("write outcome text");
}

fn write_env_value(path: &Path, key: &str, value: &str) {
    let old = fs::read_to_string(path).expect("read env file");
    let mut wrote = false;
    let mut lines = Vec::new();
    for line in old.lines() {
        if line.starts_with(&format!("{key}=")) {
            lines.push(format!("{key}={value}"));
            wrote = true;
        } else {
            lines.push(line.to_string());
        }
    }
    if !wrote {
        lines.push(format!("{key}={value}"));
    }
    fs::write(path, format!("{}\n", lines.join("\n"))).expect("write env file");
}
