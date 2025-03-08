use axum::Router;

use crate::{error, models::appstate::AppState};

/// This corresponds to the /api routes
//mod api;

/// This module contains all the pages that are normally visited by users
mod pages;

/// Returns the main router, that is served in `lib.rs`
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(pages::router())
        .fallback(error::not_found_handler)
    //.nest("/api", api::router())
}
