use anyhow::{Ok, Result};
use config::NatsConfig;
use futures::StreamExt;

use fluvio::{RecordKey, TopicProducerPool};
use fluvio_connector_common::{
    connector,
    tracing::{debug, trace},
    Source,
};

mod config;
mod source;
use source::NatsSource;

#[connector(source)]
async fn start(config: NatsConfig, producer: TopicProducerPool) -> Result<()> {
    debug!(?config);

   tokio::runtime::Runtime::new()?.block_on(async {
        let source = NatsSource::new(&config).await?;
        let mut stream = source.connect(None).await?;
        while let Some((key, value)) = stream.next().await {
            trace!(?value);
            _ = key;
            producer.send(RecordKey::NULL, value).await?;
        }
        Ok(())

    })?;
    Ok(())
}
