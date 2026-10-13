//! Action validation and resolution. The engine is the sole authority: every
//! action is checked against ground truth *and* the submitting side's fog-of-war.

use crate::action::{Action, ActionError, ActionErrorKind};
use crate::events::Event;
use crate::map::{Hex, Site};
use crate::rng::Rng;
use crate::side::Side;
use crate::state::GameState;
use crate::unit::{SupplyStatus, UnitId, UnitType};

/// Range at which a side detects enemy units (hex distance).
const DETECTION_RANGE: i32 = 2;

/// Differential CRT result (research-corrected; see docs/cna-rules.md §15).
/// Percentage losses per side, plus whether the defender must retreat.
#[derive(Debug)]
struct CombatResult {
    attacker_loss_pct: u8,
    defender_loss_pct: u8,
    defender_retreats: bool,
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
            Action::Bomb { squadron, target } => self.resolve_bomb(squadron, target),
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
            Action::Bomb { squadron, target } => self.validate_bomb(side, *squadron, *target),
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

        let motorized = unit.kind.is_mobile();
        let mut cp = 0u32;
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
            cp += mh.terrain.move_cost(motorized) as u32;
            prev = h;
        }
        if unit.movement < cp as u8 {
            return Err(ActionError::new(
                ActionErrorKind::NotEnoughMovement,
                format!("needs {cp} CP, has {}", unit.movement),
            ));
        }
        let fuel_need = (cp + 4) / 5;
        if unit.kind.is_mobile() && (unit.fuel as u32) < fuel_need {
            return Err(ActionError::new(
                ActionErrorKind::OutOfFuel,
                format!("needs {fuel_need} fuel, has {}", unit.fuel),
            ));
        }
        if unit.water == 0 {
            return Err(ActionError::new(
                ActionErrorKind::OutOfWater,
                "unit has no water",
            ));
        }
        Ok(())
    }

    fn resolve_move(&mut self, unit_id: UnitId, path: &[Hex]) {
        let motorized = self.units[&unit_id].kind.is_mobile();
        let mut cp = 0u32;
        for w in path.windows(2) {
            cp += self
                .map
                .get(w[1])
                .map(|m| m.terrain.move_cost(motorized) as u32)
                .unwrap_or(0);
        }
        let from = path[0];
        let to = *path.last().unwrap();
        let fuel_spent = ((cp + 4) / 5) as u8;

        let unit = self.units.get_mut(&unit_id).unwrap();
        unit.movement = unit.movement.saturating_sub(cp as u8);
        if unit.kind.is_mobile() {
            unit.fuel = unit.fuel.saturating_sub(fuel_spent);
        }
        unit.hex = Some(to);

        self.log.push(Event::Move {
            turn: self.turn,
            unit: unit_id,
            from,
            to,
            mp_spent: cp as u8,
            fuel_spent,
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
        if a.movement < 5 {
            return Err(ActionError::new(
                ActionErrorKind::NotEnoughMovement,
                "attack requires 5 CP",
            ));
        }
        if a.ammo == 0 {
            return Err(ActionError::new(
                ActionErrorKind::OutOfAmmo,
                "unit has no ammunition",
            ));
        }
        if a.water == 0 {
            return Err(ActionError::new(
                ActionErrorKind::OutOfWater,
                "unit has no water",
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
        let mut d_str = (t.strength as u32).max(1);
        if t.water == 0 {
            d_str = (d_str / 2).max(1); // waterless defenders fight at half strength
        }

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
        let shift = air_shift - terrain_shift;

        let result = combat_result(&mut self.rng, a_str, d_str, shift);

        // Percentage casualties → step losses (attacker rounds up, defender down).
        let a_losses = ((a_str * result.attacker_loss_pct as u32 + 99) / 100) as u8;
        let d_losses = ((d_str * result.defender_loss_pct as u32) / 100) as u8;

        self.apply_step_loss(attacker, a_losses);
        self.apply_step_loss(target, d_losses);
        if result.defender_retreats
            && !self.units.get(&target).map(|u| u.eliminated()).unwrap_or(true)
        {
            self.retreat_unit(target);
        }

        // Attacker expends its attack.
        let au = self.units.get_mut(&attacker).unwrap();
        au.has_attacked = true;
        au.movement = au.movement.saturating_sub(5);
        au.ammo = au.ammo.saturating_sub(1);

        self.log.push(Event::Attack {
            turn: self.turn,
            attacker,
            defender: target,
            odds_numerator: a_str,
            odds_denominator: d_str,
            result: format!("diff {shift:+}"),
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

    fn validate_bomb(&self, side: Side, squadron: UnitId, target: Hex) -> Result<(), ActionError> {
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

    fn resolve_bomb(&mut self, squadron: UnitId, target: Hex) {
        let side = self.active;
        {
            let su = self.units.get_mut(&squadron).unwrap();
            su.missions -= 1;
        }

        // Interception: strongest enemy air within 4 hexes of the target.
        let mut interceptors: Vec<UnitId> = self
            .units
            .values()
            .filter(|u| {
                !u.eliminated()
                    && u.side != side
                    && u.kind == UnitType::Air
                    && u.hex.map(|h| h.distance(target) <= 4).unwrap_or(false)
            })
            .map(|u| u.id)
            .collect();
        interceptors.sort_by_key(|&id| std::cmp::Reverse(self.units[&id].strength));

        let mut intercepted = false;
        if let Some(defender) = interceptors.into_iter().next() {
            intercepted = true;
            self.air_combat(squadron, defender);
            if self.units.get(&squadron).unwrap().eliminated() {
                return;
            }
        }

        let target_is_enemy_depot = self
            .map
            .get(target)
            .and_then(|m| m.site.as_ref())
            .map(|s| matches!(s, Site::Depot { side: ds, .. } if *ds != side))
            .unwrap_or(false);

        let mut supply_hit = 0;
        if target_is_enemy_depot {
            let dmg = self.depot_damage.entry(target).or_insert(0);
            *dmg += 2;
            supply_hit = 2;
        }

        self.log.push(Event::Bomb {
            turn: self.turn,
            squadron,
            target,
            intercepted,
            supply_hit,
        });
    }

    fn air_combat(&mut self, attacker: UnitId, defender: UnitId) -> (u8, u8) {
        let a = self.units.get(&attacker).unwrap().strength;
        let d = self.units.get(&defender).unwrap().strength;
        let differential = a as i32 - d as i32;
        let roll = self.rng.below(100);
        let a_loss = if differential < 0 {
            2
        } else if roll < 30 {
            1
        } else {
            0
        };
        let d_loss = if differential > 0 {
            2
        } else if roll < 30 {
            1
        } else {
            0
        };
        self.apply_step_loss(attacker, a_loss);
        self.apply_step_loss(defender, d_loss);
        self.log.push(Event::AirCombat {
            turn: self.turn,
            attacker,
            defender,
            attacker_losses: a_loss,
            defender_losses: d_loss,
        });
        (a_loss, d_loss)
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
    let differential = (attacker as i32 - defender as i32 + shift).clamp(-11, 17);
    // Median loss percentages by differential column. Structure follows the
    // published 18-column differential CRT; column VALUES are reconstructed.
    let (a_med, d_med): (u8, u8) = match differential {
        ..=-11 => (40, 0),
        -10..=-8 => (30, 5),
        -7..=-4 => (25, 5),
        -3..=-1 => (20, 10),
        0 => (15, 15),
        1..=3 => (5, 15),
        4..=6 => (5, 25),
        7..=10 => (0, 30),
        11..=16 => (0, 40),
        _ => (0, 50), // >= +17 (overrun)
    };
    let a = vary(rng, a_med);
    let d = vary(rng, d_med);
    let defender_retreats = differential >= 0 && (d_med >= 25 || rng.below(100) < 30);
    CombatResult {
        attacker_loss_pct: a,
        defender_loss_pct: d,
        defender_retreats,
    }
}

/// Vary a median loss percentage by ±10 points, clamped to 0–50.
fn vary(rng: &mut Rng, median: u8) -> u8 {
    if median == 0 {
        return 0;
    }
    let delta = rng.below(21) as i16 - 10;
    (median as i16 + delta).clamp(0, 50) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_scenario;

    #[test]
    fn bomb_disrupts_enemy_depot() {
        let mut state = GameState::new(default_scenario(), 7);
        let side = state.active;
        let squadron = state
            .units
            .values()
            .find(|u| u.side == side && u.kind == UnitType::Air)
            .expect("active side has air")
            .id;
        let depot = state.map.depots_of(side.other())[0];
        let before = state.depot_damage.get(&depot).copied().unwrap_or(0);
        assert!(state
            .apply(side, &Action::Bomb { squadron, target: depot })
            .is_ok());
        let after = state.depot_damage.get(&depot).copied().unwrap_or(0);
        assert!(after > before, "depot damage should increase after a bomb");
        assert!(state.log.events.iter().any(|e| matches!(e, Event::Bomb { .. })));
    }

    #[test]
    fn waterless_unit_cannot_move() {
        let mut state = GameState::new(default_scenario(), 13);
        let side = state.active;
        let unit_id = state
            .units
            .values()
            .find(|u| u.side == side && u.kind == UnitType::Infantry)
            .expect("active side has infantry")
            .id;
        state.units.get_mut(&unit_id).unwrap().water = 0;
        let start = state.units[&unit_id].hex.unwrap();
        let dest = start
            .neighbors()
            .into_iter()
            .find(|&h| {
                state.map.in_bounds(h)
                    && state.map.get(h).map(|m| m.terrain.passable()).unwrap_or(false)
                    && !state.occupied_by_any(h)
            })
            .expect("an empty passable neighbor exists");
        let r = state.apply(
            side,
            &Action::Move {
                unit: unit_id,
                path: vec![start, dest],
            },
        );
        assert!(
            matches!(&r, Err(e) if e.code == ActionErrorKind::OutOfWater),
            "waterless move should be rejected, got {r:?}"
        );
    }
}
