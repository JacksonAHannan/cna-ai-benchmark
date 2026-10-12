//! Units (counters), their types, and supply status.

use crate::map::Hex;
use crate::side::{Nationality, Side};
use serde::{Deserialize, Serialize};

pub type UnitId = u16;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum UnitType {
    Armor,
    Motorized,
    Infantry,
    Artillery,
    Air,
}

impl UnitType {
    pub fn is_mobile(self) -> bool {
        matches!(self, UnitType::Armor | UnitType::Motorized)
    }

    /// Base supply draw per turn (before the Italian pasta surcharge).
    pub fn supply_draw(self) -> u32 {
        match self {
            UnitType::Infantry => 1,
            UnitType::Artillery => 2,
            UnitType::Motorized => 2,
            UnitType::Armor => 3,
            UnitType::Air => 2,
        }
    }

    /// Movement allowance in MP per turn.
    pub fn movement_allowance(self) -> u8 {
        match self {
            UnitType::Armor | UnitType::Motorized => 6,
            UnitType::Infantry => 3,
            UnitType::Artillery => 2,
            UnitType::Air => 12,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SupplyStatus {
    InSupply,
    OutOfSupply,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub name: String,
    pub side: Side,
    pub nationality: Nationality,
    pub kind: UnitType,
    pub strength: u8,
    pub max_strength: u8,
    /// Movement points remaining this turn.
    pub movement: u8,
    pub movement_allowance: u8,
    /// Fuel carried by mobile/air units.
    pub fuel: u8,
    pub fuel_capacity: u8,
    pub supply: SupplyStatus,
    /// Air units: sorties remaining this turn.
    pub missions: u8,
    pub missions_allowance: u8,
    /// `None` once eliminated (or not yet deployed).
    pub hex: Option<Hex>,
    pub has_attacked: bool,
}

impl Unit {
    pub fn new(
        id: UnitId,
        name: &str,
        side: Side,
        nationality: Nationality,
        kind: UnitType,
        strength: u8,
        hex: Hex,
    ) -> Self {
        let fuel_capacity = match kind {
            UnitType::Armor => 12,
            UnitType::Motorized => 10,
            UnitType::Air => 8,
            _ => 0,
        };
        Unit {
            id,
            name: name.to_string(),
            side,
            nationality,
            kind,
            strength,
            max_strength: strength,
            movement: kind.movement_allowance(),
            movement_allowance: kind.movement_allowance(),
            fuel: fuel_capacity,
            fuel_capacity,
            supply: SupplyStatus::InSupply,
            missions: 2,
            missions_allowance: 2,
            hex: Some(hex),
            has_attacked: false,
        }
    }

    pub fn eliminated(&self) -> bool {
        self.hex.is_none()
    }

    /// Italian troops need extra water to prepare pasta: +1 supply draw.
    pub fn supply_draw(&self) -> u32 {
        let base = self.kind.supply_draw();
        if self.nationality.needs_pasta_water() {
            base + 1
        } else {
            base
        }
    }
}
