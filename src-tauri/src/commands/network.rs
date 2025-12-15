use crate::{network, Result};
use libp2p::{identity::Keypair, PeerId};
use tauri::{AppHandle, Manager, State};
use tracing::info;

#[tauri::command]
pub async fn start(app: AppHandle, keypair: State<'_, Keypair>) -> Result<()> {
    let client = network::start(&keypair)?;
    app.manage(client);

    info!("Started event loop");
    Ok(())
}

#[tauri::command]
pub async fn dial(client: State<'_, network::NetClient>, peer_id: PeerId) -> Result<()> {
    client.dial(peer_id).await
}

#[tauri::command]
pub async fn close(client: State<'_, network::NetClient>, peer_id: PeerId) -> Result<()> {
    client.close(peer_id).await
}
