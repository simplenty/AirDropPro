use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use configparser::ini::Ini;
use log::info;
use crate::utils::resolve_base_directory;

pub struct Config {
    pub name: String,
    pub port: u16,
    pub path: PathBuf,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        info!("\u{256D} Loading configuration from {:?}.", path_ref);

        let mut ini = Ini::new();
        ini.load(path_ref)
            .map_err(|error| anyhow::anyhow!(error))
            .with_context(|| format!("Failed to load config file from {:?}", path_ref))?;

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
        Ok(Self {
            name,
            port,
            path,
        })
    }
}
