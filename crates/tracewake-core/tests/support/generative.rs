use std::collections::BTreeMap;

use super::{AgentSeed, PhysicalSeed};

use tracewake_core::actions::{ActionRegistry, Proposal, ProposalOrigin};
use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, FoodSupplyId, PlaceId, ProposalId, SleepAffordanceId,
    WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::scheduler::no_human::{DayWindow, NoHumanStateMut};
use tracewake_core::state::{
    ActorBody, AgentState, FoodSupplyState, NeedModelState, PhysicalState, PlaceState,
    SleepAffordanceState, WorkplaceState,
};
use tracewake_core::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedCase {
    pub seed: u64,
    pub mask: ActionMask,
    pub sequence: Vec<GeneratedStep>,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedStep {
    pub kind: GeneratedActionKind,
    pub window_id: String,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
    pub scheduled_wait_reason: String,
    pub duration_ticks: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeneratedActionKind {
    Wait,
    Eat,
    Sleep,
    Work,
}

impl GeneratedActionKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Wait => "wait",
            Self::Eat => "eat",
            Self::Sleep => "sleep",
            Self::Work => "work_block",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionMask {
    pub wait: bool,
    pub eat: bool,
    pub sleep: bool,
    pub work: bool,
    pub interrupt_sleep: bool,
}

impl ActionMask {
    pub const fn stable_id(self) -> &'static str {
        match (
            self.wait,
            self.eat,
            self.sleep,
            self.work,
            self.interrupt_sleep,
        ) {
            (true, true, false, false, false) => "wait_eat",
            (true, false, false, true, false) => "wait_work",
            (true, false, true, false, false) => "wait_sleep",
            (true, false, true, false, true) => "wait_sleep_interrupt",
            (false, true, false, true, false) => "eat_work",
            (true, true, true, true, false) => "all",
            _ => "mixed",
        }
    }

    fn allowed_kinds(self) -> Vec<GeneratedActionKind> {
        let mut kinds = Vec::new();
        if self.wait {
            kinds.push(GeneratedActionKind::Wait);
        }
        if self.eat {
            kinds.push(GeneratedActionKind::Eat);
        }
        if self.sleep {
            kinds.push(GeneratedActionKind::Sleep);
        }
        if self.work {
            kinds.push(GeneratedActionKind::Work);
        }
        kinds
    }
}

pub const GENERATIVE_SEEDS: &[u64] = &[
    0x18_00_00_10,
    0x18_00_00_11,
    0x18_00_00_12,
    0x18_00_00_13,
    0x18_00_00_14,
    0x18_00_00_15,
    0x18_00_00_29,
    0x18_00_00_57,
];

#[derive(Clone, Debug)]
pub struct Lcg {
    state: u64,
}

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    pub fn range(&mut self, min: u64, max_inclusive: u64) -> u64 {
        min + (self.next_u64() % (max_inclusive - min + 1))
    }
}

pub fn generate_case(seed: u64) -> GeneratedCase {
    let mut rng = Lcg::new(seed);
    let mask = mask_for_seed(seed);
    let allowed_kinds = mask.allowed_kinds();
    let mut tick = rng.range(0, 2);
    let mut sequence = Vec::new();
    let step_count = rng.range(3, 6);
    for index in 0..step_count {
        let mut kind = allowed_kinds[(rng.range(0, allowed_kinds.len() as u64 - 1)) as usize];
        if index == step_count - 1 && mask.sleep {
            kind = GeneratedActionKind::Sleep;
        } else if index == step_count - 1 && mask.work {
            kind = GeneratedActionKind::Work;
        }
        let duration = match kind {
            GeneratedActionKind::Sleep | GeneratedActionKind::Work => rng.range(2, 4),
            GeneratedActionKind::Wait | GeneratedActionKind::Eat => rng.range(1, 2),
        };
        let start_tick = tick;
        let end_tick = start_tick + duration;
        sequence.push(GeneratedStep {
            kind,
            window_id: format!("generated_{seed:x}_{}_{}", mask.stable_id(), index),
            start_tick: SimTick::new(start_tick),
            end_tick: SimTick::new(end_tick),
            scheduled_wait_reason: format!("generated_wait_seed_{seed:x}_{index}"),
            duration_ticks: duration,
        });
        tick = end_tick + rng.range(0, 2);
    }
    let start_tick = sequence
        .first()
        .map(|step| step.start_tick)
        .unwrap_or(SimTick::ZERO);
    let end_tick = sequence
        .last()
        .map(|step| step.end_tick)
        .unwrap_or(SimTick::ZERO);
    GeneratedCase {
        seed,
        mask,
        sequence,
        start_tick,
        end_tick,
    }
}

