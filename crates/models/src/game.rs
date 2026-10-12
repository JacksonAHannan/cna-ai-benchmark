//! Shared game-driving loop: runs an engine [`GameState`] against two [`Agent`]s,
//! streaming every emitted event batch through a callback. Used by both the
//! headless benchmark and the live-spectate server so the rules-of-play stay in
//! one place.

use crate::Agent;
use cna_engine::{Action, ActionError, Event, GameState, Side};
use std::sync::Arc;

pub const MAX_STEPS_PER_TURN: usize = 200;
pub const MAX_RETRIES: usize = 4;

#[derive(Debug, Clone)]
pub struct GameStats {
    pub winner: Option<Side>,
    pub turns: u32,
    pub illegal_allied: u32,
    pub illegal_axis: u32,
}

/// Drive the game to completion, calling `on_events` with each batch of events
/// the engine emits (actions, resolutions, turn transitions).
pub async fn run_streaming(
    mut state: GameState,
    allied: Arc<dyn Agent>,
    axis: Arc<dyn Agent>,
    mut on_events: impl FnMut(Vec<Event>, &GameState),
) -> GameStats {
    let mut illegal_allied = 0u32;
    let mut illegal_axis = 0u32;

    while !state.is_over() {
        let side = state.active;
        let agent: &dyn Agent = if side == Side::Allied {
            allied.as_ref()
        } else {
            axis.as_ref()
        };

        let mut steps = 0usize;
        let mut feedback: Option<ActionError> = None;
        let mut consecutive = 0usize;

        loop {
            if steps >= MAX_STEPS_PER_TURN || state.is_over() {
                break;
            }

            let obs = state.observation(side);
            let action = match agent.act(&obs, feedback.as_ref()).await {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("[{}] model error: {e}", agent.name());
                    break;
                }
            };

            match state.apply(side, &action) {
                Ok(events) => {
                    steps += 1;
                    consecutive = 0;
                    feedback = None;
                    if !events.is_empty() {
                        on_events(events, &state);
                    }
                    if matches!(action, Action::EndTurn) || state.is_over() {
                        break;
                    }
                }
                Err(e) => {
                    steps += 1;
                    consecutive += 1;
                    if side == Side::Allied {
                        illegal_allied += 1;
                    } else {
                        illegal_axis += 1;
                    }
                    if consecutive > MAX_RETRIES {
                        consecutive = 0;
                        feedback = None;
                    } else {
                        feedback = Some(e);
                    }
                }
            }
        }

        // If the agent never ended its turn (step cap), force it.
        if !state.is_over() && state.active == side {
            let events = state.apply(side, &Action::EndTurn).unwrap_or_default();
            if !events.is_empty() {
                on_events(events, &state);
            }
        }
    }

    GameStats {
        winner: state.winner,
        turns: state.turn,
        illegal_allied,
        illegal_axis,
    }
}
