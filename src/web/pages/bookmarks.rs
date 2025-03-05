use askama::Template;
use axum::{extract::State, Extension};
use rand::Rng;

use crate::{
    error::Result,
    models::{appstate::AppState, bookmarks::Bookmark, tags::Tag, users::Username},
};

#[derive(Template)]
#[template(path = "bookmarks.html")]
pub struct BookmarkTemplate {
    bookmarks: Vec<Bookmark>,
    tags: [Vec<Tag>; Tag::AMOUNT],
}

// TODO: process search query
pub async fn get(
    Extension(username): Extension<Username>,
    State(ctx): State<AppState>,
) -> Result<BookmarkTemplate> {
    let test: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(7)
        .map(char::from)
        .map(|c| char::to_ascii_lowercase(&c))
        .collect();
    Tag::insert(&ctx.db, &ctx.users, &username, &test).await?;
    Ok(BookmarkTemplate {
        bookmarks: Bookmark::index_username(&ctx.db, &ctx.users, &username, Some((20, 1))).await?,
        tags: Tag::index_username_sorted(&ctx.db, &ctx.users, &username).await?,
    })
}
