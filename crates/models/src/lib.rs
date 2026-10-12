//! Provider-agnostic model adapter. One [`Agent`] controls one side of a game.

pub mod adapter;
pub mod anthropic;
pub mod game;
pub mod mock;
pub mod openai;

pub use adapter::{build_agent, Agent, AgentConfig, ModelError, Provider, ACTION_SCHEMA_DOC};
pub use game::{run_streaming, GameStats};
