use rally_core::{ComparisonReport as RallyComparison, ValidationReport as RallyValidation};
use scenarium::{
    compare_runs, ComparisonReport, Error, Finding, Metric, MetricDirection, RunRecord, RunVariant,
    Scenario, Severity,
};

pub fn validation_run(report: &RallyValidation, seed_label: &str) -> Result<RunRecord, Error> {
    let scenario = Scenario::new(&report.subject, &report.subject, seed_label)?;
    let mut run = RunRecord::new(&scenario, RunVariant::Inertia, "rally")?;
    for finding in &report.findings {
        let severity = match finding.severity.as_str() {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            _ => Severity::Note,
        };
        run.add_finding(Finding::new(
            severity,
            &finding.code,
            &finding.location,
            &finding.message,
        )?);
    }
    Ok(run)
}

pub fn comparison_report(
    report: &RallyComparison,
    seed_label: &str,
) -> Result<ComparisonReport, Error> {
    let scenario = Scenario::new(&report.subject, &report.subject, seed_label)?;
    let mut baseline = RunRecord::new(
        &scenario,
        RunVariant::baseline(&report.baseline_id)?,
        "rally",
    )?;
    let mut candidate = RunRecord::new(
        &scenario,
        RunVariant::candidate(&report.candidate_id)?,
        "rally",
    )?;

    for delta in &report.deltas {
        let direction = match delta.direction.as_str() {
            "lower" => MetricDirection::LowerIsBetter,
            _ => MetricDirection::HigherIsBetter,
        };
        baseline.record_metric(Metric::new(
            &delta.metric,
            delta.baseline,
            direction.clone(),
        )?)?;
        candidate.record_metric(Metric::new(&delta.metric, delta.candidate, direction)?)?;
    }

    compare_runs(&baseline, &candidate)
}
