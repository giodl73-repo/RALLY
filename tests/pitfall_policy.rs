use serde_json::Value;

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[test]
fn shared_harness_policy_stays_out_of_rally_core() {
    // Checks RALLY-PF-01: shared mechanics must not absorb product policy.
    let readme = include_str!("../README.md");
    let compatibility = include_str!("../docs/consumer-compatibility.md");
    let contract = include_str!("../contracts/consumer-surfaces.json");

    assert!(readme.contains("each consumer repo keeps its own"));
    assert!(readme.contains("Repo-local adapters decide what"));
    assert!(compatibility.contains("RALLY-owned surface"));
    assert!(compatibility.contains("consumer-owned semantics"));
    assert!(contract.contains("\"consumer\""));
}

#[test]
fn provider_canaries_do_not_replace_consumer_readiness() {
    // Checks RALLY-PF-02: provider-side canaries are not migration approval.
    let compatibility = include_str!("../docs/consumer-compatibility.md");

    assert!(contains_any(
        compatibility,
        &[
            "representative neutral compatibility canaries",
            "affected consumer",
            "consumer tests",
            "owner approval",
            "rollback"
        ]
    ));
    assert!(compatibility.contains("AMAZE"));
    assert!(compatibility.contains("QUEST"));
    assert!(compatibility.contains("HUNT"));
}

#[test]
fn rune_metadata_stays_neutral_contract_metadata() {
    // Checks RALLY-PF-04: RUNE descriptors are not product approval.
    let docs = include_str!("../docs/rune/README.md");
    let descriptors: Value =
        serde_json::from_str(include_str!("../docs/rune/simulation_contracts.json"))
            .expect("RUNE descriptor fixture must remain valid JSON");

    assert!(docs.contains("neutral simulator and"));
    assert!(docs.contains("validation evidence spine"));
    assert!(docs.contains("does not encode HUNT, TIGRIS, AMAZE"));
    assert!(docs.contains("product policy"));
    assert_eq!(
        descriptors["collection_id"],
        Value::String("rally.simulation_contracts".to_string())
    );
    assert_eq!(
        descriptors["collection_version"],
        Value::String("v0".to_string())
    );
    assert!(descriptors["descriptors"]
        .as_array()
        .is_some_and(|items| items.len() >= 6));
}

#[test]
fn shared_fixtures_do_not_publish_private_playtest_detail() {
    // Checks RALLY-PF-05: shared evidence packets stay product-neutral.
    let guide = include_str!("../CLAUDE.md");
    let proof_surface = include_str!("../docs/proof-surface.md");
    let accepted = include_str!("../fixtures/proof/accepted-report.json");
    let rejected = include_str!("../fixtures/proof/rejected-report.json");

    assert!(guide.contains("Do not copy private"));
    assert!(proof_surface.contains("product-neutral"));
    for fixture in [accepted, rejected] {
        assert!(!fixture.contains("vendor"));
        assert!(!fixture.contains("player"));
        assert!(!fixture.contains("campaign"));
        assert!(!fixture.contains("private"));
    }
}
