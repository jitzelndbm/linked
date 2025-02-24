use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::users::Username;

#[derive(Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub description: String,
    pub notes: String,
}

#[derive(Default, Clone)]
pub struct BookmarkStore {
    root_dir: PathBuf,
    memory: HashMap<Username, Vec<Bookmark>>,
}

//impl BookmarkStore {
//    fn new(root_dir: PathBuf) -> Self {
//        Self { root_dir }
//    }
//
//    fn save_bookmark_to_user(&self, bookmark: username: Username) -> Result<()> {
//        Ok(())
//    }
//}
