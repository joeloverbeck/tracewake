# Spec 0001 — Missing Property Village Ontology and Fixture Contracts

This is the live first-proof village ontology and fixture-contract package for Tracewake's missing-property proof. It is not a new forward implementation spec, not a certification audit, and not a replacement for foundation, architecture, execution, or reference doctrine.

**Status:** realigned in place after the foundation / architecture / execution / reference overhaul.

**Admissibility posture:** `P0-CERT not applicable`. This document-only realignment does not certify code and does not perform the future `P0-CERT` audit named by `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`.

**Authority:** subordinate to `docs/0-foundation/`, `docs/1-architecture/`, `docs/2-execution/`, and `docs/3-reference/`. If this spec appears to define a gate, amend an invariant, replace architecture, or weaken execution, the higher tier wins and this spec must be corrected.

**Source discipline:** the target commit used for this realignment is audit/spec provenance only. It is not presented here as the current product baseline. The manifest is path inventory only; branch names, default-branch lookups, connector labels, and code-search snippets are not proof of target-commit content.

## 1. Conformance declaration

This spec conforms to the live spine by treating the first village as a proof surface for already-defined doctrine, not as a source of doctrine.

| Guardrail | How this spec uses it |
|---|---|
| `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | Preserves event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first playability, validation/replay, and the truth firewall. |
| `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | Keeps the missing-property first proof centered on ordinary village life, no-human progression, replay, TUI/debug separation, canonical regression seeds, and the missing-property proof. |
| `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` | Keeps the actor case of holder-known cognition explicit: truth may validate actor actions, but truth may not plan them. |
| `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Uses `holder-known context` as the system-wide term and `actor-known` for actor cognition, planning, possession, speech, and ordinary action cases. |
| `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` | Keeps the village ontology tied to settlement, places, rooms, doors, containers, property, food, sleep, work, access, custody, ownership, and ordinary local economy. |
| `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` | Treats fixtures and sketches as review artifacts that feed validation; it does not redefine acceptance semantics. |
| `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` | Cross-references the first-proof gates by code only: `EVENT`, `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `POSSESSION-PARITY`, `NO-HUMAN-ORDINARY-LIFE`, `MISSING-PROPERTY`, `VIEW-DEBUG-SPLIT`, `REPLAY`, and `FIXTURE-NEGATIVE`. |
| `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` | Keeps fixture and validation language anti-contaminated; no hidden truth, debug panel, script marker, or fixture outcome may become an actor input. |
| `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` | Keeps content as typed, provenance-bearing fixture data and forbids outcome-chain data. |
| `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` | Keeps fixture families reviewable as replayable golden scenarios and adversarial scenarios. |
| `docs/3-reference/02_GLOSSARY.md` | Uses the realigned vocabulary: holder-known context, actor-known context, truth firewall, context sealing, provenance class, debug truth, embodied view, fixture, golden scenario, and no-human proof. |

## 2. Scope and non-authority

This spec owns the first-proof village ontology and fixture contract beneath the live spine. It does not own:

- gate definitions or certification status;
- phase-entry authority;
- architecture of the event log, scheduler, planner, epistemics, institutions, TUI, replay, LOD, or data-loading systems;
- the future `P0-CERT` audit;
- archived spec certification;
- external research doctrine.

Archived specs `0005`–`0008` are historical evidence that drove the spine overhaul. They are not live authority and do not certify the current implementation.

## 3. First-proof identity

The first proof remains: a small ordinary village in which a physical item expected by one actor is missing from an expected place, and the simulation can progress, observe, report, suspect, remember, replay, and display that situation without omniscient scripts or quest logic.

The proof is successful only when the live execution gate set says it is successful. This spec provides the ontology and fixtures that the gate set can inspect.

The proof must preserve all of the following distinctions:

- item existence versus item display;
- item location versus item ownership;
- ownership versus custody;
- custody versus legal/social control;
- access right versus physical ability;
- physical absence versus theft;
- suspicion versus proof;
- actor-known belief versus debug truth;
- household memory versus institution record;
- ordinary routine pressure versus plot directive;
- report claim versus validated fact;
- no-human progression versus author-authored outcome.

## 4. Explicit non-goals

This spec does not authorize notice boards, road danger, bounty loops, companion mechanics, combat, culprit scripting, plot beats, authored clue chains, LLM dialogue mutation, procedural town generation, or graphical client work.

A missing-property situation is not a mystery quest. It is an ordinary-life, event-sourced, epistemically subjective world proof.

## 5. Actor roster contract

The first village contains actors who live ordinary lives. No actor may exist only as a quest giver, culprit token, witness token, reward dispenser, protagonist foil, tutorial puppet, or suspicion marker.

Actor IDs are stable content IDs. Display names are not unique identifiers.

| Actor ID | Display name | Household / residence | Ordinary role / work routine | Access rights | Relationships / trust edges | Debts, obligations, needs, motives, pressures | Initial beliefs and expectations | Initial memories | Speech/report capabilities | Institutional role | Valid debug possession target | First-proof relevance | Failure / ambiguity role |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `actor_tomas` | Tomas | `household_tomas`; sleeps in `house_tomas/bedroom_tomas` | Miller and household head. Opens mill, tracks grain, pays helpers, keeps a private strongbox at home. | May enter all rooms of `house_tomas`, mill workspaces, public square, reeve's public office area. May open `strongbox_tomas`. | High trust: `actor_elena`; procedural trust: `actor_anna`; guarded trust: `actor_elias`; low trust under debt gossip: `actor_mara`, `actor_rafi`. | Must pay a flour-tax share and wages; wants household security; dislikes public embarrassment. | Expects `coin_stack_01` in `strongbox_tomas` because of seeded prior direct observation `obs_tomas_coin_check_dminus1`. Believes only he and Elena may touch the strongbox. | Remembers counting coins the prior evening, closing the strongbox, and intending to pay Rafi after morning milling. | Can ask, answer, tell, report missing property, accuse only with actor-known basis, refuse private details. | None. Household owner/custodian. | Yes. | Primary absence discoverer and property owner. | Can misremember whether he paid Rafi if fixture variant enables ambiguity; may over-suspect Mara or Rafi without proof. |
| `actor_mara` | Mara | `household_mara`; sleeps in `house_mara/sleeping_area` | Seasonal mill hand and domestic worker. Runs errands, sometimes helps Nika and Iva. | Public places; mill worker areas while scheduled; not Tomas's bedroom or strongbox; may enter Tomas main room only by invitation or household errand. | Needs Anna's procedural fairness; owes Iva; wary of Elias; mixed history with Tomas; Lena is confidant. | Debt to `actor_iva`; food insecurity; obligation to help Lena; pressure to hide shame; possible motive to move/take coins in selected fixtures. | Believes Tomas keeps cash somewhere private, but does not know exact contents unless she observes or hears. Believes Iva will press debt. | Remembers Iva's demand, Tomas's house routine, any modeled observation of container access. | Can ask, answer sincerely/uncertainly, deny knowledge, withhold, lie/mislead from modeled pressure, gossip, report. | None. | Yes. | Possible mover/taker, possible innocent suspect, source of motive pressure. | Must not be hard-coded as culprit. Can move property for reasons other than theft if fixture variant chooses. |
| `actor_elena` | Elena | `household_tomas`; sleeps in `house_tomas/bedroom_tomas` or adjoining alcove | Household worker, manages food, errands, and domestic access. | House rooms except Tomas may restrict strongbox access; may open pantry; may enter public square, tavern, reeve's office. | High trust with Tomas; polite trust with Anna; neighborly trust with Petra; uncertain view of Mara. | Wants household order; may want to avoid scandal; may protect Tomas's dignity. | Believes Tomas keeps coins in strongbox generally, but may not know exact amount today unless she saw it. | Remembers household traffic and possibly a sound/door event without visual certainty. | Can answer, report household facts, refuse gossip, repeat uncertain observation. | Household member, not formal authority. | Yes. | Potential partial witness; expectation amplifier if Tomas asks her to check. | Her heard sound is not a culprit pointer; she may conflate time/order. |
| `actor_anna` | Anna | `household_anna` near `reeves_office` or office cot if on duty | Clerk. Receives reports, writes records, maintains incident ledger, schedules follow-up. | Public office, clerk counter, records room, incident ledger; no private homes without invitation/procedure. | Procedural trust from Tomas and Reeve; Elias cooperates; Mara fears institutional exposure. | Duty to record accurately; pressure to avoid overburdening Reeve; reputation for fairness. | Believes reports are claims, not truth. Believes missing-property intake requires reporter, item description, source belief, and uncertainty. | Remembers prior petty disputes and who has complained, but not truth of unresolved claims. | Can ask intake questions, acknowledge reports, refuse incomplete claims, record hearsay, amend ledger. | Clerk of `institution_reeves_office`. | Yes. | Turns speech into institutional record without truth leakage. | May delay/refuse report if insufficient actor-known basis or wrong venue. |
| `actor_elias` | Elias | `household_elias` or guard cot at office | Guard. Watches public spaces, escorts questioning, observes without omniscience. | Public spaces, office, records only with Anna/Reeve; private spaces only by permission/procedure. | Trusts Anna's records; respects Reeve; suspicious of debtors; friendly with Soren. | Duty to maintain order; bias toward visible motive; wants quick containment. | Believes reports require investigation; may suspect actors with motive/access, not truth. | Remembers seeing Mara near the lane or Rafi at the mill if fixtures seed that observation. | Can ask, watch, tell Anna/Reeve, deny knowledge, form/report suspicion with basis. | Guard of `institution_reeves_office`. | Yes. | Institutional suspicion vector and fallible watcher. | Can form wrong suspicion from partial evidence; cannot read event log truth. |
| `actor_reeve` | Reeve | `household_reeve`; office hours in `reeves_office/private_desk` | Local authority. Reviews records, authorizes procedures, mediates property disputes. | Office public and private areas; institutional records; private homes only through procedure/invitation. | Anna is trusted clerk; Elias is guard; Tomas has standing; Mara has lower status; Iva is influential. | Obligation to preserve order and avoid unjust sanction; political pressure from households. | Believes institutional proof requires record plus evidence/testimony, not mere accusation. | Remembers prior disputes and sanctions at a coarse level. | Can receive summary, question, authorize watch, refuse sanction, amend institutional status. | Local authority. | Yes. | Keeps institution fallible but bounded. | May over-weight status unless later checks contradict; cannot resolve automatically. |
| `actor_iva` | Iva | `household_iva`; `house_iva/main_room` | Creditor, household head, small trader. | Own home/storage, public square, tavern; no Tomas private rooms. | Mara owes Iva; Petra trusts Iva's memory; Anna treats Iva formally. | Wants repayment; may pressure Mara; wants reputation as reliable trader. | Believes Mara owes debt; may believe Tomas has money because millers handle payments. | Remembers debt deadline and any public argument. | Can ask for payment, gossip, deny, report unrelated debt, withhold. | None. | Yes. | Creates modeled motive pressure without being director. | Her debt claim can be true while theft suspicion is false. |
| `actor_nika` | Nika | `household_nika`; operates `tavern_nika` | Tavern/food-place keeper. Sees public traffic and hears gossip. | Tavern, storage, public square; no private home access. | Knows Petra, Rafi, Soren; friendly to Anna; wary of Elias. | Needs payment for food; wants custom; avoids being dragged into disputes. | Believes many claims are rumor until sourced. | Remembers who bought food, who seemed anxious, and who spoke loudly. | Can tell rumor as hearsay, answer uncertainly, refuse private customer details. | None. | Yes. | Rumor propagation and public observation node. | Can amplify wrong suspicion if hearsay loses source. |
| `actor_petra` | Petra | `household_petra`; window facing lane | Weaver/neighbor. Works at home, sees lane traffic. | Own home, public square, tavern; no private homes. | Trusts Elena; trades with Iva; limited trust of Mara. | Needs quiet work; wants neighbor order. | Believes she saw movement only if fixture seeds line-of-sight; must not infer more than observed. | Remembers a figure, time window, direction, sound, or color, depending on fixture visibility. | Can answer as observation, repeat uncertainty, gossip if pressured. | None. | Yes. | Partial visual witness. | Ambiguous sightline may point at wrong actor or no actor. |
| `actor_rafi` | Rafi | `household_rafi_lena`; sometimes mill bunk | Mill helper. Moves between mill, Tomas's house for errands, and tavern. | Mill worker areas; Tomas main room by errand; no bedroom/strongbox; public spaces. | Owed wages by Tomas; friend of Soren; known by Nika; neutral with Mara. | Wants payment; resents delay; may need money for food. | Believes Tomas owes him wages; may believe Tomas keeps coins, but not exact location unless observed/heard. | Remembers asking about pay and being told to wait. | Can ask for payment, deny, answer, gossip, accuse only with basis. | None. | Yes. | Plausible wrong suspect due motive/access-adjacent routine. | Must not be treated as guilty because owed wages. |
| `actor_soren` | Soren | `household_soren`; travels local lane only | Carrier/farmhand. Moves goods between mill, square, tavern, and homes. | Public lanes, mill yard, tavern, some thresholds by errand. | Friend of Rafi; casual with Nika; known to Elias. | Must complete deliveries; may avoid being questioned. | Believes he saw errands or bundles only if seeded; does not know contents. | Remembers route timing and load. | Can answer, deny knowledge, repeat route observations, refuse delay. | None. | Yes. | Movement noise and timing ambiguity. | His ordinary route can look suspicious without being theft. |
| `actor_lena` | Lena | `household_rafi_lena` or `household_mara` depending fixture variant | Seamstress/caretaker, confidant to Mara and Rafi. | Own home, public square, tavern; no private strongbox access. | Trust edge to Mara and Rafi; wary of Iva; polite with Anna. | Needs household food; may urge Mara to repay or tell truth. | Believes debt pressure exists if Mara told her; may not know whether any theft occurred. | Remembers Mara's emotional state and conversations. | Can answer, withhold confidences, repeat hearsay with source if modeled. | None. | Yes. | Separates motive knowledge from action knowledge. | Could mislead by omission to protect Mara without knowing truth. |
| `actor_corin` | Corin | `household_corin`; near office | Runner/assistant used only when useful. Carries messages between office and village. | Public spaces, office counter; no records room without Anna; private homes only at threshold. | Looks up to Anna/Elias; gossip source for Nika if careless. | Wants approval; may rush. | Believes messages should be delivered, not interpreted. | Remembers message delivery time and recipient. | Can carry structured notice/report summary, answer limited route questions, refuse private record access. | Optional office runner, not authority. | Yes, if included in fixture. | Supports report delay/amendment variants. | Must not become a quest courier or objective marker. |

