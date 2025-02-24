//use axum::{handler::HandlerWithoutStateExt, Router};
//use tower_http::services::ServeDir;
//
//use crate::{error, model::AppState};
//
//pub fn router(asset_dir: &str) -> Router<AppState> {
//    Router::<AppState>::new().nest_service(
//        "/",
//        ServeDir::new(asset_dir).not_found_service(error::not_found_handler.into_service()),
//    )
//}
