use crate::logger::LogAndExit;

mod config;
mod logger;
mod utils;

fn main() {
    logger::initialize("app.log");

    let config = config::Config::from_file("config.ini").log_and_exit("Failed to load config");
}
