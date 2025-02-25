use axum::{
    http::header,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{error, models::appstate::AppState};

/// This corresponds to the /api routes
//mod api;

/// This module contains all the pages that are normally visited by users
mod pages;

/// Returns the main router, that is served in `lib.rs`
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(pages::router())
        .route("/styles.css", get(css_provide))
        .fallback(error::not_found_handler)

    //.nest("/api", api::router())
}

pub async fn css_provide() -> Response {
    (
        [(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/css"),
        )],
        include_str!("../../assets/styles.css"),
    )
        .into_response()
}
