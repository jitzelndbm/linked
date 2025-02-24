use askama::Template;
use askama_axum::{IntoResponse, Response};
use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    models::{bookmarks::Bookmark, users::Username},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    bookmarks: Vec<Bookmark>,
    tags: Vec<String>,
    username: Username,
}

pub async fn get(session: Session) -> Result<Response> {
    Ok(IndexTemplate {
        username: session
            .get("username")
            .await
            .map_err(|_| Error::SessionRetrieval)?
            .ok_or(Error::SessionNotFound)?,
        bookmarks: vec![Bookmark {
            id: Uuid::new_v4(),
            title: "My Bookmark".into(),
            url: "https://google.com".into(),
            tags: vec![],
            description: "".into(),
            notes: "".into(),
        }],
        tags: Vec::new(),
    }
    .into_response())
}
