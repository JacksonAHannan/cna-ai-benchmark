//! The append-only event log — the replay source and the substrate for the live
//! spectator report.

use crate::action::Action;
use crate::map::Hex;
use crate::side::Side;
use crate::unit::UnitId;
use crate::victory::{Outcome, VictoryReason};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    GameStart {
        scenario: String,
        seed: u64,
        turn_horizon: u32,
    },
    TurnStart {
        turn: u32,
        side: Side,
    },
    PhaseStart {
        turn: u32,
        side: Side,
        phase: String,
    },
    Supply {
        turn: u32,
        side: Side,
        unit: UnitId,
        name: String,
        status: String,
    },
    /// Fuel evaporation — CNA's signature 3% / 7%-British detail.
    Evaporation {
        turn: u32,
        side: Side,
        subject: String,
        amount: u32,
        rate_percent: u8,
    },
    Pasta {
        turn: u32,
        unit: UnitId,
        name: String,
        note: String,
    },
    Move {
        turn: u32,
        unit: UnitId,
        from: Hex,
        to: Hex,
        mp_spent: u8,
        fuel_spent: u8,
    },
    Attack {
        turn: u32,
        attacker: UnitId,
        defender: UnitId,
        odds_numerator: u32,
        odds_denominator: u32,
        result: String,
        attacker_losses: u8,
        defender_losses: u8,
    },
    Retreat {
        turn: u32,
        unit: UnitId,
        from: Hex,
        to: Hex,
    },
    Elimination {
        turn: u32,
        unit: UnitId,
        name: String,
        side: Side,
    },
    AirSupport {
        turn: u32,
        squadron: UnitId,
        target: Hex,
    },
    AirRecon {
        turn: u32,
        squadron: UnitId,
        target: Hex,
        revealed: Vec<UnitId>,
    },
    Reinforcement {
        turn: u32,
        unit: UnitId,
        name: String,
        side: Side,
        hex: Hex,
    },
    IllegalAction {
        turn: u32,
        side: Side,
        action: Action,
        error: String,
    },
    TurnEnd {
        turn: u32,
        side: Side,
    },
    Victory {
        turn: u32,
        side: Side,
        reason: VictoryReason,
    },
    GameOver {
        outcome: Outcome,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<Event>,
    pub seq: u64,
}

impl EventLog {
    pub fn push(&mut self, e: Event) {
        self.seq += 1;
        self.events.push(e);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