## 6. Place, room, door, and container roster

The first village is deliberately small. It is not a generated town, not a region, and not a wilderness map. It is a concrete fixture graph that can be reviewed before implementation.

### 6.1 Place and room roster

| ID | Kind | Parent | Privacy | Jurisdiction | Connections | Visibility profile | Sound profile | Affordances | First-proof role |
|---|---|---|---|---|---|---|---|---|---|
| `village_core` | settlement scope | none | mixed | informal village custom; formal disputes to `reeves_office` | contains public square, homes, mill, tavern, office | high-level map only | ambient village sound | local movement, no-human advance marker | Defines first-proof spatial boundary. |
| `public_square` | public place | `village_core` | public | `institution_reeves_office` has public-order authority | lanes to office, mill, Tomas, Mara, tavern, supporting homes | broad visibility, many observers, poor privacy | conversations audible near actors; crowd noise | move, wait, ask, tell, gossip, go-to-authority | Claim propagation and public movement hub. |
| `reeves_office` | institution building | `village_core` | public front / restricted rear | `institution_reeves_office` | public square, clerk counter, records room, private desk | front room visible; records restricted | speech carries inside; outside muffled | report, receive report, write record, question, refuse | Converts report speech into institutional claims. |
| `office_counter` | room/desk zone | `reeves_office` | semi-public | `institution_reeves_office` | public front, records room door | Anna can see entrant; entrants see desk, not ledger contents | speech audible to nearby office actors | ask, report, acknowledge, refuse, wait | Intake surface for missing-property report. |
| `records_room` | restricted room | `reeves_office` | restricted | `institution_reeves_office` | office counter via `door_records_room` | hidden from public front when door closed | muffled | open ledger, amend record, search records | Holds incident ledger artifact. |
| `private_desk_reeve` | private work zone | `reeves_office` | restricted | Reeve / institution | office interior | seen only by office staff | private conversation if door/spacing allows | review, question, authorize | Post-`PHASE-4-ENTRY` procedure sketch only. |
| `house_tomas` | home | `village_core` | domestic private | `household_tomas`; village trespass norm | public lane, main room, bedroom, pantry | threshold visibility; interior private | door sounds and household voices | enter by permission, inspect, search with rights, take/place household goods | Home of expected property. |
| `tomas_main_room` | room | `house_tomas` | household private | `household_tomas` | front door, bedroom door, pantry | visible from doorway only when open/near | voices audible at threshold | wait, speak, household work | Non-strongbox access ambiguity. |
| `bedroom_tomas` | room | `house_tomas` | high private | `household_tomas` | main room via bedroom door | not visible from lane; strongbox visible only if actor enters/searches | door/footstep sounds possible | search, open strongbox if permitted/able | Location of `strongbox_tomas`. |
| `pantry_tomas` | room/storage nook | `house_tomas` | household private | `household_tomas` | main room | shelves visible when entered | quiet | open pantry, take/place food | Separates household food from private coins. |
| `house_mara` | home | `village_core` | domestic private | `household_mara` | public lane, main room, sleeping area, small cache | threshold only | quiet; door sounds | hide/place item, speak privately, search if permitted/procedure | Possible hiding/motive location. |
| `workplace_mill` | workplace | `village_core` | work-public during hours; restricted storage | customary mill rules; Tomas manages | mill yard, workroom, grain storage, lane | yard visible, workroom partial, storage restricted | loud machinery masks quiet movement | work, move, ask payment, place/take work items | Routine reason for actors to move. |
| `mill_yard` | outdoor work area | `workplace_mill` | work-public | Tomas work authority | square lane, mill workroom | visible from lane and square edge | loud intermittent | move goods, wait, observe | Plausible actor traffic without scripting. |
| `mill_workroom` | room | `workplace_mill` | work-restricted | Tomas work authority | yard, grain storage | visible to workers | machinery masks speech | work, ask, search allowed zones | Rafi/Mara ordinary work context. |
| `grain_storage` | restricted storage | `workplace_mill` | restricted | Tomas work authority | workroom door | low visibility | muffled | open/close, inspect grain bins | Access/control contrast with home strongbox. |
| `tavern_nika` | public food place | `village_core` | public front / private storage | `household_nika`; public norms | square, tavern storage | high public visibility | conversations can spread | buy food as future economy hook, gossip, ask, wait | Rumor and overheard claims. |
| `house_iva` | supporting home | `village_core` | domestic private | `household_iva` | square lane | threshold only | quiet | ask debt, speak, store goods | Motive-pressure source. |
| `house_petra` | supporting home | `village_core` | domestic private | `household_petra` | lane facing Tomas/Mara route | window line-of-sight to lane only | can hear carts/doors faintly | observe from window, answer | Ambiguous witness sightline. |
| `house_rafi_lena` | supporting home | `village_core` | domestic private | `household_rafi_lena` | lane to mill/square | threshold only | quiet | rest, private talk | Wrong-suspicion and wage pressure context. |
| `house_soren` | supporting home | `village_core` | domestic private | `household_soren` | lane to mill/square | threshold only | quiet | route start/end | Ordinary movement ambiguity. |
| `local_lane_mill` | tiny local route | `village_core` | public | public custom | connects square, mill, Tomas, Mara, Petra | seen from Petra window and square edge | carts/footsteps carry | move locally, observe, wait | Local movement only, not road-travel proof. |

### 6.2 Door roster

Doors are physical affordance objects, not quest locks. They mediate privacy, sound, line of sight, access, and trace opportunities.

