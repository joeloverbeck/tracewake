# Reference Index and Review Checklist

## Status

This is the compact operating index for Tracewake's reference layer. It exists for future AI coding, specification, documentation, fixture, schema, prompt, and review sessions that need fast guardrails without rereading the full foundation, architecture, and execution layers.

The reference layer is not a constitution, architecture contract, execution plan, roadmap, source-note bibliography, or issue tracker. It must stay small and low-duplication. When this file conflicts with the foundation, architecture, or execution layers, use the higher-authority layer and update this file only if the drift is recurring enough to justify reference-layer maintenance.

## Reference layer contents

- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — session-start source discipline, compact review checklist, deferred-term handling, and phase-exit review prompts.
- `01_DESIGN_RISK_REGISTER.md` — operational risk register for recurring failure modes that can look locally correct while undermining Tracewake.
- `02_GLOSSARY.md` — canonical terminology for docs, schemas, code, tests, prompts, review, TUI wording, and debug tooling.

Do not merge the risk register and glossary. They do different jobs: the risk register watches failure modes; the glossary controls names and boundaries.

Do not add reference files merely for symmetry. Add one only when a compact lookup or review aid is repeatedly needed across foundation, architecture, execution, implementation, fixtures, schemas, tests, and AI-session handoffs.

## Authority order

For design meaning, use this order:

1. Exact fetched source files from the target repository and target commit.
2. Foundation layer doctrine.
3. Architecture layer contracts and subsystem boundaries.
4. Execution layer phase gates and first-proof constraints.
5. Reference layer guardrails.
6. Current AI-session inference.
7. Prior chat, memory, repository labels, branch names, connector names, filenames, or unchecked local context.

The lower levels may summarize or operationalize the higher levels, but they do not override them. A reference-layer entry is a review aid, not permission to bypass a foundation invariant, architecture boundary, or execution gate.

## Source discipline gate for AI sessions

Before analyzing repository content or producing replacement docs, implementation guidance, tests, schemas, or review findings, a session must pass this gate.

1. Treat the user-supplied target repository, target commit, and uploaded manifest as the source request.
2. Treat the manifest only as a path inventory. It does not prove branch state, latest status, file freshness, repository identity, or semantic authority.
3. Fetch each needed file by exact URL containing repository owner, repository name, full target commit, and the manifest path.
4. Do not clone, use code search, use repository-scoped connector arguments, ask for the default branch, fetch by branch name, infer from tool namespace labels, or rely on snippets.
5. Record an evidence ledger before analysis. It must state repository, commit, freshness claim, manifest role, URL fetch method, fetched exact URLs, whether clone/search/branch/default metadata were used, and whether contamination or stale-source drift was observed.
6. Abort before analysis if a needed file cannot be fetched from an exact target-commit URL, if a requested path is absent from the uploaded manifest, if a fetch rewrites to another repository or branch, or if the analysis would depend on unfetched files, snippets, memory, or prior chat.

A stale statement inside a fetched file is document content, not repository authority. The exact fetch URL and evidence ledger decide provenance. If a fetched document names another commit, branch, old repository, obsolete scope, or prior session, flag stale-document drift and continue only if the source URL itself remains exact and uncontaminated.

## Compact doctrine review checklist

Reject or escalate a proposal, schema, fixture, test, prompt, UI surface, or code plan if any answer is unsafe.

### Source and provenance

- Was every repository fact fetched from the exact target commit and listed in the evidence ledger?
- Is any claim imported from branch state, repository metadata, code search, connector labels, old chats, old filenames, hidden scratchpad, or memory?
- Can each design assertion be traced to fetched source doctrine, deliberate inference, or explicitly named external research?
- Are stale source strings, old commit mentions, or incompatible prior-session assumptions being treated as authority?

### Authority and mutation

- Does every world-affecting ordinary-play change pass through command/proposal, validation, scheduling or resolution, event commit, and projection rebuild?
- Is any UI, prompt, LLM output, prose summary, debug view, story sifter, fixture script, or presentation layer creating authoritative simulation fact?
- Are debug injections explicit test/debug metadata rather than ordinary play?
- Are validation reports and why-not explanations generated from the shared pipeline rather than reimplemented in a client?

### Epistemics

