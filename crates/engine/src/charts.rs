//! Published chart data, transcribed from the archived rulebook (§15.79 CRT).
//! Values are game mechanics/facts, not copyrightable expression.

use crate::rng::Rng;

/// Sequential 2d6 roll (11–66) → loss percentage. Columns are attacker/defender.
pub const CRT: [(&[(u8, u8, u8)], &[(u8, u8, u8)]); 18] = [
    (&[(64,66,5),(61,63,10),(51,56,15),(41,46,20),(34,36,25),(25,33,30),(16,24,40),(11,15,50)], &[(21,66,0),(12,16,5),(11,11,10)]),
    (&[(66,66,0),(63,65,5),(54,62,10),(44,53,15),(35,43,20),(31,34,25),(21,26,30),(13,16,40),(11,12,50)], &[(24,66,0),(12,23,5),(11,11,10)]),
    (&[(64,66,0),(61,63,5),(52,56,10),(42,51,15),(33,41,20),(24,32,25),(15,23,30),(12,14,40),(11,11,50)], &[(31,66,0),(14,26,5),(11,13,10)]),
    (&[(62,66,0),(54,61,5),(45,53,10),(34,44,15),(24,33,20),(16,23,25),(12,15,30),(11,11,40)], &[(32,66,0),(16,31,5),(12,15,10),(11,11,15)]),
    (&[(61,66,0),(51,56,5),(41,46,10),(31,36,15),(21,26,20),(13,16,25),(11,12,30)], &[(42,66,0),(22,41,5),(14,21,10),(11,13,15)]),
    (&[(55,66,0),(45,54,5),(34,44,10),(21,33,15),(13,18,20),(11,12,25)], &[(45,66,0),(24,44,5),(14,23,10),(12,13,15),(11,11,20)]),
    (&[(53,66,0),(43,52,5),(31,42,10),(14,26,15),(12,13,20),(11,11,25)], &[(51,66,0),(31,46,5),(21,26,10),(12,16,15),(11,11,20)]),
    (&[(52,66,0),(41,51,5),(25,36,10),(13,24,15),(12,12,20),(11,11,25)], &[(52,66,0),(33,51,5),(24,32,10),(13,23,15),(11,12,20)]),
    (&[(51,66,0),(34,46,5),(23,33,10),(13,22,15),(12,12,20),(11,11,25)], &[(52,66,0),(34,51,5),(23,33,10),(14,22,15),(11,13,20)]),
    (&[(45,66,0),(32,44,5),(21,31,10),(13,16,15),(12,12,20),(11,11,25)], &[(53,66,0),(41,52,5),(24,33,10),(14,23,15),(12,13,20),(11,11,25)]),
    (&[(43,66,0),(31,42,5),(21,26,10),(13,16,15),(11,12,20)], &[(55,66,0),(43,54,5),(26,42,10),(15,25,15),(12,14,20),(11,11,25)]),
    (&[(42,66,0),(26,41,5),(16,25,10),(13,15,15),(11,12,20)], &[(61,66,0),(46,56,5),(24,45,10),(21,33,15),(13,16,20),(11,12,25)]),
    (&[(36,66,0),(22,35,5),(14,21,10),(12,13,15),(11,11,20)], &[(63,66,0),(55,62,5),(41,54,10),(24,36,15),(15,23,20),(12,14,25),(11,11,30)]),
    (&[(34,66,0),(22,33,5),(13,21,10),(11,12,15)], &[(65,66,0),(62,64,5),(46,61,10),(32,45,15),(23,31,20),(15,22,25),(12,14,30),(11,11,40)]),
    (&[(32,66,0),(21,31,5),(12,16,10),(11,11,15)], &[(66,66,0),(64,65,5),(61,63,10),(42,56,15),(31,41,20),(21,26,25),(13,16,30),(11,12,40)]),
    (&[(31,66,0),(21,26,5),(11,16,10)], &[(66,66,0),(65,65,5),(63,64,10),(51,62,15),(34,46,20),(23,33,25),(14,22,30),(11,13,40)]),
    (&[(22,66,0),(14,21,5),(11,13,10)], &[(65,66,5),(64,64,10),(56,63,15),(44,55,20),(34,43,25),(26,33,30),(14,25,40),(11,13,50)]),
    (&[(21,66,0),(13,16,5),(11,12,10)], &[(66,66,5),(64,65,10),(61,63,15),(51,56,20),(41,46,25),(31,36,30),(21,26,40),(11,16,50)]),
];

/// Dice-sum (2–12) effects per differential column. `acap`/`dcap` = capture sums
/// (attacker/defender), `aeng` = attacker Engaged sums (apply to both sides),
/// `dr1`/`dr2`/`dr3` = defender 1/2/3-hex retreat sums.
pub struct Effects {
    pub acap: &'static [u8],
    pub aeng: &'static [u8],
    pub dcap: &'static [u8],
    pub dr1: &'static [u8],
    pub dr2: &'static [u8],
    pub dr3: &'static [u8],
}

