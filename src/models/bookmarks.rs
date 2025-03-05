use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::error::{Error, Result};

use super::{
    tags::Tag,
    users::{Username, Users},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    id: i64,
    username: Username,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}

impl Bookmark {
    /// This function accepts form data, and checks it
    pub async fn insert(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        title: &str,
        url: &str,
        description: Option<&str>,
        notes: Option<&str>,
        tags: &str,
    ) -> Result<()> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        if !["https://", "http://"].iter().any(|&p| url.starts_with(p)) {
            return Err(Error::InvalidUrlProvided(url.to_string()));
        }

        sqlx::query!("INSERT INTO bookmarks (username, title, url, description, notes) VALUES ($1, $2, $3, $4, $5)", username, title, url, description, notes)
            .execute(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string()))?;

        let existing_tags: Vec<String> = Tag::index_username(db, &users, username)
            .await?
            .iter()
            .map(|t| t.title.clone())
            .collect();

        for title in tags
            .split(' ')
            .filter(|tag| !tag.is_empty() && !existing_tags.contains(&tag.to_string()))
        {
            Tag::insert(db, users, username, title).await?;
        }

        Ok(())
    }

    pub async fn index(db: &Pool<Sqlite>, range: Option<(i64, i64)>) -> Result<Vec<Self>> {
        match range {
            Some((begin, end)) => sqlx::query_as!(
                Self,
                "SELECT * FROM bookmarks WHERE id BETWEEN $1 AND $2",
                begin,
                end
            )
            .fetch_all(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string())),
            None => sqlx::query_as!(Self, "SELECT * FROM bookmarks")
                .fetch_all(db)
                .await
                .map_err(|e| Error::QueryException(e.to_string())),
        }
    }

    pub async fn index_username(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        rpp_page: Option<(usize, usize)>,
    ) -> Result<Vec<Self>> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        match rpp_page {
            Some((records_per_page, page_number)) => {
                let limit = records_per_page as i64;
                let offset = (page_number as i64 - 1) * limit;

                sqlx::query_as!(
                    Self,
                    "SELECT * FROM bookmarks WHERE username = $1 ORDER BY id LIMIT $2 OFFSET $3",
                    username,
                    limit,
                    offset
                )
                .fetch_all(db)
                .await
                .map_err(|e| Error::QueryException(e.to_string()))
            }
            None => sqlx::query_as!(
                Self,
                "SELECT * FROM bookmarks WHERE username = $1",
                username
            )
            .fetch_all(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string())),
        }
    }
}
