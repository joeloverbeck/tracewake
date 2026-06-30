use tracewake_core::actions::ActionRegistry;

use super::{
    CapabilityClass, CapabilityEntry, EvidenceFlag, OwnershipScope, SetupOperation,
    SurfaceDisposition, Witness, WitnessKind,
};

pub fn registered_action_ids() -> Vec<String> {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
        .definitions()
        .map(|definition| definition.action_id.as_str().to_string())
        .collect()
}

pub fn entries() -> Vec<CapabilityEntry> {
    vec![
        semantic_action_entry(ActionEntrySpec {
            action_id: "check_container",
            fixture_id: "strongbox_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction {
                action_id: "check_container",
            },
            typed_assertion:
                "check.container.strongbox_tomas is present as a typed semantic action before submission",
            actor_knowledge_assertion: "container_closed is explained with actor-visible facts only",
            rendered_assertion:
                "rendered why-not contains the closed-container blocker and omits debug-only context",
            golden_path: "crates/tracewake-tui/tests/goldens/action_check_container.txt",
            anti_leak_fixtures: vec!["strongbox_001"],
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "close",
            fixture_id: "strongbox_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "close" },
            typed_assertion: "close.door.door_house_street is present in the actor-filtered current view",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas before rendering close",
            rendered_assertion: "rendered Actions section contains the close semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_close.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "continue_routine",
            fixture_id: "possession_does_not_reset_intention_001",
            viewer_actor: "actor_mara",
            setup_operation: SetupOperation::SubmitRegistryAction {
                action_id: "continue_routine",
            },
            typed_assertion: "continue_routine is present for actor_mara's active intention",
            actor_knowledge_assertion:
                "the current view is sealed to actor_mara before rendering routine continuation",
            rendered_assertion:
                "rendered Actions section contains the continue-routine semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_continue_routine.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        CapabilityEntry {
            key: "spec0057.routine.embodied_continue_workday",
            ownership_scope: OwnershipScope::FuturePack {
                namespace: "spec0057_embodied_routine_continuation",
            },
            capability_class: CapabilityClass::ActorObservableConsequence,
            surface_disposition: SurfaceDisposition::Embodied,
            disposition_rationale:
                "continue_routine reaches move-to-work and then fails closed instead of selecting work from workplace context",
            fixture_ids: vec!["ordinary_workday_001"],
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::ContinueRoutineWorkday { max_ticks: 8 },
            registry_action_id: Some("continue_routine"),
            typed_witness: Witness {
                kind: WitnessKind::TypedCausal,
                assertion:
                    "two continue_routine submissions emit ContinueRoutineProposed and ActorMoved, then reject the workplace shortcut with a typed stuck diagnostic",
            },
            actor_knowledge_witness: Witness {
                kind: WitnessKind::ActorKnowledge,
                assertion:
                    "actor_tomas reaches the known workplace but does not select work without an active work intention",
            },
            rendered_witness: Some(Witness {
                kind: WitnessKind::RenderedText,
                assertion:
                    "rendered embodied state remains actor-known after the rejected shortcut without debug truth",
            }),
            golden_path: None,
            anti_leak_fixtures: vec!["embodied_continue_hidden_workplace_001"],
            replay_evidence: EvidenceFlag::Required,
            no_human_evidence: EvidenceFlag::Required,
        },
        semantic_action_entry(ActionEntrySpec {
            action_id: "eat",
            fixture_id: "no_human_day_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "eat" },
            typed_assertion: "eat is present for actor-known local food",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas before rendering eat",
            rendered_assertion: "rendered Actions section contains the eat semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_eat.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        query_entry(
            "inspect_entity",
            "strongbox_001",
            "actor_tomas",
            "inspect_entity is registered as query-only actor-observable state",
            "the current view is sealed to actor_tomas before rendering inspectable entities",
            "rendered embodied state contains the inspectable local container",
            "crates/tracewake-tui/tests/goldens/action_inspect_entity.txt",
        ),
        query_entry(
            "inspect_place",
            "strongbox_001",
            "actor_tomas",
            "inspect_place is registered as query-only actor-observable state",
            "the current view is sealed to actor_tomas before rendering the current place",
            "rendered embodied state contains the actor-known current place",
            "crates/tracewake-tui/tests/goldens/action_inspect_place.txt",
        ),
        query_entry(
            "look",
            "strongbox_001",
            "actor_tomas",
            "look is registered as query-only actor-observable state",
            "the current view is sealed to actor_tomas before rendering look output",
            "rendered embodied state contains actor-known local observations",
            "crates/tracewake-tui/tests/goldens/action_look.txt",
        ),
        semantic_action_entry(ActionEntrySpec {
            action_id: "move",
            fixture_id: "door_access_001",
            viewer_actor: "actor_sena",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "move" },
            typed_assertion: "move.to.back_room is present as a typed semantic action before submission",
            actor_knowledge_assertion:
                "door_closed_blocks_movement is explained with actor-visible facts only",
            rendered_assertion:
                "rendered why-not contains the door blocker and omits debug-only context",
            golden_path:
                "crates/tracewake-tui/tests/goldens/base_epistemic_why_not_door_closed.txt",
            anti_leak_fixtures: vec!["door_access_001"],
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "open",
            fixture_id: "strongbox_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "open" },
            typed_assertion:
                "open.container.strongbox_tomas is present in the actor-filtered current view",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas before rendering open",
            rendered_assertion: "rendered Actions section contains the open semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_open.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "place",
            fixture_id: "place_carried_item_001",
            viewer_actor: "actor_lina",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "place" },
            typed_assertion: "place.sample_token_01 is present as a typed semantic action",
            actor_knowledge_assertion:
                "the current view is sealed to actor_lina before rendering place",
            rendered_assertion: "rendered Actions section contains the place semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_place.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "sleep",
            fixture_id: "sleep_eat_work_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "sleep" },
            typed_assertion: "sleep is present for actor-known local sleep affordance",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas before rendering sleep",
            rendered_assertion: "rendered Actions section contains the sleep semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_sleep.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "take",
            fixture_id: "view_model_local_actions_001",
            viewer_actor: "actor_lina",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "take" },
            typed_assertion: "take.sample_token_01 is present as a typed semantic action",
            actor_knowledge_assertion:
                "the current view is sealed to actor_lina before rendering take",
            rendered_assertion: "rendered Actions section contains the take semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_take.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        query_entry(
            "truthful_accuse_probe",
            "knowledge_blocker_accuse_001",
            "actor_tomas",
            "truthful_accuse_probe is registered as query-only epistemic consequence",
            "the current view is sealed to actor_tomas before rendering accusation-safe context",
            "rendered embodied state omits culprit/debug truth while the fixture provides the probe",
            "crates/tracewake-tui/tests/goldens/action_truthful_accuse_probe.txt",
        ),
        semantic_action_entry(ActionEntrySpec {
            action_id: "wait",
            fixture_id: "strongbox_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction { action_id: "wait" },
            typed_assertion: "wait.1_tick is present in the actor-filtered current view",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas holder-known context before rendering",
            rendered_assertion: "rendered Actions section contains the wait semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/base_semantic_action_wait.txt",
            anti_leak_fixtures: Vec::new(),
        }),
        semantic_action_entry(ActionEntrySpec {
            action_id: "work_block",
            fixture_id: "embodied_workplace_believed_open_truth_closed_commit_fails_001",
            viewer_actor: "actor_tomas",
            setup_operation: SetupOperation::SubmitRegistryAction {
                action_id: "work_block",
            },
            typed_assertion: "work.block.workplace_tomas is present from actor-known workplace access",
            actor_knowledge_assertion:
                "the current view is sealed to actor_tomas before rendering work",
            rendered_assertion: "rendered Actions section contains the work semantic action",
            golden_path: "crates/tracewake-tui/tests/goldens/action_work_block.txt",
            anti_leak_fixtures: Vec::new(),
        }),
    ]
}

