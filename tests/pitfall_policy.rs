use serde_json::Value;

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn shared_boundary_manifest() -> Value {
    serde_json::from_str(include_str!("../docs/shared-simulation-boundaries.v1.json"))
        .expect("shared simulation boundary manifest must remain valid JSON")
}

fn assert_boundary(manifest: &Value, pitfall: &str, required_owner: &str, blocked_claims: &[&str]) {
    let boundaries = manifest["pitfall_boundaries"]
        .as_array()
        .expect("boundary manifest must expose pitfall_boundaries");
    let boundary = boundaries
        .iter()
        .find(|boundary| boundary["pitfall"] == pitfall)
        .unwrap_or_else(|| panic!("missing boundary for {pitfall}"));

    assert_eq!(boundary["required_owner"], required_owner);
    for claim in blocked_claims {
        let claims = boundary["blocked_claims"]
            .as_array()
            .expect("boundary blocked_claims must be an array");
        assert!(
            claims.iter().any(|candidate| candidate == claim),
            "missing blocked claim for {pitfall}: {claim}"
        );
    }
}

#[test]
fn shared_harness_policy_stays_out_of_rally_core() {
    // Checks RALLY-PF-01: shared mechanics must not absorb product policy.
    let readme = include_str!("../README.md");
    let compatibility = include_str!("../docs/consumer-compatibility.md");
    let contract = include_str!("../contracts/consumer-surfaces.json");
    let boundary = shared_boundary_manifest();

    assert_eq!(boundary["$schema"], "rally.shared-simulation-boundaries.v1");
    assert_eq!(
        boundary["authority"]["neutral_run_report_comparison_and_packet_primitives"],
        "RALLY"
    );
    assert_boundary(
        &boundary,
        "RALLY-PF-01",
        "consumer repositories",
        &[
            "escape-room policy belongs in rally-core",
            "D&D adventure state belongs in rally-core",
            "puzzle publish policy belongs in rally-core",
            "board Parliament semantics belong in rally-core",
            "CERES economic meaning belongs in rally-core",
        ],
    );
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
    let boundary = shared_boundary_manifest();

    assert_eq!(
        boundary["authority"]["consumer_migration_readiness"],
        "affected consumer repositories"
    );
    assert_boundary(
        &boundary,
        "RALLY-PF-02",
        "affected consumer repositories",
        &[
            "green RALLY canary approves consumer migration",
            "provider rehearsal replaces consumer workflow tests",
            "consumer rollback plan is optional after RALLY passes",
            "RALLY release notes can approve consumer adoption",
        ],
    );
    for consumer in ["AMAZE", "QUEST", "HUNT", "TIGRIS", "BANISH", "CERES"] {
        assert!(boundary["consumer_canaries"]
            .as_array()
            .expect("consumer_canaries must be an array")
            .iter()
            .any(|candidate| candidate == consumer));
    }
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
    let extensions = descriptors["descriptors"][0]["extensions"]
        .as_array()
        .expect("simulation run descriptor must expose boundary extensions");
    for (name, value) in [
        (
            "collection_boundary",
            "neutral simulation and validation evidence spine only",
        ),
        (
            "product_policy_boundary",
            "does not encode consumer product semantics",
        ),
        (
            "adoption_boundary",
            "does not approve downstream product adoption",
        ),
    ] {
        assert!(
            extensions
                .iter()
                .any(|extension| extension["namespace"] == "rally.boundary"
                    && extension["name"] == name
                    && extension["value"] == value),
            "missing RUNE boundary extension {name}"
        );
    }
    assert!(!docs.contains("product adoption approval"));
    assert!(!descriptors.to_string().contains("adoption_lane"));
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
    let boundary = shared_boundary_manifest();

    assert_eq!(
        boundary["authority"]["private_room_campaign_vendor_player_and_playtest_evidence"],
        "source consumer repositories and privacy review"
    );
    assert_boundary(
        &boundary,
        "RALLY-PF-05",
        "source consumer repositories and privacy review",
        &[
            "real room detail belongs in shared fixture",
            "campaign notes belong in shared evidence packet",
            "vendor record belongs in public example",
            "player or playtest detail belongs in RALLY documentation",
        ],
    );
    assert!(guide.contains("Do not copy private"));
    assert!(proof_surface.contains("product-neutral"));
    for fixture in [accepted, rejected] {
        assert!(!fixture.contains("vendor"));
        assert!(!fixture.contains("player"));
        assert!(!fixture.contains("campaign"));
        assert!(!fixture.contains("private"));
    }
}
