use rally_core::{
    percent_of, sample_between, ActorTrace, ComparisonDelta, ComparisonReport, DiceRoller,
    PacketManifest, RunSeed, ScoreTrack, SimulationMetric, SimulationRun, TokenPool, TurnOrder,
    ValidationReport,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const CONTRACT: &str = include_str!("../contracts/consumer-surfaces.json");
const CONTRACT_SCHEMA: &str = "rally.consumer-surfaces.v1";

fn load_contract(raw: &str) -> Result<Value, String> {
    let contract: Value =
        serde_json::from_str(raw).map_err(|error| format!("invalid consumer contract: {error}"))?;
    if contract["schema"] != CONTRACT_SCHEMA {
        return Err(format!(
            "unsupported consumer contract schema: {}",
            contract["schema"]
        ));
    }
    if !contract["cases"].is_array() {
        return Err("consumer contract cases must be an array".to_string());
    }
    Ok(contract)
}

fn actual_surfaces() -> BTreeMap<&'static str, Value> {
    let mut surfaces = BTreeMap::new();

    let mut amaze_seed = RunSeed::from_u64(42);
    surfaces.insert(
        "AMAZE",
        json!({
            "samples": [
                sample_between(&mut amaze_seed, 10, 20),
                sample_between(&mut amaze_seed, 10, 20)
            ],
            "percent": percent_of(3, 4)
        }),
    );

    let mut quest_dice = DiceRoller::new("quest-contract");
    let quest_roll = quest_dice
        .roll("2d6+1")
        .expect("the retained QUEST expression must remain valid");
    surfaces.insert(
        "QUEST",
        json!({
            "expression": quest_roll.spec.expression,
            "rolls": quest_roll.rolls,
            "total": quest_roll.total,
            "seed_position": quest_roll.seed_position
        }),
    );

    let hunt_run = SimulationRun::new("hunt-sim", "wavelength", "smoke");
    let hunt_report = ValidationReport {
        subject: hunt_run.run_id.clone(),
        findings: Vec::new(),
    };
    surfaces.insert(
        "HUNT",
        json!({
            "run_id": hunt_run.run_id,
            "status": hunt_report.status()
        }),
    );

    let mut tigris_turns = TurnOrder::new(["human", "ai"]);
    let active_before = tigris_turns.active().map(str::to_string);
    let active_after = tigris_turns.advance().map(str::to_string);
    let mut tigris_scores = ScoreTrack::new(["human", "ai"]);
    let human_score = tigris_scores.add("human", 3);
    let mut tigris_tokens = TokenPool::new([("stake", 2)]);
    let stake_spent = tigris_tokens.spend("stake", 1);
    surfaces.insert(
        "TIGRIS",
        json!({
            "active_before": active_before,
            "active_after": active_after,
            "human_score": human_score,
            "stake_spent": stake_spent,
            "stake_remaining": tigris_tokens.count("stake")
        }),
    );

    let banish_run = SimulationRun::new("banish", "first-winter", "proof");
    let mut banish_trace = ActorTrace::new("planner", "settlement-planner");
    banish_trace.record_action();
    banish_trace.record_blocked_turn();
    let banish_metric = SimulationMetric::new("survival_rate", 80.0);
    surfaces.insert(
        "BANISH",
        json!({
            "run_id": banish_run.run_id,
            "actions": banish_trace.actions,
            "blocked_turns": banish_trace.blocked_turns,
            "metric": {
                "name": banish_metric.name,
                "value": banish_metric.value
            }
        }),
    );

    let mut ceres_comparison = ComparisonReport::new("tier-a", "market", "coop");
    ceres_comparison.add_delta(ComparisonDelta::higher_is_better("resilience", 40.0, 55.0));
    ceres_comparison.add_delta(ComparisonDelta::lower_is_better("cost", 100.0, 90.0));
    let comparison: Value =
        serde_json::from_str(&ceres_comparison.to_json()).expect("comparison JSON must be valid");
    let mut ceres_packet = PacketManifest::new("ceres:tier-a:proof");
    ceres_packet.add_artifact("comparison", "comparison.json");
    ceres_packet.add_artifact("report", "report.json");
    let packet: Value =
        serde_json::from_str(&ceres_packet.to_json()).expect("packet JSON must be valid");
    surfaces.insert(
        "CERES",
        json!({
            "comparison": comparison,
            "packet": packet
        }),
    );

    surfaces
}

#[test]
fn retained_consumers_match_their_adopted_surfaces() {
    let contract = load_contract(CONTRACT).expect("contract must be valid");

    let actual = actual_surfaces();
    let cases = contract["cases"]
        .as_array()
        .expect("contract cases must be an array");
    assert_eq!(cases.len(), actual.len());

    let mut seen = BTreeSet::new();
    for case in cases {
        let consumer = case["consumer"]
            .as_str()
            .expect("consumer must be a string");
        assert!(seen.insert(consumer), "duplicate consumer row: {consumer}");
        assert_eq!(
            actual.get(consumer),
            Some(&case["expected"]),
            "{consumer} drifted from its retained RALLY surface"
        );
    }
}

#[test]
fn contract_rejects_schema_drift() {
    let mut contract: Value = serde_json::from_str(CONTRACT).expect("contract must be valid JSON");
    contract["schema"] = Value::String("rally.consumer-surfaces.v2".to_string());
    let drifted = serde_json::to_string(&contract).expect("drifted contract must serialize");

    assert_eq!(
        load_contract(&drifted).unwrap_err(),
        "unsupported consumer contract schema: \"rally.consumer-surfaces.v2\""
    );
}
