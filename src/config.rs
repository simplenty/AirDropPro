use crate::utils::{get_config_path, resolve_base_directory};
use anyhow::{Context, Result};
use configparser::ini::Ini;
use log::info;
use std::fs::write;
use std::path::PathBuf;

pub struct Config {
    pub name: String,
    pub port: u16,
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = get_config_path().context("Failed to get config path")?;
        info!("\u{256D} Loading config on path {:?}.", config_path.display());
        if !config_path.exists() {
            info!("Failed to: {:?}", config_path);
            const DEFAULT_CONFIG_BYTES: &[u8] = include_bytes!("../config.ini");
            let default_config_str = str::from_utf8(DEFAULT_CONFIG_BYTES)
                .context("Failed to decode default config string with UTF-8")?;
            write(&config_path, default_config_str).with_context(|| {
                format!(
                    "Failed to write the default config into path {:?}",
                    config_path
                )
            })?;
        }

        let mut ini = Ini::new();
        ini.load(&config_path)
            .map_err(|error| anyhow::anyhow!(error))
            .with_context(|| format!("Failed to load config file from {:?}", config_path))?;

        let name = ini
            .get("Server", "name")
            .context("Config missing 'name' key in [Server] section")?;

        let port = ini
            .get("Server", "port")
            .context("Config missing 'port' key in [Server] section")?;
        let port = port.parse::<u16>().with_context(|| {
            format!(
                "Failed to parse 'port' value '{}' as a number between 0 and 65535",
                port
            )
        })?;

        let path = ini
            .get("Application", "download path")
            .context("Config missing 'path' key in [Application] section")?;
        let path = resolve_base_directory(&path).context("Failed to generate download path")?;

        info!("\u{2570} Configuration loaded successfully!");
        Ok(Self { name, port, path })
    }
}
