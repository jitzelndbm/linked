use askama::Template;
use axum::{extract::State, Extension};

use crate::{
    error::Result,
    models::{appstate::AppState, bookmarks::Bookmark, users::Username},
};

#[derive(Template)]
#[template(path = "bookmarks.html")]
pub struct BookmarkTemplate {
    username: Username,
    bookmarks: Vec<Bookmark>,
}

pub async fn get(
    Extension(username): Extension<Username>,
    State(ctx): State<AppState>,
) -> Result<BookmarkTemplate> {
    Ok(BookmarkTemplate {
        username: username.clone(),
        bookmarks: Bookmark::index_by_user(&ctx.db, &username, Some((20, 1))).await?,
    })
}
