use std::{fs::create_dir_all, path::PathBuf, str::FromStr};

use config::Config;
use models::{appstate::AppState, bookmarks::BookmarkStore};
use tokio::net::TcpListener;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{info, level_filters::LevelFilter};

use crate::error::Error;

mod config;
pub mod error;
mod models;
mod web;

pub async fn run(config_path: PathBuf) -> Result<(), Error> {
    // Load the config
    let config: Config = confy::load_path(config_path).map_err(|_| Error::ConfigError)?;

    // Setup logging
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(
            LevelFilter::from_str(&config.log_level).map_err(|_| Error::ConfigLogFormat)?,
        )
        .init();

    // Create a session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    // Create the store directory
    if config.storage_dir.starts_with("~") {
        create_dir_all(format!(
            "{}/{}",
            std::env::var("HOME").map_err(|_| Error::HomeEnvNotFound)?,
            config.storage_dir.to_str().expect("Path not valid unicode")
        ))
    } else {
        info!("creating {:?}", config.storage_dir);
        create_dir_all(config.storage_dir)
    }
    .map_err(|e| Error::CouldNotCreateStoreDir(e.to_string()))?;

    // Assemble the main router
    let app = web::router()
        .with_state(AppState {
            users: config.htpasswd_file.try_into()?,
            store: BookmarkStore::default(),
        })
        .layer(session_layer);

    // Bind the router
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .map_err(|_| Error::BindError)?;
    info!("serving app on {}:{}", config.host, config.port);

    axum::serve(listener, app)
        .await
        .map_err(|_| Error::BindError)?;

    Ok(())
}
