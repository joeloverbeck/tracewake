use super::{
    CapabilityClass, CapabilityEntry, EvidenceFlag, OwnershipScope, SetupOperation,
    SurfaceDisposition, Witness, WitnessKind,
};

pub const FAMILY_KEYS: [&str; 6] = [
    "base.family.debug_quarantine",
    "base.family.epistemic_filtering",
    "base.family.needs_routines",
    "base.family.no_human_autonomy",
    "base.family.notebook_leads",
    "base.family.rejection_why_not",
];

pub fn entries() -> Vec<CapabilityEntry> {
    let mut entries = vec![
        family_entry(FamilyEntrySpec {
            key: "base.family.debug_quarantine",
            class: CapabilityClass::DebugOnlyInfrastructure,
            disposition: SurfaceDisposition::DebugOnly,
            rationale:
                "debug overlay is explicitly non-diegetic and cannot satisfy embodied parity",
            fixture_id: "debug_attach_001",
            viewer_actor: "actor_iris",
            setup_operation: SetupOperation::RenderDebugOverlay,
            typed_assertion: "debug attach creates debug availability without mutating world state",
            actor_knowledge_assertion:
                "the embodied view remains actor-filtered while debug overlay is separately gated",
            rendered_assertion: "debug overlay marks knowledge context as non-diegetic",
            rendered_kind: WitnessKind::DebugQuarantine,
            golden_path: "crates/tracewake-tui/tests/goldens/family_debug_quarantine.txt",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale:
                    "debug quarantine is a debug-only surface, not an autonomous schedule run",
            },
        }),
        family_entry(FamilyEntrySpec {
            key: "base.family.epistemic_filtering",
            class: CapabilityClass::ActorObservableState,
            disposition: SurfaceDisposition::Embodied,
            rationale: "embodied state must be filtered through actor-known context",
            fixture_id: "hidden_food_unknown_route_001",
            viewer_actor: "actor_mara",
            setup_operation: SetupOperation::BindViewer,
            typed_assertion:
                "hidden physical food and route truth exist outside actor-known context",
            actor_knowledge_assertion:
                "actor_mara's current view is sealed to actor-known local context",
            rendered_assertion:
                "rendered embodied view omits hidden_workshop and food_hidden_pantry",
            rendered_kind: WitnessKind::RenderedText,
            golden_path: "crates/tracewake-tui/tests/goldens/family_epistemic_filtering.txt",
            anti_leak_fixtures: vec!["hidden_food_unknown_route_001"],
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale: "epistemic filtering family is a possessed actor view",
            },
        }),
        family_entry(FamilyEntrySpec {
            key: "base.family.needs_routines",
            class: CapabilityClass::ActorObservableState,
            disposition: SurfaceDisposition::Embodied,
            rationale: "needs and routine summaries are ordinary actor-facing state",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::BindViewer,
            typed_assertion: "need bands and active routine are typed view-model fields",
            actor_knowledge_assertion:
                "actor_tomas sees only his own needs, intention, and routine summary",
            rendered_assertion: "rendered embodied view includes Needs, Intention, and Routine",
            rendered_kind: WitnessKind::RenderedText,
            golden_path: "crates/tracewake-tui/tests/goldens/family_needs_routines.txt",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::Required,
        }),
        family_entry(FamilyEntrySpec {
            key: "base.family.no_human_autonomy",
            class: CapabilityClass::ActorObservableConsequence,
            disposition: SurfaceDisposition::Embodied,
            rationale:
                "no-human autonomy is accepted through observable consequences, not private plans",
            fixture_id: "no_human_day_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::RunNoHumanDay,
            typed_assertion: "no-human day produces typed events and metrics through the scheduler",
            actor_knowledge_assertion:
                "post-run embodied state renders actor-filtered consequences only",
            rendered_assertion:
                "rendered post-run view exposes observable state without private plan truth",
            rendered_kind: WitnessKind::RenderedText,
            golden_path: "crates/tracewake-tui/tests/goldens/family_no_human_autonomy.txt",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::Required,
        }),
        family_entry(FamilyEntrySpec {
            key: "base.family.notebook_leads",
            class: CapabilityClass::NotebookRecordSurface,
            disposition: SurfaceDisposition::Notebook,
            rationale:
                "notebook leads are actor-owned records rendered outside embodied command text",
            fixture_id: "expectation_contradiction_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::RenderNotebook,
            typed_assertion: "notebook view is built from the actor's epistemic projection records",
            actor_knowledge_assertion:
                "notebook content is scoped to actor_tomas's holder-known records",
            rendered_assertion:
                "rendered notebook records expectation/absence without culprit truth",
            rendered_kind: WitnessKind::GoldenReference,
            golden_path: "crates/tracewake-tui/tests/goldens/family_notebook_leads.txt",
            anti_leak_fixtures: vec!["expectation_contradiction_001"],
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale: "notebook family is a possessed actor record surface",
            },
        }),
        family_entry(FamilyEntrySpec {
            key: "base.family.rejection_why_not",
            class: CapabilityClass::ActorObservableConsequence,
            disposition: SurfaceDisposition::Embodied,
            rationale: "rejected local actions must leave an actor-safe why-not",
            fixture_id: "door_access_001",
            viewer_actor: "actor_sena",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "move" },
            typed_assertion: "closed-door movement rejection produces typed reason codes",
            actor_knowledge_assertion:
                "door_closed_blocks_movement is explained with actor-visible facts only",
            rendered_assertion:
                "rendered embodied view includes a why-not without validator-only truth",
            rendered_kind: WitnessKind::RenderedText,
            golden_path: "crates/tracewake-tui/tests/goldens/family_rejection_why_not.txt",
            anti_leak_fixtures: vec!["door_access_001"],
            no_human_evidence: EvidenceFlag::NotApplicable {
                rationale: "why-not family is a human-submitted rejection surface",
            },
        }),
    ];
    entries.extend(time_control_entries());
    entries
}

