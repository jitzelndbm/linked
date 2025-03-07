use askama::Template;
use axum::{
    extract::{FromRequestParts, Query, State},
    Extension,
};
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{appstate::AppState, bookmarks::Bookmark, tags::Tag, users::Username},
};

#[derive(Template)]
#[template(path = "bookmarks.html")]
pub struct BookmarkTemplate {
    bookmarks: Vec<Bookmark>,
    tags: [Vec<Tag>; Tag::AMOUNT],
    selected_tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, FromRequestParts)]
#[from_request(via(Query), rejection(Error))]
pub struct BookmarkQuery {
    q: Option<String>,
    page: Option<usize>,
    per_page: Option<usize>,
}

pub async fn get(
    Extension(username): Extension<Username>,
    State(ctx): State<AppState>,
    query: BookmarkQuery,
) -> Result<BookmarkTemplate> {
    dbg!(query.q);
    //let test: String = rand::rng()
    //    .sample_iter(&rand::distr::Alphanumeric)
    //    .take(7)
    //    .map(char::from)
    //    .map(|c| char::to_ascii_lowercase(&c))
    //    .collect();
    //Tag::insert(&ctx.db, &ctx.users, &username, &test).await?;

    Ok(BookmarkTemplate {
        bookmarks: Bookmark::index_username(
            &ctx.db,
            &ctx.users,
            &username,
            Some((query.per_page.unwrap_or(20), query.page.unwrap_or(1))),
        )
        .await?,
        tags: Tag::index_username_sorted(&ctx.db, &ctx.users, &username).await?,
        selected_tags: Vec::new(),
    })
}
