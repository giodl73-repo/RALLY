//! Bounded first-party mechanics protocol; game policy stays in consumers.
use rally_core::{
    ActionBudget, BoardEventEntry, DiceRoller, DrawPile, PacketManifest, RollKeep, RunSeed,
    SimulationRun, TokenPool, TurnOrder, ValidationFinding, ValidationReport,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
pub const MAX_REQUEST: usize = 128 * 1024;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    seed: String,
    subject: String,
    seats: Vec<String>,
    deck: Vec<String>,
    tokens: BTreeMap<String, u32>,
    action_budget: u32,
    steps: Vec<Step>,
}
#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum Step {
    Roll {
        id: String,
        expression: String,
        keep: String,
        extra: u32,
    },
    Shuffle {
        id: String,
    },
    Draw {
        id: String,
        count: u32,
    },
    Discard {
        id: String,
        card: String,
    },
    Recycle {
        id: String,
    },
    Spend {
        id: String,
        token: String,
        amount: u32,
    },
    Gain {
        id: String,
        token: String,
        amount: u32,
    },
    AdvanceTurn {
        id: String,
    },
    SpendAction {
        id: String,
        amount: u32,
    },
    RefreshActions {
        id: String,
    },
}
impl Step {
    fn id(&self) -> &str {
        match self {
            Self::Roll { id, .. }
            | Self::Shuffle { id }
            | Self::Draw { id, .. }
            | Self::Discard { id, .. }
            | Self::Recycle { id }
            | Self::Spend { id, .. }
            | Self::Gain { id, .. }
            | Self::AdvanceTurn { id }
            | Self::SpendAction { id, .. }
            | Self::RefreshActions { id } => id,
        }
    }
    fn op(&self) -> &str {
        match self {
            Self::Roll { .. } => "roll",
            Self::Shuffle { .. } => "shuffle",
            Self::Draw { .. } => "draw",
            Self::Discard { .. } => "discard",
            Self::Recycle { .. } => "recycle",
            Self::Spend { .. } => "spend",
            Self::Gain { .. } => "gain",
            Self::AdvanceTurn { .. } => "advance_turn",
            Self::SpendAction { .. } => "spend_action",
            Self::RefreshActions { .. } => "refresh_actions",
        }
    }
}
fn keep(value: &str) -> Option<RollKeep> {
    match value {
        "sum" => Some(RollKeep::Sum),
        "highest" => Some(RollKeep::Highest),
        "lowest" => Some(RollKeep::Lowest),
        _ => None,
    }
}
fn identifier(text: &str, max: usize) -> bool {
    !text.trim().is_empty() && text.len() <= max
}
fn unique(values: &[String]) -> bool {
    values.iter().all(|s| identifier(s, 128))
        && values.iter().collect::<BTreeSet<_>>().len() == values.len()
}
fn admit(req: &Request) -> Result<(), String> {
    if !identifier(&req.seed, 256) || !identifier(&req.subject, 128) {
        return Err("Non-blank seed (256 UTF-8 bytes) and subject (128 bytes) required".into());
    }
    if req.seats.is_empty()
        || req.seats.len() > 16
        || !unique(&req.seats)
        || req.deck.len() > 64
        || !unique(&req.deck)
    {
        return Err(
            "Require 1–16 unique seat IDs, up to 64 unique card IDs; each ID 1–128 UTF-8 bytes"
                .into(),
        );
    }
    if req.tokens.len() > 32
        || req
            .tokens
            .iter()
            .any(|(name, count)| !identifier(name, 128) || *count > 1000)
        || req.action_budget > 100
    {
        return Err("Up to 32 named token pools (0–1,000 each), action budget 0–100".into());
    }
    if req.steps.is_empty() || req.steps.len() > 128 {
        return Err("Supply 1–128 uniquely named steps".into());
    }
    let mut ids = BTreeSet::new();
    for step in &req.steps {
        if !identifier(step.id(), 128) || !ids.insert(step.id()) {
            return Err("Step IDs must be unique, non-blank and at most 128 UTF-8 bytes".into());
        }
        match step {
            Step::Roll {
                expression,
                keep: k,
                extra,
                ..
            } => {
                if expression.len() > 64 {
                    return Err("Dice expression exceeds 64 UTF-8 bytes".into());
                }
                let spec = rally_core::parse_roll_expression(expression)?;
                if spec.count > 32
                    || spec.sides > 1000
                    || spec.modifier.unsigned_abs() > 10000
                    || *extra > 16
                    || keep(k).is_none()
                {
                    return Err("Dice bounds: count 1–32, sides 1–1,000, modifier ±10,000, extra 0–16, keep sum/highest/lowest".into());
                }
            }
            Step::Draw { count, .. } if *count > 64 => return Err("Draw count must be 0–64".into()),
            Step::Discard { card, .. } if !identifier(card, 128) => {
                return Err("Discard requires a non-blank card ID up to 128 bytes".into())
            }
            Step::Spend { token, amount, .. } | Step::Gain { token, amount, .. }
                if !req.tokens.contains_key(token) || *amount > 1000 =>
            {
                return Err(
                    "Token operation requires an existing token name and amount 0–1,000".into(),
                )
            }
            Step::SpendAction { amount, .. } if *amount > 100 => {
                return Err("Action spend must be 0–100".into())
            }
            _ => {}
        }
    }
    Ok(())
}
struct Engine {
    run: SimulationRun,
    dice: DiceRoller,
    deck_rng: RunSeed,
    pile: DrawPile<String>,
    hand: Vec<String>,
    turns: TurnOrder,
    tokens: TokenPool,
    budget: ActionBudget,
    token_names: Vec<String>,
    findings: Vec<ValidationFinding>,
    events: String,
    frames: Vec<Value>,
}
impl Engine {
    fn new(req: &Request) -> Self {
        let run = SimulationRun::new("rally-web", &req.subject, &req.seed);
        let dice = DiceRoller::new(&run.run_id);
        let deck_rng = run.rng();
        Self {
            run,
            dice,
            deck_rng,
            pile: DrawPile::new(req.deck.clone()),
            hand: vec![],
            turns: TurnOrder::new(req.seats.clone()),
            tokens: TokenPool::new(req.tokens.iter().map(|(k, v)| (k.clone(), *v as i32))),
            budget: ActionBudget::new(req.action_budget),
            token_names: req.tokens.keys().cloned().collect(),
            findings: vec![],
            events: String::new(),
            frames: vec![],
        }
    }
    fn snapshot(&self) -> Value {
        let counts: BTreeMap<_, _> = self
            .token_names
            .iter()
            .map(|name| (name, self.tokens.count(name)))
            .collect();
        json!({"active_seat":self.turns.active(),"round":self.turns.round(),"hand":self.hand,"draw_remaining":self.pile.remaining(),"discard_count":self.pile.discard_count(),"tokens":counts,"actions_remaining":self.budget.remaining(),"dice_roll_call_position":self.dice.position()})
    }
    fn warning(&mut self, step: &Step, code: &str, message: &str) {
        self.findings
            .push(ValidationFinding::warning(code, step.id(), message));
    }
    fn result(&mut self, step: &Step) -> Result<(Value, i32, Option<String>), String> {
        match step {
            Step::Roll {
                expression,
                keep: k,
                extra,
                ..
            } => {
                let mode = keep(k).ok_or("Unrecognized keep mode")?;
                let outcome = self.dice.roll_with_extra(expression, mode, *extra)?;
                Ok((
                    json!({"expression":outcome.spec.expression,"rolls":outcome.rolls,"kept":outcome.kept,"total":outcome.total,"native_roll_call_position":outcome.seed_position}),
                    outcome.total,
                    None,
                ))
            }
            Step::Shuffle { .. } => {
                self.pile.shuffle_draw(&mut self.deck_rng);
                Ok((
                    json!({"shuffled":self.pile.remaining()}),
                    self.pile.remaining() as i32,
                    None,
                ))
            }
            Step::Draw { count, .. } => {
                let mut cards = vec![];
                for _ in 0..*count {
                    if let Some(card) = self.pile.draw() {
                        self.hand.push(card.clone());
                        cards.push(card);
                    } else {
                        break;
                    }
                }
                if cards.len() < *count as usize {
                    self.warning(
                        step,
                        "RALLY-WEB-DRAW-SHORT",
                        "Draw request exceeded remaining cards; no implicit recycle",
                    );
                }
                let actual = cards.len() as i32;
                Ok((
                    json!({"requested":count,"cards":cards,"actual":actual}),
                    actual,
                    None,
                ))
            }
            Step::Discard { card, .. } => {
                let discarded = if let Some(index) = self.hand.iter().position(|c| c == card) {
                    self.pile.discard(self.hand.remove(index));
                    true
                } else {
                    self.findings.push(ValidationFinding::error(
                        "RALLY-WEB-CARD-NOT-IN-HAND",
                        step.id(),
                        "Discard blocked: card not in workbench hand",
                    ));
                    false
                };
                Ok((
                    json!({"card":card,"discarded":discarded}),
                    i32::from(discarded),
                    Some(card.clone()),
                ))
            }
            Step::Recycle { .. } => {
                let moved = self.pile.discard_count();
                self.pile.recycle_discard(&mut self.deck_rng);
                Ok((
                    json!({"discard_cards_recycled":moved,"remaining_draw_cards_also_shuffled":true}),
                    moved as i32,
                    None,
                ))
            }
            Step::Spend { token, amount, .. } => {
                let paid = self.tokens.spend(token, *amount as i32);
                if !paid {
                    self.warning(
                        step,
                        "RALLY-WEB-SPEND-BLOCKED",
                        "Native token spend returned false; counts unchanged",
                    );
                }
                Ok((
                    json!({"token":token,"requested":amount,"paid":paid}),
                    if paid { *amount as i32 } else { 0 },
                    Some(token.clone()),
                ))
            }
            Step::Gain { token, amount, .. } => {
                let total = self.tokens.gain(token, *amount as i32);
                Ok((
                    json!({"token":token,"gained":amount,"total":total}),
                    *amount as i32,
                    Some(token.clone()),
                ))
            }
            Step::AdvanceTurn { .. } => {
                self.turns.advance();
                Ok((
                    json!({"active_seat":self.turns.active(),"round":self.turns.round(),"automatic_budget_refresh":false}),
                    0,
                    None,
                ))
            }
            Step::SpendAction { amount, .. } => {
                let paid = self.budget.spend(*amount);
                if !paid {
                    self.warning(
                        step,
                        "RALLY-WEB-ACTIONS-BLOCKED",
                        "Native action spend returned false; budget unchanged",
                    );
                }
                Ok((
                    json!({"requested":amount,"paid":paid,"remaining":self.budget.remaining()}),
                    if paid { *amount as i32 } else { 0 },
                    None,
                ))
            }
            Step::RefreshActions { .. } => {
                self.budget.refresh();
                Ok((json!({"remaining":self.budget.remaining()}), 0, None))
            }
        }
    }
    fn execute(&mut self, step: &Step) -> Result<(), String> {
        let actor = self.turns.active().map(str::to_owned);
        let (result, quantity, target) = self.result(step)?;
        let detail = json!({"step_id":step.id(),"result":result}).to_string();
        self.events.push_str(
            &BoardEventEntry::new(
                &self.run.run_id,
                step.op(),
                actor.as_deref(),
                target.as_deref(),
                quantity,
                &detail,
            )
            .to_jsonl(),
        );
        self.frames.push(json!({"step_id":step.id(),"operation":step.op(),"result":result,"state":self.snapshot()}));
        Ok(())
    }
}
fn failure(phase: &str, message: &str) -> Value {
    json!({"schema":"rally.mechanics-workbench.v1","ok":false,"phase":phase,"diagnostic":{"message":message}})
}
fn response(input: &str) -> Value {
    if input.len() > MAX_REQUEST {
        return failure("request", "Request exceeds 128 KiB UTF-8");
    }
    let req: Request = match serde_json::from_str(input) {
        Ok(req) => req,
        Err(error) => return failure("request", &error.to_string()),
    };
    if let Err(error) = admit(&req) {
        return failure("admission", &error);
    }
    let mut engine = Engine::new(&req);
    for step in &req.steps {
        if let Err(error) = engine.execute(step) {
            return failure("execution", &error);
        }
    }
    let validation = ValidationReport {
        subject: req.subject.clone(),
        findings: engine.findings.clone(),
    };
    let mut manifest = PacketManifest::new(&format!("{}:mechanics-artifacts", engine.run.run_id));
    for (name, path) in [
        ("request", "rally-request.json"),
        ("events", "rally-events.jsonl"),
        ("validation", "rally-validation.json"),
        ("report", "rally-mechanics-report.json"),
    ] {
        manifest.add_artifact(name, path);
    }
    json!({"schema":"rally.mechanics-workbench.v1","ok":true,"request_json":input,"run":{"run_id":engine.run.run_id,"adapter":engine.run.adapter,"subject":engine.run.subject,"seed_label":engine.run.seed_label},"status":validation.status(),"events_jsonl":engine.events,"validation_json":validation.to_json(),"manifest_json":manifest.to_json(),"frames":engine.frames,"final_state":engine.snapshot(),"boundaries":{"validation":"this mechanics script only; no game balance, quality or migration approval","randomness":"deterministic native modulo-based RNG, not cryptographic or proven unbiased","streams":"dice and deck are separate objects seeded from the same native run ID","position":"native dice position counts roll calls, not individual random draws","identity":"run ID is not a digest of the complete request","manifest":"artifact path references, not integrity/authenticity proof","privacy":"authored fixtures only; own inputs stay in this tab"}})
}
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn replay(input: &str) -> String {
    response(input).to_string()
}
pub fn examples() -> Value {
    let base = json!({"seed":"example-alpha","subject":"neutral-mechanics","seats":["actor-a","actor-b"],"deck":["card-A","card-B","card-C","card-D"],"tokens":{"supply":4},"action_budget":3,"steps":[{"id":"shuffle-1","op":"shuffle"},{"id":"draw-1","op":"draw","count":2},{"id":"roll-1","op":"roll","expression":"2d6+3","keep":"sum","extra":0},{"id":"spend-1","op":"spend","token":"supply","amount":2},{"id":"actions-1","op":"spend_action","amount":1},{"id":"turn-1","op":"advance_turn"}]});
    let mut cases = serde_json::Map::new();
    cases.insert("script".into(), base.clone());
    let mut changed = base.clone();
    changed["seed"] = json!("example-beta");
    cases.insert("different-seed".into(), changed);
    let mut changed = base.clone();
    changed["steps"] = json!([{"id":"high","op":"roll","expression":"1d20+2","keep":"highest","extra":1},{"id":"low","op":"roll","expression":"1d20-1","keep":"lowest","extra":1}]);
    cases.insert("keep-modes".into(), changed);
    let mut changed = base.clone();
    changed["steps"] = json!([{"id":"draw-all","op":"draw","count":4},{"id":"discard-A","op":"discard","card":"card-A"},{"id":"recycle","op":"recycle"},{"id":"draw-again","op":"draw","count":1}]);
    cases.insert("recycle".into(), changed);
    let mut changed = base.clone();
    changed["steps"] = json!([{"id":"too-many","op":"draw","count":6},{"id":"too-costly","op":"spend","token":"supply","amount":9},{"id":"too-many-actions","op":"spend_action","amount":4}]);
    cases.insert("blocked".into(), changed);
    let mut changed = base.clone();
    changed["steps"] = json!([{"id":"missing-card","op":"discard","card":"not-in-hand"},{"id":"after-error","op":"gain","token":"supply","amount":1}]);
    cases.insert("structured-error".into(), changed);
    let mut changed = base.clone();
    changed["steps"] = json!([{"id":"spend-actions","op":"spend_action","amount":3},{"id":"turn","op":"advance_turn"},{"id":"still-empty","op":"spend_action","amount":1},{"id":"refresh","op":"refresh_actions"},{"id":"spend-again","op":"spend_action","amount":1}]);
    cases.insert("explicit-budget".into(), changed);
    Value::Object(cases)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_examples_and_native_json() {
        for (_, request) in examples().as_object().unwrap() {
            let input = request.to_string();
            assert_eq!(replay(&input), replay(&input));
            let r = response(&input);
            assert_eq!(r["ok"], true);
            serde_json::from_str::<Value>(r["validation_json"].as_str().unwrap()).unwrap();
            for line in r["events_jsonl"].as_str().unwrap().lines() {
                serde_json::from_str::<Value>(line).unwrap();
            }
        }
    }
    #[test]
    fn findings_keep_failed_actions_visible() {
        assert_eq!(
            response(&examples()["blocked"].to_string())["status"],
            "review"
        );
        let r = response(&examples()["structured-error"].to_string());
        assert_eq!(r["status"], "error");
        assert_eq!(r["final_state"]["tokens"]["supply"], 5);
    }
    #[test]
    fn card_conservation() {
        let r = response(&examples()["recycle"].to_string());
        for frame in r["frames"].as_array().unwrap() {
            let s = &frame["state"];
            assert_eq!(
                s["hand"].as_array().unwrap().len() as u64
                    + s["draw_remaining"].as_u64().unwrap()
                    + s["discard_count"].as_u64().unwrap(),
                4
            );
        }
    }
    #[test]
    fn explicit_budget_and_roll_position() {
        let r = response(&examples()["explicit-budget"].to_string());
        assert_eq!(r["frames"][1]["state"]["actions_remaining"], 0);
        assert_eq!(r["final_state"]["actions_remaining"], 2);
        let r = response(&examples()["keep-modes"].to_string());
        assert_eq!(r["final_state"]["dice_roll_call_position"], 2);
    }
    #[test]
    fn unicode_control_events_and_seed_streams_stay_native() {
        let mut req = examples()["script"].clone();
        req["seed"] = json!("Café 🌿\u{1}");
        req["subject"] = json!("<img src=x onerror=alert(1)>");
        let report = response(&req.to_string());
        assert_eq!(report["ok"], true);
        for line in report["events_jsonl"].as_str().unwrap().lines() {
            let event: Value = serde_json::from_str(line).unwrap();
            let _: Value = serde_json::from_str(event["detail"].as_str().unwrap()).unwrap();
        }
        let mut altered = req.clone();
        altered["steps"].as_array_mut().unwrap().remove(0);
        let changed = response(&altered.to_string());
        let roll = |r: &Value| {
            r["frames"]
                .as_array()
                .unwrap()
                .iter()
                .find(|f| f["step_id"] == "roll-1")
                .unwrap()["result"]
                .clone()
        };
        assert_eq!(roll(&report), roll(&changed));
    }
    #[test]
    fn maximum_gains_and_step_identity_are_bounded() {
        let mut req = examples()["script"].clone();
        req["steps"] = json!((0..128)
            .map(|i| json!({"id":format!("gain-{i}"),"op":"gain","token":"supply","amount":1000}))
            .collect::<Vec<_>>());
        let report = response(&req.to_string());
        assert_eq!(report["final_state"]["tokens"]["supply"], 128004);
        req["steps"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"extra","op":"gain","token":"supply","amount":1}));
        assert_eq!(response(&req.to_string())["ok"], false);
        let mut req = examples()["script"].clone();
        req["steps"][1]["id"] = req["steps"][0]["id"].clone();
        assert_eq!(response(&req.to_string())["ok"], false);
    }
    #[test]
    fn admission_prevents_overflow_and_unknown_operations() {
        for expression in [
            "0d6",
            "999999d6",
            "1d4294967295",
            "1d6-2147483648",
            "1d6+2147483647",
        ] {
            let mut req = examples()["script"].clone();
            req["steps"][2]["expression"] = json!(expression);
            assert_eq!(response(&req.to_string())["ok"], false);
        }
        assert_eq!(response("{}")["ok"], false);
        assert_eq!(response(&" ".repeat(MAX_REQUEST + 1))["ok"], false);
    }
}
