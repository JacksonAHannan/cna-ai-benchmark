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
        let content = v["content"][0]["text"]
            .as_str()
            .ok_or_else(|| ModelError::Parse("no assistant content".to_string()))?;
        let action: Action = serde_json::from_str(content)
            .map_err(|e| ModelError::Parse(format!("{e} (content: {content})")))?;
        Ok(action)
    }
}
