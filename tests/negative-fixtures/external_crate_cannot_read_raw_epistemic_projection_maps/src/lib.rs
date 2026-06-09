use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::ids::ContentManifestId;

pub fn raw_belief_count() -> usize {
    let projection =
        EpistemicProjection::new(ContentManifestId::new("manifest_phase2a").unwrap());
    projection.beliefs_by_id.len()
}
