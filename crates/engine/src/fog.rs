//! Fog-of-war observations: what a side may see.

use crate::events::Event;
use crate::map::{Hex, Site, Terrain};
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

impl Role {
    /// Whether this role controls the given unit in 5v5 play (§9 command
    /// partition). CinC commands any unit; Air commands air; FrontLine commands
    /// armor/motorized/infantry; Logistics commands artillery; RearArea none.
    pub fn controls(self, unit: &Unit) -> bool {
        match self {
            Role::CinC => true,
            Role::Air => unit.kind == UnitType::Air,
            Role::FrontLine => matches!(
                unit.kind,
                UnitType::Armor | UnitType::Motorized | UnitType::Infantry
            ),
            Role::Logistics => unit.kind == UnitType::Artillery,
            Role::RearArea => false,
        }
    }
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
    /// Compact terrain grid (offset rows, char codes). Fog-of-war hides units,
    /// not geography; terrain is public and cheap to serialize.
    pub terrain: Vec<String>,
    /// Top-left origin of the terrain window, in offset (row, col) coordinates.
    pub terrain_origin: [i16; 2],
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
    /// Compute a terrain grid window around the side's units plus its top-left
    /// origin in offset (row, col) coordinates, so observations send only the
    /// relevant board rather than the full map.
    fn terrain_window(&self, side: Side, radius: i16) -> (Vec<String>, i16, i16) {
        let mut min_r = i16::MAX;
        let mut max_r = i16::MIN;
        let mut min_c = i16::MAX;
        let mut max_c = i16::MIN;
        let mut any = false;
        for u in self.units.values() {
            if u.side != side || u.eliminated() {
                continue;
            }
            if let Some(h) = u.hex {
                let (r, c) = h.to_offset();
                min_r = min_r.min(r);
                max_r = max_r.max(r);
                min_c = min_c.min(c);
                max_c = max_c.max(c);
                any = true;
            }
        }
        if !any {
            return (self.map.terrain_grid(), 0, 0);
        }
        let r0 = (min_r - radius).max(0);
        let r1 = (max_r + radius).min(self.map.rows - 1);
        let c0 = (min_c - radius).max(0);
        let c1 = (max_c + radius).min(self.map.cols - 1);
        let mut rows = Vec::with_capacity((r1 - r0 + 1) as usize);
        for row in r0..=r1 {
            let mut s = String::with_capacity((c1 - c0 + 1) as usize);
            for col in c0..=c1 {
                let hex = Hex::from_offset(row, col);
                let t = self
                    .map
                    .get(hex)
                    .map(|m| m.terrain)
                    .unwrap_or(Terrain::Sea);
                s.push(t.code());
            }
            rows.push(s);
        }
        (rows, r0, c0)
    }

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

        let mut cinc = Vec::new();
        let mut air = Vec::new();
        let mut front = Vec::new();
        let mut logistics = Vec::new();
        for u in &units {
            cinc.push(u.id);
            if u.kind == UnitType::Air {
                air.push(u.id);
            } else {
                front.push(u.id);
            }
            if u.side == side {
                logistics.push(u.id);
            }
        }
        let role_views = vec![
            RoleView {
                role: Role::CinC,
                note: "overall command".to_string(),
                units: cinc,
            },
            RoleView {
                role: Role::Logistics,
                note: "supply, fuel, water".to_string(),
                units: logistics,
            },
            RoleView {
                role: Role::RearArea,
                note: "reinforcements & rear".to_string(),
                units: vec![],
            },
            RoleView {
                role: Role::Air,
                note: "air squadrons".to_string(),
                units: air,
            },
            RoleView {
                role: Role::FrontLine,
                note: "ground forces".to_string(),
                units: front,
            },
        ];

        let (terrain, tr, tc) = self.terrain_window(side, 20);

        Observation {
            scenario: self.scenario_name.clone(),
            turn: self.turn,
            horizon: self.turn_horizon,
            active: self.active,
            viewer: side,
            game_over: self.game_over,
            winner: self.winner,
            terrain,
            terrain_origin: [tr, tc],
            units,
            depots,
            objectives: self.map.objectives.clone(),
            recent_events,
            role_views,
        }
    }

    /// Role-scoped observation for 5v5 play: the role sees its own controlled
    /// units (own side only) plus all visible enemy units for targeting.
    pub fn role_observation(&self, role: Role, side: Side) -> Observation {
        let mut obs = self.observation(side);
        obs.units.retain(|u| u.side != side || role.controls(u));
        obs
    }
}
