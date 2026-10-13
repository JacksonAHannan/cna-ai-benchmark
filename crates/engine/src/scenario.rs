//! Scenarios: bounded map + roster + victory horizon.

use crate::map::{Hex, Map, Terrain};
use crate::side::{Nationality, Side};
use crate::unit::{Organization, UnitType};
use serde::{Deserialize, Serialize};

/// A unit to place on the map at scenario start (or as a reinforcement).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitSpec {
    pub name: String,
    pub side: Side,
    pub nationality: Nationality,
    pub kind: UnitType,
    pub strength: u8,
    pub organization: Organization,
    pub morale: i8,
    pub row: i16,
    pub col: i16,
}

impl UnitSpec {
    fn new(
        name: &str,
        side: Side,
        nationality: Nationality,
        kind: UnitType,
        strength: u8,
        organization: Organization,
        morale: i8,
        row: i16,
        col: i16,
    ) -> Self {
        UnitSpec {
            name: name.to_string(),
            side,
            nationality,
            kind,
            strength,
            organization,
            morale,
            row,
            col,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub turn_horizon: u32,
    /// Calendar start (year, month 1–12) of Game-Turn 1.
    pub start_year: u32,
    pub start_month: u32,
    pub map: Map,
    pub units: Vec<UnitSpec>,
    /// (arrival turn, unit spec). Arrival turn is 1-based, evaluated at turn start.
    pub reinforcements: Vec<(u32, UnitSpec)>,
}

/// The Phase-1 scenario: a coastal-desert slice of the Via Balbia theatre,
/// faithful in structure to CNA's Operation-Compass era at a reduced counter count.
pub fn default_scenario() -> Scenario {
    let mut map = Map::new_grid(14, 28);

    // Land south of the Mediterranean (row 0 stays Sea).
    map.fill_terrain(1..14, 0..28, Terrain::Desert);
    // Coastline.
    map.fill_terrain(1..2, 2..26, Terrain::Coast);
    // Via Balbia — the coastal road, the theatre's supply artery.
    map.fill_terrain(2..3, 2..26, Terrain::Road);
    // Escarpment line (Libyan plateau) — channels movement toward the coast road.
    map.fill_terrain(7..10, 4..24, Terrain::Escarpment);
    // Inland desert track (secondary route).
    map.fill_terrain(5..6, 8..22, Terrain::Track);
    // Rough and mountain patches (the Jebel).
    map.set_terrain(6, 12, Terrain::Rough);
    map.set_terrain(6, 16, Terrain::Rough);
    map.set_terrain(6, 14, Terrain::Mountain);
    map.set_terrain(6, 18, Terrain::Mountain);
    // Oases and villages.
    map.set_terrain(5, 5, Terrain::Oasis);
    map.set_terrain(5, 21, Terrain::Oasis);
    map.set_terrain(10, 24, Terrain::Oasis);
    map.set_terrain(3, 16, Terrain::Village);
    map.set_terrain(6, 10, Terrain::Village);
    map.set_terrain(9, 18, Terrain::Village);

    // Supply depots / ports. Capacities reflect published port supply tonnage
    // (docs/cna-rules.md §55): Tripoli & Alexandria are the major ports; Benghazi
    // (~1/6 of Tripoli), Tobruk and Mersa Matruh are minor.
    map.add_depot(2, 3, Side::Axis, "Tripoli", 30);
    map.add_depot(2, 9, Side::Axis, "Benghazi", 6);
    map.add_depot(2, 14, Side::Axis, "Tobruk", 4);
    map.add_depot(2, 21, Side::Allied, "Mersa Matruh", 8);
    map.add_depot(2, 25, Side::Allied, "Alexandria", 30);

    // Victory objectives (the contested coastal fortresses).
    map.add_objective(4, 14, "Tobruk");
    map.add_objective(4, 16, "Bardia");
    map.add_objective(5, 18, "Halfaya Pass");

    let allied = Side::Allied;
    let axis = Side::Axis;
    let br = Nationality::British;
    let cw = Nationality::Commonwealth;
    let ger = Nationality::German;
    let ita = Nationality::Italian;

    let units = vec![
        // Commonwealth Eighth Army (east, around Mersa Matruh / Alexandria).
        // Morale is a placeholder (−3..+3) pending full OB transcription.
        UnitSpec::new("7th Armoured Division", allied, br, UnitType::Armor, 6, Organization::Division, 1, 3, 23),
        UnitSpec::new("4th Indian Division", allied, cw, UnitType::Infantry, 6, Organization::Division, 1, 3, 21),
        UnitSpec::new("6th Australian Division", allied, cw, UnitType::Infantry, 5, Organization::Division, 2, 3, 21),
        UnitSpec::new("Western Desert Force Arty", allied, br, UnitType::Artillery, 3, Organization::Brigade, 0, 3, 21),
        UnitSpec::new("RAF No.202 Group", allied, br, UnitType::Air, 12, Organization::Company, 0, 3, 25),
        UnitSpec::new("2nd NZ Division", allied, cw, UnitType::Infantry, 5, Organization::Division, 1, 3, 22),
        UnitSpec::new("9th Australian Division", allied, cw, UnitType::Infantry, 5, Organization::Division, 1, 4, 22),
        // Axis: Panzergruppe Afrika + Italian Tenth Army (west, Tripoli / Benghazi).
        UnitSpec::new("15th Panzer Division", axis, ger, UnitType::Armor, 5, Organization::Division, 2, 3, 4),
        UnitSpec::new("21st Panzer Division", axis, ger, UnitType::Armor, 4, Organization::Division, 2, 3, 4),
        UnitSpec::new("90th Light Division", axis, ger, UnitType::Motorized, 4, Organization::Division, 1, 3, 5),
        UnitSpec::new("Ariete Armoured", axis, ita, UnitType::Armor, 3, Organization::Division, 0, 3, 9),
        UnitSpec::new("Trento Motorized", axis, ita, UnitType::Motorized, 4, Organization::Division, 0, 3, 9),
        UnitSpec::new("10th Army Infantry", axis, ita, UnitType::Infantry, 5, Organization::Division, -1, 3, 10),
        UnitSpec::new("Brescia Division", axis, ita, UnitType::Infantry, 4, Organization::Division, -1, 4, 9),
        UnitSpec::new("Pavia Division", axis, ita, UnitType::Infantry, 3, Organization::Division, -1, 4, 10),
        UnitSpec::new("Regia Aeronautica", axis, ita, UnitType::Air, 9, Organization::Company, 0, 3, 3),
    ];

    let reinforcements = vec![
        (3, UnitSpec::new("5th Light Division", axis, ger, UnitType::Motorized, 4, Organization::Division, 1, 2, 3)),
        (4, UnitSpec::new("2nd Armoured Division", allied, br, UnitType::Armor, 4, Organization::Division, 1, 2, 25)),
        (5, UnitSpec::new("22nd Armoured Brigade", allied, br, UnitType::Armor, 3, Organization::Brigade, 1, 2, 25)),
        (6, UnitSpec::new("Ramcke Brigade", axis, ger, UnitType::Infantry, 3, Organization::Brigade, 1, 2, 3)),
    ];

    Scenario {
        id: "compass".to_string(),
        name: "Operation Compass (reduced)".to_string(),
        turn_horizon: 12,
        start_year: 1940,
        start_month: 12,
        map,
        units,
        reinforcements,
    }
}

/// Parse the transcribed map grids and resolve `?` cells to a placeholder:
/// Sea when adjacent to sea or the footprint edge, otherwise the dominant
/// neighboring land terrain. Returns (resolved cells, min_row, min_col).
fn full_map_cells() -> (Vec<(Hex, Terrain)>, i16, i16) {
    let text = include_str!("../../../docs/data/map-terrain.md");
    let sheets: [(char, i16); 5] = [('A', 0), ('B', 33), ('C', 66), ('D', 99), ('E', 132)];
    let mut raw: Vec<(Hex, Option<Terrain>)> = Vec::new();
    for (name, t) in sheets {
        let marker = format!("## Sheet {name}");
        let Some(sec) = text.split(&marker).nth(1) else {
            continue;
        };
        let grid = sec.split("\n## ").next().unwrap_or(sec);
        for line in grid.lines() {
            let mut parts = line.split_whitespace();
            let Ok(rr) = parts.next().unwrap_or("").parse::<i16>() else {
                continue;
            };
            let Some(row) = parts.next() else { continue };
            for (i, ch) in row.chars().enumerate() {
                let cc = i as i16 + 1;
                let q = cc;
                let r = rr + t - (cc + 1) / 2;
                raw.push((Hex::new(q, r), Terrain::from_transcription_code(ch)));
            }
        }
    }
    let by_hex: std::collections::HashMap<Hex, Terrain> = raw
        .iter()
        .filter_map(|(h, t)| t.map(|tt| (*h, tt)))
        .collect();

    let cells: Vec<(Hex, Terrain)> = raw
        .iter()
        .filter_map(|(h, t)| match t {
            Some(tt) => Some((*h, *tt)),
            None => {
                let nbrs: Vec<Terrain> = h
                    .neighbors()
                    .iter()
                    .filter_map(|n| by_hex.get(n).copied())
                    .collect();
                if nbrs.is_empty() {
                    // Isolated `?` outside the footprint — not a playable hex.
                    None
                } else if nbrs.iter().any(|t| *t == Terrain::Sea) {
                    Some((*h, Terrain::Sea))
                } else {
                    let mut counts = std::collections::HashMap::new();
                    for t in &nbrs {
                        *counts.entry(*t).or_insert(0u32) += 1;
                    }
                    let d = counts
                        .into_iter()
                        .max_by_key(|(_, n)| *n)
                        .map(|(t, _)| t)
                        .unwrap_or(Terrain::Clear);
                    Some((*h, d))
                }
            }
        })
        .collect();

    let min_row = cells.iter().map(|(h, _)| h.r).min().unwrap_or(0);
    let min_col = cells.iter().map(|(h, _)| h.to_offset().1).min().unwrap_or(0);
    (cells, min_row, min_col)
}

/// Parse the transcribed CNA map (docs/data/map-terrain.md) into a full `Map`,
/// resolving ambiguous `?` cells to a neighbor-based placeholder.
pub fn full_map() -> Map {
    let (cells, _, _) = full_map_cells();
    Map::from_axial_hexes(cells)
}

/// The Italian Campaign / Operation Compass setup on the full transcribed map
/// (§60.23, GT1/OS1). Ports, objectives, and formations are placed at their
/// printed `ccrr` coordinates (docs/data/compass-setup.md, map-features.md).
/// Unit strengths/morale are placeholders pending full OB wiring.
pub fn compass_scenario() -> Scenario {
    let (cells, min_row, min_col) = full_map_cells();
    let mut map = Map::from_axial_hexes(cells);

    // Printed ccrr -> normalized offset (row, col).
    let ccrr = |sheet: char, cc: i16, rr: i16| -> (i16, i16) {
        let t = match sheet {
            'A' => 0,
            'B' => 33,
            'C' => 66,
            'D' => 99,
            'E' => 132,
            _ => 0,
        };
        let q = cc;
        let r = rr + t - (cc + 1) / 2;
        let (row, col) = Hex::new(q, r).to_offset();
        (row - min_row, col - min_col)
    };

    let allied = Side::Allied;
    let axis = Side::Axis;
    let br = Nationality::British;
    let cw = Nationality::Commonwealth;
    let ita = Nationality::Italian;

    // Ports (depots); capacity = port tonnage ÷ 500 (reduced scale).
    let mut place = |sheet: char, cc: i16, rr: i16, side: Side, name: &str, cap: u32| {
        let (r, c) = ccrr(sheet, cc, rr);
        map.add_depot(r, c, side, name, cap);
    };
    place('A', 48, 27, axis, "Benghazi", 5);
    place('B', 59, 25, axis, "Derna", 1);
    place('C', 48, 07, axis, "Tobruk", 3);
    place('C', 43, 21, axis, "Bardia", 1);
    place('C', 40, 21, axis, "Sollum", 1);
    place('D', 37, 14, allied, "Mersa Matruh", 1);
    place('E', 36, 13, allied, "Alexandria", 30);

    // Victory objectives.
    for (sheet, cc, rr, name) in [
        ('C', 48, 07, "Tobruk"),
        ('C', 43, 21, "Bardia"),
        ('C', 39, 22, "Halfaya Pass"),
        ('C', 41, 31, "Sidi Barrani"),
        ('C', 36, 18, "Sidi Omar"),
        ('C', 30, 19, "Fort Maddalena"),
        ('C', 01, 27, "Siwa"),
        ('A', 20, 21, "Mersa Brega"),
        ('A', 18, 16, "El Agheila"),
    ] {
        let (r, c) = ccrr(sheet, cc, rr);
        map.add_objective(r, c, name);
    }

    // Starting formations (docs/data/compass-setup.md).
    let div = Organization::Division;
    let bde = Organization::Brigade;
    let units = vec![
        // Commonwealth (Western Desert Force).
        UnitSpec::new("7th Armoured Division", allied, br, UnitType::Armor, 8, div, 1, 0, 0),
        UnitSpec::new("4th Indian Division", allied, cw, UnitType::Infantry, 8, div, 1, 0, 0),
        UnitSpec::new("6th Australian Division", allied, cw, UnitType::Infantry, 6, div, 2, 0, 0),
        UnitSpec::new("2nd NZ Division", allied, cw, UnitType::Infantry, 6, div, 1, 0, 0),
        UnitSpec::new("WDF 7th Medium Arty", allied, br, UnitType::Artillery, 3, bde, 0, 0, 0),
        // Italian Tenth Army (west).
        UnitSpec::new("Maletti Division", axis, ita, UnitType::Motorized, 5, div, 0, 0, 0),
        UnitSpec::new("1st Libyan Division", axis, ita, UnitType::Infantry, 5, div, -1, 0, 0),
        UnitSpec::new("2nd Libyan Division", axis, ita, UnitType::Infantry, 5, div, -1, 0, 0),
        UnitSpec::new("1st CCNN Division", axis, ita, UnitType::Infantry, 4, div, 0, 0, 0),
        UnitSpec::new("63rd Cirene Division", axis, ita, UnitType::Infantry, 4, div, -1, 0, 0),
        UnitSpec::new("62nd Marmarica Division", axis, ita, UnitType::Infantry, 4, div, -1, 0, 0),
        UnitSpec::new("64th Catanzaro Division", axis, ita, UnitType::Infantry, 4, div, -1, 0, 0),
        UnitSpec::new("4th CCNN Division", axis, ita, UnitType::Infantry, 4, div, 0, 0, 0),
        // Garrisons.
        UnitSpec::new("Benghazi Garrison", axis, ita, UnitType::Infantry, 3, bde, -1, 0, 0),
        UnitSpec::new("Tobruk Garrison", axis, ita, UnitType::Infantry, 3, bde, 0, 0, 0),
        UnitSpec::new("Bardia Garrison", axis, ita, UnitType::Infantry, 3, bde, 0, 0, 0),
        UnitSpec::new("Derna Garrison", axis, ita, UnitType::Infantry, 2, bde, -1, 0, 0),
        UnitSpec::new("Mersa Matruh Garrison", allied, cw, UnitType::Infantry, 3, bde, 0, 0, 0),
    ];

    // Place each formation at its printed coordinate.
    let coords: [(&str, char, i16, i16); 18] = [
        ("7th Armoured Division", 'D', 36, 12),
        ("4th Indian Division", 'D', 36, 15),
        ("6th Australian Division", 'E', 14, 30),
        ("2nd NZ Division", 'E', 36, 13),
        ("WDF 7th Medium Arty", 'C', 39, 22),
        ("Maletti Division", 'C', 36, 17),
        ("1st Libyan Division", 'C', 40, 20),
        ("2nd Libyan Division", 'C', 39, 20),
        ("1st CCNN Division", 'C', 42, 18),
        ("63rd Cirene Division", 'C', 41, 20),
        ("62nd Marmarica Division", 'C', 39, 18),
        ("64th Catanzaro Division", 'C', 47, 07),
        ("4th CCNN Division", 'C', 45, 07),
        ("Benghazi Garrison", 'A', 48, 27),
        ("Tobruk Garrison", 'C', 48, 07),
        ("Bardia Garrison", 'C', 43, 21),
        ("Derna Garrison", 'B', 59, 25),
        ("Mersa Matruh Garrison", 'D', 37, 14),
    ];
    let mut units: Vec<UnitSpec> = units;
    for (name, sheet, cc, rr) in coords {
        let (r, c) = ccrr(sheet, cc, rr);
        if let Some(spec) = units.iter_mut().find(|u| u.name == name) {
            spec.row = r;
            spec.col = c;
        }
    }

    Scenario {
        id: "compass-full".to_string(),
        name: "Italian Campaign (full map)".to_string(),
        turn_horizon: 20,
        start_year: 1940,
        start_month: 9,
        map,
        units,
        reinforcements: vec![],
    }
}

impl Scenario {
    /// Build the initial unit roster (ids 1..=N) without running any phases —
    /// used by the replay spectator to reconstruct unit positions from the log.
    pub fn initial_units(&self) -> Vec<crate::unit::Unit> {
        self.units
            .iter()
            .enumerate()
            .map(|(i, spec)| {
                let hex = crate::map::Hex::from_offset(spec.row, spec.col);
                crate::unit::Unit::new(
                    (i + 1) as crate::unit::UnitId,
                    &spec.name,
                    spec.side,
                    spec.nationality,
                    spec.kind,
                    spec.strength,
                    spec.organization,
                    spec.morale,
                    hex,
                )
            })
            .collect()
    }
}
