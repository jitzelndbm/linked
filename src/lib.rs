use std::{error::Error, path::PathBuf, str::FromStr};

use axum::{routing::get, Router};
use config::Config;
use tracing::{info, level_filters::LevelFilter};

mod config;
mod controllers;
mod models;

pub async fn run(config_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let config: Config = confy::load_path(config_path)?;

    // Setup logging
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(LevelFilter::from_str(&config.log_level)?)
        .init();

    // Establish database connection
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await?;

    // Migrate the database
    info!("Running db migrations...");
    sqlx::migrate!().run(&pool).await?;

    // Assemble the router
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Bind the router
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    info!("serving app on {}:{}", config.host, config.port);
    axum::serve(listener, app).await?;

    Ok(())
}
