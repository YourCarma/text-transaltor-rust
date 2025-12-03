use std::sync::Arc;

use axum::extract::{Json, State};
use axum::response::IntoResponse;
use config::Config;

use crate::errors::ErrorResponse;
use crate::modules::loader::model_garden;
use crate::modules::loader::models::units::ModelGarden;
use crate::server::AppState;
use crate::server::config::ServerConfig;
use crate::server::errors::{ServerError, ServerResult};
use crate::server::router::models::{TextTransaltorRequest, TextTransaltorResponse};
use crate::modules::llm_client::LLMClient;
use crate::modules::llm_client::models::TranslateTask;

fn check_translate_is_available(transalte_task: &TranslateTask, available_languages: Vec<String>) -> bool{

    let source_language = transalte_task.source_language();
    let target_language = transalte_task.target_language();
    if !available_languages.contains(&source_language) || !available_languages.contains(&target_language){
        tracing::error!("Language not supported!");
        return false
    } 
    true
}
#[utoipa::path(
    get,
    path = "/api/v1/loader/model-garden",
    tags = ["Loader"],
    description = r#"
## Getting allowed languages
 
Get all the combinations of allowed languages

Allowed languages: `["ru", "en", "fr", "uk", "ar", "de", "es", "it", "zh", "pl"]`

"#,
    responses(
        (status = 200, description="Combinations of languages with formalized data on RUS", body= ModelGarden),
        (status = 500, description="### Internal Server error", body = ErrorResponse)
    )
)]
pub async fn get_available_languages<R>(
    State(state): State<Arc<AppState<R>>>,
) -> ServerResult<impl IntoResponse>
where
    R: LLMClient + Send + Sync + ?Sized,
{   
    let config = state.config.clone();
    let allowed_combinations = model_garden(&config).await?;
    Ok(Json(allowed_combinations))
}