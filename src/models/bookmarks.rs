use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::error::{Error, Result};

use super::users::{Username, Users};

#[derive(Clone, Serialize, Deserialize)]
pub struct Bookmark {
    id: i64,
    username: Username,
    title: String,
    url: String,
    description: Option<String>,
    notes: Option<String>,
}

impl Bookmark {
    pub async fn insert(
        db: &Pool<Sqlite>,
        users: Users,
        username: Username,
        title: String,
        url: String,
        description: Option<String>,
        notes: Option<String>,
    ) -> Result<()> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username));
        }

        if !["https://", "http://"].iter().any(|&p| url.starts_with(p)) {
            return Err(Error::InavlidUrlProvided(url));
        }

        sqlx::query!("INSERT INTO bookmarks (username, title, url, description, notes) VALUES ($0, $1, $2, $3, $4)", username, title, url, description, notes)
            .execute(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string()))?;

        Ok(())
    }

    pub async fn index(db: &Pool<Sqlite>, range: Option<(i64, i64)>) -> Result<Vec<Self>> {
        match range {
            Some((begin, end)) => sqlx::query_as!(
                Self,
                "SELECT * FROM bookmarks WHERE id BETWEEN $0 AND $1",
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

    pub async fn index_by_user(
        db: &Pool<Sqlite>,
        username: Username,
        rpp_page: Option<(usize, usize)>,
    ) -> Result<Vec<Self>> {
        match rpp_page {
            Some((records_per_page, page_number)) => {
                let limit = records_per_page as i64;
                let offset = page_number as i64 * limit;

                sqlx::query_as!(
                    Self,
                    "SELECT * FROM bookmarks WHERE username = $0 ORDER BY id LIMIT $1 OFFSET $2",
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
                "SELECT * FROM bookmarks WHERE username = $0",
                username
            )
            .fetch_all(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string())),
        }
    }
}
