use crate::config::Config;
use std::path::Path;

mod config;

fn main() -> anyhow::Result<()> {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.build()?.block_on(real_main())?;
    Ok(())
}

async fn real_main() -> anyhow::Result<()> {
    let default_cfg = Path::new("default.kiauh.cfg");
    let current_cfg = Path::new("kiauh.cfg");

    let config = if let Ok(true) = tokio::fs::try_exists(&current_cfg).await {
        let mut config_file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(current_cfg)
            .await?;
        Config::load(&mut config_file).await?
    } else if let Ok(true) = tokio::fs::try_exists(default_cfg).await {
        let mut config_file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(default_cfg)
            .await?;
        Config::load(&mut config_file).await?
    } else {
        Config::default()
    };

    let mut current_config_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(current_cfg)
        .await?;
    config.save(&mut current_config_file).await?;

    println!("Hello, world!");
    Ok(())
}