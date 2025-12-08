use mdns_sd::ServiceDaemon;

use crate::Result;

pub fn discover() -> Result<(
    ServiceDaemon,
    tokio::sync::mpsc::Receiver<mdns_sd::ServiceEvent>,
)> {
    let daemon = ServiceDaemon::new()?;

    let receiver = daemon.browse("_filo._tcp.local.")?;

    let (tx, rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        while let Ok(e) = receiver.recv_async().await {
            tx.send(e).await.unwrap();
        }
    });

    Ok((daemon, rx))
}
