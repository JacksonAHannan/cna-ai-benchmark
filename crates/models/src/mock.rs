//! A deterministic, heuristic agent used for offline smoke tests and as a
//! baseline competitor. It requires no network and no API key.
//!
//! It keeps light per-turn memory of *failed* moves (e.g. moving into a hex a
//! hidden enemy occupies) so it explores alternatives instead of deterministically
//! repeating an illegal action — which keeps the legality telemetry meaningful.

use crate::adapter::{Agent, ModelError};
use async_trait::async_trait;
use cna_engine::{Action, ActionError, Hex, Observation, UnitId, UnitType};
use parking_lot::Mutex;
use std::collections::HashSet;

#[derive(Default)]
struct MockState {
    turn: u32,
    failed: HashSet<(UnitId, Hex)>,
    last_move: Option<(UnitId, Hex)>,
}

impl MockState {
    fn reset(&mut self, turn: u32) {
        self.turn = turn;
        self.failed.clear();
        self.last_move = None;
    }
}

pub struct MockAgent {
    name: String,
    state: Mutex<MockState>,
}

impl MockAgent {
    pub fn new(name: String) -> Self {
        MockAgent {
            name,
            state: Mutex::new(MockState::default()),
        }
    }
}

#[async_trait]
impl Agent for MockAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn act(
        &self,
        obs: &Observation,
        feedback: Option<&ActionError>,
    ) -> Result<Action, ModelError> {
        let mut st = self.state.lock();
        if st.turn != obs.turn {
            st.reset(obs.turn);
        }
        if feedback.is_some() {
            if let Some((uid, h)) = st.last_move.take() {
                st.failed.insert((uid, h));
            }
        }
        let action = decide(obs, &st.failed);
        if let Action::Move { unit, path } = &action {
            st.last_move = Some((*unit, *path.last().unwrap()));
        } else {
            st.last_move = None;
        }
        Ok(action)
    }

    async fn chat(&self, _system: &str, _user: &str) -> Result<String, ModelError> {
        Ok("(no narrative — mock agent has no language model)".to_string())
    }
}

fn decide(obs: &Observation, failed: &HashSet<(UnitId, Hex)>) -> Action {
    let viewer = obs.viewer;
    let own: Vec<&cna_engine::Unit> = obs.units.iter().filter(|u| u.side == viewer).collect();
    let enemy: Vec<&cna_engine::Unit> = obs.units.iter().filter(|u| u.side != viewer).collect();

    if obs.game_over || own.is_empty() {
        return Action::EndTurn;
    }

    // 1. Attack any adjacent (or artillery-range) visible enemy.
    for u in &own {
        if u.kind == UnitType::Air {
            continue; // air units sortie via recon/support, not direct attack
        }
        if u.has_attacked || u.movement == 0 {
            continue;
        }
        let range = match u.kind {
            UnitType::Artillery => 2,
            _ => 1,
        };
        if let Some(t) = enemy.iter().find(|t| u.hex.unwrap().distance(t.hex.unwrap()) <= range) {
            let air: Vec<UnitId> = own
                .iter()
                .filter(|a| a.kind == UnitType::Air && a.missions > 0)
                .take(2)
                .map(|a| a.id)
                .collect();
            return Action::Attack {
                attacker: u.id,
                target: t.id,
                air_support: air,
            };
        }
    }

    // 2. Move one hex toward the nearest enemy, else the nearest objective.
    if let Some((uid, next)) = best_move(obs, &own, &enemy, failed) {
        let cur = own.iter().find(|u| u.id == uid).unwrap().hex.unwrap();
        return Action::Move {
            unit: uid,
            path: vec![cur, next],
        };
    }

    // 3. Recon toward the nearest enemy (or objective) if air is available.
    if let Some(a) = own.iter().find(|a| a.kind == UnitType::Air && a.missions > 0) {
        let target = nearest_target(obs, a.hex.unwrap(), &enemy);
        return Action::AirRecon {
            squadron: a.id,
            target,
        };
    }

    Action::EndTurn
}

fn nearest_target(obs: &Observation, cur: Hex, enemy: &[&cna_engine::Unit]) -> Hex {
    if let Some(e) = enemy.iter().map(|e| e.hex.unwrap()).min_by_key(|&h| cur.distance(h)) {
        return e;
    }
    obs.objectives
        .iter()
        .copied()
        .min_by_key(|&h| cur.distance(h))
        .unwrap_or(cur)
}

fn passable(obs: &Observation, h: Hex) -> bool {
    obs.map.get(h).map(|m| m.terrain.passable()).unwrap_or(false)
}

fn occupied(obs: &Observation, h: Hex) -> bool {
    obs.units.iter().any(|u| u.hex == Some(h))
}

fn best_move(
    obs: &Observation,
    own: &[&cna_engine::Unit],
    enemy: &[&cna_engine::Unit],
    failed: &HashSet<(UnitId, Hex)>,
) -> Option<(UnitId, Hex)> {
    let mut sorted: Vec<&&cna_engine::Unit> = own.iter().collect();
    sorted.sort_by_key(|u| u.id);

    for u in sorted {
        if u.movement == 0 {
            continue;
        }
        if u.kind.is_mobile() && u.fuel == 0 {
            continue;
        }
        let cur = u.hex?;
        let target = nearest_target(obs, cur, enemy);

        let mut best: Option<(Hex, u8, i32)> = None;
        for n in cur.neighbors() {
            if failed.contains(&(u.id, n)) {
                continue;
            }
            if !passable(obs, n) || occupied(obs, n) {
                continue;
            }
            let cost = obs.map.get(n).map(|m| m.terrain.move_cost()).unwrap_or(u8::MAX);
            if cost == u8::MAX || u.movement < cost {
                continue;
            }
            let d = n.distance(target);
            if best.map_or(true, |(_, _, bd)| d < bd) {
                best = Some((n, cost, d));
            }
        }
        if let Some((n, _, _)) = best {
            return Some((u.id, n));
        }
    }
    None
}
