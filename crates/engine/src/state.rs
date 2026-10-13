//! Game state and turn sequencing. The engine owns all mutation; agents only
//! submit [`Action`]s via [`GameState::apply`].

use crate::events::{Event, EventLog};
use crate::map::{Hex, Map, Site};
use crate::rng::Rng;
use crate::scenario::{Scenario, UnitSpec};
use crate::side::Side;
use crate::unit::{SupplyStatus, Unit, UnitId};
use crate::victory::{Outcome, VictoryReason};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Phase {
    Supply,
    Attrition,
    Movement,
    Combat,
    Air,
    RearArea,
    GameOver,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub scenario_name: String,
    pub turn_horizon: u32,
    pub seed: u64,
    pub map: Map,
    pub units: HashMap<UnitId, Unit>,
    pub turn: u32,
    pub stage: u8,
    pub phase: Phase,
    pub active: Side,
    pub(crate) first_this_turn: Side,
    pub winner: Option<Side>,
    pub game_over: bool,
    pub log: EventLog,
    pub(crate) rng: Rng,
    pub(crate) next_unit_id: UnitId,
    /// Hexes revealed by air reconnaissance (persistent strategic picture).
    pub(crate) recon: HashMap<Side, HashSet<Hex>>,
    pub(crate) depot_damage: HashMap<Hex, u32>,
    pub(crate) axis_convoy: u32,
    pub(crate) reinforcements: Vec<(u32, UnitSpec)>,
}

impl GameState {
    pub fn new(scenario: Scenario, seed: u64) -> Self {
        let mut units = HashMap::new();
        let mut next_id: UnitId = 1;
        for spec in &scenario.units {
            let hex = Hex::from_offset(spec.row, spec.col);
            let unit = Unit::new(
                next_id,
                &spec.name,
                spec.side,
                spec.nationality,
                spec.kind,
                spec.strength,
                hex,
            );
            units.insert(next_id, unit);
            next_id += 1;
        }

        let mut rng = Rng::new(seed);
        let first = if rng.below(2) == 0 {
            Side::Allied
        } else {
            Side::Axis
        };

        let mut state = GameState {
            scenario_name: scenario.name,
            turn_horizon: scenario.turn_horizon,
            seed,
            map: scenario.map,
            units,
            turn: 1,
            stage: 1,
            phase: Phase::Supply,
            active: first,
            first_this_turn: first,
            winner: None,
            game_over: false,
            log: EventLog::default(),
            rng,
            next_unit_id: next_id,
            recon: HashMap::from([
                (Side::Allied, HashSet::new()),
                (Side::Axis, HashSet::new()),
            ]),
            depot_damage: HashMap::new(),
            axis_convoy: 0,
            reinforcements: scenario.reinforcements,
        };

        state.log.push(Event::GameStart {
            scenario: state.scenario_name.clone(),
            seed,
            turn_horizon: state.turn_horizon,
        });
        state.begin_week();
        state
    }

    // ---- turn sequencing (also called from rules on EndTurn) ----

    fn roll_first(&mut self) -> Side {
        if self.rng.below(2) == 0 {
            Side::Allied
        } else {
            Side::Axis
        }
    }

    /// Start a new Game-Turn (week): reinforcements, initiative, weekly supply and
    /// attrition for both sides, then the first Operations Stage.
    pub(crate) fn begin_week(&mut self) {
        self.place_reinforcements();
        self.stage = 1;
        self.first_this_turn = self.roll_first();
        self.axis_convoy = self.rng.between(15, 45) as u32;
        self.log.push(Event::Convoy {
            turn: self.turn,
            tons: self.axis_convoy,
        });
        self.log.push(Event::TurnStart {
            turn: self.turn,
            side: self.first_this_turn,
        });
        for side in [Side::Allied, Side::Axis] {
            self.active = side;
            self.run_supply();
            self.weekly_attrition();
        }
        self.active = self.first_this_turn;
        self.reset_stage();
        self.phase = Phase::Movement;
    }

    pub(crate) fn end_side_turn(&mut self) {
        self.log.push(Event::TurnEnd {
            turn: self.turn,
            side: self.active,
        });
        if self.check_elimination() {
            return;
        }
        if self.active == self.first_this_turn.other() {
            // Both sides have acted this Operations Stage.
            if self.stage >= 3 {
                if self.turn >= self.turn_horizon {
                    self.resolve_horizon();
                    return;
                }
                self.turn += 1;
                self.begin_week();
            } else {
                self.stage += 1;
                self.first_this_turn = self.roll_first();
                self.active = self.first_this_turn;
                self.reset_stage();
            }
        } else {
            self.active = self.first_this_turn.other();
            self.reset_stage();
        }
    }

