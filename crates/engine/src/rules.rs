//! Action validation and resolution. The engine is the sole authority: every
//! action is checked against ground truth *and* the submitting side's fog-of-war.

use crate::action::{Action, ActionError, ActionErrorKind};
use crate::events::Event;
use crate::map::Hex;
use crate::rng::Rng;
use crate::side::Side;
use crate::state::GameState;
use crate::unit::{SupplyStatus, UnitId, UnitType};

/// Range at which a side detects enemy units (hex distance).
const DETECTION_RANGE: i32 = 2;

/// CRT result classes (attacker perspective).
#[derive(Debug)]
enum CombatResult {
    /// Attacker eliminated.
    Ae,
    /// Attacker retreats (with a step loss).
    Ar,
    /// Exchange — both lose a step.
    Ex,
    /// Defender retreats (with a step loss).
    Dr,
    /// Defender eliminated.
    De,
    /// No effect.
    Ne,
}

impl GameState {
    pub fn apply(&mut self, side: Side, action: &Action) -> Result<Vec<Event>, ActionError> {
        if self.game_over {
            return Err(ActionError::new(ActionErrorKind::GameOver, "game is over"));
        }
        if side != self.active {
            return Err(ActionError::new(
                ActionErrorKind::NotYourTurn,
                format!("it is {}'s turn", self.active),
            ));
        }

        if let Err(e) = self.validate(side, action) {
            self.log.push(Event::IllegalAction {
                turn: self.turn,
                side,
                action: action.clone(),
                error: e.to_string(),
            });
            return Err(e);
        }

        let start = self.log.len();
        match action.clone() {
            Action::Move { unit, path } => self.resolve_move(unit, &path),
            Action::Attack {
                attacker,
                target,
                air_support,
            } => self.resolve_attack(attacker, target, &air_support),
            Action::AirRecon { squadron, target } => self.resolve_air_recon(squadron, target),
            Action::Pass => {}
            Action::EndTurn => self.end_side_turn(),
        }
        self.check_elimination();
        Ok(self.log.events[start..].to_vec())
    }

    pub fn validate(&self, side: Side, action: &Action) -> Result<(), ActionError> {
        match action {
            Action::Move { unit, path } => self.validate_move(side, *unit, path),
            Action::Attack {
                attacker,
                target,
                air_support,
            } => self.validate_attack(side, *attacker, *target, air_support),
            Action::AirRecon { squadron, target } => {
                self.validate_air_recon(side, *squadron, *target)
            }
            Action::Pass | Action::EndTurn => Ok(()),
        }
    }

    // ---- movement ----