fn time_control_entries() -> Vec<CapabilityEntry> {
    vec![
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.human_wait_world_step",
            class: CapabilityClass::ActorObservableConsequence,
            rationale: "human wait advances the authoritative world frontier through the TUI",
            fixture_id: "strongbox_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::HumanWaitOneTick,
            typed_assertion: "wait.1_tick appends ActorWaited and advances the world tick",
            actor_knowledge_assertion:
                "actor_tomas receives the post-wait embodied view from his holder-known context",
            rendered_assertion: "rendered embodied view advances to Tick: 1",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::Required,
        }),
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.human_sleep_terminal",
            class: CapabilityClass::ActorObservableConsequence,
            rationale: "human-started sleep reaches its duration terminal through advance-until",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::StartSleepThenAdvanceUntil { max_ticks: 4 },
            typed_assertion: "sleep.here plus advance-until emits SleepCompleted",
            actor_knowledge_assertion:
                "actor_tomas receives a source-bearing sleep completion interval summary",
            rendered_assertion: "rendered interval summary includes sleep completed",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::Required,
        }),
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.human_work_terminal",
            class: CapabilityClass::ActorObservableConsequence,
            rationale: "human-started work reaches its duration terminal through advance-until",
            fixture_id: "ordinary_workday_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::MoveWorkThenAdvanceUntil { max_ticks: 4 },
            typed_assertion: "work.block.workplace_tomas plus advance-until emits WorkBlockCompleted",
            actor_knowledge_assertion:
                "actor_tomas receives a source-bearing work completion interval summary",
            rendered_assertion: "rendered interval summary includes work completed",
            anti_leak_fixtures: Vec::new(),
            no_human_evidence: EvidenceFlag::Required,
        }),
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.open_duration_wait_conflict",
            class: CapabilityClass::ActorObservableConsequence,
            rationale: "open body-exclusive durations reject ordinary wait while continuation remains typed",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::StartSleepThenWaitConflict,
            typed_assertion: "ordinary wait during sleep returns ReservationConflict",
            actor_knowledge_assertion:
                "actor_tomas sees an actor-safe reservation why-not without hidden scheduler state",
            rendered_assertion: "rendered why-not includes reservation_conflict",
            anti_leak_fixtures: vec!["sleep_eat_work_001"],
            no_human_evidence: EvidenceFlag::Required,
        }),
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.actor_known_interval_summary",
            class: CapabilityClass::ActorObservableState,
            rationale:
                "actor-known interval summaries are a view-model/render parity surface",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::StartSleepThenAdvanceUntil { max_ticks: 4 },
            typed_assertion: "advance-until builds ActorKnownIntervalSummary from source events",
            actor_knowledge_assertion:
                "hidden other-actor interval sources are filtered before rendering",
            rendered_assertion: "rendered interval summary carries source-bearing actor-known text",
            anti_leak_fixtures: vec!["hidden_food_unknown_route_001"],
            no_human_evidence: EvidenceFlag::Required,
        }),
        time_control_entry(TimeControlEntrySpec {
            key: "spec0047.time.advance_until_stop_reason",
            class: CapabilityClass::ActorObservableConsequence,
            rationale: "advance-until stop reasons are visible only through actor-known summaries",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::StartSleepThenAdvanceUntil { max_ticks: 4 },
            typed_assertion: "advance-until stops at possessed_duration_terminal",
            actor_knowledge_assertion:
                "the stop reason is paired with actor-known source-bearing interval data",
            rendered_assertion: "rendered interval summary includes possessed_duration_terminal",
            anti_leak_fixtures: vec!["hidden_food_unknown_route_001"],
            no_human_evidence: EvidenceFlag::Required,
        }),
    ]
}

