use async_nats::{Client, ServerAddr};
use fluvio::dataplane::record::RecordData;
use fluvio::{Offset, RecordKey};
use fluvio_connector_common::Source;
use futures::stream::LocalBoxStream;
use futures::StreamExt;
use async_trait::async_trait;
use anyhow::Result;
// use std::time::Duration;

use crate::config::NatsConfig;

// const CHANNEL_BUFFER_SIZE: usize = 10000;
// const DEFAULT_FETCH_MAX_BYTES_PER_PARTITION: i32 = 1024 * 1024;
// const BACKOFF_MIN: Duration = Duration::from_secs(0);
// const BACKOFF_MAX: Duration = Duration::from_secs(60);
// const BACKOFF_FACTOR: f64 = 2.0;
// const BACKOFF_LIMIT: Duration = Duration::from_secs(1024);

pub(crate) type Record = (RecordKey, RecordData);


pub(crate) struct NatsSource {
    client: Client,
    subject: String,
    queue_group: Option<String>,
}

impl NatsSource {
    pub(crate) async fn new(config: &NatsConfig) -> Result<Self> {
        let client = async_nats::connect(&config.url.resolve()?.parse::<ServerAddr>()?).await?;
        Ok(Self {
            client,
            subject: config.subject.clone(),
            queue_group: config.queue_group.clone(),
        })
    }
}

#[async_trait]
impl<'a> Source<'a, Record> for NatsSource {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxStream<'a, Record>> {
        let subscription = if let Some(queue_group) = self.queue_group {
            self.client.queue_subscribe(self.subject, queue_group.into()).await?
        } else {
            self.client.subscribe(self.subject).await?
        };

        let stream = subscription.map(|msg| {
            // Convert msg.payload to (RecordKey, RecordData)
            let key = RecordKey::from(""); // Assuming no key, adjust as needed
            let data = RecordData::from(msg.payload);
            (key, data)
        }).boxed_local();
        
        Ok(stream)
    }
}
