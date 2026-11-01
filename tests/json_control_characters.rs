use rally_core::{BoardEventEntry, PacketManifest, ValidationFinding, ValidationReport};
use serde_json::Value;

#[test]
fn native_json_roundtrips_all_c0_controls_without_changing_strings() {
    let text = format!(
        "{} quotes=\" slash=\\ Café 🌿",
        (0..=31).map(char::from).collect::<String>()
    );
    let event = BoardEventEntry::new(&text, &text, Some(&text), Some(&text), -1, &text);
    let line = event.to_jsonl();
    assert_eq!(line.lines().count(), 1);
    let value: Value = serde_json::from_str(&line).unwrap();
    for field in ["run_id", "event_type", "actor", "target", "detail"] {
        assert_eq!(value[field], text);
    }
    let report = ValidationReport {
        subject: text.clone(),
        findings: vec![ValidationFinding::error(&text, &text, &text)],
    };
    let value: Value = serde_json::from_str(&report.to_json()).unwrap();
    assert_eq!(value["subject"], text);
    assert_eq!(value["findings"][0]["message"], text);
    let mut manifest = PacketManifest::new(&text);
    manifest.add_artifact(&text, &text);
    let value: Value = serde_json::from_str(&manifest.to_json()).unwrap();
    assert_eq!(value["packet_id"], text);
    assert_eq!(value["artifacts"][&text], text);
}
