//! Sides and nationalities.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Side {
    Allied,
    Axis,
}

impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::Allied => Side::Axis,
            Side::Axis => Side::Allied,
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Side::Allied => "Allied",
            Side::Axis => "Axis",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Nationality {
    British,
    Commonwealth,
    German,
    Italian,
}

impl Nationality {
    pub fn side(self) -> Side {
        match self {
            Nationality::British | Nationality::Commonwealth => Side::Allied,
            Nationality::German | Nationality::Italian => Side::Axis,
        }
    }

    /// CNA's signature rule: Italian troops require extra water to prepare pasta.
    pub fn needs_pasta_water(self) -> bool {
        matches!(self, Nationality::Italian)
    }
}

impl std::fmt::Display for Nationality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Nationality::British => "British",
            Nationality::Commonwealth => "Commonwealth",
            Nationality::German => "German",
            Nationality::Italian => "Italian",
        })
    }
}
