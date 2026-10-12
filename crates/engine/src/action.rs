//! The structured action schema. Agents submit exactly these variants; the engine
//! validates every one against ground truth and fog-of-war, then applies it.

use crate::map::Hex;
use crate::unit::UnitId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Action {
    /// Move a unit along an explicit ordered path of hexes (first = current hex,
    /// last = destination). Engine re-validates the whole path.
    Move { unit: UnitId, path: Vec<Hex> },
    /// Attack an enemy unit; optional air support (air units already committed).
    Attack {
        attacker: UnitId,
        target: UnitId,
        #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Hex;

    #[test]
    fn attack_action_uses_camelcase_air_support() {
        // The exact JSON shape the prompt tells models to emit.
        let json = r#"{"type":"attack","attacker":9,"target":4,"airSupport":[]}"#;
        let action: Action = serde_json::from_str(json).expect("deserialize");
        match &action {
            Action::Attack {
                attacker,
                target,
                air_support,
            } => {
                assert_eq!(*attacker, 9);
                assert_eq!(*target, 4);
                assert!(air_support.is_empty());
            }
            _ => panic!("expected Attack, got {action:?}"),
        }
        let out = serde_json::to_value(&action).unwrap();
        assert_eq!(out["type"], "attack");
        assert!(out.get("airSupport").is_some(), "round-trip must use airSupport");
        assert!(out.get("air_support").is_none());
    }

    #[test]
    fn attack_action_defaults_missing_air_support() {
        let json = r#"{"type":"attack","attacker":7,"target":1}"#;
        let action: Action = serde_json::from_str(json).expect("deserialize");
        match &action {
            Action::Attack { air_support, .. } => assert!(air_support.is_empty()),
            _ => panic!("expected Attack, got {action:?}"),
        }
    }

    #[test]
    fn move_action_roundtrip() {
        let json = r#"{"type":"move","unit":3,"path":[{"q":1,"r":2},{"q":1,"r":3}]}"#;
        let action: Action = serde_json::from_str(json).expect("deserialize");
        match &action {
            Action::Move { unit, path } => {
                assert_eq!(*unit, 3);
                assert_eq!(path, &vec![Hex::new(1, 2), Hex::new(1, 3)]);
            }
            _ => panic!("expected Move, got {action:?}"),
        }
    }
}