| Door ID | Endpoints | Initial state | Sound / visibility occlusion | Access / privacy norm | Affordances | Failure cases |
|---|---|---|---|---|---|---|
| `door_tomas_front` | `public_lane_tomas` ↔ `tomas_main_room` | closed, unlocked while household active; may be latched | closed blocks clear sight; opening produces audible door sound nearby | visitors should knock or be invited; household members may enter | open, close, knock-as-ask variant, enter/exit | actor lacks permission; door stuck; actor refuses trespass; someone notices entry. |
| `door_tomas_bedroom` | `tomas_main_room` ↔ `bedroom_tomas` | closed, unlocked | closed blocks sight to strongbox; footstep/hinge sound can be heard nearby | high household privacy; non-household entry requires invitation/procedure | open, close, move through, search from threshold if open | actor cannot justify entry; blocked by occupant; later suspicion from privacy violation. |
| `door_pantry_tomas` | `tomas_main_room` ↔ `pantry_tomas` | open or ajar | low visibility occlusion | household food access, not private coin access | inspect, take/place food | confusing food custody with coin ownership is invalid. |
| `door_mara_front` | lane ↔ `house_mara/main_room` | closed, unlocked/latch | blocks sight; opening audible from inside | domestic privacy | open, close, enter by right/invitation | authority cannot enter without procedure; search without basis rejected. |
| `door_mill_front` | `mill_yard` ↔ `mill_workroom` | open during work, closed after hours | partial sound mask from mill | workers may enter on routine | open, close, move goods | after-hours access becomes suspicious but not proof. |
| `door_grain_storage` | `mill_workroom` ↔ `grain_storage` | closed, unlocked for workers or latched | blocks line of sight | restricted to mill work | open, close, inspect | actor lacks work reason; obstruction by load. |
| `door_office_front` | `public_square` ↔ `office_counter` | open during office hours | little occlusion when open; closed muffles | public report entry | enter, leave, wait, report | office closed; actor refuses public exposure; Anna unavailable. |
| `door_records_room` | `office_counter` ↔ `records_room` | closed, locked/restricted | blocks ledger visibility; muffles conversation | Anna/Reeve/Elias by role; public no access | open, close, lock/unlock as later implementation detail, enter | unauthorized access rejected; record not visible to reporter. |
| `door_tavern_front` | `public_square` ↔ `tavern_nika` | open during service | public visibility | public food place | enter, leave, ask, gossip | closed; Nika refuses disruptive dispute. |
| `door_house_petra_front` | lane ↔ `house_petra/main_room` | closed | no interior visibility; window separate | domestic privacy | open/close/answer door | Petra can witness lane from window without door access. |

### 6.3 Container roster

Containers hold physical items. They are not abstract inventory slots. Contents, expected contents, access permissions, trace profile, and search behavior matter.

| Container ID | Location | Initial contents | Initial state | Owner / custodian | Permitted users | Privacy | Trace profile | Expected contents by actor | Visibility and search behavior | Affordances | Failure cases |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `strongbox_tomas` | `bedroom_tomas` | `coin_stack_01`, optional `receipt_scrap_tomas` | closed, locked or latchable; exact lock mechanism deferred | legal owner and custodian: Tomas | Tomas; Elena only if fixture explicitly grants emergency access | high private | lid marks, dust disturbance, touch trace, missing-position trace, sound if opened | Tomas expects `coin_stack_01`; Elena expects coins only generally if she has source; others no valid expectation without source | contents visible only if actor can see/open/search; empty state is evidence only for actors with expectation | open/close, inspect, search, take/place item | no access; actor does not know to search; lock prevents action; opening creates trace; empty search without expectation is not evidence. |
| `pantry_tomas_shelf` | `pantry_tomas` | household food, jars | open shelves | `household_tomas`; custodian Elena/Tomas | household members | household private | missing food visible, low trace | Elena/Tomas expect food stock; visitors do not | visible when in pantry | take/place food | household food is shared custody, not Tomas's private coin property. |
| `basket_mara` | `house_mara/main_room` or carried by Mara | cloth, food, optional hidden item variant | open or covered | Mara | Mara, Lena by trust | private/personal | cloth disturbance, concealment trace | Mara knows contents; Lena may know if told/observed | visible only when open/search/carry posture reveals | carry, place, hide | public search requires procedure; concealed item not magically known. |
| `small_cache_mara` | `house_mara/sleeping_area` | personal scraps; optional `coin_stack_01` variant | closed, not formal lock | Mara | Mara; Lena maybe | high domestic private | dust/cloth movement, hiding trace | Mara knows if she placed item; others no expectation | search required, privacy norm strong | open/close/search/place/take | unauthorized search rejected; actor may choose to avoid privacy violation. |
| `mill_pay_box` | `mill_workroom` | low-value work tokens or empty | closed | Tomas as work custodian | Tomas; maybe Rafi for work deposits if explicit | work-restricted | visible workplace handling | Rafi may expect wages here only if told; Tomas knows current state | search from workroom if permitted | open/search/take/place work items | cannot be confused with `strongbox_tomas` unless actor has mistaken belief. |
| `grain_bin_mill` | `grain_storage` | grain sacks | open/closed bin | Tomas/workplace | workers during tasks | restricted workplace | footprints/dust | workers expect grain, not coins | visual bulk, search noisy | open/search/place | hiding coins here possible but trace/noise; not default. |
| `incident_ledger_box` | `records_room` | `incident_ledger_01` | closed, restricted | `institution_reeves_office`; custodian Anna/Reeve | Anna, Reeve; Elias with procedure | restricted institutional | page order, ink amendments, access marks | Anna expects current ledger present | ledger visible only to authorized actors | open/search/read/write | records are claims, not truth; public cannot inspect by default. |
| `tavern_till_nika` | `tavern_nika/private_storage` | Nika's takings | closed | Nika | Nika | private commercial | coin-count changes | Nika expects her own money | search requires permission/procedure | open/take/place | prevents global “coins are fungible balance” shortcut. |
| `chest_iva` | `house_iva/main_room` | debt records, goods | closed | Iva | Iva | private | paper/lock traces | Iva expects debt note | search private | open/read/search | debt record is claim source, not theft proof. |
| `satchel_soren` | carried by Soren | delivery goods | carried/closed | Soren or employer depending goods | Soren | personal/work | carry posture, contents disturbed | Soren knows load, others not | search requires direct inspection | carry/open/search | ordinary carry must not be treated as hidden culprit pointer. |

## 7. Physical item/value-token contract

The first-proof valuable item is:

`coin_stack_01`

It is a physical, custody-tracked value token. It is not an abstract balance and must not be collapsed into an economy total.

| Field | Contract |
|---|---|
| ID | `coin_stack_01` |
| Kind | physical value token: small stack/pouch of coins, count may be known exactly by Tomas and unknown/approximate to others |
| Legal/social owner | Tomas / `household_tomas` depending later legal model; for this first-proof contract, Tomas is recognized owner and primary claimant |
| Initial possessor | `strongbox_tomas` as containing physical object; Tomas is not physically holding it at start |
| Initial custodian | Tomas as strongbox custodian |
| Expected location | `strongbox_tomas`, sourced from `obs_tomas_coin_check_dminus1` and/or prior seeded accounting memory |
| Permitted users | Tomas. Elena only if a fixture variant explicitly grants emergency household access. No general worker/public permission. |
| Value significance | Enough to matter for wages, debt, household security, and reportability; not so large that it creates a regional/criminal-case proof. |
| Portability/carry constraints | Small enough for any actor to carry/conceal in pocket, basket, satchel, or hand; movement should create possible traces depending container and route. |
| Visibility | Visible only when line-of-sight and container/clothing/search conditions allow. Absence visible only after expected-location inspection/search by an actor with a relevant expectation. |
| Transfer/movement actions | `take_item`, `place_item`, `hide_or_move_item`, possibly `item_relocated_by_process` for setup/prehistory only. |
| Observation/search behavior | Presence may create `item_observed_present`; expected absence may create `expected_item_observed_absent`; actor without expectation sees empty strongbox only as an empty container unless another source makes absence salient. |
| Trace possibilities | disturbed strongbox dust, lid sound, pocket/basket bulge, dropped coin, witness of actor near room, record of prior count, inconsistent speech. Traces are not truth and may be absent or ambiguous. |
| Record/expectation references | Tomas's report may reference `coin_stack_01` description and expected location. Anna's ledger records the claim, source, uncertainty, and reporter. |
| How absence becomes evidence | Only through expectation plus observation/search: prior belief `coin_stack_01 in strongbox_tomas` + search of strongbox + observed absence + no immediate accepted explanation. |

Other coin-like objects may exist only if they do not blur the central custody proof. If later fixtures add `tavern_till_nika` or wages, their item IDs must remain distinct from `coin_stack_01` unless a modeled transfer event creates identity continuity.

## 8. Ownership, custody, access, control, proof, and belief model

Tracewake must keep the following concepts separate.

| Concept | Meaning | Example: Tomas's coins | Example: household food | Example: incident ledger | Example: stolen/hidden property |
|---|---|---|---|---|---|
| Legal/social ownership | Who the village recognizes as rightful owner or claimant under norms. | Tomas owns `coin_stack_01`. | Household food may be owned by `household_tomas`, with Elena/Tomas as ordinary users. | `institution_reeves_office` owns/controls the ledger artifact. | Ownership remains Tomas's even if Mara physically hides the coins. |
| Physical possession | Where the thing physically is, including carried-by or contained-in. | Initially inside `strongbox_tomas`; later may be in Mara's basket or cache. | Food on pantry shelf or carried by Elena. | Ledger physically in `incident_ledger_box` or on Anna's desk. | Hidden coins in `small_cache_mara` are possessed by that container, not by the truth system. |
| Custody | Who has accepted practical responsibility for care/access. | Tomas is custodian of the strongbox and coins. | Elena may be custodian of pantry stock. | Anna is day-to-day custodian of ledger; Reeve has authority. | Mara may have de facto custody while hiding property, but not legitimate ownership. |
| Access permission | Who is allowed to enter/open/use under social or institutional rules. | Tomas may open strongbox; Elena only if explicit; Mara/Rafi not permitted. | Household members may take food; visitors need permission. | Anna/Reeve may access ledger; public may report but not browse. | Authority search requires procedure; private search without basis may violate `trespass_privacy`. |
| Physical control | Who can physically act on the object now, regardless of permission. | A trespassing actor may be able to open/take if lock/state permits. | A hungry visitor may physically take food if unobserved. | Anna can write; an unauthorized intruder might physically damage but not legitimately amend. | A thief can move item but cannot make the institution know truth. |
| Proof/provenance | Evidence chain that justifies a claim, suspicion, record, or sanction. | Prior count + search absence + trace + testimony may support report. | Missing food plus household expectation may support minor household suspicion. | Ledger amendment has author/time/source speech references. | Recovery in cache plus access evidence may prove possession but not automatically motive or theft. |
| Belief about state | Actor's held proposition, with source, confidence, and time. | Tomas believes coins should be in strongbox; Anna believes Tomas claims this. | Elena believes pantry should have bread because she stocked it. | Reeve believes the ledger says a report was filed. | Elias may suspect Mara based on motive and location, even if wrong. |
| Institutional recognition | What an institution has accepted as record/procedure/status. | Report accepted as incident record, not proof of theft. | Household complaint may be outside formal incident threshold. | Ledger states claims and amendments. | A sanction requires institutional procedure, not debug truth. |

