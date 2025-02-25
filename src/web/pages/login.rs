use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, response::Redirect, Form};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{error::{Error, Result}, models::appstate::AppState};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;
pub async fn get() -> LoginTemplate {
    LoginTemplate
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn post(
    session: Session,
    State(state): State<AppState>,
    Form(form_data): Form<LoginForm>,
) -> Result<Response> {
    Ok(if state.users.verify(&form_data.username, form_data.password)? {
        session.insert("username", form_data.username).await.map_err(|_| Error::SessionAdding)?;
        Redirect::to("/").into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    })
}
