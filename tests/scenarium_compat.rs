#[path = "support/scenarium_adapter.rs"]
mod scenarium_adapter;

use rally_core::{
    ComparisonDelta as RallyDelta, ComparisonReport as RallyComparison, ValidationFinding,
    ValidationReport,
};
use scenarium::{ComparisonStatus, Error, RunStatus};
use serde_json::{json, Value};

fn status_name(status: RunStatus) -> &'static str {
    match status {
        RunStatus::Pass => "pass",
        RunStatus::Review => "review",
        RunStatus::Error => "error",
    }
}

#[test]
fn retained_validation_fixtures_map_without_losing_failures() {
    let accepted = ValidationReport {
        subject: "fixture:accepted".to_string(),
        findings: Vec::new(),
    };
    let rejected = ValidationReport {
        subject: "fixture:rejected".to_string(),
        findings: vec![ValidationFinding::error(
            "missing-beat",
            "SCENES.md#beat-cards",
            "beat reference does not exist",
        )],
    };

    assert_eq!(
        accepted.to_json(),
        include_str!("../fixtures/proof/accepted-report.json").trim()
    );
    assert_eq!(
        rejected.to_json(),
        include_str!("../fixtures/proof/rejected-report.json").trim()
    );

    let accepted_run =
        scenarium_adapter::validation_run(&accepted, "proof").expect("accepted report maps");
    let rejected_run =
        scenarium_adapter::validation_run(&rejected, "proof").expect("rejected report maps");
    let actual = json!({
        "schema": "rally.scenarium-compat.v1",
        "cases": [
            {
                "subject": accepted.subject,
                "rally_status": accepted.status(),
                "scenarium_status": status_name(accepted_run.status()),
                "run_id": accepted_run.run_id()
            },
            {
                "subject": rejected.subject,
                "rally_status": rejected.status(),
                "scenarium_status": status_name(rejected_run.status()),
                "run_id": rejected_run.run_id()
            }
        ]
    });
    let retained: Value =
        serde_json::from_str(include_str!("../fixtures/scenarium/compatibility.v1.json"))
            .expect("compatibility fixture is valid JSON");

    assert_eq!(actual, retained);
}

#[test]
fn retained_comparison_maps_to_the_same_outcome() {
    let mut rally = RallyComparison::new("wavelength", "baseline", "guided-final-set");
    rally.add_delta(RallyDelta::higher_is_better("pass_rate", 38.9, 63.9));
    rally.add_delta(RallyDelta::lower_is_better("p95_minutes", 215.0, 201.0));

    let mapped =
        scenarium_adapter::comparison_report(&rally, "proof").expect("comparison maps cleanly");

    assert_eq!(rally.status(), "improved");
    assert_eq!(mapped.status(), ComparisonStatus::Improved);
    assert_eq!(mapped.deltas().len(), rally.deltas.len());
}

#[test]
fn stricter_scenarium_failures_are_explicit() {
    let empty = RallyComparison::new("wavelength", "baseline", "candidate");

    assert_eq!(empty.status(), "empty");
    assert_eq!(
        scenarium_adapter::comparison_report(&empty, "proof"),
        Err(Error::EmptyMetricSet)
    );

    let non_finite = RallyComparison {
        subject: "wavelength".to_string(),
        baseline_id: "baseline".to_string(),
        candidate_id: "candidate".to_string(),
        deltas: vec![RallyDelta::higher_is_better("pass_rate", f64::NAN, 63.9)],
    };
    assert!(matches!(
        scenarium_adapter::comparison_report(&non_finite, "proof"),
        Err(Error::NonFiniteMetric(metric)) if metric == "pass_rate"
    ));
}
