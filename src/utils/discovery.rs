#[cfg(feature = "discovery")]
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use anyhow::Result;

/// Discovered AWTRIX3 device
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub txt_records: std::collections::HashMap<String, String>,
}

/// Discover AWTRIX3 devices on the network
#[cfg(feature = "discovery")]
pub async fn discover_devices(timeout: Duration) -> Result<Vec<DiscoveredDevice>> {
    let mdns = ServiceDaemon::new()?;
    let service_type = "_http._tcp.local.";
    let receiver = mdns.browse(service_type)?;
    
    let mut devices = Vec::new();
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
            if let ServiceEvent::ServiceResolved(info) = event {
                // Check if this is an AWTRIX device by looking at TXT records or hostname
                if is_awtrix_device(&info) {
                    let device = DiscoveredDevice {
                        name: info.get_hostname().to_string(),
                        host: info.get_addresses().iter().next()
                            .map(|addr| addr.to_string())
                            .unwrap_or_default(),
                        port: info.get_port(),
                        txt_records: std::collections::HashMap::new(), // TODO: Fix TXT records parsing
                    };
                    devices.push(device);
                }
            }
        }
    }
    
    Ok(devices)
}

#[cfg(feature = "discovery")]
fn is_awtrix_device(info: &mdns_sd::ServiceInfo) -> bool {
    // Check for AWTRIX-specific indicators
    let hostname = info.get_hostname().to_lowercase();
    
    // Check hostname patterns
    if hostname.contains("awtrix") || hostname.contains("ulanzi") {
        return true;
    }
    
    // TODO: Check TXT records for AWTRIX-specific keys when API is fixed
    // For now, just use hostname matching
    
    false
}

/// Discover devices (stub for when discovery feature is disabled)
#[cfg(not(feature = "discovery"))]
pub async fn discover_devices(_timeout: Duration) -> Result<Vec<DiscoveredDevice>> {
    Err(anyhow::anyhow!(
        "Device discovery is not available. Compile with --features discovery"
    ))
}