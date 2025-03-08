use sqlx::{Pool, Sqlite};

use crate::error::{Error, Result};

pub struct BookmarkTag {
    bookmark_id: i64,
    tag_id: i64,
}

impl BookmarkTag {
    pub async fn insert(db: &Pool<Sqlite>, bookmark_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query!(
            "INSERT INTO bookmark_tags (bookmark_id, tag_id) VALUES ($1, $2)",
            bookmark_id,
            tag_id
        )
        .execute(db)
        .await
        .map_err(|e| Error::QueryException(e.to_string()))?;

        Ok(())
    }
}
