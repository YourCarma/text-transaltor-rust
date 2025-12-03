pub mod config;
pub mod errors;
pub mod models;
pub mod openai;

use std::sync::Arc;

use serde::Deserialize;

use crate::ServiceConnect;
use crate::modules::llm_client::config::LLMClientConfig;
use crate::modules::llm_client::errors::TranslatorResult;
use crate::modules::llm_client::models::TranslateTask;
use crate::modules::llm_client::openai::OpenAIClient;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WorkingMode {
    OPENAI,
}

impl WorkingMode {
    pub async fn create_client(
        &self,
        config: &LLMClientConfig,
    ) -> TranslatorResult<Arc<dyn LLMClient + Send + Sync>> {
        match self {
            WorkingMode::OPENAI => {
                let config = config.openai();
                tracing::info!("Running OPENAI mode!");
                println!("Running OPENAI mode!");
                Ok(Arc::new(OpenAIClient::connect(config).await?))
            }
        }
    }
}

#[async_trait::async_trait]
pub trait LLMClient {
    async fn translate(&self, translate_task: TranslateTask) -> TranslatorResult<String>;
}
