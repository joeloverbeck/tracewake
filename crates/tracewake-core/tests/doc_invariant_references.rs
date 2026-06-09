use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "doc reference linter scans repository text; it is not simulation outcome code"
)]
fn doc_invariant_references_are_live() {
    let root = repo_root();
    let constitution_path = root.join("docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md");
    let constitution = std::fs::read_to_string(&constitution_path)
        .expect("constitutional invariants document is readable");
    let defined = defined_invariants(&constitution);
    assert!(
        !defined.is_empty(),
        "constitution must define at least one INV-### heading"
    );

    let mut dangling = Vec::new();
    for path in live_reference_paths(&root) {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()));
        for reference in invariant_references(&text) {
            if !defined.contains(&reference) {
                dangling.push(format!("{} references {reference}", path.display()));
            }
        }
    }

    assert!(
        dangling.is_empty(),
        "dangling INV-### references:\n{}",
        dangling.join("\n")
    );

    let mut coverage_errors = Vec::new();
    for path in live_spec_paths(&root) {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()));
        coverage_errors.extend(
            structured_invariant_coverage_errors(&text, &defined)
                .into_iter()
                .map(|error| format!("{}: {error}", path.display())),
        );
    }

    assert!(
        coverage_errors.is_empty(),
        "structured invariant coverage errors:\n{}",
        coverage_errors.join("\n")
    );

    let orphan_report = orphan_invariant_report(&defined, &root);
    assert!(
        orphan_report.contains("High-risk invariant review report"),
        "orphan/staleness report must be produced for reviewer signoff"
    );

    let synthetic = invariant_references(
        &[
            "INV-001",
            "P0-CERT",
            "SPINE-CERT",
            &["INV-", "999"].concat(),
        ]
        .join(" "),
    );
    assert!(synthetic.contains("INV-001"));
    assert!(synthetic.contains(&["INV-", "999"].concat()));
    assert!(
        !synthetic.contains("P0-CERT") && !synthetic.contains("SPINE-CERT"),
        "gate codes are opaque cross-references, not invariant references"
    );
}

#[test]
fn spec_finding_without_invariants_fails_linter() {
    let defined = BTreeSet::from(["INV-001".to_string()]);
    let errors = structured_invariant_coverage_errors(
        "### F-001 — Missing metadata\n\n**Layer:** core\n",
        &defined,
    );
    assert!(errors
        .iter()
        .any(|error| error.contains("F-001") && error.contains("Invariants")));
}

#[test]
fn spec_requirement_without_invariants_fails_linter() {
    let defined = BTreeSet::from(["INV-001".to_string()]);
    let errors = structured_invariant_coverage_errors(
        "### THIRD-AC-001 — Missing metadata\n\n**Responsible layer:** core\n",
        &defined,
    );
    assert!(errors
        .iter()
        .any(|error| error.contains("THIRD-AC-001") && error.contains("Enforces")));
}

#[test]
fn undefined_invariant_reference_fails_linter() {
    let defined = BTreeSet::from(["INV-001".to_string()]);
    let unknown = ["INV-", "999"].concat();
    let source = format!("### F-001 — Unknown invariant\n\n**Invariants:** `{unknown}`\n");
    let errors = structured_invariant_coverage_errors(&source, &defined);
    assert!(errors.iter().any(|error| error.contains(&unknown)));
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tracewake-core lives under crates/")
        .to_path_buf()
}

fn defined_invariants(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with("### INV-") || trimmed.starts_with("**INV-") {
                invariant_references(trimmed).into_iter().next()
            } else {
                None
            }
        })
        .collect()
}

