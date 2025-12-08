use mdns_sd::ServiceEvent;
use serde_json::json;
use tauri::{ipc::Channel, AppHandle, Manager};
use tracing::{info, instrument};

use crate::{
    connection::{self, DeviceInfo},
    device::DeviceId,
    identify::Identify,
    Result,
};

#[tauri::command]
pub async fn register(app: AppHandle) -> Result<()> {
    let path = app.path().home_dir()?.join(".filo").join("identify.pem");

    let identify = Identify::load(path)?;
    let device = DeviceId::from(identify.signing_key.verifying_key());

    connection::broadcast(&device)?;
    info!("Registered device {:#?}", device);
    Ok(())
}

#[instrument(skip(on_found))]
#[tauri::command]
pub async fn discover(on_found: Channel<DeviceInfo>) -> Result<()> {
    let (_, mut rx) = connection::discover()?;

    tokio::spawn(async move {
        while let Some(e) = rx.recv().await {
            match e {
                ServiceEvent::ServiceResolved(resolved_service) => {
                    let map = resolved_service
                        .get_properties()
                        .clone()
                        .into_property_map_str();

                    let device_info: DeviceInfo = serde_json::from_value(json!(map)).unwrap();

                    on_found.send(device_info).unwrap();
                }
                _ => {
                    info!("{:#?}", e);
                }
            }
        }
    });

    Ok(())
}
