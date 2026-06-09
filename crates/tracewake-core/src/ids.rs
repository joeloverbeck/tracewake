//! Stable semantic identifiers used by the Phase 1 kernel boundary.
//!
//! IDs serialize canonically as their validated inner string. Display names,
//! array indices, terminal menu rows, and process-random UUIDs are not valid
//! identity sources.

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdError {
    Empty,
    InvalidUtf8,
    InvalidStart { found: char },
    InvalidCharacter { found: char },
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::Empty => write!(f, "stable ID must not be empty"),
            IdError::InvalidUtf8 => write!(f, "stable ID bytes must be UTF-8"),
            IdError::InvalidStart { found } => {
                write!(
                    f,
                    "stable ID must start with lowercase ASCII or a digit, found {found:?}"
                )
            }
            IdError::InvalidCharacter { found } => {
                write!(f, "stable ID contains invalid character {found:?}")
            }
        }
    }
}

impl std::error::Error for IdError {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StableId(String);

impl StableId {
    fn parse(value: impl Into<String>) -> Result<Self, IdError> {
        let value = value.into();
        validate_stable_id(&value)?;
        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_stable_id(value: &str) -> Result<(), IdError> {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(IdError::Empty);
    };

    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return Err(IdError::InvalidStart { found: first });
    }

    for found in chars {
        let valid = found.is_ascii_lowercase()
            || found.is_ascii_digit()
            || matches!(found, '_' | '-' | '.');
        if !valid {
            return Err(IdError::InvalidCharacter { found });
        }
    }

    Ok(())
}

macro_rules! stable_id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(StableId);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                Ok(Self(StableId::parse(value)?))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn serialize_canonical(&self) -> &str {
                self.as_str()
            }