Forbidden collapse examples:

- Do not treat “Tomas owns coins” as “Tomas knows where coins are.”
- Do not treat “Mara possesses coins” as “Anna knows Mara did it.”
- Do not treat “ledger says Tomas reported theft” as “theft is proven.”
- Do not treat “actor has physical access” as “actor has permission.”
- Do not treat “empty container” as evidence for actors with no prior expectation.

## 9. Action vocabulary

Actions are proposed and adjudicated through the same action pipeline for human-controlled and AI-controlled actors. Human possession changes which actor proposes actions; it does not create special action semantics.

The spec names action families and contracts. It does not choose final Rust types, crate layout, content syntax, TUI library, or serialization.

### 9.1 Common action fields

Every ordinary action family must carry or derive:

- actor requirements;
- physical preconditions;
- belief/knowledge preconditions;
- social/normative preconditions;
- resource/time/duration;
- possible rejection;
- possible failure;
- events emitted on success/failure;
- traces/observations generated;
- TUI affordance visibility;
- debug explanation hook;
- same-action rule for human-controlled and AI-controlled actors.

Debug possession switching is a non-diegetic debug operation, not an ordinary actor action.

### 9.2 Primitive action families

| Action family | Actor requirements | Physical preconditions | Belief / knowledge preconditions | Social / normative preconditions | Resource / duration | Rejection / failure | Events emitted | Traces / observations | TUI affordance visibility | Debug hook | Parity rule |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `debug_attach_or_switch_possession` | Human/debug controller only; not an in-world actor action | Target actor exists and is a valid debug possession target | none | debug mode only | instant debug step | rejected if target invalid or fixture forbids debug attach | `debug_possession_changed` or debug-only marker; no world mutation | no diegetic trace | visible only in debug UI | explains current possessed actor, forbidden state mutation | AI never invokes; it is outside ordinary play and cannot prove anything. |
| `inspect_look` | actor conscious/present | actor has line of sight or sensory channel | none; may be curiosity/routine | privacy may make looking intrusive | short | rejected if no sensory path; failure if darkness/occlusion/noise | `observation_created`, possibly `item_observed_present` | sight/sound observations | shown when actor can plausibly inspect | why visible/not visible | same action for human/AI; filtered by actor senses. |
| `move_locally` | actor can move | connected endpoints; passable door/path | actor knows or can perceive route, or follows routine | trespass/privacy may block private destination | variable local travel | rejected if no route or forbidden; failure if interrupted/delayed | `actor_moved`, `intention_plan_delayed` if delayed | footstep/witness/route traces | visible as exits/routes known to actor | route cause, blockers, witnesses | same movement semantics for all actors. |
| `open_or_close_door` | actor adjacent to door | door exists; not blocked; lock state permits or actor has key/tool if modeled | actor perceives door and wants passage/privacy | access norms may forbid | short | rejected if locked/no permission; failure if stuck/noisy | `door_opened` / `door_closed`, or `action_rejected`/`action_failed` | sound, visibility change, witness | visible when adjacent and actor can interact | lock/norm/noise explanation | same for human/AI. |
| `open_or_close_container` | actor adjacent/holding container | container reachable; state permits | actor knows/perceives container; may require search intention | ownership/privacy norms apply | short | rejected if locked/forbidden; failure if jammed/noisy | `container_opened` / `container_closed`, `container_accessed` | touch/dust/noise/witness | visible for perceived reachable containers | access/norm/lock chain | same for human/AI. |
| `inspect_or_search_place_container` | actor present; enough time | place/container accessible; search space exists | either expectation, instruction, curiosity, routine, or procedure; absence evidence requires expectation/source | privacy/procedure may block | short to medium | rejected if no access/basis; failure if cursory search misses hidden item | `search_performed`, `observation_created`, `expected_item_observed_absent` or `item_observed_present` | disturbance traces; search witnessed | visible as inspect/search when actor has access and reason; intrusive actions marked | explains why empty mattered or did not | same for human/AI. |
| `take_item` | actor able to carry/reach item | item present and accessible; container open if needed | actor perceives item or searches successfully; knows/decides target | permission required unless actor chooses violation | short | rejected if item inaccessible/too heavy/forbidden; failure if interrupted | `item_transferred`, `container_accessed`, `action_rejected/failed` | missing-position trace, carry trace, witness | visible only if actor sees/knows item and can reach | ownership/access/control distinction | same; AI cannot take unseen item. |
| `place_item` | actor possesses item | destination reachable and can contain item | actor knows destination or is searching/hiding | destination permission/privacy applies | short | rejected if no capacity/access; failure if dropped/noisy | `item_relocated`, `container_accessed` | placement trace, possible witness | visible when carried item and destination affordance known | where item moved and why | same. |
| `hide_or_move_item` | actor possesses or can access item | concealment destination reachable | actor intends concealment, temporary storage, repayment avoidance, safekeeping, or theft | may violate ownership/privacy | short/medium | rejected if no concealment; failure if trace/witness/noisy | `item_relocated`, `trace_created`, `belief_updated` if actor interprets success | concealment trace; later search difficulty | visible as hide/move only when actor has motive and item/control | motive vs action vs proof | same; no scripted culprit shortcut. |
| `wait` | actor alive/present | location permits waiting | routine, uncertainty, social timing | loitering may be suspicious in private/restricted spaces | chosen duration | rejected if location forces movement; failure if interrupted | `intention_plan_delayed`, maybe `actor_waited` if later named | observation opportunities | visible generally, with privacy warnings | time advance and interruptions | same. |
| `ask` | speaker and listener/audience present or channel exists | can communicate | speaker has question proposition or information need | politeness/status/privacy may affect | short | listener refuses/unavailable/mishears | `speech_act_committed`, possible `belief_updated` | heard-by observations | visible for reachable actors and actor-known topics | question source and topic | same. |
| `answer` | listener becomes speaker | prior question or conversational context | speaker has belief, memory, uncertainty, or denial | may be obligated, private, or cautious | short | cannot answer; refuses; answers uncertainly | `speech_act_committed`, belief updates in listener | heard claim | visible as response options based on actor beliefs | sincerity/confidence debug | same. |
| `tell` | speaker has claim to communicate | listener/channel available | claim source exists: observation, memory, hearsay, record, inference, lie | speakability/privacy norm | short | listener refuses/does not hear; speaker withholds | `speech_act_committed`, `belief_formed/updated` possible | overheard speech | visible only for actor-known claims | source chain and omitted facts | same. |
| `report_missing_property` | reporter has actor-known missing-property claim | authority receiver available or report can be delayed | reporter believes item expected and absent, or reports hearsay with mode | report venue/role; false report norms later | medium | rejected if no reporter/source/item/expectation; delayed if office unavailable | `speech_act_committed`, `report_received/refused/delayed` | office witness, ledger precursor | visible when actor knows enough and can reach authority | why report allowed/blocked | same; AI Tomas can report without human. |
| `gossip` | speaker/listeners in social setting | conversational channel | claim may be hearsay/speculation; source should be retained | reputation/privacy norms | short | listener uninterested/refuses; claim degrades | `speech_act_committed`, `belief_updated/weakened` | overheard rumor | visible in tavern/square with known rumor | source loss/uncertainty | same. |
| `refuse_or_withhold` | actor addressed or pressured | communicative situation | actor has reason: privacy, fear, uncertainty, duty | norms may permit/forbid refusal | short | refusal may fail under procedure later | `speech_act_committed`, `report_refused` if institutional | suspicion/reputation trace | visible as response if actor has motive/right | reason hidden/visible split | same. |
| `lie_or_mislead` | speaker has claim and motive/pressure | channel available | speaker knows or believes statement conflicts with private belief, or strategically omits | norm violation if discovered; requires modeled motive | short | blocked if no motive/knowledge gap; failure if contradiction exposed | `speech_act_committed`, `belief_updated`, debug sincerity marker | possible contradiction trace | embodied UI shows statement, not hidden truth | debug-only lie/sincerity visibility | same; AI lies only with modeled motive. |
| `accuse_or_form_suspicion` | actor can reason/socially speak | target actor exists; channel if spoken accusation | actor-known basis required: motive/access/observation/hearsay/record; reckless accusation only if modeled | defamation/order norms; institutional threshold | short/medium | rejected if no actor-known basis; blocked if actor tries to use debug truth | `suspicion_formed`, `speech_act_committed`, or `action_rejected` | social trace, record if reported | visible only when basis exists or reckless mode allowed | why-not basis list | same. |
| `go_to_local_authority` | actor can move | route to `reeves_office` | actor believes authority relevant | office hours/access norms | local travel | fails if office closed/interrupted/refusal | `actor_moved`, maybe `intention_plan_updated` | route witness | visible when actor knows office and has reason | route/intent explanation | same. |
| `receive_report` | Anna/Reeve/authorized office actor | reporter present/channel | understands structured report fields | institutional role required | medium | refused/delayed/incomplete | `report_received/refused/delayed`, `speech_act_committed` | office witness | visible to authorized actor when report offered | missing field reasons | same for human-possessed Anna and AI Anna. |
| `create_or_update_institutional_record` | authorized recorder, usually Anna | ledger artifact accessible; writing materials | source speech/claim exists | institutional role and procedure | medium | rejected if no artifact/authorization/source; failure if interrupted | `institutional_record_opened/amended`, `memory_recorded` | ink/page access trace | visible only to authorized actor with source | record-as-claim explanation | same. |
| `question_or_watch_under_procedure` | post-`PHASE-4-ENTRY` authority/guard role | target/place accessible; procedure authorized | record/suspicion basis exists | institutional procedure required | medium/long | rejected if no basis/authority; failure if target absent | `speech_act_committed`, `observation_created`, `suspicion_updated`, `record_amended` | watch/question traces | first-proof ontology sketch only; not historical implementation work feature | basis/procedure/debug | same once implemented; no player-only interrogation. |
| `continue_or_abandon_current_intention` | actor with current intention/routine | time and conditions change | actor has goal/routine/pressure/conflict | duties/social expectations | instant/short | cannot continue if impossible; abandonment may have consequence | `intention_plan_delayed/updated` | schedule deviation visible | visible as actor notebook/debug; embodied maybe “keep doing/stop” | intention cause chain | same; later cognition uses same event hooks. |