struct ActionEntrySpec {
    action_id: &'static str,
    fixture_id: &'static str,
    viewer_actor: &'static str,
    setup_operation: SetupOperation,
    typed_assertion: &'static str,
    actor_knowledge_assertion: &'static str,
    rendered_assertion: &'static str,
    golden_path: &'static str,
    anti_leak_fixtures: Vec<&'static str>,
}

fn semantic_action_entry(spec: ActionEntrySpec) -> CapabilityEntry {
    CapabilityEntry {
        key: action_key(spec.action_id),
        ownership_scope: OwnershipScope::Base,
        capability_class: CapabilityClass::SemanticAction,
        surface_disposition: SurfaceDisposition::Embodied,
        disposition_rationale: "registered action has an actor-filtered TUI capability witness",
        fixture_ids: vec![spec.fixture_id],
        viewer_actor: spec.viewer_actor,
        setup_operation: spec.setup_operation,
        registry_action_id: Some(spec.action_id),
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
        golden_path: Some(spec.golden_path),
        anti_leak_fixtures: spec.anti_leak_fixtures,
        replay_evidence: EvidenceFlag::Required,
        no_human_evidence: EvidenceFlag::NotApplicable {
            rationale: "per-action human-submitted capability census; no-human families are covered by 0046TUISIMPLA-007",
        },
    }
}

