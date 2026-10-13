//! Post-game analysis: deterministic stats + a timeline distilled from the
//! event log, plus an optional LLM-generated strategy narrative.

use cna_engine::{Event, Side};
use cna_models::{Agent, ModelError};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct GameSummary {
    pub winner: Option<Side>,
    pub reason: String,
    pub turns: u32,
    pub allied_score: i32,
    pub axis_score: i32,
    pub illegal_allied: u32,
    pub illegal_axis: u32,
    pub moves: u32,
    pub attacks: u32,
    pub air_recons: u32,
    pub supply_checks: u32,
    pub out_of_supply: u32,
    pub evaporation: u32,
    pub pasta: u32,
    pub allied_eliminations: Vec<String>,
    pub axis_eliminations: Vec<String>,
    pub reinforcements: Vec<String>,
    /// Combat result codes in chronological order (De/Dr/Ex/Ar/Ae/Ne).
    pub attack_results: Vec<String>,
    pub timeline: Vec<String>,
}

/// Reduce an event log to a compact summary. The log is self-contained: the
/// `GameOver` event carries the final outcome.
pub fn summarize(events: &[Event]) -> GameSummary {
    let mut s = GameSummary::default();

    for ev in events {
        match ev {
            Event::Move { turn, .. } => {
                s.turns = s.turns.max(*turn);
                s.moves += 1;
            }
            Event::Attack {
                turn, result, ..
            } => {
                s.turns = s.turns.max(*turn);
                s.attacks += 1;
                s.attack_results.push(result.clone());
            }
            Event::AirRecon { turn, .. } => {
                s.turns = s.turns.max(*turn);
                s.air_recons += 1;
            }
            Event::Supply {
                turn, status, ..
            } => {
                s.turns = s.turns.max(*turn);
                s.supply_checks += 1;
                if status == "outOfSupply" {
                    s.out_of_supply += 1;
                }
            }
            Event::Evaporation { turn, .. } => {
                s.turns = s.turns.max(*turn);
                s.evaporation += 1;
            }
            Event::Pasta { turn, .. } => {
                s.turns = s.turns.max(*turn);
                s.pasta += 1;
            }
            Event::Elimination { side, name, .. } => {
                match side {
                    Side::Allied => s.allied_eliminations.push(name.clone()),
                    Side::Axis => s.axis_eliminations.push(name.clone()),
                }
                s.timeline.push(format!("✕ {name} destroyed"));
            }
            Event::Reinforcement { name, .. } => {
                s.reinforcements.push(name.clone());
                s.timeline.push(format!("+ {name} arrives"));
            }
            Event::IllegalAction { side, .. } => match side {
                Side::Allied => s.illegal_allied += 1,
                Side::Axis => s.illegal_axis += 1,
            },
            Event::TurnStart { turn, side } => {
                s.turns = s.turns.max(*turn);
                s.timeline.push(format!("▸ Turn {turn} — {side}"));
            }
            Event::Victory { side, reason, .. } => {
                s.timeline.push(format!("★ {side} victory ({reason:?})"));
            }
            Event::GameOver { outcome } => {
                s.winner = outcome.winner;
                s.reason = format!("{:?}", outcome.reason);
                s.allied_score = outcome.allied_score;
                s.axis_score = outcome.axis_score;
            }
            _ => {}
        }
    }

    s
}

const ANALYSIS_SYSTEM: &str = "You are a wargame analyst. Given a match summary from an AI-vs-AI game of a Campaign-for-North-Africa-style desert wargame, write a concise explanation of the strategy and decision-making: what each side did, the key turning points, and why the outcome happened. Ground every claim in the stats and timeline. Use plain prose, ~150-250 words.";

/// Ask an LLM agent to explain the match.
pub async fn narrate(summary: &GameSummary, agent: &dyn Agent) -> Result<String, ModelError> {
    let user = format!(
        "Match summary:\n{}\n\nWrite the analysis.",
        serde_json::to_string_pretty(summary).unwrap_or_default()
    );
    agent.chat(ANALYSIS_SYSTEM, &user).await
}