## 10. Event vocabulary

Events are primitive semantic families for this first-proof contract. They are not final serialized types. They must be sufficient for event-log authority, replay, projections, belief updates, debug explanation, and later tests.

Every event must have a stable event ID, timestamp/tick, responsible actor/process/institution, cause references, and replay-deterministic inputs. Events may be ground truth while projections and actor views remain filtered.

| Event family | Event type | Responsible actor / process / institution | Cause model | Referenced prior items | World mutation | Projection effects | Replay requirements | Visibility | Forbidden causes |
|---|---|---|---|---|---|---|---|---|---|
| Actor moved | `actor_moved` | moving actor or scheduler/routine | accepted `move_locally` action or routine continuation | prior location, route, door state, intention | actor location changes | map view, witness opportunities, routine state | deterministic route/time; seed if stochastic delay | ground-truth; actor-visible to observers with sight/sound; debug | player proximity, quest trigger, hidden director. |
| Door opened/closed | `door_opened`, `door_closed` | acting actor or environmental process if modeled | accepted open/close action | door ID, lock/state, actor access | door state changes | visibility/sound graph updates | deterministic state transition | ground-truth; visible/audible to eligible actors; debug | omniscient reveal, cutscene staging. |
| Container accessed/opened/closed | `container_accessed`, `container_opened`, `container_closed` | acting actor | accepted container action | container, actor, location, access adjudication | container state/access time changes | possible trace, visibility to contents | deterministic container state | ground-truth; visible/audible if observed; debug | “loot UI” without actor access. |
| Item transferred/relocated | `item_transferred`, `item_relocated` | actor/process moving item | take/place/hide action or seeded prehistory | source container/holder, destination, ownership/custody refs | item possession/location changes | inventory/container projections; expectation contradictions later | identity continuity and destination deterministic | ground-truth; actor-visible only if observed/remembered; debug | abstract balance delta, quest reward, director theft. |
| Item observed present | `item_observed_present` | observing actor/process | inspect/search/line-of-sight | item, location, sensory channel | no ground-truth mutation | actor belief candidates, memory | deterministic visibility; no hidden truth injection | actor-visible to observer; debug | seeing through closed containers/rooms. |
| Expected item observed absent | `expected_item_observed_absent` | observing actor | search/inspection plus prior expectation/source | expected item proposition, search event, container state | no direct mutation | contradiction belief, report affordance | must cite expectation and search scope | actor-visible to observer; debug | empty container as evidence without expectation. |
| Search performed | `search_performed` | searching actor | accepted search action | place/container, search intensity, access | may disturb traces/container state | observations, failure/success metrics | deterministic search parameters/seed if uncertain | ground-truth; maybe observed by others; debug | automatic clue discovery. |
| Trace created/observed/erased | `trace_created`, `trace_observed`, `trace_erased` | action actor, observer, or process | physical contact, movement, cleaning, weather if later | source event, surface/container, visibility | trace state changes or observation created | trace projection, belief source | deterministic source and lifespan | ground-truth for trace; actor-visible when observed; debug | culprit pointer, retroactive clue. |
| Observation created | `observation_created` | observer actor/process | sensory perception or search | perceived event/object/place, channel, constraints | no direct world mutation | actor memory/belief candidates | visibility model deterministic | actor-visible to observer; debug | omniscience, truth labels. |
| Belief formed/updated/contradicted/weakened | `belief_formed`, `belief_updated`, `belief_contradicted`, `belief_weakened` | actor cognition/projection | observation, speech, memory, inference, record | source belief/event/claim, proposition | actor belief store changes | embodied notebook, speech options, suspicion basis | deterministic update rules or explicit seed | actor-private; debug can show | global truth overwrite, stale info autocorrect. |
| Memory recorded/recalled | `memory_recorded`, `memory_recalled` | actor cognition | observation, event salience, report, routine | event/belief/proposition refs | actor memory store changes | later speech/reasoning | deterministic retention stage; seed if probabilistic later | actor-private; debug | perfect universal history for actors. |
| Speech act committed | `speech_act_committed` | speaker actor | ask/answer/tell/report/gossip/lie/refuse action | structured propositions, source beliefs, audience | social state maybe changes; no truth mutation | listener interpretations, heard-by observations | deterministic rendering from structured act | heard by listeners; debug shows sincerity only | freeform LLM state mutation. |
| Report received/refused/delayed | `report_received`, `report_refused`, `report_delayed` | institution receiver, usually Anna/Reeve | report action and intake adjudication | reporter, speech act, structured claims | report queue/status changes | institutional view, future record ability | deterministic procedure | institution-visible; reporter-visible; debug | authority reading truth. |
| Institutional record opened/amended | `institutional_record_opened`, `institutional_record_amended` | Anna/Reeve/institution process | authorized write from report/question/amendment | ledger artifact, writer, source speech/belief | ledger artifact state changes | record projection, access views | deterministic order, page refs | institution-visible to authorized; debug | record-as-truth. |
| Suspicion formed | `suspicion_formed` | actor or institution projection | motive/access/observation/hearsay/record inference | target, basis propositions, confidence | actor/institution suspicion state changes | speech/report/procedure affordances | deterministic basis scoring or explicit rule | actor-private/institution-visible if recorded; debug | hidden culprit marker. |
| Intention/plan delayed/updated | `intention_plan_delayed`, `intention_plan_updated` | actor cognition/routine scheduler | conflict, report, fear, interruption, routine | prior intention, cause event, new commitment | actor intention state changes | schedule/routine projections | deterministic planner state | actor-private; visible through behavior; debug | script forcing target outcome. |
| Action rejected | `action_rejected` | adjudicator/system | failed precondition before attempt | proposed action, missing requirement | no ordinary mutation except audit | why-not view, debug | deterministic adjudication | actor-visible as blocked affordance; debug | silent failure hiding doctrine. |
| Action failed | `action_failed` | acting actor/process | attempted action with failed execution | proposed action, risk/seed if stochastic | partial trace possible | failure memory, witnesses | seed/cause logged | actor-visible; ground-truth; debug | random unlogged failure. |
| No-human advance/run marker | `no_human_run_started`, `no_human_run_advanced`, `no_human_run_stopped` | scheduler/simulation process | explicit debug/test run command | seed, tick range, scheduler config | time advances; ordinary events happen | no-human metrics | seed and config logged | debug/test only; no diegetic visibility | player-conditioned schedule. |
| Replay/projection rebuilt | `replay_projection_rebuilt` | replay/projection process | load/rebuild command | event log slice, projection version | projection state rebuilt, not canonical events | debug and test comparison | deterministic from event log and version | debug/test only | correcting event log from projection. |

## 11. Proposition and claim vocabulary

Claims are source-bound. A claim is not truth. A proposition can be true, false, unknown, uncertain, stale, contradicted, or held by different actors with different confidence.

| Proposition family | Arguments | Possible holders | Possible sources | Contradiction patterns | Usable for speech/report/record/suspicion | TUI rendering implications |
|---|---|---|---|---|---|---|
| Item located in container/place | item ID, container/place ID, time or interval | any actor, institution record, debug projection | direct observation, prior placement memory, hearsay, record, inference | same item at incompatible location/time; later absence; source impossibility | speech, record, search target, suspicion if access/motive combine | embodied: “I think the coins are in the strongbox”; debug: truth/belief split. |
| Item missing | item ID, expected location, expected time, observer, search scope | expecting/searching actor, listener after report, institution record | expectation + search/absence observation, hearsay report | item observed present by same actor at same time; search scope invalid; expectation lacks source | report, record, suspicion, questioning | show source and uncertainty; no objective “stolen” label. |
| Actor possesses item | actor/container ID, item ID, time | observer, actor, institution, debug | direct sight, search recovery, confession/claim, inference from container | item location elsewhere; no line-of-sight; possession vs ownership confusion | suspicion/proof if sourced; report amendment | embodied: “Mara may have it” only if basis exists. |
| Actor owns item | actor/household, item, norm/source | owner, household, institution, neighbors | prior social knowledge, records, testimony | competing ownership claim; stolen property possession | report and record; not proof of current possession | distinguish “owned by Tomas” from “with Tomas.” |
| Actor may access container/location | actor, container/place, permission source, time | actor, household, institution, observers | role, invitation, routine, key custody, procedure | access expired; permission source denied; trespass observation | action adjudication, suspicion, record | TUI marks allowed/intrusive/unknown access. |
| Actor saw/heard something at place/time | actor, channel, content, place, time, confidence | witness, listener, record | observation memory, speech report | impossible visibility/sound; conflicting times | speech, report, record, suspicion basis | preserve sensory wording: “heard a door,” not “saw theft.” |
| Actor saw item present/absent | observer, item, location, time, search scope | observer, listener, record | direct observation or search event | item moved after observation; search incomplete; no expectation | report/record/evidence | absence rendering requires expected item and search scope. |
| Actor heard claim from source | hearer, speaker/source, claim ID, time, mode | any listener, institution record | speech act, rumor, record read | source denial, hearsay chain broken, contradicted content | gossip/report/record but with hearsay mode | show “Anna says Tomas reported…” not “coins stolen.” |
| Report was filed | report ID, reporter, receiver, institution, time, status | reporter, institution, readers | report event, ledger record | delayed/refused not filed; record missing/amended | institutional views, future procedure | embodied office view shows status, not truth. |
| Record says X | record ID, proposition, author, source claim | institution, authorized readers, later hearers | ledger read or report summary | amendment contradicts earlier entry; author error | speech/report/procedure | records displayed as text claims with source. |
| Actor may be responsible | target actor, incident, basis propositions, confidence | suspecting actor, institution | motive/access/opportunity/observation/hearsay | alibi, better explanation, source contradiction | suspicion, questioning, record if stated | “suspects” not “culprit”; debug can show wrongness. |
| Actor needs/intends something | actor, need/intention, source, time | actor, close confidant, observer/institution by inference | self memory, speech, observed routine, record | behavior contradiction, denial, stale need | motive analysis, speech, plan update | embodied must show uncertainty for others' minds. |
| Debt/payment/obligation unmet | debtor, creditor, amount/token/description, due time, source | debtor, creditor, hearers, records | debt note, speech, memory | payment record, denial, amount dispute | motive, report context, wrong suspicion | distinguish debt motive from theft proof. |
| Institution opened incident | institution, incident ID, reporter, status | institution, reporter, authorized readers | report received + record opened | report refused/delayed; record amended closed | procedure/status | no “case solved” quest state. |
| Suspicion candidate | incident, actor, basis set, confidence, holder | actor, institution | inference over claims/traces/records | basis disproven, alibi, source weakness | internal suspicion, speech if speakable | debug causal graph must show basis and confidence. |

