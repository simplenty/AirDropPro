#![windows_subsystem = "windows"]

use crate::logger::LogAndExit;

mod config;
mod logger;
mod mdns;
mod server;
mod tray;
mod utils;

fn main() {
    logger::initialize();

    utils::ensure_single_instance("AirDropPro").log_and_exit("Failed to ensure single instance");

    config::Config::init().log_and_exit("Failed to load config");

    mdns::publish_service().log_and_exit("Failed to publish mDNS service");

    server::publish_server().log_and_exit("Failed to publish API server");

    tray::start_gui_tray().log_and_exit("Failed to start tray");
}
