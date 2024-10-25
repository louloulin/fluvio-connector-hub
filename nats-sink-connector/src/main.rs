mod config;
use config::CustomConfig;


use futures::StreamExt;

use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};

#[connector(sink)]
async fn start(config: CustomConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting nats-sink-connector sink connector with {config:?}");
    while let Some(Ok(record)) = stream.next().await {
        let val = String::from_utf8_lossy(record.value());
        println!("{val}");
    }
    Ok(())
}