fn query_entry(
    action_id: &'static str,
    fixture_id: &'static str,
    viewer_actor: &'static str,
    typed_assertion: &'static str,
    actor_knowledge_assertion: &'static str,
    rendered_assertion: &'static str,
    golden_path: &'static str,
) -> CapabilityEntry {
    CapabilityEntry {
        key: action_key(action_id),
        ownership_scope: OwnershipScope::Base,
        capability_class: CapabilityClass::ActorObservableState,
        surface_disposition: SurfaceDisposition::Embodied,
        disposition_rationale:
            "registered query-only action is dispositioned as actor-observable state",
        fixture_ids: vec![fixture_id],
        viewer_actor,
        setup_operation: SetupOperation::ObserveQueryOnly { action_id },
        registry_action_id: Some(action_id),
        typed_witness: Witness {
            kind: WitnessKind::TypedCausal,
            assertion: typed_assertion,
        },
        actor_knowledge_witness: Witness {
            kind: WitnessKind::ActorKnowledge,
            assertion: actor_knowledge_assertion,
        },
        rendered_witness: Some(Witness {
            kind: WitnessKind::RenderedText,
            assertion: rendered_assertion,
        }),
        golden_path: Some(golden_path),
        anti_leak_fixtures: if action_id == "truthful_accuse_probe" {
            vec![fixture_id]
        } else {
            Vec::new()
        },
        replay_evidence: EvidenceFlag::Required,
        no_human_evidence: EvidenceFlag::NotApplicable {
            rationale:
                "query-only action census entry; no-human families are covered by 0046TUISIMPLA-007",
        },
    }
}

fn action_key(action_id: &'static str) -> &'static str {
    match action_id {
        "check_container" => "base.semantic_action.check_container",
        "close" => "base.semantic_action.close",
        "continue_routine" => "base.semantic_action.continue_routine",
        "eat" => "base.semantic_action.eat",
        "inspect_entity" => "base.semantic_action.inspect_entity",
        "inspect_place" => "base.semantic_action.inspect_place",
        "look" => "base.semantic_action.look",
        "move" => "base.semantic_action.move",
        "open" => "base.semantic_action.open",
        "place" => "base.semantic_action.place",
        "sleep" => "base.semantic_action.sleep",
        "take" => "base.semantic_action.take",
        "truthful_accuse_probe" => "base.semantic_action.truthful_accuse_probe",
        "wait" => "base.semantic_action.wait",
        "work_block" => "base.semantic_action.work_block",
        _ => "base.semantic_action.unknown",
    }
}
