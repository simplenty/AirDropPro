use anyhow::{Context, Result};
use local_ip_address::local_ip;
use log::info;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

pub fn publish_service(host_name: &str, port: u16) -> Result<()> {
    let service_type = "_http._tcp.local.";
    let service_name = "AirDropPro";
    info!(
        "\u{256D} Publishing service for host {:?} on port {:?}.",
        host_name, port
    );

    let ip = local_ip().context("Failed to get local IP address")?;
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
