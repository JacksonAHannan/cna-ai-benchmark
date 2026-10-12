//! Throwaway: verify the full model-adapter path (prompt -> API -> parse ->
//! validate -> apply -> feedback) over a handful of real actions.

use cna_engine::{default_scenario, GameState, Side};
use cna_models::{build_agent, Agent, AgentConfig};

#[tokio::main]
async fn main() {
    let text = std::fs::read_to_string("agents.json").expect("agents.json");
    let configs: Vec<AgentConfig> = serde_json::from_str(&text).expect("parse agents.json");
    let allied = build_agent(&configs[0]);
    let axis = build_agent(&configs[1]);

    let mut state = GameState::new(default_scenario(), 1234);
    let mut feedback: Option<cna_engine::ActionError> = None;

    for step in 0..12 {
        let side = state.active;
        let agent: &dyn Agent = if side == Side::Allied {
            allied.as_ref()
        } else {
            axis.as_ref()
        };
        let obs = state.observation(side);
        match agent.act(&obs, feedback.as_ref()).await {
            Ok(action) => {
                println!("[{step:02}] {side:<6} {action:?}");
                match state.apply(side, &action) {
                    Ok(events) => {
                        println!("       -> ok ({} events)", events.len());
                        feedback = None;
                    }
                    Err(e) => {
                        println!("       !! illegal: {}", e.message);
                        feedback = Some(e);
                    }
                }
            }
            Err(e) => {
                println!("[{step:02}] {side:<6} MODEL ERROR: {e}");
                feedback = None;
            }
        }
        if state.is_over() {
            println!("       game over, winner={:?}", state.winner);
            break;
        }
    }
}
