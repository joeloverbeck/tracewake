use std::collections::BTreeMap;

use super::{AgentSeed, PhysicalSeed};

use tracewake_core::actions::{ActionRegistry, Proposal, ProposalOrigin};
use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
use tracewake_core::ids::{ActionId, ActorId, ContentManifestId, PlaceId, ProposalId};
use tracewake_core::scheduler::no_human::{DayWindow, NoHumanStateMut};
use tracewake_core::state::{ActorBody, AgentState, NeedModelState, PhysicalState, PlaceState};
use tracewake_core::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedCase {
    pub seed: u64,
    pub sequence: Vec<GeneratedStep>,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedStep {
    pub window_id: String,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
    pub scheduled_wait_reason: String,
}

pub const GENERATIVE_SEEDS: &[u64] = &[0x17_00_00_01, 0x17_00_00_29, 0x17_00_00_57, 0x17_00_00_91];

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
    let mut tick = rng.range(0, 2);
    let mut sequence = Vec::new();
    for index in 0..4 {
        let duration = rng.range(1, 4);
        let start_tick = tick;
        let end_tick = start_tick + duration;
        sequence.push(GeneratedStep {
            window_id: format!("generated_{seed:x}_{index}"),
            start_tick: SimTick::new(start_tick),
            end_tick: SimTick::new(end_tick),
            scheduled_wait_reason: format!("generated_wait_seed_{seed:x}_{index}"),
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
        sequence,
        start_tick,
        end_tick,
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
    registry
}

pub fn initial_world(seed: u64) -> PhysicalState {
    let mut state = PhysicalSeed::from_state(&PhysicalState::empty(NeedModelState::new(5, 3)));
    let place_id = PlaceId::new("generated_home").unwrap();
    state.places_mut().insert(
        place_id.clone(),
        PlaceState::new(place_id.clone(), "Generated home"),
    );
    state
        .actors_mut()
        .insert(actor_id(), ActorBody::new(actor_id(), place_id));
    let _ = seed;
    state.build()
}

pub fn initial_agent_state(seed: u64) -> AgentState {
    let mut rng = Lcg::new(seed ^ 0xA11CE);
    let hunger = rng.range(100, 650) as i32;
    let fatigue = rng.range(100, 650) as i32;
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

pub fn scheduled_waits(case: &GeneratedCase) -> Vec<Proposal> {
    case.sequence
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let mut proposal = Proposal::new(
                ProposalId::new(format!("proposal_generated_{}_{index}", case.seed)).unwrap(),
                ProposalOrigin::Scheduler,
                Some(actor_id()),
                ActionId::new("wait").unwrap(),
                step.start_tick,
            );
            proposal
                .parameters
                .insert("reason".to_string(), step.scheduled_wait_reason.clone());
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
