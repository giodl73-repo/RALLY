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
pub struct RollSpec {
    pub expression: String,
    pub count: u32,
    pub sides: u32,
    pub modifier: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollKeep {
    Sum,
    Highest,
    Lowest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollOutcome {
    pub spec: RollSpec,
    pub rolls: Vec<u32>,
    pub kept: Option<u32>,
    pub total: i32,
    pub seed_position: u64,
}

pub struct DiceRoller {
    rng: RunSeed,
    position: u64,
}

impl DiceRoller {
    pub fn new(seed: &str) -> Self {
        Self {
            rng: RunSeed::new(seed),
            position: 0,
        }
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn roll(&mut self, expression: &str) -> Result<RollOutcome, String> {
        self.roll_keep(expression, RollKeep::Sum)
    }

    pub fn roll_keep(&mut self, expression: &str, keep: RollKeep) -> Result<RollOutcome, String> {
        self.roll_with_extra(expression, keep, 0)
    }

    pub fn roll_with_extra(
        &mut self,
        expression: &str,
        keep: RollKeep,
        extra_rolls: u32,
    ) -> Result<RollOutcome, String> {
        let spec = parse_roll_expression(expression)?;
        let modifier = spec.modifier;
        let mut rolls = (0..(spec.count + extra_rolls))
            .map(|_| self.roll_die_value(spec.sides))
            .collect::<Vec<_>>();
        let kept = match keep {
            RollKeep::Sum => None,
            RollKeep::Highest => rolls.iter().copied().max(),
            RollKeep::Lowest => rolls.iter().copied().min(),
        };
        let base = kept.unwrap_or_else(|| rolls.iter().sum());
        let outcome = RollOutcome {
            spec,
            rolls: {
                rolls.shrink_to_fit();
                rolls
            },
            kept,
            total: base as i32 + modifier,
            seed_position: self.position,
        };
        self.position += 1;
        Ok(outcome)
    }

    pub fn roll_die_value(&mut self, sides: u32) -> u32 {
        if sides == 0 {
            0
        } else {
            self.rng.next_bounded(sides) + 1
        }
    }
}

pub fn parse_roll_expression(expression: &str) -> Result<RollSpec, String> {
    let expr = expression.trim().to_ascii_lowercase();
    let Some((count_text, rest)) = expr.split_once('d') else {
        return Err(format!("invalid expression: {expression}"));
    };
    let count = count_text
        .parse::<u32>()
        .map_err(|_| format!("invalid expression: {expression}"))?;
    let split_at = rest.find(['+', '-']).unwrap_or(rest.len());
    let sides = rest[..split_at]
        .parse::<u32>()
        .map_err(|_| format!("invalid expression: {expression}"))?;
    let modifier = if split_at < rest.len() {
        rest[split_at..]
            .parse::<i32>()
            .map_err(|_| format!("invalid expression: {expression}"))?
    } else {
        0
    };
    if count == 0 || sides == 0 {
        return Err(format!("invalid expression: {expression}"));
    }
    Ok(RollSpec {
        expression: expression.to_string(),
        count,
        sides,
        modifier,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnOrder {
    seats: Vec<String>,
    active: usize,
    round: u32,
}

impl TurnOrder {
    pub fn new(seats: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            seats: seats.into_iter().map(Into::into).collect(),
            active: 0,
            round: 1,
        }
    }

    pub fn active(&self) -> Option<&str> {
        self.seats.get(self.active).map(String::as_str)
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn advance(&mut self) -> Option<&str> {
        if self.seats.is_empty() {
            return None;
        }
        self.active = (self.active + 1) % self.seats.len();
        if self.active == 0 {
            self.round += 1;
        }
        self.active()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreTrack {
    scores: BTreeMap<String, i32>,
}

impl ScoreTrack {
    pub fn new(seats: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            scores: seats.into_iter().map(|seat| (seat.into(), 0)).collect(),
        }
    }

    pub fn add(&mut self, seat: &str, delta: i32) -> i32 {
        let score = self.scores.entry(seat.to_string()).or_insert(0);
        *score += delta;
        *score
    }

    pub fn get(&self, seat: &str) -> i32 {
        self.scores.get(seat).copied().unwrap_or(0)
    }

    pub fn leader(&self) -> Option<(&str, i32)> {
        self.scores
            .iter()
            .max_by_key(|(_, score)| *score)
            .map(|(seat, score)| (seat.as_str(), *score))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPool {
    tokens: BTreeMap<String, i32>,
}

impl TokenPool {
    pub fn new(tokens: impl IntoIterator<Item = (impl Into<String>, i32)>) -> Self {
        Self {
            tokens: tokens
                .into_iter()
                .map(|(name, count)| (name.into(), count.max(0)))
                .collect(),
        }
    }

    pub fn count(&self, token: &str) -> i32 {
        self.tokens.get(token).copied().unwrap_or(0)
    }

    pub fn spend(&mut self, token: &str, count: i32) -> bool {
        if count <= 0 || self.count(token) < count {
            return false;
        }
        if let Some(available) = self.tokens.get_mut(token) {
            *available -= count;
            true
        } else {
            false
        }
    }

    pub fn gain(&mut self, token: &str, count: i32) -> i32 {
        let available = self.tokens.entry(token.to_string()).or_insert(0);
        *available += count.max(0);
        *available
    }
}

pub fn shuffle_with_seed<T>(rng: &mut RunSeed, items: &mut [T]) {
    for index in (1..items.len()).rev() {
        let swap_with = rng.next_bounded((index + 1) as u32) as usize;
        items.swap(index, swap_with);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawPile<T> {
    draw: Vec<T>,
    discard: Vec<T>,
}

impl<T> DrawPile<T> {
    pub fn new(cards: impl IntoIterator<Item = T>) -> Self {
        Self {
            draw: cards.into_iter().collect(),
            discard: Vec::new(),
        }
    }

    pub fn remaining(&self) -> usize {
        self.draw.len()
    }

    pub fn discard_count(&self) -> usize {
        self.discard.len()
    }

    pub fn shuffle_draw(&mut self, rng: &mut RunSeed) {
        shuffle_with_seed(rng, &mut self.draw);
    }

    pub fn draw(&mut self) -> Option<T> {
        self.draw.pop()
    }

    pub fn discard(&mut self, card: T) {
        self.discard.push(card);
    }

    pub fn recycle_discard(&mut self, rng: &mut RunSeed) {
        self.draw.append(&mut self.discard);
        self.shuffle_draw(rng);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseTrack {
    phases: Vec<String>,
    active: usize,
}

impl PhaseTrack {
    pub fn new(phases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            phases: phases.into_iter().map(Into::into).collect(),
            active: 0,
        }
    }

    pub fn active(&self) -> Option<&str> {
        self.phases.get(self.active).map(String::as_str)
    }

    pub fn advance(&mut self) -> Option<&str> {
        if self.phases.is_empty() {
            return None;
        }
        self.active = (self.active + 1) % self.phases.len();
        self.active()
    }

    pub fn set_active(&mut self, phase: &str) -> bool {
        if let Some(index) = self.phases.iter().position(|candidate| candidate == phase) {
            self.active = index;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBudget {
    max: u32,
    remaining: u32,
}

impl ActionBudget {
    pub fn new(max: u32) -> Self {
        Self {
            max,
            remaining: max,
        }
    }

    pub fn remaining(&self) -> u32 {
        self.remaining
    }

    pub fn spend(&mut self, count: u32) -> bool {
        if count == 0 || self.remaining < count {
            return false;
        }
        self.remaining -= count;
        true
    }

    pub fn refresh(&mut self) {
        self.remaining = self.max;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn orthogonal_neighbors(&self) -> [GridPoint; 4] {
        [
            GridPoint::new(self.x, self.y - 1),
            GridPoint::new(self.x + 1, self.y),
            GridPoint::new(self.x, self.y + 1),
            GridPoint::new(self.x - 1, self.y),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RectGrid {
    pub width: i32,
    pub height: i32,
}

impl RectGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width.max(0),
            height: height.max(0),
        }
    }

    pub fn contains(&self, point: GridPoint) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }

    pub fn orthogonal_neighbors(&self, point: GridPoint) -> Vec<GridPoint> {
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|neighbor| self.contains(*neighbor))
            .collect()
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
    fn dice_rolls_are_repeatable_and_support_keep_modes() {
        let mut left = DiceRoller::new("quest-scene");
        let mut right = DiceRoller::new("quest-scene");

        let l = left
            .roll_with_extra("1d20+5", RollKeep::Highest, 1)
            .expect("roll succeeds");
        let r = right
            .roll_with_extra("1d20+5", RollKeep::Highest, 1)
            .expect("roll succeeds");

        assert_eq!(l, r);
        assert_eq!(l.spec.count, 1);
        assert_eq!(l.spec.sides, 20);
        assert_eq!(l.spec.modifier, 5);
        assert_eq!(l.rolls.len(), 2);
        assert_eq!(l.kept, l.rolls.iter().copied().max());
        assert_eq!(l.seed_position, 0);
        assert_eq!(left.position(), 1);
    }

    #[test]
    fn board_primitives_track_turns_scores_and_tokens() {
        let mut turns = TurnOrder::new(["human", "ai"]);
        assert_eq!(turns.active(), Some("human"));
        assert_eq!(turns.advance(), Some("ai"));
        assert_eq!(turns.advance(), Some("human"));
        assert_eq!(turns.round(), 2);

        let mut scores = ScoreTrack::new(["human", "ai"]);
        scores.add("human", 3);
        scores.add("ai", 1);
        assert_eq!(scores.leader(), Some(("human", 3)));

        let mut tokens = TokenPool::new([("tiger", 2), ("coin", 1)]);
        assert!(tokens.spend("tiger", 1));
        assert!(!tokens.spend("coin", 2));
        assert_eq!(tokens.gain("coin", 2), 3);
    }

    #[test]
    fn draw_piles_shuffle_draw_and_recycle_repeatably() {
        let mut left = DrawPile::new(["a", "b", "c", "d"]);
        let mut right = DrawPile::new(["a", "b", "c", "d"]);
        let mut left_rng = RunSeed::new("deck-seed");
        let mut right_rng = RunSeed::new("deck-seed");

        left.shuffle_draw(&mut left_rng);
        right.shuffle_draw(&mut right_rng);

        assert_eq!(left.draw(), right.draw());
        let card = left.draw().expect("card remains");
        left.discard(card);
        assert_eq!(left.discard_count(), 1);
        left.recycle_discard(&mut left_rng);
        assert_eq!(left.discard_count(), 0);
        assert_eq!(left.remaining(), 3);
    }

    #[test]
    fn phase_action_and_grid_helpers_cover_common_table_flow() {
        let mut phases = PhaseTrack::new(["draft", "action", "cleanup"]);
        assert_eq!(phases.active(), Some("draft"));
        assert_eq!(phases.advance(), Some("action"));
        assert!(phases.set_active("cleanup"));
        assert_eq!(phases.active(), Some("cleanup"));
        assert!(!phases.set_active("missing"));

        let mut actions = ActionBudget::new(2);
        assert!(actions.spend(1));
        assert!(!actions.spend(2));
        actions.refresh();
        assert_eq!(actions.remaining(), 2);

        let grid = RectGrid::new(3, 2);
        let neighbors = grid.orthogonal_neighbors(GridPoint::new(1, 0));
        assert_eq!(
            neighbors,
            vec![
                GridPoint::new(2, 0),
                GridPoint::new(1, 1),
                GridPoint::new(0, 0)
            ]
        );
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
