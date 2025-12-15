use libp2p::{identity::Keypair, PeerId};
use tauri::{AppHandle, Manager};

use crate::Result;

#[tauri::command]
pub async fn generate_keypair() -> Result<Vec<u8>> {
    Ok(Keypair::generate_ed25519().to_protobuf_encoding()?)
}

#[tauri::command]
pub async fn register_keypair(app: AppHandle, keypair: Vec<u8>) -> Result<PeerId> {
    let keypair = Keypair::from_protobuf_encoding(&keypair)?;
    let peer_id = keypair.public().to_peer_id();

    app.manage(keypair);
    Ok(peer_id)
}
