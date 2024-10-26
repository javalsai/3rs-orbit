// Wasm would use directly the lib, this is by nature `not(wasm)`

pub mod args;

use std::{fs::File, io::Read};
use args::Args;
use clap::Parser;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut conf_file = File::open(args.config)?;
    let mut config = String::new();
    conf_file.read_to_string(&mut config)?;
    let config: threed_test::config::Config = toml::from_str(&config)?;

    threed_test::run(config).await
}
