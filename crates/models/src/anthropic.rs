//! Anthropic Messages API agent.

use crate::adapter::{build_user_prompt, Agent, AgentConfig, ModelError, ACTION_SCHEMA_DOC};
use async_trait::async_trait;
use cna_engine::{Action, ActionError, Observation};
use serde_json::json;

pub struct AnthropicAgent {
    name: String,
    model: String,
    api_key: String,
    temperature: f32,
    client: reqwest::Client,
}

impl AnthropicAgent {
    pub fn new(config: &AgentConfig) -> Self {
        let api_key = config
            .api_key
            .clone()
            .unwrap_or_else(|| std::env::var("ANTHROPIC_API_KEY").unwrap_or_default());
        AnthropicAgent {
            name: config.name.clone(),
            model: config.model.clone(),
            api_key,
            temperature: config.temperature,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Agent for AnthropicAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn act(
        &self,
        obs: &Observation,
        feedback: Option<&ActionError>,
    ) -> Result<Action, ModelError> {
        let obs_json = serde_json::to_string(obs).map_err(|e| ModelError::Parse(e.to_string()))?;
        let mut body = json!({
            "model": self.model,
            "max_tokens": 1024,
            "system": ACTION_SCHEMA_DOC,
            "messages": [
                {"role": "user", "content": build_user_prompt(&obs_json, feedback)},
            ]
        });
        // claude-5.x rejects `temperature` entirely; only send it when non-zero.
        if self.temperature > 0.0 {
            body["temperature"] = json!(self.temperature);
        }

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| ModelError::Http(e.to_string()))?;

        let status = resp.status();
        let text = resp.text().await.map_err(|e| ModelError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(ModelError::Api(format!("{status}: {text}")));
        }

        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ModelError::Parse(e.to_string()))?;
        let content = extract_text(&v)
            .ok_or_else(|| ModelError::Parse("no assistant text content".to_string()))?;
        let action: Action = serde_json::from_str(&content)
            .map_err(|e| ModelError::Parse(format!("{e} (content: {content})")))?;
        Ok(action)
    }
}

/// Collect every `text` block from an Anthropic message response. Thinking models
/// emit `content: [{type:"thinking", ...}, {type:"text", ...}]` — the answer is in
/// the `text` block(s), not necessarily `content[0]`.
fn extract_text(v: &serde_json::Value) -> Option<String> {
    let blocks = v["content"].as_array()?;
    let texts: Vec<&str> = blocks
        .iter()
        .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
        .collect();
    if texts.is_empty() {
        None
    } else {
        Some(texts.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::extract_text;
    use serde_json::json;

    #[test]
    fn extracts_text_after_thinking_block() {
        let v = json!({"content": [
            {"type": "thinking", "thinking": "hmm"},
            {"type": "text", "text": "{\"type\":\"pass\"}"}
        ]});
        assert_eq!(extract_text(&v).unwrap(), "{\"type\":\"pass\"}");
    }

    #[test]
    fn returns_none_when_only_thinking() {
        let v = json!({"content": [{"type": "thinking", "thinking": "hmm"}]});
        assert!(extract_text(&v).is_none());
    }

    #[test]
    fn extracts_plain_text_block() {
        let v = json!({"content": [{"type": "text", "text": "{\"type\":\"endTurn\"}"}]});
        assert_eq!(extract_text(&v).unwrap(), "{\"type\":\"endTurn\"}");
    }
}