            pub fn serialize_canonical_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }

            pub fn deserialize_canonical(value: &[u8]) -> Result<Self, IdError> {
                let value = std::str::from_utf8(value).map_err(|_| IdError::InvalidUtf8)?;
                Self::new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = IdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

stable_id_type!(FixtureId);
stable_id_type!(ActorId);
stable_id_type!(PlaceId);
stable_id_type!(ExitId);
stable_id_type!(DoorId);
stable_id_type!(ContainerId);
stable_id_type!(ItemId);
stable_id_type!(FoodSupplyId);
stable_id_type!(WorkplaceId);
stable_id_type!(SleepAffordanceId);
stable_id_type!(ActionId);
stable_id_type!(SemanticActionId);
stable_id_type!(EventId);
stable_id_type!(EventTypeId);
stable_id_type!(ValidationReportId);
stable_id_type!(ControllerId);
stable_id_type!(ProcessId);
stable_id_type!(ContentManifestId);
stable_id_type!(ContentVersion);
stable_id_type!(SchemaVersion);
stable_id_type!(PropositionId);
stable_id_type!(ObservationId);
stable_id_type!(BeliefId);
stable_id_type!(ContradictionId);
stable_id_type!(EpistemicProjectionVersion);
stable_id_type!(IntentionId);
stable_id_type!(RoutineTemplateId);
stable_id_type!(RoutineExecutionId);
stable_id_type!(CandidateGoalId);
stable_id_type!(DecisionTraceId);
stable_id_type!(StuckDiagnosticId);
stable_id_type!(AgentProjectionVersion);
stable_id_type!(ProposalId);
stable_id_type!(DebugReportId);
stable_id_type!(ProjectionId);
stable_id_type!(ViewModelId);
stable_id_type!(HolderKnownContextId);

/// Derive a deterministic UUID-shaped stable ID from content location.
///
/// This is only for the spec's allowed case where an identifier is derived
/// from stable content path + parent ID + stable ordinal. It is not random and
/// must not be used as a shortcut for display names, menu rows, or insertion
/// positions. The output is lowercase hexadecimal in UUID text shape, with
/// version and variant bits set after a stable FNV-1a based 128-bit digest.
pub fn deterministic_uuid_from_content_path(
    content_path: &str,
    parent_id: &str,
    stable_ordinal: u64,
) -> String {
    let mut bytes = [0_u8; 16];
    let left = stable_fnv1a64(
        0xcbf2_9ce4_8422_2325,
        content_path,
        parent_id,
        stable_ordinal,
    );
    let right = stable_fnv1a64(
        0x8422_2325_cbf2_9ce4,
        parent_id,
        content_path,
        stable_ordinal,
    );
    bytes[..8].copy_from_slice(&left.to_be_bytes());
    bytes[8..].copy_from_slice(&right.to_be_bytes());

    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

fn stable_fnv1a64(offset: u64, content_path: &str, parent_id: &str, stable_ordinal: u64) -> u64 {
    let mut hash = offset;
    for byte in content_path
        .as_bytes()
        .iter()
        .copied()
        .chain([0])
        .chain(parent_id.as_bytes().iter().copied())
        .chain([0])
        .chain(stable_ordinal.to_be_bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn valid_id_round_trips_byte_identically() {
        let id = SemanticActionId::new("take.item.coin_stack_01.from.strongbox_tomas").unwrap();

        assert_eq!(
            id.serialize_canonical(),
            "take.item.coin_stack_01.from.strongbox_tomas"
        );
        assert_eq!(
            SemanticActionId::deserialize_canonical(id.serialize_canonical_bytes()).unwrap(),
            id
        );
    }

    #[test]
    fn display_name_shapes_are_rejected() {
        assert_eq!(ActorId::new("").unwrap_err(), IdError::Empty);
        assert_eq!(
            ActorId::new("Tomas Miller").unwrap_err(),
            IdError::InvalidStart { found: 'T' }
        );
        assert_eq!(
            ActorId::new("tomas miller").unwrap_err(),
            IdError::InvalidCharacter { found: ' ' }
        );
    }

    #[test]
    fn ordering_is_by_stable_string_value() {
        let mut ids = [
            ItemId::new("coin_stack_10").unwrap(),
            ItemId::new("coin_stack_02").unwrap(),
            ItemId::new("coin_stack_01").unwrap(),
        ];

        ids.sort();

        let ordered: Vec<_> = ids.iter().map(ItemId::as_str).collect();
        assert_eq!(ordered, ["coin_stack_01", "coin_stack_02", "coin_stack_10"]);
    }

    #[test]
    fn epistemic_ids_reject_display_names_and_order_by_value() {
        assert_eq!(PropositionId::new("").unwrap_err(), IdError::Empty);
        assert_eq!(
            ObservationId::new("Observed Coin").unwrap_err(),
            IdError::InvalidStart { found: 'O' }
        );
        assert_eq!(
            BeliefId::new("belief tomas").unwrap_err(),
            IdError::InvalidCharacter { found: ' ' }
        );
        assert_eq!(
            ContradictionId::new("contradiction:tomas").unwrap_err(),
            IdError::InvalidCharacter { found: ':' }
        );

        let mut ids = [
            PropositionId::new("prop_10").unwrap(),
            PropositionId::new("prop_02").unwrap(),
            PropositionId::new("prop_01").unwrap(),
        ];

        ids.sort();

        let ordered: Vec<_> = ids.iter().map(PropositionId::as_str).collect();
        assert_eq!(ordered, ["prop_01", "prop_02", "prop_10"]);
    }

    #[test]
    fn cognition_ids_reject_display_names_and_order_by_value() {
        assert_eq!(IntentionId::new("").unwrap_err(), IdError::Empty);
        assert_eq!(
            RoutineTemplateId::new("Morning Wake").unwrap_err(),
            IdError::InvalidStart { found: 'M' }
        );
        assert_eq!(
            RoutineExecutionId::new("routine execution").unwrap_err(),
            IdError::InvalidCharacter { found: ' ' }
        );
        assert_eq!(
            CandidateGoalId::new("goal:breakfast").unwrap_err(),
            IdError::InvalidCharacter { found: ':' }
        );
        assert_eq!(
            DecisionTraceId::new("decisionTrace").unwrap_err(),
            IdError::InvalidCharacter { found: 'T' }
        );
        assert_eq!(
            StuckDiagnosticId::new("stuck diagnostic").unwrap_err(),
            IdError::InvalidCharacter { found: ' ' }
        );
        assert_eq!(
            AgentProjectionVersion::new("Agent Projection").unwrap_err(),
            IdError::InvalidStart { found: 'A' }
        );

        let mut ids = [
            IntentionId::new("intention_10").unwrap(),
            IntentionId::new("intention_02").unwrap(),
            IntentionId::new("intention_01").unwrap(),
        ];

        ids.sort();

        let ordered: Vec<_> = ids.iter().map(IntentionId::as_str).collect();
        assert_eq!(ordered, ["intention_01", "intention_02", "intention_10"]);
    }

    #[test]
    fn id_kinds_are_distinct_types() {
        fn accepts_actor(_: ActorId) {}
        fn accepts_item(_: ItemId) {}

        accepts_actor(ActorId::new("actor_tomas").unwrap());
        accepts_item(ItemId::new("coin_stack_01").unwrap());
        assert_ne!(TypeId::of::<ActorId>(), TypeId::of::<ItemId>());
    }

    #[test]
    fn deterministic_uuid_helper_is_stable_and_valid() {
        let first = deterministic_uuid_from_content_path(
            "fixtures/strongbox/entities.toml",
            "place_shop",
            7,
        );
        let second = deterministic_uuid_from_content_path(
            "fixtures/strongbox/entities.toml",
            "place_shop",
            7,
        );

        assert_eq!(first, second);
        assert_eq!(first.len(), 36);
        assert_eq!(first.chars().filter(|ch| *ch == '-').count(), 4);
        assert!(ItemId::new(first).is_ok());
    }
}
