use std::env::args;

const DEFAULT_CONFIG_PATH: &str = "/var/lib/linked/config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    linked::run(
        args()
            .nth(1)
            .unwrap_or(DEFAULT_CONFIG_PATH.to_string())
            .into(),
    )
    .await
}
