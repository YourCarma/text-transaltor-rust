use getset::{CopyGetters, Getters};
use serde::Deserialize;

use crate::modules::llm_client::openai::config::OpenAIClientConfig;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct LLMClientConfig {
    openai: OpenAIClientConfig,
}
