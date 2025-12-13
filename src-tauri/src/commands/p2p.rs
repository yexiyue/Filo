use base32::Alphabet;
use libp2p::{futures::StreamExt, identity::Keypair};
use tracing::info;

use crate::{p2p::FiloSwarm, Result};

#[tauri::command]
pub async fn start() -> Result<String> {
    let keypair = Keypair::generate_ed25519();
    let keypair_str = base32::encode(
        Alphabet::Crockford,
        &keypair.clone().try_into_ed25519().unwrap().to_bytes(),
    );
    let mut filo_swarm = FiloSwarm::new(keypair).await?;

    let _: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        filo_swarm.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            let event = filo_swarm.swarm.select_next_some().await;
            info!("event: {:?}", event)
        }
    });

    Ok(keypair_str)
}
