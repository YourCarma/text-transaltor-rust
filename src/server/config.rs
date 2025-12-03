use getset::{CopyGetters, Getters};
use serde::Deserialize;

use crate::modules::llm_client::WorkingMode;

#[derive(Clone, Deserialize, CopyGetters, Getters, Debug)]
#[getset(get = "pub")]
pub struct ServerConfig {
    address: String,
    llm_mode: WorkingMode,
    allowed_languages: Vec<String>,
}
