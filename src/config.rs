use crate::utils::{get_config_path, resolve_base_directory, set_auto_startup};
use anyhow::{Context, Result, ensure};
use configparser::ini::Ini;
use log::info;
use std::fs::write;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub name: String,
    pub port: u16,
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = get_config_path().context("Failed to get config path")?;
        info!(
            "\u{256D} Loading config on path {:?}.",
            config_path.display()
        );
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

        let auto_launch = ini
            .get("Application", "auto launch")
            .context("Config missing 'auto launch' key in [Application] section")?;
        let auto_launch = match auto_launch.as_str() {
            "true" | "t" | "yes" | "y" | "1" | "on" => Some(true),
            "false" | "f" | "no" | "n" | "0" | "off" => Some(false),
            _ => None,
        }
        .context("Failed to parse 'auto launch' key")?;
        set_auto_startup(auto_launch).context("Failed to set auto launch")?;

        info!("\u{2570} Configuration loaded successfully!");
        Ok(Self { name, port, path })
    }

    pub fn init() -> Result<()> {
        let config = Self::new()?;
        CONFIG
            .set(config)
            .map_err(|_| anyhow::anyhow!("Config already initialized"))?;

        ensure!(
            CONFIG.get().is_some(),
            "Failed to ensure the CONFIG isn't None"
        );

        Ok(())
    }

    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }
}
