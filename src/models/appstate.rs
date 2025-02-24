use sqlx::{Pool, Sqlite};

use super::users::Users;

#[derive(Clone)]
pub struct AppState {
    pub users: Users,
    pub db: Pool<Sqlite>,
}
