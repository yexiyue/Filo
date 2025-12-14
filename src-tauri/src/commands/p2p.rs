use base32::Alphabet;
use libp2p::{futures::StreamExt, identity::Keypair, mdns, swarm::SwarmEvent};
use tracing::info;

use crate::{
    network::{FiloBehaviourEvent, FiloSwarm},
    Result,
};

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
            // info!("event: {:?}", event);

            match event {
                SwarmEvent::NewExternalAddrOfPeer { peer_id, address } => {
                    info!("NewExternalAddrOfPeer: {:?}", address);
                }
                SwarmEvent::Behaviour(behaviour) => match behaviour {
                    FiloBehaviourEvent::Mdns(e) => {
                        info!("Mdns: {:#?}", e);
                        match e {
                            mdns::Event::Discovered(list) => {
                                for (peer_id, multiaddr) in list {
                                    info!("Discovered: {:?}", multiaddr);
                                    // filo_swarm
                                    //     .swarm
                                    //     .behaviour_mut()
                                    //     .gossipsub
                                    //     .add_explicit_peer(&peer_id);
                                    filo_swarm.swarm.dial(multiaddr)?;

                                    info!("Dial: {:?}", peer_id);
                                }
                            }
                            mdns::Event::Expired(list) => {}
                        }
                    }
                    FiloBehaviourEvent::Identify(e) => {
                        info!("Identify: {:#?}", e);
                    }
                    FiloBehaviourEvent::Gossipsub(e) => {
                        info!("Gossipsub: {:#?}", e);
                    }
                },
                _ => {}
            }
        }
    });

    Ok(keypair_str)
}
