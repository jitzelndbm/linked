use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub htpasswd_file: PathBuf,
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 5005,
            log_level: "info".into(),
            htpasswd_file: "/var/lib/linked/htpasswd".into(),
            db_url: "sqlite:///var/lib/linked/linked.db".into(),
        }
    }
}
