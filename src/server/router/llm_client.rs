use std::sync::Arc;

use axum::extract::{Json, State};
use axum::response::IntoResponse;
use config::Config;

use crate::errors::ErrorResponse;
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
    post,
    path = "/api/v1/translate/text",
    request_body = TranslateTask,
    tags = ["Translator"],
    description = r#"
## Translate text

Translate text to following language using LLM

### Arguments
- `source_language` (string, ISO-639): Source language of text
- `target_language` (string, ISO-639): Target language of text.
- `text` (string): Text to translate

"#,
    responses(
        (status = 200, description="Transalted Data", body = TextTransaltorResponse),
        (status = 204, description="### IO error. Maybe error on saving or reading file", body = ErrorResponse),
        (status = 400, description="### Bad request to API", body = ErrorResponse),
        (status = 401, description="### Unauth user on target API", body = ErrorResponse),
        (status = 402, description="### No credits on target API", body = ErrorResponse),
        (status = 403, description="### Model in target API is on moderation",body = ErrorResponse),
        (status = 408, description="### Timeout on target API", body = ErrorResponse),
        (status = 429, description="### Too many requests", body = ErrorResponse),
        (status = 500, description="### Internal Server error", body = ErrorResponse),
        (status = 502, description="### Deserialization Error. Response of model has no `image` field", body = ErrorResponse),
        (status = 503, description="### Provider of target API is not available", body = ErrorResponse),
    )
)]
pub async fn translate_text<R>(
    State(state): State<Arc<AppState<R>>>,
    Json(transalte_body): Json<TextTransaltorRequest>,
) -> ServerResult<impl IntoResponse>
where
    R: LLMClient + Send + Sync + ?Sized,
{
    let task = transalte_body.translate_task().to_owned();
    let available_languages = state.config.server().allowed_languages().to_owned();
    if !check_translate_is_available(&task, available_languages){
        return  Err(ServerError::UnsupportedLanguage(format!("Указанный язык не поддерживается")))
    }
    
    let translated_text = state.llm_client.translate(task).await?;
    let translated_response = TextTransaltorResponse::new(translated_text);
    Ok(Json(translated_response))
}