fn invariant_references(source: &str) -> BTreeSet<String> {
    let bytes = source.as_bytes();
    let mut refs = BTreeSet::new();
    let mut index = 0;
    while index + 7 <= bytes.len() {
        if &bytes[index..index + 4] == b"INV-"
            && bytes[index + 4..index + 7].iter().all(u8::is_ascii_digit)
            && bytes
                .get(index + 7)
                .is_none_or(|byte| !byte.is_ascii_digit())
        {
            refs.insert(source[index..index + 7].to_string());
            index += 7;
        } else {
            index += 1;
        }
    }
    refs
}

fn structured_invariant_coverage_errors(source: &str, defined: &BTreeSet<String>) -> Vec<String> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut errors = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index].trim_start();
        let Some((heading, required_field)) = structured_heading_requirement(line) else {
            index += 1;
            continue;
        };

        let mut field_refs = BTreeSet::new();
        let mut cursor = index + 1;
        while cursor < lines.len() && !lines[cursor].trim_start().starts_with("### ") {
            let trimmed = lines[cursor].trim_start();
            if trimmed.starts_with(required_field) {
                field_refs.extend(invariant_references(trimmed));
            }
            cursor += 1;
        }

        if field_refs.is_empty() {
            errors.push(format!(
                "{heading} is missing {required_field} INV-### metadata"
            ));
        }
        for reference in field_refs {
            if !defined.contains(&reference) {
                errors.push(format!("{heading} references undefined {reference}"));
            }
        }
        index = cursor;
    }

    errors
}

fn structured_heading_requirement(line: &str) -> Option<(String, &'static str)> {
    if line.starts_with("### F-") {
        Some((
            line.trim_start_matches("### ").to_string(),
            "**Invariants:**",
        ))
    } else if line.starts_with("### THIRD-AC-") {
        Some((line.trim_start_matches("### ").to_string(), "**Enforces:**"))
    } else {
        None
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "doc reference linter scans repository text; it is not simulation outcome code"
)]
fn live_reference_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for relative in [
        "docs/4-specs",
        "specs",
        "tickets",
        "crates/tracewake-core/tests",
        "crates/tracewake-content/tests",
        "crates/tracewake-tui/tests",
    ] {
        collect_matching_files(&root.join(relative), &mut paths);
    }
    paths.sort();
    paths
}

#[allow(
    clippy::disallowed_methods,
    reason = "doc reference linter scans repository text; it is not simulation outcome code"
)]
fn live_spec_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    collect_matching_files(&root.join("specs"), &mut paths);
    paths.retain(|path| path.extension().and_then(|extension| extension.to_str()) == Some("md"));
    paths.sort();
    paths
}

#[allow(
    clippy::disallowed_methods,
    reason = "doc reference linter scans repository text; it is not simulation outcome code"
)]
fn orphan_invariant_report(defined: &BTreeSet<String>, root: &Path) -> String {
    let high_risk = [
        "INV-009", "INV-011", "INV-017", "INV-018", "INV-092", "INV-099", "INV-100", "INV-101",
        "INV-102", "INV-104", "INV-105", "INV-106",
    ];
    let mut referenced = BTreeSet::new();
    for path in live_reference_paths(root) {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()));
        referenced.extend(invariant_references(&text));
    }

    let mut report = String::from("High-risk invariant review report\n");
    for invariant in high_risk {
        if defined.contains(invariant) && !referenced.contains(invariant) {
            report.push_str(&format!("- {invariant} has no live references\n"));
        }
    }
    report
}

#[allow(
    clippy::disallowed_methods,
    reason = "doc reference linter scans repository text; it is not simulation outcome code"
)]
fn collect_matching_files(path: &Path, paths: &mut Vec<PathBuf>) {
    if !path.exists() {
        return;
    }
    if path.is_file() {
        if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("md" | "rs")
        ) {
            paths.push(path.to_path_buf());
        }
        return;
    }

    for entry in std::fs::read_dir(path)
        .unwrap_or_else(|error| panic!("{} directory is readable: {error}", path.display()))
    {
        let entry = entry.expect("directory entry is readable");
        collect_matching_files(&entry.path(), paths);
    }
}
