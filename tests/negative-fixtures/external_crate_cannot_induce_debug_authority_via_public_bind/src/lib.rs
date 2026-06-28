use tracewake_core::runtime::LoadedWorldRuntime;

pub fn induce_debug_authority_without_operator_proof(runtime: &LoadedWorldRuntime) {
    let _authority = runtime.local_operator_debug_authority();
}
