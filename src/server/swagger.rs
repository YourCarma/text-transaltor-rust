use crate::errors::*;
use crate::server::router::llm_client::*;
use crate::server::router::loader::*;
use crate::server::router::models::{
    ModelGardenResponse, TextTransaltorRequest, TextTransaltorResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title="Text Translator",
        version="0.9.0",
        description = "Service for text transalting with LLM"
    ),
    tags(
        (
            name = "Translator",
            description = "### Transalting text with LLM.",
        ),
        (
            name = "Loader",
            description = "### Get formalized available language translation combinations",
        ),
    ),

    components(
        schemas(
            TextTransaltorRequest,
            TextTransaltorResponse,
            ModelGardenResponse,
            Successful,
            ErrorResponse,
        ),
    ),
    paths(
    get_available_languages,
    translate_text,
    )
)]
pub(super) struct ApiDoc;

pub trait SwaggerExample {
    type Example;

    fn example(value: Option<&str>) -> Self::Example;
}

impl SwaggerExample for Successful {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("Done");
        Successful::new(200, msg)
    }
}

impl SwaggerExample for ErrorResponse {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("bad client request");
        ErrorResponse::new(400, "Bad request", msg)
    }
}
