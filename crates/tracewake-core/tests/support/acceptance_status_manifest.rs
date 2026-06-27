use std::collections::{BTreeMap, BTreeSet};

pub const STATUS_FENCE: &str = "```tracewake-acceptance-status";

const CLOSED_STATUSES: &[&str] = &[
    "closed",
    "open",
    "routed-forward",
    "pending-governance",
    "historical-only",
    "not-in-scope",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputedResult {
    Pass,
    NonPass,
}

impl ComputedResult {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "pass" => Ok(Self::Pass),
            "non-pass" => Ok(Self::NonPass),
            _ => Err(format!(
                "overall_result must be pass or non-pass, got {value:?}"
            )),
        }
    }
}

#[derive(Debug)]
pub struct AcceptanceStatusManifest {
    pub computed_result: ComputedResult,
    pub has_blocking_status: bool,
    pub has_mutation_survivors: bool,
}

#[derive(Debug)]
struct Finding {
    status: String,
    fields: BTreeMap<String, String>,
}

#[derive(Debug)]
struct Survivor {
    fields: BTreeMap<String, String>,
}

pub fn validate_status_manifest(text: &str) -> Result<AcceptanceStatusManifest, String> {
    let block = extract_status_block(text)?;
    let parsed = parse_status_block(block)?;
    let computed_result = compute_result(&parsed)?;
    if parsed.stated_result != computed_result {
        return Err(format!(
            "stated overall_result {:?} does not match computed result {:?}",
            parsed.stated_result, computed_result
        ));
    }
    Ok(AcceptanceStatusManifest {
        computed_result,
        has_blocking_status: parsed.has_blocking_status(),
        has_mutation_survivors: parsed.has_mutation_survivors(),
    })
}

pub fn extract_status_block(text: &str) -> Result<&str, String> {
    let Some((_, after_start)) = text.split_once(STATUS_FENCE) else {
        return Err("missing tracewake acceptance status block".to_string());
    };
    let after_start = after_start
        .strip_prefix('\n')
        .ok_or_else(|| "status block fence must be followed by a newline".to_string())?;
    let Some((block, _)) = after_start.split_once("\n```") else {
        return Err("unterminated tracewake acceptance status block".to_string());
    };
    if after_start.split(STATUS_FENCE).count() > 1 || text.matches(STATUS_FENCE).count() > 1 {
        return Err("multiple tracewake acceptance status blocks".to_string());
    }
    Ok(block)
}

#[derive(Debug)]
struct ParsedManifest {
    stated_result: ComputedResult,
    expected_findings: Vec<String>,
    branch_protection: String,
    governance_independence: String,
    mutation_status: String,
    mutation_survivors: String,
    findings: BTreeMap<String, Finding>,
    survivors: Vec<Survivor>,
}

impl ParsedManifest {
    fn has_blocking_status(&self) -> bool {
        self.branch_protection != "ruleset-transcript-current"
            || !governance_is_independent(&self.governance_independence)
            || self
                .findings
                .values()
                .any(|finding| finding.status != "closed")
            || compute_result(self).ok() == Some(ComputedResult::NonPass)
    }

    fn has_mutation_survivors(&self) -> bool {
        self.mutation_survivors != "none" || !self.survivors.is_empty()
    }
}

fn parse_status_block(block: &str) -> Result<ParsedManifest, String> {
    let mut scalars = BTreeMap::new();
    let mut findings = BTreeMap::new();
    let mut survivors = Vec::new();
    for raw_line in block.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            return Err(format!(
                "status line is missing key/value separator: {line}"
            ));
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "finding" => {
                let (label, status, fields) = parse_pipe_record(value)?;
                if !CLOSED_STATUSES.contains(&status.as_str()) {
                    return Err(format!("unknown status {status:?} for {label}"));
                }
                if findings
                    .insert(label.clone(), Finding { status, fields })
                    .is_some()
                {
                    return Err(format!("duplicate finding label: {label}"));
                }
            }
            "survivor" => {
                let (_label, _status, fields) = parse_pipe_record(value)?;
                survivors.push(Survivor { fields });
            }
            "overall_result"
            | "commit_under_test"
            | "source_acquisition"
            | "expected_findings"
            | "branch_protection"
            | "governance_independence"
            | "mutation_status"
            | "mutation_survivors" => {
                if scalars.insert(key.to_string(), value.to_string()).is_some() {
                    return Err(format!("duplicate scalar key: {key}"));
                }
            }
            _ => return Err(format!("unknown status key: {key}")),
        }
    }

    for required in [
        "overall_result",
        "commit_under_test",
        "source_acquisition",
        "expected_findings",
        "branch_protection",
        "governance_independence",
        "mutation_status",
        "mutation_survivors",
    ] {
        if !scalars.contains_key(required) {
            return Err(format!("missing required status key: {required}"));
        }
    }
    if scalars["commit_under_test"].is_empty() {
        return Err("commit_under_test must be non-empty".to_string());
    }
    if scalars["source_acquisition"].is_empty() {
        return Err("source_acquisition must be non-empty".to_string());
    }
    let expected_findings = parse_expected_findings(&scalars["expected_findings"])?;
    let expected_set: BTreeSet<_> = expected_findings.iter().map(String::as_str).collect();

    let present: BTreeSet<_> = findings.keys().map(String::as_str).collect();
    for required in &expected_findings {
        if !present.contains(required.as_str()) {
            return Err(format!("missing required finding: {required}"));
        }
    }
    for present in &present {
        if !expected_set.contains(present) {
            return Err(format!("unknown finding label: {present}"));
        }
    }

    Ok(ParsedManifest {
        stated_result: ComputedResult::parse(&scalars["overall_result"])?,
        expected_findings,
        branch_protection: scalars.remove("branch_protection").unwrap(),
        governance_independence: scalars.remove("governance_independence").unwrap(),
        mutation_status: scalars.remove("mutation_status").unwrap(),
        mutation_survivors: scalars.remove("mutation_survivors").unwrap(),
        findings,
        survivors,
    })
}

