//! The structured action schema. Agents submit exactly these variants; the engine
//! validates every one against ground truth and fog-of-war, then applies it.

use crate::map::Hex;
use crate::unit::UnitId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    /// Move a unit along an explicit ordered path of hexes (first = current hex,
    /// last = destination). Engine re-validates the whole path.
    Move { unit: UnitId, path: Vec<Hex> },
    /// Attack an enemy unit; optional air support (air units already committed).
    Attack {
        attacker: UnitId,
        target: UnitId,
        air_support: Vec<UnitId>,
    },
    /// Recon a hex with an air squadron, revealing enemy units there.
    AirRecon { squadron: UnitId, target: Hex },
    Pass,
    EndTurn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionErrorKind {
    GameOver,
    NotYourTurn,
    UnknownUnit,
    EliminatedUnit,
    NotYourUnit,
    UnseenUnit,
    UnseenTarget,
    UnknownHex,
    ImpassableHex,
    PathDiscontinuous,
    NotEnoughMovement,
    OutOfFuel,
    OutOfSupply,
    NotAdjacent,
    NotInRange,
    NotAirUnit,
    NoMissions,
    InvalidTarget,
    AlreadyAttacked,
    CannotRetreat,
    EmptyPath,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionError {
    pub code: ActionErrorKind,
    pub message: String,
}

impl ActionError {
    pub fn new(code: ActionErrorKind, message: impl Into<String>) -> Self {
        ActionError {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}
