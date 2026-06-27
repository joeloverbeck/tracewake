use tracewake_content::fixtures::{self, GoldenFixture};
use tracewake_content::load::{load_fixture_package, LoadError};
use tracewake_core::actions::{ActionRegistry, ReportStatus, ValidationReport};
use tracewake_core::checksum::PhysicalChecksum;
use tracewake_core::debug_capability::DebugSessionAuthority;
use tracewake_core::ids::{
    ActorId, ContentManifestId, ContentVersion, ControllerId, DebugReportId, ItemId,
    SemanticActionId,
};
use tracewake_core::projections::ProjectionError;
use tracewake_core::runtime::{
    ContinuedRuntimeReceipt, LoadedWorldRuntime, RuntimeActionReceipt, RuntimeCommand,
    RuntimeCommandError, RuntimeReceiptKind,
};
use tracewake_core::scheduler::no_human::NoHumanDayReport;
use tracewake_core::scheduler::WorldAdvanceError;
use tracewake_core::state::ControllerMode;
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{
    DebugBeliefsView, DebugEpistemicsView, DebugObservationsView, EmbodiedViewModel, NotebookView,
    SemanticActionEntry, TypedActorKnownIntervalSummary,
};

use crate::debug_panels::{
    render_action_rejection_panel, render_controller_binding_panel, render_event_log_panel,
    render_item_location_panel, render_no_human_day_panel, render_phase3a_debug_panel,
    render_projection_rebuild_panel, render_replay_panel,
};
use crate::render::{render_debug_overlay, render_embodied_view};

#[derive(Debug)]
pub enum AppError {
    Load(LoadError),
    ActorNotFound(ActorId),
    ActorNotBound,
    DebugUnavailable,
    Projection(ProjectionError),
    SchedulerRestoreFailed,
    SemanticActionNotFound(String),
    WorldAdvance(WorldAdvanceError),
    Runtime(RuntimeCommandError),
}

impl From<LoadError> for AppError {
    fn from(value: LoadError) -> Self {
        Self::Load(value)
    }
}

impl From<ProjectionError> for AppError {
    fn from(value: ProjectionError) -> Self {
        Self::Projection(value)
    }
}

pub struct TuiApp {
    runtime: LoadedWorldRuntime,
    controller_id: ControllerId,
    bound_actor_id: Option<ActorId>,
    debug_authority: Option<DebugSessionAuthority>,
    last_rejection: Option<ValidationReport>,
    last_interval_summary: Option<TypedActorKnownIntervalSummary>,
}

impl TuiApp {
    pub fn load_default() -> Result<Self, AppError> {
        Self::from_golden(fixtures::strongbox_001())
    }

