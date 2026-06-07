use tracewake_core::ids::{ContentManifestId, ContentVersion, FixtureId, SchemaVersion};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentManifest {
    pub manifest_id: ContentManifestId,
    pub fixture_id: FixtureId,
    pub schema_version: SchemaVersion,
    pub content_version: ContentVersion,
    pub canonical_paths: Vec<String>,
    pub actor_roster: Vec<String>,
    pub no_human_day_windows: Vec<String>,
    pub content_fingerprint: String,
}

impl ContentManifest {
    pub fn new(
        manifest_id: ContentManifestId,
        fixture_id: FixtureId,
        schema_version: SchemaVersion,
        content_version: ContentVersion,
        mut canonical_paths: Vec<String>,
        canonical_payload: &[u8],
    ) -> Self {
        canonical_paths.sort();
        let mut fingerprint_input = canonical_paths.join("\n").into_bytes();
        fingerprint_input.extend_from_slice(b"\n--payload--\n");
        fingerprint_input.extend_from_slice(canonical_payload);
        Self {
            manifest_id,
            fixture_id,
            schema_version,
            content_version,
            canonical_paths,
            actor_roster: Vec::new(),
            no_human_day_windows: Vec::new(),
            content_fingerprint: stable_fingerprint(&fingerprint_input),
        }
    }
}

pub fn stable_fingerprint(bytes: &[u8]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("twf1-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_carries_identity_and_fingerprint() {
        let manifest = ContentManifest::new(
            ContentManifestId::new("manifest_strongbox").unwrap(),
            FixtureId::new("strongbox_001").unwrap(),
            SchemaVersion::new("schema_v1").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec!["z.fixture".to_string(), "a.fixture".to_string()],
            b"payload",
        );

        assert_eq!(manifest.fixture_id.as_str(), "strongbox_001");
        assert_eq!(manifest.schema_version.as_str(), "schema_v1");
        assert_eq!(manifest.content_version.as_str(), "content_v1");
        assert_eq!(manifest.canonical_paths, ["a.fixture", "z.fixture"]);
        assert!(manifest.actor_roster.is_empty());
        assert!(manifest.no_human_day_windows.is_empty());
        assert!(manifest.content_fingerprint.starts_with("twf1-"));
    }
}