    fn place_reinforcements(&mut self) {
        let mut to_place = Vec::new();
        let mut remaining = Vec::new();
        for (turn, spec) in std::mem::take(&mut self.reinforcements) {
            if turn <= self.turn {
                to_place.push(spec);
            } else {
                remaining.push((turn, spec));
            }
        }
        self.reinforcements = remaining;

        for spec in to_place {
            let hex = Hex::from_offset(spec.row, spec.col);
            let id = self.next_unit_id;
            self.next_unit_id += 1;
            let unit = Unit::new(
                id,
                &spec.name,
                spec.side,
                spec.nationality,
                spec.kind,
                spec.strength,
                hex,
            );
            self.log.push(Event::Reinforcement {
                turn: self.turn,
                unit: id,
                name: unit.name.clone(),
                side: unit.side,
                kind: unit.kind,
                strength: unit.strength,
                hex,
            });
            self.units.insert(id, unit);
        }
    }

    // ---- supply ----

    fn depot_capacity(&self, hex: Hex) -> u32 {
        self.map
            .get(hex)
            .and_then(|m| m.site.as_ref())
            .and_then(|s| match s {
                Site::Depot { capacity, .. } => Some(*capacity),
                _ => None,
            })
            .unwrap_or(0)
    }

    /// A unit can trace supply to a depot if it is on the depot, both are on the
    /// road/coast network, or the depot is within the off-road radius.
    fn traces_to(&self, unit_hex: Hex, depot: Hex) -> bool {
        if unit_hex == depot {
            return true;
        }
        let uh = self.map.get(unit_hex).map(|m| m.terrain);
        let dh = self.map.get(depot).map(|m| m.terrain);
        if let (Some(uh), Some(dh)) = (uh, dh) {
            if uh.is_road_network() && dh.is_road_network() {
                return true;
            }
        }
        unit_hex.distance(depot) <= 4
    }

    fn run_supply(&mut self) {
        let side = self.active;
        let depots = self.map.depots_of(side);
        let port_capacity: u32 = depots
            .iter()
            .map(|&d| {
                self.depot_capacity(d)
                    .saturating_sub(self.depot_damage.get(&d).copied().unwrap_or(0))
            })
            .sum();
        // Axis supply arrives by sea (variable convoys); Allies draw on unlimited
        // reserves but remain bounded by port capacity.
        let capacity = if side == Side::Axis {
            port_capacity.min(self.axis_convoy)
        } else {
            port_capacity
        };

        // Sort active-side units nearest-to-depot first so that, when demand
        // exceeds capacity, the farthest units starve first (faithful to logistics).
        let mut ids: Vec<UnitId> = self
            .units
            .values()
            .filter(|u| !u.eliminated() && u.side == side)
            .map(|u| u.id)
            .collect();
        ids.sort_by_key(|&id| {
            let hex = self.units[&id].hex.unwrap();
            depots
                .iter()
                .map(|&d| hex.distance(d))
                .min()
                .unwrap_or(i32::MAX)
        });

        let mut remaining = capacity;
        for id in ids {
            let (hex, demand, name) = {
                let unit = self.units.get(&id).unwrap();
                let fuel_need = unit.fuel_capacity.saturating_sub(unit.fuel) as u32;
                let ammo_need = unit.ammo_capacity.saturating_sub(unit.ammo) as u32;
                let demand = unit.stores_draw() + unit.water_draw() + fuel_need + ammo_need;
                (unit.hex.unwrap(), demand, unit.name.clone())
            };
            let traced = depots.iter().any(|&d| self.traces_to(hex, d));
            let in_supply = traced && demand <= remaining;
            if in_supply {
                remaining -= demand;
            }
            let unit = self.units.get_mut(&id).unwrap();
            unit.supply = if in_supply {
                SupplyStatus::InSupply
            } else {
                SupplyStatus::OutOfSupply
            };
            self.log.push(Event::Supply {
                turn: self.turn,
                side,
                unit: id,
                name,
                status: if in_supply { "inSupply" } else { "outOfSupply" }.to_string(),
            });
        }
    }

