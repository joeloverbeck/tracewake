# Brief template & target-type reads

This file defines (A) the canonical anatomy of the emitted ChatGPT-Pro prompt and (B) the
research-target → load-bearing-reads map. The SKILL.md flow references both.

---

## A. Canonical brief anatomy

The emitted file `reports/<topic>-research-brief.md` is the *prompt the user pastes into
ChatGPT-Pro Session 2*. It is self-contained: Session 2 sees only this prompt plus the
uploaded manifest. Use these eight sections, in order. Scale each to the target; omit a
section only when genuinely N/A and say so.

### 1. Context

One or two sentences. Begin with the manifest pointer, then repo identity, then the **exact
fetch-baseline commit** Session 2 must read every file from (the verified repo HEAD per the
Step 6 baseline-commit rule — never a commit string copied from a report without confirming it
contains the §2 read-list):

> The uploaded manifest is the path inventory of the `joeloverbeck/tracewake` repo —
> a causality-first living-world simulation in Rust (event-sourced kernel, subjective
> epistemics, fallible institutions, TUI-first). Docs are layered authority:
> `0-foundation` → `1-architecture` → `2-execution` → `3-reference` → `4-specs`; earlier
> tiers govern later ones. Fetch every file from commit `<HEAD>` — the manifest reflects that
> tree. (If a referenced report cites a different "commit of record," note the divergence here
> and use the verified HEAD, not the report's string. If you then reassure Session 2 that the
> predecessor's findings carry forward *because the target files are unchanged between the
> report's commit and the new baseline*, that is a factual claim — verify it before writing it,
> per the Step 6 baseline-commit rule, never assert it from memory.)

If this brief **continues a prior one** (a multi-block campaign, or a follow-up to earlier
research), name the predecessor `reports/<...>-research-brief.md` and state what it already
delivered, so Session 2 treats this as a *delta* — not a cold start — and does not re-commission
completed work. When the campaign chains **several** prior sessions (e.g. a foundation determination,
then a routing memo, then an upstream-tier report), name each, state what each delivered, and identify
which is the **freshest / most-specific seed** for this pass — later predecessors often supersede
earlier ones as the primary input (a successor report's forward-routing appendix usually outranks the
original routing memo it descends from).

Distinguish a **lineage predecessor** from a **cross-line structural precedent**. A *lineage
predecessor* is a prior brief/spec in the *same* campaign line, one phase back (e.g. a gate's audit
brief that this remediation brief succeeds) — name it here in §1 to frame the *delta* and avoid
re-commissioning completed work. A *cross-line structural precedent* is a prior brief/spec from a
*sibling* line reused only as the **shape model** (e.g. a `P0-CERT` mutation-remediation spec cloned
as the template for a `SPINE-CERT` mutation-remediation brief) — it is not a delta seed, so list it
in §2 as a *structural model* read rather than naming it as a predecessor here. A single pass can
carry both at once; keep their roles separate so Session 2 reads the lineage predecessor for *what is
already done* and the structural precedent for *what shape to produce*.

