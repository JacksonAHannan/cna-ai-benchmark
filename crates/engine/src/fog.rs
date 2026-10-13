//! Fog-of-war observations: what a side may see.

use crate::events::Event;
use crate::map::{Hex, Map, Site};
use crate::side::Side;
use crate::state::GameState;
use crate::unit::{Unit, UnitId, UnitType};
use serde::{Deserialize, Serialize};

/// CNA's five staff roles — the observation partition of a side (one agent per
/// side plays all five; see ADR 0005).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Role {
    CinC,
    Logistics,
    RearArea,
    Air,
    FrontLine,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleView {
    pub role: Role,
    pub note: String,
    pub units: Vec<UnitId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepotView {
    pub hex: Hex,
    pub name: String,
    pub side: Side,
    pub capacity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub scenario: String,
    pub turn: u32,
    pub horizon: u32,
    pub active: Side,
    pub viewer: Side,
    pub game_over: bool,
    pub winner: Option<Side>,
    /// Terrain and sites are public (fog-of-war hides units, not geography).
    pub map: Map,
    /// Visible units: the side's own units in full, enemy units only if detected
    /// (within detection range or in a reconnoitered hex).
    pub units: Vec<Unit>,
    pub depots: Vec<DepotView>,
    pub objectives: Vec<Hex>,
    /// Last few events for situational context.
    pub recent_events: Vec<Event>,
    /// The five-role staff partition: which units each role is responsible for.
    pub role_views: Vec<RoleView>,
}

impl GameState {
    pub fn observation(&self, side: Side) -> Observation {
        let mut units = Vec::new();
        for u in self.units.values() {
            if u.eliminated() {
                continue;
            }
            let visible = u.side == side
                || u
                    .hex
                    .map(|h| self.is_visible_to(side, h))
                    .unwrap_or(false);
            if visible {
                units.push(u.clone());
            }
        }
        // Own units first, then detected enemy units; stable within each group.
        units.sort_by_key(|u| (!(u.side == side), u.id));

        let mut depots = Vec::new();
        for &h in &self.map.depots {
            if let Some(m) = self.map.get(h) {
                if let Some(Site::Depot {
                    side: s,
                    name,
                    capacity,
                }) = &m.site
                {
                    depots.push(DepotView {
                        hex: h,
                        name: name.clone(),
                        side: *s,
                        capacity: *capacity,
                    });
                }
            }
        }

        let recent_events = self
            .log
            .events
            .iter()
            .rev()
            .take(12)
            .cloned()
            .rev()
            .collect();

        let mut role_views = Vec::new();
        for role in [Role::CinC, Role::Logistics, Role::RearArea, Role::Air, Role::FrontLine] {
            let ids: Vec<UnitId> = units
                .iter()
                .filter(|u| match role {
                    Role::Air => u.kind == UnitType::Air,
                    Role::FrontLine => u.kind != UnitType::Air,
                    Role::Logistics => u.side == side,
                    Role::CinC => true,
                    Role::RearArea => false,
                })
                .map(|u| u.id)
                .collect();
            let note = match role {
                Role::CinC => "overall command".to_string(),
                Role::Logistics => "supply, fuel, water".to_string(),
                Role::RearArea => "reinforcements & rear".to_string(),
                Role::Air => "air squadrons".to_string(),
                Role::FrontLine => "ground forces".to_string(),
            };
            role_views.push(RoleView {
                role,
                note,
                units: ids,
            });
        }

        Observation {
            scenario: self.scenario_name.clone(),
            turn: self.turn,
            horizon: self.turn_horizon,
            active: self.active,
            viewer: side,
            game_over: self.game_over,
            winner: self.winner,
            map: self.map.clone(),
            units,
            depots,
            objectives: self.map.objectives.clone(),
            recent_events,
            role_views,
        }
    }
}
