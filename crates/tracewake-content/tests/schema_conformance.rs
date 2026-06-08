#[derive(Clone, Copy)]
struct ContentSchemaEvidence {
    requirement: &'static str,
    layer: &'static str,
    test_name: &'static str,
}

const CONTENT_SCHEMA_EVIDENCE: &[ContentSchemaEvidence] = &[
    ContentSchemaEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/validation",
        test_name: "content_prose_born_fact_rejected",
    },
    ContentSchemaEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/schema",
        test_name: "content_new_field_requires_validator_and_canonical_serialization",
    },
];

const FORBIDDEN_CONTENT_RS: &str = include_str!("forbidden_content.rs");

#[test]
fn schema_conformance_maps_content_spine_requirements_to_named_tests() {
    for evidence in CONTENT_SCHEMA_EVIDENCE {
        assert_eq!(
            evidence.requirement, "SPINE-AC-010",
            "content schema conformance owns only the content/schema and content/validation slice"
        );
        assert!(
            ["content/schema", "content/validation"].contains(&evidence.layer),
            "unknown content conformance layer {}",
            evidence.layer
        );
        let needle = format!("fn {}(", evidence.test_name);
        assert!(
            FORBIDDEN_CONTENT_RS.contains(&needle),
            "missing {} evidence for {} in {}",
            evidence.test_name,
            evidence.requirement,
            evidence.layer
        );
    }

    assert!(CONTENT_SCHEMA_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "content/schema"));
    assert!(CONTENT_SCHEMA_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "content/validation"));
}