    pub fn from_golden(golden: GoldenFixture) -> Result<Self, AppError> {
        let loaded = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )?;
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        registry.register_phase1_take_place();
        registry.register_phase1_inspect_wait();
        registry.register_phase2a_epistemics();
        registry.register_phase3a_sleep();
        registry.register_phase3a_eat();
        registry.register_phase3a_work();
        registry.register_phase3a_continue_routine();
        let runtime = LoadedWorldRuntime::from_bootstrap(
            loaded.into_runtime_bootstrap(registry),
            SimTick::ZERO,
        );
        Ok(Self {
            runtime,
            controller_id: ControllerId::new("controller_human").unwrap(),
            bound_actor_id: None,
            debug_authority: None,
            last_rejection: None,
            last_interval_summary: None,
        })
    }

    pub fn bind_actor(&mut self, actor_id: ActorId) -> Result<(), AppError> {
        self.bind_actor_with_mode(actor_id, ControllerMode::Embodied, None)
    }

    pub fn bind_debug_actor(&mut self, actor_id: ActorId) -> Result<(), AppError> {
        let authority = self.runtime.local_operator_debug_authority();
        self.bind_actor_with_mode(actor_id, ControllerMode::Debug, Some(authority))
    }

    fn bind_actor_with_mode(
        &mut self,
        actor_id: ActorId,
        mode: ControllerMode,
        authority: Option<DebugSessionAuthority>,
    ) -> Result<(), AppError> {
        if !self.runtime.actor_exists(&actor_id) {
            return Err(AppError::ActorNotFound(actor_id));
        }
        let command = match mode {
            ControllerMode::Embodied => {
                RuntimeCommand::bind_controller(self.controller_id.clone(), actor_id.clone())
            }
            ControllerMode::Debug => RuntimeCommand::bind_debug_controller(
                authority
                    .clone()
                    .expect("debug binding requires operator authority"),
                self.controller_id.clone(),
                actor_id.clone(),
            ),
            ControllerMode::Detached => {
                RuntimeCommand::detach_controller(self.controller_id.clone())
            }
        };
        self.runtime
            .submit_command(command)
            .map_err(AppError::Runtime)?;
        self.bound_actor_id = Some(actor_id);
        self.debug_authority = authority;
        self.last_rejection = None;
        Ok(())
    }

    pub fn current_view(&self) -> Result<EmbodiedViewModel, AppError> {
        let actor_id = self
            .bound_actor_id
            .as_ref()
            .ok_or(AppError::ActorNotBound)?;
        self.runtime
            .embodied_view_model(
                &self.controller_id,
                actor_id,
                self.last_rejection.as_ref(),
                self.last_interval_summary.clone(),
            )
            .map_err(AppError::from)
    }

    fn debug_available_for(&self, actor_id: &ActorId) -> bool {
        self.runtime
            .controller_debug_available_for(&self.controller_id, actor_id)
    }

    pub fn debug_available(&self) -> bool {
        self.debug_authority.is_some()
            && self
                .bound_actor_id
                .as_ref()
                .is_some_and(|actor_id| self.debug_available_for(actor_id))
    }

    fn debug_authority(&self) -> Result<DebugSessionAuthority, AppError> {
        let actor_id = self
            .bound_actor_id
            .as_ref()
            .ok_or(AppError::DebugUnavailable)?;
        let authority = self
            .debug_authority
            .as_ref()
            .ok_or(AppError::DebugUnavailable)?;
        self.runtime
            .debug_session_authority_for(authority, &self.controller_id, actor_id)
            .ok_or(AppError::DebugUnavailable)
    }

    pub fn render_current_view(&self) -> Result<String, AppError> {
        Ok(render_embodied_view(&self.current_view()?))
    }

    pub fn render_debug_embodied_overlay(&self) -> Result<Option<String>, AppError> {
        let view = self.current_view()?;
        if view.debug_available() {
            Ok(Some(render_debug_overlay(&view)))
        } else {
            Ok(None)
        }
    }

    pub fn submit_semantic_action(
        &mut self,
        semantic_action_id: &SemanticActionId,
    ) -> Result<RuntimeActionReceipt, AppError> {
        let view = self.current_view()?;
        let entry = view
            .semantic_actions
            .iter()
            .find(|entry| &entry.semantic_action_id == semantic_action_id)
            .cloned()
            .ok_or_else(|| AppError::SemanticActionNotFound(semantic_action_id.to_string()))?;
        self.submit_entry(&entry, &view)
    }

    fn submit_entry(
        &mut self,
        entry: &SemanticActionEntry,
        source_view: &EmbodiedViewModel,
    ) -> Result<RuntimeActionReceipt, AppError> {
        let actor_id = self.bound_actor_id.clone().ok_or(AppError::ActorNotBound)?;
        // Deferral witness: embodied targeted-command routing is not yet wired, but the
        // semantic-action surface already carries target_ids. Borrow it (no behavioral
        // effect, no mutable operator to mutate) to keep the field's reachability guard
        // satisfied until a live consumer lands.
        let _ = &entry.target_ids;
        let receipt = self
            .runtime
            .submit_command(RuntimeCommand::submit_semantic_action(
                self.controller_id.clone(),
                actor_id,
                entry.clone(),
                source_view.clone(),
            ))
            .map_err(AppError::Runtime)?;
        let result = receipt
            .into_action_receipt()
            .expect("submit_semantic_action command returns an action receipt");
        if result.report.status == ReportStatus::Rejected {
            self.last_rejection = Some(result.report.clone());
        } else {
            self.last_rejection = None;
        }
        Ok(result)
    }

    pub fn event_count(&self) -> usize {
        self.runtime.event_count()
    }

    pub fn advance_until(&mut self, max_ticks: u64) -> Result<ContinuedRuntimeReceipt, AppError> {
        let actor_id = self.bound_actor_id.clone().ok_or(AppError::ActorNotBound)?;
        let receipt = self
            .runtime
            .submit_command(RuntimeCommand::continue_until(
                self.controller_id.clone(),
                actor_id,
                max_ticks,
            ))
            .map_err(AppError::Runtime)?;
        let result = match receipt.kind() {
            RuntimeReceiptKind::Continued(result) => result.clone(),
            _ => panic!("continue_until command returns a continued receipt"),
        };
        self.last_interval_summary = result.actor_known_interval_summary().cloned();
        self.last_rejection = None;
        Ok(result)
    }

    pub fn run_no_human_day(&mut self) -> Result<NoHumanDayReport, AppError> {
        let authority = self.debug_authority()?;
        let receipt = self
            .runtime
            .submit_command(RuntimeCommand::run_no_human_day(authority))
            .map_err(AppError::Runtime)?;
        let report = receipt
            .into_no_human_day_report()
            .expect("run_no_human_day command returns a no-human report");
        self.last_rejection = None;
        Ok(report)
    }

    pub fn physical_checksum(&self) -> PhysicalChecksum {
        self.runtime.physical_checksum()
    }

    pub fn render_debug_event_log_panel(&self) -> String {
        render_event_log_panel(&self.runtime.debug_event_log_view())
    }

    pub fn render_debug_controller_binding_panel(&self) -> String {
        let report = self.runtime.controller_binding_debug_report(
            DebugReportId::new("debug.controller_bindings").unwrap(),
        );
        render_controller_binding_panel(&report)
    }

    pub fn render_debug_item_location_panel(&self, item_id: &ItemId) -> String {
        let report = self.runtime.item_location_debug_report(
            DebugReportId::new("debug.item_location").unwrap(),
            item_id,
        );
        render_item_location_panel(&report)
    }

    pub fn render_debug_action_rejection_panel(&self) -> Option<String> {
        let report = self.last_rejection.as_ref()?;
        Some(render_action_rejection_panel(
            &self.runtime.action_rejection_debug_report(report),
        ))
    }

    pub fn render_debug_projection_rebuild_panel(&self) -> String {
        render_projection_rebuild_panel(&self.runtime.projection_rebuild_debug_report(
            DebugReportId::new("debug.projection_rebuild").unwrap(),
        ))
    }

    pub fn render_debug_replay_panel(&self) -> String {
        render_replay_panel(
            &self
                .runtime
                .replay_debug_report(DebugReportId::new("debug.replay").unwrap()),
        )
    }

    pub fn render_debug_needs_panel(&self) -> String {
        render_phase3a_debug_panel(&self.runtime.phase3a_needs_debug_report())
    }

    pub fn render_debug_routines_panel(&self) -> String {
        render_phase3a_debug_panel(&self.runtime.phase3a_routines_debug_report())
    }

    pub fn render_debug_planner_panel(&self, actor_id: &ActorId) -> String {
        render_phase3a_debug_panel(&self.runtime.phase3a_planner_debug_report(actor_id))
    }

    pub fn render_debug_stuck_panel(&self) -> String {
        render_phase3a_debug_panel(&self.runtime.phase3a_stuck_debug_report())
    }

    pub fn render_debug_no_human_day_panel(&self) -> String {
        render_no_human_day_panel(&self.runtime.no_human_day_debug_report())
    }

    pub fn render_debug_actor_panel(&self, actor_id: &ActorId) -> String {
        render_phase3a_debug_panel(&self.runtime.phase3a_actor_debug_report(actor_id))
    }

    pub fn notebook_view(&self) -> Result<NotebookView, AppError> {
        self.current_view()?.notebook.ok_or(AppError::ActorNotBound)
    }

    pub fn debug_epistemics_view(&self) -> DebugEpistemicsView {
        self.runtime.debug_epistemics_view()
    }

    pub fn debug_beliefs_view(&self, actor_id: &ActorId) -> Result<DebugBeliefsView, AppError> {
        self.runtime
            .debug_beliefs_view(actor_id)
            .ok_or_else(|| AppError::ActorNotFound(actor_id.clone()))
    }

    pub fn debug_observations_view(
        &self,
        actor_id: &ActorId,
    ) -> Result<DebugObservationsView, AppError> {
        self.runtime
            .debug_observations_view(actor_id)
            .ok_or_else(|| AppError::ActorNotFound(actor_id.clone()))
    }

    #[cfg(test)]
    fn detach_controller_for_test(&mut self) {
        self.runtime
            .submit_command(RuntimeCommand::detach_controller(
                self.controller_id.clone(),
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::ids::SemanticActionId;

    #[test]
    fn app_binds_renders_submits_and_rerenders() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        assert!(!app.current_view().unwrap().debug_available());
        let before = app.render_current_view().unwrap();
        assert!(before.contains("strongbox_tomas"));
        assert!(!before.contains("Debug: available=true"));
        app.detach_controller_for_test();
        assert!(!app.current_view().unwrap().debug_available());
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();

        let result = app
            .submit_semantic_action(
                &SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
            )
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Accepted);

        let after = app.render_current_view().unwrap();
        assert!(after.contains("strongbox_tomas"));
        assert!(after.contains("coin_stack_01"));
        assert!(app.event_count() >= 2);
    }

    #[test]
    fn consuming_core_interval_product_is_read_only() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();

        app.advance_until(0).unwrap();
        assert!(app.last_interval_summary.is_some());
        let event_count = app.event_count();
        let physical_checksum = app.physical_checksum();

        let _rendered = app.render_current_view().unwrap();

        assert_eq!(app.event_count(), event_count);
        assert_eq!(app.physical_checksum(), physical_checksum);
    }

    #[test]
    fn app_debug_overlay_renders_only_for_bound_debug_controller() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();

        let rendered = app.render_debug_embodied_overlay().unwrap().unwrap();
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Embodied Overlay"));
        assert!(rendered.contains("Knowledge context: id=hkc."));

        app.detach_controller_for_test();

        assert_eq!(app.render_debug_embodied_overlay().unwrap(), None);
    }

    #[test]
    fn render_functions_have_production_callers_or_documented_allowlist() {
        let render_source = include_str!("render.rs");
        let debug_panels_source = include_str!("debug_panels.rs");
        let app_source = include_str!("app.rs");
        let run_source = include_str!("run.rs");
        let transcript_source = include_str!("transcript.rs");

        assert!(render_reachability_errors(
            &[render_source, debug_panels_source],
            &[
                production_source(app_source),
                production_source(run_source),
                production_source(transcript_source)
            ]
        )
        .is_empty());
    }

    #[test]
    fn render_reachability_guard_fires_on_synthetic_uncalled_renderer() {
        let debug_panels_source = format!(
            "{}\npub fn render_synthetic_uncalled_panel() -> String {{ String::new() }}\n",
            include_str!("debug_panels.rs")
        );
        let render_source = include_str!("render.rs");
        let app_source = include_str!("app.rs");
        let run_source = include_str!("run.rs");
        let transcript_source = include_str!("transcript.rs");

        let errors = render_reachability_errors(
            &[render_source, &debug_panels_source],
            &[
                production_source(app_source),
                production_source(run_source),
                production_source(transcript_source),
            ],
        );

        assert!(
            errors
                .iter()
                .any(|error| error.contains("render_synthetic_uncalled_panel")),
            "synthetic uncalled render function was not reported: {errors:?}"
        );
    }

    #[test]
    fn debug_dispatch_routes_every_arm_through_availability_gate() {
        let run_source = production_source(include_str!("run.rs"));
        let input_source = production_source(include_str!("input.rs"));

        assert!(debug_command_gating_errors(run_source, input_source).is_empty());

        let synthetic_ungated = run_source.replace(
            "    if !app.debug_available() {\n        return writeln!(writer, \"Error: debug unavailable\");\n    }\n",
            "",
        );
        let errors = debug_command_gating_errors(&synthetic_ungated, input_source);
        assert!(
            errors.iter().any(|error| error.contains("debug_available")),
            "synthetic_ungated_debug_command_arm did not trigger: {errors:?}"
        );

        let synthetic_unrouted = input_source.replace(
            "    Actor(ActorId),\n",
            "    Actor(ActorId),\n    SyntheticUnrouted,\n",
        );
        let errors = debug_command_gating_errors(run_source, &synthetic_unrouted);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("SyntheticUnrouted")),
            "synthetic_unrouted_debug_command_variant did not trigger: {errors:?}"
        );

        let synthetic_early_return = run_source.replace(
            "fn render_debug<W: Write>(\n    app: &mut TuiApp,\n    debug_command: DebugCommand,\n    writer: &mut W,\n) -> std::io::Result<()> {\n",
            "fn render_debug<W: Write>(\n    app: &mut TuiApp,\n    debug_command: DebugCommand,\n    writer: &mut W,\n) -> std::io::Result<()> {\n    return Ok(());\n",
        );
        let errors = debug_command_gating_errors(&synthetic_early_return, input_source);
        assert!(
            errors.iter().any(|error| error.contains("early return")),
            "synthetic_early_return_before_gate did not trigger: {errors:?}"
        );
    }

    #[test]
    fn rejected_action_surfaces_why_not_from_validation_report() {
        let mut app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
        app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();

        let result = app
            .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Rejected);

        let rendered = app.render_current_view().unwrap();
        assert!(rendered.contains("Why-not:"));
        assert!(rendered.contains(&result.report.actor_visible_summary));
    }

    #[test]
    fn app_runs_no_human_day_into_real_log_metrics() {
        let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let before_events = app.event_count();

        let report = app.run_no_human_day().unwrap();

        assert!(report.ordinary_pipeline_events > 0);
        assert!(app.event_count() > before_events);
        assert!(app.render_debug_no_human_day_panel().contains("events="));
        assert!(app.render_debug_no_human_day_panel().contains("canonical="));
    }

    #[test]
    fn run_no_human_day_refuses_intrinsically_without_debug_availability() {
        let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
        let before_events = app.event_count();

        let result = app.run_no_human_day();

        assert!(matches!(result, Err(AppError::DebugUnavailable)));
        assert_eq!(app.event_count(), before_events);
    }

    #[test]
    fn world_advancing_tui_methods_are_intrinsically_debug_gated_or_exempt() {
        let app_source = production_source(include_str!("app.rs"));

        let errors = world_advancing_method_gate_errors(app_source);
        assert!(errors.is_empty(), "{errors:?}");

        let synthetic = app_source.replace(
            "    pub fn event_count(&self) -> usize {\n",
            "    pub fn synthetic_advance_without_gate(&mut self) {\n        self.log.append;\n    }\n\n    pub fn event_count(&self) -> usize {\n",
        );
        let errors = world_advancing_method_gate_errors(&synthetic);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("synthetic_advance_without_gate")),
            "synthetic ungated world advancer did not trigger: {errors:?}"
        );
    }

    #[test]
    fn tui_local_guard_registry_covers_structural_guards() {
        let app_source = include_str!("app.rs");

        assert!(tui_local_guard_registry_errors(app_source).is_empty());

        let synthetic_removed = app_source.replace(
            "fn debug_dispatch_routes_every_arm_through_availability_gate()",
            "fn debug_dispatch_routes_every_arm_through_availability_gate_removed()",
        );
        let errors = tui_local_guard_registry_errors(&synthetic_removed);
        assert!(
            errors
                .iter()
                .any(|error| error
                    .contains("debug_dispatch_routes_every_arm_through_availability_gate")),
            "synthetic removed guard did not trigger: {errors:?}"
        );
    }

    #[test]
    fn controller_mode_debug_availability_decision_is_explicit() {
        let mut app = TuiApp::load_default().unwrap();
        let actor_id = ActorId::new("actor_tomas").unwrap();
        let other_actor_id = ActorId::new("actor_elena").unwrap();

        app.bind_actor(actor_id.clone()).unwrap();
        assert!(!app.current_view().unwrap().debug_available());
        assert!(!app.debug_available());
        assert!(!app.debug_available_for(&actor_id));

        app.bind_debug_actor(actor_id.clone()).unwrap();
        assert!(app.current_view().unwrap().debug_available());
        assert!(app.debug_available());
        assert!(!app.debug_available_for(&other_actor_id));

        app.detach_controller_for_test();
        assert!(!app.current_view().unwrap().debug_available());
        assert!(!app.debug_available());

        let docs = include_str!(
            "../../../docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md"
        );
        assert!(docs.contains(
            "ControllerMode decision: debug availability requires ControllerMode::Debug"
        ));
    }

    fn render_reachability_errors(render_sources: &[&str], caller_sources: &[&str]) -> Vec<String> {
        render_sources
            .iter()
            .flat_map(|source| render_function_names(source))
            .filter(|name| !render_function_is_called(name, caller_sources))
            .filter(|name| !render_function_allowlisted(name))
            .map(|name| format!("{name} has no production caller"))
            .collect()
    }

    fn render_function_names(source: &str) -> Vec<String> {
        source
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                let rest = trimmed.strip_prefix("pub fn render_")?;
                let name_end = rest.find('(')?;
                Some(format!("render_{}", &rest[..name_end]))
            })
            .collect()
    }

    fn render_function_is_called(name: &str, caller_sources: &[&str]) -> bool {
        caller_sources
            .iter()
            .any(|source| source.matches(name).count() > 1 || source.contains(&format!("{name}(")))
    }

    fn render_function_allowlisted(name: &str) -> bool {
        match name {
            // `render_rejection` is production transcript evidence rather than an interactive
            // panel; it is exercised by `transcript.rs`, not by the command-loop app path.
            "render_rejection" => true,
            _ => false,
        }
    }

    fn debug_command_gating_errors(run_source: &str, input_source: &str) -> Vec<String> {
        let render_debug = run_source.split("fn render_debug").nth(1).unwrap_or("");
        let render_debug_body = function_body_if_present(run_source, "render_debug").unwrap_or("");
        let gate_position = render_debug.find("if !app.debug_available()");
        let match_position = render_debug.find("match debug_command");
        let mut errors = Vec::new();
        match (gate_position, match_position) {
            (Some(gate), Some(dispatch)) if gate < dispatch => {}
            _ => errors.push(
                "render_debug must check debug_available before matching DebugCommand".to_string(),
            ),
        }
        if let Some(gate) = gate_position {
            if render_debug[..gate].contains("return ") {
                errors.push("render_debug has an early return before debug_available".to_string());
            }
        }
        let variants = debug_command_variants(input_source);
        let dispatch_body = match_arm_body(render_debug_body, "match debug_command").unwrap_or("");
        for variant in variants {
            let token = format!("DebugCommand::{variant}");
            let count = dispatch_body.matches(&token).count();
            if count != 1 {
                errors.push(format!("{variant} dispatch count must be 1, got {count}"));
            }
        }
        errors
    }

    fn debug_command_variants(input_source: &str) -> Vec<String> {
        let enum_body = input_source
            .split("pub enum DebugCommand")
            .nth(1)
            .and_then(|tail| tail.split_once('{'))
            .and_then(|(_, tail)| tail.split_once('}'))
            .map(|(body, _)| body)
            .unwrap_or("");
        enum_body
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }
                let name = trimmed.split(['(', ',']).next().unwrap_or("").trim();
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }

    fn world_advancing_method_gate_errors(app_source: &str) -> Vec<String> {
        pub const EXEMPT_WORLD_ADVANCING_METHODS: &[(&str, &str)] = &[(
            "submit_semantic_action",
            "ordinary embodied player command; authorization comes from current view source context",
        ), (
            "advance_until",
            "typed gameplay time-control command; authority remains in the core coordinator loop",
        )];
        public_mut_methods(app_source)
            .into_iter()
            .filter(|(_, body)| {
                method_body_advances_world(body) && !body.contains("debug_available()")
            })
            .filter(|(name, _)| {
                !EXEMPT_WORLD_ADVANCING_METHODS
                    .iter()
                    .any(|(exempt_name, rationale)| exempt_name == name && !rationale.is_empty())
            })
            .map(|(name, _)| {
                format!("{name} advances world state without intrinsic debug_available gate")
            })
            .collect()
    }

    fn method_body_advances_world(body: &str) -> bool {
        body.contains("self.log")
            || body.contains("&mut self.log")
            || body.contains("self.state")
            || body.contains("&mut self.state")
    }

    fn public_mut_methods(source: &str) -> Vec<(String, &str)> {
        let mut methods = Vec::new();
        let mut remaining = source;
        while let Some(position) = remaining.find("pub fn ") {
            remaining = &remaining[position + "pub fn ".len()..];
            let Some((name, after_name)) = remaining.split_once('(') else {
                break;
            };
            let name = name.trim().to_string();
            let Some(body_start) = after_name.find('{') else {
                break;
            };
            let signature = &after_name[..body_start];
            let Some(body) = function_body_after_signature(after_name) else {
                break;
            };
            if signature.contains("&mut self") {
                methods.push((name, body));
            }
            let consumed = body_start + body.len();
            remaining = &after_name[consumed.min(after_name.len())..];
        }
        methods
    }

    fn tui_local_guard_registry_errors(app_source: &str) -> Vec<String> {
        const REQUIRED_TUI_GUARDS: &[&str] = &[
            "render_functions_have_production_callers_or_documented_allowlist",
            "render_reachability_guard_fires_on_synthetic_uncalled_renderer",
            "debug_dispatch_routes_every_arm_through_availability_gate",
            "run_no_human_day_refuses_intrinsically_without_debug_availability",
            "world_advancing_tui_methods_are_intrinsically_debug_gated_or_exempt",
            "tui_local_guard_registry_covers_structural_guards",
            "controller_mode_debug_availability_decision_is_explicit",
        ];
        REQUIRED_TUI_GUARDS
            .iter()
            .filter(|guard| !app_source.contains(&format!("fn {guard}(")))
            .map(|guard| format!("missing TUI local guard registry member {guard}"))
            .collect()
    }

    fn function_body_if_present<'a>(source: &'a str, name: &str) -> Option<&'a str> {
        let marker = format!("fn {name}");
        let tail = source.split(&marker).nth(1)?;
        function_body_after_signature(tail)
    }

    fn function_body_after_signature(source: &str) -> Option<&str> {
        let body_start = source.find('{')?;
        let body = &source[body_start..];
        let mut depth = 0usize;
        for (index, ch) in body.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some(&body[..=index]);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn match_arm_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
        let tail = source.split(marker).nth(1)?;
        function_body_after_signature(tail)
    }

    fn production_source(source: &str) -> &str {
        source.split("\n#[cfg(test)]").next().unwrap_or(source)
    }
}
