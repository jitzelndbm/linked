use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::error::{Error, Result};

use super::{
    bookmark_tags::BookmarkTag,
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
    pub async fn insert(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        title: &str,
        url: &str,
        description: Option<&str>,
        notes: Option<&str>,
        tags: Vec<&str>,
    ) -> Result<()> {
        // Run checks on the data
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }
        if !["https://", "http://"].iter().any(|&p| url.starts_with(p)) {
            return Err(Error::InvalidUrlProvided(url.to_string()));
        }

        // Insert the bookmark
        let bookmark_id = sqlx::query!("INSERT INTO bookmarks (username, title, url, description, notes) VALUES ($1, $2, $3, $4, $5)", username, title, url, description, notes)
            .execute(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string()))?
            .last_insert_rowid();

        // Insert bookmark tags
        for title in tags {
            // Try to get the id of the tag
            let tag_id = if let Some(tag) = sqlx::query_as!(
                Tag,
                "SELECT * FROM tags WHERE title = $1 AND username = $2",
                title,
                username
            )
            .fetch_optional(db)
            .await
            .map_err(|e| Error::QueryException(e.to_string()))?
            {
                // Just return the id if it exists
                tag.id
            } else {
                // If it doesn't, insert, and return id
                Tag::insert(db, users, username, title)
                    .await?
                    .last_insert_rowid()
            };

            // Insert the relation between the bookmark and the tag
            BookmarkTag::insert(db, bookmark_id, tag_id).await?
        }

        Ok(())
    }

    //pub async fn index(db: &Pool<Sqlite>, range: Option<(i64, i64)>) -> Result<Vec<Self>> {
    //    match range {
    //        Some((begin, end)) => sqlx::query_as!(
    //            Self,
    //            "SELECT * FROM bookmarks WHERE id BETWEEN $1 AND $2",
    //            begin,
    //            end
    //        )
    //        .fetch_all(db)
    //        .await
    //        .map_err(|e| Error::QueryException(e.to_string())),
    //        None => sqlx::query_as!(Self, "SELECT * FROM bookmarks")
    //            .fetch_all(db)
    //            .await
    //            .map_err(|e| Error::QueryException(e.to_string())),
    //    }
    //}

    pub async fn index_username(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        rpp_page: (usize, usize),
    ) -> Result<Vec<Self>> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        let limit = rpp_page.0 as i64;
        let offset = (rpp_page.1 as i64 - 1) * limit;

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

    pub async fn index_username_with_filters(
        db: &Pool<Sqlite>,
        users: &Users,
        username: &Username,
        rpp_page: (usize, usize),
        tags: Vec<String>,
        words: Vec<String>,
    ) -> Result<Vec<Self>> {
        if !users.contains(&username) {
            return Err(Error::UserNotFound(username.to_string()));
        }

        // Construct list of tag id's and check if all supplied tags exist
        let existing_tags: Vec<Tag> = Tag::index_username(db, users, username).await?;
        let tag_ids: Vec<i64> = tags
            .iter()
            .map(|title| {
                existing_tags
                    .iter()
                    .find(|tag| tag.title == *title)
                    .map(|t| t.id)
                    // TODO: maybe it is not a good idea to send 400 if a single tag cannot be
                    // found, maybe silent error is better approach
                    .ok_or_else(|| Error::TagNotFound(title.to_string(), username.to_string()))
            })
            .collect::<Result<Vec<i64>>>()?;

        //let query_string = format!(
        //    r#"
        //    SELECT b.*
        //    FROM bookmarks b
        //    JOIN bookmark_tags bt ON b.id = bt.bookmark_id
        //    WHERE b.username = $1
        //    AND bt.tag_id IN ({})
        //    ORDER BY b.title
        //    LIMIT $3 OFFSET $4"#,
        //    tag_ids.iter().join(", ")
        //);

        let limit = rpp_page.0;
        let offset = (rpp_page.1 - 1) * limit;

        let mut bookmarks: Vec<Bookmark> = Vec::new();
        for id in tag_ids.iter().unique() {
            bookmarks.append(&mut sqlx::query_as!(
                Bookmark,
                "SELECT b.* FROM bookmarks b JOIN bookmark_tags bt ON b.id = bt.bookmark_id WHERE b.username = $1 AND bt.tag_id = $2 ORDER BY b.title",
                username,
                id
            ).fetch_all(db).await.map_err(|e| Error::QueryException(e.to_string()))?);
        }

        bookmarks.sort_by_key(|item| item.title.clone());

        let paginated_bookmarks: Vec<Bookmark> =
            bookmarks.into_iter().skip(offset).take(limit).collect();

        //let bookmarks: Vec<Self> = sqlx::query_as(&query_string)
        //    .bind(username)
        //    .bind(limit)
        //    .bind(offset)
        //    .fetch_all(db)
        //    .await
        //    .map_err(|e| Error::QueryException(e.to_string()))?;

        Ok(paginated_bookmarks)
    }
}
