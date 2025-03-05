use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, response::Redirect, Extension, Form};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    error::{Error, Result},
    models::{appstate::AppState, users::Username},
};

const SESSION_LOGIN_ERROR_KEY: &str = "display_login_error_message";

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    display_error_message: bool,
}
pub async fn get(
    session: Session,
    Extension(username): Extension<Option<Username>>,
) -> Result<Response> {
    Ok(if username.is_some() {
        Redirect::to("/bookmarks").into_response()
    } else {
        LoginTemplate {
            display_error_message: session
                .remove(SESSION_LOGIN_ERROR_KEY)
                .await
                .map_err(|_| Error::SessionRemoval)?
                .unwrap_or(false),
        }
        .into_response()
    })
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn post(
    session: Session,
    State(state): State<AppState>,
    Extension(username): Extension<Option<Username>>,
    Form(form_data): Form<LoginForm>,
) -> Result<Redirect> {
    // check if user is not logged in yet
    if username.is_some() {
        return Ok(Redirect::to("/bookmarks"));
    }

    if !state
        .users
        .verify(&form_data.username, form_data.password)
        .unwrap_or(false)
    {
        session
            .insert(SESSION_LOGIN_ERROR_KEY, true)
            .await
            .map_err(|_| Error::SessionRetrieval)?;

        return Ok(Redirect::to("/login"));
    }

    session
        .insert("username", form_data.username)
        .await
        .map_err(|_| Error::SessionRetrieval)?;

    Ok(Redirect::to("/"))
}
