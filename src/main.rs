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

    let config = config::Config::new().log_and_exit("Failed to load config");

    mdns::publish_service(&config.name, config.port).log_and_exit("Failed to publish mDNS service");

    server::publish_server(config.port, config.path).log_and_exit("Failed to publish API server");

    tray::start_gui_tray().log_and_exit("Failed to start tray");
}
