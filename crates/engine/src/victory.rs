//! Victory conditions and outcomes.

use crate::side::Side;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum VictoryReason {
    /// One side's units are all eliminated.
    Elimination,
    /// Objective-control majority at the turn horizon.
    ObjectiveControl,
    /// Both sides survived the horizon with equal objectives.
    Draw,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub winner: Option<Side>,
    pub reason: VictoryReason,
    pub allied_score: i32,
    pub axis_score: i32,
}
