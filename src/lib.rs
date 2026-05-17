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

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    pub fn choose_index(&mut self, len: usize) -> Option<usize> {
        if len == 0 {
            return None;
        }
        Some(self.next_u32() as usize % len)
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
