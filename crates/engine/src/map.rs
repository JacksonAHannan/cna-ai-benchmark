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
    Clear,
    Desert,
    Rough,
    Mountain,
    SaltMarsh,
    Escarpment,
    Road,
    Track,
    Coast,
    Oasis,
    Village,
    MajorCity,
}

impl Terrain {
    /// Movement cost in CP (published §8.37 chart; motorized costs more off-road).
    pub fn move_cost(self, motorized: bool) -> u8 {
        match self {
            Terrain::Sea => u8::MAX,
            Terrain::Clear | Terrain::Village => 2,
            Terrain::Desert | Terrain::Oasis | Terrain::Rough => {
                if motorized {
                    4
                } else {
                    3
                }
            }
            Terrain::Mountain => {
                if motorized {
                    6
                } else {
                    4
                }
            }
            Terrain::SaltMarsh => {
                if motorized {
                    2
                } else {
                    3
                }
            }
            // Up-escarpment: +6 non-motorized, prohibited for motorized.
            Terrain::Escarpment => {
                if motorized {
                    u8::MAX
                } else {
                    6
                }
            }
            Terrain::Road | Terrain::Coast | Terrain::Track | Terrain::MajorCity => 1,
        }
    }

    /// Close-assault column shift favoring the defender (positive = L columns).
    pub fn assault_defense(self) -> i8 {
        match self {
            Terrain::Rough => 2,
            Terrain::Mountain => 3,
            Terrain::Escarpment => 3,
            Terrain::SaltMarsh => -1, // R1 favors the attacker
            _ => 0,
        }
    }

    pub fn passable(self) -> bool {
        !matches!(self, Terrain::Sea)
    }

    /// Unlimited water source (§52): major cities and oases provide water.
    pub fn is_water_source(self) -> bool {
        matches!(self, Terrain::Oasis | Terrain::MajorCity)
    }

    /// Road or coast — the Via Balbia supply artery.
    pub fn is_road_network(self) -> bool {
        matches!(self, Terrain::Road | Terrain::Coast)
    }

    /// One-char code for the compact serialized terrain grid.
    pub fn code(self) -> char {
        match self {
            Terrain::Sea => 'S',
            Terrain::Clear => '.',
            Terrain::Desert => 'D',
            Terrain::Rough => 'G',
            Terrain::Mountain => 'M',
            Terrain::SaltMarsh => 'L',
            Terrain::Escarpment => 'E',
            Terrain::Road => 'R',
            Terrain::Track => 'T',
            Terrain::Coast => 'C',
            Terrain::Oasis => 'O',
            Terrain::Village => 'V',
            Terrain::MajorCity => 'K',
        }
    }

    pub fn from_code(c: char) -> Terrain {
        match c {
            'S' => Terrain::Sea,
            '.' => Terrain::Clear,
            'D' => Terrain::Desert,
            'G' => Terrain::Rough,
            'M' => Terrain::Mountain,
            'L' => Terrain::SaltMarsh,
            'E' => Terrain::Escarpment,
            'R' => Terrain::Road,
            'T' => Terrain::Track,
            'C' => Terrain::Coast,
            'O' => Terrain::Oasis,
            'V' => Terrain::Village,
            'K' => Terrain::MajorCity,
            _ => Terrain::Desert,
        }
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
    /// O(1) lookup: the `Vec` is stored in row-major offset order.
    pub fn get(&self, hex: Hex) -> Option<&MapHex> {
        let (row, col) = hex.to_offset();
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            return None;
        }
        Some(&self.hexes[(row * self.cols + col) as usize])
    }

    pub fn get_mut(&mut self, hex: Hex) -> Option<&mut MapHex> {
        let (row, col) = hex.to_offset();
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            return None;
        }
        Some(&mut self.hexes[(row * self.cols + col) as usize])
    }

    pub fn in_bounds(&self, hex: Hex) -> bool {
        let (row, col) = hex.to_offset();
        row >= 0 && row < self.rows && col >= 0 && col < self.cols
    }

    /// Compact serialized terrain: one string per offset row, one char per hex.
    pub fn terrain_grid(&self) -> Vec<String> {
        let mut rows = Vec::with_capacity(self.rows as usize);
        for row in 0..self.rows {
            let mut s = String::with_capacity(self.cols as usize);
            for col in 0..self.cols {
                let hex = Hex::from_offset(row, col);
                let t = self.get(hex).map(|m| m.terrain).unwrap_or(Terrain::Sea);
                s.push(t.code());
            }
            rows.push(s);
        }
        rows
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