fn parse_expected_findings(value: &str) -> Result<Vec<String>, String> {
    let findings: Vec<_> = value
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(str::to_string)
        .collect();
    if findings.is_empty() {
        return Err("expected_findings must list at least one finding".to_string());
    }
    let unique: BTreeSet<_> = findings.iter().map(String::as_str).collect();
    if unique.len() != findings.len() {
        return Err("expected_findings contains duplicate labels".to_string());
    }
    Ok(findings)
}

fn parse_pipe_record(value: &str) -> Result<(String, String, BTreeMap<String, String>), String> {
    let mut parts = value.split('|').map(str::trim);
    let label = parts
        .next()
        .filter(|part| !part.is_empty())
        .ok_or_else(|| format!("record missing label: {value}"))?
        .to_string();
    let status = parts
        .next()
        .filter(|part| !part.is_empty())
        .ok_or_else(|| format!("record missing status: {value}"))?
        .to_string();
    let mut fields = BTreeMap::new();
    for part in parts {
        let Some((key, value)) = part.split_once('=') else {
            return Err(format!("record field missing '=': {part}"));
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            return Err(format!("record field has empty key or value: {part}"));
        }
        if fields.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("duplicate field {key:?} in record {label}"));
        }
    }
    Ok((label, status, fields))
}

fn compute_result(parsed: &ParsedManifest) -> Result<ComputedResult, String> {
    let mut pass = true;

    if parsed.branch_protection != "ruleset-transcript-current" {
        pass = false;
    }
    match parsed.governance_independence.as_str() {
        "independent-review" | "last-push-required-reviewer" => {}
        "pending-governance" | "status-checks-only" | "zero-approval" => pass = false,
        other => return Err(format!("unknown governance_independence: {other}")),
    }
    if parsed.expected_findings.is_empty() {
        pass = false;
    }

    for (label, finding) in &parsed.findings {
        match finding.status.as_str() {
            "closed" => {
                validate_closed_finding(label, finding)?;
            }
            "routed-forward" => {
                pass = false;
                require_forcing_fields(label, &finding.fields)?;
            }
            "open" | "pending-governance" | "historical-only" | "not-in-scope" => {
                pass = false;
            }
            _ => unreachable!("status set validated during parsing"),
        }
    }

    match parsed.mutation_status.as_str() {
        "killed" => {
            if parsed.mutation_survivors != "none" || !parsed.survivors.is_empty() {
                pass = false;
            }
        }
        "non-blocking-bounded-forcing" => {
            pass = false;
            if parsed.mutation_survivors == "none" || parsed.survivors.is_empty() {
                return Err(
                    "bounded-forcing mutation status requires explicit survivor rows".to_string(),
                );
            }
            for (index, survivor) in parsed.survivors.iter().enumerate() {
                require_forcing_map(&format!("survivor[{index}]"), &survivor.fields)?;
            }
        }
        "open" | "pending" | "timeout" => pass = false,
        other => return Err(format!("unknown mutation_status: {other}")),
    }

    Ok(if pass {
        ComputedResult::Pass
    } else {
        ComputedResult::NonPass
    })
}

fn governance_is_independent(value: &str) -> bool {
    matches!(value, "independent-review" | "last-push-required-reviewer")
}

fn validate_closed_finding(label: &str, finding: &Finding) -> Result<(), String> {
    for field in [
        "evidence_file",
        "evidence_test",
        "negative_file",
        "negative_test",
        "evidence_scope",
        "negative_scope",
    ] {
        require_field(label, finding, field)?;
    }
    for field in ["evidence_scope", "negative_scope"] {
        if finding.fields[field] != "current" {
            return Err(format!(
                "closed finding {label} has non-current {field}: {}",
                finding.fields[field]
            ));
        }
    }
    for field in [
        "evidence_file",
        "evidence_test",
        "negative_file",
        "negative_test",
    ] {
        let value = &finding.fields[field];
        if value.contains("display-only")
            || value.contains("self-authored")
            || value.contains("historical-only")
            || value.contains("stale")
        {
            return Err(format!(
                "closed finding {label} uses non-current or non-behavior evidence in {field}: {value}"
            ));
        }
    }
    Ok(())
}

fn require_field(label: &str, finding: &Finding, field: &str) -> Result<(), String> {
    if finding.fields.contains_key(field) {
        Ok(())
    } else {
        Err(format!("closed finding {label} missing {field}"))
    }
}

fn require_forcing_fields(label: &str, fields: &BTreeMap<String, String>) -> Result<(), String> {
    require_forcing_map(label, fields)
}

fn require_forcing_map(label: &str, fields: &BTreeMap<String, String>) -> Result<(), String> {
    for required in [
        "owner",
        "reason",
        "next_move",
        "max_epochs",
        "failing_check",
    ] {
        if !fields.contains_key(required) {
            return Err(format!(
                "routed-forward {label} missing forcing-function field {required}"
            ));
        }
    }
    Ok(())
}