## 12. Belief, observation, memory, and expectation contract

### 12.1 Belief shape

This belief contract must include at least:

- `belief_id`;
- `holder_id` actor or institution/projection;
- `proposition_family` and structured arguments;
- `stance`: believes, disbelieves, suspects, questions, denies, intends, expects;
- `confidence`: qualitative scale such as certain/high/medium/low/unknown, not a final numeric API;
- `source_refs`: observation, memory, speech act, record, inference, seed/prehistory event;
- `source_channel`: sight, sound, touch/search, self-memory, hearsay, record, inference;
- `acquisition_time`;
- `believed_event_time` or interval if known;
- `contradiction_links` to conflicting beliefs/propositions;
- `privacy` and `speakability`: private, household, public, institutional, confidential;
- `stale_status`: fresh, stale, contradicted, weakened, refreshed;
- `debug_notes`: non-diegetic explanation only.

A belief is not truth. A true proposition is not actor knowledge until acquired through modeled channels.

### 12.2 Observation shape

An observation must include:

- `observation_id`;
- `observer_id`;
- `channel`: sight, sound, touch, smell if later needed, search, record-read;
- `place_id` and actor location;
- `time` or interval;
- sensory object/event observed;
- visibility/sound constraints;
- confidence or ambiguity;
- whether it is direct, inferred, or hearsay-derived;
- generated candidate proposition IDs.

Observation and interpretation are separate. Petra may observe “a figure on the lane near dusk.” She does not thereby observe “Mara stole Tomas's coins.”

### 12.3 Expectation as belief/projection

Expectation is a belief about what should be true now or at a future time, grounded in source/provenance.

Required seeded expectation:

```text
belief_tomas_coin_expected_001
holder: actor_tomas
proposition: item located in container(coin_stack_01, strongbox_tomas, morning_d0)
stance: expects
confidence: high
source_refs:
  - obs_tomas_coin_check_dminus1
  - memory_tomas_closed_strongbox_dminus1
source_channel: prior direct observation + self-memory
speakability: private until Tomas chooses to report/tell
```

`obs_tomas_coin_check_dminus1` is a seeded prior direct observation/prehistory event. It is allowed because prehistory establishes starting facts and actor memories; it must not force future theft, discovery, report, suspicion, or resolution.

### 12.4 Absence-as-evidence

Absence is evidence only when an actor has an expectation or instruction that makes absence meaningful.

Valid chain:

```text
Tomas expects coin_stack_01 in strongbox_tomas
Tomas opens/searches strongbox_tomas
search scope includes where coin_stack_01 should be
coin_stack_01 is not observed there
expected_item_observed_absent event is created
Tomas's belief is contradicted or updated
Tomas may form item_missing claim
```

Invalid chain:

```text
Mara sees an empty strongbox with no prior expectation
=> Mara automatically knows Tomas's coins are missing   # forbidden
```

### 12.5 Memory depth staged for the first proof

This spec preserves the memory contract as first-proof ontology and fixture material. The current implementation and future certification sequence are governed by `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md`, `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`, `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`, and `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md`. Historical phase labels are evidence only; they are not certification.

Memory must be fallible/stale-capable by contract even if early executable phases use simple retention.

## 13. Speech and report contract

Speech is structured action. Rendered text is a surface. No freeform LLM speech appears in Spec 0001, and LLM text may not mutate game state.

Every speech act must include:

- speaker;
- listener/audience;
- structured propositions carried;
- source beliefs or source gaps;
- claim mode: asserted, hearsay, speculation, question, denial, lie, promise, command;
- confidence/uncertainty;
- listener interpretation;
- possible belief update;
- failure/refusal modes;
- deterministic rendering rule;
- debug-only sincerity/lie visibility.

| Speech act | Speaker / listener | Structured propositions carried | Source beliefs | Claim mode | Confidence / uncertainty | Listener interpretation | Possible belief update | Failure / refusal modes | Deterministic rendering | Debug-only sincerity / lie visibility |
|---|---|---|---|---|---|---|---|---|---|---|
| `ask` | any actor to reachable listener | question target proposition | information need, prior belief, routine | question | may carry uncertainty | listener receives request for claim | listener may recall/search/answer/refuse | listener absent, privacy, no topic basis | template from topic and relationship | no hidden truth shown. |
| `answer_sincerely` | respondent to asker | proposition(s) respondent believes | respondent belief/memory/record | asserted or hearsay | confidence mirrors belief | listener treats as sourced claim | forms/updates belief with speaker/source | respondent lacks memory or refuses | template by proposition family and confidence | debug may mark sincere. |
| `answer_uncertainly` | respondent to asker | weak/ambiguous proposition | stale memory/ambiguous observation | asserted with uncertainty | low/medium | listener receives weak evidence | belief formed weakly or not at all | over-demanding listener may reject | hedged template | debug shows uncertainty source. |
| `deny_knowledge` | respondent to asker | denial of knowledge about proposition, not denial of truth unless explicit | absence of belief, privacy, fear | denial | variable | listener may believe ignorance or suspect withholding | updates speaker-knows proposition, not world truth | contradicted if respondent has known source and chooses lie mode | “I do not know…” style template | debug distinguishes honest ignorance, privacy withholding, lie. |
| `lie_or_mislead` | actor with motive to listener | false assertion or selective omission | private belief conflicts or omission target | lie/mislead | chosen confidence may differ from private belief | listener may form false belief if trust/source enough | belief update with hidden contradiction | blocked if no modeled motive/knowledge gap | same surface as assertion; no “lie” label embodied | debug marks sincerity false and motive. |
| `report_missing_property` | reporter to Anna/Reeve/authorized receiver | item missing, expected location, ownership/custody, source expectation, search result | reporter belief chain | asserted report | confidence from expectation/search | receiver records report as claim | institution gains report/record belief, not truth | refused if no item/source/reporter/venue; delayed if office unavailable | formal intake template | debug shows fields accepted/rejected. |
| `repeat_rumor_or_testimony` | speaker to listener/audience | heard claim with source chain | prior speech act or record read | hearsay/speculation | confidence degraded unless source trusted | listener tracks source if contract works | hearsay belief, rumor propagation | source forgotten if modeled; listener refuses gossip | hearsay template naming source if known | debug shows chain degradation. |
| `institutional_intake_acknowledgment` | Anna/Reeve to reporter | report status, missing fields, next procedure | received report and institutional rules | acknowledgment/command/request | procedural certainty | reporter knows record/refusal/delay status | belief about institution opened/refused incident | incomplete fields, access/time constraints | office-formal template | debug shows no truth validation occurred. |
| `refuse_or_withhold` | any actor | refusal reason if speakable; no proposition if silent | privacy, fear, duty, uncertainty | refusal/withholding | n/a or low | listener may form belief about refusal | suspicion or trust change possible | procedure may later compel/override if post-`PHASE-4-ENTRY` work rules | relationship/status template | debug shows hidden reason only. |

## 14. Institution, household, norm, and record contracts

Households are first-class domestic institutions. They are not decorative labels.

### 14.1 Household contracts

| Household ID | Members | Places / artifacts | Rights and duties | First-proof role |
|---|---|---|---|---|
| `household_tomas` | Tomas, Elena | `house_tomas`, `pantry_tomas_shelf`, `strongbox_tomas` | domestic privacy, food custody, private strongbox custody | Owns/claims expected property context. |
| `household_mara` | Mara, optionally Lena depending fixture | `house_mara`, `basket_mara`, `small_cache_mara` | domestic privacy, personal goods | Possible hiding location and privacy conflict. |
| `household_iva` | Iva | `house_iva`, `chest_iva` | debt records/goods, domestic privacy | Debt pressure source. |
| `household_nika` | Nika | `tavern_nika`, `tavern_till_nika` | commercial/customary privacy | Rumor/public-food context. |
| `household_rafi_lena` | Rafi, Lena | `house_rafi_lena` | domestic privacy, household support | Wrong-suspicion and wage-pressure context. |
| `household_petra` | Petra | `house_petra`, lane-facing window | domestic privacy; visual access from home | Partial witness context. |
| `household_soren` | Soren | `house_soren`, `satchel_soren` | work/personal goods | Ordinary route ambiguity. |

### 14.2 Local authority

`institution_reeves_office` is the only local authority in Spec 0001.

Roles:

- Anna: clerk; receives reports, writes/amends ledger, tracks source claims and uncertainty.
- Elias: guard; watches, asks, escorts, reports observations; cannot create proof from status bias.
- Reeve: local authority; reviews records, authorizes procedures, refuses unsupported sanction.
- Corin: optional assistant/runner; carries structured messages only if needed; not a quest courier.

### 14.3 Norms

| Norm ID | Type | Applies to | Violation | Detection | Suspicion | Report | Record | Proof | Sanction | Notes |
|---|---|---|---|---|---|---|---|---|---|---|
| `norm_property_theft` | regulative property norm | physical items with owner/custodian and access limits | actor intentionally takes/keeps item without permission or legitimate excuse | observation, missing expected property, trace, confession, recovery | actor-known basis combining motive/access/opportunity/evidence | property owner or witness may report | Anna records claims and sources | requires evidence chain, not debug truth | deferred; post-`PHASE-4-ENTRY` work sketch only | Violation, detection, suspicion, report, record, proof, sanction, notice, and payment are distinct. |
| `norm_trespass_privacy` | domestic/institution privacy norm | private rooms, homes, restricted records, containers | actor enters/searches/opens without permission/procedure | witness, trace, admission, observation | access violation may support suspicion but not theft proof | household/institution may complain | record if reported | requires source-bound evidence | deferred; post-`PHASE-4-ENTRY` work sketch only | Prevents search-as-universal-solution. |

### 14.4 Incident ledger record shape

Records are claims, not truth.

Required incident ledger record shape:

