use getset::{CopyGetters, Getters};
use serde::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct OpenAIClientConfig {
    address: String,
    openai_api_key: String,
    model_name: String,
    use_proxy: bool,
    proxy_address: String,
}