    fn validate_move(
        &self,
        side: Side,
        unit_id: UnitId,
        path: &[Hex],
    ) -> Result<(), ActionError> {
        if path.is_empty() {
            return Err(ActionError::new(
                ActionErrorKind::EmptyPath,
                "path is empty",
            ));
        }
        let unit = self
            .units
            .get(&unit_id)
            .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownUnit, "unknown unit"))?;
        if unit.eliminated() {
            return Err(ActionError::new(
                ActionErrorKind::EliminatedUnit,
                "unit is eliminated",
            ));
        }
        if unit.side != side {
            return Err(ActionError::new(
                ActionErrorKind::NotYourUnit,
                "not your unit",
            ));
        }
        let start = unit
            .hex
            .ok_or_else(|| ActionError::new(ActionErrorKind::EliminatedUnit, "unit is eliminated"))?;
        if path[0] != start {
            return Err(ActionError::new(
                ActionErrorKind::PathDiscontinuous,
                "path must start at the unit's current hex",
            ));
        }
        if path.len() < 2 {
            return Err(ActionError::new(
                ActionErrorKind::EmptyPath,
                "path has no movement",
            ));
        }

        let mut mp = 0u32;
        let mut fuel = 0u32;
        let mut prev = start;
        for &h in &path[1..] {
            if prev.distance(h) != 1 {
                return Err(ActionError::new(
                    ActionErrorKind::PathDiscontinuous,
                    "path steps must be adjacent",
                ));
            }
            let mh = self
                .map
                .get(h)
                .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownHex, "hex out of bounds"))?;
            if !mh.terrain.passable() {
                return Err(ActionError::new(
                    ActionErrorKind::ImpassableHex,
                    "hex is impassable",
                ));
            }
            if self.occupied_by_any(h) {
                return Err(ActionError::new(
                    ActionErrorKind::InvalidTarget,
                    "hex is occupied",
                ));
            }
            mp += mh.terrain.move_cost() as u32;
            fuel += 1;
            prev = h;
        }
        if unit.movement < mp as u8 {
            return Err(ActionError::new(
                ActionErrorKind::NotEnoughMovement,
                format!("needs {mp} MP, has {}", unit.movement),
            ));
        }
        if unit.kind.is_mobile() && unit.fuel < fuel as u8 {
            return Err(ActionError::new(
                ActionErrorKind::OutOfFuel,
                format!("needs {fuel} fuel, has {}", unit.fuel),
            ));
        }
        Ok(())
    }

    fn resolve_move(&mut self, unit_id: UnitId, path: &[Hex]) {
        let mut mp = 0u32;
        let mut fuel = 0u32;
        for w in path.windows(2) {
            mp += self.map.get(w[1]).map(|m| m.terrain.move_cost() as u32).unwrap_or(0);
            fuel += 1;
        }
        let from = path[0];
        let to = *path.last().unwrap();

        let unit = self.units.get_mut(&unit_id).unwrap();
        unit.movement = unit.movement.saturating_sub(mp as u8);
        if unit.kind.is_mobile() {
            unit.fuel = unit.fuel.saturating_sub(fuel as u8);
        }
        unit.hex = Some(to);

        self.log.push(Event::Move {
            turn: self.turn,
            unit: unit_id,
            from,
            to,
            mp_spent: mp as u8,
            fuel_spent: fuel as u8,
        });
    }

    // ---- combat ----

    fn validate_attack(
        &self,
        side: Side,
        attacker: UnitId,
        target: UnitId,
        air_support: &[UnitId],
    ) -> Result<(), ActionError> {
        let a = self
            .units
            .get(&attacker)
            .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownUnit, "unknown attacker"))?;
        if a.eliminated() {
            return Err(ActionError::new(
                ActionErrorKind::EliminatedUnit,
                "attacker is eliminated",
            ));
        }
        if a.side != side {
            return Err(ActionError::new(ActionErrorKind::NotYourUnit, "not your unit"));
        }
        if a.has_attacked {
            return Err(ActionError::new(
                ActionErrorKind::AlreadyAttacked,
                "unit already attacked this turn",
            ));
        }
        if a.movement < 1 {
            return Err(ActionError::new(
                ActionErrorKind::NotEnoughMovement,
                "unit has no movement left to attack",
            ));
        }

        let t = self
            .units
            .get(&target)
            .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownUnit, "unknown target"))?;
        if t.eliminated() {
            return Err(ActionError::new(
                ActionErrorKind::EliminatedUnit,
                "target is eliminated",
            ));
        }
        if t.side == side {
            return Err(ActionError::new(
                ActionErrorKind::InvalidTarget,
                "cannot attack your own unit",
            ));
        }

        let target_hex = t.hex.unwrap();
        if !self.is_visible_to(side, target_hex) {
            return Err(ActionError::new(
                ActionErrorKind::UnseenTarget,
                "target is not observed (fog of war)",
            ));
        }

        let range = a.hex.unwrap().distance(target_hex);
        match a.kind {
            UnitType::Artillery => {
                if !(1..=2).contains(&range) {
                    return Err(ActionError::new(
                        ActionErrorKind::NotInRange,
                        "artillery range is 1-2 hexes",
                    ));
                }
            }
            UnitType::Air => {
                return Err(ActionError::new(
                    ActionErrorKind::InvalidTarget,
                    "air units sortie via AirRecon or air support",
                ));
            }
            _ => {
                if range != 1 {
                    return Err(ActionError::new(
                        ActionErrorKind::NotAdjacent,
                        "attacker must be adjacent",
                    ));
                }
            }
        }

        for &s in air_support {
            let su = self
                .units
                .get(&s)
                .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownUnit, "unknown air unit"))?;
            if su.side != side {
                return Err(ActionError::new(ActionErrorKind::NotYourUnit, "not your air unit"));
            }
            if su.kind != UnitType::Air {
                return Err(ActionError::new(ActionErrorKind::NotAirUnit, "not an air unit"));
            }
            if su.eliminated() {
                return Err(ActionError::new(
                    ActionErrorKind::EliminatedUnit,
                    "air unit is eliminated",
                ));
            }
            if su.missions == 0 {
                return Err(ActionError::new(
                    ActionErrorKind::NoMissions,
                    "air unit has no sorties left",
                ));
            }
        }
        Ok(())
    }

    fn resolve_attack(&mut self, attacker: UnitId, target: UnitId, air_support: &[UnitId]) {
        let a = self.units.get(&attacker).unwrap().clone();
        let t = self.units.get(&target).unwrap().clone();

        let mut a_str = a.strength as u32;
        if a.supply == SupplyStatus::OutOfSupply {
            a_str = (a_str / 2).max(1);
        }
        let d_str = (t.strength as u32).max(1);

        let terrain_shift = self
            .map
            .get(t.hex.unwrap())
            .map(|m| m.terrain.defense() as i32)
            .unwrap_or(0);

        let mut air_shift = 0i32;
        for &s in air_support {
            let su = self.units.get_mut(&s).unwrap();
            if su.missions > 0 {
                su.missions -= 1;
                air_shift += 1;
            }
        }
        air_shift = air_shift.min(2);
        let shift = terrain_shift + air_shift;

        let result = combat_result(&mut self.rng, a_str, d_str, shift);

        let a_losses;
        let d_losses;
        match result {
            CombatResult::De => {
                a_losses = 0;
                d_losses = t.strength;
                self.eliminate_unit(target, "combat");
            }
            CombatResult::Dr => {
                a_losses = 0;
                d_losses = 1;
                self.apply_step_loss(target, 1);
                self.retreat_unit(target);
            }
            CombatResult::Ex => {
                a_losses = 1;
                d_losses = 1;
                self.apply_step_loss(attacker, 1);
                self.apply_step_loss(target, 1);
            }
            CombatResult::Ar => {
                a_losses = 1;
                d_losses = 0;
                self.apply_step_loss(attacker, 1);
                self.retreat_unit(attacker);
            }
            CombatResult::Ae => {
                a_losses = a.strength;
                d_losses = 0;
                self.eliminate_unit(attacker, "combat");
            }
            CombatResult::Ne => {
                a_losses = 0;
                d_losses = 0;
            }
        }

        // Attacker expends its attack.
        let au = self.units.get_mut(&attacker).unwrap();
        au.has_attacked = true;
        au.movement = au.movement.saturating_sub(1);

        self.log.push(Event::Attack {
            turn: self.turn,
            attacker,
            defender: target,
            odds_numerator: a_str,
            odds_denominator: d_str,
            result: format!("{result:?}"),
            attacker_losses: a_losses,
            defender_losses: d_losses,
        });
    }

    fn apply_step_loss(&mut self, id: UnitId, n: u8) {
        let strength = {
            let u = self.units.get_mut(&id).unwrap();
            u.strength = u.strength.saturating_sub(n);
            u.strength
        };
        if strength == 0 {
            self.eliminate_unit(id, "combat");
        }
    }

    fn retreat_unit(&mut self, id: UnitId) {
        let cur = match self.units.get(&id).and_then(|u| u.hex) {
            Some(h) => h,
            None => return, // already eliminated (e.g. by a prior step loss)
        };
        let candidates: Vec<Hex> = cur
            .neighbors()
            .into_iter()
            .filter(|&h| {
                self.map.in_bounds(h)
                    && self.map.get(h).unwrap().terrain.passable()
                    && !self.occupied_by_any(h)
            })
            .collect();
        if candidates.is_empty() {
            self.eliminate_unit(id, "no retreat path");
        } else {
            let idx = self.rng.below(candidates.len() as u64) as usize;
            let to = candidates[idx];
            self.units.get_mut(&id).unwrap().hex = Some(to);
            self.log.push(Event::Retreat {
                turn: self.turn,
                unit: id,
                from: cur,
                to,
            });
        }
    }

    // ---- air ----

    fn validate_air_recon(&self, side: Side, squadron: UnitId, target: Hex) -> Result<(), ActionError> {
        let su = self
            .units
            .get(&squadron)
            .ok_or_else(|| ActionError::new(ActionErrorKind::UnknownUnit, "unknown air unit"))?;
        if su.side != side {
            return Err(ActionError::new(ActionErrorKind::NotYourUnit, "not your air unit"));
        }
        if su.kind != UnitType::Air {
            return Err(ActionError::new(ActionErrorKind::NotAirUnit, "not an air unit"));
        }
        if su.eliminated() {
            return Err(ActionError::new(
                ActionErrorKind::EliminatedUnit,
                "air unit is eliminated",
            ));
        }
        if su.missions == 0 {
            return Err(ActionError::new(
                ActionErrorKind::NoMissions,
                "air unit has no sorties left",
            ));
        }
        if !self.map.in_bounds(target) {
            return Err(ActionError::new(ActionErrorKind::UnknownHex, "hex out of bounds"));
        }
        Ok(())
    }

    fn resolve_air_recon(&mut self, squadron: UnitId, target: Hex) {
        {
            let su = self.units.get_mut(&squadron).unwrap();
            su.missions -= 1;
        }
        let mut hexes = vec![target];
        hexes.extend(target.neighbors());

        let mut revealed = Vec::new();
        let side = self.active;
        for h in hexes {
            self.recon.get_mut(&side).unwrap().insert(h);
            for u in self.units.values() {
                if u.side != side && !u.eliminated() && u.hex == Some(h) {
                    revealed.push(u.id);
                }
            }
        }
        self.log.push(Event::AirRecon {
            turn: self.turn,
            squadron,
            target,
            revealed,
        });
    }

    // ---- shared helpers ----

    pub(crate) fn occupied_by_any(&self, hex: Hex) -> bool {
        self.units.values().any(|u| !u.eliminated() && u.hex == Some(hex))
    }

    pub(crate) fn is_visible_to(&self, side: Side, hex: Hex) -> bool {
        if self
            .recon
            .get(&side)
            .map(|s| s.contains(&hex))
            .unwrap_or(false)
        {
            return true;
        }
        self.units.values().any(|u| {
            !u.eliminated()
                && u.side == side
                && u.hex
                    .map(|uh| uh.distance(hex) <= DETECTION_RANGE)
                    .unwrap_or(false)
        })
    }
}

fn combat_result(rng: &mut Rng, attacker: u32, defender: u32, shift: i32) -> CombatResult {
    let mut ratio = attacker as f64 / (defender.max(1) as f64);
    if shift >= 0 {
        ratio *= 2f64.powi(shift);
    } else {
        ratio /= 2f64.powi(-shift);
    }

    let roll = rng.below(100);
    if ratio >= 3.0 {
        if roll < 70 {
            CombatResult::De
        } else {
            CombatResult::Dr
        }
    } else if ratio >= 2.0 {
        if roll < 40 {
            CombatResult::De
        } else if roll < 80 {
            CombatResult::Dr
        } else {
            CombatResult::Ex
        }
    } else if ratio >= 1.0 {
        if roll < 40 {
            CombatResult::Ex
        } else if roll < 65 {
            CombatResult::Dr
        } else {
            CombatResult::Ar
        }
    } else if ratio >= 0.5 {
        if roll < 50 {
            CombatResult::Ar
        } else if roll < 80 {
            CombatResult::Ex
        } else {
            CombatResult::Ne
        }
    } else if roll < 30 {
        CombatResult::Ae
    } else {
        CombatResult::Ar
    }
}
