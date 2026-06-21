#!/usr/bin/env python3
"""Reconcile cargo-mutants shard outputs by normalized identity set."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path


EXPECTED_OUTCOME_FIELDS = {
    "scenario",
    "summary",
    "log_path",
    "diff_path",
    "phase_results",
}
EXPECTED_MUTANT_FIELDS = {
    "diff",
    "file",
    "function",
    "genre",
    "name",
    "package",
    "replacement",
    "span",
}
SUMMARY_TO_TEXT = {
    "CaughtMutant": "caught.txt",
    "MissedMutant": "missed.txt",
    "Timeout": "timeout.txt",
    "Unviable": "unviable.txt",
}


class MergeError(Exception):
    pass


def load_json(path: Path):
    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except json.JSONDecodeError as exc:
        raise MergeError(f"malformed JSON in {path}: {exc}") from exc
    except OSError as exc:
        raise MergeError(f"cannot read {path}: {exc}") from exc


def read_env(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        raise MergeError(f"cannot read {path}: {exc}") from exc
    for line in lines:
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            raise MergeError(f"malformed env line in {path}: {line}")
        key, value = line.split("=", 1)
        values[key] = value
    return values


def normalize_text(value: object) -> str:
    return " ".join(str(value or "").split())


def function_name(mutant: dict) -> str:
    function = mutant.get("function")
    if not isinstance(function, dict):
        return ""
    return normalize_text(function.get("function_name"))


def validate_mutant(mutant: dict, path: Path) -> None:
    fields = set(mutant)
    unknown = fields - EXPECTED_MUTANT_FIELDS
    missing = EXPECTED_MUTANT_FIELDS - fields
    if unknown:
        raise MergeError(f"{path}: mutant has unknown fields: {sorted(unknown)}")
    if missing:
        raise MergeError(f"{path}: mutant missing fields: {sorted(missing)}")


def identity_key(mutant: dict, path: Path) -> str:
    validate_mutant(mutant, path)
    return "|".join(
        [
            normalize_text(mutant.get("file")),
            function_name(mutant),
            normalize_text(mutant.get("genre")),
            normalize_text(mutant.get("replacement")),
            normalize_text(mutant.get("diff")),
        ]
    )


def fingerprint(items) -> str:
    digest = hashlib.sha256()
    for item in sorted(items):
        digest.update(item.encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def load_mutant_list(path: Path) -> tuple[dict[str, dict], dict[str, str]]:
    data = load_json(path)
    if not isinstance(data, list):
        raise MergeError(f"{path}: expected a JSON list")
    by_key: dict[str, dict] = {}
    name_to_key: dict[str, str] = {}
    for mutant in data:
        if not isinstance(mutant, dict):
            raise MergeError(f"{path}: mutant entry is not an object")
        key = identity_key(mutant, path)
        if key in by_key:
            raise MergeError(f"{path}: duplicate normalized identity: {key}")
        by_key[key] = mutant
        name = normalize_text(mutant.get("name"))
        if name:
            name_to_key[name] = key
    return by_key, name_to_key


def load_canonical_names(path: Path) -> set[str]:
    try:
        return {
            normalize_text(line)
            for line in path.read_text(encoding="utf-8").splitlines()
            if normalize_text(line)
        }
    except OSError as exc:
        raise MergeError(f"cannot read {path}: {exc}") from exc


def outcome_name_and_identity(outcome: dict, path: Path, identity_mode: str) -> tuple[str, str, str]:
    fields = set(outcome)
    unknown = fields - EXPECTED_OUTCOME_FIELDS
    missing = EXPECTED_OUTCOME_FIELDS - fields
    if unknown:
        raise MergeError(f"{path}: outcome has unknown fields: {sorted(unknown)}")
    if missing:
        raise MergeError(f"{path}: outcome missing fields: {sorted(missing)}")
    scenario = outcome.get("scenario")
    if scenario == "Baseline":
        raise MergeError(f"{path}: baseline outcome must not appear in shard outcomes")
    if not isinstance(scenario, dict) or not isinstance(scenario.get("Mutant"), dict):
        raise MergeError(f"{path}: outcome scenario is not a Mutant object")
    mutant = scenario["Mutant"]
    name = normalize_text(mutant.get("name"))
    if not name:
        raise MergeError(f"{path}: outcome mutant has blank name")
    identity = identity_key(mutant, path) if identity_mode == "structured" else name
    summary = normalize_text(outcome.get("summary"))
    if summary not in SUMMARY_TO_TEXT:
        raise MergeError(f"{path}: unsupported outcome summary: {summary}")
    return name, identity, summary


def text_outcome_names(mutants_out: Path, filename: str) -> set[str]:
    path = mutants_out / filename
    if not path.exists():
        raise MergeError(f"missing outcome text file: {path}")
    return {
        normalize_text(line)
        for line in path.read_text(encoding="utf-8").splitlines()
        if normalize_text(line)
    }


def load_shard(shard_dir: Path, canonical_members: set[str], identity_mode: str) -> dict:
    shard_env = read_env(shard_dir / "shard.env")
    status = read_env(shard_dir / "status.env")
    if status.get("supervisor_result") != "child_exit_0":
        raise MergeError(
            f"{shard_dir}: non-normal supervisor_result={status.get('supervisor_result')}"
        )
    for required in [
        "shard_index",
        "shard_total",
        "commit",
        "config_fingerprint",
        "toolchain_fingerprint",
    ]:
        if required not in shard_env:
            raise MergeError(f"{shard_dir}: missing shard.env field {required}")

    assigned, _ = load_mutant_list(shard_dir / "assigned-mutants.json")
    outcomes_path = shard_dir / "mutants.out" / "outcomes.json"
    outcomes = load_json(outcomes_path)
    if not isinstance(outcomes, dict) or set(outcomes) - {
        "cargo_mutants_version",
        "caught",
        "end_time",
        "missed",
        "outcomes",
        "start_time",
        "success",
        "timeout",
        "total_mutants",
        "unviable",
    }:
        raise MergeError(f"{outcomes_path}: unexpected outcomes.json top-level schema")
    outcome_rows = outcomes.get("outcomes")
    if not isinstance(outcome_rows, list):
        raise MergeError(f"{outcomes_path}: outcomes field is not a list")

    outcome_keys: dict[str, str] = {}
    json_names_by_file = {filename: set() for filename in SUMMARY_TO_TEXT.values()}
    for outcome in outcome_rows:
        if not isinstance(outcome, dict):
            raise MergeError(f"{outcomes_path}: outcome entry is not an object")
        name, identity, summary = outcome_name_and_identity(outcome, outcomes_path, identity_mode)
        if identity in outcome_keys:
            raise MergeError(f"{shard_dir}: duplicate outcome identity: {identity}")
        if identity not in canonical_members:
            raise MergeError(f"{shard_dir}: outcome identity not in canonical set: {name}")
        outcome_keys[identity] = summary
        json_names_by_file[SUMMARY_TO_TEXT[summary]].add(name)

    mutants_out = shard_dir / "mutants.out"
    text_names_all: set[str] = set()
    for filename, json_names in json_names_by_file.items():
        text_names = text_outcome_names(mutants_out, filename)
        text_names_all.update(text_names)
        if text_names != json_names:
            raise MergeError(f"{shard_dir}: text/JSON outcome-set disagreement in {filename}")
    for name in text_names_all:
        if not name:
            raise MergeError(f"{shard_dir}: blank text outcome name")

    if identity_mode == "structured":
        assigned_keys = set(assigned)
    else:
        assigned_keys = {
            normalize_text(mutant.get("name"))
            for mutant in assigned.values()
            if normalize_text(mutant.get("name"))
        }
    final_keys = set(outcome_keys)
    if assigned_keys != final_keys:
        missing = sorted(assigned_keys - final_keys)
        extra = sorted(final_keys - assigned_keys)
        raise MergeError(
            f"{shard_dir}: assigned/outcome set mismatch missing={len(missing)} extra={len(extra)}"
        )

    counts = {
        "caught": len(json_names_by_file["caught.txt"]),
        "missed": len(json_names_by_file["missed.txt"]),
        "timeout": len(json_names_by_file["timeout.txt"]),
        "unviable": len(json_names_by_file["unviable.txt"]),
    }
    if sum(counts.values()) != len(final_keys):
        raise MergeError(f"{shard_dir}: outcome count equation does not match outcome set")

    return {
        "dir": str(shard_dir),
        "env": shard_env,
        "status": status,
        "assigned_keys": assigned_keys,
        "outcome_keys": final_keys,
        "counts": counts,
    }


def write_outputs(args, canonical_members: set[str], shards: list[dict], aggregate: dict) -> None:
    machine = {
        "final_sha": args.commit,
        "canonical_count": len(canonical_members),
        "canonical_fingerprint": fingerprint(canonical_members),
        "canonical_identity_mode": aggregate["identity_mode"],
        "shard_total": args.expected_shards,
        "shards": [
            {
                "index": shard["env"]["shard_index"],
                "dir": shard["dir"],
                "assigned_count": len(shard["assigned_keys"]),
                "assigned_fingerprint": fingerprint(shard["assigned_keys"]),
                "outcome_count": len(shard["outcome_keys"]),
                "outcome_fingerprint": fingerprint(shard["outcome_keys"]),
                "supervisor_result": shard["status"].get("supervisor_result"),
                "counts": shard["counts"],
            }
            for shard in shards
        ],
        "aggregate": aggregate,
        "merger": {
            "command": "tools/merge-mutation-shards.py",
            "version": "0045FIRPROCER-003",
        },
        "certification_disposition": "complete-union" if aggregate["pass"] else "failed",
    }
    args.out_json.parent.mkdir(parents=True, exist_ok=True)
    args.out_json.write_text(json.dumps(machine, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    lines = [
        "# 0045 FIRST-PROOF-CERT mutation completion manifest",
        "",
        f"- Final SHA: `{args.commit}`",
        f"- Canonical denominator count: {len(canonical_members)}",
        f"- Canonical fingerprint: `{fingerprint(canonical_members)}`",
        f"- Canonical identity mode: `{aggregate['identity_mode']}`",
        f"- Shards received: {len(shards)} / {args.expected_shards}",
        f"- Pairwise intersection empty: {aggregate['pairwise_disjoint']}",
        f"- Union equals canonical: {aggregate['union_equals_canonical']}",
        f"- Aggregate counts: caught={aggregate['caught']}, missed={aggregate['missed']}, timeout={aggregate['timeout']}, unviable={aggregate['unviable']}",
        f"- Certification disposition: {machine['certification_disposition']}",
        "",
        "| Shard | Assigned | Outcomes | Supervisor |",
        "|---|---:|---:|---|",
    ]
    for shard in shards:
        lines.append(
            f"| {shard['env']['shard_index']}/{shard['env']['shard_total']} "
            f"| {len(shard['assigned_keys'])} | {len(shard['outcome_keys'])} "
            f"| {shard['status'].get('supervisor_result')} |"
        )
    args.out_md.parent.mkdir(parents=True, exist_ok=True)
    args.out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")


def reconcile(args) -> None:
    if args.canonical:
        canonical, _canonical_name_to_key = load_mutant_list(args.canonical)
        canonical_members = set(canonical)
        identity_mode = "structured"
    else:
        canonical_members = load_canonical_names(args.canonical_list)
        identity_mode = "display-name"
    shards = [load_shard(path, canonical_members, identity_mode) for path in args.shard]

    seen_indices: set[int] = set()
    all_assigned: set[str] = set()
    aggregate_counts = {"caught": 0, "missed": 0, "timeout": 0, "unviable": 0}
    pairwise_disjoint = True
    for shard in shards:
        env = shard["env"]
        try:
            index = int(env["shard_index"])
            total = int(env["shard_total"])
        except ValueError as exc:
            raise MergeError(f"{shard['dir']}: shard index/total must be integers") from exc
        if total != args.expected_shards:
            raise MergeError(f"{shard['dir']}: mismatched shard denominator {total}")
        if index < 0 or index >= args.expected_shards:
            raise MergeError(f"{shard['dir']}: shard index out of range {index}")
        if index in seen_indices:
            raise MergeError(f"duplicate shard index: {index}")
        seen_indices.add(index)
        for field, expected in [
            ("commit", args.commit),
            ("config_fingerprint", args.config_fingerprint),
            ("toolchain_fingerprint", args.toolchain_fingerprint),
        ]:
            if env[field] != expected:
                raise MergeError(f"{shard['dir']}: mismatched {field}: {env[field]}")
        overlap = all_assigned & shard["assigned_keys"]
        if overlap:
            pairwise_disjoint = False
            raise MergeError(f"{shard['dir']}: overlapping identity count={len(overlap)}")
        all_assigned |= shard["assigned_keys"]
        for key, value in shard["counts"].items():
            aggregate_counts[key] += value

    expected_indices = set(range(args.expected_shards))
    if seen_indices != expected_indices:
        missing = sorted(expected_indices - seen_indices)
        raise MergeError(f"missing shard indices: {missing}")

    missing_canonical = canonical_members - all_assigned
    extra = all_assigned - canonical_members
    if missing_canonical or extra:
        raise MergeError(
            f"union does not equal canonical missing={len(missing_canonical)} extra={len(extra)}"
        )

    aggregate = {
        **aggregate_counts,
        "pairwise_disjoint": pairwise_disjoint,
        "union_equals_canonical": all_assigned == canonical_members,
        "count_equation": sum(aggregate_counts.values()) == len(canonical_members),
        "identity_mode": identity_mode,
        "pass": all_assigned == canonical_members
        and sum(aggregate_counts.values()) == len(canonical_members)
        and aggregate_counts["missed"] == 0
        and aggregate_counts["timeout"] == 0,
    }
    if aggregate_counts["missed"] or aggregate_counts["timeout"]:
        raise MergeError(
            f"survivor floor present missed={aggregate_counts['missed']} timeout={aggregate_counts['timeout']}"
        )
    if not aggregate["pass"]:
        raise MergeError("aggregate count equation failed")
    write_outputs(args, canonical_members, shards, aggregate)


def parse_args(argv: list[str]):
    parser = argparse.ArgumentParser()
    canonical = parser.add_mutually_exclusive_group(required=True)
    canonical.add_argument("--canonical", type=Path)
    canonical.add_argument("--canonical-list", type=Path)
    parser.add_argument("--expected-shards", type=int, required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--config-fingerprint", required=True)
    parser.add_argument("--toolchain-fingerprint", required=True)
    parser.add_argument("--out-md", type=Path, required=True)
    parser.add_argument("--out-json", type=Path, required=True)
    parser.add_argument("--shard", type=Path, action="append", required=True)
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    try:
        reconcile(args)
    except MergeError as exc:
        print(f"merge-mutation-shards: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
