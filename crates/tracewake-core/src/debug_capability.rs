pub const DEBUG_NON_DIEGETIC_MARKER: &str = "DEBUG NON-DIEGETIC";

/// Capability token for privileged, non-diegetic debug surfaces.
///
/// The token can be named outside the crate, but only Tracewake core can mint
/// it or place it into privileged debug view/report structs.
///
/// ```compile_fail
/// use tracewake_core::debug_capability::DebugCapability;
///
/// let _capability = DebugCapability {
///     marker: "DEBUG NON-DIEGETIC",
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugCapability {
    marker: &'static str,
}

impl DebugCapability {
    pub(crate) const fn mint() -> Self {
        Self {
            marker: DEBUG_NON_DIEGETIC_MARKER,
        }
    }

    pub const fn debug_only(&self) -> bool {
        true
    }

    pub const fn marker(&self) -> &'static str {
        self.marker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_is_debug_only_and_non_diegetic() {
        let capability = DebugCapability::mint();

        assert!(capability.debug_only());
        assert_eq!(capability.marker(), DEBUG_NON_DIEGETIC_MARKER);
    }
}
