pub mod runner;
pub mod scenario;

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
    pub actor_knowledge_witness: Witness,
    pub rendered_witness: Option<Witness>,
    pub golden_path: Option<&'static str>,
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
    let mut entries = vec![
        CapabilityEntry {
            key: "base.epistemic.why_not.door_closed",
            ownership_scope: OwnershipScope::Base,
            capability_class: CapabilityClass::ActorObservableState,
            surface_disposition: SurfaceDisposition::Embodied,
            disposition_rationale:
                "blocked movement renders an actor-safe why-not without debug or hidden-truth leakage",
            fixture_ids: vec!["door_access_001"],
            viewer_actor: "actor_sena",
            setup_operation: SetupOperation::SubmitSemanticAction {
                semantic_action_id: "move.to.back_room",
            },
            typed_witness: Witness {
                kind: WitnessKind::TypedCausal,
                assertion: "move.to.back_room is present as a typed semantic action before submission",
            },
            actor_knowledge_witness: Witness {
                kind: WitnessKind::ActorKnowledge,
                assertion: "door_closed_blocks_movement is explained with actor-visible facts only",
            },
            rendered_witness: Some(Witness {
                kind: WitnessKind::RenderedText,
                assertion: "rendered why-not contains the door blocker and omits debug-only context",
            }),
            golden_path: Some("crates/tracewake-tui/tests/goldens/base_epistemic_why_not_door_closed.txt"),
            anti_leak_fixtures: vec!["door_access_001"],
            replay_evidence: EvidenceFlag::Required,
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale: "the exemplar is a submitted rejection surface; no-human coverage is filled by later census entries",
            },
        },
        CapabilityEntry {
            key: "base.semantic_action.wait",
            ownership_scope: OwnershipScope::Base,
            capability_class: CapabilityClass::SemanticAction,
            surface_disposition: SurfaceDisposition::Embodied,
            disposition_rationale:
                "wait is an ordinary actor-facing semantic action rendered from the current view",
            fixture_ids: vec!["strongbox_001"],
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitSemanticAction {
                semantic_action_id: "wait.1_tick",
            },
            typed_witness: Witness {
                kind: WitnessKind::TypedCausal,
                assertion: "semantic_action_id=wait.1_tick is present in the actor-filtered current view",
            },
            actor_knowledge_witness: Witness {
                kind: WitnessKind::ActorKnowledge,
                assertion: "the current view is sealed to actor_tomas holder-known context before rendering",
            },
            rendered_witness: Some(Witness {
                kind: WitnessKind::RenderedText,
                assertion: "rendered Actions section contains the wait semantic action",
            }),
            golden_path: Some("crates/tracewake-tui/tests/goldens/base_semantic_action_wait.txt"),
            anti_leak_fixtures: Vec::new(),
            replay_evidence: EvidenceFlag::Required,
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale: "the exemplar is a human-submitted semantic action; no-human coverage is filled by later census entries",
            },
        },
    ];
    entries.sort_by_key(|entry| entry.key);
    entries
}