```text
record_id: stable record ID
physical_artifact_id: incident_ledger_01 / page or entry reference
institution_id: institution_reeves_office
author_writer_id: actor_anna or other authorized writer
reporter_id: actor_tomas or source actor
receiver_id: actor_anna / actor_reeve
created_time: tick/date
structured_claims:
  - item_missing(...)
  - item_located_expected(...)
  - actor_owns_item(...)
  - actor_saw_item_absent(...)
source_beliefs:
  - reporter belief IDs
source_speech_acts:
  - report speech act IDs
confidence_uncertainty: per claim
status: opened / incomplete / amended / under_review / refused / closed_unresolved / closed_proven_later
amendments:
  - amendment IDs, authors, source claims, contradiction links
readers:
  - authorized actor IDs and read events
access_permissions:
  - Anna/Reeve; Elias by procedure; public no direct read by default
lifecycle_events:
  - report_received
  - institutional_record_opened
  - institutional_record_amended
  - report_delayed/refused if relevant
```

A record can say “Tomas reports that his coins are missing from his strongbox.” It must not say “Mara stole Tomas's coins” unless an actor has made that sourced claim and the record preserves source/confidence/status.

## 15. Fixture suite and variants

The fixture suite is now treated as live first-proof contract material and code-adjacent review evidence. Some fixture families are already represented by executable content fixtures under `crates/tracewake-content/src/fixtures/`; this spec does not certify those fixtures. Certification belongs to the execution gate sequence and future `P0-CERT` work.

### 15.1 Implemented fixture inventory

The following content fixture files exist in the target tree and are the live code-adjacent surface this ontology must stay consistent with:

| Fixture file | Contract role in this spec | Gate references |
|---|---|---|
| `container_item_move_001.rs` | Physical item movement, container location, and event causality. | `EVENT`, `MISSING-PROPERTY`, `REPLAY` |
| `debug_attach_001.rs` | Debug attachment is non-diegetic and separate from embodied play. | `VIEW-DEBUG-SPLIT`, `TRUTH-FIREWALL` |
| `debug_omniscience_excluded_001.rs` | Debug truth does not leak into embodied holder-known contexts. | `VIEW-DEBUG-SPLIT`, `TRUTH-FIREWALL`, `FIXTURE-NEGATIVE` |
| `door_access_001.rs` | Door, access, room, and embodied visibility baseline. | `EVENT`, `ACTOR-KNOWN` |
| `expectation_contradiction_001.rs` | Expected item absence contradicts prior actor-known belief. | `ACTOR-KNOWN`, `MISSING-PROPERTY`, `FIXTURE-NEGATIVE` |
| `food_unavailable_replan_001.rs` | Ordinary need pressure replans without hidden truth. | `NO-HUMAN-ORDINARY-LIFE`, `TRUTH-FIREWALL` |
| `hidden_food_closed_container_001.rs` | Closed-container hidden truth may not plan action. | `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `FIXTURE-NEGATIVE` |
| `hidden_food_unknown_route_001.rs` | Unknown route/food facts cannot be injected into planning. | `TRUTH-FIREWALL`, `ACTOR-KNOWN` |
| `hidden_route_edge_001.rs` | Route knowledge is holder-known, not global truth for actors. | `TRUTH-FIREWALL`, `ACTOR-KNOWN` |
| `knowledge_blocker_accuse_001.rs` | Accusation is blocked when the actor lacks actor-known basis. | `ACTOR-KNOWN`, `TRUTH-FIREWALL`, `FIXTURE-NEGATIVE` |
| `no_hidden_truth_planning_001.rs` | Planner inputs exclude hidden truth. | `TRUTH-FIREWALL`, `ACTOR-KNOWN`, `FIXTURE-NEGATIVE` |
| `no_human_advance_001.rs` | Time can advance without human action. | `NO-HUMAN-ORDINARY-LIFE`, `EVENT` |
| `no_human_day_001.rs` | Ordinary day progression without player steering. | `NO-HUMAN-ORDINARY-LIFE`, `REPLAY` |
| `no_human_epistemic_check_001.rs` | No-human progression preserves epistemic boundaries. | `NO-HUMAN-ORDINARY-LIFE`, `ACTOR-KNOWN`, `TRUTH-FIREWALL` |
| `ordinary_workday_001.rs` | Work routine baseline for ordinary local economy. | `NO-HUMAN-ORDINARY-LIFE`, `EVENT` |
| `planner_trace_001.rs` | Planner trace observability without hidden-truth planning. | `TRUTH-FIREWALL`, `ACTOR-KNOWN` |
| `possession_does_not_reset_intention_001.rs` | Possession does not erase actor state or intention. | `POSSESSION-PARITY`, `ACTOR-KNOWN` |
| `possession_parity_001.rs` | Human and non-human control use the same actor-known filters. | `POSSESSION-PARITY`, `ACTOR-KNOWN`, `TRUTH-FIREWALL` |
| `replay_item_location_001.rs` | Rebuild/replay preserves item location from events. | `EVENT`, `REPLAY`, `MISSING-PROPERTY` |
| `routine_blocked_diagnostic_001.rs` | Diagnostic output explains blockage without becoming diegetic truth. | `VIEW-DEBUG-SPLIT`, `TRUTH-FIREWALL` |
| `routine_no_teleport_001.rs` | Routine execution must respect spatial/event constraints. | `EVENT`, `NO-HUMAN-ORDINARY-LIFE` |
| `sleep_eat_work_001.rs` | Sleep, food, and work pressures remain ordinary-life mechanics. | `NO-HUMAN-ORDINARY-LIFE`, `EVENT` |
| `sound_uncertainty_001.rs` | Observation may be uncertain and source-bound. | `ACTOR-KNOWN`, `TRUTH-FIREWALL` |
| `strongbox_001.rs` | Tomas, `strongbox_tomas`, `coin_stack_01`, custody, access, and search baseline. | `EVENT`, `ACTOR-KNOWN`, `MISSING-PROPERTY`, `REPLAY` |
| `view_filtering_001.rs` | Embodied views are filtered by holder-known context. | `VIEW-DEBUG-SPLIT`, `ACTOR-KNOWN`, `TRUTH-FIREWALL` |
| `view_model_local_actions_001.rs` | Local affordances expose only holder-known selectable actions. | `VIEW-DEBUG-SPLIT`, `ACTOR-KNOWN` |
| `workplace_assignment_provenance_001.rs` | Work assignment provenance is recorded rather than implied. | `EVENT`, `NO-HUMAN-ORDINARY-LIFE` |

Gate names above are cross-references only. Their definitions live in `docs/2-execution/`.

### 15.2 Contract fixture families

#### Strongbox / item / custody family

- **Purpose:** Establish `strongbox_tomas` as a physical private container containing physical `coin_stack_01`, with ownership/custody/access separated.
- **Minimum material:** Tomas's house, bedroom, strongbox, coins, Tomas prior memory, initial container state, access permissions, and at least one search event.
- **Positive assertions:** item location is event-derived; Tomas may open/search when actor-known and access preconditions pass; ownership does not equal current possession; replay rebuilds the same item location.
- **Negative assertions:** actor outside room cannot see contents; no actor knows absence before observation or report; coins are not an abstract balance; debug truth cannot create a belief.
- **Implemented surface:** `strongbox_001.rs`, `container_item_move_001.rs`, `replay_item_location_001.rs`.

#### Expectation contradiction family

- **Purpose:** Prove absence-as-evidence from a sourced prior expectation plus a scoped search.
- **Minimum material:** Tomas has a sourced expectation; the coins are absent from the searched scope; Tomas observes absence; the resulting claim remains actor-known and source-bound.
- **Positive assertions:** the missing-property claim references prior expectation and search scope; Tomas can report missing property.
- **Negative assertions:** Mara, Anna, and other holders do not know the absence until told, shown, or procedurally recorded; the absence is not automatically theft.
- **Implemented surface:** `expectation_contradiction_001.rs`, `sound_uncertainty_001.rs`.

#### Possession parity and view filtering family

- **Purpose:** Prove that human-controlled actors and autonomous actors use identical actor-known filters and that debug surfaces are visibly quarantined.
- **Minimum material:** same event prefix under multiple possession states; at least one embodied actor, debug view, local action view, and no-human continuation.
- **Positive assertions:** the same actor-known context drives selectable actions and descriptions regardless of human control; debug truth appears only in debug surfaces.
- **Negative assertions:** possessing Anna does not reveal coin location; possessing Mara does not reveal Tomas's current belief unless learned; debug panels cannot update actor memory.
- **Implemented surface:** `possession_parity_001.rs`, `possession_does_not_reset_intention_001.rs`, `view_filtering_001.rs`, `view_model_local_actions_001.rs`, `debug_attach_001.rs`, `debug_omniscience_excluded_001.rs`.

#### No-human ordinary-life family

- **Purpose:** Prove that ordinary settlement life can advance without player steering and without authored outcome chains.
- **Minimum material:** sleep, food, work, movement, access, routine blockage, replanning, workplace assignment, and deterministic replay boundaries.
- **Positive assertions:** ordinary actions occur from needs, routines, intentions, access, and actor-known context; no-human advancement can produce relevant world changes without a quest controller.
- **Negative assertions:** no teleporting routines; no hidden-food or hidden-route planning; no player-only causal step; no marker action standing in for ordinary activity.
- **Implemented surface:** `no_human_advance_001.rs`, `no_human_day_001.rs`, `no_human_epistemic_check_001.rs`, `ordinary_workday_001.rs`, `sleep_eat_work_001.rs`, `food_unavailable_replan_001.rs`, `routine_blocked_diagnostic_001.rs`, `routine_no_teleport_001.rs`, `workplace_assignment_provenance_001.rs`.

#### Anti-contamination adversarial family

- **Purpose:** Keep fixture data, planner traces, local actions, and diagnostics from becoming hidden scripts or omniscient inputs.
- **Minimum material:** actor-known basis checks, no-hidden-truth planning checks, closed-container checks, unknown-route checks, and diagnostic quarantine.
- **Positive assertions:** an action proposal can be rejected for lack of actor-known basis; diagnostic output identifies blockage without changing actor knowledge; planner trace is review evidence, not an actor input.
- **Negative assertions:** forged or stale proposal parameters cannot authorize action; debug state cannot satisfy preconditions; fixture outcome cannot preordain selection.
- **Implemented surface:** `knowledge_blocker_accuse_001.rs`, `no_hidden_truth_planning_001.rs`, `hidden_food_closed_container_001.rs`, `hidden_food_unknown_route_001.rs`, `hidden_route_edge_001.rs`, `planner_trace_001.rs`, `routine_blocked_diagnostic_001.rs`.

#### Institutional report and wrong-suspicion family

- **Purpose:** Preserve the missing-property institution path without certifying post-`PHASE-4-ENTRY` implementation.
- **Minimum material:** a sourced missing-property report, an institution holder-known context, source-bound record entries, and fallible suspicion that remains distinct from proof.
- **Positive assertions:** a report is a claim/record with source provenance; an institution can know that a report exists without knowing truth; suspicion remains basis-bound and revisable.
- **Negative assertions:** records do not become omniscient truth; suspicion does not identify culprit; institutional procedure does not bypass actor/institution holder-known context.
- **Implementation status:** live contract sketch only. The current fixture inventory contains supporting epistemic and accusation-blocking surfaces, but this family is governed by `docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md` and any future specs beneath it.

## 16. Embodied and debug view-model sketches

Embodied views are actor-known filtered and uncertainty-preserving. Debug views are visibly non-diegetic and outside every embodied holder-known context.

### 16.1 Embodied view as Tomas before discovery

```text
You are Tomas.
Place: house_tomas / tomas_main_room.
Intentions: open mill soon; pay Rafi after checking household money.
Known nearby: Elena may be in the pantry. The bedroom door is closed.
Private memory: You counted coin_stack_01 in strongbox_tomas last evening and closed the box.
Expected: coin_stack_01 should be in strongbox_tomas.
Visible affordances: go to bedroom; ask Elena; wait; go to mill.
Not shown: whether the coins are actually still there; who will move them; any culprit label.
```

### 16.2 Embodied view as Mara after moving/taking coins

```text
You are Mara.
Place: house_mara / main_room.
Current private state: You remember handling coin_stack_01 if and only if this fixture branch actually had you do so.
Carried/nearby: shown only if the coins are in your possession or visible in your container.
Pressures: Iva's debt demand; fear of Elias; loyalty/fear involving Lena.
Beliefs: Tomas may notice if he checks the strongbox. You do not know whether he has checked yet unless you observed/heard it.
Visible affordances: hide/place carried item; speak to Lena; go to square; wait; deny/withhold if asked.
Not shown: Anna's private record; Tomas's current belief unless learned through speech/observation; “culprit” label.
```

### 16.3 Embodied view as Tomas after discovering absence

```text
You are Tomas.
Place: house_tomas / bedroom_tomas.
Observed: strongbox_tomas is open. coin_stack_01 is not where you expected it.
Contradicted expectation: You remember seeing the coins here last evening.
Current belief: The coins are missing from the strongbox. You do not know who moved them.
Possible next actions: search bedroom; ask Elena; report to Anna; go to mill; wait.
Blocked accusation note: You cannot accuse Mara yet without actor-known basis. You may say you are worried or ask if anyone saw something.
```

### 16.4 Embodied view as Anna after report

```text
You are Anna.
Place: reeves_office / office_counter.
Incoming claim: Tomas reports coin_stack_01 missing from strongbox_tomas.
Source supplied: Tomas says he counted it last evening and searched the strongbox this morning.
Institutional status: report can be accepted as a claim if required fields are complete.
Your knowledge: You do not know whether the coins were stolen. You know Tomas made a structured report.
Visible affordances: ask follow-up; receive report; refuse incomplete report; open/amend incident ledger; tell Elias a sourced summary.
Not shown: actual coin location unless separately observed or recorded through authorized process.
```

### 16.5 Debug view of truth/belief mismatch

```text
[DEBUG / NON-DIEGETIC]
Truth projection:
  coin_stack_01 location: small_cache_mara  (from evt_item_relocated_014)
  strongbox_tomas: empty, closed
