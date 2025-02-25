use axum::{
    extract::Request,
    middleware::{self, Next},
    response::{Redirect, Response},
    routing::get,
    Extension, Router,
};
use tower_sessions::Session;

use crate::{
    error::{Error, Result},
    models::{appstate::AppState, users::Username},
};

mod bookmarks;
mod index;
mod login;
mod new;
mod signout;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/bookmarks", get(bookmarks::get))
        .route("/bookmarks/new", get(new::get).post(new::post))
        .layer(middleware::from_fn(require_auth))
        .route("/", get(index::get))
        .route("/login", get(login::get).post(login::post))
        .layer(middleware::from_fn(insert_username))
        .route("/signout", get(signout))
}

async fn signout(session: Session) -> Redirect {
    session.clear().await;
    let _ = session.delete().await;
    Redirect::to("/login")
}

async fn insert_username(session: Session, mut request: Request, next: Next) -> Result<Response> {
    let username: Option<String> = session
        .get("username")
        .await
        .map_err(|_| Error::SessionRetrieval)?
        .unwrap_or(None);

    request.extensions_mut().insert(username);
    Ok(next.run(request).await)
}

async fn require_auth(
    Extension(username): Extension<Option<Username>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let username = username.ok_or(Error::SessionNotFound)?;
    request.extensions_mut().insert(username);
    Ok(next.run(request).await)
}
