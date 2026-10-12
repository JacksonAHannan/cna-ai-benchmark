//! Hex map: axial coordinates, terrain, depots, and objectives.
//!
//! `Map.hexes` is a `Vec` (each cell carries its own `Hex`) rather than a
//! `HashMap<Hex, _>` so the map serializes to clean JSON — `serde_json` cannot
//! use a struct as a map key. The grid is small (a few hundred cells), so lookup
//! is a linear scan.

use crate::side::Side;
use serde::{Deserialize, Serialize};

/// Axial hex coordinate. `q` is the column axis, `r` the row axis.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Hex {
    pub q: i16,
    pub r: i16,
}

impl Hex {
    pub fn new(q: i16, r: i16) -> Self {
        Hex { q, r }
    }

    /// Convert from "odd-r" offset coordinates (authoring-friendly rectangular grid).
    pub fn from_offset(row: i16, col: i16) -> Self {
        let q = col - (row - (row & 1)) / 2;
        Hex { q, r: row }
    }

    pub fn to_offset(self) -> (i16, i16) {
        let row = self.r;
        let col = self.q + (row - (row & 1)) / 2;
        (row, col)
    }

    pub fn neighbors(self) -> [Hex; 6] {
        const DIRS: [(i16, i16); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        DIRS.map(|(dq, dr)| Hex::new(self.q + dq, self.r + dr))
    }

    /// Hex (cube) distance between two hexes.
    pub fn distance(self, other: Hex) -> i32 {
        let ax = self.q as i32;
        let az = self.r as i32;
        let ay = -ax - az;
        let bx = other.q as i32;
        let bz = other.r as i32;
        let by = -bx - bz;
        ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) / 2
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Terrain {
    Sea,
    Desert,
    Road,
    Coast,
    Escarpment,
    Oasis,
    Village,
}

impl Terrain {
    pub fn move_cost(self) -> u8 {
        match self {
            Terrain::Sea => u8::MAX,
            Terrain::Road | Terrain::Coast | Terrain::Village => 1,
            Terrain::Desert | Terrain::Oasis => 2,
            Terrain::Escarpment => 3,
        }
    }

    /// CRT defensive shift applied to a defender in this terrain.
    pub fn defense(self) -> i8 {
        match self {
            Terrain::Escarpment => 1,
            _ => 0,
        }
    }

    pub fn passable(self) -> bool {
        !matches!(self, Terrain::Sea)
    }

    /// Road or coast — the Via Balbia supply artery.
    pub fn is_road_network(self) -> bool {
        matches!(self, Terrain::Road | Terrain::Coast)
    }
}

/// A named feature on a hex. Depots are supply sources; objectives are victory points.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Site {
    Depot { side: Side, name: String, capacity: u32 },
    Objective { name: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapHex {
    pub hex: Hex,
    pub terrain: Terrain,
    /// Cosmetic name (e.g. "Tobruk") for the report and UI.
    pub name: Option<String>,
    pub site: Option<Site>,
}

impl MapHex {
    fn blank(hex: Hex) -> Self {
        MapHex {
            hex,
            terrain: Terrain::Sea,
            name: None,
            site: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub rows: i16,
    pub cols: i16,
    pub hexes: Vec<MapHex>,
    pub depots: Vec<Hex>,
    pub objectives: Vec<Hex>,
}

impl Map {
    pub fn get(&self, hex: Hex) -> Option<&MapHex> {
        self.hexes.iter().find(|h| h.hex == hex)
    }

    pub fn get_mut(&mut self, hex: Hex) -> Option<&mut MapHex> {
        self.hexes.iter_mut().find(|h| h.hex == hex)
    }

    pub fn in_bounds(&self, hex: Hex) -> bool {
        self.get(hex).is_some()
    }

    /// Build an all-sea grid of `rows` x `cols` (odd-r offset).
    pub fn new_grid(rows: i16, cols: i16) -> Self {
        let mut hexes = Vec::with_capacity((rows * cols) as usize);
        for row in 0..rows {
            for col in 0..cols {
                hexes.push(MapHex::blank(Hex::from_offset(row, col)));
            }
        }
        Map {
            rows,
            cols,
            hexes,
            depots: Vec::new(),
            objectives: Vec::new(),
        }
    }

    /// Set the terrain of every hex in an offset row/col rectangle.
    pub fn fill_terrain(&mut self, rows: std::ops::Range<i16>, cols: std::ops::Range<i16>, t: Terrain) {
        for row in rows {
            for col in cols.clone() {
                let hex = Hex::from_offset(row, col);
                if let Some(h) = self.get_mut(hex) {
                    h.terrain = t;
                }
            }
        }
    }

    pub fn set_terrain(&mut self, row: i16, col: i16, t: Terrain) {
        let hex = Hex::from_offset(row, col);
        if let Some(h) = self.get_mut(hex) {
            h.terrain = t;
        }
    }

    pub fn add_depot(&mut self, row: i16, col: i16, side: Side, name: &str, capacity: u32) -> Hex {
        let hex = Hex::from_offset(row, col);
        if let Some(h) = self.get_mut(hex) {
            h.site = Some(Site::Depot {
                side,
                name: name.to_string(),
                capacity,
            });
            h.name = Some(name.to_string());
        }
        self.depots.push(hex);
        hex
    }

    pub fn add_objective(&mut self, row: i16, col: i16, name: &str) -> Hex {
        let hex = Hex::from_offset(row, col);
        if let Some(h) = self.get_mut(hex) {
            h.site = Some(Site::Objective {
                name: name.to_string(),
            });
            h.name = Some(name.to_string());
        }
        self.objectives.push(hex);
        hex
    }

    pub fn depots_of(&self, side: Side) -> Vec<Hex> {
        self.depots
            .iter()
            .copied()
            .filter(|&h| {
                matches!(
                    self.get(h).and_then(|m| m.site.as_ref()),
                    Some(Site::Depot { side: s, .. }) if *s == side
                )
            })
            .collect()
    }
}
