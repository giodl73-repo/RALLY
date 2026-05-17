use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSeed {
    state: u64,
}

impl RunSeed {
    pub fn new(seed: &str) -> Self {
        let mut state = 0xcbf2_9ce4_8422_2325u64;
        for byte in seed.bytes() {
            state ^= u64::from(byte);
            state = state.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self { state }
    }

    pub fn from_u64(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 32) as u32
    }

    pub fn next_bounded(&mut self, max: u32) -> u32 {
        if max == 0 {
            return 0;
        }
        self.next_u32() % max
    }

    pub fn choose_index(&mut self, len: usize) -> Option<usize> {
        if len == 0 {
            return None;
        }
        Some(self.next_bounded(len as u32) as usize)
    }

    pub fn percent_chance(&mut self, chance: u32) -> bool {
        self.next_bounded(100) < chance.min(100)
    }
}

pub fn sample_between(rng: &mut RunSeed, min: u32, max_inclusive: u32) -> u32 {
    if max_inclusive <= min {
        return min;
    }
    min + rng.next_bounded(max_inclusive - min + 1)
}

pub fn percent_of(numerator: u32, denominator: u32) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 * 100.0 / denominator as f64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationRun {
    pub run_id: String,
    pub adapter: String,
    pub subject: String,
    pub seed_label: String,
}

impl SimulationRun {
    pub fn new(adapter: &str, subject: &str, seed_label: &str) -> Self {
        Self {
            run_id: format!("{adapter}:{subject}:{seed_label}"),
            adapter: adapter.to_string(),
            subject: subject.to_string(),
            seed_label: seed_label.to_string(),
        }
    }

