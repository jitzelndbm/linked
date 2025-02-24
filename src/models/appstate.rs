use super::{bookmarks::BookmarkStore, users::Users};

#[derive(Clone)]
pub struct AppState {
    pub users: Users,
    pub store: BookmarkStore,
}
