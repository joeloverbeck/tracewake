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
///
/// ```compile_fail
/// use tracewake_core::debug_capability::DebugCapability;
///
/// let _capability = DebugCapability::mint();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugCapability {
    marker: &'static str,
}

/// Runtime-minted authority for debug/operator commands and views.
///
/// The token can be named by clients, but only core runtime/controller binding
/// operator entrypoints can mint it in production.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSessionAuthority {
    capability: DebugCapability,
}

/// Explicit local-operator proof for non-diegetic debug sessions.
///
/// This capability is created by launch/session setup code, outside ordinary
/// embodied command input. It can be named by clients, but its fields remain
/// private so ordinary parsers cannot fabricate a debug session authority from
/// actor input or controller binding state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalOperatorDebugAuthority {
    authority: DebugSessionAuthority,
}

impl DebugCapability {
    pub(crate) const fn mint() -> Self {
        Self {
            marker: DEBUG_NON_DIEGETIC_MARKER,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.marker == DEBUG_NON_DIEGETIC_MARKER
    }

    pub const fn marker(&self) -> &'static str {
        self.marker
    }

    #[cfg(test)]
    pub(crate) const fn test_non_debug() -> Self {
        Self {
            marker: "NOT DEBUG",
        }
    }
}

impl DebugSessionAuthority {
    pub(crate) const fn mint() -> Self {
        Self {
            capability: DebugCapability::mint(),
        }
    }

    pub(crate) fn capability(&self) -> DebugCapability {
        self.capability.clone()
    }

    pub fn debug_only(&self) -> bool {
        self.capability.debug_only()
    }

    #[cfg(feature = "test-support")]
    pub fn for_test() -> Self {
        Self::mint()
    }
}

impl LocalOperatorDebugAuthority {
    pub fn for_local_operator_launch() -> Self {
        Self {
            authority: DebugSessionAuthority::mint(),
        }
    }

    pub fn session_authority(&self) -> DebugSessionAuthority {
        self.authority.clone()
    }

    pub fn debug_only(&self) -> bool {
        self.authority.debug_only()
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

    #[test]
    fn session_authority_debug_only_delegates_to_capability_marker() {
        let authority = DebugSessionAuthority::mint();
        assert!(authority.debug_only());

        let forged_non_debug = DebugSessionAuthority {
            capability: DebugCapability::test_non_debug(),
        };
        assert!(!forged_non_debug.debug_only());
    }

    #[test]
    fn local_operator_authority_mints_debug_session_authority() {
        let operator = LocalOperatorDebugAuthority::for_local_operator_launch();

        assert!(operator.debug_only());
        assert!(operator.session_authority().debug_only());
    }
}
