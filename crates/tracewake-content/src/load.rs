use tracewake_core::ids::{ContentManifestId, ContentVersion};

use crate::manifest::ContentManifest;
use crate::schema::FixtureSchema;
use crate::serialization::{deserialize_fixture, serialize_fixture, SerializationError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedFixture {
    pub fixture: FixtureSchema,
    pub manifest: ContentManifest,
    pub canonical_world: tracewake_core::state::PhysicalState,
}

pub fn load_fixture_package(
    manifest_id: ContentManifestId,
    content_version: ContentVersion,
    mut files: Vec<SourceFile>,
) -> Result<LoadedFixture, SerializationError> {
    files.sort_by(|left, right| left.path.cmp(&right.path));
    let primary = files
        .first()
        .ok_or(SerializationError::MissingField("source_file"))?;
    let mut fixture = deserialize_fixture(&primary.bytes)?;
    fixture.canonicalize();
    let canonical_bytes = serialize_fixture(&fixture);
    let manifest = ContentManifest::new(
        manifest_id,
        fixture.fixture_id.clone(),
        fixture.schema_version.clone(),
        content_version,
        files.iter().map(|file| file.path.clone()).collect(),
        &canonical_bytes,
    );
    let canonical_world = fixture.to_physical_state();
    Ok(LoadedFixture {
        fixture,
        manifest,
        canonical_world,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActorSchema, ContainerSchema, FixtureSchema, ItemSchema, PlaceSchema};
    use crate::serialization::serialize_fixture;
    use tracewake_core::ids::{ActorId, ContainerId, FixtureId, ItemId, PlaceId, SchemaVersion};
    use tracewake_core::location::Location;

    fn fixture() -> FixtureSchema {
        FixtureSchema {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            actors: vec![ActorSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                current_place_id: PlaceId::new("shop_front").unwrap(),
            }],
            places: vec![PlaceSchema {
                place_id: PlaceId::new("shop_front").unwrap(),
                display_label: "Shop front".to_string(),
                adjacent_place_ids: Vec::new(),
            }],
            doors: Vec::new(),
            containers: vec![ContainerSchema {
                container_id: ContainerId::new("strongbox_tomas").unwrap(),
                place_id: PlaceId::new("shop_front").unwrap(),
                is_open: false,
                is_locked: false,
                contents: vec![ItemId::new("coin_stack_01").unwrap()],
                contents_visible_when_closed: false,
            }],
            items: vec![ItemSchema {
                item_id: ItemId::new("coin_stack_01").unwrap(),
                portable: true,
                location: Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
            }],
        }
    }

    #[test]
    fn shuffled_input_paths_load_to_same_canonical_world() {
        let bytes = serialize_fixture(&fixture());
        let first = load_fixture_package(
            ContentManifestId::new("manifest_strongbox").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![
                SourceFile {
                    path: "fixture.twf".to_string(),
                    bytes: bytes.clone(),
                },
                SourceFile {
                    path: "z_note.ignored".to_string(),
                    bytes: b"ignored".to_vec(),
                },
            ],
        )
        .unwrap();
        let second = load_fixture_package(
            ContentManifestId::new("manifest_strongbox").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![
                SourceFile {
                    path: "z_note.ignored".to_string(),
                    bytes: b"ignored".to_vec(),
                },
                SourceFile {
                    path: "fixture.twf".to_string(),
                    bytes,
                },
            ],
        )
        .unwrap();

        assert_eq!(first.canonical_world, second.canonical_world);
        assert_eq!(
            first.manifest.content_fingerprint,
            second.manifest.content_fingerprint
        );
    }
}
