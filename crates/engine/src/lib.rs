//! `cna-engine` — the pure, deterministic rules engine.
//!
//! No I/O, no randomness sources except the serializable `SplitMix64` PRNG seeded
//! from the game seed. The same state + the same actions always produce the same
//! event log. This crate is the single authority on game truth; agents (models)
//! only ever submit [`Action`]s which are validated against ground truth and the
//! submitting side's fog-of-war view.

pub mod action;
pub mod events;
pub mod fog;
pub mod map;
pub mod rng;
pub mod rules;
pub mod scenario;
pub mod side;
pub mod state;
pub mod unit;
pub mod victory;

pub use action::{Action, ActionError, ActionErrorKind};
pub use events::{Event, EventLog};
pub use fog::Observation;
pub use map::{Hex, Map, MapHex, Site, Terrain};
pub use scenario::{default_scenario, Scenario};
pub use side::{Nationality, Side};
pub use state::{GameState, Phase};
pub use unit::{SupplyStatus, Unit, UnitId, UnitType};
pub use victory::{Outcome, VictoryReason};
