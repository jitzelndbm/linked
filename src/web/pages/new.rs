use askama::Template;
use axum::{extract::State, response::Redirect, Extension, Form};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{
    error::{Error, Result},
    models::{appstate::AppState, bookmarks::Bookmark, tags::Tag, users::Username},
};

const SESSION_NEW_ERROR_KEY: &str = "new_bookmark_error_message";
const SESSION_NEW_ERROR_MESSAGE_URL: &str = "An invalid URL was provided!";

#[derive(Debug, Deserialize, Serialize)]
pub struct NewForm {
    title: String,
    url: String,
    tags: String,
    // This field will be Some("") through default html form submisssion
    description: Option<String>,
    // This field will be Some("") through default html form submisssion
    notes: Option<String>,
}

#[derive(Template)]
#[template(path = "new.html")]
pub struct NewTemplate {
    tags: Vec<Tag>,
    message: Option<String>,
}

pub async fn get(
    session: Session,
    Extension(username): Extension<Username>,
    State(ctx): State<AppState>,
) -> Result<NewTemplate> {
    Ok(NewTemplate {
        tags: Tag::index_username(&ctx.db, &ctx.users, &username).await?,
        message: session
            .remove(SESSION_NEW_ERROR_KEY)
            .await
            .map_err(|_| Error::SessionRemoval)?,
    })
}

pub async fn post(
    session: Session,
    Extension(username): Extension<Username>,
    State(ctx): State<AppState>,
    Form(form): Form<NewForm>,
) -> Result<Redirect> {
    match Bookmark::insert(
        &ctx.db,
        &ctx.users,
        &username,
        &form.title,
        &form.url,
        form.description.as_deref(),
        form.notes.as_deref(),
        &form.tags,
    )
    .await
    {
        Ok(_) => Ok(Redirect::to("/bookmarks")),
        Err(e) => match e {
            Error::InvalidUrlProvided(_) => {
                session
                    .insert(SESSION_NEW_ERROR_KEY, SESSION_NEW_ERROR_MESSAGE_URL)
                    .await
                    .map_err(|_| Error::SessionRetrieval)?;

                Ok(Redirect::to("/bookmarks/new"))
            }
            _ => Err(e),
        },
    }
}