fn mask_for_seed(seed: u64) -> ActionMask {
    match seed & 0x0f {
        0 => ActionMask {
            wait: true,
            eat: true,
            sleep: false,
            work: false,
            interrupt_sleep: false,
        },
        1 => ActionMask {
            wait: true,
            eat: false,
            sleep: false,
            work: true,
            interrupt_sleep: false,
        },
        2 => ActionMask {
            wait: true,
            eat: false,
            sleep: true,
            work: false,
            interrupt_sleep: false,
        },
        3 => ActionMask {
            wait: true,
            eat: false,
            sleep: true,
            work: false,
            interrupt_sleep: true,
        },
        4 => ActionMask {
            wait: false,
            eat: true,
            sleep: false,
            work: true,
            interrupt_sleep: false,
        },
        _ => ActionMask {
            wait: true,
            eat: true,
            sleep: true,
            work: true,
            interrupt_sleep: false,
        },
    }
}

pub fn actor_id() -> ActorId {
    ActorId::new("actor_generated").unwrap()
}

pub fn content_manifest_id(seed: u64) -> ContentManifestId {
    ContentManifestId::new(format!("manifest_generated_{seed:x}")).unwrap()
}

pub fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

pub fn initial_world(seed: u64) -> PhysicalState {
    let mut state = PhysicalSeed::from_state(&PhysicalState::empty(NeedModelState::new(5, 3)));
    let home_place_id = place_id();
    state.places_mut().insert(
        home_place_id.clone(),
        PlaceState::new(home_place_id.clone(), "Generated home"),
    );
    state
        .actors_mut()
        .insert(actor_id(), ActorBody::new(actor_id(), home_place_id));
    let food_supply_id = food_supply_id();
    state.food_supplies_mut().insert(
        food_supply_id.clone(),
        FoodSupplyState::new(food_supply_id, Location::AtPlace(place_id()), 24, 220),
    );
    let sleep_affordance_id = sleep_affordance_id();
    state.sleep_affordances_mut().insert(
        sleep_affordance_id.clone(),
        SleepAffordanceState::new(sleep_affordance_id, place_id(), 3, 80, 8),
    );
    let workplace_id = workplace_id();
    let mut workplace = WorkplaceState::new(
        workplace_id.clone(),
        place_id(),
        2,
        6,
        4,
        850,
        850,
        "generated_work",
    );
    workplace.assigned_actor_ids.insert(actor_id());
    state.workplaces_mut().insert(workplace_id, workplace);
    let _ = seed;
    state.build()
}

pub fn initial_agent_state(seed: u64) -> AgentState {
    let mut rng = Lcg::new(seed ^ 0xA11CE);
    let mask = mask_for_seed(seed);
    let hunger = if mask.interrupt_sleep {
        930
    } else {
        rng.range(100, 500) as i32
    };
    let fatigue = rng.range(100, 500) as i32;
    let safety = rng.range(0, 200) as i32;
    let mut agent_state = AgentSeed::default();
    agent_state.needs_by_actor_mut().insert(
        actor_id(),
        BTreeMap::from([
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, hunger, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, fatigue, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, safety, NeedChangeCause::FixtureInitial),
            ),
        ]),
    );
    agent_state.build()
}

pub fn place_id() -> PlaceId {
    PlaceId::new("generated_home").unwrap()
}

pub fn food_supply_id() -> FoodSupplyId {
    FoodSupplyId::new("food_generated_stew").unwrap()
}

pub fn sleep_affordance_id() -> SleepAffordanceId {
    SleepAffordanceId::new("sleep_generated_bed").unwrap()
}

pub fn workplace_id() -> WorkplaceId {
    WorkplaceId::new("workplace_generated").unwrap()
}

pub fn windows(case: &GeneratedCase) -> Vec<DayWindow> {
    case.sequence
        .iter()
        .map(|step| DayWindow {
            window_id: step.window_id.clone(),
            start_tick: step.start_tick,
            end_tick: step.end_tick,
        })
        .collect()
}

pub fn scheduled_proposals(case: &GeneratedCase) -> Vec<Proposal> {
    case.sequence
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let mut proposal = Proposal::new(
                ProposalId::new(format!(
                    "proposal_generated_{}_{}_{}",
                    case.seed,
                    step.kind.stable_id(),
                    index
                ))
                .unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id()),
                ActionId::new(step.kind.stable_id()).unwrap(),
                step.start_tick,
            );
            match step.kind {
                GeneratedActionKind::Wait => {
                    proposal
                        .parameters
                        .insert("reason".to_string(), step.scheduled_wait_reason.clone());
                }
                GeneratedActionKind::Eat => {
                    proposal.target_ids.push(food_supply_id().to_string());
                }
                GeneratedActionKind::Sleep => {
                    proposal.parameters.insert(
                        "sleep_affordance_id".to_string(),
                        sleep_affordance_id().to_string(),
                    );
                    proposal.parameters.insert(
                        "duration_ticks".to_string(),
                        step.duration_ticks.to_string(),
                    );
                }
                GeneratedActionKind::Work => {
                    proposal.target_ids.push(workplace_id().to_string());
                }
            }
            proposal
        })
        .collect()
}

pub fn no_human_state_mut<'a>(
    physical: &'a mut PhysicalState,
    agent: &'a mut AgentState,
) -> NoHumanStateMut<'a> {
    NoHumanStateMut { physical, agent }
}
