use tracewake_core::scheduler::DeterministicScheduler;
use tracewake_core::time::SimTick;

pub fn assign_scheduler_frontier() {
    let mut scheduler = DeterministicScheduler::new(SimTick::ZERO);
    scheduler.current_tick = SimTick::new(99);
}