Actor beliefs:
  Tomas: expects coin_stack_01 in strongbox_tomas until evt_expected_absent_022; then believes missing, culprit unknown
  Mara: knows/does-not-know based on branch; if mover, remembers relocation event
  Anna: knows only report claim after evt_report_received_031
  Elias: suspects Rafi from basis {wage_motive, seen_near_mill}, confidence low/medium
Records:
  incident_ledger_01/entry_001: Tomas reports missing coins; status opened; no proof
Warnings:
  No embodied actor may read this panel.
```

### 16.6 Actor notebook

```text
Notebook: Tomas
Memories:
  - Last evening: counted coin_stack_01 in strongbox_tomas. Source: direct observation.
  - This morning: searched strongbox_tomas; did not find coin_stack_01. Source: direct search.
Beliefs:
  - coin_stack_01 missing from strongbox_tomas. Confidence: high.
  - unknown actor may have moved it. Confidence: low/speculative.
Speakable claims:
  - report missing property to Anna.
  - ask Elena whether she moved or saw the coins.
Not speakable as fact:
  - “Mara stole it.” No actor-known basis yet.
```

### 16.7 Why-not explanation for blocked accusation

```text
Action blocked: accuse actor_mara of theft.
Actor: Tomas.
Missing requirements:
  - no direct observation of Mara taking coin_stack_01;
  - no sourced claim that Mara possessed coin_stack_01;
  - no record linking Mara to strongbox_tomas;
  - debt/motive claim not currently known by Tomas in this branch;
  - strongbox absence proves absence from expected place, not culprit.
Allowed alternatives:
  - report missing property;
  - ask Elena;
  - search nearby;
  - state worry/speculation if reckless accusation mode is explicitly modeled later.
```

### 16.8 Why-not explanation for blocked report

```text
Action blocked: report missing property.
Actor: Tomas.
Reason variant A: Tomas has not searched the expected container yet.
Reason variant B: office receiver unavailable; report may be delayed, not filed.
Reason variant C: actor is reporting hearsay but cannot name source or item.
Needed fields:
  - reporter;
  - item description or ID known to actor;
  - expected location/source;
  - observation/search basis for absence;
  - confidence/uncertainty.
```

### 16.9 Debug causal graph excerpt

```text
[DEBUG / NON-DIEGETIC]
seed_memory_001: Tomas saw coin_stack_01 in strongbox_tomas last evening
  -> belief_tomas_coin_expected_001

evt_item_relocated_014: actor_mara moved coin_stack_01 from strongbox_tomas to small_cache_mara
  -> truth coin location changed
  -> trace_created strongbox_dust_disturbed

evt_search_021: Tomas searched strongbox_tomas
  + belief_tomas_coin_expected_001
  -> evt_expected_absent_022
  -> belief_tomas_coin_missing_023
  -> report_missing_property affordance visible to Tomas

evt_speech_report_030: Tomas reports missing property to Anna
  -> evt_report_received_031
  -> record_incident_001 opened as claim, not truth

evt_suspicion_041: Elias suspects Rafi
  basis: wage_motive_claim + route_observation
  contradiction: no possession evidence
```

## 17. No-scripting review

Spec 0001 rejects the following shortcuts. A fixture, future implementation, or future spec that uses one of these has drifted from the first proof.

- Forced theft.
- Guaranteed discovery.
- Guaranteed accusation.
- Guaranteed institutional closure.
- Hidden culprit pointer.
- Player-conditioned events.
- Quest flags.
- Reward/completion state.
- Objective markers.
- Direct fixture mutation as ordinary play.
- Institutions reading truth.
- LLM text mutating state.
- Abstract balance replacing physical value token.
- Hidden drama director.
- Stale information auto-correcting.
- Actor view reading debug state.
- Records treated as proof without source/evidence.
- Suspicion treated as proof.
- Motive treated as action evidence.
- Possession treated as ownership.
- Access treated as permission.
- Private search treated as harmless default behavior.

Review question: could the same chain occur, or fail to occur, with no human attached? If not, it is probably scripted.

## 18. Validation mapping

This section maps this spec's material to live execution gate codes. It does not define gates, set pass/fail semantics, assign owners, or authorize phase entry.

| Gate code | Spec material that may be inspected | Definition source |
|---|---|---|
| `EVENT` | action/event vocabulary; physical state changes; item movement; access attempts; report/record events; replayable fixture chains. | `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` |
| `TRUTH-FIREWALL` | forbidden-collapse rules; anti-contamination fixture families; debug/embodied split; no hidden truth planning; source-bound claims. | `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` |
| `ACTOR-KNOWN` | actor roster beliefs, expectations, observations, memories, speech/report preconditions, action preconditions, and possession parity material. | `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` |
| `POSSESSION-PARITY` | possession parity fixtures and view sketches proving human control does not change actor-known filters. | `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` |
| `NO-HUMAN-ORDINARY-LIFE` | sleep/eat/work fixtures, ordinary-workday fixtures, routine/replan fixtures, and no-human day material. | `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` |
| `MISSING-PROPERTY` | `strongbox_tomas`, `coin_stack_01`, expectation contradiction, absence observation, report claim, suspicion/proof distinction. | `docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md` |
| `VIEW-DEBUG-SPLIT` | embodied view sketches, debug view sketch, actor notebook, local action view, diagnostic quarantine. | `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` |
| `REPLAY` | event vocabulary, fixture event chains, replay item location fixture, deterministic projection expectations. | `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` |
| `FIXTURE-NEGATIVE` | explicit negative assertions in every fixture family and the forbidden-collapse checklist. | `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` |

Content/schema validation material must also stay subordinate to `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` and observability/review material must stay subordinate to `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.

## 19. Certification and future-spec boundary

This spec is a live contract for the first-proof ontology and fixtures, but it is not itself a certification artifact.

- `P0-CERT` is the next major execution move named by the execution tier; this spec does not perform it.
- Historical specs `0005`–`0008` landed implementation work, then exposed gaps that drove the upper-tier overhaul; they remain history, not certification.
- Future implementation specs may cite this file for ontology, fixture names, and negative assertions, but gate pass/fail authority stays in `docs/2-execution/`.
- Future data/schema tickets may use this file as content contract material only when the higher-tier authoring and validation rules permit it.
- No future spec may treat the village story as a quest script, culprit script, clue chain, or drama-director outline.

## 20. Retread-prevention

Spec 0001 is the stable reference for the missing-property first-proof ontology and fixture contracts. Future specs should cite it rather than re-litigating:

- first-proof identity;
- actor roster baseline;
- core place/container/item contracts;
- ownership/custody/access/control/proof/belief distinctions;
- action/event/proposition vocabularies;
- fixture families and their positive/negative assertions;
- embodied/debug view-model boundaries;
- forbidden shortcuts;
- gate-reference mapping.

Future specs may refine implementation shape, validation tooling, executable test coverage, schemas, or UI layout only beneath higher-tier authority. They must not reopen notice boards, road threats, bounties, companions, quest objectives, LLM dialogue, or drama-director shortcuts as if the first proof had not already rejected them.
