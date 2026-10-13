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

    /// Movement allowance in CP per turn (research §6; tanks ~25, infantry ~10).
    pub fn movement_allowance(self) -> u8 {
        match self {
            UnitType::Armor => 25,
            UnitType::Motorized => 20,
            UnitType::Infantry => 10,
            UnitType::Artillery => 20,
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
    /// Ammunition points: refilled when in supply, consumed per attack. No ammo
    /// means the unit cannot attack.
    pub ammo: u8,
    pub ammo_capacity: u8,
    /// Water points: refilled when in supply, consumed per turn (Italian +1 for
    /// pasta). Waterless units cannot move or attack, and defend at half strength.
    pub water: u8,
    pub water_capacity: u8,
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
        let ammo_capacity = match kind {
            UnitType::Air => 0,
            UnitType::Artillery => 4,
            _ => 3,
        };
        Unit {
            id,
            name: name.to_string(),
            side,
            nationality,
            kind,
            strength,
            max_strength: if kind == UnitType::Air {
                nationality.squadron_capacity().1
            } else {
                strength
            },
            movement: kind.movement_allowance(),
            movement_allowance: kind.movement_allowance(),
            fuel: fuel_capacity,
            fuel_capacity,
            supply: SupplyStatus::InSupply,
            missions: 2,
            missions_allowance: 2,
            ammo: ammo_capacity,
            ammo_capacity,
            water: 2,
            water_capacity: 2,
            hex: Some(hex),
            has_attacked: false,
        }
    }

    pub fn eliminated(&self) -> bool {
        self.hex.is_none()
    }

    /// Stores (food/misc) demand per turn.
    pub fn stores_draw(&self) -> u32 {
        self.kind.supply_draw()
    }

    /// Water demand per turn. Italian troops need +1 to prepare pasta.
    pub fn water_draw(&self) -> u32 {
        if self.nationality.needs_pasta_water() {
            2
        } else {
            1
        }
    }
}
