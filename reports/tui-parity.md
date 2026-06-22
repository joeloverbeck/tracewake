  What I found

  The seam is clean: tracewake-core builds an EmbodiedViewModel, and tracewake-tui's render_embodied_view(view: &EmbodiedViewModel) -> String  
  turns it into text. The renderer is a pure function returning a String — no terminal needed to test it. That's a big asset; parity can be  
  asserted as plain string checks.

  But render_embodied_view reads fields à la carte (view.visible_exits, view.carried_items, view.semantic_actions, …). It never destructures  
  the struct. So:

  ▎ If you add a field to EmbodiedViewModel, the renderer compiles cleanly and silently ignores it. That is exactly your nightmare, and it's  
  ▎ live today.

  The render tests reinforce this: they're hand-written, one per field (renderer_prints_semantic_action_ids,  
  renderer_prints_door_endpoints_and_item_sources, …). Good coverage of what exists, but a new field gets a test only if someone remembers.  
  Nothing forces it.

  There are actually two hops to keep in parity

  Your worry decomposes, and the two halves need different guards:

  Hop 2 — view model → rendered text. This one is cheaply enforceable at compile time. Make the renderer destructure exhaustively with no ..  
  rest pattern:

  pub fn render_embodied_view(view: &EmbodiedViewModel) -> String {  
      let EmbodiedViewModel {  
          view_model_id, mode, viewer_actor_id, sim_tick, place_id, place_label,  
          visible_exits, visible_doors, visible_containers, visible_items,  
          carried_items, local_actors, semantic_actions, /* … */ debug_available,  
      } = view;  
      // …render using the locals…  
  }

  Now the moment a new field lands, -D warnings (you already run clippy that way in CI) fires unused_variables and the build goes red until  
  someone consciously either renders it or binds it to _field with a comment saying why it's intentionally not shown. It doesn't prove the  
  field renders well — but it converts "forgot the surface exists" into a forced, reviewable decision at the exact moment of divergence. This  
  is the single highest-leverage change, and it's small.

  Pair it with exhaustive match (no _ arm) on the action/affordance enums (SemanticActionEntry kinds, WhyNotFailureKind, etc.). Then every new  
  action variant the simulation can produce is a compile error in the renderer until it's handled. Same mechanism, applied to "new kinds"  
  rather than "new fields."

  Hop 1 — core feature → view model. This is the deeper version of your fear, and it's the one that'll bite when Phase 4 lands. A field-level  
  guard can't catch "institutions exist in core but the projection that builds EmbodiedViewModel never looks at them" — the view model is  
  simply blind, so there's no field to forget. The guard here is a scenario-driven golden/snapshot test: run a fixture through the real  
  pipeline that exercises the feature, render the embodied surface, and assert the feature is visible in the output. You already have the  
  ingredients — embodied_flow.rs, app.render_current_view(), and a view_model_local_actions_001 fixture. A snapshot tool like insta over a  
  "kitchen-sink" scenario makes the diff impossible to ignore: when a feature changes the world but not the surface, the reviewer sees it.

  Recommendation

  Layer them, cheapest first:

  1. Exhaustive destructure + exhaustive match in the renderer — compile-time parity for hop 2. Do this now; it's nearly free and permanent.  
  2. One golden snapshot acceptance test per major feature scenario, driven through the real pipeline — covers hop 1, where Phase  
  4/notices/institutions would otherwise vanish.  
  3. Optionally, a tiny coverage test that enumerates the affordance/feature enum and asserts each renders non-empty — ties "new variant" to  
  "rendering obligation" explicitly.

  This also isn't ad-hoc relative to your doctrine — it operationalizes the existing VIEW-DEBUG-SPLIT / EMBODIED-TEMPORAL view-model gates  
  (execution docs 07 and 10). So it could even ride in as a small Certification-posture addition to the ORD-LIFE/FIRST-PROOF evidence package  
  rather than a one-off, which keeps it consistent with how the repo governs everything else.  
