use std::{env::args, io::stderr};

use linked::error::default_error_handler;

const DEFAULT_CONFIG_PATH: &str = "/var/lib/linked/config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    match linked::run(
        args()
            .nth(1)
            .unwrap_or(DEFAULT_CONFIG_PATH.to_string())
            .into(),
    )
    .await
    {
        Ok(()) => {}
        Err(error) => default_error_handler(&error, &mut stderr().lock()),
    };

    Ok(())
}
