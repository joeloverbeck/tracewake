use std::collections::{BTreeMap, BTreeSet};

use tracewake_core::actions::ActionRegistry;
use tracewake_core::content::load::LoadedFixture;
use tracewake_core::content::manifest::ContentManifest;
use tracewake_core::content::schema::{FixtureSchema, FixtureScope, NeedModelSchema};
use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::events::log::EventLog;
use tracewake_core::ids::{ContentManifestId, ContentVersion, FixtureId, SchemaVersion};
use tracewake_core::runtime::{LoadedWorldRuntime, ValidatedLoadedWorldBootstrap};
use tracewake_core::state::{AgentState, NeedModelState, PhysicalState};
use tracewake_core::time::SimTick;

pub fn fabricate_loaded_world_bootstrap() {
    let manifest_id = ContentManifestId::new("fixture_manifest").unwrap();
    let physical_state = PhysicalState::from_validated_content_parts(
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
    let agent_state = AgentState::from_validated_content_parts(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let manifest = ContentManifest::new(
        manifest_id.clone(),
        FixtureId::new("fixture_external").unwrap(),
        SchemaVersion::new("schema_v1").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
        Vec::new(),
    );
    let loaded = LoadedFixture {
        fixture: FixtureSchema {
            fixture_id: FixtureId::new("fixture_external").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            fixture_scope: FixtureScope::Phase1,
            need_model: NeedModelSchema {
                awake_hunger_delta_per_tick: 5,
                awake_fatigue_delta_per_tick: 3,
            },
            actors: Vec::new(),
            places: Vec::new(),
            doors: Vec::new(),
            containers: Vec::new(),
            items: Vec::new(),
            affordances: Vec::new(),
            initial_beliefs: Vec::new(),
            initial_needs: Vec::new(),
            homes: Vec::new(),
            sleep_places: Vec::new(),
            food_supplies: Vec::new(),
            known_food_sources: Vec::new(),
            workplaces: Vec::new(),
            routine_templates: Vec::new(),
            routine_assignments: Vec::new(),
            day_windows: Vec::new(),
        },
        manifest,
        canonical_world: physical_state.clone(),
        canonical_agent_state: agent_state.clone(),
        epistemic_projection: EpistemicProjection::new(manifest_id.clone()),
        seed_event_log: EventLog::new(),
    };
    let _loaded_bootstrap = loaded.into_runtime_bootstrap(ActionRegistry::new());
    let sealed = ValidatedLoadedWorldBootstrap::from_validated_content(
        ActionRegistry::new(),
        physical_state,
        agent_state,
        EventLog::new(),
        EpistemicProjection::new(manifest_id.clone()),
        manifest_id.clone(),
        FixtureId::new("fixture_external").unwrap(),
        ContentVersion::new("content_v1").unwrap(),
    );
    let _runtime = LoadedWorldRuntime::from_bootstrap(sealed, SimTick::ZERO);
}