struct TimeControlEntrySpec {
    key: &'static str,
    class: CapabilityClass,
    rationale: &'static str,
    fixture_id: &'static str,
    viewer_actor: &'static str,
    setup_operation: SetupOperation,
    typed_assertion: &'static str,
    actor_knowledge_assertion: &'static str,
    rendered_assertion: &'static str,
    anti_leak_fixtures: Vec<&'static str>,
    no_human_evidence: EvidenceFlag,
}

fn time_control_entry(spec: TimeControlEntrySpec) -> CapabilityEntry {
    CapabilityEntry {
        key: spec.key,
        ownership_scope: OwnershipScope::FuturePack {
            namespace: "spec0047_tui_authoritative_world_advance",
        },
        capability_class: spec.class,
        surface_disposition: SurfaceDisposition::Embodied,
        disposition_rationale: spec.rationale,
        fixture_ids: vec![spec.fixture_id],
        viewer_actor: spec.viewer_actor,
        setup_operation: spec.setup_operation,
        registry_action_id: None,
        typed_witness: Witness {
            kind: WitnessKind::TypedCausal,
            assertion: spec.typed_assertion,
        },
        actor_knowledge_witness: Witness {
            kind: WitnessKind::ActorKnowledge,
            assertion: spec.actor_knowledge_assertion,
        },
        rendered_witness: Some(Witness {
            kind: WitnessKind::RenderedText,
            assertion: spec.rendered_assertion,
        }),
        golden_path: None,
        anti_leak_fixtures: spec.anti_leak_fixtures,
        replay_evidence: EvidenceFlag::Required,
        no_human_evidence: spec.no_human_evidence,
    }
}

struct FamilyEntrySpec {
    key: &'static str,
    class: CapabilityClass,
    disposition: SurfaceDisposition,
    rationale: &'static str,
    fixture_id: &'static str,
    viewer_actor: &'static str,
    setup_operation: SetupOperation,
    typed_assertion: &'static str,
    actor_knowledge_assertion: &'static str,
    rendered_assertion: &'static str,
    rendered_kind: WitnessKind,
    golden_path: &'static str,
    anti_leak_fixtures: Vec<&'static str>,
    no_human_evidence: EvidenceFlag,
}

fn family_entry(spec: FamilyEntrySpec) -> CapabilityEntry {
    CapabilityEntry {
        key: spec.key,
        ownership_scope: OwnershipScope::Base,
        capability_class: spec.class,
        surface_disposition: spec.disposition,
        disposition_rationale: spec.rationale,
        fixture_ids: vec![spec.fixture_id],
        viewer_actor: spec.viewer_actor,
        setup_operation: spec.setup_operation,
        registry_action_id: None,
        typed_witness: Witness {
            kind: WitnessKind::TypedCausal,
            assertion: spec.typed_assertion,
        },
        actor_knowledge_witness: Witness {
            kind: WitnessKind::ActorKnowledge,
            assertion: spec.actor_knowledge_assertion,
        },
        rendered_witness: Some(Witness {
            kind: spec.rendered_kind,
            assertion: spec.rendered_assertion,
        }),
        golden_path: Some(spec.golden_path),
        anti_leak_fixtures: spec.anti_leak_fixtures,
        replay_evidence: EvidenceFlag::Required,
        no_human_evidence: spec.no_human_evidence,
    }
}
