use crate::logger::LogAndExit;

mod config;
mod logger;
mod mdns;
mod server;
mod utils;

fn main() {
    logger::initialize("app.log");

    let config = config::Config::from_file("config.ini").log_and_exit("Failed to load config");

    mdns::publish_service(&config.name, config.port).log_and_exit("Failed to publish mDNS service");

    server::publish_server(config.port, config.path).log_and_exit("Failed to publish API server");
}