pub const FX: [Effects; 18] = [
    Effects { acap: &[2,3,4,5,6,7], aeng: &[10,11,12], dcap: &[], dr1: &[], dr2: &[], dr3: &[] },
    Effects { acap: &[2,3,4,5,6], aeng: &[10,11], dcap: &[], dr1: &[], dr2: &[], dr3: &[] },
    Effects { acap: &[2,3,4,5], aeng: &[11,12], dcap: &[], dr1: &[], dr2: &[], dr3: &[] },
    Effects { acap: &[2,3,4], aeng: &[12], dcap: &[], dr1: &[], dr2: &[], dr3: &[] },
    Effects { acap: &[2,3], aeng: &[11,12], dcap: &[], dr1: &[10], dr2: &[], dr3: &[] },
    Effects { acap: &[2], aeng: &[10,11], dcap: &[], dr1: &[9], dr2: &[], dr3: &[] },
    Effects { acap: &[], aeng: &[10,11,12], dcap: &[], dr1: &[8], dr2: &[], dr3: &[] },
    Effects { acap: &[], aeng: &[9,10,12], dcap: &[2], dr1: &[5,6], dr2: &[], dr3: &[] },
    Effects { acap: &[], aeng: &[9,10,11], dcap: &[2], dr1: &[4,5,6], dr2: &[], dr3: &[] },
    Effects { acap: &[], aeng: &[9,10,11,12], dcap: &[2], dr1: &[5,6,7], dr2: &[], dr3: &[] },
    Effects { acap: &[], aeng: &[8,9,10,12], dcap: &[2,3], dr1: &[4,5,6,7], dr2: &[11], dr3: &[] },
    Effects { acap: &[], aeng: &[8,9,10], dcap: &[2,3], dr1: &[3,4,5,6,7], dr2: &[11], dr3: &[] },
    Effects { acap: &[], aeng: &[9,10,12], dcap: &[2,3,4], dr1: &[7,8,9], dr2: &[5], dr3: &[] },
    Effects { acap: &[], aeng: &[10,11,12], dcap: &[2,3,4], dr1: &[7,8,9,12], dr2: &[5], dr3: &[] },
    Effects { acap: &[], aeng: &[9,10,11], dcap: &[2,3,4,5], dr1: &[4,5,6,7,12], dr2: &[8], dr3: &[] },
    Effects { acap: &[], aeng: &[], dcap: &[2,3,4,5,6], dr1: &[3,4,6,7,8,9], dr2: &[12], dr3: &[11] },
    Effects { acap: &[], aeng: &[], dcap: &[2,3,4,5,6,7], dr1: &[2,5,6,7,9,11], dr2: &[8,10], dr3: &[12] },
    Effects { acap: &[], aeng: &[], dcap: &[2,3,4,5,6,7,8], dr1: &[2,3,4,5,6,7,8,9,11], dr2: &[10], dr3: &[12] },
];

/// Close-assault outcome.
pub struct CrtOutcome {
    pub attacker_loss_pct: u8,
    pub defender_loss_pct: u8,
    pub attacker_captured: bool,
    pub defender_captured: bool,
    /// Defender retreat distance in hexes (0 = none).
    pub defender_retreat: u8,
    /// Both sides remain Engaged (attacker's Engaged result; ignored on retreat).
    pub engaged: bool,
}

fn column_index(d: i32) -> usize {
    match d {
        ..=-11 => 0,
        -10..=-8 => 1,
        -7..=-6 => 2,
        -5..=-4 => 3,
        -3 => 4,
        -2 => 5,
        -1 => 6,
        0 => 7,
        1 => 8,
        2 => 9,
        3 => 10,
        4 => 11,
        5..=6 => 12,
        7..=8 => 13,
        9..=10 => 14,
        11..=13 => 15,
        14..=16 => 16,
        _ => 17,
    }
}

fn loss_for(ranges: &[(u8, u8, u8)], roll: u8) -> u8 {
    for &(lo, hi, pct) in ranges {
        if roll >= lo && roll <= hi {
            return pct;
        }
    }
    0
}

fn sum_in(sums: &[u8], sum: u8) -> bool {
    sums.contains(&sum)
}

/// Roll two distinguishable d6; return (tens, ones) so the sequential roll is
/// `tens * 10 + ones` (11–66) and the sum is `tens + ones` (2–12).
fn roll_pair(rng: &mut Rng) -> (u8, u8) {
    (rng.between(1, 6) as u8, rng.between(1, 6) as u8)
}

pub fn resolve_crt(rng: &mut Rng, differential: i32) -> CrtOutcome {
    let col = column_index(differential);
    let (at, ao) = roll_pair(rng);
    let (dt, do_) = roll_pair(rng);
    let a_seq = at * 10 + ao;
    let d_seq = dt * 10 + do_;
    let a_sum = at + ao;
    let d_sum = dt + do_;

    let attacker_loss_pct = loss_for(CRT[col].0, a_seq);
    let defender_loss_pct = loss_for(CRT[col].1, d_seq);

    let fx = &FX[col];
    let attacker_captured = sum_in(fx.acap, a_sum);
    let defender_captured = sum_in(fx.dcap, d_sum);
    let engaged = sum_in(fx.aeng, a_sum);

    let defender_retreat = if sum_in(fx.dr3, d_sum) {
        3
    } else if sum_in(fx.dr2, d_sum) {
        2
    } else if sum_in(fx.dr1, d_sum) {
        1
    } else {
        0
    };

    CrtOutcome {
        attacker_loss_pct,
        defender_loss_pct,
        attacker_captured,
        defender_captured,
        defender_retreat,
        // Retreat overrides Engaged.
        engaged: engaged && defender_retreat == 0,
    }
}

