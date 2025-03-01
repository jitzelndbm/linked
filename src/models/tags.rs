use itertools::Itertools;
use sqlx::{Pool, Sqlite};

use crate::error::{Error, Result};

use super::users::{Username, Users};

#[derive(Debug)]
pub struct Tag {
    id: i64,
    username: String,
    pub title: String,
}

impl Tag {
    pub async fn insert(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        title: &str,
    ) -> Result<()> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        sqlx::query!(
            "INSERT INTO tags (username, title) VALUES ($1, $2)",
            username,
            title
        )
        .execute(db)
        .await
        .map_err(|e| Error::QueryException(e.to_string()))?;

        Ok(())
    }

    pub async fn index_sorted(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
    ) -> Result<Vec<Vec<Tag>>> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        let tags: Vec<Tag> = sqlx::query_as!(
            Tag,
            "SELECT * FROM tags WHERE username = $1 ORDER BY username",
            username
        )
        .fetch_all(db)
        .await
        .map_err(|e| Error::QueryException(e.to_string()))?;

        let sorted_tags: Vec<Vec<Tag>> = tags
            .into_iter()
            .chunk_by(|tag| {
                tag.title
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase())
                    .unwrap_or('\0')
            })
            .into_iter()
            .map(|(_initial, group)| group.collect())
            .collect();

        Ok(sorted_tags)
    }
}
