# Ordinary Life and Economy

## Core claim

Adventure is the disruption of ordinary life. Therefore ordinary life must be simulated before adventure systems become meaningful.

A burglary matters because there are homes, locks, property expectations, neighbors, sleeping schedules, social trust, authorities, records, hiding places, and consequences. A bandit threat matters because there are roads, trade, travelers, taxes, food, wages, guards, and families.

Without ordinary life, emergent content becomes random violence in a toy box.

## The substrate of normality

The first village should support these ordinary processes:

- sleep and waking;
- meals and hunger;
- work obligations;
- domestic storage;
- property ownership;
- buying and selling;
- conversation and gossip;
- visiting homes/taverns/workplaces;
- road travel;
- simple production chain;
- authority office hours;
- public notice maintenance;
- household relationships;
- fear and routine disruption.

## Routines as intentions

Do not implement schedules as hard scripts.

Bad:

```text
08:00 blacksmith goes to forge.
12:00 blacksmith goes to tavern.
20:00 blacksmith goes home.
```

Better:

```text
The blacksmith forms a workday intention because:
- she believes today is a workday;
- she has outstanding orders;
- she needs income;
- she has tools and fuel;
- the forge is reachable;
- she is healthy enough;
- no stronger event has interrupted her.
```

This allows work to fail naturally when coal is missing, the door is blocked, the blacksmith is ill, a child is missing, or guards demand testimony.

## Needs

Start with a small set.

```yaml
Needs:
  hunger:
    normal_actions: [eat_at_home, buy_food, eat_at_tavern]
    desperate_actions: [beg, steal_food]
  fatigue:
    normal_actions: [sleep_at_home, nap]
    desperate_actions: [sleep_in_public, collapse]
  safety:
    normal_actions: [avoid_threat, seek_guard, stay_home]
    desperate_actions: [flee_town, arm_self]
```

Needs should create pressure, not robotic compulsion. A guard can remain on duty while hungry. A terrified worker may abandon work.

## Ownership and possession

Separate ownership from possession.

```yaml
Ownership:
  legal_owner: actor_tomas
  possessor: actor_mara
  expected_location: item_strongbox_01
  permitted_users: [actor_tomas, actor_elena]
```

This enables:

- theft;
- borrowing;
- custody;
- evidence handling;
- inheritance;
- misplaced items;
- contested ownership;
- confiscation;
- ransom;
- found property.

Agents should have expectations about where important property is.

## Domestic spaces

Homes must matter.

Homes provide:

- sleep location;
- storage;
- privacy;
- family interaction;
- social boundary;
- property boundary;
- crime scene substrate;
- invitation/trespass logic;
- household routines.

A door is not scenery. A door controls access, sound, privacy, line of sight, and legality.

## Work

Work should produce goods, services, money, records, and social expectations.

Example miller workday:

```text
wake -> eat -> go to mill -> open mill -> process grain if grain present
-> receive customers -> record debts/payments -> close mill -> return home
```

Failure cases:

- grain not delivered;
- millstone broken;
- customer refuses payment;
- worker ill;
- bandits block road;
- authority summons worker as witness;
- stolen money prevents tax payment.

## Minimal economy

Do not build a full economy first. Build a causal economy.

Initial goods:

- food;
- coin;
- grain/flour/bread;
- tools;
- paper/ink;
- weapons;
- stolen valuables.

Initial actors:

- producer;
- merchant/tavern keeper;
- authority office;
- household;
- bandits.

Initial flows:

```text
farmer -> grain -> miller -> flour -> baker/tavern -> food -> villagers
travelers -> road tolls/trade -> market -> tax -> authority
bandits -> stolen goods -> fence/hidden stash -> threat reports
```

The goal is not price realism. The goal is consequence.

## Markets

Markets are information hubs as much as trade hubs.

Market events:

- price change;
- stock shortage;
- public argument;
- rumor spread;
- theft accusation;
- notice read;
- traveler arrival;
- guard patrol;
- debt collection.

A market should generate observations and social links.

## Taverns

A tavern should be the first social simulation testbed.

Tavern affordances:

- eat;
- drink;
- listen;
- gossip;
- hire;
- gamble if desired later;
- hide in crowd;
- start fight;
- meet travelers;
- read notices copied from board;
- observe who is absent.

Taverns make information move.

## Roads

Roads are causal arteries.

Road events:

- travel;
- delay;
- ambush;
- corpse discovery;
- merchant avoidance;
- patrol;
- rumor import/export;
- supply shortage;
- bounty motivation.

If bandits attack a road, consequences should reach town through trade and testimony.

## Bandit economy

Bandits should not be spawn points. They need simple motives:

- hunger;
- greed;
- fear of authority;
- need for supplies;
- group loyalty;
- hiding;
- fencing stolen goods;
- choosing targets based on risk.

Bandits should:

- scout;
- attack vulnerable travelers;
- flee strong patrols;
- consume supplies;
- hide loot;
- react to bounties if they learn of them;
- move camp if threatened.

A bandit camp is an actor group with needs, not a dungeon label.

## Disruption

Ordinary routines should be interruptible by:

- theft;
- violence;
- fire;
- illness;
- missing person;
- guard questioning;
- family crisis;
- blocked road;
- shortage;
- weather;
- death;
- public notice;
- rumor.

Disruption should be visible in the town:

- closed shop;
- late delivery;
- crowd at office;
- guard absence;
- hungry household;
- price increase;
- tavern panic;
- gossip spike.

## Minimal ordinary-life implementation

For the vertical slice, build:

- 30–80 agents;
- 10–20 buildings;
- homes, workplace, tavern, authority office, notice board;
- sleep/eat/work/social routines;
- property storage;
- simple coin economy;
- one food chain;
- one road;
- one bandit group;
- one authority office;
- rumor spread at tavern/market;
- routine interruption.

## Anti-patterns

- Decorative NPCs with no needs or homes.
- Shops open forever.
- Items with no owner.
- Bandits who exist only to be killed.
- Work that produces no records, money, goods, or obligations.
- Townspeople who do not react to missing people, closed shops, or violence.
- Economy systems that are numerically elaborate but causally irrelevant.
