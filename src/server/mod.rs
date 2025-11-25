pub mod config;
pub mod errors;
// pub mod router;
// pub mod swagger;

// use std::sync::Arc;

// use axum::Router;
// use axum::response::Html;
// use axum::routing::{get, post};
// use axum_prometheus::PrometheusMetricLayer;
// use swagger::ApiDoc;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

// use crate::modules::llm_client::LLMClient;

// pub struct AppState<R>
// where
//     R: LLMClient,
// {
//     llm_client: Arc<R>,
// }

// impl<R> AppState<R>
// where
//     R: LLMClient,
// {
//     pub fn new(llm_client: Arc<R>) -> Self {
//         AppState { llm_client: llm_client }
//     }
// }

// pub fn init_server<R>(app: AppState<R>) -> Router
// where
//     R: LLMClient + Send + Sync + 'static,
// {
//     let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

//     let app_arc = Arc::new(app);
//     Router::new()
//         .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
//         .route("/", get(Html("<a href=\"/docs\">ДОКУМЕНТАЦИЯ</h1>")))
//         .route(
//             "/api/v1/images/generate/file",
//             post(router::llm_client::generate_image_to_file)
//         )
//         .route(
//             "/api/v1/images/generate/url",
//             post(router::llm_client::generate_image_to_url)
//         )
//         .route("/metrics", get(|| async move { metric_handle.render() }))
//         .layer(prometheus_layer)
//         .with_state(app_arc)
// }

