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

    /// Free-form text generation — used for post-game strategy narratives.
    async fn chat(&self, system: &str, user: &str) -> Result<String, ModelError>;
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
- {"type":"bomb","squadron":<airUnitId>,"target":<hex>}  bombs a depot (disrupts supply) or pins enemy units; enemy air within 4 hexes intercepts.
- {"type":"pass"}
- {"type":"endTurn"}

Key rules:
- Out-of-supply units move at half rate and lose strength each turn.
- Mobile units burn fuel to move; fuel evaporates each turn (3%, 7% for British).
- Italian units draw extra water for pasta.
- You may only move/attack units you can see (fog of war); your own units are always visible.
- A unit can attack once per turn; movement and attacks consume MP.

IMPORTANT: Respond with raw JSON only — no XML, no markdown code fences, no HTML/XML tags, no surrounding prose."#;

pub fn build_user_prompt(obs_json: &str, feedback: Option<&ActionError>) -> String {
    let mut s = format!("Observation:\n{obs_json}");
    if let Some(e) = feedback {
        s.push_str(&format!(
            "\n\nYour previous action was illegal: {e}. Correct it and resubmit a legal action."
        ));
    }
    s
}

/// Parse a model's action JSON, tolerating wrappers (markdown fences, `<term>`
/// tags, surrounding prose, and XML `<parameter name="…">…</parameter>` output).
pub fn parse_action(content: &str) -> Result<Action, ModelError> {
    // 1. Direct JSON.
    if let Ok(action) = serde_json::from_str(content) {
        return Ok(action);
    }
    // 2. First `{...}` object embedded in noise.
    if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
        if end > start {
            if let Ok(action) = serde_json::from_str(&content[start..=end]) {
                return Ok(action);
            }
        }
    }
    // 3. XML `<parameter name="key">value</parameter>` form.
    if let Some(v) = xml_parameters_to_json(content) {
        if let Ok(action) = serde_json::from_value(v) {
            return Ok(action);
        }
    }
    let snippet: String = content.chars().take(120).collect();
    Err(ModelError::Parse(format!("no valid JSON in: {snippet}…")))
}

fn xml_parameters_to_json(content: &str) -> Option<serde_json::Value> {
    let mut obj = serde_json::Map::new();
    let mut rest = content;
    while let Some(start) = rest.find("<parameter name=\"") {
        let after_name = &rest[start + "<parameter name=\"".len()..];
        let name_end = after_name.find('"')?;
        let name = &after_name[..name_end];
        let after_name = &after_name[name_end + 1..];
        let value_start = after_name.find('>')?;
        let after_value = &after_name[value_start + 1..];
        let value_end = after_value.find("</parameter>")?;
        let raw_value = &after_value[..value_end];
        let value = serde_json::from_str(raw_value)
            .unwrap_or_else(|_| serde_json::Value::String(raw_value.to_string()));
        obj.insert(name.to_string(), value);
        rest = &after_value[value_end + "</parameter>".len()..];
    }
    if obj.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(obj))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_action;
    use cna_engine::Action;

    #[test]
    fn parses_clean_json() {
        assert!(matches!(parse_action("{\"type\":\"pass\"}").unwrap(), Action::Pass));
    }

    #[test]
    fn parses_markdown_fenced_json() {
        let content = "```json\n{\"type\":\"pass\"}\n```";
        assert!(matches!(parse_action(content).unwrap(), Action::Pass));
    }

    #[test]
    fn parses_term_wrapped_json() {
        let content = "<term>{\"type\":\"endTurn\"}</termml>";
        assert!(matches!(parse_action(content).unwrap(), Action::EndTurn));
    }

    #[test]
    fn parses_xml_parameter_output() {
        let content = "<types>\n<parameter name=\"type\">attack</parameter>\n<parameter name=\"attacker\">10</parameter>\n<parameter name=\"target\">3</parameter>\n</types>";
        let action = parse_action(content).unwrap();
        match action {
            Action::Attack {
                attacker,
                target,
                air_support,
            } => {
                assert_eq!(attacker, 10);
                assert_eq!(target, 3);
                assert!(air_support.is_empty());
            }
            other => panic!("expected Attack, got {other:?}"),
        }
    }
}
