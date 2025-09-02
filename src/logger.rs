use anyhow::{Error, Result};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::OpenOptions;
use std::path::Path;
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

pub fn initialize<P: AsRef<Path>>(path: P) {
    let log_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Failed to open or create log file");

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)
        .expect("Failed to initialize logger");

    log::info!("\u{25CF} Logger has been initialized successfully!");
}
