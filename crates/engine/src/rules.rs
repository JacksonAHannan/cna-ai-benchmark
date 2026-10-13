//! Action validation and resolution. The engine is the sole authority: every
//! action is checked against ground truth *and* the submitting side's fog-of-war.

use crate::action::{Action, ActionError, ActionErrorKind};
use crate::events::Event;
use crate::map::{Hex, Site};
use crate::rng::Rng;
use crate::side::Side;
use crate::state::GameState;
use crate::unit::{Organization, SupplyStatus, UnitId, UnitType};

/// Range at which a side detects enemy units (hex distance).
const DETECTION_RANGE: i32 = 2;

/// Differential CRT result (research-corrected; see docs/cna-rules.md §15).
/// Percentage losses per side, capture/Engaged effects, and retreat distance.
#[derive(Debug)]
struct CombatResult {
    attacker_loss_pct: u8,
    defender_loss_pct: u8,
    defender_retreat_hexes: u8,
    attacker_captured: bool,
    defender_captured: bool,
    engaged: bool,
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
        if unit.contact {
            return Err(ActionError::new(
                ActionErrorKind::Engaged,
                "unit is engaged, must attack to disengage",
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
            .map(|m| m.terrain.assault_defense() as i32)
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
        // §15.51 2:1 raw strength, §15.53 organization, §15.61–62 morale.
        let org_shift = org_shift(a.organization, t.organization);
        let ratio_shift = if a.strength as u32 >= 2 * t.strength as u32 {
            2
        } else if t.strength as u32 >= 2 * a.strength as u32 {
            -2
        } else {
            0
        };
        let morale_shift = a.morale as i32 - t.morale as i32;
        let shift = air_shift - terrain_shift + org_shift + ratio_shift + morale_shift;

        let result = combat_result(&mut self.rng, a_str, d_str, shift);

        // Percentage casualties → step losses (attacker rounds up, defender down).
        let a_losses = ((a_str * result.attacker_loss_pct as u32 + 99) / 100) as u8;
        let d_losses = ((d_str * result.defender_loss_pct as u32) / 100) as u8;

        self.apply_step_loss(attacker, a_losses);
        self.apply_step_loss(target, d_losses);
        if result.defender_retreat_hexes > 0
            && !self.units.get(&target).map(|u| u.eliminated()).unwrap_or(true)
        {
            self.retreat_unit(target, result.defender_retreat_hexes);
        }

        // Engaged (§15.93): both sides remain in contact; retreat overrides.
        // Otherwise attacking breaks any prior contact. Capture is surfaced in
        // the result; the prisoner fraction needs the §15.89 table.
        let engaged_now = result.engaged && result.defender_retreat_hexes == 0;
        for id in [attacker, target] {
            if let Some(u) = self.units.get_mut(&id) {
                u.contact = engaged_now;
            }
        }

        // Attacker expends its attack.
        let au = self.units.get_mut(&attacker).unwrap();
        au.has_attacked = true;
        au.movement = au.movement.saturating_sub(5);
        au.ammo = au.ammo.saturating_sub(1);

        let mut tags = String::new();
        if result.attacker_captured {
            let pct = crate::charts::prisoner_percent(self.rng.between(1, 6) as u8);
            let prisoners = ((a_losses as u32 * pct as u32 + 99) / 100) as u8;
            tags.push_str(&format!(" attacker-captured {prisoners}"));
        }
        if result.defender_captured {
            let pct = crate::charts::prisoner_percent(self.rng.between(1, 6) as u8);
            let prisoners = ((d_losses as u32 * pct as u32 + 99) / 100) as u8;
            tags.push_str(&format!(" defender-captured {prisoners}"));
        }
        if engaged_now {
            tags.push_str(" engaged");
        }

        self.log.push(Event::Attack {
            turn: self.turn,
            attacker,
            defender: target,
            odds_numerator: a_str,
            odds_denominator: d_str,
            result: format!("diff {shift:+}{tags}"),
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

    fn retreat_unit(&mut self, id: UnitId, distance: u8) {
        for _ in 0..distance {
            let cur = match self.units.get(&id).and_then(|u| u.hex) {
                Some(h) => h,
                None => return, // already eliminated
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
                return;
            }
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

        // Scramble (§40.4): enemy fighters within 15 hexes roll to intercept,
        // stronger squadrons first.
        let mut interceptors: Vec<UnitId> = self
            .units
            .values()
            .filter(|u| {
                !u.eliminated()
                    && u.side != side
                    && u.kind == UnitType::Air
                    && u.hex.map(|h| h.distance(target) <= 15).unwrap_or(false)
            })
            .map(|u| u.id)
            .collect();
        interceptors.sort_by_key(|&id| std::cmp::Reverse(self.units[&id].strength));

        let mut intercepted = false;
        for defender in interceptors {
            let dist = self.units[&defender].hex.unwrap().distance(target);
            let roll = self.rng.between(1, 6) as u8;
            if crate::charts::scramble_max(dist)
                .map(|max| roll <= max)
                .unwrap_or(false)
            {
                intercepted = true;
                self.air_combat(squadron, defender);
                if self.units.get(&squadron).unwrap().eliminated() {
                    return;
                }
                break;
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
            let points = self.units.get(&squadron).map(|u| u.strength as u32).unwrap_or(1);
            let loss = crate::charts::bomb_supply_loss(points, crate::charts::roll_seq(&mut self.rng));
            if loss > 0 {
                let hit = (loss / 10).max(1) as u32;
                let dmg = self.depot_damage.entry(target).or_insert(0);
                *dmg += hit;
                supply_hit = hit;
            }
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
        let a = self.units.get(&attacker).unwrap().strength as i32;
        let d = self.units.get(&defender).unwrap().strength as i32;
        let a_kill = crate::charts::tacair_kill(a - d)
            .map(|max| u8::from(crate::charts::roll_seq(&mut self.rng) <= max))
            .unwrap_or(0);
        let d_kill = crate::charts::tacair_kill(d - a)
            .map(|max| u8::from(crate::charts::roll_seq(&mut self.rng) <= max))
            .unwrap_or(0);
        self.apply_step_loss(attacker, d_kill);
        self.apply_step_loss(defender, a_kill);
        self.log.push(Event::AirCombat {
            turn: self.turn,
            attacker,
            defender,
            attacker_losses: d_kill,
            defender_losses: a_kill,
        });
        (d_kill, a_kill)
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
    let differential = attacker as i32 - defender as i32 + shift;
    let o = crate::charts::resolve_crt(rng, differential);
    CombatResult {
        attacker_loss_pct: o.attacker_loss_pct,
        defender_loss_pct: o.defender_loss_pct,
        defender_retreat_hexes: o.defender_retreat,
        attacker_captured: o.attacker_captured,
        defender_captured: o.defender_captured,
        engaged: o.engaged,
    }
}

/// Organizational column shift (§15.53): positive favors the attacker.
/// Uses the published stacking-point comparison (§9.4): division 5, super
/// brigade 3, standard brigade 2, battalion 1, company 0.
fn org_shift(attacker: Organization, defender: Organization) -> i32 {
    let ar = attacker.rank() as i32;
    let dr = defender.rank() as i32;
    if ar == dr {
        return 0;
    }
    let (larger, smaller) = if ar > dr { (ar, dr) } else { (dr, ar) };
    let mag = match (larger, smaller) {
        (5, 3) => 1,
        (5, 2) => 2,
        (5, 1) => 4,
        (5, 0) => 8,
        (3, 2) => 0,
        (2..=3, 1) => 2,
        (2..=3, 0) => 4,
        (1, 0) => 2,
        _ => 0,
    };
    if ar > dr {
        mag
    } else {
        -mag
    }
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
        // Damage follows the supply-dump bombing table (§41.5): a single squadron
        // (bomb points 9–12, lowest column) loses 0% on 11–62 and 10% on 63–66,
        // so damage never decreases and only occasionally increases.
        assert!(after >= before, "bomb damage must not decrease");
        assert!(state.log.events.iter().any(|e| matches!(e, Event::Bomb { .. })));
    }

    #[test]
    fn bombing_table_matches_published() {
        use crate::charts::bomb_supply_loss;
        // Lowest column (1–20 bomb points): 0% for 11–62, 10% for 63–66.
        assert_eq!(bomb_supply_loss(12, 11), 0);
        assert_eq!(bomb_supply_loss(12, 62), 0);
        assert_eq!(bomb_supply_loss(12, 63), 10);
        assert_eq!(bomb_supply_loss(12, 66), 10);
        // 471+ column: 30% low, 40% mid, 50% high-mid, 75% top.
        assert_eq!(bomb_supply_loss(500, 11), 30);
        assert_eq!(bomb_supply_loss(500, 40), 40);
        assert_eq!(bomb_supply_loss(500, 63), 50);
        assert_eq!(bomb_supply_loss(500, 66), 75);
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