    pub fn rng(&self) -> RunSeed {
        RunSeed::new(&self.run_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorTrace {
    pub actor_id: String,
    pub role: String,
    pub actions: u32,
    pub blocked_turns: u32,
}

impl ActorTrace {
    pub fn new(actor_id: &str, role: &str) -> Self {
        Self {
            actor_id: actor_id.to_string(),
            role: role.to_string(),
            actions: 0,
            blocked_turns: 0,
        }
    }

    pub fn record_action(&mut self) {
        self.actions += 1;
    }

    pub fn record_blocked_turn(&mut self) {
        self.blocked_turns += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationMetric {
    pub name: String,
    pub value: f64,
}

impl SimulationMetric {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonDelta {
    pub metric: String,
    pub baseline: f64,
    pub candidate: f64,
    pub direction: String,
}

impl ComparisonDelta {
    pub fn higher_is_better(metric: &str, baseline: f64, candidate: f64) -> Self {
        Self {
            metric: metric.to_string(),
            baseline,
            candidate,
            direction: "higher".to_string(),
        }
    }

    pub fn lower_is_better(metric: &str, baseline: f64, candidate: f64) -> Self {
        Self {
            metric: metric.to_string(),
            baseline,
            candidate,
            direction: "lower".to_string(),
        }
    }

    pub fn change(&self) -> f64 {
        self.candidate - self.baseline
    }

    pub fn improved(&self) -> bool {
        if self.direction == "lower" {
            self.candidate < self.baseline
        } else {
            self.candidate > self.baseline
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonReport {
    pub subject: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub deltas: Vec<ComparisonDelta>,
}

impl ComparisonReport {
    pub fn new(subject: &str, baseline_id: &str, candidate_id: &str) -> Self {
        Self {
            subject: subject.to_string(),
            baseline_id: baseline_id.to_string(),
            candidate_id: candidate_id.to_string(),
            deltas: Vec::new(),
        }
    }

    pub fn add_delta(&mut self, delta: ComparisonDelta) {
        self.deltas.push(delta);
    }

    pub fn improved_count(&self) -> usize {
        self.deltas.iter().filter(|delta| delta.improved()).count()
    }

    pub fn status(&self) -> &'static str {
        if self.deltas.is_empty() {
            "empty"
        } else if self.improved_count() == self.deltas.len() {
            "improved"
        } else if self.improved_count() == 0 {
            "regressed"
        } else {
            "mixed"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeatRef {
    pub scenario_id: String,
    pub scene_id: String,
    pub beat_id: String,
}

impl BeatRef {
    pub fn new(scenario_id: &str, scene_id: &str, beat_id: &str) -> Self {
        Self {
            scenario_id: scenario_id.to_string(),
            scene_id: scene_id.to_string(),
            beat_id: beat_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventLogEntry {
    pub run_id: String,
    pub beat: BeatRef,
    pub event_type: String,
    pub message: String,
}

impl EventLogEntry {
    pub fn to_jsonl(&self) -> String {
        format!(
            "{{\"run_id\":\"{}\",\"scenario_id\":\"{}\",\"scene_id\":\"{}\",\"beat_id\":\"{}\",\"event_type\":\"{}\",\"message\":\"{}\"}}\n",
            escape_json(&self.run_id),
            escape_json(&self.beat.scenario_id),
            escape_json(&self.beat.scene_id),
            escape_json(&self.beat.beat_id),
            escape_json(&self.event_type),
            escape_json(&self.message),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    pub severity: String,
    pub code: String,
    pub location: String,
    pub message: String,
}

impl ValidationFinding {
    pub fn warning(code: &str, location: &str, message: &str) -> Self {
        Self {
            severity: "warning".to_string(),
            code: code.to_string(),
            location: location.to_string(),
            message: message.to_string(),
        }
    }

    pub fn error(code: &str, location: &str, message: &str) -> Self {
        Self {
            severity: "error".to_string(),
            code: code.to_string(),
            location: location.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub subject: String,
    pub findings: Vec<ValidationFinding>,
}

impl ValidationReport {
    pub fn status(&self) -> &'static str {
        if self
            .findings
            .iter()
            .any(|finding| finding.severity == "error")
        {
            "error"
        } else if self.findings.is_empty() {
            "pass"
        } else {
            "review"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketManifest {
    pub packet_id: String,
    pub artifacts: BTreeMap<String, String>,
}

impl PacketManifest {
    pub fn new(packet_id: &str) -> Self {
        Self {
            packet_id: packet_id.to_string(),
            artifacts: BTreeMap::new(),
        }
    }

    pub fn add_artifact(&mut self, name: &str, path: &str) {
        self.artifacts.insert(name.to_string(), path.to_string());
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_runs_are_repeatable() {
        let mut left = RunSeed::new("room-001:team-confused");
        let mut right = RunSeed::new("room-001:team-confused");

        assert_eq!(left.next_u32(), right.next_u32());
        assert_eq!(left.choose_index(7), right.choose_index(7));
    }

    #[test]
    fn numeric_seeds_support_bounded_sampling() {
        let mut rng = RunSeed::from_u64(42);
        for _ in 0..20 {
            let sample = sample_between(&mut rng, 2, 4);
            assert!((2..=4).contains(&sample));
        }
        assert_eq!(percent_of(1, 4), 25.0);
        assert_eq!(percent_of(1, 0), 0.0);
    }

    #[test]
    fn validation_report_status_reflects_findings() {
        let pass = ValidationReport {
            subject: "scenario".to_string(),
            findings: vec![],
        };
        let review = ValidationReport {
            subject: "scenario".to_string(),
            findings: vec![ValidationFinding::warning(
                "late-hint",
                "SCENES.md#beat-cards",
                "hint arrives after slow max",
            )],
        };
        let error = ValidationReport {
            subject: "scenario".to_string(),
            findings: vec![ValidationFinding::error(
                "missing-beat",
                "SCENES.md#beat-cards",
                "beat reference does not exist",
            )],
        };

        assert_eq!(pass.status(), "pass");
        assert_eq!(review.status(), "review");
        assert_eq!(error.status(), "error");
    }

    #[test]
    fn simulation_runs_create_repeatable_rngs() {
        let run = SimulationRun::new("hunt-sim", "wavelength", "smoke");
        let mut left = run.rng();
        let mut right = run.rng();

        assert_eq!(run.run_id, "hunt-sim:wavelength:smoke");
        assert_eq!(left.next_u32(), right.next_u32());
    }

    #[test]
    fn actor_traces_count_actions_and_blocks() {
        let mut trace = ActorTrace::new("solver-speedster", "solver");
        trace.record_action();
        trace.record_blocked_turn();

        assert_eq!(trace.actions, 1);
        assert_eq!(trace.blocked_turns, 1);
    }

    #[test]
    fn comparison_reports_classify_improvement() {
        let mut report = ComparisonReport::new("wavelength", "baseline", "guided-final-set");
        report.add_delta(ComparisonDelta::higher_is_better("pass_rate", 38.9, 63.9));
        report.add_delta(ComparisonDelta::lower_is_better(
            "p95_minutes",
            215.0,
            201.0,
        ));

        assert_eq!(report.status(), "improved");
        assert_eq!(report.improved_count(), 2);
        assert_eq!(report.deltas[0].change(), 25.0);
    }

    #[test]
    fn event_entries_emit_jsonl() {
        let entry = EventLogEntry {
            run_id: "SIM-001".to_string(),
            beat: BeatRef::new("room-001", "scene-01", "beat-02"),
            event_type: "hint".to_string(),
            message: "operator said \"look at the dial\"".to_string(),
        };

        assert!(entry.to_jsonl().contains("\"run_id\":\"SIM-001\""));
        assert!(entry.to_jsonl().contains("\\\"look at the dial\\\""));
    }
}
