# Ordinary Life, Economy, and Settlements

## Core claim

Adventure is the disruption of ordinary life. Ordinary life must be simulated before adventure becomes meaningful.

A theft matters because homes, locks, expectations, neighbors, sleep, social trust, authorities, records, and hiding places exist. A threat matters because roads, trade, travelers, food, wages, guards, families, and rumors exist.

## Settlement substrate

The first settlement supports sleep, meals, work, storage, ownership, buying/selling, gossip, visiting homes/taverns/workplaces/offices, road travel, simple production chains, office hours, notice maintenance, household relationships, fear, and routine disruption.

## Routines as intentions

Bad:

```text
08:00 blacksmith goes to forge.
```

Better:

```text
The blacksmith works because she believes today is a workday,
has orders, needs income, has tools and fuel, can reach the forge,
and has no stronger interruption.
```

This allows work to fail when coal is missing, a child is sick, a guard summons her, the road is blocked, or the key is stolen.

## Needs

Start with hunger, fatigue, and safety.

```yaml
Needs:
  hunger:
    ordinary_actions: [eat_at_home, buy_food, eat_at_tavern]
    desperate_actions: [beg, steal_food]
  fatigue:
    ordinary_actions: [sleep_at_home, nap]
    desperate_actions: [sleep_in_public, collapse]
  safety:
    ordinary_actions: [avoid_threat, seek_guard, stay_home]
    desperate_actions: [flee_region, arm_self]
```

Needs create pressure, not compulsion.

## Ownership and possession

Separate legal owner, possessor, custodian, expected location, and permitted users.

This enables theft, borrowing, custody, evidence handling, inheritance, contested ownership, confiscation, ransom, and found property.

## Domestic spaces

Homes provide sleep, storage, privacy, kinship interaction, permission boundaries, property boundaries, crime scenes, invitation/trespass logic, line of sight, and sound. A door controls access, sound, privacy, legality, and evidence.

## Work

Work produces goods, money, records, services, and social expectations.

Example miller workday:

```text
wake -> eat -> go to mill -> open mill -> process grain
-> receive customers -> record debts/payments -> close mill -> return home
```

Failure cases include missing grain, broken millstone, nonpaying customer, illness, bandit road closure, witness summons, and stolen tax money.

## Minimal causal economy

Initial goods: food, coin, grain/flour/bread, tools, paper/ink, weapons, medicine/bandages, stolen valuables.

Initial flows:

```text
farmer -> grain -> miller -> flour -> baker/tavern -> food -> villagers
travelers -> trade -> market -> tax -> authority
bandits -> stolen goods -> hidden stash/fence -> threat reports
authority -> funds -> bounty/payment -> adventurers
```

The goal is consequence, not price realism.

## Shops and services

A shop is a location, owner, inventory, hours, prices/beliefs, records/debts, supply dependencies, reputation, and vulnerability to theft, illness, closure, and fear.

## Taverns

Taverns allow eating, drinking, listening, gossiping, hiring, hiding in a crowd, meeting travelers, observing absence, overhearing disputes, and reading copied notices. Taverns move information.

## Roads

Road events include travel, delay, ambush, corpse discovery, patrol, caravan movement, disease spread, rumor import/export, supply shortage, and bounty motivation.

## Adventurers and companions

Adventurers are ordinary agents with motives, rates, fear, reputation, equipment, relationships, risk thresholds, payment expectations, and independent goals. They may accept, refuse, betray, flee, demand proof, split loot, or insist on rest.

## Settlement growth and decline

Long-term simulation should eventually allow births/deaths, immigration/emigration, shop openings/closures, construction, road safety changes, shortages, disease, reputation shifts, abandoned homes, and population changes.

## First settlement

Build 30–60 named agents at varied LOD, 8–12 homes, tavern, market/notice board, authority office, mill/workshop, one road, one nearby threat site, food/coin/tool flows, work/sleep/eat/social routines, property storage, rumor spread, and routine interruption.
