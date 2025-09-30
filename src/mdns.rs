use anyhow::{Context, Result};
use local_ip_address::local_ip;
use log::{info, warn};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;

pub fn wait_for_local_ip() -> Result<IpAddr> {
    let mut ip: Option<IpAddr> = None;
    while ip.is_none() {
        ip = match local_ip() {
            Ok(ip) => Some(ip),
            Err(error) => {
                warn!("Failed to get the local IP address: {}, try again.", error);
                thread::sleep(Duration::from_secs(3));
                None
            }
        }
    }
    Ok(ip.unwrap())
}

pub fn publish_service(host_name: &str, port: u16) -> Result<()> {
    let service_type = "_http._tcp.local.";
    let service_name = "AirDropPro";
    info!(
        "\u{256D} Publishing service for host {:?} on port {:?}.",
        host_name, port
    );

    let ip = wait_for_local_ip().context("Failed to get local IP address")?;
    let host_name = if host_name.ends_with(".local.") {
        host_name
    } else {
        &format!("{}.local.", host_name)
    };

    let mdns = ServiceDaemon::new().context("Failed to create service daemon")?;
    let service_info = ServiceInfo::new(
        service_type,
        service_name,
        host_name,
        ip,
        port,
        HashMap::new(),
    )
    .context("Failed to create service")?;

    mdns.register(service_info)
        .context("Failed to register service")?;
    info!("\u{2570} Service has been registered successfully!");
    Ok(())
}
