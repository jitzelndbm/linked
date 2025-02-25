use axum::response::Redirect;
use tower_sessions::Session;

/// If the user is authenicated, redirect to /bookmark, otherwise redirect to /login
pub async fn get(session: Session) -> Redirect {
    if session
        .get("username")
        .await
        .is_ok_and(|ok: Option<String>| ok.is_some())
    {
        Redirect::to("/bookmarks")
    } else {
        Redirect::to("/login")
    }
}
