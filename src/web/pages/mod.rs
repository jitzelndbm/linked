use axum::{routing::get, Router};

use crate::models::appstate::AppState;

mod index;
mod login;
mod new;
mod signout;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", get(index::get))
        .route("/login", get(login::get).post(login::post))
    //.route("/signout", get(|| async { "Hello" }))
}
