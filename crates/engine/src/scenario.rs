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
    let mut map = Map::new_grid(12, 16);

    // Land south of the Mediterranean (row 0 stays Sea).
    map.fill_terrain(1..12, 0..16, Terrain::Desert);
    // Coastline.
    map.fill_terrain(1..2, 2..14, Terrain::Coast);
    // Via Balbia — the coastal road, the theatre's supply artery.
    map.fill_terrain(2..3, 2..14, Terrain::Road);
    // Escarpment line.
    map.fill_terrain(7..9, 4..12, Terrain::Escarpment);
    // Oases and villages.
    map.set_terrain(5, 3, Terrain::Oasis);
    map.set_terrain(9, 13, Terrain::Oasis);
    map.set_terrain(3, 10, Terrain::Village);
    map.set_terrain(6, 6, Terrain::Village);
    map.set_terrain(5, 12, Terrain::Village);

    // Supply depots / ports.
    map.add_depot(2, 3, Side::Axis, "Tripoli", 30);
    map.add_depot(2, 7, Side::Axis, "Benghazi", 20);
    map.add_depot(2, 10, Side::Allied, "Mersa Matruh", 20);
    map.add_depot(2, 13, Side::Allied, "Alexandria", 30);

    // Victory objectives.
    map.add_objective(4, 9, "Tobruk");
    map.add_objective(4, 11, "Bardia");
    map.add_objective(5, 12, "Halfaya Pass");

    let allied = Side::Allied;
    let axis = Side::Axis;
    let br = Nationality::British;
    let cw = Nationality::Commonwealth;
    let ger = Nationality::German;
    let ita = Nationality::Italian;

    let units = vec![
        // Commonwealth Eighth Army (west-to-east orientation).
        UnitSpec::new("7th Armoured Division", allied, br, UnitType::Armor, 6, 3, 13),
        UnitSpec::new("4th Indian Division", allied, cw, UnitType::Infantry, 6, 3, 11),
        UnitSpec::new("6th Australian Division", allied, cw, UnitType::Infantry, 5, 3, 11),
        UnitSpec::new("Western Desert Force Arty", allied, br, UnitType::Artillery, 3, 3, 11),
        UnitSpec::new("RAF No.202 Group", allied, br, UnitType::Air, 4, 3, 13),
        // Axis: Panzergruppe Afrika + Italian Tenth Army.
        UnitSpec::new("15th Panzer Division", axis, ger, UnitType::Armor, 5, 3, 3),
        UnitSpec::new("21st Panzer Division", axis, ger, UnitType::Armor, 4, 3, 3),
        UnitSpec::new("Ariete Armoured", axis, ita, UnitType::Armor, 3, 3, 7),
        UnitSpec::new("Trento Motorized", axis, ita, UnitType::Motorized, 4, 3, 7),
        UnitSpec::new("10th Army Infantry", axis, ita, UnitType::Infantry, 5, 3, 7),
        UnitSpec::new("Regia Aeronautica", axis, ita, UnitType::Air, 3, 3, 3),
    ];

    let reinforcements = vec![
        (3, UnitSpec::new("5th Light Division", axis, ger, UnitType::Motorized, 4, 2, 3)),
        (4, UnitSpec::new("2nd Armoured Division", allied, br, UnitType::Armor, 4, 2, 13)),
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