**Second-pass / re-audit pass.** A brief may be a *follow-up pass over the same predecessor report*
(not a new campaign block) — re-auditing a surface that a prior pass's findings were supposed to
remediate. Two conventions apply. **(1) Naming:** attach the pass-ordinal suffix to the **base topic
slug** — base → `-second-pass` → `-third-pass` → … — on **both** the `brief` and the `report`. The
ordinal **replaces** the prior pass's ordinal; it does **not** append to the predecessor's
already-suffixed slug. So the third pass is `<topic>-research-brief-third-pass.md` (from the base
slug), **not** `<topic>-research-brief-second-pass-third-pass.md`. Examples:
`reports/<topic>-research-brief-second-pass.md` → `<topic>-research-report-second-pass.md` (second
pass); `reports/<topic>-research-brief-third-pass.md` → `<topic>-research-report-third-pass.md` (third
pass), per the established archive precedent
(`architecture-tier-alignment-research-{brief,report}-second-pass.md`,
`execution-tier-alignment-…-second-pass.md`, `reference-tier-alignment-research-report-second-pass.md`).
Do **not** mint a fresh slug (`…-v2`, `<topic>-second-…`) — it breaks the brief↔report 1:1 mapping.
**(2) Non-carry-forward posture:** when the intervening remediation has since merged into the fetch
baseline (so the predecessor's pinned commit is an *ancestor/parent* of HEAD and the §2 seams changed
wholesale), the predecessor's findings are the **pre-remediation baseline to re-verify, not carry
forward** — run the SKILL.md Step 5 equivalence check to establish that non-equivalence, and make
"assume nothing about the remediated surface's compliance" a settled intention.
**(2b) Source-equivalence re-audit (the empty-diff-since-the-remediation-keystone case).** The §(2)
diff is run against the *predecessor report's* pinned commit, which is pre-remediation — so it shows
wholesale change. But also diff against the **remediation spec's keystone implementation commit** (the
commit the prior pass's report was *implemented* as), because the source may be **byte-identical**
between that keystone and the fetch baseline when only archival / doc-truthing / ledger / merge commits
intervened. When `git diff --stat <remediation-keystone> <baseline>` over the §2 seams is **empty**, the
remediated source *is* the audited source: there is no drift to reconcile, and the remediation's own
**acceptance artifact describes the exact code at the baseline**. Frame this as an *advantage-and-trap*
re-audit — advantage: the acceptance's per-finding closure table maps onto the baseline line-for-line;
trap: this is precisely the recurrence being audited, so **re-verify the acceptance's own `pass` /
qualified-`pass` rows from the code rather than inheriting them**, and record genuinely-closed properties
as *present* while reporting any `pass` that the code does not bear out. State the dual-diff result in
§1 (wholesale vs. the report's commit; empty vs. the keystone) so Session 2 reads the posture correctly.
Which commit you diff against decides the posture: against the report's pre-remediation commit gauges
how much changed since the *audit*; against the keystone decides source-equivalence vs. drift. When the
acceptance artifact names **more than one** implementation commit — e.g. a capstone code / standing-campaign
commit and a separate commit-under-test, with only a governance- or doc-only fix between them — diff against
the **commit-under-test** (the broadest implementation state the acceptance certifies); if it and the
capstone-code commit diverge over the §2 seams, diff both and state which commit the source-equivalence claim
rests on (an empty diff against either is sufficient when they agree).
**Mixed delta (code identical, an adjacent layer changed by an *intervening* spec).** Distinct from the
"more than one implementation commit" case above (where the gov/doc fix sits *between* two commits the same
acceptance names): here a **separate later spec** changed an adjacent layer after the keystone, and that
changed layer is itself an in-scope live audit surface. When the keystone→baseline diff is **empty over the
primary (code) seams** but **non-empty over that adjacent layer** (e.g. governance/CI or a doctrine tier
amended by an intervening spec — not archival / doc-truthing / merge noise), the posture is **mixed**, not a
clean source-equivalence. Partition the §2 seams and hold both halves at once: re-verify the code as
source-equivalent (inherit nothing) **and** treat the changed layer as the **live delta to measure against
current HEAD**, not against the keystone. State both halves of the dual-diff in §1 (empty over code seams;
what the intervening spec changed), and encode the changed layer's audit boundary — including any dimension
that intervening spec *settled* (route it through §3 as a negative settled intention) — explicitly.
**(3) Evolving distinctive axis (multi-pass campaign).** A recurring re-audit campaign typically gives each
pass a *distinctive axis* beyond the standing re-verification (e.g. one pass adds a process-integrity /
verification-honesty contribution). When the immediately-prior pass's recommendation was **implemented** in
the interval, the newly-built mechanism — and the first acceptance/artifact produced under it — becomes a
**first-class audit target for this pass**: the recurrence's own fix is now on trial. Do not assume the prior
axis carries over unchanged; surface the evolving axis (and whether it is co-equal with the code re-audit) as
a bounded interview question (SKILL.md Step 4, path (b)), and carry the resolved emphasis into §3 as a settled
intention.

### 2. Read in full (authority order)

An explicit, tiered path list — every file Session 2 must read before producing — each
with a one-line reason it is load-bearing *for this target*. Built from Step 2 exploration.
Order strictly by authority tier. Example shape:

```
Read these in full, in this order:

docs/README.md — authority order and the layering rule.
docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md — INV-001…INV-NNN; every deliverable must satisfy these.
docs/0-foundation/<file> — <why this target touches it>.
docs/1-architecture/<file> — <subsystem contract this target depends on>.
docs/2-execution/<file> — <current sequencing/gates this target realigns or extends>.
reports/<report> — <prior finding-set this target builds on>.
archive/specs/<spec> — <completed work that established the current state>.
```

**Boundary-awareness reads.** When the target mandates reading a whole tier (the user says "read all of
`docs/0-foundation/*`"), or when a scoped audit must read adjacent docs *only* to know what is **out** of
scope, mark those entries as *boundary-awareness (read to bound scope, not a conformance target)* — distinct
from *primary (load-bearing)* entries. This keeps §2 useful at high file counts and stops Session 2 from
auditing or "correcting" code the scope intentions exclude. Call out the primary entries in each tier
explicitly; list the rest grouped, with the boundary-awareness purpose stated once per tier.

### 3. Settled intentions

The decisions the interview resolved — the heart of why Session 2 is *locked*. State each
as a committed decision, not an option. This section pre-empts every clarifying question
Session 2 might otherwise ask. Carry any early-exit gaps here as `assumption: <X>` lines so
they read as defaults the user can override, not as open questions.

When the interview settles a dimension **out of scope** — a decision the user has already made elsewhere
(e.g. a prior spec or an upstream session) — state it as a *negative* settled intention: name the
dimension, cite what settled it, and instruct Session 2 not to re-open it; reinforce it in §6 as a scope
guard. A locked Session 2 will otherwise re-raise exactly what a prior pass flagged (especially in a
recurring campaign, where the very thing one pass treated as a finding is what an unprompted re-audit
surfaces again), so the exclusion must be explicit, never implied by silence.

### 4. The task

A precise statement of what Session 2 must achieve — the goal behind the deliverable. One
tight paragraph. Name the target type (new spec / thorny fix / hardening / overhaul).

### 5. Exploration + online-research mandate

Authorize depth explicitly:

> Explore the repository as deeply as needed beyond the files listed above. Research online
> as deeply as needed — similar implementations, research papers, prior art — wherever it
> sharpens the deliverable. Cite sources for any external claim that shapes a decision.

### 6. Doctrine & constraints

Pointers Session 2 must honor:

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` is the constitution — every
  product-behavior decision must satisfy it; a genuine divergence requires amending an
  invariant first, never designing against it silently.
- Authority order: if execution conflicts with architecture or foundation, execution is
  wrong; if implementation is more convenient than the accepted gates, implementation is
  wrong.
- No backwards-compatibility shims or alias paths in new work.
- Anti-contamination: no simulation fact may be born from prose; preserve event-sourced
  causality, subjective epistemics, ordinary agents, possession parity, fallible
  institutions, validation/replay.

Trim to the constraints the target actually engages.

### 7. Deliverable specification

Exactly what Session 2 outputs — leave no ambiguity:

- each **downloadable markdown document**, by filename and whether it **replaces** an
  existing file or is **new**;
- for replacements, name the file being replaced and what must be preserved vs. changed;
- when the deliverable is a **numbered spec**: **fast path first** — if a same-line
  lineage-predecessor brief chose the immediately-prior staging number and recent git history
  (`git log --name-status --find-renames -n 20`) shows no intervening renumber/restart, continue
  N → N+1, **cite that predecessor**, and skip the epoch derivation below (this is the
  *Lineage-predecessor short-circuit*, detailed in full at the end of this bullet). Otherwise,
  derive its number and path from the live
  spec ledger + specs README (e.g. `docs/4-specs/SPEC_LEDGER.md`), not from the archive:
  prefer the next free *live* number (a realigned ledger may restart numbering, so an
  archived `0002` does not block a live `0002`) — **but first scan `archive/specs/` and
  `specs/` for the highest existing staging-filename number**: this repo stages specs
  sequentially across the whole campaign before archiving, so a live-ledger number can
  collide with an archived staging filename (e.g. a strict "next free live number" of
  `0002` would clash with an archived `0002_*` staging file). When the live-ledger number
  and the staging-sequence number diverge, do not assert the live number — continue the
  visible staging sequence and carry the choice as a labeled `assumption:` line, or surface
  it as a bounded interview question. **Identify the *current* staging epoch first:** a repo
  may have restarted numbering, so `archive/specs/` can hold two overlapping series (e.g. an
  older `0002…0008` and a newer `0002 → 0004` re-hardening epoch). Continue the **most recent
  contiguous series** — cross-checked against the recent `SPEC_LEDGER.md` entries — *not* the
  global-max filename; the next number is the one after the highest filename **in that current
  epoch** (e.g. `0005` after a `0002 → 0004` epoch, even though an older `0008` exists).
  Overlapping archived numbers from an earlier epoch are expected and do not block. **Before
  trusting either the global-max or a per-epoch number, check recent git history**
  (`git log --name-status --find-renames -n 20`, or inspect the latest commits) for a **rename or
  renumbering that unified or restarted the staging series**: a recent *unification* collapses the
  overlapping epochs into one contiguous series and makes the **global-max filename authoritative**
  (the opposite of the per-epoch rule just above — e.g. a "Renamed specs" commit that renumbers a
  `0002 → 0006` re-hardening epoch onto the tail of an existing `0002…0008` series, yielding a single
  `0002…0013` whose next number is `0014`); a recent *restart* does not. The git evidence governs over
  the filename pattern alone. **Lineage-predecessor short-circuit:** when a same-campaign lineage
  predecessor brief (one gate/phase back on the *same* line — e.g. a `SPINE-CERT` brief preceding an
  `EPI-CERT` brief on the certification ladder) already established the current staging epoch and
  chose the prior gate's number, continue the sequence (N → N+1) and **cite that predecessor** instead
  of re-deriving the full epoch/global-max/unification analysis — reserve that full derivation for when
  no such predecessor exists, or when recent git history shows an intervening renumber/restart since
  the predecessor was authored (re-run the recent-history check to confirm none did). Carry any
  residual placement ambiguity (`specs/` staging vs. final `docs/4-specs/`) as a labeled
  `assumption:` line rather than asserting it;
- the **locked / no-questions** instruction, verbatim intent:

> Produce the deliverables directly as downloadable markdown documents. Do not interview,
> do not ask clarifying questions — the requirements above are final. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed
> with the most faithful interpretation.

**Determination-plus-conditional targets.** When the research target is "decide whether X is
needed, and *if so* produce X" (common for hardening / anti-contamination passes), the deliverable
is contingent on a judgment Session 2 must make first. Do not leave the contingency implicit. The
brief must (a) instruct Session 2 to produce a clearly labeled, evidence-based **determination /
verdict** ("is a new spec warranted, and why"), and (b) state — as a settled intention resolved in
the interview — which of **three** modes governs the artifact: (i) **unconditionally** (one always-produced
document with the verdict embedded as a section); (ii) **only if the verdict is positive** (nothing authored
on a negative verdict); or (iii) **always produce, form follows the verdict** — one document is always
produced, but its *shape* depends on the verdict (e.g. a full spec if warranted, a standalone rationale
report if clean). Prefer (i) "always produce, with the verdict as a section" when the artifact's value
survives a negative verdict (e.g. it locks already-correct properties); choose (iii) when a negative verdict
still warrants a substantial evidence-complete document but in a *different form* than the spec; reserve
(ii) "produce only if positive" for when a negative verdict means there is genuinely nothing to author. **For a multi-pass / lineage campaign, the predecessor passes' established production mode governs this choice over the current invocation's surface phrasing** (see §A §1 *lineage predecessor*): when every prior pass always produced a verdict-embedded report, default to (i) even though the invocation reads conditionally ("decide whether X is needed, and *if so*…") — re-open the mode only if something material changed since the predecessor. The
§(a) determination/verdict is returned **regardless of mode** — under (ii), "nothing authored on a negative
verdict" refers to the spec/artifact *file* only; the reasoned verdict is still surfaced as Session 2's
response on a clean outcome (so the brief should instruct: produce the spec iff positive, otherwise return
the evidence-complete determination and author no file). **The conditional artifact need not be a
spec** — a hardening / anti-contamination pass commonly produces a *recommendation report* under
exactly this mode: always render the verdict, write the downloadable report iff the verdict is
positive, and on a clean verdict surface the determination inline with no file. That "inline on a
clean verdict" shape **is** mode (ii)'s "author no file," not a separate mode — do not mislabel it
(iii). For
mode (iii), the brief's deliverable spec (§7) must define **both** artifact shapes (a Branch A / Branch B
specification) so Session 2 commits to one without asking.

**Non-executable deliverables (audit / certification specs).** When the deliverable is an artifact
the external researcher **structurally cannot execute** — a certification or code-audit spec whose
authoritative result requires running `cargo test`, mutation testing, replay, or other live-code
execution Session 2 has no way to perform — do not commission an impossible "certified result."
Scope the deliverable to the **audit plan**: the seam inventory, each gate's required evidence, the
positive and adversarial fixture families, the failure-diagnostic-by-layer obligations, and the
acceptance-artifact shape — i.e. *what the implementing session will prove and how*, not a rendered
pass/fail. State this as a settled intention: (a) the spec specifies the audit, it does not render
the verdict; (b) Session 2 MAY include a clearly-labeled **preliminary static survey** (what reading
the code at the baseline suggests about likely gate satisfaction/risk) as *informative,
non-certifying* evidence — explicitly marked "preliminary, not certification" — or omit it; and (c)
authoritative pass/fail belongs to the implementing session that executes the gates. This pattern
recurs across a gate sequence (e.g. a `P0-CERT → SPINE-CERT → EPI-CERT → …` certification ladder),
so it is worth pinning explicitly rather than re-deriving each pass.

**Analysis / recommendation report (not a numbered spec).** When the deliverable is a consolidated
report of *recommended changes* rather than a ratified artifact — the recurring output
of a **downward-cascade realignment** (§B) and of doc-overhaul passes generally, **and also of a
cross-cutting hardening / anti-contamination pass whose ask is "what to change + where" rather than an
implementation spec** (the §B hardening row's reads still apply, but the deliverable is this report, not a
spec — do not assume a hardening target produces a `specs/` artifact; and when the pass audits
*recently-completed* work, the spec/acceptance's pinned implementation commit frequently lags HEAD —
a feature-branch tip versus merged `main` — so set the analysis baseline to verified HEAD, run the
Step 6 equivalence check, and elevate any material post-acceptance code delta to first-class audit
context in the brief rather than a provenance footnote) — say so explicitly,
and **do not** apply the numbered-spec numbering/ledger/epoch rules above (they are N/A; this is not a
`docs/4-specs/` artifact). Specify the report(s) by filename, **new** (not a replacement): one
consolidated `reports/<tier>-alignment-research-report.md`, **or** — when a single cascade pass spans
multiple tiers — one report per tier (`reports/<tier-a>-alignment-research-report.md`,
`reports/<tier-b>-…`). Resolve the count in the Step 4 interview; default to the per-tier split when the
tiers carry distinct authority boundaries (the campaign's architecture- and execution-tier passes each
produced their own report). **A live report slug may match an archived report of the same name**
from an earlier cascade epoch — this campaign reuses per-tier slugs (e.g.
`architecture-tier-alignment-research-report.md`) across epochs, disambiguated by `reports/` (live)
vs. `archive/reports/` (prior epoch). An archived namesake is **expected**: it is neither a
replacement nor a collision — confirm the live `reports/<slug>` path is absent (per the §2-completeness
check) and proceed. **A seed/source note may already occupy a `reports/` slug close to the brief's** — the
in-repo note this pass triages, often untracked — distinct from both the brief and the commissioned
report. It may sit on the **bare** topic stem (e.g. seed `reports/tui-parity.md` vs. brief
`reports/tui-parity-research-brief.md` vs. report `reports/tui-parity-research-report.md`), or carry its
own **descriptive suffix** off the stem (e.g. seed `reports/tui-human-wait-...-issue.md` vs. brief
`...-research-brief.md` vs. report `...-research-report.md` — the `-issue` / `-bug` / `-gap` seed need not
even share the chosen `<topic>` stem). Either way, confirm the `-research-brief` / `-research-report`
suffixes keep all three filenames distinct, and have the brief name the seed explicitly (e.g. "the
untracked `reports/<topic>-issue.md` is the seed, a different file") so Session 2 does not conflate the
report it writes with the seed it reads. **For a top-tier (`0-foundation`) pass** the report is an *amendment*
recommendation, not a realignment to anything upstream; prior precedent named it
`foundation-amendment-research-report.md`. Choosing `-alignment-` for cross-campaign per-tier symmetry
vs. `-amendment-` for precision is a labeled decision — surface it as a bounded interview question or
carry an `assumption:` line, do not silently default. For a **non-top-tier cascade pass**
(`1-architecture` / `2-execution` / `3-reference` / `4-specs` realigned to an upstream amendment),
`-alignment-` is the cross-campaign-symmetry default; reserve `-amendment-` for the top-tier case above. Direct Session 2 to
deliver **substance + home, not ratified text** — for each finding, *what doctrine the target doc must
own* (Session 2's own prose, at the right altitude for that tier) and *which file* it lands in (new
section / addition / correction) — explicitly **without** final paste-ready wording or invented
identifiers (`INV-###`, gate codes), which remain the repo's own reassess/amend process. The canonical
report shape this campaign reuses: a **disposition table** (one row per finding → target doc → verdict
→ one-line basis) → method/provenance ledger → per-finding sections (driver → current coverage →
tier-fit verdict → recommendation) → a **forward-routing appendix** (findings belonging to tiers below
the target, handed off to their later sessions — for a terminal lowest-tier pass this degenerates: say
so, and route only to owner/reassess decisions and future implementation specs; a **cross-cutting
hardening pass** spans *all* tiers rather than sitting at the bottom, so its appendix likewise routes only
to owner/reassess decisions and future implementation specs — not to a lower tier — but state the reason
as *cross-cutting*, not lowest-tier) → open questions →
references. Carry a labeled
`assumption:` line if the report's filename or placement is not pinned in the interview.

### 8. Self-check

A short acceptance checklist Session 2 runs against its own output before returning —
e.g. every replacement preserves the load-bearing content of the original; no new doctrine
weakens an upstream tier; every external claim is cited; the deliverable set matches §7
exactly; the §1 fetch-baseline commit contains every file named in the §2 read-in-full list.

---

## B. Target-type → load-bearing reads

A starting map for §2; always refine against Step 2 exploration. `02_CONSTITUTIONAL_INVARIANTS.md`
and `docs/README.md` are load-bearing for every type.

| Target type | Load-bearing tiers / files (beyond the two universal) |
|---|---|
| **new-spec** | `docs/1-architecture/**` for the touched subsystem; the relevant `docs/2-execution/**` phase + gates; `docs/4-specs/SPEC_LEDGER.md` and sibling specs; `tickets/README.md` if decomposition follows. **Certification/audit new-specs** (the `P0-CERT → SPINE-CERT → EPI-CERT → ORD-LIFE-CERT → FIRST-PROOF-CERT` ladder — see §7 *Non-executable deliverables*): also union in the **hardening** row's firewall/provenance reads (`docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`, `docs/1-architecture/03_*`, `docs/2-execution/04_*`) plus the validation/observability docs (`docs/1-architecture/13_*`, `docs/2-execution/10_*`), since a cert spec audits exactly those cross-cutting no-direct-dispatch / anti-contamination / evidence-honesty properties, not a single subsystem. **Distinguish a *subsystem* cert from a *capstone / integration* cert.** A subsystem cert (`SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`) audits one substrate's seams and decomposes into per-seam numbered audit points. The *terminal* ladder gate (`FIRST-PROOF-CERT`) is a **capstone**: it **composes already-`passed` predecessor gates** and certifies their *coherent integration*, not a fresh subsystem — so its primary interview axis is **composition posture** (consume + cite the predecessor `*-CERT passed` artifacts and require one integrated, single-unified-commit coherence re-proof, **vs** re-audit their internals), plus any **evidence bundle a predecessor explicitly deferred forward to it** (e.g. a consolidated temporal bundle routed to `FIRST-PROOF-CERT`), which becomes a first-class audit surface rather than seam decomposition. A cert-ladder **mutation-remediation** spec (a `Remediation`-posture deliverable, vs. the `Certification`-posture *audit* spec) shifts the load-bearing reads: its primary reads are the prior **remediation** specs (`archive/specs/` — the structural precedent whose shape it clones) plus the audit's **survivor-or-completion-blocker evidence** reports (the triage register — recording either a survivor floor *or* an incomplete / timed-out certifying campaign with zero survivors — plus the scoped-remediation acceptance artifact — the delta seed it remediates), per §A §1's lineage-vs-structural-precedent distinction and §7's "audit brief → mutation-remediation brief" sequence; the firewall/validation reads above remain secondary boundary context. Once `FIRST-PROOF-CERT` is passed the ladder's cert tail is exhausted: the next moves (`PHASE-4-ENTRY`, `SECOND-PROOF-ENTRY`) are **Expansion**-posture entry-gate specs, *not* certification audit/remediation pairs — classify those by the new-spec defaults, not the cert pattern. |
| **thorny-fix** | the architecture contract for the affected subsystem; the relevant code seams; any `reports/**` or `archive/reports/**` audit touching the defect; the execution doc whose gate the fix must still satisfy. |
| **hardening / anti-contamination** | `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` and the firewall/provenance architecture (`docs/1-architecture/03_*`); the subsystem's foundation + architecture docs; prior hardening specs in `archive/specs/**`; the validation/observability docs. |
| **foundational / doc-overhaul** | the full tier being overhauled plus every tier above it (authority flows downward); `docs/README.md` for the authority table; the staleness/downstream report(s) in `reports/**`; cross-references in lower tiers that the overhaul will invalidate. **Top-tier (`0-foundation`) overhauls:** there is no tier above, so read the tiers *below* as **boundary-awareness** reads (mark them per §2) to run the tier-fit test — what genuinely belongs in the constitution vs. a lower tier — and route lower-tier-bound findings *forward* to later per-tier sessions rather than amending them here. **Downward-cascade realignment** (a *lower* tier re-aligned to an *already-amended* upstream tier — e.g. realigning `1-architecture/` after a ratified `0-foundation/` amendment, often one of several sibling per-tier sessions a routing memo schedules): read the amended upstream tier(s) in full as the **immutable governing reference** (the authority you measure against — never an amendment target), the realigned tier as the **amendment target**, and the tiers *below* it as **forward-routing** boundary-awareness (mark per §2) — flag and hand off findings that belong further down (their own later per-tier session) rather than encoding them in the target tier. This is the mirror of the top-tier case: there you route findings *down* out of the constitution; here the governing tier is *above* and you route surplus findings *further down*. **Multi-driver tier pass:** a single tier pass may carry more than one driver — a freshly-landed upstream amendment to cascade *plus* themes a prior determination report already routed to this same tier (whose target docs may not overlap the amendment's own cross-references). §1 picks the *freshest seed*, but the freshest seed need not name every driver: enumerate each body of work and make each one's in/out-of-scope status an explicit settled intention resolved in the interview, rather than assuming a single cascade. **Bottom-of-stack (lowest tier) realignment:** when the target is the lowest tier(s) (e.g. `3-reference`/`4-specs`), there is no tier below to route to — the forward-routing appendix degenerates: state explicitly that nothing routes further down, and the only legitimate out-routes are owner/reassess decisions and future implementation specs. Pair with the §7 *analysis / recommendation report* deliverable archetype. |
| **other** | derive entirely from exploration; default to the universal two plus whatever the target names. |
