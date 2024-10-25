#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::Read};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let mut conf_file = File::open("config.toml")?;
    let mut config = String::new();
    conf_file.read_to_string(&mut config)?;
    let config: threed_test::config::Config = toml::from_str(&config)?;

    threed_test::run(config).await
}
