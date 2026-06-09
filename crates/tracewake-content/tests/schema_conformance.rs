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
        test_name:
            "content_new_field_requires_typed_validation_and_canonical_serialization_metadata",
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

#[test]
fn fixture_scope_is_registered_and_canonicalized() {
    use tracewake_content::fixtures;
    use tracewake_content::schema::{
        canonical_key_for_schema_field, content_field_by_schema_field, FixtureScope,
        ForbiddenConstructPolicy, ValidationPhase,
    };
    use tracewake_content::serialization::{deserialize_fixture, serialize_fixture};

    let registration = content_field_by_schema_field("fixture_scope")
        .expect("fixture_scope must be in the content field registry");
    assert_eq!(registration.canonical_serialization_key, "fixture_scope");
    assert_eq!(registration.validation_phase, ValidationPhase::ParseSchema);
    assert_eq!(
        registration.forbidden_construct_policy,
        ForbiddenConstructPolicy::TypedAffordance
    );
    assert_eq!(
        canonical_key_for_schema_field("fixture_scope"),
        "fixture_scope"
    );

    let golden = fixtures::sleep_eat_work_001();
    assert_eq!(
        golden.fixture.fixture_scope,
        FixtureScope::Phase3AHistorical
    );
    let serialized = serialize_fixture(&golden.fixture);
    let text = String::from_utf8(serialized.clone()).unwrap();
    assert!(
        text.lines()
            .any(|line| line == "fixture_scope|phase3a_historical"),
        "serialized fixture must carry fixture_scope as canonical data"
    );
    let round_tripped = deserialize_fixture(&serialized).unwrap();
    assert_eq!(round_tripped.fixture_scope, FixtureScope::Phase3AHistorical);
}
