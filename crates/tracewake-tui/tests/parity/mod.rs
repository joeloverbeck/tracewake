pub mod census_actions;
pub mod census_families;
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
    pub registry_action_id: Option<&'static str>,
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
    HumanWaitOneTick,
    StartSleepThenAdvanceUntil { max_ticks: u64 },
    MoveWorkThenAdvanceUntil { max_ticks: u64 },
    StartSleepThenWaitConflict,
    SubmitSemanticAction { semantic_action_id: &'static str },
    SubmitRegistryAction { action_id: &'static str },
    ObserveQueryOnly { action_id: &'static str },
    AdvanceNoHuman,
    RenderNotebook,
    RenderDebugOverlay,
    RunNoHumanDay,
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
    let mut entries = census_actions::entries();
    entries.extend(census_families::entries());
    entries.sort_by_key(|entry| entry.key);
    entries
}
