use std::collections::{BTreeMap, BTreeSet};

use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{ContentManifestId, ContentVersion, FixtureId};
use tracewake_core::runtime::{
    LoadedWorldBootstrap, LoadedWorldRuntime, ValidatedLoadedWorldBootstrap,
};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

pub fn fabricate_loaded_world_bootstrap() {
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let physical_state = PhysicalState::from_seed_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        NeedModelState::new(5, 3),
    );
    let agent_state = AgentState::from_seed_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let raw_bootstrap = LoadedWorldBootstrap::from_loaded_state(
        ActionRegistry::new(),
        physical_state.clone(),
        agent_state.clone(),
        EventLog::new(),
        EpistemicProjection::new(manifest_id.clone()),
        manifest_id.clone(),
        FixtureId::new("fixture_external").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
    );
    let sealed = ValidatedLoadedWorldBootstrap::from_loaded_state(raw_bootstrap);
    let _runtime = LoadedWorldRuntime::from_bootstrap(sealed, SimTick::ZERO);
}
