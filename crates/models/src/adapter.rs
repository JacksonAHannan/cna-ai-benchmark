//! The [`Agent`] trait and provider registry.

use async_trait::async_trait;
use cna_engine::{Action, ActionError, Observation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Mock,
    OpenAi,
    OpenAiCompatible,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub provider: Provider,
    pub name: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
}

impl AgentConfig {
    pub fn mock(name: &str) -> Self {
        AgentConfig {
            provider: Provider::Mock,
            name: name.to_string(),
            model: "mock".to_string(),
            base_url: None,
            api_key: None,
            temperature: 0.0,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("http error: {0}")]
    Http(String),
    #[error("api error: {0}")]
    Api(String),
    #[error("could not parse action JSON: {0}")]
    Parse(String),
}

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;

    /// Produce the next action for the given observation. `feedback` carries the
    /// previous action's validation error (if any) so the model can correct itself.
    async fn act(
        &self,
        obs: &Observation,
        feedback: Option<&ActionError>,
    ) -> Result<Action, ModelError>;
}

pub fn build_agent(config: &AgentConfig) -> Box<dyn Agent> {
    match config.provider {
        Provider::Mock => Box::new(crate::mock::MockAgent::new(config.name.clone())),
        Provider::OpenAi | Provider::OpenAiCompatible => {
            Box::new(crate::openai::OpenAiAgent::new(config))
        }
        Provider::Anthropic => Box::new(crate::anthropic::AnthropicAgent::new(config)),
    }
}

/// The system prompt and JSON action schema shared by all LLM agents.
pub const ACTION_SCHEMA_DOC: &str = r#"You are a wargame commander in a digital adaptation of SPI's "The Campaign for North Africa" (faithful-in-spirit: weekly turns, logistics, fuel evaporation, the Italian pasta-water rule, fog of war).

You receive a JSON observation of the game and must respond with EXACTLY ONE JSON action — nothing else, no commentary, no markdown fences.

Action variants (hexes are {"q":int,"r":int} axial coordinates):
- {"type":"move","unit":<unitId>,"path":[<hex>,...]}   path[0] must equal the unit's current hex; each step must be adjacent, passable, and unoccupied.
- {"type":"attack","attacker":<unitId>,"target":<unitId>,"airSupport":[<airUnitId>,...]}  attacker must be adjacent (artillery 1-2 hexes); target must be visible.
- {"type":"airRecon","squadron":<airUnitId>,"target":<hex>}  reveals enemy units in/around the hex.
- {"type":"pass"}
- {"type":"endTurn"}

Key rules:
- Out-of-supply units move at half rate and lose strength each turn.
- Mobile units burn fuel to move; fuel evaporates each turn (3%, 7% for British).
- Italian units draw extra water for pasta.
- You may only move/attack units you can see (fog of war); your own units are always visible.
- A unit can attack once per turn; movement and attacks consume MP."#;

pub fn build_user_prompt(obs_json: &str, feedback: Option<&ActionError>) -> String {
    let mut s = format!("Observation:\n{obs_json}");
    if let Some(e) = feedback {
        s.push_str(&format!(
            "\n\nYour previous action was illegal: {e}. Correct it and resubmit a legal action."
        ));
    }
    s
}
