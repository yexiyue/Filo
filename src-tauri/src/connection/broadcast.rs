use std::collections::HashMap;

use crate::{device::DeviceId, Result};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_os::{hostname, platform};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub hostname: String,
    pub platform: String,
    pub os_type: String,
    pub os_version: String,
    pub os_arch: String,
    pub device_id: String,
}

impl DeviceInfo {
    pub fn new(device: &DeviceId) -> Self {
        Self {
            hostname: hostname(),
            platform: platform().to_string(),
            os_type: tauri_plugin_os::type_().to_string(),
            os_version: tauri_plugin_os::version().to_string(),
            os_arch: tauri_plugin_os::arch().to_string(),
            device_id: device.label(),
        }
    }
}

pub fn broadcast(device: &DeviceId) -> Result<ServiceDaemon> {
    let daemon = ServiceDaemon::new()?;

    let device_info = DeviceInfo::new(device);

    let value = serde_json::from_value::<HashMap<String, String>>(json!(device_info))?;

    let service_info = ServiceInfo::new(
        "_filo._tcp.local.",
        &device.label(),
        &format!("{}.local.", device.label()),
        "",
        43210,
        value,
    )?;

    info!("service info {:#?}", service_info);

    daemon.register(service_info.enable_addr_auto())?;
    
    Ok(daemon)
}