- Are ground truth, observation, actor belief, memory, claim, speech act, trace, record, institutional fact, projection, debug truth, and player-facing surface wording kept distinct?
- Does embodied mode expose only actor-known information, uncertainty, and source-backed inferences?
- Can a false belief, stale record, wrong suspicion, or unsupported claim persist without being silently corrected from truth?
- Does possession change control and viewpoint only, not identity, memory, guilt, privilege, or knowledge?

### Ordinary-life play

- Can the scenario run without a human controller and still produce ordinary life, needs, routines, interruptions, and consequences?
- Does the currently possessed actor remain ordinary, with no privileged action, knowledge, scheduling, plot gravity, or protected role?
- Is the feature reachable and inspectable through the TUI or through stable view-model tests before it is considered accepted?
- Are no-human runs, possession parity, actor-known view filtering, replay, and debug explanation part of acceptance?

### Institutions and social machinery

- Do institutions act from reports, records, roles, procedures, resources, jurisdiction, norms, evidence thresholds, bias, and institutional knowledge rather than truth?
- Are violation, detection, suspicion, report, proof, sanction, notice, and record lifecycle separate?
- Does every record have provenance, artifact or storage context, author or issuer, claims, readers, amendments, and lifecycle events where applicable?
- Is any global legal meter, automatic proof, truth-reading notice, or record without reporter/issuer being smuggled in?

### Language, story, and presentation

- Are LLMs outside simulation authority and disabled or mocked for acceptance-critical operation?
- Are speech acts structured propositions with speaker stance, source, listeners, and validation?
- Is story sifting retrospective and non-mutating, with salience unable to pace or repair events?
- Does player-facing wording remain a surface over canonical state rather than ontology?

### Content, schemas, and fixtures

- Are content packages defining possibility space, not hidden outcomes?
- Do fixtures seed initial state and test chains without forcing ordinary-play result sequences?
- Are schema names aligned with the glossary, with stable IDs, versioning, provenance, source-backed beliefs, content-version compatibility, and rejection of forbidden core terms?
- Do random draws have replayable seeds or recorded draw labels sufficient for audit?

### Scale and deferral

- Is the first playable proof still small, local, TUI-first, replayable, inspectable, no-human-capable, and centered on missing expected property, belief divergence, records, and wrong suspicion?
- Are regional LOD, notices as central play, travel, route threats, bounties, companions, proof/payment flows, and broader world history deferred until the first proof is strong?
- If LOD or compaction is introduced, are active traces, beliefs, records, leads, suspicions, procedures, and causal ancestry preserved?

## Deferred term handling

Deferred terms are not banned forever. They are unsafe only when they harden before the proof that can support them.

- **Notice** is a canonical artifact, but notice-board-centric play belongs after the first proof unless used as a small local artifact with explicit author, claims, readers, lifecycle, and stale risk.
- **Bounty** is allowed later as a public contract, obligation, report, sanction, or payment procedure. It is not an objective menu item.
- **Companion** is allowed later as an autonomous actor, recruited actor, co-traveler, escort, contract party, or follower with needs, beliefs, refusal, and independent action. It is not a privileged party slot.
- **Travel** and **route threat** are allowed later as spatial actions and boundary processes with observation, evidence, uncertainty, route knowledge, resources, and consequences. They are not hidden pacing tools.
- **Proof** and **payment** are allowed as institutional or contractual procedure outcomes, not as completion flags or guaranteed rewards.
- **Regional history** and **LOD** are allowed only with explicit ancestry preservation and no human-proximity probability bias.

A deferred term entering first-proof core must justify itself as a small local artifact or process required by the first proof. Otherwise, move it back behind the second-proof boundary.

## Phase-exit review prompt

At every phase exit, answer these questions in writing or in the review artifact:

1. What would still work if the human controller never possessed anyone?
2. What does each relevant actor or institution know, believe, report, remember, suspect, or record, and from what source?
3. What event, validation report, random draw, trace, record, projection rebuild, or debug query proves the causal chain?
4. Which risk-register entries are active, newly escalated, or ready for retirement, and what evidence supports that status?

A phase cannot exit on demo success alone. It needs replay evidence, no-human evidence, actor-known view evidence, negative/failure evidence, TUI or view-model evidence, and causal explanation evidence appropriate to the phase.

## Maintenance rule

When a future session proposes reference-layer edits, keep only material that future AI sessions need at the moment of coding, spec writing, fixture authoring, schema review, or phase acceptance. Move doctrine back to foundation, contracts back to architecture, phase detail back to execution, research bibliography back to source-note files, and implementation tasks out of the reference layer entirely.
