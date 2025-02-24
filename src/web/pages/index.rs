use askama::Template;
use askama_axum::{IntoResponse, Response};
use tower_sessions::Session;

use crate::{
    error::{Error, Result},
    models::{bookmarks::Bookmark, users::Username},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    username: Username,
}

pub async fn get(session: Session) -> Result<Response> {
    Ok(IndexTemplate {
        username: session
            .get("username")
            .await
            .map_err(|_| Error::SessionRetrieval)?
            .ok_or(Error::SessionNotFound)?,
    }
    .into_response())
}
