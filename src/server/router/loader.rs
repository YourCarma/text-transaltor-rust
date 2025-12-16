use std::sync::Arc;

use axum::extract::{Json, State};
use axum::response::IntoResponse;

use crate::errors::ErrorResponse;
use crate::modules::llm_client::LLMClient;
use crate::modules::loader::model_garden;
use crate::modules::loader::models::units::ModelGarden;
use crate::server::AppState;
use crate::server::errors::ServerResult;

#[utoipa::path(
    get,
    path = "/api/v1/loader/model-garden",
    tags = ["Loader"],
    description = r#"
## Getting allowed languages
 
Get all the combinations of allowed languages

Allowed languages: `["ru", "en", "fr", "uk", "ar", "de", "es", "it", "zh", "pl", "he", "ja", "tr", "pt", "ko", "cs"]`

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
