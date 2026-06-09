use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::ids::ContentManifestId;

pub fn raw_notebook_actor_count() -> usize {
    let projection =
        EpistemicProjection::new(ContentManifestId::new("manifest_phase2a").unwrap());
    projection.notebook_entries_by_actor.len()
}