    fn weekly_attrition(&mut self) {
        let side = self.active;

        // Collect ids first to satisfy the borrow checker.
        let ids: Vec<UnitId> = self
            .units
            .values()
            .filter(|u| !u.eliminated() && u.side == side)
            .map(|u| u.id)
            .collect();

        for id in ids {
            let unit = self.units.get_mut(&id).unwrap();

            // Refuel/rearm/resupply in-supply units from the supply line.
            if unit.supply == SupplyStatus::InSupply {
                unit.ammo = unit.ammo_capacity;
                unit.water = unit.water_capacity;
                if unit.fuel_capacity > 0 {
                    unit.fuel = unit.fuel_capacity;
                }
            } else {
                // Out of supply: water depletes (Italian pasta draws it faster).
                let w = unit.water_draw() as u8;
                unit.water = unit.water.saturating_sub(w);
            }

            // CNA signature: fuel evaporates each Game-Turn (research-corrected
            // 6% baseline / 9% Commonwealth).
            if unit.fuel > 0 {
                let rate: u8 = unit.nationality.fuel_evaporation_percent();
                let amount = ((unit.fuel as u32 * rate as u32 + 99) / 100) as u8;
                let amount = amount.max(1);
                unit.fuel = unit.fuel.saturating_sub(amount);
                let subject = format!("{} (fuel {})", unit.name, unit.fuel);
                self.log.push(Event::Evaporation {
                    turn: self.turn,
                    side,
                    subject,
                    amount: amount as u32,
                    rate_percent: rate,
                });
            }

            // Pasta rule: Italian units draw extra water.
            if unit.nationality.needs_pasta_water() {
                let name = unit.name.clone();
                self.log.push(Event::Pasta {
                    turn: self.turn,
                    unit: id,
                    name,
                    note: "extra water demand for pasta (+1 supply draw)".to_string(),
                });
            }

            // Out-of-supply attrition.
            if unit.supply == SupplyStatus::OutOfSupply {
                unit.strength = unit.strength.saturating_sub(1);
                if unit.strength == 0 {
                    self.eliminate_unit(id, "attrition (out of supply)");
                }
            }

            // Air squadrons refit: recover a share of the reserve (§38.37).
            if unit.kind == UnitType::Air {
                let reserve = unit.max_strength.saturating_sub(unit.strength);
                if reserve > 0 {
                    let pct = crate::charts::refit_percent(self.rng.between(1, 9) as u8);
                    let recovered = ((reserve as u32 * pct as u32 + 99) / 100) as u8;
                    unit.strength = unit.strength.saturating_add(recovered.max(1));
                }
            }
        }
    }

    /// Per-Operations-Stage reset: fresh CP allowance, missions, and attack flag.
    fn reset_stage(&mut self) {
        let side = self.active;
        let ids: Vec<UnitId> = self
            .units
            .values()
            .filter(|u| !u.eliminated() && u.side == side)
            .map(|u| u.id)
            .collect();
        for id in ids {
            let unit = self.units.get_mut(&id).unwrap();
            let mut allowance = unit.kind.movement_allowance();
            if unit.supply == SupplyStatus::OutOfSupply {
                allowance = (allowance / 2).max(1);
            }
            unit.movement_allowance = allowance;
            unit.movement = allowance;
            unit.missions = unit.missions_allowance;
            unit.has_attacked = false;
        }
    }

    pub(crate) fn eliminate_unit(&mut self, id: UnitId, _cause: &str) {
        let unit = self.units.get_mut(&id).unwrap();
        unit.hex = None;
        unit.strength = 0;
        unit.movement = 0;
        let name = unit.name.clone();
        let side = unit.side;
        self.log.push(Event::Elimination {
            turn: self.turn,
            unit: id,
            name,
            side,
        });
    }

    // ---- victory ----

    pub fn check_elimination(&mut self) -> bool {
        let allied = self
            .units
            .values()
            .filter(|u| !u.eliminated() && u.side == Side::Allied)
            .count();
        let axis = self
            .units
            .values()
            .filter(|u| !u.eliminated() && u.side == Side::Axis)
            .count();

        if allied == 0 && axis == 0 {
            self.finish(Outcome {
                winner: None,
                reason: VictoryReason::Draw,
                allied_score: 0,
                axis_score: 0,
            });
            return true;
        }
        if allied == 0 {
            self.finish(Outcome {
                winner: Some(Side::Axis),
                reason: VictoryReason::Elimination,
                allied_score: 0,
                axis_score: self.objectives_controlled(Side::Axis),
            });
            return true;
        }
        if axis == 0 {
            self.finish(Outcome {
                winner: Some(Side::Allied),
                reason: VictoryReason::Elimination,
                allied_score: self.objectives_controlled(Side::Allied),
                axis_score: 0,
            });
            return true;
        }
        false
    }

    pub fn objectives_controlled(&self, side: Side) -> i32 {
        let mut score = 0;
        for &hex in &self.map.objectives {
            let occupant = self
                .units
                .values()
                .find(|u| !u.eliminated() && u.hex == Some(hex));
            if let Some(u) = occupant {
                if u.side == side {
                    score += 1;
                }
            }
        }
        score
    }

    fn resolve_horizon(&mut self) {
        let allied = self.objectives_controlled(Side::Allied);
        let axis = self.objectives_controlled(Side::Axis);
        let (winner, reason) = if allied > axis {
            (Some(Side::Allied), VictoryReason::ObjectiveControl)
        } else if axis > allied {
            (Some(Side::Axis), VictoryReason::ObjectiveControl)
        } else {
            (None, VictoryReason::Draw)
        };
        self.finish(Outcome {
            winner,
            reason,
            allied_score: allied,
            axis_score: axis,
        });
    }

    fn finish(&mut self, outcome: Outcome) {
        self.winner = outcome.winner;
        self.game_over = true;
        self.phase = Phase::GameOver;
        if let Some(w) = outcome.winner {
            self.log.push(Event::Victory {
                turn: self.turn,
                side: w,
                reason: outcome.reason,
            });
        }
        self.log.push(Event::GameOver {
            outcome: outcome.clone(),
        });
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }
}
