use anyhow::{Error, Result};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::{OpenOptions, create_dir_all};
use std::process;

pub trait LogAndExit<T> {
    fn log_and_exit(self, prompt: &str) -> T;
}

impl<T> LogAndExit<T> for Result<T, Error> {
    fn log_and_exit(self, prompt: &str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                log::error!("{:?} : {:?}", prompt, error);
                process::exit(1);
            }
        }
    }
}

pub fn initialize() {
    let mut logger_path = dirs::config_dir().expect("Failed to get standard config directory");
    logger_path.push("AirDropPro");
    create_dir_all(&logger_path)
        .expect(format!("Failed to create config path: {:?}", logger_path).as_str());
    logger_path.push("app.log");

    let log_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(logger_path)
        .expect("Failed to open or create log file");

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)
        .expect("Failed to initialize logger");

    log::info!("\u{25CF} Logger has been initialized successfully!");
}