/// Roll sequential 2d6 (11–66).
pub fn roll_seq(rng: &mut Rng) -> u8 {
    let (t, o) = roll_pair(rng);
    t * 10 + o
}

/// Air-to-air: maximum sequential roll that scores a kill (§45.5), or None if no
/// kill is possible.
pub fn tacair_kill(d: i32) -> Option<u8> {
    Some(match d {
        ..=-13 => return None,
        -12..=-5 => 11,
        -4..=-2 => 12,
        -1 => 13,
        0 => 14,
        1 => 15,
        2 => 16,
        3 => 22,
        4 => 24,
        5 => 26,
        6 => 32,
        7 => 34,
        8 => 36,
        9 => 43,
        10 => 46,
        11 => 53,
        _ => 56,
    })
}

/// Squadron-refit die → percentage of planes refitted (§38.37).
pub fn refit_percent(die: u8) -> u8 {
    match die {
        1 => 100,
        2 => 80,
        3 => 70,
        4 => 60,
        5 => 50,
        6..=7 => 40,
        _ => 33,
    }
}

/// Supply-dump bombing loss (§41.5): 11 bomb-point columns × roll ranges.
pub const BOMB_SUPPLY: [&[(u8, u8, u8)]; 11] = [
    &[(11,62,0),(63,66,10)],
    &[(11,60,0),(61,66,10)],
    &[(11,50,0),(51,66,10)],
    &[(11,34,0),(35,64,10),(65,66,20)],
    &[(11,20,0),(21,56,10),(57,60,0),(61,66,20)],
    &[(11,46,10),(47,50,0),(51,64,20),(65,66,30)],
    &[(11,34,10),(35,56,20),(57,60,0),(61,66,30)],
    &[(11,16,10),(17,20,0),(21,46,20),(47,50,0),(51,64,30),(65,66,40)],
    &[(11,34,20),(35,56,30),(57,60,0),(61,66,40)],
    &[(11,16,20),(17,20,0),(21,46,30),(47,50,0),(51,64,40),(65,66,50)],
    &[(11,34,30),(35,56,40),(57,60,0),(61,64,50),(65,66,75)],
];

fn bomb_column(points: u32) -> usize {
    match points {
        1..=20 => 0,
        21..=40 => 1,
        41..=80 => 2,
        81..=120 => 3,
        121..=160 => 4,
        161..=200 => 5,
        201..=260 => 6,
        261..=320 => 7,
        321..=390 => 8,
        391..=470 => 9,
        _ => 10,
    }
}

/// Percentage loss of each supply type for a bombing run of `points` bomb points.
pub fn bomb_supply_loss(points: u32, roll: u8) -> u8 {
    loss_for(BOMB_SUPPLY[bomb_column(points)], roll)
}

// === Supply/logistics chart (§2, §54) ===

/// Tons per supply point of each resource (§54.5).
pub const FUEL_TONS: f64 = 1.0 / 8.0;
pub const AMMO_TONS: f64 = 4.0;
pub const STORES_TONS: f64 = 1.0;
pub const WATER_TONS: f64 = 1.0 / 6.0;

/// Carrying capacity of one truck point (alternatives, not additive; §54).
pub struct TruckCapacity {
    pub infantry: f64, // TOE points (light carries 1/2)
    pub guns: u32,     // 0 = prohibited
    pub aa: u32,
    pub ammo: u32,
    pub fuel: u32,
    pub stores: u32,
    pub water: u32,
    pub own_fuel: u32,
}

pub const TRUCK_CAPACITY: [TruckCapacity; 3] = [
    TruckCapacity { infantry: 0.5, guns: 0, aa: 1, ammo: 2, fuel: 50, stores: 6, water: 40, own_fuel: 8 },
    TruckCapacity { infantry: 1.0, guns: 1, aa: 2, ammo: 4, fuel: 120, stores: 15, water: 100, own_fuel: 6 },
    TruckCapacity { infantry: 2.0, guns: 1, aa: 4, ammo: 8, fuel: 250, stores: 30, water: 200, own_fuel: 6 },
];

/// Maximum dump stock, order ammo/fuel/stores/water, by location class (§54).
pub const DUMP_MAXIMA: [[u32; 4]; 3] = [
    [2500, 8000, 3000, 1000],                                 // village
    [1500, 5000, 1000, 1000],                                 // other terrain
    [u32::MAX, u32::MAX, u32::MAX, u32::MAX],                 // major city / Tripoli / Tunis
];

/// Commonwealth rail tonnage per Operations Stage (§54).
pub const RAIL_TONS: u32 = 1500;
