use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::ContentManifestId;
use tracewake_core::runtime::{LoadedWorldBootstrap, LoadedWorldRuntime, RuntimeInitialState};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};

pub fn mutate_loaded_runtime_fields() {
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let _forbidden_initial_state: Option<RuntimeInitialState> = None;
    let bootstrap = LoadedWorldBootstrap::from_loaded_state(
        ActionRegistry::new(),
        PhysicalState::empty(NeedModelState::new(5, 3)),
        AgentState::default(),
        EventLog::new(),
        EpistemicProjection::new(manifest_id.clone()),
        manifest_id,
    );
    let mut runtime =
        LoadedWorldRuntime::from_bootstrap(bootstrap, tracewake_core::time::SimTick::ZERO);

    runtime.physical_state = PhysicalState::empty(NeedModelState::new(5, 3));
}
