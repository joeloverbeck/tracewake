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
