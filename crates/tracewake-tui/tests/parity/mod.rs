#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityEntry {
    pub key: &'static str,
    pub ownership_scope: OwnershipScope,
    pub capability_class: CapabilityClass,
    pub surface_disposition: SurfaceDisposition,
    pub disposition_rationale: &'static str,
    pub fixture_ids: Vec<&'static str>,
    pub viewer_actor: &'static str,
    pub setup_operation: SetupOperation,
    pub typed_witness: Witness,
    pub rendered_witness: Option<Witness>,
    pub anti_leak_fixtures: Vec<&'static str>,
    pub replay_evidence: EvidenceFlag,
    pub no_human_evidence: EvidenceFlag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OwnershipScope {
    Base,
    FuturePack { namespace: &'static str },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityClass {
    SemanticAction,
    ActorObservableState,
    ActorObservableConsequence,
    NotebookRecordSurface,
    DebugOnlyInfrastructure,
    HeadlessInfrastructure,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceDisposition {
    Embodied,
    Notebook,
    DebugOnly,
    Headless,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetupOperation {
    BindViewer,
    SubmitSemanticAction { semantic_action_id: &'static str },
    AdvanceNoHuman,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Witness {
    pub kind: WitnessKind,
    pub assertion: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WitnessKind {
    TypedCausal,
    ActorKnowledge,
    RenderedText,
    GoldenReference,
    AntiLeakNegative,
    DebugQuarantine,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvidenceFlag {
    Required,
    NotApplicable { rationale: &'static str },
}

pub fn registry() -> Vec<CapabilityEntry> {
    let mut entries = vec![CapabilityEntry {
        key: "base.semantic_action.wait",
        ownership_scope: OwnershipScope::Base,
        capability_class: CapabilityClass::SemanticAction,
        surface_disposition: SurfaceDisposition::Embodied,
        disposition_rationale:
            "wait is an ordinary actor-facing semantic action rendered from the current view",
        fixture_ids: vec!["strongbox_001"],
        viewer_actor: "actor_tomas",
        setup_operation: SetupOperation::SubmitSemanticAction {
            semantic_action_id: "wait",
        },
        typed_witness: Witness {
            kind: WitnessKind::TypedCausal,
            assertion: "semantic_action_id=wait is present in the actor-filtered current view",
        },
        rendered_witness: Some(Witness {
            kind: WitnessKind::RenderedText,
            assertion: "rendered Actions section contains the wait semantic action",
        }),
        anti_leak_fixtures: Vec::new(),
        replay_evidence: EvidenceFlag::Required,
        no_human_evidence: EvidenceFlag::NotApplicable {
            rationale: "the exemplar is a human-submitted semantic action; no-human coverage is filled by later census entries",
        },
    }];
    entries.sort_by_key(|entry| entry.key);
    entries
}
