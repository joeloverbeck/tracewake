# Genre-Agnostic Domain Model

## Core claim

Tracewake should be flavor-agnostic at the core and scenario-specific at the domain layer.

Fantasy, post-apocalyptic, Lovecraftian, historical, science-fiction, or low-magic scenarios should run on the same causal substrate: actors, beliefs, events, traces, places, needs, roles, norms, institutions, objects, affordances, speech acts, travel, and records.

## Core vs domain

The core owns event sourcing, action pipeline, entities/components, belief model, perception, speech acts, traces, institutions/norms, ordinary life, travel, LOD, UI view models, and replay/debugging.

A domain pack owns species/body types, technology level, magic/metaphysics, monsters/threats, institutions, laws/norms, occupations, items, object affordances, unique traces, diseases/conditions, speech style/cultural tags, terrain/place types, procedural constraints, and scenario seeds.

## Domain examples

### Low fantasy

Guilds, temples, minor magic, beasts, oaths, feudal authority, adventurers, herbal healing, tracks/blood/tokens as proof.

### Post-apocalyptic

Scarcity, scavenging, raiders, toxins, barter, water purification, fragile institutions, old-world artifacts, disease/injury risk.

### Lovecraftian

Forbidden knowledge, sanity/fear/corruption as belief/emotion modifiers, cult institutions, unreliable perception, hidden entities, rituals, cosmic traces, authorities suppressing truth.

Lovecraftian unreliability must still be causal. Madness is not permission for random nonsense.

## Magic and special senses

Magic is modeled causality.

A divination spell is an information channel with caster, cost, target, reliability, distortion, traces, institutional meaning, counters, and failure modes. It does not bypass belief provenance.

## Monsters and threats

A monster is not a quest target. It is an actor, group, process, or hazard with needs, territory, perception, behavior, traces, vulnerabilities, movement, origin if relevant, ecological/institutional consequences, and public beliefs that may be wrong.

A beast killing livestock should create tracks, missing animals, fear, reports, patrols, rumors, and economic effects.

## Technology differences

The same action schema can support different technologies.

Communication examples:

```text
Fantasy: messenger, town crier, magic mirror
Post-apocalyptic: radio, courier, signal flare
Lovecraftian: letter, telegram, dream, occult sign
```

All are information channels with delay, reliability, range, and distortion.

## Norm differences

Different domains define different norms: necromancy may be illegal, sacred, or bureaucratically licensed; theft may lead to jail, exile, debt bondage, or feud; monster hunting may require license or be tolerated vigilantism.

The core norm engine stays the same.

## Injury and healing

Core eventually supports injury, pain, bleeding, mobility impairment, unconsciousness, death, recovery, infection, and disease hooks. Domain packs define herbal medicine, surgery, magic, antibiotics, ritual purification, or alien technology.

## First domain recommendation

Use restrained low-fantasy or neutral medieval-ish village for the first slice. Notices, taverns, roads, authorities, and adventurers are intuitive; technology is simple; evidence is imperfect; institutions can be fallible without modern forensics.

Do not overbuild magic, species, religion, or combat first.

## Anti-patterns

Hard-coding fantasy into the kernel, magic as omniscience, monsters as quest targets, sanity as random behavior, post-apocalyptic scarcity as mere low item counts, and domain prose creating facts.
