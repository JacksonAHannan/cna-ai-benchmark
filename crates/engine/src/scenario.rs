//! Scenarios: bounded map + roster + victory horizon.

use crate::map::{Map, Terrain};
use crate::side::{Nationality, Side};
use crate::unit::UnitType;
use serde::{Deserialize, Serialize};

/// A unit to place on the map at scenario start (or as a reinforcement).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitSpec {
    pub name: String,
    pub side: Side,
    pub nationality: Nationality,
    pub kind: UnitType,
    pub strength: u8,
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
        row: i16,
        col: i16,
    ) -> Self {
        UnitSpec {
            name: name.to_string(),
            side,
            nationality,
            kind,
            strength,
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
        UnitSpec::new("7th Armoured Division", allied, br, UnitType::Armor, 6, 3, 23),
        UnitSpec::new("4th Indian Division", allied, cw, UnitType::Infantry, 6, 3, 21),
        UnitSpec::new("6th Australian Division", allied, cw, UnitType::Infantry, 5, 3, 21),
        UnitSpec::new("Western Desert Force Arty", allied, br, UnitType::Artillery, 3, 3, 21),
        UnitSpec::new("RAF No.202 Group", allied, br, UnitType::Air, 4, 3, 25),
        UnitSpec::new("2nd NZ Division", allied, cw, UnitType::Infantry, 5, 3, 22),
        UnitSpec::new("9th Australian Division", allied, cw, UnitType::Infantry, 5, 4, 22),
        // Axis: Panzergruppe Afrika + Italian Tenth Army (west, Tripoli / Benghazi).
        UnitSpec::new("15th Panzer Division", axis, ger, UnitType::Armor, 5, 3, 4),
        UnitSpec::new("21st Panzer Division", axis, ger, UnitType::Armor, 4, 3, 4),
        UnitSpec::new("90th Light Division", axis, ger, UnitType::Motorized, 4, 3, 5),
        UnitSpec::new("Ariete Armoured", axis, ita, UnitType::Armor, 3, 3, 9),
        UnitSpec::new("Trento Motorized", axis, ita, UnitType::Motorized, 4, 3, 9),
        UnitSpec::new("10th Army Infantry", axis, ita, UnitType::Infantry, 5, 3, 10),
        UnitSpec::new("Brescia Division", axis, ita, UnitType::Infantry, 4, 4, 9),
        UnitSpec::new("Pavia Division", axis, ita, UnitType::Infantry, 3, 4, 10),
        UnitSpec::new("Regia Aeronautica", axis, ita, UnitType::Air, 3, 3, 3),
    ];

    let reinforcements = vec![
        (3, UnitSpec::new("5th Light Division", axis, ger, UnitType::Motorized, 4, 2, 3)),
        (4, UnitSpec::new("2nd Armoured Division", allied, br, UnitType::Armor, 4, 2, 25)),
        (5, UnitSpec::new("22nd Armoured Brigade", allied, br, UnitType::Armor, 3, 2, 25)),
        (6, UnitSpec::new("Ramcke Brigade", axis, ger, UnitType::Infantry, 3, 2, 3)),
    ];

    Scenario {
        id: "compass".to_string(),
        name: "Operation Compass (reduced)".to_string(),
        turn_horizon: 12,
        map,
        units,
        reinforcements,
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
                    hex,
                )
            })
            .collect()
    }
}
