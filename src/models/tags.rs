use std::array;

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
    pub const AMOUNT: usize = 27;

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

    pub async fn index_username(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
    ) -> Result<Vec<Tag>> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        let tags: Vec<Tag> = sqlx::query_as!(
            Tag,
            "SELECT * FROM tags WHERE username = $1 ORDER BY title",
            username
        )
        .fetch_all(db)
        .await
        .map_err(|e| Error::QueryException(e.to_string()))?;

        Ok(tags)
    }

    pub async fn index_username_sorted(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
    ) -> Result<[Vec<Tag>; Self::AMOUNT]> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        let tags: Vec<Tag> = sqlx::query_as!(
            Tag,
            "SELECT * FROM tags WHERE username = $1 ORDER BY title",
            username
        )
        .fetch_all(db)
        .await
        .map_err(|e| Error::QueryException(e.to_string()))?;

        let sorted_tags = tags
            .into_iter()
            .fold(array::from_fn(|_| Vec::new()), |mut acc, tag| {
                let index = tag
                    .title
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase())
                    .filter(|c| c.is_ascii_alphabetic())
                    .map(|c| (c as u8 - b'a' + 1) as usize)
                    .unwrap_or(0);

                acc[index].push(tag);
                acc
            });

        Ok(sorted_tags)
    }
}
