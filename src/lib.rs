use std::{path::PathBuf, str::FromStr};

use config::Config;
use models::{appstate::AppState, users::Users};
use sqlx::sqlite::SqlitePoolOptions;
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

    let db = SqlitePoolOptions::new()
        .connect(&config.db_url)
        .await
        .map_err(|e| Error::DatabaseConnect(e.to_string()))?;
    let users: Users = config.htpasswd_file.try_into()?;

    // Assemble the main router
    let app = web::router()
        .with_state(AppState { users, db })
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
