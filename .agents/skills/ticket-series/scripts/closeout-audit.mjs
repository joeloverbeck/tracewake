#!/usr/bin/env node

import { spawnSync } from "node:child_process";

function usage() {
  console.log(`Usage:
  node .agents/skills/ticket-series/scripts/closeout-audit.mjs --ticket-prefix PREFIX [options]

Options:
  --active-spec PATH       Live spec path expected to be gone.
  --archived-spec PATH     Archived spec path expected to exist.
  --active-report PATH     Live report path expected to be gone.
  --archived-report PATH   Archived report path expected to exist.
  --active-ticket PATH     Live ticket path expected to be gone. Repeatable.
  --archived-ticket PATH   Archived ticket path expected to exist. Repeatable.

The helper emits commands and results for mechanical closeout checks. Inspect
matches manually; historical archive prose can be valid.`);
}

const args = process.argv.slice(2);
const options = {
  activeTicket: [],
  archivedTicket: [],
};

for (let i = 0; i < args.length; i += 1) {
  const arg = args[i];
  const value = args[i + 1];

  if (arg === "--help" || arg === "-h") {
    usage();
    process.exit(0);
  }

  if (!arg.startsWith("--") || value === undefined || value.startsWith("--")) {
    console.error(`Invalid or missing value for ${arg}`);
    usage();
    process.exit(2);
  }

  i += 1;
  switch (arg) {
    case "--ticket-prefix":
      options.ticketPrefix = value;
      break;
    case "--active-spec":
      options.activeSpec = value;
      break;
    case "--archived-spec":
      options.archivedSpec = value;
      break;
    case "--active-report":
      options.activeReport = value;
      break;
    case "--archived-report":
      options.archivedReport = value;
      break;
    case "--active-ticket":
      options.activeTicket.push(value);
      break;
    case "--archived-ticket":
      options.archivedTicket.push(value);
      break;
    default:
      console.error(`Unknown option: ${arg}`);
      usage();
      process.exit(2);
  }
}

if (!options.ticketPrefix) {
  console.error("Missing required --ticket-prefix.");
  usage();
  process.exit(2);
}

function shellQuote(value) {
  return `'${String(value).replace(/'/g, "'\\''")}'`;
}

function regexEscape(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function run(label, command, argsForCommand, expectedExitCodes = [0]) {
  console.log(`\n## ${label}`);
  console.log([command, ...argsForCommand.map(shellQuote)].join(" "));
  const result = spawnSync(command, argsForCommand, {
    encoding: "utf8",
    maxBuffer: 10 * 1024 * 1024,
  });
  if (result.stdout) {
    process.stdout.write(result.stdout);
  }
  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  const status = result.status ?? 1;
  if (expectedExitCodes.includes(status)) {
    console.log(`status: ${status} (expected)`);
    return true;
  }
  console.log(`status: ${status} (unexpected)`);
  return false;
}

function capture(label, command, argsForCommand, expectedExitCodes = [0]) {
  console.log(`\n## ${label}`);
  console.log([command, ...argsForCommand.map(shellQuote)].join(" "));
  const result = spawnSync(command, argsForCommand, {
    encoding: "utf8",
    maxBuffer: 10 * 1024 * 1024,
  });
  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  const status = result.status ?? 1;
  const ok = expectedExitCodes.includes(status);
  console.log(`status: ${status} (${ok ? "expected" : "unexpected"})`);
  return {
    ok,
    stdout: result.stdout ?? "",
  };
}

function testPath(label, argsForTest) {
  return run(label, "test", argsForTest, [0]);
}

let ok = true;
ok = run("Git status", "git", ["status", "--short"], [0]) && ok;
ok = run("Matching active tickets", "bash", ["-lc", `rg --files tickets | rg ${shellQuote(options.ticketPrefix)}`], [1]) && ok;

const archivedTicketList = capture("Archived tickets by prefix", "rg", ["--files", "archive/tickets"], [0, 1]);
ok = archivedTicketList.ok && ok;
const archivedTicketsByPrefix = archivedTicketList.stdout
  .split(/\r?\n/)
  .filter((path) => path.includes(options.ticketPrefix))
  .sort();

if (archivedTicketsByPrefix.length === 0) {
  console.log(`No archived tickets matched prefix: ${options.ticketPrefix}`);
  ok = false;
} else {
  for (const path of archivedTicketsByPrefix) {
    console.log(path);
  }
}

const archivedTicketPaths = Array.from(new Set([
  ...archivedTicketsByPrefix,
  ...options.archivedTicket,
]));

for (const path of options.activeTicket) {
  ok = testPath(`Active ticket absent: ${path}`, ["!", "-e", path]) && ok;
}
for (const path of archivedTicketPaths) {
  ok = testPath(`Archived ticket present: ${path}`, ["-e", path]) && ok;
}
if (archivedTicketPaths.length > 0) {
  ok = run(
    "Archived ticket Outcome headings",
    "rg",
    ["--files-without-match", "^## Outcome", ...archivedTicketPaths],
    [1],
  ) && ok;
  ok = run(
    "Archived ticket Completed lines",
    "rg",
    ["--files-without-match", "^Completed: ", ...archivedTicketPaths],
    [1],
  ) && ok;
}
if (options.activeSpec) {
  ok = testPath(`Active spec absent: ${options.activeSpec}`, ["!", "-e", options.activeSpec]) && ok;
}
if (options.archivedSpec) {
  ok = testPath(`Archived spec present: ${options.archivedSpec}`, ["-e", options.archivedSpec]) && ok;
}
if (options.activeReport) {
  ok = testPath(`Active report absent: ${options.activeReport}`, ["!", "-e", options.activeReport]) && ok;
}
if (options.archivedReport) {
  ok = testPath(`Archived report present: ${options.archivedReport}`, ["-e", options.archivedReport]) && ok;
}

const stalePatterns = [
  options.activeSpec,
  options.activeReport,
  ...options.activeTicket,
].filter(Boolean).map((path) => `(?<!archive/)${regexEscape(path)}`);

if (stalePatterns.length > 0) {
  ok = run(
    "Stale live path sweep",
    "rg",
    ["-P", "-n", stalePatterns.join("|"), "docs", "reports", "specs", "tickets", "archive/reports", "archive/tickets", "archive/specs"],
    [1],
  ) && ok;
}

const archivePatterns = [
  options.archivedSpec,
  options.archivedReport,
  ...options.archivedTicket,
].filter(Boolean).map(regexEscape);

if (archivePatterns.length > 0) {
  ok = run(
    "Archive reference audit",
    "rg",
    ["-n", archivePatterns.join("|"), "docs", "reports", "archive"],
    [0, 1],
  ) && ok;
}

process.exit(ok ? 0 : 1);
