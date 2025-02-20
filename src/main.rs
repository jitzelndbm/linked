use std::error::Error;

use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
struct User {
    _id: i64,
    _username: String,
    _password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(env!("DATABASE_URL"))
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"select Id as "_id", Username as "_username", Password as "_password" from Users"#
    )
    .fetch_all(&pool)
    .await?;

    dbg!(users);

    Ok(())
}
