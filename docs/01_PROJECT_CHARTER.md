# Project Charter

## Working title

Tracewake.

The name points to the project’s real subject: actions leave wakes, wakes leave traces, and agents later interpret those traces through partial belief.

## One-sentence pitch

Tracewake is a causality-first regional life simulation where the player intervenes by temporarily controlling ordinary agents inside a fallible world of beliefs, records, routines, institutions, travel, and stale information.

## Product fantasy

You walk into a town. People are sleeping, working, hiring help, gossiping, guarding roads, lying to spouses, delivering flour, hiding evidence, tending wounds, posting notices, reading rumors, and pursuing plans unrelated to you.

You learn from a notice board that something has been killing livestock north of the road. You ask locals. You hire two adventurers. You travel with them. When you arrive, the creature is gone. Maybe it migrated. Maybe someone else killed it. Maybe the notice was wrong. Maybe the locals lied. You are not given a quest correction. You investigate, adapt, exploit, or walk away.

## Long-term fantasy

Tracewake eventually supports a region that can be simulated for years before play. People age, die, migrate, marry, feud, inherit, lose work, join institutions, abandon homes, build shops, spread disease, change routes, distort rumors, and leave public records. The player enters the current state of that region.

## Design pillars

1. **Causality before drama.** Events happen because state, actors, institutions, environment, or regional processes made them happen.
2. **Belief before truth.** Agents act from subjective beliefs, not ground truth.
3. **The player is not special.** The human is a controller binding, not a privileged world entity.
4. **Every player action is agent-possible.** UI clarity is allowed; exclusive verbs are not.
5. **Institutions are fallible machines.** Law, commerce, guilds, households, temples, gangs, and offices act through roles, procedures, records, resources, delay, bias, and failure.
6. **Ordinary life is the substrate.** Adventure matters only if normal life is real enough to be disrupted.
7. **Quests are projections.** The engine stores incidents, requests, contracts, obligations, records, notices, rumors, and opportunities.
8. **Language is a surface.** LLMs may render and propose. The kernel validates and decides.
9. **The first UI is a serious TUI.** It must be playable, truthful, and reusable in architecture.
10. **Scale grows by honest abstraction.** LOD summaries preserve causality; they do not hide a drama director.

## Non-goals

Tracewake is not a procedural quest generator, a RimWorld-like storyteller, a Skyrim radiant quest replacement, a generic LLM NPC demo, an authored narrative RPG, a colony sim centered on a player faction, or a large map full of fake schedules.

## Required first miracle

For any surprising event, the system should answer:

- what happened;
- why it was possible;
- who caused it;
- who observed it;
- what traces it left;
- who believes what;
- who is wrong;
- what institution recorded or ignored it;
- what later events became possible because of it.

## Prototype success

A successful prototype runs a small village for several simulated days in a playable TUI and produces:

- theft discovered through expectation violation;
- uncertain testimony becoming rumor;
- an authority opening a case from partial information;
- a public notice posted for a threat;
- a stale notice acted on by someone;
- a wrong suspect questioned for legible reasons;
- a work routine disrupted by modeled causes;
- possession switching without knowledge leakage.

## Strong opinion

Do not build combat, terrain, magic, world generation, full LLM chat, or a graphical UI first. Build a town whose people can be wrong for the right reasons.
