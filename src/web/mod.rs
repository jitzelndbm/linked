use axum::Router;

use crate::models::appstate::AppState;

// /// This corresponds to the /api routes
//mod api;

/// This module serves files from the /assets folder
mod assets;

/// This module contains all the pages that are normally visited by users
mod pages;

/// Returns the main router, that is served in `lib.rs`
pub fn router() -> Router<AppState> {
    Router::<AppState>::new().merge(pages::router())
    //.nest("/assets", assets::router())

    //.nest("/api", api::router())
    //.fallback(error::not_found_handler)
}
